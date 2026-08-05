//! Deterministic policy evaluation over admitted semantic snapshots.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use semantic_pack::{
    AdapterBindingId, ArtifactHash, CapabilityId, CapabilitySelectorSource, EligibilityDefault,
    PackIdentity, PolicyAttributeId, PolicyContextId, PrivilegeId, SemanticSnapshot, SourceHash,
};
use thiserror::Error;
use uuid::Uuid;

/// Host-neutral principal data used by pack policy.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrincipalContext {
    roles: BTreeSet<String>,
}

impl PrincipalContext {
    /// Construct a principal from application role identifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// use sem_os_policy::pack_policy::PrincipalContext;
    /// let principal = PrincipalContext::new(["Operator", "Reviewer"]);
    /// assert!(principal.roles().contains("operator"));
    /// ```
    pub fn new(roles: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self {
            roles: roles
                .into_iter()
                .map(|role| role.as_ref().to_ascii_lowercase())
                .collect(),
        }
    }

    /// Borrow normalized role identifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// use sem_os_policy::pack_policy::PrincipalContext;
    /// assert_eq!(PrincipalContext::new(["Admin"]).roles().len(), 1);
    /// ```
    #[must_use]
    pub fn roles(&self) -> &BTreeSet<String> {
        &self.roles
    }
}

/// Deterministic reason for a capability decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyReason {
    ExplicitDeny(CapabilitySelectorSource),
    ExplicitAllow(CapabilitySelectorSource),
    DefaultAllow,
    DefaultDeny,
    RoleGrant,
    RoleDenied,
}

/// Content-addressed evidence attached to every policy decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvidence {
    pub pack: PackIdentity,
    pub source_hash: SourceHash,
    pub artifact_hash: ArtifactHash,
}

/// Result of evaluating one capability in one declared context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDecision {
    pub allowed: bool,
    pub context: PolicyContextId,
    pub capability: CapabilityId,
    pub reason: PolicyReason,
    pub evidence: PolicyEvidence,
}

/// Typed failure from snapshot-backed policy evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PackPolicyError {
    #[error("semantic pack does not declare policy context `{0}`")]
    MissingContext(PolicyContextId),
    #[error("semantic pack does not declare privilege `{0}`")]
    MissingPrivilege(PrivilegeId),
    #[error("semantic pack does not declare a persistent UUID namespace")]
    MissingIdentityNamespace,
    #[error("capability `{0}` does not declare an adapter binding")]
    MissingAdapterBinding(CapabilityId),
    #[error("adapter binding `{0}` is already registered")]
    DuplicateAdapterBinding(AdapterBindingId),
    #[error("adapter binding `{0}` is not registered")]
    UnregisteredAdapterBinding(AdapterBindingId),
}

/// Evaluate capability eligibility and exact role grants from one snapshot.
///
/// Deny selectors take precedence over allow selectors. Exact role grants are
/// applied only when at least one role declaration names this capability.
///
/// # Examples
///
/// See the external `pack_policy` integration tests for a complete admitted
/// snapshot example.
pub fn evaluate_capability(
    snapshot: &SemanticSnapshot,
    principal: &PrincipalContext,
    context: &PolicyContextId,
    capability: &CapabilityId,
) -> Result<CapabilityDecision, PackPolicyError> {
    let policy = snapshot
        .pack()
        .policy()
        .eligibility
        .iter()
        .find(|policy| &policy.context == context)
        .ok_or_else(|| PackPolicyError::MissingContext(context.clone()))?;

    let (mut allowed, mut reason) = if let Some(selector) = policy
        .deny
        .iter()
        .find(|selector| selector.matches(capability))
    {
        (false, PolicyReason::ExplicitDeny(selector.clone()))
    } else if let Some(selector) = policy
        .allow
        .iter()
        .find(|selector| selector.matches(capability))
    {
        (true, PolicyReason::ExplicitAllow(selector.clone()))
    } else {
        match policy.default {
            EligibilityDefault::Allow => (true, PolicyReason::DefaultAllow),
            EligibilityDefault::Deny => (false, PolicyReason::DefaultDeny),
        }
    };

    if allowed {
        let relevant_grants = snapshot
            .pack()
            .policy()
            .roles
            .iter()
            .filter(|grant| grant.capabilities.contains(capability))
            .collect::<Vec<_>>();
        if !relevant_grants.is_empty() {
            allowed = relevant_grants
                .iter()
                .any(|grant| principal.roles.contains(grant.role.as_str()));
            reason = if allowed {
                PolicyReason::RoleGrant
            } else {
                PolicyReason::RoleDenied
            };
        }
    }

    Ok(CapabilityDecision {
        allowed,
        context: context.clone(),
        capability: capability.clone(),
        reason,
        evidence: evidence(snapshot),
    })
}

/// Test whether an exact role grant permits a declared capability.
///
/// This is the context-independent primitive used by framework operations
/// such as changeset mutation. The role vocabulary remains entirely in the
/// admitted pack.
///
/// # Examples
///
/// See the external `pack_policy` integration tests for YAML-backed grants.
#[must_use]
pub fn has_capability_grant(
    snapshot: &SemanticSnapshot,
    principal: &PrincipalContext,
    capability: &CapabilityId,
) -> bool {
    snapshot.pack().policy().roles.iter().any(|grant| {
        grant.capabilities.contains(capability) && principal.roles.contains(grant.role.as_str())
    })
}

/// Test whether a principal has a pack-declared privilege.
///
/// # Examples
///
/// See the external `pack_policy` integration tests for YAML-backed role
/// selector examples.
pub fn has_privilege(
    snapshot: &SemanticSnapshot,
    principal: &PrincipalContext,
    privilege: &PrivilegeId,
) -> Result<bool, PackPolicyError> {
    let grant = snapshot
        .pack()
        .policy()
        .privileges
        .iter()
        .find(|grant| &grant.privilege == privilege)
        .ok_or_else(|| PackPolicyError::MissingPrivilege(privilege.clone()))?;
    Ok(principal.roles.iter().any(|role| {
        grant
            .roles
            .iter()
            .any(|selector| selector.matches_normalized(role))
    }))
}

/// Test whether a policy context declares a feature attribute.
///
/// # Examples
///
/// See the external `pack_policy` integration tests for snapshot-backed
/// attribute inspection.
pub fn context_has_attribute(
    snapshot: &SemanticSnapshot,
    context: &PolicyContextId,
    attribute: &PolicyAttributeId,
) -> Result<bool, PackPolicyError> {
    snapshot
        .pack()
        .policy()
        .eligibility
        .iter()
        .find(|policy| &policy.context == context)
        .map(|policy| policy.attributes.contains(attribute))
        .ok_or_else(|| PackPolicyError::MissingContext(context.clone()))
}

/// Read the persistent UUID namespace from an admitted snapshot.
///
/// # Examples
///
/// See the external `pack_policy` integration tests for namespace admission.
pub fn identity_namespace_uuid(snapshot: &SemanticSnapshot) -> Result<Uuid, PackPolicyError> {
    snapshot
        .pack()
        .identity_namespace_uuid()
        .ok_or(PackPolicyError::MissingIdentityNamespace)
}

fn evidence(snapshot: &SemanticSnapshot) -> PolicyEvidence {
    PolicyEvidence {
        pack: snapshot.pack().identity(),
        source_hash: snapshot.pack().receipt().source_hash.clone(),
        artifact_hash: snapshot.pack().receipt().artifact_hash.clone(),
    }
}

/// Technical adapter registered for one stable binding identifier.
pub trait CapabilityAdapter: Send + Sync {
    /// Stable binding implemented by this adapter.
    fn binding_id(&self) -> &AdapterBindingId;
}

/// In-process registry for concrete technical capability adapters.
#[derive(Default)]
pub struct CapabilityAdapterRegistry {
    adapters: BTreeMap<AdapterBindingId, Arc<dyn CapabilityAdapter>>,
}

impl CapabilityAdapterRegistry {
    /// Register one technical adapter exactly once.
    ///
    /// # Examples
    ///
    /// See the external `pack_policy` integration tests for registration and
    /// snapshot-based resolution.
    pub fn register(&mut self, adapter: Arc<dyn CapabilityAdapter>) -> Result<(), PackPolicyError> {
        let binding = adapter.binding_id().clone();
        if self.adapters.insert(binding.clone(), adapter).is_some() {
            return Err(PackPolicyError::DuplicateAdapterBinding(binding));
        }
        Ok(())
    }

    /// Resolve the adapter selected by a pack capability.
    ///
    /// # Examples
    ///
    /// See the external `pack_policy` integration tests for registration and
    /// snapshot-based resolution.
    pub fn resolve(
        &self,
        snapshot: &SemanticSnapshot,
        capability: &CapabilityId,
    ) -> Result<Arc<dyn CapabilityAdapter>, PackPolicyError> {
        let binding = snapshot
            .pack()
            .adapter_binding(capability)
            .ok_or_else(|| PackPolicyError::MissingAdapterBinding(capability.clone()))?;
        self.adapters
            .get(binding)
            .cloned()
            .ok_or_else(|| PackPolicyError::UnregisteredAdapterBinding(binding.clone()))
    }
}

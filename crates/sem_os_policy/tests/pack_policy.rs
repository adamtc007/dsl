use std::sync::Arc;

use sem_os_policy::pack_policy::{
    context_has_attribute, evaluate_capability, has_privilege, identity_namespace_uuid,
    CapabilityAdapter, CapabilityAdapterRegistry, PackPolicyError, PolicyReason, PrincipalContext,
};
use semantic_pack::{
    admit_pack, AdapterBindingId, CapabilityId, InMemoryPackRegistry, PackBytes, PackRegistry,
    PolicyAttributeId, PolicyContextId, PrivilegeId,
};

const POLICY_PACK: &str = r#"
schema_version: 1
pack:
  id: example.policy
  version: 1.0.0
  domain: example
  identity_namespace: example.semantic.v1
  identity_namespace_uuid: 7a3b9f42-e1d4-5a8b-910c-4f2d6e8a1b3c
  canonicalization_version: 1
  dependencies: []
  provenance: { source: fixtures/policy.yaml, revision: test-v1 }
declarations: { domain_types: [], slot_kinds: [], focus_kinds: [] }
capabilities:
  - id: change.add
    adapter_binding: example.change.add
    title: Add a change
    intent_summary: Add one declared change.
    action_class: create
    applicability: A change set is active.
    effect: One change is appended.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: reversible
  - id: record.read
    adapter_binding: example.record.read
    title: Read a record
    intent_summary: Read one admitted record.
    action_class: read
    applicability: The record exists.
    effect: The record is returned without mutation.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: read_only
policy:
  phrase_ambiguity: reject
  abstention: { enabled: true, candidate_id: abstain.none_of_the_above }
  roles:
    - role: administrator
      capabilities: [change.add]
  eligibility:
    - context: mode.research
      default: allow
      deny:
        - { kind: prefix, value: change. }
      attributes: [feature.full-introspection]
    - context: mode.governed
      default: deny
      allow:
        - { kind: exact, value: change.add }
        - { kind: prefix, value: record. }
  privileges:
    - privilege: evidence.review
      roles:
        - { kind: contains, value: steward }
        - { kind: exact, value: compliance_officer }
extensions: {}
"#;

fn snapshot() -> semantic_pack::SemanticSnapshot {
    let pack = admit_pack(PackBytes::new("policy.yaml", POLICY_PACK)).unwrap();
    InMemoryPackRegistry::new().install(pack).unwrap()
}

#[test]
fn selectors_defaults_role_grants_and_evidence_are_deterministic() {
    let snapshot = snapshot();
    let admin = PrincipalContext::new(["administrator"]);
    let viewer = PrincipalContext::new(["viewer"]);
    let change = CapabilityId::new("change.add").unwrap();

    let denied = evaluate_capability(
        &snapshot,
        &admin,
        &PolicyContextId::new("mode.research").unwrap(),
        &change,
    )
    .unwrap();
    assert!(!denied.allowed);
    assert!(matches!(denied.reason, PolicyReason::ExplicitDeny(_)));

    let allowed = evaluate_capability(
        &snapshot,
        &admin,
        &PolicyContextId::new("mode.governed").unwrap(),
        &change,
    )
    .unwrap();
    assert!(allowed.allowed);
    assert_eq!(allowed.reason, PolicyReason::RoleGrant);
    assert_eq!(
        allowed.evidence.artifact_hash,
        snapshot.pack().receipt().artifact_hash
    );

    let role_denied = evaluate_capability(
        &snapshot,
        &viewer,
        &PolicyContextId::new("mode.governed").unwrap(),
        &change,
    )
    .unwrap();
    assert!(!role_denied.allowed);
    assert_eq!(role_denied.reason, PolicyReason::RoleDenied);
}

#[test]
fn privilege_attributes_namespace_and_missing_context_are_typed() {
    let snapshot = snapshot();
    let privilege = PrivilegeId::new("evidence.review").unwrap();
    assert!(has_privilege(
        &snapshot,
        &PrincipalContext::new(["data_steward"]),
        &privilege
    )
    .unwrap());
    assert!(has_privilege(
        &snapshot,
        &PrincipalContext::new(["COMPLIANCE_OFFICER"]),
        &privilege
    )
    .unwrap());
    assert!(!has_privilege(&snapshot, &PrincipalContext::new(["analyst"]), &privilege).unwrap());
    assert!(context_has_attribute(
        &snapshot,
        &PolicyContextId::new("mode.research").unwrap(),
        &PolicyAttributeId::new("feature.full-introspection").unwrap(),
    )
    .unwrap());
    assert_eq!(
        identity_namespace_uuid(&snapshot).unwrap().to_string(),
        "7a3b9f42-e1d4-5a8b-910c-4f2d6e8a1b3c"
    );
    let error = evaluate_capability(
        &snapshot,
        &PrincipalContext::default(),
        &PolicyContextId::new("mode.missing").unwrap(),
        &CapabilityId::new("record.read").unwrap(),
    )
    .unwrap_err();
    assert!(matches!(error, PackPolicyError::MissingContext(_)));
}

struct ReadAdapter {
    binding: AdapterBindingId,
}

impl CapabilityAdapter for ReadAdapter {
    fn binding_id(&self) -> &AdapterBindingId {
        &self.binding
    }
}

#[test]
fn adapter_registry_resolves_only_pack_selected_bindings() {
    let snapshot = snapshot();
    let adapter = Arc::new(ReadAdapter {
        binding: AdapterBindingId::new("example.record.read").unwrap(),
    });
    let mut registry = CapabilityAdapterRegistry::default();
    registry.register(adapter.clone()).unwrap();
    let resolved = registry
        .resolve(&snapshot, &CapabilityId::new("record.read").unwrap())
        .unwrap();
    assert_eq!(resolved.binding_id(), adapter.binding_id());
}

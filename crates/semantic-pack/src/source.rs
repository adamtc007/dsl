use std::collections::BTreeMap;

use semantic_decision_contracts::{ActionClass, ArgumentKind, HarmClass, PhraseRole};
use serde::{Deserialize, Serialize};

use crate::{
    AdapterBindingId, CapabilityId, DomainIdentity, DomainTypeId, FocusKind, GraphNodeId,
    IdentityNamespace, PackId, PackSourceError, PackVersion, RoleId, SlotKind,
};

/// Host-supplied request for exact pack bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackRequest {
    /// Requested pack ID.
    pub id: PackId,
    /// Optional exact version. Sources must not silently substitute another version.
    pub version: Option<PackVersion>,
}

/// Source bytes plus stable diagnostic metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackBytes {
    /// Human-readable stable source name, such as an embedded resource path.
    pub source_name: String,
    /// Exact YAML bytes.
    pub bytes: Vec<u8>,
}

impl PackBytes {
    /// Construct pack bytes from a source name and byte buffer.
    #[must_use]
    pub fn new(source_name: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            source_name: source_name.into(),
            bytes: bytes.into(),
        }
    }
}

/// Port implemented by embedded, filesystem, database or network adapters.
pub trait PackSource {
    /// Load exact bytes for a request without parsing or activating them.
    fn load(&self, request: &PackRequest) -> Result<PackBytes, PackSourceError>;
}

/// Normative YAML semantic pack source document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackDocument {
    /// Source schema version.
    pub schema_version: u32,
    /// Stable metadata and provenance.
    pub pack: PackMetadataSource,
    /// Pack-declared generic semantic kinds.
    #[serde(default)]
    pub declarations: DeclarationSource,
    /// Legal semantic capabilities.
    #[serde(default)]
    pub capabilities: Vec<CapabilitySource>,
    /// Optional deterministic decision graph.
    #[serde(default)]
    pub graph: Option<GraphSource>,
    /// Declarative ambiguity, abstention and role policy.
    #[serde(default)]
    pub policy: PackPolicySource,
    /// Bounded namespaced data not yet promoted to the core schema.
    #[serde(default)]
    pub extensions: BTreeMap<String, ConfigValue>,
    /// Diagnostic source name, assigned by `parse_pack` and excluded from YAML.
    #[serde(skip)]
    pub(crate) source_name: String,
    /// SHA-256 of exact source bytes, assigned by `parse_pack`.
    #[serde(skip)]
    pub(crate) source_hash: String,
}

/// Stable pack identity, namespace, dependencies and provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackMetadataSource {
    pub id: PackId,
    pub version: PackVersion,
    pub domain: DomainIdentity,
    pub identity_namespace: IdentityNamespace,
    pub canonicalization_version: u32,
    #[serde(default)]
    pub dependencies: Vec<DependencySource>,
    pub provenance: ProvenanceSource,
}

/// Exact dependency on another compiled pack artifact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencySource {
    pub id: PackId,
    pub version: PackVersion,
    pub artifact_hash: String,
}

/// Auditable source provenance supplied by the owning application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceSource {
    pub source: String,
    pub revision: String,
}

/// Generic semantic types declared by this pack.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationSource {
    #[serde(default)]
    pub domain_types: Vec<DomainTypeId>,
    #[serde(default)]
    pub slot_kinds: Vec<SlotKind>,
    #[serde(default)]
    pub focus_kinds: Vec<FocusKind>,
}

/// One application capability and its model-visible semantic projection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilitySource {
    pub id: CapabilityId,
    pub adapter_binding: AdapterBindingId,
    pub title: String,
    pub intent_summary: String,
    pub action_class: ActionClass,
    pub applicability: String,
    pub effect: String,
    #[serde(default)]
    pub arguments: Vec<ArgumentSource>,
    #[serde(default)]
    pub phrases: Vec<PhraseSource>,
    #[serde(default)]
    pub positive_examples: Vec<String>,
    #[serde(default)]
    pub negative_contrasts: Vec<NegativeContrastSource>,
    pub risk: HarmClass,
    #[serde(default)]
    pub aliases: Vec<CapabilityId>,
    #[serde(default)]
    pub deprecation: Option<DeprecationSource>,
    #[serde(default)]
    pub extensions: BTreeMap<String, ConfigValue>,
}

/// Typed capability argument declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArgumentSource {
    pub name: SlotKind,
    pub kind: ArgumentKind,
    pub required: bool,
    pub clarification_prompt: String,
    #[serde(default)]
    pub default: Option<ConfigValue>,
    #[serde(default)]
    pub constraints: Vec<ArgumentConstraint>,
}

/// Closed declarative argument validation constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ArgumentConstraint {
    AllowedValues { values: Vec<ConfigValue> },
    IntegerRange { minimum: i64, maximum: i64 },
    TextLength { minimum: u32, maximum: u32 },
    Pattern { pattern: String },
}

/// Governed phrase evidence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhraseSource {
    pub text: String,
    pub locale: String,
    pub role: PhraseRole,
    pub provenance: String,
}

/// Explicit contrast with a nearby candidate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NegativeContrastSource {
    pub candidate_id: CapabilityId,
    pub distinction: String,
}

/// Compatibility lifecycle for a capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationSource {
    pub since: PackVersion,
    #[serde(default)]
    pub replacement: Option<CapabilityId>,
    pub message: String,
}

/// Optional deterministic decision/narrowing graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphSource {
    pub acyclic: bool,
    pub entry_nodes: Vec<GraphNodeId>,
    pub nodes: Vec<GraphNodeSource>,
    #[serde(default)]
    pub edges: Vec<GraphEdgeSource>,
}

/// One graph position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphNodeSource {
    pub id: GraphNodeId,
    #[serde(default)]
    pub candidates: Vec<CapabilityId>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub terminal: Option<TerminalDisposition>,
}

/// Directed graph edge with a declarative narrowing predicate label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphEdgeSource {
    pub from: GraphNodeId,
    pub to: GraphNodeId,
    pub when: String,
}

/// Terminal graph outcome. Execution remains an application responsibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDisposition {
    Candidate,
    ClarifyCandidates,
    OutOfScope,
    Escalate,
}

/// Ambiguous phrase/alias admission policy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhraseAmbiguityPolicy {
    #[default]
    Reject,
    AllowExplicitClarification,
}

/// Declarative pack policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackPolicySource {
    #[serde(default)]
    pub phrase_ambiguity: PhraseAmbiguityPolicy,
    #[serde(default)]
    pub abstention: AbstentionSource,
    #[serde(default)]
    pub roles: Vec<RoleGrantSource>,
}

impl Default for PackPolicySource {
    fn default() -> Self {
        Self {
            phrase_ambiguity: PhraseAmbiguityPolicy::Reject,
            abstention: AbstentionSource::default(),
            roles: Vec::new(),
        }
    }
}

/// Framework abstention declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstentionSource {
    pub enabled: bool,
    pub candidate_id: CapabilityId,
}

impl Default for AbstentionSource {
    fn default() -> Self {
        Self {
            enabled: true,
            candidate_id: CapabilityId::new("abstain.none_of_the_above")
                .expect("framework abstention ID is valid"),
        }
    }
}

/// Role-to-capability grants declared by a pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoleGrantSource {
    pub role: RoleId,
    pub capabilities: Vec<CapabilityId>,
}

/// Bounded typed configuration value for namespaced extensions and defaults.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Text(String),
    List(Vec<ConfigValue>),
    Map(BTreeMap<String, ConfigValue>),
}

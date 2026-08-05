use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::{
    CapabilityId, CapabilitySource, ConfigValue, DeclarationSource, DependencySource,
    DomainIdentity, GraphNodeId, GraphNodeSource, GraphSource, IdentityNamespace, PackDocument,
    PackIdentity, PackPolicySource, ProvenanceSource,
};

/// SHA-256 of exact input bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SourceHash(pub(crate) String);

impl SourceHash {
    /// Construct a validated source hash.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        validate_hash(value.into()).map(Self)
    }

    /// Borrow the lowercase hexadecimal digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SourceHash {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SourceHash> for String {
    fn from(value: SourceHash) -> Self {
        value.0
    }
}

/// SHA-256 of canonical compiled artifact bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ArtifactHash(pub(crate) String);

impl ArtifactHash {
    /// Construct a validated artifact hash.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        validate_hash(value.into()).map(Self)
    }

    /// Borrow the lowercase hexadecimal digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ArtifactHash {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ArtifactHash> for String {
    fn from(value: ArtifactHash) -> Self {
        value.0
    }
}

fn validate_hash(value: String) -> Result<String, String> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(value)
    } else {
        Err("hash must contain 64 lowercase hexadecimal characters".to_owned())
    }
}

/// Reproducibility and provenance receipt for one admitted artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackReceipt {
    pub identity: PackIdentity,
    pub schema_version: u32,
    pub canonicalization_version: u32,
    pub compiler_version: String,
    pub source_name: String,
    pub source_hash: SourceHash,
    pub dependencies: Vec<DependencySource>,
    pub adapter_bindings: Vec<String>,
    pub artifact_hash: ArtifactHash,
}

/// Immutable normalized capability. This alias remains source-shaped so hosts
/// can project it without another semantic mirror.
pub type CompiledCapability = CapabilitySource;

/// Immutable normalized graph node.
pub type CompiledGraphNode = GraphNodeSource;

/// Immutable normalized decision graph.
pub type CompiledGraph = GraphSource;

/// Deterministically normalized, immutable semantic pack artifact.
#[derive(Debug, Clone)]
pub struct CompiledPack {
    pub(crate) document: Arc<PackDocument>,
    pub(crate) canonical_bytes: Arc<[u8]>,
    pub(crate) receipt: Arc<PackReceipt>,
}

impl CompiledPack {
    /// Stable logical identity.
    #[must_use]
    pub fn identity(&self) -> PackIdentity {
        PackIdentity::new(
            self.document.pack.id.clone(),
            self.document.pack.version.clone(),
        )
    }

    /// Domain identity declared by the owning application.
    #[must_use]
    pub fn domain(&self) -> &DomainIdentity {
        &self.document.pack.domain
    }

    /// Identity namespace used for stable application IDs.
    #[must_use]
    pub fn identity_namespace(&self) -> &IdentityNamespace {
        &self.document.pack.identity_namespace
    }

    /// Optional persistent UUID namespace declared by the pack.
    #[must_use]
    pub fn identity_namespace_uuid(&self) -> Option<uuid::Uuid> {
        self.document.pack.identity_namespace_uuid
    }

    /// Exact provenance declaration.
    #[must_use]
    pub fn provenance(&self) -> &ProvenanceSource {
        &self.document.pack.provenance
    }

    /// Generic semantic kinds declared by the pack.
    #[must_use]
    pub fn declarations(&self) -> &DeclarationSource {
        &self.document.declarations
    }

    /// Declarative policy.
    #[must_use]
    pub fn policy(&self) -> &PackPolicySource {
        &self.document.policy
    }

    /// Bounded, namespaced pack-level configuration.
    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<String, ConfigValue> {
        &self.document.extensions
    }

    /// Canonically ordered capabilities.
    pub fn capabilities(&self) -> impl ExactSizeIterator<Item = &CompiledCapability> {
        self.document.capabilities.iter()
    }

    /// Resolve one stable capability ID.
    #[must_use]
    pub fn capability(&self, id: &CapabilityId) -> Option<&CompiledCapability> {
        self.document
            .capabilities
            .iter()
            .find(|item| &item.id == id)
    }

    /// Resolve the named adapter binding for a capability.
    #[must_use]
    pub fn adapter_binding(&self, id: &CapabilityId) -> Option<&crate::AdapterBindingId> {
        self.capability(id)
            .map(|capability| &capability.adapter_binding)
    }

    /// Optional compiled graph.
    #[must_use]
    pub fn graph(&self) -> Option<&CompiledGraph> {
        self.document.graph.as_ref()
    }

    /// Resolve one graph position.
    #[must_use]
    pub fn graph_node(&self, id: &GraphNodeId) -> Option<&CompiledGraphNode> {
        self.graph()?.nodes.iter().find(|node| &node.id == id)
    }

    /// Return graph successors in canonical ID order.
    #[must_use]
    pub fn successors(&self, id: &GraphNodeId) -> Vec<&CompiledGraphNode> {
        let Some(graph) = self.graph() else {
            return Vec::new();
        };
        let targets: BTreeSet<_> = graph
            .edges
            .iter()
            .filter(|edge| &edge.from == id)
            .map(|edge| &edge.to)
            .collect();
        targets
            .into_iter()
            .filter_map(|target| self.graph_node(target))
            .collect()
    }

    /// Canonical artifact bytes used by the artifact hash.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    /// Reproducibility receipt.
    #[must_use]
    pub fn receipt(&self) -> &PackReceipt {
        &self.receipt
    }
}

/// Addressable immutable registry snapshot.
#[derive(Debug, Clone)]
pub struct SemanticSnapshot {
    pack: Arc<CompiledPack>,
}

impl SemanticSnapshot {
    pub(crate) fn new(pack: Arc<CompiledPack>) -> Self {
        Self { pack }
    }

    /// Borrow the immutable compiled pack.
    #[must_use]
    pub fn pack(&self) -> &CompiledPack {
        &self.pack
    }

    /// Clone the shared immutable compiled pack handle.
    #[must_use]
    pub fn into_pack(self) -> Arc<CompiledPack> {
        self.pack
    }
}

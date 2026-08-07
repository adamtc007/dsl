#![forbid(unsafe_code)]

//! Deterministic compilation and immutable registration of semantic packs.
//!
//! The crate is deliberately host-neutral: applications own YAML sources and
//! implementations for the stable adapter binding IDs declared by those
//! sources. Compilation itself accepts bytes and performs no ambient I/O.

mod artifact;
mod compile;
mod diagnostic;
mod identity;
mod registry;
mod source;
mod validate;

pub use artifact::{
    ArtifactHash, CompiledCapability, CompiledGraph, CompiledGraphNode, CompiledPack, PackReceipt,
    SemanticSnapshot, SourceHash,
};
pub use compile::{admit_pack, compile_pack};
pub use diagnostic::{
    Diagnostic, DiagnosticCode, PackAdmissionError, PackCompileError, PackParseError,
    PackSourceError, PackValidationErrors, RegistryError,
};
pub use dsl_types::{CapabilityId, DomainTypeId, FocusKind, SlotKind};
pub use identity::{
    AdapterBindingId, CapabilityPrefix, DomainIdentity, GraphNodeId, IdentityError,
    IdentityNamespace, PackId, PackIdentity, PackVersion, PolicyAttributeId, PolicyContextId,
    PrivilegeId, RoleFragment, RoleId,
};
pub use registry::{InMemoryPackRegistry, PackRegistry};
pub use source::{
    AbstentionSource, ArgumentConstraint, ArgumentSource, CapabilitySelectorSource,
    CapabilitySource, ConfigValue, DeclarationSource, DependencySource, DeprecationSource,
    EligibilityDefault, EligibilityPolicySource, EvidenceCueSource, EvidenceFeatureSource,
    EvidenceGateEffect, EvidenceGateSource, EvidencePolicySource, FeedbackOptionSource,
    GraphEdgeSource, GraphNodeSource, GraphSource, PackBytes, PackDocument, PackMetadataSource,
    PackPolicySource, PackRequest, PackSource, PhraseAmbiguityPolicy, PrivilegeGrantSource,
    ProvenanceSource, RoleGrantSource, RoleSelectorSource, RuleExplanationSource,
    TerminalDisposition,
};
pub use validate::{parse_pack, validate_pack, ValidatedPack};

/// Source schema version accepted by this compiler.
pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;

/// Canonical artifact encoding version emitted by this compiler.
pub const SUPPORTED_CANONICALIZATION_VERSION: u32 = 1;

/// Stable compiler identifier recorded in every artifact receipt.
pub const COMPILER_VERSION: &str = concat!("semantic-pack/", env!("CARGO_PKG_VERSION"));

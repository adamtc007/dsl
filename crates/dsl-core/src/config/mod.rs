//! YAML-driven DSL configuration
//!
//! This module provides runtime configuration loading for DSL verbs,
//! allowing verb definitions to be modified without recompiling Rust code.
//!
//! # Architecture
//!
//! ```text
//! config/verbs.yaml → ConfigLoader → VerbsConfig → RuntimeVerbRegistry
//! ```
//!
//! # Usage
//!
//! ```ignore
//! use crate::dsl_v2::config::ConfigLoader;
//!
//! let loader = ConfigLoader::from_env();
//! let verbs = loader.load_verbs()?;
//! ```

pub(crate) mod dag;
pub(crate) mod dag_validator;
pub(crate) mod effect_class;
pub(crate) mod escalation;
pub(crate) mod green_when_coverage;
pub(crate) mod loader;
pub(crate) mod manifest;
pub(crate) mod pack_loader;
pub(crate) mod phrase_gen;
pub(crate) mod predicate; // used by sem_os_core::frontier::hydrator
pub(crate) mod resource_dependency;
pub(crate) mod runbook_composition;
pub(crate) mod types;
pub(crate) mod validator;

pub use dag::{load_dags_from_dir, load_domain_pack_owned_dags, Dag, LoadedDag};
// ob-poc-specific filesystem walkers (validate_constellation_map_dir_*)
// are pub(crate) in dag_validator — not
// re-exported here because they assume ob-poc's config directory layout.
pub use dag_validator::DagWarning;
pub use dag_validator::{
    entity_kinds_from_taxonomy_yaml, validate_dags_with_context,
    validate_resolved_template_gate_metadata, validate_slot_gating_states, DagError,
    DagValidationContext, DagValidationReport,
};

pub use loader::ConfigLoader;

pub use types::{
    ActionClass, ArgConfig, ArgType, ArgValidation, BatchPolicyConfig, ConfirmPolicyConfig,
    ConsequenceDeclaration, ConsequenceTier, CrudConfig, CrudOperation, DomainConfig,
    DurableConfig, DurableRuntime, DynamicVerbConfig, EscalationPredicate, EscalationRule,
    ExternalEffect, FuzzyCheckConfig, GraphQueryConfig, GraphQueryOperation, HarmClass,
    LockAccessConfig, LockModeConfig, LookupConfig, PolicyConfig, ResolutionMode, ReturnTypeConfig,
    ReturnsConfig, SearchKeyConfig, SlotType, SourceOfTruth, StateEffect, ThreeAxisDeclaration,
    TransitionArgs, TransitionEdge, VerbBehavior, VerbConfig, VerbConsumes, VerbFlavour,
    VerbLifecycle, VerbMetadata, VerbOutputConfig, VerbProduces, VerbRoleGuard, VerbScope,
    VerbSentences, VerbStatus, VerbTier, VerbTransitions, VerbWriteConfig, VerbsConfig,
};
pub use validator::StructuralError;
pub use validator::{
    collect_declared_fqns, validate_pack_fqns, validate_verbs_config, Location, PolicyWarning,
    ValidationContext, ValidationReport, WellFormednessError,
};

pub use manifest::{wiring_check, ManifestError, VerbDeclaration, VerbManifest, WiringReport};
pub use pack_loader::{flatten_pack_entries, load_packs_from_dir, LoadedPack};
pub use phrase_gen::{generate_phrases, set_phrase_gen_nouns, PhraseGenNouns};
pub use resource_dependency::{ResolvedResourceDependency, ResourceDependency};

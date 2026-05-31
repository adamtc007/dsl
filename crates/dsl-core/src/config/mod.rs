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
pub(crate) mod phrase_gen;
pub(crate) mod predicate;                  // used by sem_os_core::frontier::hydrator
pub(crate) mod resource_dependency;
pub(crate) mod runbook_composition;
pub(crate) mod types;
pub(crate) mod validator;

pub use dag::{load_dags_from_dir, load_domain_pack_owned_dags, Dag, LoadedDag};
// ob-poc-specific filesystem walkers (validate_constellation_map_dir_*)
// are pub(crate) in dag_validator — not
// re-exported here because they assume ob-poc's config directory layout.
pub(crate) use dag_validator::{
    validate_constellation_map_schema_coordination, DagWarning,
};
pub use dag_validator::{
    validate_dags_with_context, validate_resolved_template_gate_metadata, DagError,
    DagValidationContext, DagValidationReport,
};
pub(crate) use green_when_coverage::{
    green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
    GreenWhenExclusionReason,
};

pub(crate) use escalation::{
    compute_effective_tier, EvaluationContext,
};
pub use loader::ConfigLoader;

pub(crate) use runbook_composition::{
    compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
};
pub use types::{
    ActionClass, ArgConfig, ArgType, ArgValidation, ConfirmPolicyConfig,
    ConsequenceDeclaration, ConsequenceTier, CrudConfig, CrudOperation,
    DomainConfig, DurableConfig, DurableRuntime, EscalationPredicate,
    EscalationRule, ExternalEffect, FuzzyCheckConfig, GraphQueryConfig, GraphQueryOperation,
    HarmClass, LookupConfig, ResolutionMode,
    ReturnTypeConfig, ReturnsConfig, SearchKeyConfig,
    SlotType, SourceOfTruth, StateEffect, ThreeAxisDeclaration, TransitionArgs, TransitionEdge,
    VerbBehavior, VerbConfig, VerbConsumes, VerbFlavour, VerbLifecycle, VerbMetadata,
    VerbOutputConfig, VerbProduces, VerbRoleGuard, VerbScope, VerbSentences, VerbStatus, VerbTier,
    VerbTransitions, VerbsConfig, PolicyConfig, VerbWriteConfig,
    BatchPolicyConfig, DynamicVerbConfig, LockAccessConfig, LockModeConfig,
};
pub(crate) use validator::StructuralError;
pub use validator::{
    validate_verbs_config, Location, PolicyWarning, ValidationContext,
    ValidationReport, WellFormednessError,
};

pub use resource_dependency::{ResolvedResourceDependency, ResourceDependency};
pub use phrase_gen::{generate_phrases, PhraseGenNouns, set_phrase_gen_nouns};
pub use manifest::{VerbManifest, VerbDeclaration, ManifestError, WiringReport, wiring_check};


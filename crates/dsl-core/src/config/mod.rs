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

pub mod dag;
pub mod dag_validator;
pub mod effect_class;
pub mod escalation;
pub mod green_when_coverage;
pub mod loader;
pub mod phrase_gen;
pub mod predicate;                  // used by sem_os_core::frontier::hydrator
pub mod resource_dependency;
pub mod runbook_composition;
pub mod types;
pub mod validator;

pub use dag::{load_dags_from_dir, load_domain_pack_owned_dags, Dag, LoadedDag};
// ob-poc-specific filesystem walkers (validate_constellation_map_dir_*)
// are pub(crate) in dag_validator — not
// re-exported here because they assume ob-poc's config directory layout.
pub use dag_validator::{
    validate_constellation_map_schema_coordination,
    validate_dags_with_context, validate_resolved_template_gate_metadata, DagError,
    DagValidationContext, DagValidationReport, DagWarning, SchemaCoordinationKnownDeferred,
};
pub use green_when_coverage::{
    green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
    GreenWhenCoverageRow, GreenWhenCoverageSummary, GreenWhenExclusionReason,
};

pub use escalation::{
    compute_effective_tier, compute_effective_tier_with_trace, evaluate_predicate,
    EvaluationContext,
};
pub use loader::ConfigLoader;

pub use runbook_composition::{
    component_a, component_b, component_c, compute_runbook_tier,
    AggregationRule, CrossScopeRule, RunbookStep,
};
pub use types::{
    ActionClass, AppliesTo, ArgConfig, ArgType, ArgValidation, ConfirmPolicyConfig,
    ConsequenceDeclaration, ConsequenceTier, ConstraintRule, CrudConfig, CrudOperation,
    DomainConfig, DurableConfig, DurableRuntime, EscalationPredicate,
    EscalationRule, ExternalEffect, FuzzyCheckConfig, GraphQueryConfig, GraphQueryOperation,
    HarmClass, JurisdictionCondition, JurisdictionRule, LookupConfig, ResolutionMode,
    ReturnTypeConfig, ReturnsConfig, RuleCondition, RuleRequirement, RuleSeverity, SearchKeyConfig,
    SlotType, SourceOfTruth, StateEffect, ThreeAxisDeclaration, TransitionArgs, TransitionEdge,
    VerbBehavior, VerbConfig, VerbConsumes, VerbFlavour, VerbLifecycle, VerbMetadata,
    VerbOutputConfig, VerbProduces, VerbRoleGuard, VerbScope, VerbSentences, VerbStatus, VerbTier,
    VerbTransitions, VerbsConfig, WarningRule,
};
pub use validator::{
    validate_verbs_config, Location, PolicyWarning, StructuralError, ValidationContext,
    ValidationReport, WellFormednessError,
};

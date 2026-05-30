//! dsl-core: Core DSL parser, AST, and types for OB-POC
//!
//! This crate contains the pure DSL logic with NO database dependencies:
//! - AST types (Program, Statement, VerbCall, AstNode, etc.)
//! - Nom-based S-expression parser
//! - Binding context for symbol resolution
//! - Diagnostic types for error reporting
//! - Op-free compiler (compile_to_steps, CompileStep)
//! - YAML configuration types and loader
//!
//! The Op enum and DAG builder were removed in Phase 3 CR A4.
//! The execution layer (generic_executor, custom_ops) remains in ob-poc
//! as it requires database access.

// Keep submodules private to the crate
pub(crate) mod ast;
pub(crate) mod binding_context;
pub(crate) mod compiler;
pub(crate) mod config;
pub(crate) mod diagnostics;
pub(crate) mod executable_plan;
pub(crate) mod execution_dag;
pub(crate) mod frontier;
pub(crate) mod parser;
pub(crate) mod resolver;
pub(crate) mod viewport_parser;

// Re-export from ast
pub use ast::{
    count_entity_refs, Argument, AstNode, ConfidenceZone, EnhanceArg, EntityRefStats, ExportFormat,
    FocusTarget, Literal, NavDirection, NavTarget, Program, Span, Statement, VerbCall, ViewType,
    ViewportVerb,
};

// Re-export from binding_context
pub use binding_context::{BindingContext, BindingInfo};

// Re-export from compiler
pub use compiler::{CompileError, CompileStep, CompiledSteps, compile_to_steps};

// Re-export from config
pub use config::{
    ActionClass, AppliesTo, ArgConfig, ArgType, ArgValidation, ConfirmPolicyConfig,
    ConsequenceDeclaration, ConsequenceTier, ConstraintRule, CrudConfig, CrudOperation,
    Dag, DagError, DagValidationContext, DagValidationReport, DomainConfig, DurableConfig,
    DurableRuntime, EscalationPredicate, EscalationRule, evaluate_predicate, ExternalEffect,
    FuzzyCheckConfig, GraphQueryConfig, GraphQueryOperation, HarmClass, JurisdictionCondition,
    JurisdictionRule, LoadedDag, Location, LookupConfig, PolicyWarning, ResolutionMode,
    ReturnTypeConfig, ReturnsConfig, RuleCondition, RuleRequirement, RuleSeverity, SearchKeyConfig,
    SlotType, SourceOfTruth, StateEffect, ThreeAxisDeclaration, TransitionArgs, TransitionEdge,
    validate_verbs_config, ValidationContext, ValidationReport, VerbBehavior, VerbConfig,
    VerbConsumes, VerbFlavour, VerbLifecycle, VerbMetadata, VerbOutputConfig, VerbProduces,
    VerbRoleGuard, VerbScope, VerbSentences, VerbStatus, VerbTier, VerbTransitions, VerbsConfig,
    WarningRule, WellFormednessError,
    load_dags_from_dir, load_domain_pack_owned_dags,
    validate_dags_with_context, validate_resolved_template_gate_metadata,
    SchemaCoordinationKnownDeferred,
    GreenWhenCoverageRow, GreenWhenCoverageSummary,
    compute_effective_tier_with_trace,
    component_a, component_b, component_c,
    ConfigLoader, PolicyConfig, VerbWriteConfig,
    BatchPolicyConfig, DynamicVerbConfig, LockAccessConfig, LockModeConfig,
    ResolvedResourceDependency, ResourceDependency, PhraseGenNouns, generate_phrases,
};

// Re-export from config::dag
pub use config::dag::{
    CascadeRule, CrossWorkspaceConstraint, DerivationCondition, DerivedCrossWorkspaceState, EntryVia,
    Phase, StateSelector, SlotStateMachine, PredicateBinding, Slot as DagSlot,
    Severity as DagSeverity,
};

// Re-export from config::predicate
pub use config::predicate::{
    parse_green_when, AttrValue, CmpOp, EntityRef as PredicateEntityRef, EntitySetRef,
    Predicate, Validity,
};

// Re-export from diagnostics
pub use diagnostics::{
    cycle_error, implicit_create_hint, missing_arg_error, undefined_symbol_error,
    unknown_verb_error, Diagnostic, DiagnosticCode, RelatedInfo, Severity, SourceSpan, SuggestedFix,
    // Note: DagWarning is not used or re-exported here
};

// Crate-internal re-export of TransactionPolicy
pub(crate) use executable_plan::TransactionPolicy;

// Re-export from executable_plan
pub use executable_plan::{
    AuthorityContext, BindingFrameSchema, BindingSlot, EffectClass, ExecutablePlan,
    ExecutionStepSummary, InstructionInput, PlanId, RuntimeInstruction, SemOsSnapshotId,
};

// Re-export from execution_dag
pub use execution_dag::{BindingSlotId, DagEdge, JoinBarrierMode, NodeId, PopulatedExecutionDag};

// Re-export from frontier
pub use frontier::{
    CompletenessAssertionStatus, DiscretionaryReason, EntityRef, FrontierFact, FrontierFacts,
    GreenWhenStatus, HydrateFrontierError, InstanceFrontier, InvalidFact, InvalidFactDetail,
    MissingFact, ReachableDestination,
};

// Re-export from parser
pub use parser::{parse_program, parse_single_verb};

// Re-export from resolver
pub use resolver::{
    compute_version_hash, ResolvedSlot, ResolvedSource, ResolvedTemplate, ResolvedTransition,
    ResolverProvenance, ShapeRef, SlotProvenance, WorkspaceId, VersionHash, ManifestOptions, ResolverManifest
};

// Re-export from dsl_types
pub use dsl_types::{ClosureType, RoleGuard, EligibilityConstraint};

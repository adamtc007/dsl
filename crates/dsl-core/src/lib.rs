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
    count_entity_refs, find_unresolved_ref_locations, Argument, AstNode, ConfidenceZone,
    EnhanceArg, EntityRefStats, FocusTarget, Literal, Program, Span, Statement,
    UnresolvedRefLocation, VerbCall,
};

// Re-export from binding_context
pub use binding_context::{BindingContext, BindingInfo};

// Re-export from compiler
pub use compiler::{compile_to_steps, CompileError, CompileStep, CompiledSteps};

// Re-export from config
pub use config::{
    collect_declared_fqns, entity_kinds_from_taxonomy_yaml, flatten_pack_entries,
    load_packs_from_dir, set_phrase_gen_nouns, validate_pack_fqns, wiring_check, LoadedPack,
    ManifestError, VerbDeclaration, VerbManifest, WiringReport,
};
pub use config::{
    generate_phrases, load_dags_from_dir, load_domain_pack_owned_dags, validate_dags_with_context,
    validate_resolved_template_gate_metadata, validate_slot_gating_states, validate_verbs_config,
    ActionClass, ArgConfig, ArgType, ArgValidation, BatchPolicyConfig, ConfigLoader,
    ConfirmPolicyConfig, ConsequenceDeclaration, ConsequenceTier, CrudConfig, CrudOperation, Dag,
    DagError, DagValidationContext, DagValidationReport, DagWarning, DomainConfig, DurableConfig,
    DurableRuntime, DynamicVerbConfig, EscalationPredicate, EscalationRule, ExternalEffect,
    FuzzyCheckConfig, GraphQueryConfig, GraphQueryOperation, HarmClass, LoadedDag, Location,
    LockAccessConfig, LockModeConfig, LookupConfig, PhraseGenNouns, PolicyConfig, PolicyWarning,
    ResolutionMode, ResolvedResourceDependency, ResourceDependency, ReturnTypeConfig,
    ReturnsConfig, SearchKeyConfig, SlotType, SourceOfTruth, StateEffect, StructuralError,
    ThreeAxisDeclaration, TransitionArgs, TransitionEdge, ValidationContext, ValidationReport,
    VerbBehavior, VerbConfig, VerbConsumes, VerbFlavour, VerbLifecycle, VerbMetadata,
    VerbOutputConfig, VerbProduces, VerbRoleGuard, VerbScope, VerbSentences, VerbStatus, VerbTier,
    VerbTransitions, VerbWriteConfig, VerbsConfig, WellFormednessError,
};

// Re-export from config::dag
pub use config::dag::{
    CascadeRule, CrossWorkspaceConstraint, DerivationCondition, DerivedCrossWorkspaceState,
    EntryVia, Phase, PredicateBinding, Severity as DagSeverity, Slot as DagSlot, SlotStateMachine,
    StateSelector,
};

// Re-export from config::predicate
pub use config::predicate::{
    parse_green_when, AttrValue, CmpOp, EntityRef as PredicateEntityRef, EntitySetRef, Predicate,
    Validity,
};

// Re-export from diagnostics
pub use diagnostics::{
    cycle_error,
    implicit_create_hint,
    missing_arg_error,
    undefined_symbol_error,
    unknown_verb_error,
    Diagnostic,
    DiagnosticCode,
    RelatedInfo,
    Severity,
    SourceSpan,
    SuggestedFix,
    // Note: DagWarning is not used or re-exported here
};

// Crate-internal re-export of TransactionPolicy

// Re-export from executable_plan
pub use executable_plan::{
    validate_program_admission, BindingFrameSchema, BindingSlot, CatalogProvider, EffectClass,
    LensBinding, PackDagContext, SemOsSnapshotId,
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
    compute_version_hash, ManifestOptions, ResolvedSlot, ResolvedSource, ResolvedTemplate,
    ResolvedTransition, ResolverManifest, ResolverProvenance, ShapeRef, SlotProvenance,
    VersionHash, WorkspaceId,
};

// Re-export from dsl_types
pub use dsl_types::{
    ClosureType, EligibilityConstraint, GatingStatus, RoleGuard, SlotGatingState, SlotKey,
};

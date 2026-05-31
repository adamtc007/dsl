//! `dsl_types` — Level 0 substrate types.
//!
//! Pure data with serde. No DB, no SemOS, no app coupling.
//! This crate is the bottom of the dependency graph:
//!
//! ```text
//! dsl_types  (this crate — std + serde only)
//!     ↑
//! dsl-lang   (parser, compiler, ops/IR)
//!     ↑
//! sem-os     (frontier, resolver, navigation)
//!     ↑
//! ob-poc / bpmn-lite  (apps)
//! ```
//!
//! ## What lives here
//!
//! - `constellation_map_def` — the authored slot/join/cardinality vocabulary
//!   used by both the DSL compiler (`dsl-lang`) and the SemOS resolver.
//!   Moved here from `sem_os_ontology` so `dsl-lang` can reference these
//!   types without depending on the SemOS layer.

pub(crate) mod constellation_map_def;
pub(crate) mod resolver_facts;
pub(crate) mod dag;

pub use constellation_map_def::{
    AuditClass, Cardinality, ClosureType, CompletenessAssertionConfig, ConstellationMapDefBody,
    DependencyEntry, EligibilityConstraint, JoinDef, RoleGuard, SlotDef, SlotType,
    VerbPaletteEntry, VerbAvailability, SeedConstellationMap,
    SlotKey, GatingStatus, SlotGatingState,
};
pub use resolver_facts::StructuralFacts;

pub use dag::{
    Dag, LoadedDag, Slot, SlotStateMachine, StateMachine, StateDef, TransitionDef, Severity,
    CrossWorkspaceConstraint, DerivedCrossWorkspaceState, CascadeRule, EntryVia, Phase,
    StateSelector, PredicateBinding, DerivationCondition, ParentSlot, ParentJoin,
    StateDependency, DualLifecycle, PeriodicReviewCadence, RiskTierOverride, ReviewScope,
    EvidenceType, CategoryGated, ProductModuleGates, ConditionalGate, PruneCascadeRule,
    PruneCascadeTarget, PrunePreValidation, ExpectedLifetime, PredicateBindingSourceKind,
};

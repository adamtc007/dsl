//! DAG topology structures.
use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Dag {
    #[serde(default)]
    pub version: String,
    pub workspace: String,
    pub dag_id: String,

    #[serde(default)]
    pub overall_lifecycle: Option<OverallLifecycle>,

    #[serde(default)]
    pub slots: Vec<Slot>,

    #[serde(default)]
    pub cross_slot_constraints: Vec<CrossSlotConstraint>,

    // --- v1.3 additions ---
    #[serde(default)]
    pub cross_workspace_constraints: Vec<CrossWorkspaceConstraint>,

    #[serde(default)]
    pub derived_cross_workspace_state: Vec<DerivedCrossWorkspaceState>,

    #[serde(default)]
    pub evidence_types: Vec<EvidenceType>,

    /// Canonical DSL verbs owned by this DAG but not necessarily represented
    /// as lifecycle transitions. Used by reconciliation harnesses to keep
    /// verb registry, packs, macros, and DAG taxonomy aligned.
    #[serde(default)]
    pub dsl_verb_reconciliation: BTreeMap<String, Vec<String>>,

    // --- existing sections ---
    #[serde(default)]
    pub product_module_gates: Option<ProductModuleGates>,

    #[serde(default)]
    pub out_of_scope: Vec<String>,

    #[serde(default)]
    pub prune_cascade_rules: Vec<PruneCascadeRule>,

    #[serde(default)]
    pub prune_pre_validation: Option<PrunePreValidation>,
}

// =============================================================================
// OVERALL LIFECYCLE (v1.2-2)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OverallLifecycle {
    pub id: String,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub derived: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub phases: Vec<Phase>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Phase {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub derivation: Option<Derivation>,
    /// Verbs that can drive progression out of this phase. Tolerant to
    /// list form (canonical) OR free-text string (documentation escape
    /// used in some workspaces, e.g. KYC "remediation" phase).
    #[serde(default)]
    pub progression_verbs: ProgressionVerbs,
    #[serde(default)]
    pub next_phase: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProgressionVerbs {
    List(Vec<String>),
    Text(String),
}

impl Default for ProgressionVerbs {
    fn default() -> Self {
        ProgressionVerbs::List(Vec::new())
    }
}

/// Phase derivation clause. Tolerant to free-form strings (current v1.2
/// style) and structured workspace-slot-state entries (v1.3 aggregate
/// derivation). A single clause may mix both.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Derivation {
    #[serde(default)]
    pub all_of: Vec<DerivationCondition>,
    #[serde(default)]
    pub any_of: Vec<DerivationCondition>,
}

/// A derivation condition — either a free-text SQL-like predicate (v1.2)
/// or a structured workspace/slot/state reference (v1.3 Mode B).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DerivationCondition {
    Raw(String),
    Structured(StructuredDerivationCondition),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuredDerivationCondition {
    pub workspace: String,
    pub slot: String,
    #[serde(default)]
    pub state: Option<StateSelector>,
    #[serde(default)]
    pub predicate: Option<String>,
}

/// State selector — a single state ID or a set of allowable states.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum StateSelector {
    Single(String),
    Set(Vec<String>),
}

use crate::{
    AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
};

// =============================================================================
// SLOTS
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Slot {
    pub id: String,

    #[serde(default)]
    pub stateless: bool,

    #[serde(default)]
    pub rationale: Option<String>,

    /// Per-slot state machine (absent for stateless slots).
    #[serde(default)]
    pub state_machine: Option<SlotStateMachine>,

    // v1.2 product gating
    #[serde(default)]
    pub requires_products: Vec<String>,

    // Phase 1.5B gate metadata
    #[serde(default)]
    pub closure: Option<ClosureType>,

    #[serde(default)]
    pub eligibility: Option<EligibilityConstraint>,

    #[serde(default)]
    pub cardinality_max: Option<u64>,

    #[serde(default)]
    pub entry_state: Option<String>,

    #[serde(default)]
    pub attachment_predicates: Vec<String>,

    #[serde(default)]
    pub addition_predicates: Vec<String>,

    #[serde(default)]
    pub aggregate_breach_checks: Vec<String>,

    #[serde(
        default,
        rename = "+attachment_predicates",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additive_attachment_predicates: Vec<String>,

    #[serde(
        default,
        rename = "+addition_predicates",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additive_addition_predicates: Vec<String>,

    #[serde(
        default,
        rename = "+aggregate_breach_checks",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additive_aggregate_breach_checks: Vec<String>,

    #[serde(default)]
    pub role_guard: Option<RoleGuard>,

    #[serde(default)]
    pub justification_required: Option<bool>,

    #[serde(default)]
    pub audit_class: Option<AuditClass>,

    #[serde(default)]
    pub completeness_assertion: Option<CompletenessAssertionConfig>,

    // v1.3 additions
    #[serde(default)]
    pub parent_slot: Option<ParentSlot>,

    #[serde(default)]
    pub state_dependency: Option<StateDependency>,

    #[serde(default)]
    pub dual_lifecycle: Vec<DualLifecycle>,

    #[serde(default)]
    pub periodic_review_cadence: Option<PeriodicReviewCadence>,

    #[serde(default)]
    pub category_gated: Option<CategoryGated>,

    /// Explicit opt-out of the SUSPENDED-universal lint (V1.3-4).
    #[serde(default)]
    pub suspended_state_exempt: bool,

    /// Additional free-form metadata retained for forward compatibility
    /// (e.g. `product_gates:`, `cross_workspace_gate:` used by KYC DAG).
    #[serde(flatten)]
    pub extra: BTreeMap<String, YamlValue>,
}

/// Slot-level state machine. `state_machine:` may be a full block OR a
/// string reference ("reconcile-existing" form) — we accept both.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SlotStateMachine {
    Structured(Box<StateMachine>),
    Reference(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateMachine {
    pub id: String,
    #[serde(default)]
    pub source_entity: Option<String>,
    #[serde(default)]
    pub state_column: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub description: Option<String>,

    /// Predicate entity bindings used by `green_when` evaluation.
    ///
    /// These bindings tell the Frontier/evaluator how names that appear in
    /// state predicates map onto substrate rows. The field is optional so
    /// existing DAG YAML remains loadable while authored predicates are
    /// migrated from free text to machine evaluation.
    #[serde(default)]
    pub predicate_bindings: Vec<PredicateBinding>,

    #[serde(default)]
    pub states: Vec<StateDef>,

    #[serde(default)]
    pub transitions: Vec<TransitionDef>,

    #[serde(default)]
    pub terminal_states: Vec<String>,

    // v1.3 additions
    #[serde(default)]
    pub expected_lifetime: Option<ExpectedLifetime>,

    /// Ownership label for the lifecycle — governance/KYC artefact per
    /// OQ-3 resolution (2026-04-24). Runtime does NOT enforce.
    #[serde(default)]
    pub owner: Option<String>,

    #[serde(default)]
    pub note: Option<String>,

    /// Tolerate additional author-supplied metadata (description fields,
    /// etc.) without failing the load.
    #[serde(flatten)]
    pub extra: BTreeMap<String, YamlValue>,
}

/// Substrate binding for one entity kind named by a `green_when` predicate.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PredicateBinding {
    /// Predicate entity kind as authored, e.g. `red_flag`.
    pub entity: String,

    /// Whether this binding names a concrete substrate carrier or a DAG-level entity.
    #[serde(default)]
    pub source_kind: PredicateBindingSourceKind,

    /// Backing table or view, e.g. `"ob-poc".red_flags`.
    #[serde(default)]
    pub source_entity: Option<String>,

    /// Column carrying the entity's lifecycle state.
    #[serde(default)]
    pub state_column: Option<String>,

    /// Column carrying a scalar value for attribute predicates.
    #[serde(default)]
    pub value_column: Option<String>,

    /// Primary key or stable identifier column for diagnostics.
    #[serde(default)]
    pub id_column: Option<String>,

    /// Human-readable scope label, e.g. `attached_to this UBO`.
    #[serde(default)]
    pub scope: Option<String>,

    /// Column on this entity that points back to the parent/current row.
    #[serde(default)]
    pub parent_key: Option<String>,

    /// Column on the parent/current row used by `parent_key`.
    #[serde(default)]
    pub child_key: Option<String>,

    /// Required-universe binding for `every required <entity> exists`.
    #[serde(default)]
    pub required_universe: Option<PredicateRequiredUniverse>,

    /// Whether shape rules may replace this binding rather than tighten it.
    #[serde(default)]
    pub replaceable_by_shape: bool,

    /// Forward-compatible metadata for schema-specific binding details.
    #[serde(flatten)]
    pub extra: BTreeMap<String, YamlValue>,
}

/// Universe of required rows for a `required` predicate binding.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PredicateRequiredUniverse {
    /// Whether the required universe is a concrete carrier or a DAG-level set.
    #[serde(default)]
    pub source_kind: PredicateBindingSourceKind,

    /// Table or view that declares required entities.
    pub source_entity: String,

    /// Identifier column in the required-universe source.
    #[serde(default)]
    pub id_column: Option<String>,

    /// Column identifying the required entity kind.
    #[serde(default)]
    pub required_column: Option<String>,

    /// Column linking the required universe to the current parent row.
    #[serde(default)]
    pub parent_key: Option<String>,

    /// Column on the current parent row used by `parent_key`.
    #[serde(default)]
    pub child_key: Option<String>,

    /// Forward-compatible metadata for domain-specific required-set rules.
    #[serde(flatten)]
    pub extra: BTreeMap<String, YamlValue>,
}

/// Source kind for a predicate binding.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PredicateBindingSourceKind {
    /// Concrete substrate table/view binding.
    #[default]
    Substrate,

    /// DAG-declared entity whose carrier may be resolved later.
    DagEntity,

    /// DSL/runtime fact produced by a verb or resolver.
    DslFact,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedLifetime {
    ShortLived,
    LongLived,
    Ephemeral,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateDef {
    pub id: String,
    #[serde(default)]
    pub entry: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_via: Option<EntryVia>,
    /// v1.4 (D-016, 2026-04-29): the green-switch predicate. State is
    /// "green" when this predicate holds — entity in this state is
    /// considered satisfied/ready. Verbs that move INTO this state are
    /// expected to have made changes that flip the predicate true; the
    /// destination state's green_when IS the postcondition of the
    /// transition. Optional: states without a green_when are permissive
    /// (no entry test).
    #[serde(default)]
    pub green_when: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntryVia {
    Verb,
    Cascade { parent: String },
    Trigger { name: String },
    Scheduler { name: String },
    Signal { source: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitionDef {
    pub from: YamlValue, // accepts "STATE" or "(STATE_A, STATE_B)" string or list
    pub to: String,
    #[serde(default)]
    pub via: Option<YamlValue>, // accepts single verb or list
    #[serde(default)]
    pub precondition: Option<String>,
    #[serde(default)]
    pub args: Option<YamlValue>,
}

// =============================================================================
// CROSS-SLOT CONSTRAINTS (v1.2)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CrossSlotConstraint {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub rule: Option<String>,
    #[serde(default)]
    pub severity: Severity,

    #[serde(default)]
    pub v1_3_candidate: bool,

    #[serde(default)]
    pub enforced_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Informational,
}

// =============================================================================
// V1.3-1 CROSS-WORKSPACE CONSTRAINTS (Mode A blocking)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CrossWorkspaceConstraint {
    pub id: String,
    #[serde(default)]
    pub description: Option<String>,

    pub source_workspace: String,
    pub source_slot: String,

    #[serde(default)]
    pub source_state: Option<StateSelector>,

    #[serde(default)]
    pub source_predicate: Option<String>,

    pub target_workspace: String,
    pub target_slot: String,

    /// e.g. "STATE_A -> STATE_B" or "* -> STATE_B"
    pub target_transition: String,

    #[serde(default)]
    pub severity: Severity,

    /// Whether shape rules may structurally replace this constraint.
    #[serde(default)]
    pub replaceable_by_shape: bool,
}

// =============================================================================
// V1.3-2 DERIVED CROSS-WORKSPACE STATE (Mode B aggregation / tollgate)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DerivedCrossWorkspaceState {
    pub id: String,
    #[serde(default)]
    pub description: Option<String>,

    pub host_workspace: String,
    pub host_slot: String,
    pub host_state: String,

    pub derivation: Derivation,

    #[serde(default)]
    pub exposure: Option<ExposureConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExposureConfig {
    #[serde(default)]
    pub visible_as: Option<Visibility>,

    /// Per OQ-2 resolution (2026-04-24): cache scope = session /
    /// workspace-context. This flag is retained for future granularity
    /// but the runtime default is session-cached unless explicitly false.
    #[serde(default = "default_cacheable")]
    pub cacheable: bool,
}

fn default_cacheable() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    FirstClassState,
    Annotation,
}

// =============================================================================
// V1.3-3 PARENT SLOT + STATE DEPENDENCY (Mode C hierarchy)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParentSlot {
    #[serde(default)]
    pub workspace: Option<String>, // defaults to same workspace
    pub slot: String,
    #[serde(default)]
    pub join: Option<ParentJoin>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParentJoin {
    #[serde(default)]
    pub via: Option<String>,
    #[serde(default)]
    pub parent_fk: Option<String>,
    #[serde(default)]
    pub child_fk: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateDependency {
    #[serde(default)]
    pub cascade_rules: Vec<CascadeRule>,
    #[serde(default)]
    pub severity: Severity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CascadeRule {
    pub parent_state: String,
    pub child_allowed_states: Vec<String>,
    #[serde(default)]
    pub cascade_on_parent_transition: bool,
    #[serde(default)]
    pub default_child_state_on_cascade: Option<String>,
}

// =============================================================================
// V1.3-5 DUAL LIFECYCLE
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DualLifecycle {
    pub id: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Governance artefact — does NOT enforce at runtime (OQ-3).
    #[serde(default)]
    pub owner: Option<String>,
    pub junction_state_from_primary: String,
    #[serde(default)]
    pub states: Vec<StateDef>,
    #[serde(default)]
    pub transitions: Vec<TransitionDef>,
    #[serde(default)]
    pub terminal_states: Vec<String>,
}

// =============================================================================
// V1.3-6 PERIODIC REVIEW CADENCE + EVIDENCE TYPE VALIDITY WINDOWS
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PeriodicReviewCadence {
    pub base_window: String, // ISO 8601 duration: "P1Y", "P2Y", etc.
    #[serde(default)]
    pub risk_tiered_overrides: Vec<RiskTierOverride>,
    #[serde(default)]
    pub review_scope: Option<ReviewScope>,
    #[serde(default)]
    pub scheduler_hook: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskTierOverride {
    pub risk_tier: String,
    pub window: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewScope {
    Full,
    Partial,
}

/// An evidence-type reference entry. `validity_window` may be `"once"`
/// (special value — no refresh) or an ISO 8601 duration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidenceType {
    pub id: String,
    pub validity_window: String,
}

// =============================================================================
// V1.3-8 CATEGORY GATED
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryGated {
    pub category_column: String,
    pub category_source: String, // table name, e.g. "cbus"
    #[serde(default)]
    pub activated_by: Vec<String>,
    #[serde(default)]
    pub deactivated_by: Vec<String>,
    #[serde(default)]
    pub lifecycle_variant_map: HashMap<String, String>,
}

// =============================================================================
// PRODUCT MODULE GATES (v1.2)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ProductModuleGates {
    #[serde(default)]
    pub always_on: Vec<String>,
    #[serde(default)]
    pub conditionally_on: Vec<ConditionalGate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConditionalGate {
    pub slot: String,
    #[serde(default)]
    pub activated_by: Vec<String>,
    #[serde(default)]
    pub rationale: Option<String>,
}

// =============================================================================
// PRUNE SEMANTICS (v1.2-4)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PruneCascadeRule {
    pub id: String,
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub cascades_to: Vec<PruneCascadeTarget>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PruneCascadeTarget {
    pub target: String,
    #[serde(default)]
    pub rule: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrunePreValidation {
    #[serde(default)]
    pub required_verbs: Vec<String>,
    #[serde(default)]
    pub abort_conditions: Vec<String>,
}

// =============================================================================
// LOADER
// =============================================================================

#[derive(Debug, Clone)]
pub struct LoadedDag {
    pub source_path: PathBuf,
    pub dag: Dag,
}

# dsl-runtime E0603/E0170 Re-Path Review Document

## 1. COMMANDS RUN
1. `cargo check -p dsl-runtime` (before repath edits)
2. Inventory ripgrep commands for the following types:
   - `EffectClass`
   - `DerivationCondition`
   - `DerivedCrossWorkspaceState`
   - `StateSelector`
   - `CrossWorkspaceConstraint`
   - `CascadeRule`
   - `Severity`
   - `DagSeverity`
3. `rg -n 'EffectClass|...|DagSeverity' /Users/adamtc007/Dev/dsl/crates/dsl-core/src/lib.rs` (facade reachability check)
4. Consumer edits in `ob-poc/rust` workspace
5. `cargo check -p dsl-runtime` (after repath edits)
6. `cargo check --workspace --exclude dsl-runtime --exclude dsl-lsp --exclude ob-poc --exclude ob-poc-web --exclude ob-poc-agent --exclude sem_os_harness --exclude sem_os_postgres --exclude sem_os_server --exclude xtask --all-features`
7. `git diff`
8. `git status --porcelain`

---

## 2. INVENTORY

### A. Cargo Check Output Before Edits
```
$ cargo check -p dsl-runtime
warning: unused imports: `CategoryGated`, `ConditionalGate`, `DualLifecycle`, `EvidenceType`, `ParentJoin`, `ParentSlot`, `PeriodicReviewCadence`, `ProductModuleGates`, `PruneCascadeRule`, `PruneCascadeTarget`, `PrunePreValidation`, `ReviewScope`, `RiskTierOverride`, `StateDef`, `StateDependency`, `StateMachine`, and `TransitionDef`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag.rs:26:5
   |
26 |     StateMachine, StateDef, TransitionDef, ParentSlot, ParentJoin, StateDependency,
   |     ^^^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^
27 |     PeriodicReviewCadence, RiskTierOverride, ReviewScope, EvidenceType, CategoryGated,
   |     ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^
28 |     ProductModuleGates, ConditionalGate, PruneCascadeRule, PruneCascadeTarget,
   |     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^
29 |     PrunePreValidation, ExpectedLifetime, PredicateBindingSourceKind, DualLifecycle,
   |     ^^^^^^^^^^^^^^^^^^                                                ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `AuditClass`, `CompletenessAssertionConfig`, and `RoleGuard`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag.rs:34:5
   |
34 |     AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
   |     ^^^^^^^^^^               ^^^^^^^^^^^^^^^^^^^^^^^^^^^                         ^^^^^^^^^

warning: unused imports: `EntityQualifier` and `RelationScope`
 --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:6:22
  |
6 | pub(crate) use ast::{EntityQualifier, RelationScope};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `ast::State`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:11:16
   |
11 | pub(crate) use ast::State;
   |                ^^^^^^^^^^

warning: unused import: `parser::ParseError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:13:16
   |
13 | pub(crate) use parser::ParseError;
   |                ^^^^^^^^^^^^^^^^^^

warning: unused imports: `DagWarning` and `validate_constellation_map_schema_coordination`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:39:5
   |
39 |     validate_constellation_map_schema_coordination, DagWarning,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^

warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:46:5
   |
46 |     green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
47 |     GreenWhenExclusionReason,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `EvaluationContext` and `compute_effective_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:51:5
   |
51 |     compute_effective_tier, EvaluationContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:56:5
   |
56 |     compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
   |     ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `validator::StructuralError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:71:16
   |
71 | pub(crate) use validator::StructuralError;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `executable_plan::TransactionPolicy`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:82:16
   |
82 | pub(crate) use executable_plan::TransactionPolicy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `DagWarning` is more private than the item `DagValidationReport::warnings`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:476:5
    |
476 |     pub warnings: Vec<DagWarning>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `DagValidationReport::warnings` is reachable at visibility `pub`
    |
note: but type `DagWarning` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:386:1
    |
386 | pub(crate) enum DagWarning {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(private_interfaces)]` on by default

warning: type `RelationScope` is more private than the item `predicate::ast::EntityRef::Scoped::scope`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:76:9
    |
 76 |         scope: RelationScope,
    |         ^^^^^^^^^^^^^^^^^^^^ field `predicate::ast::EntityRef::Scoped::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `EntityQualifier` is more private than the item `EntitySetRef::qualifier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:86:5
   |
86 |     pub qualifier: Option<EntityQualifier>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::qualifier` is reachable at visibility `pub`
   |
note: but type `EntityQualifier` is only usable at visibility `pub(crate)`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:93:1
   |
93 | pub(crate) enum EntityQualifier {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `RelationScope` is more private than the item `EntitySetRef::scope`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:88:5
    |
 88 |     pub scope: Option<RelationScope>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `StructuralError` is more private than the item `ValidationReport::structural`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:295:5
    |
295 |     pub structural: Vec<StructuralError>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `ValidationReport::structural` is reachable at visibility `pub`
    |
note: but type `StructuralError` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:72:1
    |
 72 | pub(crate) enum StructuralError {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:53:19
   |
51 | impl Program {
   | ------------ methods in this implementation
52 |     /// Render the program back to DSL source (for execution - shows UUIDs when resolved)
53 |     pub(crate) fn to_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^
...
63 |     pub(crate) fn to_user_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:81:19
   |
79 | impl Statement {
   | -------------- methods in this implementation
80 |     /// Render the statement back to DSL source (for execution)
81 |     pub(crate) fn to_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^
...
89 |     pub(crate) fn to_user_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:110:19
    |
108 | impl VerbCall {
    | ------------- methods in this implementation
109 |     /// Render the verb call back to DSL source (for execution - shows UUIDs)
110 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
126 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^

warning: methods `get_arg` and `get_value` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:149:19
    |
142 | impl VerbCall {
    | ------------- methods in this implementation
...
149 |     pub(crate) fn get_arg(&self, key: &str) -> Option<&Argument> {
    |                   ^^^^^^^
...
154 |     pub(crate) fn get_value(&self, key: &str) -> Option<&AstNode> {
    |                   ^^^^^^^^^

warning: multiple associated items are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:293:19
    |
287 | impl AstNode {
    | ------------ associated items in this implementation
...
293 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
329 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^
...
362 |     pub(crate) fn integer(i: i64) -> Self {
    |                   ^^^^^^^
...
385 |     pub(crate) fn resolved_entity_ref(
    |                   ^^^^^^^^^^^^^^^^^^^
...
404 |     pub(crate) fn symbol_ref(name: impl Into<String>, span: Span) -> Self {
    |                   ^^^^^^^^^^
...
427 |     pub(crate) fn is_resolved_entity_ref(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^^^^^^
...
439 |     pub(crate) fn is_symbol_ref(&self) -> bool {
    |                   ^^^^^^^^^^^^^
...
444 |     pub(crate) fn is_literal(&self) -> bool {
    |                   ^^^^^^^^^^
...
453 |     pub(crate) fn as_string(&self) -> Option<&str> {
    |                   ^^^^^^^^^
...
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |                   ^^^^^^^
...
475 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
...
491 |     pub(crate) fn as_integer(&self) -> Option<i64> {
    |                   ^^^^^^^^^^
...
499 |     pub(crate) fn as_decimal(&self) -> Option<Decimal> {
    |                   ^^^^^^^^^^
...
508 |     pub(crate) fn as_boolean(&self) -> Option<bool> {
    |                   ^^^^^^^^^^
...
516 |     pub(crate) fn as_list(&self) -> Option<&[AstNode]> {
    |                   ^^^^^^^
...
524 |     pub(crate) fn as_map(&self) -> Option<&[(String, AstNode)]> {
    |                   ^^^^^^
...
532 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
554 |     pub(crate) fn with_resolved_key(&self, key: String) -> Self {
    |                   ^^^^^^^^^^^^^^^^^
...
564 |     pub(crate) fn try_with_resolved_key(&self, key: String) -> Result<Self, String> {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:626:19
    |
624 | impl Literal {
    | ------------ method in this implementation
625 |     /// Render the literal back to DSL source
626 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: associated function `merge` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:657:19
    |
651 | impl Span {
    | --------- associated function in this implementation
...
657 |     pub(crate) fn merge(a: Span, b: Span) -> Span {
    |                   ^^^^^

warning: function `find_unresolved_refs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:741:15
    |
741 | pub(crate) fn find_unresolved_refs(program: &Program) -> Vec<&AstNode> {
    |               ^^^^^^^^^^^^^^^^^^^^

warning: methods `is_fully_resolved`, `resolved_count`, and `resolution_percentage` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:775:19
    |
773 | impl EntityRefStats {
    | ------------------- methods in this implementation
774 |     /// Returns true if all EntityRefs are resolved
775 |     pub(crate) fn is_fully_resolved(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^
...
780 |     pub(crate) fn resolved_count(&self) -> i32 {
    |                   ^^^^^^^^^^^^^^
...
785 |     pub(crate) fn resolution_percentage(&self) -> u8 {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: methods `span`, `verb_name`, and `to_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:879:19
    |
877 | impl ViewportVerb {
    | ----------------- methods in this implementation
878 |     /// Get the span of this verb
879 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
893 |     pub(crate) fn verb_name(&self) -> &'static str {
    |                   ^^^^^^^^^
...
907 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:968:19
    |
966 | impl FocusTarget {
    | ---------------- methods in this implementation
967 |     /// Get the span of this target
968 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
982 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1022:19
     |
1020 | impl EnhanceArg {
     | --------------- method in this implementation
1021 |     /// Render the argument to DSL string
1022 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1048:19
     |
1046 | impl NavTarget {
     | -------------- methods in this implementation
1047 |     /// Get the span of this target
1048 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1057 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1079:19
     |
1077 | impl NavDirection {
     | ----------------- method in this implementation
1078 |     /// Render the direction to DSL string
1079 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: associated items `to_dsl_string` and `all` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1134:19
     |
1132 | impl ViewType {
     | ------------- associated items in this implementation
1133 |     /// Render the view type to DSL string
1134 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1161 |     pub(crate) fn all() -> &'static [ViewType] {
     |                   ^^^

warning: associated items `to_dsl_string`, `min_confidence`, and `from_score` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1198:19
     |
1196 | impl ConfidenceZone {
     | ------------------- associated items in this implementation
1197 |     /// Render the zone to DSL string
1198 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1219 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^
...
1229 |     pub(crate) fn from_score(score: f32) -> Self {
     |                   ^^^^^^^^^^

warning: methods `to_dsl_string`, `extension`, and `mime_type` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1260:19
     |
1258 | impl ExportFormat {
     | ----------------- methods in this implementation
1259 |     /// Render the format to DSL string
1260 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1281 |     pub(crate) fn extension(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1291 |     pub(crate) fn mime_type(&self) -> &'static str {
     |                   ^^^^^^^^^

warning: methods `merge`, `names`, and `available_types` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/binding_context.rs:94:19
    |
 88 | impl BindingContext {
    | ------------------- methods in this implementation
...
 94 |     pub(crate) fn merge(&mut self, other: &BindingContext) {
    |                   ^^^^^
...
111 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^
...
126 |     pub(crate) fn available_types(&self) -> std::collections::HashSet<String> {
    |                   ^^^^^^^^^^^^^^^

warning: method `is_clean` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:480:19
    |
479 | impl DagValidationReport {
    | ------------------------ method in this implementation
480 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: enum `SchemaCoordinationKnownDeferred` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:495:17
    |
495 | pub(crate) enum SchemaCoordinationKnownDeferred {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `harden_schema_coordination_warnings` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:686:15
    |
686 | pub(crate) fn harden_schema_coordination_warnings(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_known_deferred_key` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:704:4
    |
704 | fn schema_coordination_known_deferred_key(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_warning_to_error` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:738:4
    |
738 | fn schema_coordination_warning_to_error(warning: DagWarning) -> DagError {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_source_name` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:772:4
    |
772 | fn schema_coordination_source_name(location: &DagLocation) -> String {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `EvaluationContext` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:29:19
   |
29 | pub(crate) struct EvaluationContext {
   |                   ^^^^^^^^^^^^^^^^^

warning: associated items `new`, `with_arg`, `with_entity_attr`, and `with_flag` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:40:19
   |
39 | impl EvaluationContext {
   | ---------------------- associated items in this implementation
40 |     pub(crate) fn new() -> Self {
   |                   ^^^
...
44 |     pub(crate) fn with_arg(mut self, name: impl Into<String>, value: serde_json::Value) -> Self {
   |                   ^^^^^^^^
...
49 |     pub(crate) fn with_entity_attr(
   |                   ^^^^^^^^^^^^^^^^
...
62 |     pub(crate) fn with_flag(mut self, flag: impl Into<String>, value: bool) -> Self {
   |                   ^^^^^^^^^

warning: function `evaluate_predicate` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:72:15
   |
72 | pub(crate) fn evaluate_predicate(pred: &EscalationPredicate, ctx: &EvaluationContext) -> bool {
   |               ^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:128:15
    |
128 | pub(crate) fn compute_effective_tier(
    |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier_with_trace` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:142:15
    |
142 | pub(crate) fn compute_effective_tier_with_trace<'a>(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `as_f64` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:160:4
    |
160 | fn as_f64(v: &serde_json::Value) -> Option<f64> {
    |    ^^^^^^

warning: struct `GreenWhenCoverageRow` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:12:19
   |
12 | pub(crate) struct GreenWhenCoverageRow {
   |                   ^^^^^^^^^^^^^^^^^^^^

warning: enum `GreenWhenExclusionReason` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:24:17
   |
24 | pub(crate) enum GreenWhenExclusionReason {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `GreenWhenCoverageSummary` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:31:19
   |
31 | pub(crate) struct GreenWhenCoverageSummary {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dags` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:39:15
   |
39 | pub(crate) fn green_when_coverage_for_dags(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dag` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:54:15
   |
54 | pub(crate) fn green_when_coverage_for_dag(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_summary` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:82:15
   |
82 | pub(crate) fn green_when_coverage_summary(rows: &[GreenWhenCoverageRow]) -> GreenWhenCoverageSummary {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `row_for_state` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:97:4
   |
97 | fn row_for_state(
   |    ^^^^^^^^^^^^^

warning: function `inbound_verbs_by_destination` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:135:4
    |
135 | fn inbound_verbs_by_destination(transitions: &[TransitionDef]) -> BTreeMap<String, Vec<String>> {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `verbs_from_transition` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:150:4
    |
150 | fn verbs_from_transition(transition: &TransitionDef) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^

warning: function `states_from_yaml_value` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:158:4
    |
158 | fn states_from_yaml_value(value: &YamlValue) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^^

warning: function `split_tupleish` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:169:4
    |
169 | fn split_tupleish(value: &str) -> Vec<String> {
    |    ^^^^^^^^^^^^^^

warning: associated functions `entity_uuid`, `entity_uuid_binding`, and `natural_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/resource_dependency.rs:96:19
    |
 94 | impl ResourceDependency {
    | ----------------------- associated functions in this implementation
 95 |     /// Construct an `EntityUuid` dependency with a known UUID.
 96 |     pub(crate) fn entity_uuid(entity_type: impl Into<String>, uuid: uuid::Uuid) -> Self {
    |                   ^^^^^^^^^^^
...
105 |     pub(crate) fn entity_uuid_binding(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^
...
113 |     pub(crate) fn natural_key(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^

warning: associated functions `compile_resolved_entity`, `binding_resolved_entity`, and `runtime_create_natural_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/resource_dependency.rs:169:19
    |
167 | impl ResolvedResourceDependency {
    | ------------------------------- associated functions in this implementation
168 |     /// An entity UUID that was resolved at compile time.
169 |     pub(crate) fn compile_resolved_entity(entity_type: impl Into<String>, uuid: uuid::Uuid) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
...
178 |     pub(crate) fn binding_resolved_entity(entity_type: impl Into<String>, slot: BindingSlotId) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
...
187 |     pub(crate) fn runtime_create_natural_key(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `RunbookStep` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:39:19
   |
39 | pub(crate) struct RunbookStep {
   |                   ^^^^^^^^^^^

warning: enum `AggregationRule` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:67:17
   |
67 | pub(crate) enum AggregationRule {
   |                 ^^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:93:19
    |
 92 | impl AggregationRule {
    | -------------------- methods in this implementation
 93 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^
...
101 |     pub(crate) fn tier(&self) -> ConsequenceTier {
    |                   ^^^^
...
109 |     pub(crate) fn matches(&self, steps: &[RunbookStep]) -> bool {
    |                   ^^^^^^^

warning: enum `CrossScopeRule` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:139:17
    |
139 | pub(crate) enum CrossScopeRule {
    |                 ^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:161:19
    |
160 | impl CrossScopeRule {
    | ------------------- methods in this implementation
161 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^
...
169 |     pub(crate) fn tier(&self) -> ConsequenceTier {
    |                   ^^^^
...
177 |     pub(crate) fn matches(&self, steps: &[RunbookStep]) -> bool {
    |                   ^^^^^^^

warning: function `compute_runbook_tier` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:212:15
    |
212 | pub(crate) fn compute_runbook_tier(
    |               ^^^^^^^^^^^^^^^^^^^^

warning: function `component_a` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:228:15
    |
228 | pub(crate) fn component_a(steps: &[RunbookStep]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_b` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:235:15
    |
235 | pub(crate) fn component_b(steps: &[RunbookStep], rules: &[AggregationRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_c` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:243:15
    |
243 | pub(crate) fn component_c(steps: &[RunbookStep], rules: &[CrossScopeRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: struct `CsgRulesConfig` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:26:19
   |
26 | pub(crate) struct CsgRulesConfig {
   |                   ^^^^^^^^^^^^^^

warning: methods `is_simple`, `discriminators`, and `min_confidence` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1398:19
     |
1376 | impl SearchKeyConfig {
     | -------------------- methods in this implementation
...
1398 |     pub(crate) fn is_simple(&self) -> bool {
     |                   ^^^^^^^^^
...
1417 |     pub(crate) fn discriminators(&self) -> &[SearchDiscriminator] {
     |                   ^^^^^^^^^^^^^^
...
1426 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^

warning: method `arg_name` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1791:19
     |
1789 | impl SearchDiscriminator {
     | ------------------------ method in this implementation
1790 |     /// Get the argument name (uses field name if from_arg not specified)
1791 |     pub(crate) fn arg_name(&self) -> &str {
     |                   ^^^^^^^^

warning: struct `ConstraintRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1910:19
     |
1910 | pub(crate) struct ConstraintRule {
     |                   ^^^^^^^^^^^^^^

warning: struct `WarningRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1920:19
     |
1920 | pub(crate) struct WarningRule {
     |                   ^^^^^^^^^^^

warning: struct `JurisdictionRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1932:19
     |
1932 | pub(crate) struct JurisdictionRule {
     |                   ^^^^^^^^^^^^^^^^

warning: struct `CompositeRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1944:19
     |
1944 | pub(crate) struct CompositeRule {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleCondition` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1955:19
     |
1955 | pub(crate) struct RuleCondition {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleRequirement` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1973:19
     |
1973 | pub(crate) struct RuleRequirement {
     |                   ^^^^^^^^^^^^^^^

warning: struct `JurisdictionCondition` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1983:19
     |
1983 | pub(crate) struct JurisdictionCondition {
     |                   ^^^^^^^^^^^^^^^^^^^^^

warning: struct `AppliesTo` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1995:19
     |
1995 | pub(crate) struct AppliesTo {
     |                   ^^^^^^^^^

warning: enum `RuleSeverity` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:2002:17
     |
2002 | pub(crate) enum RuleSeverity {
     |                 ^^^^^^^^^^^^

warning: variant `TransitionWithoutEdges` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:75:5
   |
72 | pub(crate) enum StructuralError {
   |                 --------------- variant in this enum
...
75 |     TransitionWithoutEdges(Location),
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `StructuralError` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: method `is_clean` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:301:19
    |
300 | impl ValidationReport {
    | --------------------- method in this implementation
301 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: associated function `warning` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/diagnostics.rs:150:19
    |
136 | impl Diagnostic {
    | --------------- associated function in this implementation
...
150 |     pub(crate) fn warning(code: DiagnosticCode, message: impl Into<String>) -> Self {
    |                   ^^^^^^^

warning: struct `PlanId` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:43:19
   |
43 | pub(crate) struct PlanId(pub(crate) Uuid);
   |                   ^^^^^^

warning: associated function `new` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:46:19
   |
45 | impl PlanId {
   | ----------- associated function in this implementation
46 |     pub(crate) fn new() -> Self {
   |                   ^^^

warning: struct `SemOsSnapshotId` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:70:19
   |
70 | pub(crate) struct SemOsSnapshotId(pub(crate) u64);
   |                   ^^^^^^^^^^^^^^^

warning: enum `TransactionPolicy` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:141:17
    |
141 | pub(crate) enum TransactionPolicy {
    |                 ^^^^^^^^^^^^^^^^^

warning: associated function `from_effect_classes` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:162:19
    |
156 | impl TransactionPolicy {
    | ---------------------- associated function in this implementation
...
162 |     pub(crate) fn from_effect_classes(classes: impl IntoIterator<Item = EffectClass>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^

warning: struct `AuthorityContext` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:203:19
    |
203 | pub(crate) struct AuthorityContext {
    |                   ^^^^^^^^^^^^^^^^

warning: enum `InstructionInput` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:240:17
    |
240 | pub(crate) enum InstructionInput {
    |                 ^^^^^^^^^^^^^^^^

warning: struct `RuntimeInstruction` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:259:19
    |
259 | pub(crate) struct RuntimeInstruction {
    |                   ^^^^^^^^^^^^^^^^^^

warning: struct `ExecutablePlan` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:311:19
    |
311 | pub(crate) struct ExecutablePlan {
    |                   ^^^^^^^^^^^^^^

warning: associated constant `FORMAT_VERSION` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:347:22
    |
345 | impl ExecutablePlan {
    | ------------------- associated constant in this implementation
346 |     /// Current plan format version.
347 |     pub(crate) const FORMAT_VERSION: u32 = 1;
    |                      ^^^^^^^^^^^^^^

warning: struct `ExecutionStepSummary` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:358:19
    |
358 | pub(crate) struct ExecutionStepSummary {
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: methods `imposes_order` and `ordering_pair` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:158:19
    |
153 | impl DagEdge {
    | ------------ methods in this implementation
...
158 |     pub(crate) fn imposes_order(&self) -> bool {
    |                   ^^^^^^^^^^^^^
...
168 |     pub(crate) fn ordering_pair(&self) -> Option<(NodeId, NodeId)> {
    |                   ^^^^^^^^^^^^^

warning: method `ordering_pairs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:208:19
    |
193 | impl PopulatedExecutionDag {
    | -------------------------- method in this implementation
...
208 |     pub(crate) fn ordering_pairs(&self) -> Vec<(usize, usize)> {
    |                   ^^^^^^^^^^^^^^

warning: `dsl-core` (lib) generated 96 warnings (run `cargo fix --lib -p dsl-core` to apply 11 suggestions)
    Checking dsl-runtime v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime)
error[E0603]: module `executable_plan` is private
  --> crates/dsl-runtime/src/coordination.rs:21:15
   |
21 | use dsl_core::executable_plan::EffectClass;
   |               ^^^^^^^^^^^^^^^  ----------- enum `EffectClass` is not publicly re-exported
   |               |
   |               private module
   |
note: the module `executable_plan` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:21:1
   |
21 | pub(crate) mod executable_plan;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
  --> crates/dsl-runtime/src/cross_workspace/derived_state.rs:24:15
   |
24 | use dsl_core::config::dag::{DerivationCondition, DerivedCrossWorkspaceState, StateSelector};
   |               ^^^^^^        ------------------- enum `DerivationCondition` is not publicly re-exported
   |               |
   |               private module
   |
note: the module `config` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
   |
19 | pub(crate) mod config;
   | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
  --> crates/dsl-runtime/src/cross_workspace/derived_state.rs:24:15
   |
24 | use dsl_core::config::dag::{DerivationCondition, DerivedCrossWorkspaceState, StateSelector};
   |               ^^^^^^  --- module `dag` is not publicly re-exported
   |               |
   |               private module
   |
note: the module `config` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
   |
19 | pub(crate) mod config;
   | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
  --> crates/dsl-runtime/src/cross_workspace/derived_state.rs:24:15
   |
24 | use dsl_core::config::dag::{DerivationCondition, DerivedCrossWorkspaceState, StateSelector};
   |               ^^^^^^ private module                                          ------------- enum `StateSelector` is not publicly re-exported
   |
note: the module `config` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
   |
19 | pub(crate) mod config;
   | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
  --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:40:15
   |
40 | use dsl_core::config::dag::{CrossWorkspaceConstraint, StateSelector};
   |               ^^^^^^  --- module `dag` is not publicly re-exported
   |               |
   |               private module
   |
note: the module `config` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
   |
19 | pub(crate) mod config;
   | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
  --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:40:15
   |
40 | use dsl_core::config::dag::{CrossWorkspaceConstraint, StateSelector};
   |               ^^^^^^ private module                   ------------- enum `StateSelector` is not publicly re-exported
   |
note: the module `config` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
   |
19 | pub(crate) mod config;
   | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:277:19
    |
277 |     use dsl_core::config::dag::Severity::*;
    |                   ^^^^^^       -------- enum `Severity` is not publicly re-exported
    |                   |
    |                   private module
    |
note: the module `config` is defined here
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
    |
 19 | pub(crate) mod config;
    | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
  --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:31:15
   |
31 | use dsl_core::config::dag::CascadeRule;
   |               ^^^^^^  --- module `dag` is not publicly re-exported
   |               |
   |               private module
   |
note: the module `config` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
   |
19 | pub(crate) mod config;
   | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:208:19
    |
208 |     use dsl_core::config::dag::Severity::*;
    |                   ^^^^^^       -------- enum `Severity` is not publicly re-exported
    |                   |
    |                   private module
    |
note: the module `config` is defined here
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
    |
 19 | pub(crate) mod config;
    | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:276:31
    |
276 | fn severity_str(s: &dsl_core::config::dag::Severity) -> String {
    |                               ^^^^^^       -------- enum `Severity` is not publicly re-exported
    |                               |
    |                               private module
    |
note: the module `config` is defined here
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
    |
 19 | pub(crate) mod config;
    | ^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `config` is private
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:207:31
    |
207 | fn severity_str(s: &dsl_core::config::dag::Severity) -> String {
    |                               ^^^^^^       -------- enum `Severity` is not publicly re-exported
    |                               |
    |                               private module
    |
note: the module `config` is defined here
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:19:1
    |
 19 | pub(crate) mod config;
    | ^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `DualLifecycle`, `Slot`, and `StateSelector`
  --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:28:64
   |
28 |     CrossWorkspaceConstraint, Dag, DerivedCrossWorkspaceState, DualLifecycle, LoadedDag,
   |                                                                ^^^^^^^^^^^^^
29 |     ParentSlot, Slot, SlotStateMachine, StateDependency, StateSelector, TransitionDef,
   |                 ^^^^                                     ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0170]: pattern binding `Error` is named the same as one of the variants of the type `dsl_core::DagSeverity`
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:279:9
    |
279 |         Error => "error",
    |         ^^^^^ help: to match on the variant, qualify the path: `dsl_core::DagSeverity::Error`
    |
    = note: `#[deny(bindings_with_variant_name)]` on by default

error[E0170]: pattern binding `Warning` is named the same as one of the variants of the type `dsl_core::DagSeverity`
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:280:9
    |
280 |         Warning => "warning",
    |         ^^^^^^^ help: to match on the variant, qualify the path: `dsl_core::DagSeverity::Warning`

error[E0170]: pattern binding `Informational` is named the same as one of the variants of the type `dsl_core::DagSeverity`
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:281:9
    |
281 |         Informational => "informational",
    |         ^^^^^^^^^^^^^ help: to match on the variant, qualify the path: `dsl_core::DagSeverity::Informational`

warning: unreachable pattern
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:280:9
    |
279 |         Error => "error",
    |         ----- matches any value
280 |         Warning => "warning",
    |         ^^^^^^^ no value can reach this
    |
    = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default

warning: unreachable pattern
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:281:9
    |
279 |         Error => "error",
    |         ----- matches any value
280 |         Warning => "warning",
281 |         Informational => "informational",
    |         ^^^^^^^^^^^^^ no value can reach this

warning: unused variable: `Error`
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:279:9
    |
279 |         Error => "error",
    |         ^^^^^
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
help: you might have meant to pattern match on the similarly named variant `Error`
    |
279 |         dsl_core::DagSeverity::Error => "error",
    |         +++++++++++++++++++++++
help: if this is intentional, prefix it with an underscore
    |
279 |         _Error => "error",
    |         +

warning: unused variable: `Warning`
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:280:9
    |
280 |         Warning => "warning",
    |         ^^^^^^^
    |
help: you might have meant to pattern match on the similarly named variant `Warning`
    |
280 |         dsl_core::DagSeverity::Warning => "warning",
    |         +++++++++++++++++++++++
help: if this is intentional, prefix it with an underscore
    |
280 |         _Warning => "warning",
    |         +

warning: unused variable: `Informational`
   --> crates/dsl-runtime/src/cross_workspace/gate_checker.rs:281:9
    |
281 |         Informational => "informational",
    |         ^^^^^^^^^^^^^
    |
help: you might have meant to pattern match on the similarly named variant `Informational`
    |
281 |         dsl_core::DagSeverity::Informational => "informational",
    |         +++++++++++++++++++++++
help: if this is intentional, prefix it with an underscore
    |
281 |         _Informational => "informational",
    |         +

error[E0170]: pattern binding `Error` is named the same as one of the variants of the type `dsl_core::DagSeverity`
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:210:9
    |
210 |         Error => "error",
    |         ^^^^^ help: to match on the variant, qualify the path: `dsl_core::DagSeverity::Error`

error[E0170]: pattern binding `Warning` is named the same as one of the variants of the type `dsl_core::DagSeverity`
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:211:9
    |
211 |         Warning => "warning",
    |         ^^^^^^^ help: to match on the variant, qualify the path: `dsl_core::DagSeverity::Warning`

error[E0170]: pattern binding `Informational` is named the same as one of the variants of the type `dsl_core::DagSeverity`
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:212:9
    |
212 |         Informational => "informational",
    |         ^^^^^^^^^^^^^ help: to match on the variant, qualify the path: `dsl_core::DagSeverity::Informational`

warning: unreachable pattern
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:211:9
    |
210 |         Error => "error",
    |         ----- matches any value
211 |         Warning => "warning",
    |         ^^^^^^^ no value can reach this

warning: unreachable pattern
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:212:9
    |
210 |         Error => "error",
    |         ----- matches any value
211 |         Warning => "warning",
212 |         Informational => "informational",
    |         ^^^^^^^^^^^^^ no value can reach this

warning: unused variable: `Error`
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:210:9
    |
210 |         Error => "error",
    |         ^^^^^
    |
help: you might have meant to pattern match on the similarly named variant `Error`
    |
210 |         dsl_core::DagSeverity::Error => "error",
    |         +++++++++++++++++++++++
help: if this is intentional, prefix it with an underscore
    |
210 |         _Error => "error",
    |         +

warning: unused variable: `Warning`
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:211:9
    |
211 |         Warning => "warning",
    |         ^^^^^^^
    |
help: you might have meant to pattern match on the similarly named variant `Warning`
    |
211 |         dsl_core::DagSeverity::Warning => "warning",
    |         +++++++++++++++++++++++
help: if this is intentional, prefix it with an underscore
    |
211 |         _Warning => "warning",
    |         +

warning: unused variable: `Informational`
   --> crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:212:9
    |
212 |         Informational => "informational",
    |         ^^^^^^^^^^^^^
    |
help: you might have meant to pattern match on the similarly named variant `Informational`
    |
212 |         dsl_core::DagSeverity::Informational => "informational",
    |         +++++++++++++++++++++++
help: if this is intentional, prefix it with an underscore
    |
212 |         _Informational => "informational",
    |         +

Some errors have detailed explanations: E0170, E0603.
For more information about an error, try `rustc --explain E0170`.
warning: `dsl-runtime` (lib) generated 11 warnings
error: could not compile `dsl-runtime` (lib) due to 17 previous errors; 11 warnings emitted
```

### B. Definition Homes for Private-Import Types

```
$ rg -n 'pub (struct|enum|type) EffectClass\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl-core/src/executable_plan.rs:95:pub enum EffectClass {
```

```
$ rg -n 'pub (struct|enum|type) DerivationCondition\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs:116:pub enum DerivationCondition {
```

```
$ rg -n 'pub (struct|enum|type) DerivedCrossWorkspaceState\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs:518:pub struct DerivedCrossWorkspaceState {
```

```
$ rg -n 'pub (struct|enum|type) StateSelector\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs:135:pub enum StateSelector {
```

```
$ rg -n 'pub (struct|enum|type) CrossWorkspaceConstraint\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs:485:pub struct CrossWorkspaceConstraint {
```

```
$ rg -n 'pub (struct|enum|type) CascadeRule\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs:588:pub struct CascadeRule {
```

```
$ rg -n 'pub (struct|enum|type) Severity\b' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs:473:pub enum Severity {
/Users/adamtc007/Dev/dsl/crates/dsl-core/src/diagnostics.rs:10:pub enum Severity {
```

```
$ rg -n 'DagSeverity' /Users/adamtc007/Dev/dsl/crates
/Users/adamtc007/Dev/dsl/crates/dsl-core/src/lib.rs:65:    Severity as DagSeverity,
```

### C. Reachability in dsl-core Root Facade (`lib.rs`)
```
$ rg -n 'EffectClass|...|DagSeverity' /Users/adamtc007/Dev/dsl/crates/dsl-core/src/lib.rs
63:    CascadeRule, CrossWorkspaceConstraint, DerivationCondition, DerivedCrossWorkspaceState, EntryVia,
64:    Phase, StateSelector, SlotStateMachine, PredicateBinding, Slot as DagSlot,
65:    Severity as DagSeverity,
77:    unknown_verb_error, Diagnostic, DiagnosticCode, RelatedInfo, Severity, SourceSpan, SuggestedFix,
86:    BindingFrameSchema, BindingSlot, EffectClass,
```

---

## 3. THE EDIT

The consumer imports in the `ob-poc/rust` workspace were repointed:
- `DerivationCondition`, `DerivedCrossWorkspaceState`, `StateSelector`, `CrossWorkspaceConstraint`, and `CascadeRule` now live in `dsl_types` and were repointed directly to the `dsl_types` crate.
- `dsl_core::config::dag::Severity` was repointed to the public facade path `dsl_core::DagSeverity`.
- E0170 pattern binding shadowing warnings were fixed by qualifying match arm bindings (`dsl_core::DagSeverity::Error`, etc.) in `gate_checker.rs` and `hierarchy_cascade.rs`.
- `EffectClass` remains imported from `dsl_core::executable_plan::EffectClass` because it is NOT in `dsl_types` and is reachable only via the private module `executable_plan` in `dsl-core` (not re-exported at the root of `dsl-core`). No change was made to this private module import or visibility in `dsl-core` per the constraints.

---

## 4. EVIDENCE

### A. Cargo Check Output After Edits
```
$ cargo check -p dsl-runtime
warning: unused imports: `CategoryGated`, `ConditionalGate`, `DualLifecycle`, `EvidenceType`, `ParentJoin`, `ParentSlot`, `PeriodicReviewCadence`, `ProductModuleGates`, `PruneCascadeRule`, `PruneCascadeTarget`, `PrunePreValidation`, `ReviewScope`, `RiskTierOverride`, `StateDef`, `StateDependency`, `StateMachine`, and `TransitionDef`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag.rs:26:5
   |
26 |     StateMachine, StateDef, TransitionDef, ParentSlot, ParentJoin, StateDependency,
   |     ^^^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^
27 |     PeriodicReviewCadence, RiskTierOverride, ReviewScope, EvidenceType, CategoryGated,
   |     ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^
28 |     ProductModuleGates, ConditionalGate, PruneCascadeRule, PruneCascadeTarget,
   |     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^
29 |     PrunePreValidation, ExpectedLifetime, PredicateBindingSourceKind, DualLifecycle,
   |     ^^^^^^^^^^^^^^^^^^                                                ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `AuditClass`, `CompletenessAssertionConfig`, and `RoleGuard`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag.rs:34:5
   |
34 |     AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
   |     ^^^^^^^^^^               ^^^^^^^^^^^^^^^^^^^^^^^^^^^                         ^^^^^^^^^

warning: unused imports: `EntityQualifier` and `RelationScope`
 --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:6:22
  |
6 | pub(crate) use ast::{EntityQualifier, RelationScope};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `ast::State`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:11:16
   |
11 | pub(crate) use ast::State;
   |                ^^^^^^^^^^

warning: unused import: `parser::ParseError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:13:16
   |
13 | pub(crate) use parser::ParseError;
   |                ^^^^^^^^^^^^^^^^^^

warning: unused imports: `DagWarning` and `validate_constellation_map_schema_coordination`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:39:5
   |
39 |     validate_constellation_map_schema_coordination, DagWarning,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^

warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:46:5
   |
46 |     green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
47 |     GreenWhenExclusionReason,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `EvaluationContext` and `compute_effective_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:51:5
   |
51 |     compute_effective_tier, EvaluationContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:56:5
   |
56 |     compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
   |     ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `validator::StructuralError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:71:16
   |
71 | pub(crate) use validator::StructuralError;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `executable_plan::TransactionPolicy`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:82:16
   |
82 | pub(crate) use executable_plan::TransactionPolicy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `DagWarning` is more private than the item `DagValidationReport::warnings`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:476:5
    |
476 |     pub warnings: Vec<DagWarning>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `DagValidationReport::warnings` is reachable at visibility `pub`
    |
note: but type `DagWarning` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:386:1
    |
386 | pub(crate) enum DagWarning {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(private_interfaces)]` on by default

warning: type `RelationScope` is more private than the item `predicate::ast::EntityRef::Scoped::scope`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:76:9
    |
 76 |         scope: RelationScope,
    |         ^^^^^^^^^^^^^^^^^^^^ field `predicate::ast::EntityRef::Scoped::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `EntityQualifier` is more private than the item `EntitySetRef::qualifier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:86:5
   |
86 |     pub qualifier: Option<EntityQualifier>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::qualifier` is reachable at visibility `pub`
   |
note: but type `EntityQualifier` is only usable at visibility `pub(crate)`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:93:1
   |
93 | pub(crate) enum EntityQualifier {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `RelationScope` is more private than the item `EntitySetRef::scope`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:88:5
    |
 88 |     pub scope: Option<RelationScope>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `StructuralError` is more private than the item `ValidationReport::structural`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:295:5
    |
295 |     pub structural: Vec<StructuralError>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `ValidationReport::structural` is reachable at visibility `pub`
    |
note: but type `StructuralError` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:72:1
    |
 72 | pub(crate) enum StructuralError {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:53:19
   |
51 | impl Program {
   | ------------ methods in this implementation
52 |     /// Render the program back to DSL source (for execution - shows UUIDs when resolved)
53 |     pub(crate) fn to_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^
...
63 |     pub(crate) fn to_user_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:81:19
   |
79 | impl Statement {
   | -------------- methods in this implementation
80 |     /// Render the statement back to DSL source (for execution)
81 |     pub(crate) fn to_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^
...
89 |     pub(crate) fn to_user_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:110:19
    |
108 | impl VerbCall {
    | ------------- methods in this implementation
109 |     /// Render the verb call back to DSL source (for execution - shows UUIDs)
110 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
126 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^

warning: methods `get_arg` and `get_value` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:149:19
    |
142 | impl VerbCall {
    | ------------- methods in this implementation
...
149 |     pub(crate) fn get_arg(&self, key: &str) -> Option<&Argument> {
    |                   ^^^^^^^
...
154 |     pub(crate) fn get_value(&self, key: &str) -> Option<&AstNode> {
    |                   ^^^^^^^^^

warning: multiple associated items are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:293:19
    |
287 | impl AstNode {
    | ------------ associated items in this implementation
...
293 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
329 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^
...
362 |     pub(crate) fn integer(i: i64) -> Self {
    |                   ^^^^^^^
...
385 |     pub(crate) fn resolved_entity_ref(
    |                   ^^^^^^^^^^^^^^^^^^^
...
404 |     pub(crate) fn symbol_ref(name: impl Into<String>, span: Span) -> Self {
    |                   ^^^^^^^^^^
...
427 |     pub(crate) fn is_resolved_entity_ref(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^^^^^^
...
439 |     pub(crate) fn is_symbol_ref(&self) -> bool {
    |                   ^^^^^^^^^^^^^
...
444 |     pub(crate) fn is_literal(&self) -> bool {
    |                   ^^^^^^^^^^
...
453 |     pub(crate) fn as_string(&self) -> Option<&str> {
    |                   ^^^^^^^^^
...
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |                   ^^^^^^^
...
475 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
...
491 |     pub(crate) fn as_integer(&self) -> Option<i64> {
    |                   ^^^^^^^^^^
...
499 |     pub(crate) fn as_decimal(&self) -> Option<Decimal> {
    |                   ^^^^^^^^^^
...
508 |     pub(crate) fn as_boolean(&self) -> Option<bool> {
    |                   ^^^^^^^^^^
...
516 |     pub(crate) fn as_list(&self) -> Option<&[AstNode]> {
    |                   ^^^^^^^
...
524 |     pub(crate) fn as_map(&self) -> Option<&[(String, AstNode)]> {
    |                   ^^^^^^
...
532 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
554 |     pub(crate) fn with_resolved_key(&self, key: String) -> Self {
    |                   ^^^^^^^^^^^^^^^^^
...
564 |     pub(crate) fn try_with_resolved_key(&self, key: String) -> Result<Self, String> {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:626:19
    |
624 | impl Literal {
    | ------------ method in this implementation
625 |     /// Render the literal back to DSL source
626 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: associated function `merge` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:657:19
    |
651 | impl Span {
    | --------- associated function in this implementation
...
657 |     pub(crate) fn merge(a: Span, b: Span) -> Span {
    |                   ^^^^^

warning: function `find_unresolved_refs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:741:15
    |
741 | pub(crate) fn find_unresolved_refs(program: &Program) -> Vec<&AstNode> {
    |               ^^^^^^^^^^^^^^^^^^^^

warning: methods `is_fully_resolved`, `resolved_count`, and `resolution_percentage` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:775:19
    |
773 | impl EntityRefStats {
    | ------------------- methods in this implementation
774 |     /// Returns true if all EntityRefs are resolved
775 |     pub(crate) fn is_fully_resolved(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^
...
780 |     pub(crate) fn resolved_count(&self) -> i32 {
    |                   ^^^^^^^^^^^^^^
...
785 |     pub(crate) fn resolution_percentage(&self) -> u8 {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: methods `span`, `verb_name`, and `to_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:879:19
    |
877 | impl ViewportVerb {
    | ----------------- methods in this implementation
878 |     /// Get the span of this verb
879 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
893 |     pub(crate) fn verb_name(&self) -> &'static str {
    |                   ^^^^^^^^^
...
907 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:968:19
    |
966 | impl FocusTarget {
    | ---------------- methods in this implementation
967 |     /// Get the span of this target
968 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
982 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1022:19
     |
1020 | impl EnhanceArg {
     | --------------- method in this implementation
1021 |     /// Render the argument to DSL string
1022 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1048:19
     |
1046 | impl NavTarget {
     | -------------- methods in this implementation
1047 |     /// Get the span of this target
1048 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1057 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1079:19
     |
1077 | impl NavDirection {
     | ----------------- method in this implementation
1078 |     /// Render the direction to DSL string
1079 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: associated items `to_dsl_string` and `all` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1134:19
     |
1132 | impl ViewType {
     | ------------- associated items in this implementation
1133 |     /// Render the view type to DSL string
1134 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1161 |     pub(crate) fn all() -> &'static [ViewType] {
     |                   ^^^

warning: associated items `to_dsl_string`, `min_confidence`, and `from_score` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1198:19
     |
1196 | impl ConfidenceZone {
     | ------------------- associated items in this implementation
1197 |     /// Render the zone to DSL string
1198 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1219 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^
...
1229 |     pub(crate) fn from_score(score: f32) -> Self {
     |                   ^^^^^^^^^^

warning: methods `to_dsl_string`, `extension`, and `mime_type` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1260:19
     |
1258 | impl ExportFormat {
     | ----------------- methods in this implementation
1259 |     /// Render the format to DSL string
1260 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1281 |     pub(crate) fn extension(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1291 |     pub(crate) fn mime_type(&self) -> &'static str {
     |                   ^^^^^^^^^

warning: methods `merge`, `names`, and `available_types` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/binding_context.rs:94:19
    |
 88 | impl BindingContext {
    | ------------------- methods in this implementation
...
 94 |     pub(crate) fn merge(&mut self, other: &BindingContext) {
    |                   ^^^^^
...
111 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^
...
126 |     pub(crate) fn available_types(&self) -> std::collections::HashSet<String> {
    |                   ^^^^^^^^^^^^^^^

warning: method `is_clean` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:480:19
    |
479 | impl DagValidationReport {
    | ------------------------ method in this implementation
480 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: enum `SchemaCoordinationKnownDeferred` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:495:17
    |
495 | pub(crate) enum SchemaCoordinationKnownDeferred {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `harden_schema_coordination_warnings` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:686:15
    |
686 | pub(crate) fn harden_schema_coordination_warnings(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_known_deferred_key` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:704:4
    |
704 | fn schema_coordination_known_deferred_key(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_warning_to_error` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:738:4
    |
738 | fn schema_coordination_warning_to_error(warning: DagWarning) -> DagError {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_source_name` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:772:4
    |
772 | fn schema_coordination_source_name(location: &DagLocation) -> String {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `EvaluationContext` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:29:19
   |
29 | pub(crate) struct EvaluationContext {
   |                   ^^^^^^^^^^^^^^^^^

warning: associated items `new`, `with_arg`, `with_entity_attr`, and `with_flag` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:40:19
   |
39 | impl EvaluationContext {
   | ---------------------- associated items in this implementation
40 |     pub(crate) fn new() -> Self {
   |                   ^^^
...
44 |     pub(crate) fn with_arg(mut self, name: impl Into<String>, value: serde_json::Value) -> Self {
   |                   ^^^^^^^^
...
49 |     pub(crate) fn with_entity_attr(
   |                   ^^^^^^^^^^^^^^^^
...
62 |     pub(crate) fn with_flag(mut self, flag: impl Into<String>, value: bool) -> Self {
   |                   ^^^^^^^^^

warning: function `evaluate_predicate` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:72:15
   |
72 | pub(crate) fn evaluate_predicate(pred: &EscalationPredicate, ctx: &EvaluationContext) -> bool {
   |               ^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:128:15
    |
128 | pub(crate) fn compute_effective_tier(
    |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier_with_trace` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:142:15
    |
142 | pub(crate) fn compute_effective_tier_with_trace<'a>(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `as_f64` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:160:4
    |
160 | fn as_f64(v: &serde_json::Value) -> Option<f64> {
    |    ^^^^^^

warning: struct `GreenWhenCoverageRow` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:12:19
   |
12 | pub(crate) struct GreenWhenCoverageRow {
   |                   ^^^^^^^^^^^^^^^^^^^^

warning: enum `GreenWhenExclusionReason` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:24:17
   |
24 | pub(crate) enum GreenWhenExclusionReason {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `GreenWhenCoverageSummary` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:31:19
   |
31 | pub(crate) struct GreenWhenCoverageSummary {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dags` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:39:15
   |
39 | pub(crate) fn green_when_coverage_for_dags(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dag` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:54:15
   |
54 | pub(crate) fn green_when_coverage_for_dag(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_summary` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:82:15
   |
82 | pub(crate) fn green_when_coverage_summary(rows: &[GreenWhenCoverageRow]) -> GreenWhenCoverageSummary {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `row_for_state` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:97:4
   |
97 | fn row_for_state(
   |    ^^^^^^^^^^^^^

warning: function `inbound_verbs_by_destination` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:135:4
    |
135 | fn inbound_verbs_by_destination(transitions: &[TransitionDef]) -> BTreeMap<String, Vec<String>> {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `verbs_from_transition` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:150:4
    |
150 | fn verbs_from_transition(transition: &TransitionDef) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^

warning: function `states_from_yaml_value` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:158:4
    |
158 | fn states_from_yaml_value(value: &YamlValue) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^^

warning: function `split_tupleish` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:169:4
    |
169 | fn split_tupleish(value: &str) -> Vec<String> {
    |    ^^^^^^^^^^^^^^

warning: associated functions `entity_uuid`, `entity_uuid_binding`, and `natural_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/resource_dependency.rs:96:19
    |
 94 | impl ResourceDependency {
    | ----------------------- associated functions in this implementation
 95 |     /// Construct an `EntityUuid` dependency with a known UUID.
 96 |     pub(crate) fn entity_uuid(entity_type: impl Into<String>, uuid: uuid::Uuid) -> Self {
    |                   ^^^^^^^^^^^
...
105 |     pub(crate) fn entity_uuid_binding(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^
...
113 |     pub(crate) fn natural_key(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^

warning: associated functions `compile_resolved_entity`, `binding_resolved_entity`, and `runtime_create_natural_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/resource_dependency.rs:169:19
    |
167 | impl ResolvedResourceDependency {
    | ------------------------------- associated functions in this implementation
168 |     /// An entity UUID that was resolved at compile time.
169 |     pub(crate) fn compile_resolved_entity(entity_type: impl Into<String>, uuid: uuid::Uuid) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
...
178 |     pub(crate) fn binding_resolved_entity(entity_type: impl Into<String>, slot: BindingSlotId) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
...
187 |     pub(crate) fn runtime_create_natural_key(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `RunbookStep` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:39:19
   |
39 | pub(crate) struct RunbookStep {
   |                   ^^^^^^^^^^^

warning: enum `AggregationRule` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:67:17
   |
67 | pub(crate) enum AggregationRule {
   |                 ^^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:93:19
    |
 92 | impl AggregationRule {
    | -------------------- methods in this implementation
 93 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^
...
101 |     pub(crate) fn tier(&self) -> ConsequenceTier {
    |                   ^^^^
...
109 |     pub(crate) fn matches(&self, steps: &[RunbookStep]) -> bool {
    |                   ^^^^^^^

warning: enum `CrossScopeRule` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:139:17
    |
139 | pub(crate) enum CrossScopeRule {
    |                 ^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:161:19
    |
160 | impl CrossScopeRule {
    | ------------------- methods in this implementation
161 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^
...
169 |     pub(crate) fn tier(&self) -> ConsequenceTier {
    |                   ^^^^
...
177 |     pub(crate) fn matches(&self, steps: &[RunbookStep]) -> bool {
    |                   ^^^^^^^

warning: function `compute_runbook_tier` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:212:15
    |
212 | pub(crate) fn compute_runbook_tier(
    |               ^^^^^^^^^^^^^^^^^^^^

warning: function `component_a` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:228:15
    |
228 | pub(crate) fn component_a(steps: &[RunbookStep]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_b` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:235:15
    |
235 | pub(crate) fn component_b(steps: &[RunbookStep], rules: &[AggregationRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_c` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:243:15
    |
243 | pub(crate) fn component_c(steps: &[RunbookStep], rules: &[CrossScopeRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: struct `CsgRulesConfig` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:26:19
   |
26 | pub(crate) struct CsgRulesConfig {
   |                   ^^^^^^^^^^^^^^

warning: methods `is_simple`, `discriminators`, and `min_confidence` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1398:19
     |
1376 | impl SearchKeyConfig {
     | -------------------- methods in this implementation
...
1398 |     pub(crate) fn is_simple(&self) -> bool {
     |                   ^^^^^^^^^
...
1417 |     pub(crate) fn discriminators(&self) -> &[SearchDiscriminator] {
     |                   ^^^^^^^^^^^^^^
...
1426 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^

warning: method `arg_name` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1791:19
     |
1789 | impl SearchDiscriminator {
     | ------------------------ method in this implementation
1790 |     /// Get the argument name (uses field name if from_arg not specified)
1791 |     pub(crate) fn arg_name(&self) -> &str {
     |                   ^^^^^^^^

warning: struct `ConstraintRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1910:19
     |
1910 | pub(crate) struct ConstraintRule {
     |                   ^^^^^^^^^^^^^^

warning: struct `WarningRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1920:19
     |
1920 | pub(crate) struct WarningRule {
     |                   ^^^^^^^^^^^

warning: struct `JurisdictionRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1932:19
     |
1932 | pub(crate) struct JurisdictionRule {
     |                   ^^^^^^^^^^^^^^^^

warning: struct `CompositeRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1944:19
     |
1944 | pub(crate) struct CompositeRule {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleCondition` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1955:19
     |
1955 | pub(crate) struct RuleCondition {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleRequirement` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1973:19
     |
1973 | pub(crate) struct RuleRequirement {
     |                   ^^^^^^^^^^^^^^^

warning: struct `JurisdictionCondition` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1983:19
     |
1983 | pub(crate) struct JurisdictionCondition {
     |                   ^^^^^^^^^^^^^^^^^^^^^

warning: struct `AppliesTo` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1995:19
     |
1995 | pub(crate) struct AppliesTo {
     |                   ^^^^^^^^^

warning: enum `RuleSeverity` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:2002:17
     |
2002 | pub(crate) enum RuleSeverity {
     |                 ^^^^^^^^^^^^

warning: variant `TransitionWithoutEdges` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:75:5
   |
72 | pub(crate) enum StructuralError {
   |                 --------------- variant in this enum
...
75 |     TransitionWithoutEdges(Location),
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `StructuralError` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: method `is_clean` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:301:19
    |
300 | impl ValidationReport {
    | --------------------- method in this implementation
301 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: associated function `warning` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/diagnostics.rs:150:19
    |
136 | impl Diagnostic {
    | --------------- associated function in this implementation
...
150 |     pub(crate) fn warning(code: DiagnosticCode, message: impl Into<String>) -> Self {
    |                   ^^^^^^^

warning: struct `PlanId` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:43:19
   |
43 | pub(crate) struct PlanId(pub(crate) Uuid);
   |                   ^^^^^^

warning: associated function `new` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:46:19
   |
45 | impl PlanId {
   | ----------- associated function in this implementation
46 |     pub(crate) fn new() -> Self {
   |                   ^^^

warning: struct `SemOsSnapshotId` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:70:19
   |
70 | pub(crate) struct SemOsSnapshotId(pub(crate) u64);
   |                   ^^^^^^^^^^^^^^^

warning: enum `TransactionPolicy` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:141:17
    |
141 | pub(crate) enum TransactionPolicy {
    |                 ^^^^^^^^^^^^^^^^^

warning: associated function `from_effect_classes` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:162:19
    |
156 | impl TransactionPolicy {
    | ---------------------- associated function in this implementation
...
162 |     pub(crate) fn from_effect_classes(classes: impl IntoIterator<Item = EffectClass>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^

warning: struct `AuthorityContext` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:203:19
    |
203 | pub(crate) struct AuthorityContext {
    |                   ^^^^^^^^^^^^^^^^

warning: enum `InstructionInput` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:240:17
    |
240 | pub(crate) enum InstructionInput {
    |                 ^^^^^^^^^^^^^^^^

warning: struct `RuntimeInstruction` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:259:19
    |
259 | pub(crate) struct RuntimeInstruction {
    |                   ^^^^^^^^^^^^^^^^^^

warning: struct `ExecutablePlan` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:311:19
    |
311 | pub(crate) struct ExecutablePlan {
    |                   ^^^^^^^^^^^^^^

warning: associated constant `FORMAT_VERSION` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:347:22
    |
345 | impl ExecutablePlan {
    | ------------------- associated constant in this implementation
346 |     /// Current plan format version.
347 |     pub(crate) const FORMAT_VERSION: u32 = 1;
    |                      ^^^^^^^^^^^^^^

warning: struct `ExecutionStepSummary` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:358:19
    |
358 | pub(crate) struct ExecutionStepSummary {
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: methods `imposes_order` and `ordering_pair` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:158:19
    |
153 | impl DagEdge {
    | ------------ methods in this implementation
...
158 |     pub(crate) fn imposes_order(&self) -> bool {
    |                   ^^^^^^^^^^^^^
...
168 |     pub(crate) fn ordering_pair(&self) -> Option<(NodeId, NodeId)> {
    |                   ^^^^^^^^^^^^^

warning: method `ordering_pairs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:208:19
    |
193 | impl PopulatedExecutionDag {
    | -------------------------- method in this implementation
...
208 |     pub(crate) fn ordering_pairs(&self) -> Vec<(usize, usize)> {
    |                   ^^^^^^^^^^^^^^

warning: `dsl-core` (lib) generated 96 warnings (run `cargo fix --lib -p dsl-core` to apply 11 suggestions)
    Checking dsl-runtime v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime)
error[E0603]: module `executable_plan` is private
  --> crates/dsl-runtime/src/coordination.rs:21:15
   |
21 | use dsl_core::executable_plan::EffectClass;
   |               ^^^^^^^^^^^^^^^  ----------- enum `EffectClass` is not publicly re-exported
   |               |
   |               private module
   |
note: the module `executable_plan` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:21:1
   |
21 | pub(crate) mod executable_plan;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `DualLifecycle`, `Slot`, and `StateSelector`
  --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:28:64
   |
28 |     CrossWorkspaceConstraint, Dag, DerivedCrossWorkspaceState, DualLifecycle, LoadedDag,
   |                                                                ^^^^^^^^^^^^^
29 |     ParentSlot, Slot, SlotStateMachine, StateDependency, StateSelector, TransitionDef,
   |                 ^^^^                                     ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

For more information about this error, try `rustc --explain E0603`.
warning: `dsl-runtime` (lib) generated 1 warning
error: could not compile `dsl-runtime` (lib) due to 1 previous error; 1 warning emitted
```

### B. Quarantined Workspace Gate Check
```
$ cargo check --workspace --exclude dsl-runtime --exclude dsl-lsp --exclude ob-poc --exclude ob-poc-web --exclude ob-poc-agent --exclude sem_os_harness --exclude sem_os_postgres --exclude sem_os_server --exclude xtask --all-features
warning: unused imports: `CategoryGated`, `ConditionalGate`, `DualLifecycle`, `EvidenceType`, `ParentJoin`, `ParentSlot`, `PeriodicReviewCadence`, `ProductModuleGates`, `PruneCascadeRule`, `PruneCascadeTarget`, `PrunePreValidation`, `ReviewScope`, `RiskTierOverride`, `StateDef`, `StateDependency`, `StateMachine`, and `TransitionDef`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag.rs:26:5
   |
26 |     StateMachine, StateDef, TransitionDef, ParentSlot, ParentJoin, StateDependency,
   |     ^^^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^
27 |     PeriodicReviewCadence, RiskTierOverride, ReviewScope, EvidenceType, CategoryGated,
   |     ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^
28 |     ProductModuleGates, ConditionalGate, PruneCascadeRule, PruneCascadeTarget,
   |     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^
29 |     PrunePreValidation, ExpectedLifetime, PredicateBindingSourceKind, DualLifecycle,
   |     ^^^^^^^^^^^^^^^^^^                                                ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `AuditClass`, `CompletenessAssertionConfig`, and `RoleGuard`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag.rs:34:5
   |
34 |     AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
   |     ^^^^^^^^^^               ^^^^^^^^^^^^^^^^^^^^^^^^^^^                         ^^^^^^^^^

warning: unused imports: `EntityQualifier` and `RelationScope`
 --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:6:22
  |
6 | pub(crate) use ast::{EntityQualifier, RelationScope};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `ast::State`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:11:16
   |
11 | pub(crate) use ast::State;
   |                ^^^^^^^^^^

warning: unused import: `parser::ParseError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/mod.rs:13:16
   |
13 | pub(crate) use parser::ParseError;
   |                ^^^^^^^^^^^^^^^^^^

warning: unused imports: `DagWarning` and `validate_constellation_map_schema_coordination`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:39:5
   |
39 |     validate_constellation_map_schema_coordination, DagWarning,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^

warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:46:5
   |
46 |     green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
47 |     GreenWhenExclusionReason,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `EvaluationContext` and `compute_effective_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:51:5
   |
51 |     compute_effective_tier, EvaluationContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:56:5
   |
56 |     compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
   |     ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `validator::StructuralError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:71:16
   |
71 | pub(crate) use validator::StructuralError;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `executable_plan::TransactionPolicy`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:82:16
   |
82 | pub(crate) use executable_plan::TransactionPolicy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `DagWarning` is more private than the item `DagValidationReport::warnings`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:476:5
    |
476 |     pub warnings: Vec<DagWarning>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `DagValidationReport::warnings` is reachable at visibility `pub`
    |
note: but type `DagWarning` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:386:1
    |
386 | pub(crate) enum DagWarning {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(private_interfaces)]` on by default

warning: type `RelationScope` is more private than the item `predicate::ast::EntityRef::Scoped::scope`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:76:9
    |
 76 |         scope: RelationScope,
    |         ^^^^^^^^^^^^^^^^^^^^ field `predicate::ast::EntityRef::Scoped::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `EntityQualifier` is more private than the item `EntitySetRef::qualifier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:86:5
   |
86 |     pub qualifier: Option<EntityQualifier>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::qualifier` is reachable at visibility `pub`
   |
note: but type `EntityQualifier` is only usable at visibility `pub(crate)`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:93:1
   |
93 | pub(crate) enum EntityQualifier {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `RelationScope` is more private than the item `EntitySetRef::scope`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:88:5
    |
 88 |     pub scope: Option<RelationScope>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `StructuralError` is more private than the item `ValidationReport::structural`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:295:5
    |
295 |     pub structural: Vec<StructuralError>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `ValidationReport::structural` is reachable at visibility `pub`
    |
note: but type `StructuralError` is only usable at visibility `pub(crate)`
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:72:1
    |
 72 | pub(crate) enum StructuralError {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:53:19
   |
51 | impl Program {
   | ------------ methods in this implementation
52 |     /// Render the program back to DSL source (for execution - shows UUIDs when resolved)
53 |     pub(crate) fn to_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^
...
63 |     pub(crate) fn to_user_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:81:19
   |
79 | impl Statement {
   | -------------- methods in this implementation
80 |     /// Render the statement back to DSL source (for execution)
81 |     pub(crate) fn to_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^
...
89 |     pub(crate) fn to_user_dsl_string(&self) -> String {
   |                   ^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string` and `to_user_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:110:19
    |
108 | impl VerbCall {
    | ------------- methods in this implementation
109 |     /// Render the verb call back to DSL source (for execution - shows UUIDs)
110 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
126 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^

warning: methods `get_arg` and `get_value` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:149:19
    |
142 | impl VerbCall {
    | ------------- methods in this implementation
...
149 |     pub(crate) fn get_arg(&self, key: &str) -> Option<&Argument> {
    |                   ^^^^^^^
...
154 |     pub(crate) fn get_value(&self, key: &str) -> Option<&AstNode> {
    |                   ^^^^^^^^^

warning: multiple associated items are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:293:19
    |
287 | impl AstNode {
    | ------------ associated items in this implementation
...
293 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
329 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^
...
362 |     pub(crate) fn integer(i: i64) -> Self {
    |                   ^^^^^^^
...
385 |     pub(crate) fn resolved_entity_ref(
    |                   ^^^^^^^^^^^^^^^^^^^
...
404 |     pub(crate) fn symbol_ref(name: impl Into<String>, span: Span) -> Self {
    |                   ^^^^^^^^^^
...
427 |     pub(crate) fn is_resolved_entity_ref(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^^^^^^
...
439 |     pub(crate) fn is_symbol_ref(&self) -> bool {
    |                   ^^^^^^^^^^^^^
...
444 |     pub(crate) fn is_literal(&self) -> bool {
    |                   ^^^^^^^^^^
...
453 |     pub(crate) fn as_string(&self) -> Option<&str> {
    |                   ^^^^^^^^^
...
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |                   ^^^^^^^
...
475 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
...
491 |     pub(crate) fn as_integer(&self) -> Option<i64> {
    |                   ^^^^^^^^^^
...
499 |     pub(crate) fn as_decimal(&self) -> Option<Decimal> {
    |                   ^^^^^^^^^^
...
508 |     pub(crate) fn as_boolean(&self) -> Option<bool> {
    |                   ^^^^^^^^^^
...
516 |     pub(crate) fn as_list(&self) -> Option<&[AstNode]> {
    |                   ^^^^^^^
...
524 |     pub(crate) fn as_map(&self) -> Option<&[(String, AstNode)]> {
    |                   ^^^^^^
...
532 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
554 |     pub(crate) fn with_resolved_key(&self, key: String) -> Self {
    |                   ^^^^^^^^^^^^^^^^^
...
564 |     pub(crate) fn try_with_resolved_key(&self, key: String) -> Result<Self, String> {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:626:19
    |
624 | impl Literal {
    | ------------ method in this implementation
625 |     /// Render the literal back to DSL source
626 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: associated function `merge` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:657:19
    |
651 | impl Span {
    | --------- associated function in this implementation
...
657 |     pub(crate) fn merge(a: Span, b: Span) -> Span {
    |                   ^^^^^

warning: function `find_unresolved_refs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:741:15
    |
741 | pub(crate) fn find_unresolved_refs(program: &Program) -> Vec<&AstNode> {
    |               ^^^^^^^^^^^^^^^^^^^^

warning: methods `is_fully_resolved`, `resolved_count`, and `resolution_percentage` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:775:19
    |
773 | impl EntityRefStats {
    | ------------------- methods in this implementation
774 |     /// Returns true if all EntityRefs are resolved
775 |     pub(crate) fn is_fully_resolved(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^
...
780 |     pub(crate) fn resolved_count(&self) -> i32 {
    |                   ^^^^^^^^^^^^^^
...
785 |     pub(crate) fn resolution_percentage(&self) -> u8 {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: methods `span`, `verb_name`, and `to_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:879:19
    |
877 | impl ViewportVerb {
    | ----------------- methods in this implementation
878 |     /// Get the span of this verb
879 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
893 |     pub(crate) fn verb_name(&self) -> &'static str {
    |                   ^^^^^^^^^
...
907 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:968:19
    |
966 | impl FocusTarget {
    | ---------------- methods in this implementation
967 |     /// Get the span of this target
968 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^
...
982 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1022:19
     |
1020 | impl EnhanceArg {
     | --------------- method in this implementation
1021 |     /// Render the argument to DSL string
1022 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1048:19
     |
1046 | impl NavTarget {
     | -------------- methods in this implementation
1047 |     /// Get the span of this target
1048 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1057 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1079:19
     |
1077 | impl NavDirection {
     | ----------------- method in this implementation
1078 |     /// Render the direction to DSL string
1079 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: associated items `to_dsl_string` and `all` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1134:19
     |
1132 | impl ViewType {
     | ------------- associated items in this implementation
1133 |     /// Render the view type to DSL string
1134 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1161 |     pub(crate) fn all() -> &'static [ViewType] {
     |                   ^^^

warning: associated items `to_dsl_string`, `min_confidence`, and `from_score` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1198:19
     |
1196 | impl ConfidenceZone {
     | ------------------- associated items in this implementation
1197 |     /// Render the zone to DSL string
1198 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1219 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^
...
1229 |     pub(crate) fn from_score(score: f32) -> Self {
     |                   ^^^^^^^^^^

warning: methods `to_dsl_string`, `extension`, and `mime_type` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1260:19
     |
1258 | impl ExportFormat {
     | ----------------- methods in this implementation
1259 |     /// Render the format to DSL string
1260 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1281 |     pub(crate) fn extension(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1291 |     pub(crate) fn mime_type(&self) -> &'static str {
     |                   ^^^^^^^^^

warning: methods `merge`, `names`, and `available_types` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/binding_context.rs:94:19
    |
 88 | impl BindingContext {
    | ------------------- methods in this implementation
...
 94 |     pub(crate) fn merge(&mut self, other: &BindingContext) {
    |                   ^^^^^
...
111 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^
...
126 |     pub(crate) fn available_types(&self) -> std::collections::HashSet<String> {
    |                   ^^^^^^^^^^^^^^^

warning: method `is_clean` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:480:19
    |
479 | impl DagValidationReport {
    | ------------------------ method in this implementation
480 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: enum `SchemaCoordinationKnownDeferred` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:495:17
    |
495 | pub(crate) enum SchemaCoordinationKnownDeferred {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `harden_schema_coordination_warnings` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:686:15
    |
686 | pub(crate) fn harden_schema_coordination_warnings(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_known_deferred_key` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:704:4
    |
704 | fn schema_coordination_known_deferred_key(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_warning_to_error` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:738:4
    |
738 | fn schema_coordination_warning_to_error(warning: DagWarning) -> DagError {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_source_name` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:772:4
    |
772 | fn schema_coordination_source_name(location: &DagLocation) -> String {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `EvaluationContext` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:29:19
   |
29 | pub(crate) struct EvaluationContext {
   |                   ^^^^^^^^^^^^^^^^^

warning: associated items `new`, `with_arg`, `with_entity_attr`, and `with_flag` are never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:40:19
   |
39 | impl EvaluationContext {
   | ---------------------- associated items in this implementation
40 |     pub(crate) fn new() -> Self {
   |                   ^^^
...
44 |     pub(crate) fn with_arg(mut self, name: impl Into<String>, value: serde_json::Value) -> Self {
   |                   ^^^^^^^^
...
49 |     pub(crate) fn with_entity_attr(
   |                   ^^^^^^^^^^^^^^^^
...
62 |     pub(crate) fn with_flag(mut self, flag: impl Into<String>, value: bool) -> Self {
   |                   ^^^^^^^^^

warning: function `evaluate_predicate` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:72:15
   |
72 | pub(crate) fn evaluate_predicate(pred: &EscalationPredicate, ctx: &EvaluationContext) -> bool {
   |               ^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:128:15
    |
128 | pub(crate) fn compute_effective_tier(
    |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier_with_trace` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:142:15
    |
142 | pub(crate) fn compute_effective_tier_with_trace<'a>(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `as_f64` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/escalation.rs:160:4
    |
160 | fn as_f64(v: &serde_json::Value) -> Option<f64> {
    |    ^^^^^^

warning: struct `GreenWhenCoverageRow` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:12:19
   |
12 | pub(crate) struct GreenWhenCoverageRow {
   |                   ^^^^^^^^^^^^^^^^^^^^

warning: enum `GreenWhenExclusionReason` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:24:17
   |
24 | pub(crate) enum GreenWhenExclusionReason {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `GreenWhenCoverageSummary` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:31:19
   |
31 | pub(crate) struct GreenWhenCoverageSummary {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dags` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:39:15
   |
39 | pub(crate) fn green_when_coverage_for_dags(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dag` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:54:15
   |
54 | pub(crate) fn green_when_coverage_for_dag(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_summary` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:82:15
   |
82 | pub(crate) fn green_when_coverage_summary(rows: &[GreenWhenCoverageRow]) -> GreenWhenCoverageSummary {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `row_for_state` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:97:4
   |
97 | fn row_for_state(
   |    ^^^^^^^^^^^^^

warning: function `inbound_verbs_by_destination` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:135:4
    |
135 | fn inbound_verbs_by_destination(transitions: &[TransitionDef]) -> BTreeMap<String, Vec<String>> {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `verbs_from_transition` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:150:4
    |
150 | fn verbs_from_transition(transition: &TransitionDef) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^

warning: function `states_from_yaml_value` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:158:4
    |
158 | fn states_from_yaml_value(value: &YamlValue) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^^

warning: function `split_tupleish` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs:169:4
    |
169 | fn split_tupleish(value: &str) -> Vec<String> {
    |    ^^^^^^^^^^^^^^

warning: associated functions `entity_uuid`, `entity_uuid_binding`, and `natural_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/resource_dependency.rs:96:19
    |
 94 | impl ResourceDependency {
    | ----------------------- associated functions in this implementation
 95 |     /// Construct an `EntityUuid` dependency with a known UUID.
 96 |     pub(crate) fn entity_uuid(entity_type: impl Into<String>, uuid: uuid::Uuid) -> Self {
    |                   ^^^^^^^^^^^
...
105 |     pub(crate) fn entity_uuid_binding(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^
...
113 |     pub(crate) fn natural_key(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^

warning: associated functions `compile_resolved_entity`, `binding_resolved_entity`, and `runtime_create_natural_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/resource_dependency.rs:169:19
    |
167 | impl ResolvedResourceDependency {
    | ------------------------------- associated functions in this implementation
168 |     /// An entity UUID that was resolved at compile time.
169 |     pub(crate) fn compile_resolved_entity(entity_type: impl Into<String>, uuid: uuid::Uuid) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
...
178 |     pub(crate) fn binding_resolved_entity(entity_type: impl Into<String>, slot: BindingSlotId) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
...
187 |     pub(crate) fn runtime_create_natural_key(entity_type: impl Into<String>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `RunbookStep` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:39:19
   |
39 | pub(crate) struct RunbookStep {
   |                   ^^^^^^^^^^^

warning: enum `AggregationRule` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:67:17
   |
67 | pub(crate) enum AggregationRule {
   |                 ^^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:93:19
    |
 92 | impl AggregationRule {
    | -------------------- methods in this implementation
 93 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^
...
101 |     pub(crate) fn tier(&self) -> ConsequenceTier {
    |                   ^^^^
...
109 |     pub(crate) fn matches(&self, steps: &[RunbookStep]) -> bool {
    |                   ^^^^^^^

warning: enum `CrossScopeRule` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:139:17
    |
139 | pub(crate) enum CrossScopeRule {
    |                 ^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:161:19
    |
160 | impl CrossScopeRule {
    | ------------------- methods in this implementation
161 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^
...
169 |     pub(crate) fn tier(&self) -> ConsequenceTier {
    |                   ^^^^
...
177 |     pub(crate) fn matches(&self, steps: &[RunbookStep]) -> bool {
    |                   ^^^^^^^

warning: function `compute_runbook_tier` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:212:15
    |
212 | pub(crate) fn compute_runbook_tier(
    |               ^^^^^^^^^^^^^^^^^^^^

warning: function `component_a` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:228:15
    |
228 | pub(crate) fn component_a(steps: &[RunbookStep]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_b` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:235:15
    |
235 | pub(crate) fn component_b(steps: &[RunbookStep], rules: &[AggregationRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_c` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/runbook_composition.rs:243:15
    |
243 | pub(crate) fn component_c(steps: &[RunbookStep], rules: &[CrossScopeRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: struct `CsgRulesConfig` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:26:19
   |
26 | pub(crate) struct CsgRulesConfig {
   |                   ^^^^^^^^^^^^^^

warning: methods `is_simple`, `discriminators`, and `min_confidence` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1398:19
     |
1376 | impl SearchKeyConfig {
     | -------------------- methods in this implementation
...
1398 |     pub(crate) fn is_simple(&self) -> bool {
     |                   ^^^^^^^^^
...
1417 |     pub(crate) fn discriminators(&self) -> &[SearchDiscriminator] {
     |                   ^^^^^^^^^^^^^^
...
1426 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^

warning: method `arg_name` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1791:19
     |
1789 | impl SearchDiscriminator {
     | ------------------------ method in this implementation
1790 |     /// Get the argument name (uses field name if from_arg not specified)
1791 |     pub(crate) fn arg_name(&self) -> &str {
     |                   ^^^^^^^^

warning: struct `ConstraintRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1910:19
     |
1910 | pub(crate) struct ConstraintRule {
     |                   ^^^^^^^^^^^^^^

warning: struct `WarningRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1920:19
     |
1920 | pub(crate) struct WarningRule {
     |                   ^^^^^^^^^^^

warning: struct `JurisdictionRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1932:19
     |
1932 | pub(crate) struct JurisdictionRule {
     |                   ^^^^^^^^^^^^^^^^

warning: struct `CompositeRule` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1944:19
     |
1944 | pub(crate) struct CompositeRule {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleCondition` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1955:19
     |
1955 | pub(crate) struct RuleCondition {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleRequirement` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1973:19
     |
1973 | pub(crate) struct RuleRequirement {
     |                   ^^^^^^^^^^^^^^^

warning: struct `JurisdictionCondition` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1983:19
     |
1983 | pub(crate) struct JurisdictionCondition {
     |                   ^^^^^^^^^^^^^^^^^^^^^

warning: struct `AppliesTo` is never constructed
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1995:19
     |
1995 | pub(crate) struct AppliesTo {
     |                   ^^^^^^^^^

warning: enum `RuleSeverity` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:2002:17
     |
2002 | pub(crate) enum RuleSeverity {
     |                 ^^^^^^^^^^^^

warning: variant `TransitionWithoutEdges` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:75:5
   |
72 | pub(crate) enum StructuralError {
   |                 --------------- variant in this enum
...
75 |     TransitionWithoutEdges(Location),
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `StructuralError` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: method `is_clean` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/validator.rs:301:19
    |
300 | impl ValidationReport {
    | --------------------- method in this implementation
301 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: associated function `warning` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/diagnostics.rs:150:19
    |
136 | impl Diagnostic {
    | --------------- associated function in this implementation
...
150 |     pub(crate) fn warning(code: DiagnosticCode, message: impl Into<String>) -> Self {
    |                   ^^^^^^^

warning: struct `PlanId` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:43:19
   |
43 | pub(crate) struct PlanId(pub(crate) Uuid);
   |                   ^^^^^^

warning: associated function `new` is never used
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:46:19
   |
45 | impl PlanId {
   | ----------- associated function in this implementation
46 |     pub(crate) fn new() -> Self {
   |                   ^^^

warning: struct `SemOsSnapshotId` is never constructed
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:70:19
   |
70 | pub(crate) struct SemOsSnapshotId(pub(crate) u64);
   |                   ^^^^^^^^^^^^^^^

warning: enum `TransactionPolicy` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:141:17
    |
141 | pub(crate) enum TransactionPolicy {
    |                 ^^^^^^^^^^^^^^^^^

warning: associated function `from_effect_classes` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:162:19
    |
156 | impl TransactionPolicy {
    | ---------------------- associated function in this implementation
...
162 |     pub(crate) fn from_effect_classes(classes: impl IntoIterator<Item = EffectClass>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^

warning: struct `AuthorityContext` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:203:19
    |
203 | pub(crate) struct AuthorityContext {
    |                   ^^^^^^^^^^^^^^^^

warning: enum `InstructionInput` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:240:17
    |
240 | pub(crate) enum InstructionInput {
    |                 ^^^^^^^^^^^^^^^^

warning: struct `RuntimeInstruction` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:259:19
    |
259 | pub(crate) struct RuntimeInstruction {
    |                   ^^^^^^^^^^^^^^^^^^

warning: struct `ExecutablePlan` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:311:19
    |
311 | pub(crate) struct ExecutablePlan {
    |                   ^^^^^^^^^^^^^^

warning: associated constant `FORMAT_VERSION` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:347:22
    |
345 | impl ExecutablePlan {
    | ------------------- associated constant in this implementation
346 |     /// Current plan format version.
347 |     pub(crate) const FORMAT_VERSION: u32 = 1;
    |                      ^^^^^^^^^^^^^^

warning: struct `ExecutionStepSummary` is never constructed
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/executable_plan.rs:358:19
    |
358 | pub(crate) struct ExecutionStepSummary {
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: methods `imposes_order` and `ordering_pair` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:158:19
    |
153 | impl DagEdge {
    | ------------ methods in this implementation
...
158 |     pub(crate) fn imposes_order(&self) -> bool {
    |                   ^^^^^^^^^^^^^
...
168 |     pub(crate) fn ordering_pair(&self) -> Option<(NodeId, NodeId)> {
    |                   ^^^^^^^^^^^^^

warning: method `ordering_pairs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:208:19
    |
193 | impl PopulatedExecutionDag {
    | -------------------------- method in this implementation
...
208 |     pub(crate) fn ordering_pairs(&self) -> Vec<(usize, usize)> {
    |                   ^^^^^^^^^^^^^^

warning: `dsl-core` (lib) generated 96 warnings (run `cargo fix --lib -p dsl-core` to apply 11 suggestions)
warning: unused imports: `CrudConfig`, `DomainConfig`, `DurableConfig`, `ReturnsConfig`, and `SearchKeyConfig`
  --> crates/dsl-analysis/src/runtime_registry.rs:24:44
   |
24 |     ArgConfig, ArgType, BatchPolicyConfig, CrudConfig, CrudOperation, DomainConfig, DurableConfig,
   |                                            ^^^^^^^^^^                 ^^^^^^^^^^^^  ^^^^^^^^^^^^^
25 |     DurableRuntime, DynamicVerbConfig, FuzzyCheckConfig, GraphQueryOperation, HarmClass,
26 |     LockAccessConfig, LockModeConfig, LookupConfig, PolicyConfig, ReturnTypeConfig, ReturnsConfig,
   |                                                                                     ^^^^^^^^^^^^^
27 |     SearchKeyConfig, VerbBehavior, VerbConfig, VerbConsumes, VerbLifecycle, VerbProduces,
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `dsl-analysis` (lib) generated 1 warning (run `cargo fix --lib -p dsl-analysis` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
```

### C. Workspace Git Status and Diff

#### git status --porcelain
```
$ git status --porcelain
 M rust/Cargo.lock
 M rust/Cargo.toml
 M rust/crates/dsl-runtime/Cargo.toml
 M rust/crates/dsl-runtime/src/cross_workspace/derived_state.rs
 M rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs
 M rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
 M rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
 M rust/crates/dsl-runtime/src/cross_workspace/mod.rs
 M rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs
 M rust/crates/dsl-runtime/src/cross_workspace/test_harness/live.rs
 M rust/crates/dsl-runtime/src/cross_workspace/test_harness/mod.rs
 M rust/crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs
?? current_unreachable_pub.txt
?? ob_poc_unreachable_pub.txt
?? ob_poc_unreachable_pub_v2.txt
?? patch.rs
?? rust/check.txt
?? rust/check_after.txt
?? rust/crates/dsl-runtime/src/cross_workspace/dag_registry.rs
```

#### git diff
```
$ git diff
diff --git a/rust/Cargo.lock b/rust/Cargo.lock
index d31f96bf..6a71edbc 100644
--- a/rust/Cargo.lock
+++ b/rust/Cargo.lock
@@ -1708,6 +1708,7 @@ dependencies = [
  "bigdecimal",
  "chrono",
  "dsl-core",
+ "dsl_types",
  "futures",
  "hex",
  "nom",
diff --git a/rust/Cargo.toml b/rust/Cargo.toml
index ab9ff0d0..48297ea5 100644
--- a/rust/Cargo.toml
+++ b/rust/Cargo.toml
@@ -370,11 +370,10 @@ members = [
 # overrides these with ~/dev/ paths during development.
 dsl_types    = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
 dsl-core     = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
-sem_os_types    = { git = "https://github.com/adamtc007/sem-os", tag = "v0.1.5" }
-sem_os_core     = { git = "https://github.com/adamtc007/sem-os", tag = "v0.1.5" }
-sem_os_ontology = { git = "https://github.com/adamtc007/sem-os", tag = "v0.1.5" }
-sem_os_policy   = { git = "https://github.com/adamtc007/sem-os", tag = "v0.1.5" }
-sem_os_taxonomy = { git = "https://github.com/adamtc007/sem-os", tag = "v0.1.5" }
+sem_os_types    = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
+sem_os_core     = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
+sem_os_ontology = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
+sem_os_policy   = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
 
 # bpmn-lite published as github.com/adamtc007/bpmn-lite v0.2.0 (T1 + T2:
 # dsl-manifest, dsl-bus-*, dmn-lite-server, bpmn-lite plan-walker bus
diff --git a/rust/crates/dsl-runtime/Cargo.toml b/rust/crates/dsl-runtime/Cargo.toml
index 810c3ed7..503b7e10 100644
--- a/rust/crates/dsl-runtime/Cargo.toml
+++ b/rust/crates/dsl-runtime/Cargo.toml
@@ -28,6 +28,7 @@ ob-poc-types = { path = "../ob-poc-types" }
 # Phase 5a composite-blocker #23 — domain_ops still uses dsl_core::config
 # loader/types for verb metadata loading.
 dsl-core.workspace = true
+dsl_types.workspace = true
 # dsl-runtime-split v1 Phase 10 (2026-05-14): the following deps were
 # removed after the analyser-tier modules consuming them moved to
 # `dsl-analysis`:
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/derived_state.rs b/rust/crates/dsl-runtime/src/cross_workspace/derived_state.rs
index 48ae0167..d60173a4 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/derived_state.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/derived_state.rs
@@ -21,7 +21,7 @@
 //! itself is stateless and just executes one evaluation pass.
 
 use anyhow::Result;
-use dsl_core::config::dag::{DerivationCondition, DerivedCrossWorkspaceState, StateSelector};
+use dsl_types::{DerivationCondition, DerivedCrossWorkspaceState, StateSelector};
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs b/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs
index 06ab2b86..cb9f9aad 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs
@@ -16,7 +16,7 @@
 //! verb-touched slots).
 
 use anyhow::Result;
-use dsl_core::config::DagRegistry;
+use crate::cross_workspace::DagRegistry;
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
@@ -106,7 +106,7 @@ mod tests {
     use crate::cross_workspace::SameEntityResolver;
     use crate::cross_workspace::SlotStateProvider;
     use async_trait::async_trait;
-    use dsl_core::config::dag::{Dag, LoadedDag};
+    use dsl_types::{Dag, LoadedDag};
     use std::collections::BTreeMap;
     use std::path::PathBuf;
     use std::sync::Mutex;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs b/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
index c8387a21..932d7926 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
@@ -37,8 +37,8 @@
 
 use anyhow::Result;
 use async_trait::async_trait;
-use dsl_core::config::dag::{CrossWorkspaceConstraint, StateSelector};
-use dsl_core::config::DagRegistry;
+use dsl_types::{CrossWorkspaceConstraint, StateSelector};
+use crate::cross_workspace::DagRegistry;
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
@@ -273,12 +273,11 @@ fn required_states(c: &CrossWorkspaceConstraint) -> Vec<String> {
     }
 }
 
-fn severity_str(s: &dsl_core::config::dag::Severity) -> String {
-    use dsl_core::config::dag::Severity::*;
+fn severity_str(s: &dsl_core::DagSeverity) -> String {
     match s {
-        Error => "error",
-        Warning => "warning",
-        Informational => "informational",
+        dsl_core::DagSeverity::Error => "error",
+        dsl_core::DagSeverity::Warning => "warning",
+        dsl_core::DagSeverity::Informational => "informational",
     }
     .to_string()
 }
@@ -290,7 +289,7 @@ fn severity_str(s: &dsl_core::config::dag::Severity) -> String {
 #[cfg(test)]
 mod tests {
     use super::*;
-    use dsl_core::config::dag::{Dag, LoadedDag};
+    use dsl_types::{Dag, LoadedDag};
     use std::collections::BTreeMap;
     use std::path::PathBuf;
     use std::sync::Mutex;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs b/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
index 2fe05548..6ba68bd7 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
@@ -28,8 +28,8 @@
 
 use anyhow::Result;
 use async_trait::async_trait;
-use dsl_core::config::dag::CascadeRule;
-use dsl_core::config::DagRegistry;
+use dsl_types::CascadeRule;
+use crate::cross_workspace::DagRegistry;
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
@@ -204,12 +204,11 @@ impl CascadePlanner {
     }
 }
 
-fn severity_str(s: &dsl_core::config::dag::Severity) -> String {
-    use dsl_core::config::dag::Severity::*;
+fn severity_str(s: &dsl_core::DagSeverity) -> String {
     match s {
-        Error => "error",
-        Warning => "warning",
-        Informational => "informational",
+        dsl_core::DagSeverity::Error => "error",
+        dsl_core::DagSeverity::Warning => "warning",
+        dsl_core::DagSeverity::Informational => "informational",
     }
     .to_string()
 }
@@ -227,7 +226,7 @@ fn _silence_unused(_: &CascadeRule) {}
 mod tests {
     use super::*;
     use async_trait::async_trait;
-    use dsl_core::config::dag::{Dag, LoadedDag};
+    use dsl_types::{Dag, LoadedDag};
     use std::collections::BTreeMap;
     use std::path::PathBuf;
     use std::sync::Mutex;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/mod.rs b/rust/crates/dsl-runtime/src/cross_workspace/mod.rs
index 217185f3..cd69fc3d 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/mod.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/mod.rs
@@ -126,9 +126,12 @@
 //! submodules (legacy from ob-poc's conditional feature) are dropped.
 
 mod compensation;
+mod dag_registry;
 mod derived_state;
 mod derived_state_projector;
 mod fact_refs;
+
+pub(crate) use dag_registry::DagRegistry;
 mod fact_versions;
 mod gate_checker;
 mod hierarchy_cascade;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs b/rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs
index 92233c14..ca4fbaf4 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs
@@ -27,7 +27,7 @@
 
 use anyhow::{anyhow, Result};
 use async_trait::async_trait;
-use dsl_core::config::DagRegistry;
+use crate::cross_workspace::DagRegistry;
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
@@ -158,7 +158,7 @@ fn is_safe_ident(s: &str) -> bool {
 #[cfg(test)]
 mod tests {
     use super::*;
-    use dsl_core::config::dag::{Dag, LoadedDag};
+    use dsl_types::{Dag, LoadedDag};
     use std::collections::BTreeMap;
     use std::path::PathBuf;
 
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/test_harness/live.rs b/rust/crates/dsl-runtime/src/cross_workspace/test_harness/live.rs
index 94f5964e..7ae1620d 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/test_harness/live.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/test_harness/live.rs
@@ -11,7 +11,7 @@
 //! See `tests/cross_workspace_dag_live_scenarios.rs` for the entry point.
 
 use anyhow::{anyhow, Context, Result};
-use dsl_core::config::DagRegistry;
+use crate::cross_workspace::DagRegistry;
 use serde_json::Value as JsonValue;
 use sqlx::PgPool;
 use std::collections::HashMap;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/test_harness/mod.rs b/rust/crates/dsl-runtime/src/cross_workspace/test_harness/mod.rs
index 4c72b1bd..9c044fe3 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/test_harness/mod.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/test_harness/mod.rs
@@ -3,7 +3,7 @@
 //! Loads scenario YAMLs that declare initial state, predicate truth tables,
 //! and a sequence of operations (`check_transition`, `evaluate_derived`,
 //! `plan_cascade`, `mutate`). The runner constructs the real
-//! [`DagRegistry`](dsl_core::config::DagRegistry) from the workspace's
+//! [`DagRegistry`](crate::cross_workspace::DagRegistry) from the workspace's
 //! DAG taxonomy YAMLs, wires in-memory mock providers, fires each
 //! operation, and asserts on outcomes.
 //!
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs b/rust/crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs
index e9df046e..65dc7d1e 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs
@@ -1,7 +1,7 @@
 //! Scenario runner — loads YAML, wires mocks, executes steps, captures outcomes.
 
 use anyhow::{anyhow, bail, Context, Result};
-use dsl_core::config::DagRegistry;
+use crate::cross_workspace::DagRegistry;
 use sqlx::postgres::PgPoolOptions;
 use sqlx::PgPool;
 use std::collections::HashMap;
```

---

## 5. WHAT I DID NOT DO

### Residual Facade Decisions Left for Lockdown Tranche:
1. **`EffectClass`** (defined in `crates/dsl-core/src/executable_plan.rs`):
   - **Current Home:** Private module `dsl_core::executable_plan`.
   - **Why Stuck:** It is referenced by `dsl-runtime/src/coordination.rs` but is not re-exported in the public facade `dsl_core::*`. Per constraints, we cannot add a `pub use` in `dsl-core`'s `lib.rs` or modify `dsl-core` visibility. Therefore, resolving the remaining `E0603` compiler error on `EffectClass` must wait until a global facade mapping update or relocation of execution types to `dsl_types` is decided during the lockdown tranche.

### Workspace Quarantine Status:
- The workspace quarantine **cannot** be lifted at this stage. Since `dsl-runtime` does not compile cleanly due to the residual private import of `EffectClass`, the quarantine list (excluding `dsl-runtime` and its downstream dependents) must remain in place to keep the rest of the workspace green.

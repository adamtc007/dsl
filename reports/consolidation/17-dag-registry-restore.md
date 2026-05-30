# DagRegistry Restoration Review Document

## 1. COMMANDS RUN
1. `rg -n 'dsl_core::config::DagRegistry' crates/`
2. `cargo check -p dsl-runtime` (before edits)
3. `cargo check -p dsl-runtime` (after edits)
4. `cargo check --workspace --exclude dsl-runtime --exclude dsl-lsp --exclude ob-poc --exclude ob-poc-web --exclude ob-poc-agent --exclude sem_os_harness --exclude sem_os_postgres --exclude sem_os_server --exclude xtask --all-features`
5. `git diff`
6. `git status --porcelain`
7. `cat crates/dsl-runtime/src/cross_workspace/dag_registry.rs`

---

## 2. THE EDIT

The `DagRegistry` module was restored into `crates/dsl-runtime/src/cross_workspace/dag_registry.rs` with the verbatim body from `06232bf^:crates/dsl-core/src/config/dag_registry.rs`, with imports updated as follows:
- Glob imports were replaced with explicit imports of only used types from `dsl_types` and `dsl_core`.
- Visibility of `DagRegistry` and its fields/methods is kept strictly as `pub(crate)`.
- Re-pointed consumers in `derived_state_projector.rs`, `gate_checker.rs`, `hierarchy_cascade.rs`, `postgres_child_resolver.rs`, `test_harness/live.rs`, and `test_harness/runner.rs`.

---

## 3. EVIDENCE

### A. Repoint Verification (should be empty/0 hits)
```
$ rg -n 'dsl_core::config::DagRegistry' crates/
```

### B. Cargo Check BEFORE Edit
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
error[E0432]: unresolved import `dsl_types`
  --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:27:5
   |
27 | use dsl_types::{
   |     ^^^^^^^^^ use of unresolved module or unlinked crate `dsl_types`
   |
   = help: if you wanted to use a crate named `dsl_types`, use `cargo add dsl_types` to add it to your `Cargo.toml`

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

error[E0308]: mismatched types
   --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:375:36
    |
375 |                         workspace: ws.clone(),
    |                                    ^^^^^^^^^^ expected `String`, found `&str`
    |
help: try using a conversion method
    |
375 -                         workspace: ws.clone(),
375 +                         workspace: ws.to_string(),
    |

error[E0308]: mismatched types
   --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:387:36
    |
387 |                         workspace: ws.clone(),
    |                                    ^^^^^^^^^^ expected `String`, found `&str`
    |
help: try using a conversion method
    |
387 -                         workspace: ws.clone(),
387 +                         workspace: ws.to_string(),
    |

error[E0308]: mismatched types
   --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:402:50
    |
402 | ...                   declaring_workspace: ws.clone(),
    |                                            ^^^^^^^^^^ expected `String`, found `&str`
    |
help: try using a conversion method
    |
402 -                             declaring_workspace: ws.clone(),
402 +                             declaring_workspace: ws.to_string(),
    |

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

Some errors have detailed explanations: E0170, E0308, E0432, E0603.
For more information about an error, try `rustc --explain E0170`.
warning: `dsl-runtime` (lib) generated 10 warnings
error: could not compile `dsl-runtime` (lib) due to 21 previous errors; 10 warnings emitted
```

### C. Cargo Check AFTER Edit
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

### D. Quarantined Workspace Gate Check (GREEN)
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
```

### E. Workspace Git Status and Diff

#### git status --porcelain
```
$ git status --porcelain
M rust/Cargo.lock
 M rust/Cargo.toml
 M rust/crates/dsl-runtime/Cargo.toml
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
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs b/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs
index 06ab2b86..5c023da3 100644
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
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs b/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
index c8387a21..7b1b0bfd 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs
@@ -38,7 +38,7 @@
 use anyhow::Result;
 use async_trait::async_trait;
 use dsl_core::config::dag::{CrossWorkspaceConstraint, StateSelector};
-use dsl_core::config::DagRegistry;
+use crate::cross_workspace::DagRegistry;
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
diff --git a/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs b/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
index 2fe05548..160de4e0 100644
--- a/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
+++ b/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs
@@ -29,7 +29,7 @@
 use anyhow::Result;
 use async_trait::async_trait;
 use dsl_core::config::dag::CascadeRule;
-use dsl_core::config::DagRegistry;
+use crate::cross_workspace::DagRegistry;
 use sqlx::PgPool;
 use std::sync::Arc;
 use uuid::Uuid;
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
index 92233c14..05177f78 100644
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

#### mod.rs diff only
```
$ git diff rust/crates/dsl-runtime/src/cross_workspace/mod.rs
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
```

### F. Restored dag_registry.rs in Full
```rust
//! Runtime-side DAG registry — pre-indexed lookups for v1.3 enforcement.
//!
//! [`load_dags_from_dir`] / [`Dag`] (in `dag.rs`) give us the typed,
//! parsed shape. [`validate_dags`] (in `dag_validator.rs`) is the
//! build-time check.
//!
//! This module sits between the two: a runtime-loaded snapshot of all
//! DAGs that pre-computes the indices needed for hot-path checks:
//!
//!   * V1.3-1 cross_workspace_constraints — given (workspace, slot, from,
//!     to), find any constraints to enforce as a blocking gate.
//!   * V1.3-2 derived_cross_workspace_state — given (workspace, slot),
//!     find any derived states hosted on it (for evaluation at hydration).
//!   * V1.3-3 parent_slot — given (workspace, slot), find the parent slot
//!     reference (for cascade propagation).
//!
//! The registry itself is read-only after construction and meant to be
//! held in an `Arc<DagRegistry>` shared across the runtime. Reload is a
//! full re-construction (cheap; 9 DAGs at present).
//!
//! Pure data structure — no I/O at lookup time, no DB, no async. The
//! heavy lifting (running the source-state SQL queries that constraints
//! gate on, evaluating predicates) is done by callers using the registry
//! plus a SlotStateProvider (separate crate).

use dsl_core::load_dags_from_dir;
use dsl_types::{
    CrossWorkspaceConstraint, Dag, DerivedCrossWorkspaceState, DualLifecycle, LoadedDag,
    ParentSlot, Slot, SlotStateMachine, StateDependency, StateSelector, TransitionDef,
};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

// ---------------------------------------------------------------------------
// Index keys
// ---------------------------------------------------------------------------

/// (workspace, slot) — addresses a single slot within a single workspace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SlotKey {
    pub(crate) workspace: String,
    pub(crate) slot: String,
}

impl SlotKey {
    pub(crate) fn new(workspace: impl Into<String>, slot: impl Into<String>) -> Self {
        Self {
            workspace: workspace.into(),
            slot: slot.into(),
        }
    }
}

/// Identifies a transition for constraint matching: target_workspace,
/// target_slot, and the (from, to) pair. `from` is `None` when the
/// constraint matches `* -> to` (any source state).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TransitionKey {
    pub(crate) workspace: String,
    pub(crate) slot: String,
    pub(crate) from_state: Option<String>,
    pub(crate) to_state: String,
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// Pre-indexed snapshot of all loaded DAGs.
///
/// Construction parses each DAG's `cross_workspace_constraints` /
/// `derived_cross_workspace_state` / per-slot `parent_slot` into hashmaps
/// keyed for the runtime's hot lookup paths.
#[derive(Debug, Clone, Default)]
pub(crate) struct DagRegistry {
    /// All DAGs by workspace name. Ownership lives here.
    dags: BTreeMap<String, Dag>,

    /// constraints_by_target[TransitionKey] → list of indices into
    /// the owning Dag's cross_workspace_constraints. Stores
    /// (workspace, index) for re-lookup against `dags`.
    constraints_by_target: HashMap<TransitionKey, Vec<ConstraintLocator>>,

    /// derived_states_by_host[SlotKey] → list of (workspace, index)
    /// into the owning Dag's derived_cross_workspace_state.
    derived_states_by_host: HashMap<SlotKey, Vec<DerivedStateLocator>>,

    /// parent_slot_by_child[SlotKey of child] → (parent SlotKey,
    /// the workspace+slot index where the child slot is declared).
    parent_slot_by_child: HashMap<SlotKey, ParentSlotLocator>,

    /// transitions_by_verb_fqn[verb_fqn] → list of transitions that
    /// declare this verb in their `via:` field. Used by runtime to
    /// answer "what transitions could this verb cause?" — the
    /// foundation for hooking GateChecker into verb dispatch.
    transitions_by_verb_fqn: HashMap<String, Vec<TransitionRef>>,

    /// children_by_parent[parent SlotKey] → list of child SlotKeys
    /// (slots whose parent_slot points back to the parent). Reverse
    /// of parent_slot_by_child; used by V1.3-3 cascade planning to
    /// answer "given a parent transition, which child slots need to
    /// react?".
    children_by_parent: HashMap<SlotKey, Vec<SlotKey>>,
}

/// A reference to a single declared transition, materialised for
/// verb-fqn lookup. `from` may be a comma-list / parenthesised group
/// in the source YAML (e.g. `(PROSPECT, QUALIFYING) -> CANCELLED`);
/// the parser flattens this into one TransitionRef per source state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TransitionRef {
    pub(crate) workspace: String,
    pub(crate) slot: String,
    pub(crate) from_state: String,
    pub(crate) to_state: String,
    /// Whether this transition lives in a `dual_lifecycle:` chain
    /// (rather than the slot's primary `state_machine`).
    pub(crate) from_dual_lifecycle: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstraintLocator {
    workspace: String,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DerivedStateLocator {
    workspace: String,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParentSlotLocator {
    parent: SlotKey,
    declaring_workspace: String,
    declaring_slot_index: usize,
}

impl DagRegistry {
    /// Build a registry from an already-loaded map (e.g. from
    /// [`load_dags_from_dir`]).
    pub(crate) fn from_loaded(loaded: BTreeMap<String, LoadedDag>) -> Self {
        let mut registry = DagRegistry::default();
        for (ws, ld) in loaded {
            registry.dags.insert(ws, ld.dag);
        }
        registry.rebuild_indices();
        registry
    }

    /// Convenience: load + index from disk in one call.
    pub(crate) fn from_dir(dir: &Path) -> anyhow::Result<Self> {
        let loaded = load_dags_from_dir(dir)?;
        Ok(DagRegistry::from_loaded(loaded))
    }

    /// Number of workspaces / DAGs in the registry.
    pub(crate) fn len(&self) -> usize {
        self.dags.len()
    }

    /// Whether the registry is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.dags.is_empty()
    }

    /// Borrow the DAG for a given workspace, if loaded.
    pub(crate) fn dag(&self, workspace: &str) -> Option<&Dag> {
        self.dags.get(workspace)
    }

    /// Iterate over all loaded DAGs.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&String, &Dag)> {
        self.dags.iter()
    }

    // -----------------------------------------------------------------------
    // V1.3-1 lookups
    // -----------------------------------------------------------------------

    /// Find all cross_workspace_constraints whose target_transition matches
    /// the given (workspace, slot, from_state → to_state). Both
    /// from-specific (`A -> B`) and wildcard (`* -> B`) constraints are
    /// returned.
    ///
    /// Result is borrow-anchored on the registry; callers can pull the
    /// constraint's source_workspace / source_slot / source_state /
    /// source_predicate / severity to drive enforcement.
    pub(crate) fn constraints_for_transition(
        &self,
        workspace: &str,
        slot: &str,
        from_state: &str,
        to_state: &str,
    ) -> Vec<&CrossWorkspaceConstraint> {
        let mut out = Vec::new();
        // Specific from-state lookup.
        let key = TransitionKey {
            workspace: workspace.to_string(),
            slot: slot.to_string(),
            from_state: Some(from_state.to_string()),
            to_state: to_state.to_string(),
        };
        if let Some(locators) = self.constraints_by_target.get(&key) {
            for loc in locators {
                if let Some(c) = self.lookup_constraint(loc) {
                    out.push(c);
                }
            }
        }
        // Wildcard `* -> to_state` lookup.
        let wildcard_key = TransitionKey {
            workspace: workspace.to_string(),
            slot: slot.to_string(),
            from_state: None,
            to_state: to_state.to_string(),
        };
        if let Some(locators) = self.constraints_by_target.get(&wildcard_key) {
            for loc in locators {
                if let Some(c) = self.lookup_constraint(loc) {
                    out.push(c);
                }
            }
        }
        out
    }

    fn lookup_constraint(&self, loc: &ConstraintLocator) -> Option<&CrossWorkspaceConstraint> {
        self.dags
            .get(&loc.workspace)
            .and_then(|dag| dag.cross_workspace_constraints.get(loc.index))
    }

    // -----------------------------------------------------------------------
    // V1.3-2 lookups
    // -----------------------------------------------------------------------

    /// Find all derived_cross_workspace_state entries hosted on (workspace,
    /// slot). Used at hydration / aggregate-evaluation time.
    pub(crate) fn derived_states_for_slot(
        &self,
        workspace: &str,
        slot: &str,
    ) -> Vec<&DerivedCrossWorkspaceState> {
        let key = SlotKey::new(workspace, slot);
        let mut out = Vec::new();
        if let Some(locators) = self.derived_states_by_host.get(&key) {
            for loc in locators {
                if let Some(ds) = self.lookup_derived_state(loc) {
                    out.push(ds);
                }
            }
        }
        out
    }

    fn lookup_derived_state(
        &self,
        loc: &DerivedStateLocator,
    ) -> Option<&DerivedCrossWorkspaceState> {
        self.dags
            .get(&loc.workspace)
            .and_then(|dag| dag.derived_cross_workspace_state.get(loc.index))
    }

    // -----------------------------------------------------------------------
    // V1.3-3 lookups
    // -----------------------------------------------------------------------

    /// Find the parent slot reference for a given (workspace, slot), if
    /// declared. Returns `(parent_workspace, parent_slot)`.
    pub(crate) fn parent_slot_for(&self, workspace: &str, slot: &str) -> Option<&ParentSlot> {
        let key = SlotKey::new(workspace, slot);
        let loc = self.parent_slot_by_child.get(&key)?;
        let dag = self.dags.get(&loc.declaring_workspace)?;
        dag.slots
            .get(loc.declaring_slot_index)
            .and_then(|s| s.parent_slot.as_ref())
    }

    /// Get the resolved parent SlotKey for a child slot, with the
    /// child's declaring workspace as the default if `parent_slot.workspace`
    /// is unset.
    pub(crate) fn parent_slot_key(&self, workspace: &str, slot: &str) -> Option<SlotKey> {
        self.parent_slot_by_child
            .get(&SlotKey::new(workspace, slot))
            .map(|loc| loc.parent.clone())
    }

    // -----------------------------------------------------------------------
    // Verb → transitions lookup (foundation for verb-dispatch gate hook)
    // -----------------------------------------------------------------------

    /// Find all transitions a verb participates in (across all DAGs,
    /// all slots, both primary state machines and dual_lifecycle chains).
    ///
    /// Returns a slice of `TransitionRef` — borrowed from the registry's
    /// internal index, valid for the lifetime of the registry.
    ///
    /// Used by the runtime to answer "what transitions could this verb
    /// cause?" — the input to deciding which gate checks apply.
    pub(crate) fn transitions_for_verb(&self, verb_fqn: &str) -> &[TransitionRef] {
        self.transitions_by_verb_fqn
            .get(verb_fqn)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Find all child slots whose parent_slot points to (workspace,
    /// slot). Used by V1.3-3 cascade planning to answer "given a
    /// parent transitioned to state X, which children need to react?"
    pub(crate) fn children_of(&self, parent_workspace: &str, parent_slot: &str) -> &[SlotKey] {
        self.children_by_parent
            .get(&SlotKey::new(parent_workspace, parent_slot))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Look up a child slot's `state_dependency` block, if declared.
    /// Returns the cascade rules that govern how the child reacts to
    /// parent state changes.
    pub(crate) fn state_dependency_for(&self, workspace: &str, slot: &str) -> Option<&StateDependency> {
        let dag = self.dags.get(workspace)?;
        dag.slots
            .iter()
            .find(|s| s.id == slot)
            .and_then(|s| s.state_dependency.as_ref())
    }

    // -----------------------------------------------------------------------
    // Index construction
    // -----------------------------------------------------------------------

    fn rebuild_indices(&mut self) {
        self.constraints_by_target.clear();
        self.derived_states_by_host.clear();
        self.parent_slot_by_child.clear();
        self.transitions_by_verb_fqn.clear();
        self.children_by_parent.clear();

        for (ws, dag) in &self.dags {
            // Verb → transition index: walk every slot's primary +
            // dual lifecycle transitions, extract verb FQNs from `via:`,
            // and record one TransitionRef per (from_state, to_state)
            // pair the verb is declared on.
            for slot in &dag.slots {
                if let Some(SlotStateMachine::Structured(sm)) = &slot.state_machine {
                    for t in &sm.transitions {
                        index_transition(ws, &slot.id, t, false, &mut self.transitions_by_verb_fqn);
                    }
                }
                for dl in &slot.dual_lifecycle {
                    for t in &dl.transitions {
                        index_transition(ws, &slot.id, t, true, &mut self.transitions_by_verb_fqn);
                    }
                }
            }

            // V1.3-1 cross_workspace_constraints — index by target
            // transition.
            for (idx, c) in dag.cross_workspace_constraints.iter().enumerate() {
                let (from, to) = parse_transition(&c.target_transition);
                let key = TransitionKey {
                    workspace: c.target_workspace.clone(),
                    slot: c.target_slot.clone(),
                    from_state: from,
                    to_state: to,
                };
                self.constraints_by_target
                    .entry(key)
                    .or_default()
                    .push(ConstraintLocator {
                        workspace: ws.clone(),
                        index: idx,
                    });
            }

            // V1.3-2 derived_cross_workspace_state — index by host.
            for (idx, d) in dag.derived_cross_workspace_state.iter().enumerate() {
                let key = SlotKey::new(&d.host_workspace, &d.host_slot);
                self.derived_states_by_host
                    .entry(key)
                    .or_default()
                    .push(DerivedStateLocator {
                        workspace: ws.clone(),
                        index: idx,
                    });
            }

            // V1.3-3 parent_slot — index by child + reverse index by parent.
            for (slot_idx, slot) in dag.slots.iter().enumerate() {
                if let Some(parent) = &slot.parent_slot {
                    let parent_ws = parent.workspace.clone().unwrap_or_else(|| ws.clone());
                    let child_key = SlotKey::new(ws, &slot.id);
                    let parent_key = SlotKey::new(&parent_ws, &parent.slot);
                    self.parent_slot_by_child.insert(
                        child_key.clone(),
                        ParentSlotLocator {
                            parent: parent_key.clone(),
                            declaring_workspace: ws.clone(),
                            declaring_slot_index: slot_idx,
                        },
                    );
                    self.children_by_parent
                        .entry(parent_key)
                        .or_default()
                        .push(child_key);
                }
            }
        }
    }
}

/// Index a single TransitionDef into the verb→transition map.
fn index_transition(
    workspace: &str,
    slot: &str,
    t: &TransitionDef,
    from_dual: bool,
    out: &mut HashMap<String, Vec<TransitionRef>>,
) {
    let verbs = extract_verbs_from_via(&t.via);
    if verbs.is_empty() {
        return;
    }
    let froms = extract_from_states(&t.from);
    for verb_fqn in verbs {
        for from_state in &froms {
            // Skip transitions whose `from` we couldn't parse as a
            // state id (e.g. `"(any non-terminal)"` free-text escape) —
            // those are documentation, not enforceable transitions.
            if !is_valid_state_id(from_state) {
                continue;
            }
            out.entry(verb_fqn.clone())
                .or_default()
                .push(TransitionRef {
                    workspace: workspace.to_string(),
                    slot: slot.to_string(),
                    from_state: from_state.clone(),
                    to_state: t.to.clone(),
                    from_dual_lifecycle: from_dual,
                });
        }
    }
}

/// Pull verb FQNs from a `via:` field that may be a string, a list,
/// or a backend-marker string like `"(backend: ...)"` (in which case
/// no verbs are returned — backend transitions aren't verb-driven).
fn extract_verbs_from_via(via: &Option<serde_yaml::Value>) -> Vec<String> {
    let Some(v) = via else { return Vec::new() };
    match v {
        serde_yaml::Value::String(s) => {
            // "(backend: ...)" / "(time-decay)" / "(implicit: ...)" etc
            // are documentation strings, not verb FQNs.
            if s.trim().starts_with('(') {
                return Vec::new();
            }
            vec![s.clone()]
        }
        serde_yaml::Value::Sequence(seq) => seq
            .iter()
            .filter_map(|item| match item {
                serde_yaml::Value::String(s) if !s.trim().starts_with('(') => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Pull from-state ids from a `from:` field.
///
/// The `from:` field can be:
///   - A bare state id: `"PROSPECT"` or `PROSPECT` (string)
///   - A list (YAML sequence): `[PROSPECT, QUALIFYING]`
///   - A quoted parenthesised group: `"(PROSPECT, QUALIFYING)"` —
///     parsed by splitting on commas inside the parens.
///   - A free-text descriptor: `"(any non-terminal)"` — unparseable;
///     returned as-is for caller filtering.
fn extract_from_states(from: &serde_yaml::Value) -> Vec<String> {
    match from {
        serde_yaml::Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.starts_with('(') && trimmed.ends_with(')') {
                let inner = &trimmed[1..trimmed.len() - 1];
                inner
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect()
            } else {
                vec![trimmed.to_string()]
            }
        }
        serde_yaml::Value::Sequence(seq) => seq
            .iter()
            .filter_map(|item| match item {
                serde_yaml::Value::String(s) => Some(s.trim().to_string()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Heuristic: a state id is a single token of letters / digits /
/// underscores. Excludes free-text escapes like "any non-terminal".
fn is_valid_state_id(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Parse a `"FROM -> TO"` or `"* -> TO"` string into (Option<from>, to).
/// Whitespace tolerant. Returns ("", "") for malformed input — validator
/// catches structural errors at build time, so runtime tolerates oddities
/// silently.
fn parse_transition(s: &str) -> (Option<String>, String) {
    let parts: Vec<&str> = s.split("->").map(|p| p.trim()).collect();
    if parts.len() != 2 {
        return (None, String::new());
    }
    let from = if parts[0] == "*" {
        None
    } else {
        Some(parts[0].to_string())
    };
    (from, parts[1].to_string())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::dag::Dag;
    use std::path::PathBuf;

    fn ws_dag(yaml: &str) -> LoadedDag {
        let dag: Dag = serde_yaml::from_str(yaml).unwrap();
        LoadedDag {
            source_path: PathBuf::new(),
            dag,
        }
    }

    fn registry_from(workspaces: &[(&str, &str)]) -> DagRegistry {
        let mut map = BTreeMap::new();
        for (name, yaml) in workspaces {
            map.insert(name.to_string(), ws_dag(yaml));
        }
        DagRegistry::from_loaded(map)
    }

    #[test]
    fn empty_registry() {
        let r = DagRegistry::default();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
        assert!(r.dag("foo").is_none());
    }

    #[test]
    fn basic_dag_loads() {
        let r = registry_from(&[(
            "demo",
            r#"
workspace: demo
dag_id: demo_dag
slots:
  - id: thing
    stateless: true
"#,
        )]);
        assert_eq!(r.len(), 1);
        assert_eq!(r.dag("demo").unwrap().slots.len(), 1);
    }

    #[test]
    fn cross_workspace_constraint_indexed_and_looked_up() {
        let r = registry_from(&[
            (
                "kyc",
                r#"
workspace: kyc
dag_id: kyc_dag
slots:
  - id: kyc_case
    stateless: false
    state_machine:
      id: kyc_case_lifecycle
      states: [{ id: APPROVED }]
"#,
            ),
            (
                "deal",
                r#"
workspace: deal
dag_id: deal_dag
cross_workspace_constraints:
  - id: deal_contracted_requires_kyc_approved
    source_workspace: kyc
    source_slot: kyc_case
    source_state: APPROVED
    target_workspace: deal
    target_slot: deal
    target_transition: "KYC_CLEARANCE -> CONTRACTED"
    severity: error
slots:
  - id: deal
    stateless: false
    state_machine:
      id: deal_lifecycle
      states:
        - { id: KYC_CLEARANCE, entry: true }
        - { id: CONTRACTED }
"#,
            ),
        ]);

        let hits = r.constraints_for_transition("deal", "deal", "KYC_CLEARANCE", "CONTRACTED");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].source_workspace, "kyc");
        assert_eq!(
            hits[0].source_state.as_ref(),
            Some(&StateSelector::Single("APPROVED".to_string())),
        );

        // No match for unrelated transition.
        let no_hits = r.constraints_for_transition("deal", "deal", "PROSPECT", "QUALIFYING");
        assert!(no_hits.is_empty());
    }

    #[test]
    fn wildcard_target_transition_matches_any_from() {
        let r = registry_from(&[(
            "deal",
            r#"
workspace: deal
dag_id: deal_dag
cross_workspace_constraints:
  - id: any_to_active_requires_billing
    source_workspace: deal
    source_slot: billing_profile
    source_state: ACTIVE
    target_workspace: deal
    target_slot: deal
    target_transition: "* -> ACTIVE"
    severity: error
slots:
  - id: deal
    stateless: false
    state_machine: { id: dl, states: [{ id: ACTIVE }] }
  - id: billing_profile
    stateless: false
    state_machine: { id: bpl, states: [{ id: ACTIVE }] }
"#,
        )]);

        // Should match transitions from any state into ACTIVE.
        let hits1 = r.constraints_for_transition("deal", "deal", "ONBOARDING", "ACTIVE");
        let hits2 = r.constraints_for_transition("deal", "deal", "SUSPENDED", "ACTIVE");
        assert_eq!(hits1.len(), 1);
        assert_eq!(hits2.len(), 1);

        // But not for transitions OUT of ACTIVE.
        let no = r.constraints_for_transition("deal", "deal", "ACTIVE", "SUSPENDED");
        assert!(no.is_empty());
    }

    #[test]
    fn derived_state_indexed_by_host() {
        let r = registry_from(&[(
            "cbu",
            r#"
workspace: cbu
dag_id: cbu_dag
slots:
  - id: cbu
    stateless: false
    state_machine: { id: cl, states: [{ id: VALIDATED }] }
derived_cross_workspace_state:
  - id: cbu_operationally_active
    host_workspace: cbu
    host_slot: cbu
    host_state: operationally_active
    derivation:
      all_of:
        - { workspace: kyc, slot: kyc_case, state: APPROVED }
"#,
        )]);

        let hits = r.derived_states_for_slot("cbu", "cbu");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].host_state, "operationally_active");

        let none = r.derived_states_for_slot("cbu", "evidence");
        assert!(none.is_empty());
    }

    #[test]
    fn parent_slot_indexed_by_child() {
        let r = registry_from(&[(
            "cbu",
            r#"
workspace: cbu
dag_id: cbu_dag
slots:
  - id: cbu
    stateless: false
    parent_slot:
      workspace: cbu
      slot: cbu
      join:
        via: cbu_entity_relationships
        parent_fk: parent_cbu_id
        child_fk: child_cbu_id
    state_machine: { id: cl, states: [{ id: VALIDATED }] }
"#,
        )]);

        let parent = r.parent_slot_for("cbu", "cbu").unwrap();
        assert_eq!(parent.slot, "cbu");
        assert_eq!(parent.workspace.as_deref(), Some("cbu"));

        let key = r.parent_slot_key("cbu", "cbu").unwrap();
        assert_eq!(key.workspace, "cbu");
        assert_eq!(key.slot, "cbu");

        // Slot without parent_slot returns None.
        let none = r.parent_slot_for("cbu", "nonexistent");
        assert!(none.is_none());
    }

    #[test]
    fn parent_slot_defaults_to_owning_workspace_when_omitted() {
        let r = registry_from(&[(
            "demo",
            r#"
workspace: demo
dag_id: demo_dag
slots:
  - id: child
    stateless: true
    parent_slot:
      slot: parent
"#,
        )]);

        let key = r.parent_slot_key("demo", "child").unwrap();
        assert_eq!(key.workspace, "demo"); // defaulted
        assert_eq!(key.slot, "parent");
    }

    #[test]
    fn parse_transition_helper() {
        assert_eq!(
            parse_transition("KYC_CLEARANCE -> CONTRACTED"),
            (Some("KYC_CLEARANCE".to_string()), "CONTRACTED".to_string()),
        );
        assert_eq!(
            parse_transition("* -> ACTIVE"),
            (None, "ACTIVE".to_string()),
        );
        assert_eq!(parse_transition("malformed"), (None, String::new()),);
    }

    #[test]
    fn verb_to_transition_index_single_via() {
        let r = registry_from(&[(
            "deal",
            r#"
workspace: deal
dag_id: deal_dag
slots:
  - id: deal
    stateless: false
    state_machine:
      id: dl
      states: [{ id: PROSPECT, entry: true }, { id: QUALIFYING }]
      transitions:
        - from: PROSPECT
          to: QUALIFYING
          via: deal.update-status
"#,
        )]);
        let hits = r.transitions_for_verb("deal.update-status");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].workspace, "deal");
        assert_eq!(hits[0].slot, "deal");
        assert_eq!(hits[0].from_state, "PROSPECT");
        assert_eq!(hits[0].to_state, "QUALIFYING");
        assert!(!hits[0].from_dual_lifecycle);
    }

    #[test]
    fn verb_to_transition_index_via_list() {
        let r = registry_from(&[(
            "deal",
            r#"
workspace: deal
dag_id: deal_dag
slots:
  - id: deal
    stateless: false
    state_machine:
      id: dl
      states: [{ id: PROSPECT, entry: true }, { id: QUALIFYING }]
      transitions:
        - from: PROSPECT
          to: QUALIFYING
          via: [deal.create, deal.update-status]
"#,
        )]);
        // Both verbs should index the same transition.
        let create_hits = r.transitions_for_verb("deal.create");
        let update_hits = r.transitions_for_verb("deal.update-status");
        assert_eq!(create_hits.len(), 1);
        assert_eq!(update_hits.len(), 1);
        assert_eq!(create_hits[0].to_state, "QUALIFYING");
        assert_eq!(update_hits[0].to_state, "QUALIFYING");
    }

    #[test]
    fn verb_to_transition_index_parenthesised_from() {
        let r = registry_from(&[(
            "deal",
            r#"
workspace: deal
dag_id: deal_dag
slots:
  - id: deal
    stateless: false
    state_machine:
      id: dl
      states:
        - { id: PROSPECT, entry: true }
        - { id: QUALIFYING }
        - { id: NEGOTIATING }
        - { id: CANCELLED }
      transitions:
        - from: "(PROSPECT, QUALIFYING, NEGOTIATING)"
          to: CANCELLED
          via: deal.cancel
"#,
        )]);
        let hits = r.transitions_for_verb("deal.cancel");
        assert_eq!(hits.len(), 3);
        let froms: Vec<&str> = hits.iter().map(|h| h.from_state.as_str()).collect();
        assert!(froms.contains(&"PROSPECT"));
        assert!(froms.contains(&"QUALIFYING"));
        assert!(froms.contains(&"NEGOTIATING"));
        assert!(hits.iter().all(|h| h.to_state == "CANCELLED"));
    }

    #[test]
    fn verb_to_transition_index_dual_lifecycle_marked() {
        let r = registry_from(&[(
            "deal",
            r#"
workspace: deal
dag_id: deal_dag
slots:
  - id: deal
    stateless: false
    state_machine:
      id: primary
      states: [{ id: CONTRACTED, entry: true }]
      transitions: []
    dual_lifecycle:
      - id: ops
        junction_state_from_primary: CONTRACTED
        states: [{ id: ONBOARDING }, { id: ACTIVE }]
        transitions:
          - from: ONBOARDING
            to: ACTIVE
            via: deal.update-status
"#,
        )]);
        let hits = r.transitions_for_verb("deal.update-status");
        assert_eq!(hits.len(), 1);
        assert!(hits[0].from_dual_lifecycle);
        assert_eq!(hits[0].from_state, "ONBOARDING");
        assert_eq!(hits[0].to_state, "ACTIVE");
    }

    #[test]
    fn verb_to_transition_index_skips_backend_via() {
        let r = registry_from(&[(
            "im",
            r#"
workspace: im
dag_id: im_dag
slots:
  - id: trading_activity
    stateless: false
    state_machine:
      id: ta
      states: [{ id: never_traded, entry: true }, { id: trading }]
      transitions:
        - from: never_traded
          to: trading
          via: "(backend: first trade posted)"
"#,
        )]);
        // No verbs declared; backend-marker via doesn't get indexed.
        // Only check that no real verb FQN got accidentally indexed.
        assert!(r.transitions_for_verb("backend").is_empty());
        assert!(r
            .transitions_for_verb("(backend: first trade posted)")
            .is_empty());
    }

    #[test]
    fn verb_to_transition_index_skips_unparseable_from() {
        let r = registry_from(&[(
            "kyc",
            r#"
workspace: kyc
dag_id: kyc_dag
slots:
  - id: kyc_case
    stateless: false
    state_machine:
      id: kc
      states: [{ id: BLOCKED }]
      transitions:
        - from: "(any non-terminal)"
          to: BLOCKED
          via: kyc-case.escalate
"#,
        )]);
        // The free-text from is not a state id; index drops it.
        let hits = r.transitions_for_verb("kyc-case.escalate");
        assert!(hits.is_empty());
    }

    #[test]
    fn loads_real_dags_from_disk() {
        // Live integration: registry should pick up all 9 DAGs cleanly
        // from the repo's actual dag_taxonomies/ directory.
        let path = std::path::Path::new("../../config/sem_os_seeds/dag_taxonomies");
        if !path.exists() {
            eprintln!("real DAG dir not present (test running outside repo) — skipping");
            return;
        }
        let r = DagRegistry::from_dir(path).expect("load real DAGs");
        assert!(r.len() >= 4, "expected at least the Tranche-2 DAGs to load");

        // CBU should have a derived_cross_workspace_state for the tollgate.
        let cbu_aggregates = r.derived_states_for_slot("cbu", "cbu");
        assert!(
            cbu_aggregates
                .iter()
                .any(|d| d.host_state == "operationally_active"),
            "expected cbu_operationally_active tollgate to be indexed"
        );

        // Deal should have its KYC-clearance constraint indexed.
        let deal_constraints =
            r.constraints_for_transition("deal", "deal", "IN_CLEARANCE", "CONTRACTED");
        assert!(
            deal_constraints
                .iter()
                .any(|c| c.id == "deal_contracted_requires_kyc_approved"),
            "expected deal contract gate to be indexed"
        );

        // deal.cancel should be indexed across many from-states.
        let cancel_hits = r.transitions_for_verb("deal.cancel");
        assert!(
            cancel_hits.len() >= 4,
            "expected deal.cancel to participate in multiple transitions; got {}",
            cancel_hits.len()
        );
        assert!(
            cancel_hits
                .iter()
                .all(|h| h.workspace == "deal" && h.slot == "deal"),
            "all deal.cancel transitions should target the deal slot"
        );
        assert!(
            cancel_hits.iter().any(|h| h.to_state == "CANCELLED"),
            "deal.cancel should have a transition into CANCELLED"
        );

        // deal.bac-approve is now an IN_CLEARANCE substate transition:
        // deal_status remains IN_CLEARANCE while bac_status moves forward.
        let bac_hits = r.transitions_for_verb("deal.bac-approve");
        assert_eq!(
            bac_hits.len(),
            1,
            "expected deal.bac-approve once: {bac_hits:?}"
        );
        assert_eq!(bac_hits[0].from_state, "IN_CLEARANCE");
        assert_eq!(bac_hits[0].to_state, "IN_CLEARANCE");
    }
}
```

---

## 4. WHAT I DID NOT DO
- Did not fix the E0603/E0170 compilation errors in `dsl-runtime`. These remain as the only compilation blocks to be resolved in a subsequent step.
- Did not lift the workspace quarantine.
- Did not run the restored module's tests (`dsl-runtime` does not compile fully yet due to the facade path E0603/E0170 errors).
- Did not introduce any `#[allow(...)]` overrides or wildcards.

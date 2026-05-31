# Track B Consolidation Receipt

## 1. Gate 1 Cargo Output (Verbatim)

### dsl workspace:
```text
Created At: 2026-05-31T12:14:08Z
Completed At: 2026-05-31T12:14:14Z

				The command failed with exit code: 101
				Output:
				<truncated 878 lines>

warning: `dsl-core` (test "tranche_d_facade_evidence") generated 1 warning
    Checking sem_os_policy v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_policy)
    Checking dsl-integration-tests v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl-integration-tests)
warning: type `dag_validator::DagWarning` is more private than the item `DagValidationReport::warnings`
   --> crates/dsl-core/src/config/dag_validator.rs:476:5
    |
476 |     pub warnings: Vec<DagWarning>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `DagValidationReport::warnings` is reachable at visibility `pub`
    |
note: but type `dag_validator::DagWarning` is only usable at visibility `pub(crate)`
   --> crates/dsl-core/src/config/dag_validator.rs:386:1
    |
386 | pub(crate) enum DagWarning {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(private_interfaces)]` on by default

warning: type `validator::StructuralError` is more private than the item `ValidationReport::structural`
   --> crates/dsl-core/src/config/validator.rs:295:5
    |
295 |     pub structural: Vec<StructuralError>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `ValidationReport::structural` is reachable at visibility `pub`
    |
note: but type `validator::StructuralError` is only usable at visibility `pub(crate)`
   --> crates/dsl-core/src/config/validator.rs:72:1
    |
 72 | pub(crate) enum StructuralError {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `to_dsl_string`, `to_user_dsl_string`, and `resolved_key` are never used
   --> crates/dsl-core/src/ast.rs:293:19
    |
287 | impl AstNode {
    | ------------ methods in this implementation
...
293 |     pub(crate) fn to_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^
...
329 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |                   ^^^^^^^^^^^^^^^^^^
...
475 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^

warning: method `span` is never used
   --> crates/dsl-core/src/ast.rs:968:19
    |
966 | impl FocusTarget {
    | ---------------- method in this implementation
967 |     /// Get the span of this target
968 |     pub(crate) fn span(&self) -> Span {
    |                   ^^^^

warning: method `span` is never used
    --> crates/dsl-core/src/ast.rs:1048:19
     |
1046 | impl NavTarget {
     | -------------- method in this implementation
1047 |     /// Get the span of this target
1048 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^

warning: method `names` is never used
   --> crates/dsl-core/src/binding_context.rs:111:19
    |
 88 | impl BindingContext {
    | ------------------- method in this implementation
...
111 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^

warning: method `name` is never used
  --> crates/dsl-core/src/config/runbook_composition.rs:93:19
   |
92 | impl AggregationRule {
   | -------------------- method in this implementation
93 |     pub(crate) fn name(&self) -> &str {
   |                   ^^^^

warning: method `name` is never used
   --> crates/dsl-core/src/config/runbook_composition.rs:161:19
    |
160 | impl CrossScopeRule {
    | ------------------- method in this implementation
161 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^

warning: method `imposes_order` is never used
   --> crates/dsl-core/src/execution_dag.rs:158:19
    |
153 | impl DagEdge {
    | ------------ method in this implementation
...
158 |     pub(crate) fn imposes_order(&self) -> bool {
    |                   ^^^^^^^^^^^^^

warning: `dsl-core` (lib test) generated 40 warnings (30 duplicates) (run `cargo fix --lib -p dsl-core --tests` to apply 1 suggestion)
error[E0308]: mismatched types
   --> crates/sem_os_policy/src/grounding.rs:179:13
    |
178 |         let state_allowed = match entry {
    |                                   ----- this expression has type `&dsl_types::constellation_map_def::VerbPaletteEntry`
179 |             VerbPaletteEntry::Simple(_) => true,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `VerbPaletteEntry`, found a different `VerbPaletteEntry`
    |
    = note: `VerbPaletteEntry` and `VerbPaletteEntry` have similar names, but are actually distinct types
note: `VerbPaletteEntry` is defined in crate `sem_os_ontology`
   --> crates/sem_os_ontology/src/constellation_map_def.rs:21:1
    |
 21 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: `VerbPaletteEntry` is defined in crate `dsl_types`
   --> crates/dsl_types/src/constellation_map_def.rs:211:1
    |
211 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> crates/sem_os_policy/src/grounding.rs:180:13
    |
178 |         let state_allowed = match entry {
    |                                   ----- this expression has type `&dsl_types::constellation_map_def::VerbPaletteEntry`
179 |             VerbPaletteEntry::Simple(_) => true,
180 |             VerbPaletteEntry::Gated { when, .. } => when
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `VerbPaletteEntry`, found a different `VerbPaletteEntry`
    |
    = note: `VerbPaletteEntry` and `VerbPaletteEntry` have similar names, but are actually distinct types
note: `VerbPaletteEntry` is defined in crate `sem_os_ontology`
   --> crates/sem_os_ontology/src/constellation_map_def.rs:21:1
    |
 21 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: `VerbPaletteEntry` is defined in crate `dsl_types`
   --> crates/dsl_types/src/constellation_map_def.rs:211:1
    |
211 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> crates/sem_os_policy/src/grounding.rs:201:50
    |
201 |                     action_kind: action_kind_for(entry).to_string(),
    |                                  --------------- ^^^^^ expected `VerbPaletteEntry`, found a different `VerbPaletteEntry`
    |                                  |
    |                                  arguments to this function are incorrect
    |
    = note: `VerbPaletteEntry` and `VerbPaletteEntry` have similar names, but are actually distinct types
note: `VerbPaletteEntry` is defined in crate `dsl_types`
   --> crates/dsl_types/src/constellation_map_def.rs:211:1
    |
211 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: `VerbPaletteEntry` is defined in crate `sem_os_ontology`
   --> crates/sem_os_ontology/src/constellation_map_def.rs:21:1
    |
 21 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: function defined here
   --> crates/sem_os_policy/src/grounding.rs:328:4
    |
328 | fn action_kind_for(entry: &VerbPaletteEntry) -> &'static str {
    |    ^^^^^^^^^^^^^^^ ------------------------

error[E0308]: mismatched types
   --> crates/sem_os_policy/src/grounding.rs:220:46
    |
220 |                 action_kind: action_kind_for(entry).to_string(),
    |                              --------------- ^^^^^ expected `VerbPaletteEntry`, found a different `VerbPaletteEntry`
    |                              |
    |                              arguments to this function are incorrect
    |
    = note: `VerbPaletteEntry` and `VerbPaletteEntry` have similar names, but are actually distinct types
note: `VerbPaletteEntry` is defined in crate `dsl_types`
   --> crates/dsl_types/src/constellation_map_def.rs:211:1
    |
211 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: `VerbPaletteEntry` is defined in crate `sem_os_ontology`
   --> crates/sem_os_ontology/src/constellation_map_def.rs:21:1
    |
 21 | pub enum VerbPaletteEntry {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: function defined here
   --> crates/sem_os_policy/src/grounding.rs:328:4
    |
328 | fn action_kind_for(entry: &VerbPaletteEntry) -> &'static str {
    |    ^^^^^^^^^^^^^^^ ------------------------

For more information about this error, try `rustc --explain E0308`.
error: could not compile `sem_os_policy` (lib) due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `sem_os_policy` (lib test) due to 4 previous errors


```

### ob-poc workspace:
```text
    Checking dsl_types v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl_types)
    Checking dsl-core v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl-core)
    Checking sem_os_ontology v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_ontology)
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
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:40:5
   |
40 |     validate_constellation_map_schema_coordination, DagWarning,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^

warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:47:5
   |
47 |     green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
48 |     GreenWhenExclusionReason,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `EvaluationContext` and `compute_effective_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:52:5
   |
52 |     compute_effective_tier, EvaluationContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:57:5
   |
57 |     compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
   |     ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `validator::StructuralError`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:72:16
   |
72 | pub(crate) use validator::StructuralError;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `executable_plan::TransactionPolicy`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:84:16
   |
84 | pub(crate) use executable_plan::TransactionPolicy;
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
444 |     pub(crate) fn is_literal(&self) -> bool {
    |                   ^^^^^^^^^^
...
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |                   ^^^^^^^
...
475 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
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

warning: `dsl-core` (lib) generated 95 warnings (run `cargo fix --lib -p dsl-core` to apply 11 suggestions)
    Checking sem_os_core v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_core)
    Checking ob-poc-compiler v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-compiler)
    Checking ob-poc-ontology v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-ontology)
    Checking dsl-semos-frontend v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-semos-frontend)
    Checking ob-agentic v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-agentic)
    Checking sem_os_policy v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_policy)
    Checking dsl-analysis v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-analysis)
    Checking sem_os_obpoc_adapter v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_obpoc_adapter)
warning: unused imports: `ActionClass`, `HarmClass`, and `VerbConsumes`
   --> crates/sem_os_obpoc_adapter/src/scanner.rs:976:9
    |
976 |         ActionClass, ArgConfig, ArgType, CrudConfig, CrudOperation, DomainConfig, HarmClass,
    |         ^^^^^^^^^^^                                                               ^^^^^^^^^
977 |         LookupConfig, SearchKeyConfig, VerbBehavior, VerbConfig, VerbConsumes, VerbLifecycle,
    |                                                                  ^^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `sem_os_obpoc_adapter` (lib test) generated 1 warning (run `cargo fix --lib -p sem_os_obpoc_adapter --tests` to apply 1 suggestion)
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

warning: unused imports: `DurableConfig` and `SearchKeyConfig`
  --> crates/dsl-analysis/src/runtime_registry.rs:24:85
   |
24 |     ArgConfig, ArgType, BatchPolicyConfig, CrudConfig, CrudOperation, DomainConfig, DurableConfig,
   |                                                                                     ^^^^^^^^^^^^^
...
27 |     SearchKeyConfig, VerbBehavior, VerbConfig, VerbConsumes, VerbLifecycle, VerbProduces,
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `dsl-analysis` (lib test) generated 1 warning (run `cargo fix --lib -p dsl-analysis --tests` to apply 1 suggestion)
warning: `dsl-analysis` (lib) generated 1 warning (run `cargo fix --lib -p dsl-analysis` to apply 1 suggestion)
    Checking dsl-runtime v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime)
    Checking sem_os_client v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_client)
    Checking ob-poc-boundary v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-boundary)
    Checking sem_os_mcp v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_mcp)
warning: unused imports: `DualLifecycle`, `Slot`, and `StateSelector`
  --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:28:64
   |
28 |     CrossWorkspaceConstraint, Dag, DerivedCrossWorkspaceState, DualLifecycle, LoadedDag,
   |                                                                ^^^^^^^^^^^^^
29 |     ParentSlot, Slot, SlotStateMachine, StateDependency, StateSelector, TransitionDef,
   |                 ^^^^                                     ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `DualLifecycle` and `Slot`
  --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:28:64
   |
28 |     CrossWorkspaceConstraint, Dag, DerivedCrossWorkspaceState, DualLifecycle, LoadedDag,
   |                                                                ^^^^^^^^^^^^^
29 |     ParentSlot, Slot, SlotStateMachine, StateDependency, StateSelector, TransitionDef,
   |                 ^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: methods `len`, `is_empty`, `dag`, `iter`, and `parent_slot_key` are never used
   --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:159:19
    |
140 | impl DagRegistry {
    | ---------------- methods in this implementation
...
159 |     pub(crate) fn len(&self) -> usize {
    |                   ^^^
...
164 |     pub(crate) fn is_empty(&self) -> bool {
    |                   ^^^^^^^^
...
169 |     pub(crate) fn dag(&self, workspace: &str) -> Option<&Dag> {
    |                   ^^^
...
174 |     pub(crate) fn iter(&self) -> impl Iterator<Item = (&String, &Dag)> {
    |                   ^^^^
...
285 |     pub(crate) fn parent_slot_key(&self, workspace: &str, slot: &str) -> Option<SlotKey> {
    |                   ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `iter` is never used
   --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:174:19
    |
140 | impl DagRegistry {
    | ---------------- method in this implementation
...
174 |     pub(crate) fn iter(&self) -> impl Iterator<Item = (&String, &Dag)> {
    |                   ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `dsl-runtime` (lib) generated 2 warnings (run `cargo fix --lib -p dsl-runtime` to apply 1 suggestion)
    Checking sem_os_postgres v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_postgres)
    Checking dsl-lsp v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-lsp)
    Checking ob-poc-agent v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-agent)
warning: `dsl-runtime` (lib test) generated 2 warnings (run `cargo fix --lib -p dsl-runtime --tests` to apply 1 suggestion)
    Checking ob-poc v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust)
    Checking sem_os_server v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_server)
    Checking sem_os_harness v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_harness)
error[E0432]: unresolved import `super::config::types::GraphQueryOperation`
  --> src/dsl_v2/graph_executor.rs:13:5
   |
13 | use super::config::types::GraphQueryOperation;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `GraphQueryOperation` in `dsl_v2::config::types`

error[E0432]: unresolved import `crate::dsl_v2::ast::find_unresolved_ref_locations`
  --> src/mcp/intent_pipeline.rs:46:5
   |
46 | use crate::dsl_v2::ast::find_unresolved_ref_locations;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `find_unresolved_ref_locations` in `dsl_v2::ast`

error[E0432]: unresolved import `crate::dsl_v2::ast::find_unresolved_ref_locations`
  --> src/agent/orchestrator.rs:28:5
   |
28 | use crate::dsl_v2::ast::find_unresolved_ref_locations;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `find_unresolved_ref_locations` in `dsl_v2::ast`

error[E0432]: unresolved imports `super::super::config::types::ArgConfig`, `super::super::config::types::CrudConfig`, `super::super::config::types::DomainConfig`, `super::super::config::types::ResolutionMode`, `super::super::config::types::VerbBehavior`, `super::super::config::types::VerbConfig`, `super::super::config::types::VerbsConfig`
   --> src/dsl_v2/enrichment.rs:262:9
    |
262 |         ArgConfig, ArgType, CrudConfig, CrudOperation, DomainConfig, LookupConfig, ResolutionMode,
    |         ^^^^^^^^^           ^^^^^^^^^^                 ^^^^^^^^^^^^                ^^^^^^^^^^^^^^ no `ResolutionMode` in `dsl_v2::config::types`
    |         |                   |                          |
    |         |                   |                          no `DomainConfig` in `dsl_v2::config::types`
    |         |                   no `CrudConfig` in `dsl_v2::config::types`
    |         no `ArgConfig` in `dsl_v2::config::types`
263 |         SearchKeyConfig, VerbBehavior, VerbConfig, VerbsConfig,
    |                          ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^ no `VerbsConfig` in `dsl_v2::config::types`
    |                          |             |
    |                          |             no `VerbConfig` in `dsl_v2::config::types`
    |                          no `VerbBehavior` in `dsl_v2::config::types`
    |
    = help: consider importing this enum instead:
            crate::semtaxonomy_v2::ResolutionMode
    = help: consider importing this variant instead:
            crate::api::acp_dsl_dag_coverage::AcpDslDagSurfaceKind::VerbConfig

error[E0432]: unresolved import `crate::dsl_v2::ast::find_unresolved_ref_locations`
    --> src/mcp/intent_pipeline.rs:1469:13
     |
1469 |         use crate::dsl_v2::ast::find_unresolved_ref_locations;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `find_unresolved_ref_locations` in `dsl_v2::ast`

error[E0432]: unresolved import `crate::dsl_v2::ast::find_unresolved_ref_locations`
    --> src/mcp/intent_pipeline.rs:1606:34
     |
1606 |         use crate::dsl_v2::ast::{find_unresolved_ref_locations, Statement};
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `find_unresolved_ref_locations` in `dsl_v2::ast`

error[E0425]: cannot find type `SourceSpan` in module `crate::dsl_v2::diagnostics`
   --> src/mcp/types.rs:130:39
    |
130 | impl From<crate::dsl_v2::diagnostics::SourceSpan> for Location {
    |                                       ^^^^^^^^^^ not found in `crate::dsl_v2::diagnostics`
    |
help: consider importing this struct through its public re-export
    |
  6 + use crate::session::SourceSpan;
    |
help: if you import `SourceSpan`, refer to it directly
    |
130 - impl From<crate::dsl_v2::diagnostics::SourceSpan> for Location {
130 + impl From<SourceSpan> for Location {
    |

error[E0425]: cannot find type `SourceSpan` in module `crate::dsl_v2::diagnostics`
   --> src/mcp/types.rs:131:47
    |
131 |     fn from(span: crate::dsl_v2::diagnostics::SourceSpan) -> Self {
    |                                               ^^^^^^^^^^ not found in `crate::dsl_v2::diagnostics`
    |
help: consider importing this struct through its public re-export
    |
  6 + use crate::session::SourceSpan;
    |
help: if you import `SourceSpan`, refer to it directly
    |
131 -     fn from(span: crate::dsl_v2::diagnostics::SourceSpan) -> Self {
131 +     fn from(span: SourceSpan) -> Self {
    |

error[E0603]: module `compiler` is private
  --> src/dsl_v2/mod.rs:81:19
   |
81 | pub use dsl_core::compiler;
   |                   ^^^^^^^^ private module
   |
note: the module `compiler` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:18:1
   |
18 | pub(crate) mod compiler;
   | ^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `compiler` is private
  --> src/dsl_v2/mod.rs:82:19
   |
82 | pub use dsl_core::compiler::{
   |                   ^^^^^^^^ private module
   |
note: the module `compiler` is defined here
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:18:1
   |
18 | pub(crate) mod compiler;
   | ^^^^^^^^^^^^^^^^^^^^^^^

error[E0624]: method `as_uuid` is private
   --> src/domain_ops/helpers.rs:370:35
    |
370 |     if let Some(uuid) = arg.value.as_uuid() {
    |                                   ^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:462:5
    |
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |     -------------------------------------------- private method defined here

error[E0624]: method `as_uuid` is private
   --> src/domain_ops/helpers.rs:379:46
    |
379 |                 if let Some(uuid) = items[2].as_uuid() {
    |                                              ^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:462:5
    |
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |     -------------------------------------------- private method defined here

error[E0599]: no function or associated item named `from_produces` found for struct `dsl_core::BindingInfo` in the current scope
   --> src/dsl_v2/csg_linter.rs:636:61
    |
636 |                         pending_context.insert(BindingInfo::from_produces(binding_name, produces));
    |                                                             ^^^^^^^^^^^^^ function or associated item not found in `dsl_core::BindingInfo`
    |
help: there is an associated function `from_ref` with a similar name
   --> /Users/adamtc007/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/axum-core-0.4.5/src/extract/from_ref.rs:15:5
    |
 15 |     fn from_ref(input: &T) -> Self;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0624]: associated function `synthetic` is private
   --> src/dsl_v2/enrichment.rs:181:64
    |
181 |                     AstNode::Literal(Literal::String(s), Span::synthetic())
    |                                                                ^^^^^^^^^ private associated function
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
    |
678 |     pub(crate) fn synthetic() -> Self {
    |     --------------------------------- private associated function defined here

error[E0624]: associated function `synthetic` is private
   --> src/dsl_v2/enrichment.rs:212:65
    |
212 |                     AstNode::Literal(Literal::Uuid(uuid), Span::synthetic())
    |                                                                 ^^^^^^^^^ private associated function
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
    |
678 |     pub(crate) fn synthetic() -> Self {
    |     --------------------------------- private associated function defined here

error[E0624]: associated function `synthetic` is private
   --> src/dsl_v2/execution_plan.rs:830:21
    |
830 |         span: Span::synthetic(),
    |                     ^^^^^^^^^ private associated function
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
    |
678 |     pub(crate) fn synthetic() -> Self {
    |     --------------------------------- private associated function defined here

error[E0624]: method `arg_name` is private
    --> src/dsl_v2/semantic_validator.rs:1245:29
     |
1245 |         let arg_name = disc.arg_name();
     |                             ^^^^^^^^ private method
     |
    ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1791:5
     |
1791 |     pub(crate) fn arg_name(&self) -> &str {
     |     ------------------------------------- private method defined here

error[E0624]: associated function `synthetic` is private
   --> src/dsl_v2/submission.rs:547:60
    |
547 |                 AstNode::Literal(Literal::Uuid(*id), Span::synthetic())
    |                                                            ^^^^^^^^^ private associated function
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
    |
678 |     pub(crate) fn synthetic() -> Self {
    |     --------------------------------- private associated function defined here

error[E0624]: method `as_uuid` is private
   --> src/domain_ops/helpers.rs:35:41
    |
 35 |             if let Some(uuid) = a.value.as_uuid() {
    |                                         ^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:462:5
    |
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |     -------------------------------------------- private method defined here

error[E0624]: method `as_uuid` is private
   --> src/domain_ops/helpers.rs:64:41
    |
 64 |             if let Some(uuid) = a.value.as_uuid() {
    |                                         ^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:462:5
    |
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |     -------------------------------------------- private method defined here

error[E0624]: method `as_uuid` is private
   --> src/domain_ops/helpers.rs:179:41
    |
179 |             if let Some(uuid) = a.value.as_uuid() {
    |                                         ^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:462:5
    |
462 |     pub(crate) fn as_uuid(&self) -> Option<Uuid> {
    |     -------------------------------------------- private method defined here

error[E0624]: method `is_literal` is private
   --> src/dsl_v2/enrichment.rs:545:36
    |
545 |             assert!(name_arg.value.is_literal());
    |                                    ^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:444:5
    |
444 |     pub(crate) fn is_literal(&self) -> bool {
    |     --------------------------------------- private method defined here

error[E0599]: no method named `is_entity_ref` found for enum `dsl_core::AstNode` in the current scope
   --> src/dsl_v2/enrichment.rs:550:37
    |
550 |             assert!(juris_arg.value.is_entity_ref());
    |                                     ^^^^^^^^^^^^^ method not found in `dsl_core::AstNode`

error[E0624]: method `is_unresolved_entity_ref` is private
   --> src/dsl_v2/enrichment.rs:551:37
    |
551 |             assert!(juris_arg.value.is_unresolved_entity_ref());
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:416:5
    |
416 |     pub(crate) fn is_unresolved_entity_ref(&self) -> bool {
    |     ----------------------------------------------------- private method defined here

error[E0599]: no method named `is_entity_ref` found for enum `dsl_core::AstNode` in the current scope
   --> src/dsl_v2/enrichment.rs:620:38
    |
620 |             assert!(entity_arg.value.is_entity_ref());
    |                                      ^^^^^^^^^^^^^ method not found in `dsl_core::AstNode`

error[E0599]: no method named `is_entity_ref` found for enum `dsl_core::AstNode` in the current scope
   --> src/dsl_v2/enrichment.rs:631:36
    |
631 |             assert!(role_arg.value.is_entity_ref());
    |                                    ^^^^^^^^^^^^^ method not found in `dsl_core::AstNode`

error[E0599]: no method named `is_entity_ref` found for enum `dsl_core::AstNode` in the current scope
   --> src/dsl_v2/enrichment.rs:699:34
    |
699 |                 assert!(items[0].is_entity_ref());
    |                                  ^^^^^^^^^^^^^ method not found in `dsl_core::AstNode`

error[E0599]: no method named `is_entity_ref` found for enum `dsl_core::AstNode` in the current scope
   --> src/dsl_v2/enrichment.rs:700:34
    |
700 |                 assert!(items[1].is_entity_ref());
    |                                  ^^^^^^^^^^^^^ method not found in `dsl_core::AstNode`

error[E0624]: method `is_literal` is private
   --> src/dsl_v2/enrichment.rs:740:36
    |
740 |             assert!(name_arg.value.is_literal());
    |                                    ^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:444:5
    |
444 |     pub(crate) fn is_literal(&self) -> bool {
    |     --------------------------------------- private method defined here

error[E0599]: no method named `is_synthetic` found for struct `dsl_core::Span` in the current scope
    --> src/dsl_v2/execution_plan.rs:1581:32
     |
1581 |         assert!(synthetic.span.is_synthetic());
     |                                ^^^^^^^^^^^^ method not found in `dsl_core::Span`

error[E0599]: no method named `is_synthetic` found for struct `dsl_core::Span` in the current scope
    --> src/dsl_v2/execution_plan.rs:1587:25
     |
1587 |         assert!(!normal.is_synthetic());
     |                         ^^^^^^^^^^^^ method not found in `dsl_core::Span`

error[E0624]: associated function `synthetic` is private
    --> src/dsl_v2/execution_plan.rs:1589:31
     |
1589 |         let synthetic = Span::synthetic();
     |                               ^^^^^^^^^ private associated function
     |
    ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
     |
 678 |     pub(crate) fn synthetic() -> Self {
     |     --------------------------------- private associated function defined here

error[E0599]: no method named `is_synthetic` found for struct `dsl_core::Span` in the current scope
    --> src/dsl_v2/execution_plan.rs:1590:27
     |
1590 |         assert!(synthetic.is_synthetic());
     |                           ^^^^^^^^^^^^ method not found in `dsl_core::Span`

error[E0624]: method `to_dsl_string` is private
   --> src/api/session.rs:212:39
    |
212 |                 let context_line = vc.to_dsl_string();
    |                                       ^^^^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:110:5
    |
110 |     pub(crate) fn to_dsl_string(&self) -> String {
    |     -------------------------------------------- private method defined here

error[E0624]: method `discriminators` is private
    --> src/api/session.rs:379:14
     |
 379 |             .discriminators()
     |              ^^^^^^^^^^^^^^ private method
     |
    ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1417:5
     |
1417 |     pub(crate) fn discriminators(&self) -> &[SearchDiscriminator] {
     |     ------------------------------------------------------------- private method defined here

error[E0624]: method `to_user_dsl_string` is private
   --> src/api/session.rs:960:24
    |
960 |             .map(|s| s.to_user_dsl_string())
    |                        ^^^^^^^^^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:89:5
    |
 89 |     pub(crate) fn to_user_dsl_string(&self) -> String {
    |     ------------------------------------------------- private method defined here

error[E0624]: method `to_dsl_string` is private
   --> src/api/session.rs:970:24
    |
970 |             .map(|s| s.to_dsl_string())
    |                        ^^^^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:81:5
    |
 81 |     pub(crate) fn to_dsl_string(&self) -> String {
    |     -------------------------------------------- private method defined here

error[E0624]: method `to_dsl_string` is private
    --> src/api/session.rs:2538:24
     |
2538 |             .map(|s| s.to_dsl_string())
     |                        ^^^^^^^^^^^^^ private method
     |
    ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:81:5
     |
  81 |     pub(crate) fn to_dsl_string(&self) -> String {
     |     -------------------------------------------- private method defined here

error[E0624]: method `to_dsl_string` is private
   --> src/runbook/compiler.rs:679:29
    |
679 |             .map(Statement::to_dsl_string)
    |                             ^^^^^^^^^^^^^ private method
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:81:5
    |
 81 |     pub(crate) fn to_dsl_string(&self) -> String {
    |     -------------------------------------------- private method defined here

error[E0624]: associated function `synthetic` is private
   --> src/runbook/compiler.rs:757:61
    |
757 |     let value = AstNode::Literal(Literal::Uuid(uuid), Span::synthetic());
    |                                                             ^^^^^^^^^ private associated function
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
    |
678 |     pub(crate) fn synthetic() -> Self {
    |     --------------------------------- private associated function defined here

error[E0624]: associated function `synthetic` is private
   --> src/runbook/compiler.rs:766:29
    |
766 |                 span: Span::synthetic(),
    |                             ^^^^^^^^^ private associated function
    |
   ::: /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:678:5
    |
678 |     pub(crate) fn synthetic() -> Self {
    |     --------------------------------- private associated function defined here

Some errors have detailed explanations: E0425, E0432, E0599, E0603, E0624.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `ob-poc` (lib) due to 26 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `ob-poc` (lib test) due to 41 previous errors

```

## 2. Gate 2 Cargo Output (Verbatim)

### dsl workspace (`cargo check --workspace --all-targets`):
```text
warning: patch `bpmn-lite-engine v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-engine)` was not used in the crate graph
warning: patch `bpmn-lite-ffi-grpc v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-grpc)` was not used in the crate graph
warning: patch `bpmn-lite-ffi-http v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-http)` was not used in the crate graph
warning: patch `bpmn-lite-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-server)` was not used in the crate graph
warning: patch `bpmn-lite-store v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-store)` was not used in the crate graph
warning: patch `dmn-lite-bridge v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dmn-lite-bridge)` was not used in the crate graph
warning: patch `dsl-bus-client v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-client)` was not used in the crate graph
warning: patch `dsl-bus-protocol v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-protocol)` was not used in the crate graph
warning: patch `dsl-bus-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-server)` was not used in the crate graph
warning: patch `dsl-bus-storage v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-storage)` was not used in the crate graph
warning: patch `dsl-manifest v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-manifest)` was not used in the crate graph
warning: patch `ffi-catalogue v0.1.0 (/Users/adamtc007/dev/bpmn-lite/ffi-catalogue)` was not used in the crate graph
help: Check that the patched package version and available features are compatible
      with the dependency requirements. If the patch has a different version from
      what is locked in the Cargo.lock file, run `cargo update` to use the new
      version. This may also occur with an optional dependency that is not enabled.
warning: unused imports: `CategoryGated`, `ConditionalGate`, `DualLifecycle`, `EvidenceType`, `ParentJoin`, `ParentSlot`, `PeriodicReviewCadence`, `ProductModuleGates`, `PruneCascadeRule`, `PruneCascadeTarget`, `PrunePreValidation`, `ReviewScope`, `RiskTierOverride`, `StateDef`, `StateDependency`, `StateMachine`, and `TransitionDef`
  --> crates/dsl-core/src/config/dag.rs:26:5
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
  --> crates/dsl-core/src/config/dag.rs:34:5
   |
34 |     AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
   |     ^^^^^^^^^^               ^^^^^^^^^^^^^^^^^^^^^^^^^^^                         ^^^^^^^^^

warning: unused imports: `EntityQualifier` and `RelationScope`
 --> crates/dsl-core/src/config/predicate/mod.rs:6:22
  |
6 | pub(crate) use ast::{EntityQualifier, RelationScope};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `ast::State`
  --> crates/dsl-core/src/config/predicate/mod.rs:11:16
   |
11 | pub(crate) use ast::State;
   |                ^^^^^^^^^^

warning: unused import: `parser::ParseError`
  --> crates/dsl-core/src/config/predicate/mod.rs:13:16
   |
13 | pub(crate) use parser::ParseError;
   |                ^^^^^^^^^^^^^^^^^^

warning: unused import: `dag_validator::validate_constellation_map_schema_coordination`
  --> crates/dsl-core/src/config/mod.rs:40:16
   |
40 | pub(crate) use dag_validator::validate_constellation_map_schema_coordination;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
  --> crates/dsl-core/src/config/mod.rs:47:5
   |
47 |     green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
48 |     GreenWhenExclusionReason,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `EvaluationContext` and `compute_effective_tier`
  --> crates/dsl-core/src/config/mod.rs:52:5
   |
52 |     compute_effective_tier, EvaluationContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
  --> crates/dsl-core/src/config/mod.rs:57:5
   |
57 |     compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
   |     ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `executable_plan::TransactionPolicy`
  --> crates/dsl-core/src/lib.rs:89:16
   |
89 | pub(crate) use executable_plan::TransactionPolicy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `RelationScope` is more private than the item `predicate::ast::EntityRef::Scoped::scope`
   --> crates/dsl-core/src/config/predicate/ast.rs:76:9
    |
 76 |         scope: RelationScope,
    |         ^^^^^^^^^^^^^^^^^^^^ field `predicate::ast::EntityRef::Scoped::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(private_interfaces)]` on by default

warning: type `EntityQualifier` is more private than the item `EntitySetRef::qualifier`
  --> crates/dsl-core/src/config/predicate/ast.rs:86:5
   |
86 |     pub qualifier: Option<EntityQualifier>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::qualifier` is reachable at visibility `pub`
   |
note: but type `EntityQualifier` is only usable at visibility `pub(crate)`
  --> crates/dsl-core/src/config/predicate/ast.rs:93:1
   |
93 | pub(crate) enum EntityQualifier {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `RelationScope` is more private than the item `EntitySetRef::scope`
   --> crates/dsl-core/src/config/predicate/ast.rs:88:5
    |
 88 |     pub scope: Option<RelationScope>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `EntitySetRef::scope` is reachable at visibility `pub`
    |
note: but type `RelationScope` is only usable at visibility `pub(crate)`
   --> crates/dsl-core/src/config/predicate/ast.rs:100:1
    |
100 | pub(crate) enum RelationScope {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated items `integer`, `resolved_entity_ref`, `symbol_ref`, `resolved_key`, `with_resolved_key`, and `try_with_resolved_key` are never used
   --> crates/dsl-core/src/ast.rs:362:19
    |
287 | impl AstNode {
    | ------------ associated items in this implementation
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
479 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
...
558 |     pub(crate) fn with_resolved_key(&self, key: String) -> Self {
    |                   ^^^^^^^^^^^^^^^^^
...
568 |     pub(crate) fn try_with_resolved_key(&self, key: String) -> Result<Self, String> {
    |                   ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `merge` is never used
   --> crates/dsl-core/src/ast.rs:661:19
    |
655 | impl Span {
    | --------- associated function in this implementation
...
661 |     pub(crate) fn merge(a: Span, b: Span) -> Span {
    |                   ^^^^^

warning: function `find_unresolved_refs` is never used
   --> crates/dsl-core/src/ast.rs:750:15
    |
750 | pub(crate) fn find_unresolved_refs(program: &Program) -> Vec<&AstNode> {
    |               ^^^^^^^^^^^^^^^^^^^^

warning: methods `is_fully_resolved`, `resolved_count`, and `resolution_percentage` are never used
   --> crates/dsl-core/src/ast.rs:877:19
    |
875 | impl EntityRefStats {
    | ------------------- methods in this implementation
876 |     /// Returns true if all EntityRefs are resolved
877 |     pub(crate) fn is_fully_resolved(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^
...
882 |     pub(crate) fn resolved_count(&self) -> i32 {
    |                   ^^^^^^^^^^^^^^
...
887 |     pub(crate) fn resolution_percentage(&self) -> u8 {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: methods `span`, `verb_name`, and `to_dsl_string` are never used
    --> crates/dsl-core/src/ast.rs:981:19
     |
 979 | impl ViewportVerb {
     | ----------------- methods in this implementation
 980 |     /// Get the span of this verb
 981 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
 995 |     pub(crate) fn verb_name(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1009 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> crates/dsl-core/src/ast.rs:1070:19
     |
1068 | impl FocusTarget {
     | ---------------- methods in this implementation
1069 |     /// Get the span of this target
1070 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1084 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> crates/dsl-core/src/ast.rs:1124:19
     |
1122 | impl EnhanceArg {
     | --------------- method in this implementation
1123 |     /// Render the argument to DSL string
1124 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> crates/dsl-core/src/ast.rs:1150:19
     |
1148 | impl NavTarget {
     | -------------- methods in this implementation
1149 |     /// Get the span of this target
1150 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1159 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> crates/dsl-core/src/ast.rs:1181:19
     |
1179 | impl NavDirection {
     | ----------------- method in this implementation
1180 |     /// Render the direction to DSL string
1181 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: associated items `to_dsl_string` and `all` are never used
    --> crates/dsl-core/src/ast.rs:1236:19
     |
1234 | impl ViewType {
     | ------------- associated items in this implementation
1235 |     /// Render the view type to DSL string
1236 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1263 |     pub(crate) fn all() -> &'static [ViewType] {
     |                   ^^^

warning: associated items `to_dsl_string`, `min_confidence`, and `from_score` are never used
    --> crates/dsl-core/src/ast.rs:1300:19
     |
1298 | impl ConfidenceZone {
     | ------------------- associated items in this implementation
1299 |     /// Render the zone to DSL string
1300 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1321 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^
...
1331 |     pub(crate) fn from_score(score: f32) -> Self {
     |                   ^^^^^^^^^^

warning: methods `to_dsl_string`, `extension`, and `mime_type` are never used
    --> crates/dsl-core/src/ast.rs:1362:19
     |
1360 | impl ExportFormat {
     | ----------------- methods in this implementation
1361 |     /// Render the format to DSL string
1362 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1383 |     pub(crate) fn extension(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1393 |     pub(crate) fn mime_type(&self) -> &'static str {
     |                   ^^^^^^^^^

warning: methods `merge` and `names` are never used
   --> crates/dsl-core/src/binding_context.rs:105:19
    |
 99 | impl BindingContext {
    | ------------------- methods in this implementation
...
105 |     pub(crate) fn merge(&mut self, other: &BindingContext) {
    |                   ^^^^^
...
122 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^

warning: method `is_clean` is never used
   --> crates/dsl-core/src/config/dag_validator.rs:480:19
    |
479 | impl DagValidationReport {
    | ------------------------ method in this implementation
480 |     pub(crate) fn is_clean(&self) -> bool {
    |                   ^^^^^^^^

warning: enum `SchemaCoordinationKnownDeferred` is never used
   --> crates/dsl-core/src/config/dag_validator.rs:495:17
    |
495 | pub(crate) enum SchemaCoordinationKnownDeferred {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `harden_schema_coordination_warnings` is never used
   --> crates/dsl-core/src/config/dag_validator.rs:696:15
    |
696 | pub(crate) fn harden_schema_coordination_warnings(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_known_deferred_key` is never used
   --> crates/dsl-core/src/config/dag_validator.rs:714:4
    |
714 | fn schema_coordination_known_deferred_key(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_warning_to_error` is never used
   --> crates/dsl-core/src/config/dag_validator.rs:748:4
    |
748 | fn schema_coordination_warning_to_error(warning: DagWarning) -> DagError {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_source_name` is never used
   --> crates/dsl-core/src/config/dag_validator.rs:782:4
    |
782 | fn schema_coordination_source_name(location: &DagLocation) -> String {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `EvaluationContext` is never constructed
  --> crates/dsl-core/src/config/escalation.rs:29:19
   |
29 | pub(crate) struct EvaluationContext {
   |                   ^^^^^^^^^^^^^^^^^

warning: associated items `new`, `with_arg`, `with_entity_attr`, and `with_flag` are never used
  --> crates/dsl-core/src/config/escalation.rs:40:19
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
  --> crates/dsl-core/src/config/escalation.rs:72:15
   |
72 | pub(crate) fn evaluate_predicate(pred: &EscalationPredicate, ctx: &EvaluationContext) -> bool {
   |               ^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier` is never used
   --> crates/dsl-core/src/config/escalation.rs:128:15
    |
128 | pub(crate) fn compute_effective_tier(
    |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `compute_effective_tier_with_trace` is never used
   --> crates/dsl-core/src/config/escalation.rs:142:15
    |
142 | pub(crate) fn compute_effective_tier_with_trace<'a>(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `as_f64` is never used
   --> crates/dsl-core/src/config/escalation.rs:160:4
    |
160 | fn as_f64(v: &serde_json::Value) -> Option<f64> {
    |    ^^^^^^

warning: struct `GreenWhenCoverageRow` is never constructed
  --> crates/dsl-core/src/config/green_when_coverage.rs:12:19
   |
12 | pub(crate) struct GreenWhenCoverageRow {
   |                   ^^^^^^^^^^^^^^^^^^^^

warning: enum `GreenWhenExclusionReason` is never used
  --> crates/dsl-core/src/config/green_when_coverage.rs:24:17
   |
24 | pub(crate) enum GreenWhenExclusionReason {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `GreenWhenCoverageSummary` is never constructed
  --> crates/dsl-core/src/config/green_when_coverage.rs:31:19
   |
31 | pub(crate) struct GreenWhenCoverageSummary {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dags` is never used
  --> crates/dsl-core/src/config/green_when_coverage.rs:39:15
   |
39 | pub(crate) fn green_when_coverage_for_dags(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_for_dag` is never used
  --> crates/dsl-core/src/config/green_when_coverage.rs:54:15
   |
54 | pub(crate) fn green_when_coverage_for_dag(
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `green_when_coverage_summary` is never used
  --> crates/dsl-core/src/config/green_when_coverage.rs:82:15
   |
82 | pub(crate) fn green_when_coverage_summary(rows: &[GreenWhenCoverageRow]) -> GreenWhenCoverageSummary {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `row_for_state` is never used
  --> crates/dsl-core/src/config/green_when_coverage.rs:97:4
   |
97 | fn row_for_state(
   |    ^^^^^^^^^^^^^

warning: function `inbound_verbs_by_destination` is never used
   --> crates/dsl-core/src/config/green_when_coverage.rs:135:4
    |
135 | fn inbound_verbs_by_destination(transitions: &[TransitionDef]) -> BTreeMap<String, Vec<String>> {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `verbs_from_transition` is never used
   --> crates/dsl-core/src/config/green_when_coverage.rs:150:4
    |
150 | fn verbs_from_transition(transition: &TransitionDef) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^

warning: function `states_from_yaml_value` is never used
   --> crates/dsl-core/src/config/green_when_coverage.rs:158:4
    |
158 | fn states_from_yaml_value(value: &YamlValue) -> Vec<String> {
    |    ^^^^^^^^^^^^^^^^^^^^^^

warning: function `split_tupleish` is never used
   --> crates/dsl-core/src/config/green_when_coverage.rs:169:4
    |
169 | fn split_tupleish(value: &str) -> Vec<String> {
    |    ^^^^^^^^^^^^^^

warning: associated functions `entity_uuid`, `entity_uuid_binding`, and `natural_key` are never used
   --> crates/dsl-core/src/config/resource_dependency.rs:96:19
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
   --> crates/dsl-core/src/config/resource_dependency.rs:169:19
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
  --> crates/dsl-core/src/config/runbook_composition.rs:39:19
   |
39 | pub(crate) struct RunbookStep {
   |                   ^^^^^^^^^^^

warning: enum `AggregationRule` is never used
  --> crates/dsl-core/src/config/runbook_composition.rs:67:17
   |
67 | pub(crate) enum AggregationRule {
   |                 ^^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> crates/dsl-core/src/config/runbook_composition.rs:93:19
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
   --> crates/dsl-core/src/config/runbook_composition.rs:139:17
    |
139 | pub(crate) enum CrossScopeRule {
    |                 ^^^^^^^^^^^^^^

warning: methods `name`, `tier`, and `matches` are never used
   --> crates/dsl-core/src/config/runbook_composition.rs:161:19
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
   --> crates/dsl-core/src/config/runbook_composition.rs:212:15
    |
212 | pub(crate) fn compute_runbook_tier(
    |               ^^^^^^^^^^^^^^^^^^^^

warning: function `component_a` is never used
   --> crates/dsl-core/src/config/runbook_composition.rs:228:15
    |
228 | pub(crate) fn component_a(steps: &[RunbookStep]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_b` is never used
   --> crates/dsl-core/src/config/runbook_composition.rs:235:15
    |
235 | pub(crate) fn component_b(steps: &[RunbookStep], rules: &[AggregationRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: function `component_c` is never used
   --> crates/dsl-core/src/config/runbook_composition.rs:243:15
    |
243 | pub(crate) fn component_c(steps: &[RunbookStep], rules: &[CrossScopeRule]) -> ConsequenceTier {
    |               ^^^^^^^^^^^

warning: struct `CsgRulesConfig` is never constructed
  --> crates/dsl-core/src/config/types.rs:26:19
   |
26 | pub(crate) struct CsgRulesConfig {
   |                   ^^^^^^^^^^^^^^

warning: methods `is_simple` and `min_confidence` are never used
    --> crates/dsl-core/src/config/types.rs:1398:19
     |
1376 | impl SearchKeyConfig {
     | -------------------- methods in this implementation
...
1398 |     pub(crate) fn is_simple(&self) -> bool {
     |                   ^^^^^^^^^
...
1426 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^

warning: struct `ConstraintRule` is never constructed
    --> crates/dsl-core/src/config/types.rs:1910:19
     |
1910 | pub(crate) struct ConstraintRule {
     |                   ^^^^^^^^^^^^^^

warning: struct `WarningRule` is never constructed
    --> crates/dsl-core/src/config/types.rs:1920:19
     |
1920 | pub(crate) struct WarningRule {
     |                   ^^^^^^^^^^^

warning: struct `JurisdictionRule` is never constructed
    --> crates/dsl-core/src/config/types.rs:1932:19
     |
1932 | pub(crate) struct JurisdictionRule {
     |                   ^^^^^^^^^^^^^^^^

warning: struct `CompositeRule` is never constructed
    --> crates/dsl-core/src/config/types.rs:1944:19
     |
1944 | pub(crate) struct CompositeRule {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleCondition` is never constructed
    --> crates/dsl-core/src/config/types.rs:1955:19
     |
1955 | pub(crate) struct RuleCondition {
     |                   ^^^^^^^^^^^^^

warning: struct `RuleRequirement` is never constructed
    --> crates/dsl-core/src/config/types.rs:1973:19
     |
1973 | pub(crate) struct RuleRequirement {
     |                   ^^^^^^^^^^^^^^^

warning: struct `JurisdictionCondition` is never constructed
    --> crates/dsl-core/src/config/types.rs:1983:19
     |
1983 | pub(crate) struct JurisdictionCondition {
     |                   ^^^^^^^^^^^^^^^^^^^^^

warning: struct `AppliesTo` is never constructed
    --> crates/dsl-core/src/config/types.rs:1995:19
     |
1995 | pub(crate) struct AppliesTo {
     |                   ^^^^^^^^^

warning: enum `RuleSeverity` is never used
    --> crates/dsl-core/src/config/types.rs:2002:17
     |
2002 | pub(crate) enum RuleSeverity {
     |                 ^^^^^^^^^^^^

warning: associated function `warning` is never used
   --> crates/dsl-core/src/diagnostics.rs:150:19
    |
136 | impl Diagnostic {
    | --------------- associated function in this implementation
...
150 |     pub(crate) fn warning(code: DiagnosticCode, message: impl Into<String>) -> Self {
    |                   ^^^^^^^

warning: struct `PlanId` is never constructed
  --> crates/dsl-core/src/executable_plan.rs:43:19
   |
43 | pub(crate) struct PlanId(pub(crate) Uuid);
   |                   ^^^^^^

warning: associated function `new` is never used
  --> crates/dsl-core/src/executable_plan.rs:46:19
   |
45 | impl PlanId {
   | ----------- associated function in this implementation
46 |     pub(crate) fn new() -> Self {
   |                   ^^^

warning: struct `SemOsSnapshotId` is never constructed
  --> crates/dsl-core/src/executable_plan.rs:70:19
   |
70 | pub(crate) struct SemOsSnapshotId(pub(crate) u64);
   |                   ^^^^^^^^^^^^^^^

warning: enum `TransactionPolicy` is never used
   --> crates/dsl-core/src/executable_plan.rs:141:17
    |
141 | pub(crate) enum TransactionPolicy {
    |                 ^^^^^^^^^^^^^^^^^

warning: associated function `from_effect_classes` is never used
   --> crates/dsl-core/src/executable_plan.rs:162:19
    |
156 | impl TransactionPolicy {
    | ---------------------- associated function in this implementation
...
162 |     pub(crate) fn from_effect_classes(classes: impl IntoIterator<Item = EffectClass>) -> Self {
    |                   ^^^^^^^^^^^^^^^^^^^

warning: struct `AuthorityContext` is never constructed
   --> crates/dsl-core/src/executable_plan.rs:203:19
    |
203 | pub(crate) struct AuthorityContext {
    |                   ^^^^^^^^^^^^^^^^

warning: enum `InstructionInput` is never used
   --> crates/dsl-core/src/executable_plan.rs:240:17
    |
240 | pub(crate) enum InstructionInput {
    |                 ^^^^^^^^^^^^^^^^

warning: struct `RuntimeInstruction` is never constructed
   --> crates/dsl-core/src/executable_plan.rs:259:19
    |
259 | pub(crate) struct RuntimeInstruction {
    |                   ^^^^^^^^^^^^^^^^^^

warning: struct `ExecutablePlan` is never constructed
   --> crates/dsl-core/src/executable_plan.rs:311:19
    |
311 | pub(crate) struct ExecutablePlan {
    |                   ^^^^^^^^^^^^^^

warning: associated constant `FORMAT_VERSION` is never used
   --> crates/dsl-core/src/executable_plan.rs:347:22
    |
345 | impl ExecutablePlan {
    | ------------------- associated constant in this implementation
346 |     /// Current plan format version.
347 |     pub(crate) const FORMAT_VERSION: u32 = 1;
    |                      ^^^^^^^^^^^^^^

warning: struct `ExecutionStepSummary` is never constructed
   --> crates/dsl-core/src/executable_plan.rs:358:19
    |
358 | pub(crate) struct ExecutionStepSummary {
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: method `ordering_pairs` is never used
   --> crates/dsl-core/src/execution_dag.rs:208:19
    |
193 | impl PopulatedExecutionDag {
    | -------------------------- method in this implementation
...
208 |     pub(crate) fn ordering_pairs(&self) -> Vec<(usize, usize)> {
    |                   ^^^^^^^^^^^^^^

warning: `dsl-core` (lib) generated 84 warnings (run `cargo fix --lib -p dsl-core` to apply 10 suggestions)
warning: unused imports: `ConstellationMapDefBody` and `SeedConstellationMap`
 --> crates/dsl-core/src/config/dag_validator/integration_tests/dag_validator_gate.rs:9:17
  |
9 | use dsl_types::{ConstellationMapDefBody, SeedConstellationMap};
  |                 ^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^

warning: method `resolved_key` is never used
   --> crates/dsl-core/src/ast.rs:479:19
    |
287 | impl AstNode {
    | ------------ method in this implementation
...
479 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `span` is never used
    --> crates/dsl-core/src/ast.rs:1070:19
     |
1068 | impl FocusTarget {
     | ---------------- method in this implementation
1069 |     /// Get the span of this target
1070 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^

warning: method `span` is never used
    --> crates/dsl-core/src/ast.rs:1150:19
     |
1148 | impl NavTarget {
     | -------------- method in this implementation
1149 |     /// Get the span of this target
1150 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^

warning: method `names` is never used
   --> crates/dsl-core/src/binding_context.rs:122:19
    |
 99 | impl BindingContext {
    | ------------------- method in this implementation
...
122 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^

warning: method `name` is never used
  --> crates/dsl-core/src/config/runbook_composition.rs:93:19
   |
92 | impl AggregationRule {
   | -------------------- method in this implementation
93 |     pub(crate) fn name(&self) -> &str {
   |                   ^^^^

warning: method `name` is never used
   --> crates/dsl-core/src/config/runbook_composition.rs:161:19
    |
160 | impl CrossScopeRule {
    | ------------------- method in this implementation
161 |     pub(crate) fn name(&self) -> &str {
    |                   ^^^^

warning: function `check_resolved_template_slot` is never used
  --> crates/dsl-core/tests/tranche_d_facade_evidence.rs:43:8
   |
43 |     fn check_resolved_template_slot(template: &dsl_core::ResolvedTemplate) {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `dsl-core` (lib test) generated 32 warnings (25 duplicates) (run `cargo fix --lib -p dsl-core --tests` to apply 1 suggestion)
warning: `dsl-core` (test "tranche_d_facade_evidence") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s

```

### ob-poc workspace (`cargo check --workspace`):
```text
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

warning: unused import: `dag_validator::validate_constellation_map_schema_coordination`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:40:16
   |
40 | pub(crate) use dag_validator::validate_constellation_map_schema_coordination;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:47:5
   |
47 |     green_when_coverage_for_dag, green_when_coverage_for_dags, green_when_coverage_summary,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
48 |     GreenWhenExclusionReason,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `EvaluationContext` and `compute_effective_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:52:5
   |
52 |     compute_effective_tier, EvaluationContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/mod.rs:57:5
   |
57 |     compute_runbook_tier, AggregationRule, CrossScopeRule, RunbookStep,
   |     ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `executable_plan::TransactionPolicy`
  --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/lib.rs:89:16
   |
89 | pub(crate) use executable_plan::TransactionPolicy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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
    = note: `#[warn(private_interfaces)]` on by default

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

warning: associated items `integer`, `resolved_entity_ref`, `symbol_ref`, `resolved_key`, `with_resolved_key`, and `try_with_resolved_key` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:362:19
    |
287 | impl AstNode {
    | ------------ associated items in this implementation
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
479 |     pub(crate) fn resolved_key(&self) -> Option<&str> {
    |                   ^^^^^^^^^^^^
...
558 |     pub(crate) fn with_resolved_key(&self, key: String) -> Self {
    |                   ^^^^^^^^^^^^^^^^^
...
568 |     pub(crate) fn try_with_resolved_key(&self, key: String) -> Result<Self, String> {
    |                   ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `merge` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:661:19
    |
655 | impl Span {
    | --------- associated function in this implementation
...
661 |     pub(crate) fn merge(a: Span, b: Span) -> Span {
    |                   ^^^^^

warning: function `find_unresolved_refs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:750:15
    |
750 | pub(crate) fn find_unresolved_refs(program: &Program) -> Vec<&AstNode> {
    |               ^^^^^^^^^^^^^^^^^^^^

warning: methods `is_fully_resolved`, `resolved_count`, and `resolution_percentage` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:877:19
    |
875 | impl EntityRefStats {
    | ------------------- methods in this implementation
876 |     /// Returns true if all EntityRefs are resolved
877 |     pub(crate) fn is_fully_resolved(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^
...
882 |     pub(crate) fn resolved_count(&self) -> i32 {
    |                   ^^^^^^^^^^^^^^
...
887 |     pub(crate) fn resolution_percentage(&self) -> u8 {
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: methods `span`, `verb_name`, and `to_dsl_string` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:981:19
     |
 979 | impl ViewportVerb {
     | ----------------- methods in this implementation
 980 |     /// Get the span of this verb
 981 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
 995 |     pub(crate) fn verb_name(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1009 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1070:19
     |
1068 | impl FocusTarget {
     | ---------------- methods in this implementation
1069 |     /// Get the span of this target
1070 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1084 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1124:19
     |
1122 | impl EnhanceArg {
     | --------------- method in this implementation
1123 |     /// Render the argument to DSL string
1124 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: methods `span` and `to_dsl_string` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1150:19
     |
1148 | impl NavTarget {
     | -------------- methods in this implementation
1149 |     /// Get the span of this target
1150 |     pub(crate) fn span(&self) -> Span {
     |                   ^^^^
...
1159 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: method `to_dsl_string` is never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1181:19
     |
1179 | impl NavDirection {
     | ----------------- method in this implementation
1180 |     /// Render the direction to DSL string
1181 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^

warning: associated items `to_dsl_string` and `all` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1236:19
     |
1234 | impl ViewType {
     | ------------- associated items in this implementation
1235 |     /// Render the view type to DSL string
1236 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1263 |     pub(crate) fn all() -> &'static [ViewType] {
     |                   ^^^

warning: associated items `to_dsl_string`, `min_confidence`, and `from_score` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1300:19
     |
1298 | impl ConfidenceZone {
     | ------------------- associated items in this implementation
1299 |     /// Render the zone to DSL string
1300 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1321 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^
...
1331 |     pub(crate) fn from_score(score: f32) -> Self {
     |                   ^^^^^^^^^^

warning: methods `to_dsl_string`, `extension`, and `mime_type` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/ast.rs:1362:19
     |
1360 | impl ExportFormat {
     | ----------------- methods in this implementation
1361 |     /// Render the format to DSL string
1362 |     pub(crate) fn to_dsl_string(&self) -> String {
     |                   ^^^^^^^^^^^^^
...
1383 |     pub(crate) fn extension(&self) -> &'static str {
     |                   ^^^^^^^^^
...
1393 |     pub(crate) fn mime_type(&self) -> &'static str {
     |                   ^^^^^^^^^

warning: methods `merge` and `names` are never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/binding_context.rs:105:19
    |
 99 | impl BindingContext {
    | ------------------- methods in this implementation
...
105 |     pub(crate) fn merge(&mut self, other: &BindingContext) {
    |                   ^^^^^
...
122 |     pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    |                   ^^^^^

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
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:696:15
    |
696 | pub(crate) fn harden_schema_coordination_warnings(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_known_deferred_key` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:714:4
    |
714 | fn schema_coordination_known_deferred_key(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_warning_to_error` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:748:4
    |
748 | fn schema_coordination_warning_to_error(warning: DagWarning) -> DagError {
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `schema_coordination_source_name` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/dag_validator.rs:782:4
    |
782 | fn schema_coordination_source_name(location: &DagLocation) -> String {
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

warning: methods `is_simple` and `min_confidence` are never used
    --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/config/types.rs:1398:19
     |
1376 | impl SearchKeyConfig {
     | -------------------- methods in this implementation
...
1398 |     pub(crate) fn is_simple(&self) -> bool {
     |                   ^^^^^^^^^
...
1426 |     pub(crate) fn min_confidence(&self) -> f32 {
     |                   ^^^^^^^^^^^^^^

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

warning: method `ordering_pairs` is never used
   --> /Users/adamtc007/dev/dsl/crates/dsl-core/src/execution_dag.rs:208:19
    |
193 | impl PopulatedExecutionDag {
    | -------------------------- method in this implementation
...
208 |     pub(crate) fn ordering_pairs(&self) -> Vec<(usize, usize)> {
    |                   ^^^^^^^^^^^^^^

warning: `dsl-core` (lib) generated 84 warnings (run `cargo fix --lib -p dsl-core` to apply 10 suggestions)
warning: unused imports: `DualLifecycle`, `Slot`, and `StateSelector`
  --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:28:64
   |
28 |     CrossWorkspaceConstraint, Dag, DerivedCrossWorkspaceState, DualLifecycle, LoadedDag,
   |                                                                ^^^^^^^^^^^^^
29 |     ParentSlot, Slot, SlotStateMachine, StateDependency, StateSelector, TransitionDef,
   |                 ^^^^                                     ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: methods `len`, `is_empty`, `dag`, and `parent_slot_key` are never used
   --> crates/dsl-runtime/src/cross_workspace/dag_registry.rs:159:19
    |
140 | impl DagRegistry {
    | ---------------- methods in this implementation
...
159 |     pub(crate) fn len(&self) -> usize {
    |                   ^^^
...
164 |     pub(crate) fn is_empty(&self) -> bool {
    |                   ^^^^^^^^
...
169 |     pub(crate) fn dag(&self, workspace: &str) -> Option<&Dag> {
    |                   ^^^
...
285 |     pub(crate) fn parent_slot_key(&self, workspace: &str, slot: &str) -> Option<SlotKey> {
    |                   ^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `dsl-runtime` (lib) generated 2 warnings (run `cargo fix --lib -p dsl-runtime` to apply 1 suggestion)
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

### ob-poc workspace (`cargo check --workspace --all-targets` - showing only environment-specific SQLX offline caching failures in 3 integration test files):
```text
    Checking ob-poc-types v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-types)
    Checking dsl_types v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl_types)
    Checking ob-poc-diagnostics v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-diagnostics)
    Checking ob-templates v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-templates)
    Checking ob-semantic-matcher v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-semantic-matcher)
    Checking ob-poc-bods v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-bods)
    Checking ob-workflow v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-workflow)
    Checking ob-poc-deal v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-deal)
    Checking ob-poc-booking-principal v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-booking-principal)
    Checking dsl-core v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl-core)
    Checking sem_os_ontology v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_ontology)
warning: unused imports: `CategoryGated`, `ConditionalGate`, `DualLifecycle`, `EvidenceType`, `ParentJoin`, `ParentSlot`, `PeriodicReviewCadence`, `ProductModuleGates`, `PruneCascadeRule`, `PruneCascadeTarget`, `PrunePreValidation`, `ReviewScope`, `RiskTierOverride`, `StateDef`, `StateDependency`, `StateMachine`, and `TransitionDef`
warning: unused imports: `AuditClass`, `CompletenessAssertionConfig`, and `RoleGuard`
warning: unused imports: `EntityQualifier` and `RelationScope`
warning: unused import: `ast::State`
warning: unused import: `parser::ParseError`
warning: unused import: `dag_validator::validate_constellation_map_schema_coordination`
warning: unused imports: `GreenWhenExclusionReason`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, and `green_when_coverage_summary`
warning: unused imports: `EvaluationContext` and `compute_effective_tier`
warning: unused imports: `AggregationRule`, `CrossScopeRule`, `RunbookStep`, and `compute_runbook_tier`
warning: unused import: `executable_plan::TransactionPolicy`
    Checking ob-poc-journey v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-journey)
    Checking ob-poc-trading-profile v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-trading-profile)
    Checking ob-poc-entity-linking v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-entity-linking)
    Checking bpmn-controller v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/bpmn-controller)
    Checking ob-poc-authoring v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-authoring)
    Checking ob-poc-taxonomy v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-taxonomy)
    Checking ob-poc-sage v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-sage)
    Checking ob-poc-semtaxonomy v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-semtaxonomy)
    Checking ob-poc-derived-attributes v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-derived-attributes)
    Checking inspector-projection v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/inspector-projection)
warning: type `RelationScope` is more private than the item `predicate::ast::EntityRef::Scoped::scope`
warning: type `EntityQualifier` is more private than the item `EntitySetRef::qualifier`
warning: type `RelationScope` is more private than the item `EntitySetRef::scope`
warning: associated items `integer`, `resolved_entity_ref`, `symbol_ref`, `resolved_key`, `with_resolved_key`, and `try_with_resolved_key` are never used
warning: associated function `merge` is never used
warning: function `find_unresolved_refs` is never used
warning: methods `is_fully_resolved`, `resolved_count`, and `resolution_percentage` are never used
warning: methods `span`, `verb_name`, and `to_dsl_string` are never used
warning: methods `span` and `to_dsl_string` are never used
warning: method `to_dsl_string` is never used
warning: methods `span` and `to_dsl_string` are never used
warning: method `to_dsl_string` is never used
warning: associated items `to_dsl_string` and `all` are never used
warning: associated items `to_dsl_string`, `min_confidence`, and `from_score` are never used
warning: methods `to_dsl_string`, `extension`, and `mime_type` are never used
warning: methods `merge` and `names` are never used
warning: method `is_clean` is never used
warning: enum `SchemaCoordinationKnownDeferred` is never used
warning: function `harden_schema_coordination_warnings` is never used
warning: function `schema_coordination_known_deferred_key` is never used
warning: function `schema_coordination_warning_to_error` is never used
748 | fn schema_coordination_warning_to_error(warning: DagWarning) -> DagError {
warning: function `schema_coordination_source_name` is never used
warning: struct `EvaluationContext` is never constructed
warning: associated items `new`, `with_arg`, `with_entity_attr`, and `with_flag` are never used
warning: function `evaluate_predicate` is never used
warning: function `compute_effective_tier` is never used
warning: function `compute_effective_tier_with_trace` is never used
warning: function `as_f64` is never used
warning: struct `GreenWhenCoverageRow` is never constructed
warning: enum `GreenWhenExclusionReason` is never used
warning: struct `GreenWhenCoverageSummary` is never constructed
warning: function `green_when_coverage_for_dags` is never used
warning: function `green_when_coverage_for_dag` is never used
warning: function `green_when_coverage_summary` is never used
warning: function `row_for_state` is never used
warning: function `inbound_verbs_by_destination` is never used
warning: function `verbs_from_transition` is never used
warning: function `states_from_yaml_value` is never used
warning: function `split_tupleish` is never used
warning: associated functions `entity_uuid`, `entity_uuid_binding`, and `natural_key` are never used
warning: associated functions `compile_resolved_entity`, `binding_resolved_entity`, and `runtime_create_natural_key` are never used
warning: struct `RunbookStep` is never constructed
warning: enum `AggregationRule` is never used
warning: methods `name`, `tier`, and `matches` are never used
warning: enum `CrossScopeRule` is never used
warning: methods `name`, `tier`, and `matches` are never used
warning: function `compute_runbook_tier` is never used
warning: function `component_a` is never used
warning: function `component_b` is never used
warning: function `component_c` is never used
warning: struct `CsgRulesConfig` is never constructed
warning: methods `is_simple` and `min_confidence` are never used
warning: struct `ConstraintRule` is never constructed
warning: struct `WarningRule` is never constructed
warning: struct `JurisdictionRule` is never constructed
warning: struct `CompositeRule` is never constructed
warning: struct `RuleCondition` is never constructed
warning: struct `RuleRequirement` is never constructed
warning: struct `JurisdictionCondition` is never constructed
warning: struct `AppliesTo` is never constructed
warning: enum `RuleSeverity` is never used
warning: associated function `warning` is never used
warning: struct `PlanId` is never constructed
warning: associated function `new` is never used
warning: struct `SemOsSnapshotId` is never constructed
warning: enum `TransactionPolicy` is never used
warning: associated function `from_effect_classes` is never used
warning: struct `AuthorityContext` is never constructed
warning: enum `InstructionInput` is never used
warning: struct `RuntimeInstruction` is never constructed
warning: struct `ExecutablePlan` is never constructed
warning: associated constant `FORMAT_VERSION` is never used
warning: struct `ExecutionStepSummary` is never constructed
warning: method `ordering_pairs` is never used
warning: `dsl-core` (lib) generated 84 warnings (run `cargo fix --lib -p dsl-core` to apply 10 suggestions)
    Checking sem_os_core v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_core)
    Checking ob-poc-compiler v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-compiler)
    Checking ob-poc-ontology v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-ontology)
    Checking ob-agentic v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-agentic)
    Checking sem_os_policy v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_policy)
    Checking dsl-analysis v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-analysis)
    Checking sem_os_obpoc_adapter v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_obpoc_adapter)
warning: unused imports: `CrudConfig`, `DomainConfig`, `DurableConfig`, `ReturnsConfig`, and `SearchKeyConfig`
warning: `dsl-analysis` (lib) generated 1 warning (run `cargo fix --lib -p dsl-analysis` to apply 1 suggestion)
    Checking dsl-runtime v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime)
    Checking ob-poc-boundary v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-boundary)
    Checking sem_os_client v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_client)
warning: unused imports: `DualLifecycle`, `Slot`, and `StateSelector`
warning: methods `len`, `is_empty`, `dag`, and `parent_slot_key` are never used
warning: `dsl-runtime` (lib) generated 2 warnings (run `cargo fix --lib -p dsl-runtime` to apply 1 suggestion)
    Checking sem_os_postgres v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_postgres)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
   = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query_scalar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
    = note: this error originates in the macro `$crate::sqlx_macros::expand_query` which comes from the expansion of the macro `sqlx::query` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not compile `ob-poc` (test "threshold_rfi_integration") due to 41 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `ob-poc` (test "capital_ownership_integration") due to 28 previous errors
error: could not compile `ob-poc` (test "incremental_session") due to 2 previous errors
```

## 3. git diff --stat

### dsl workspace:
```text
 crates/dsl-core/src/ast.rs                         | 128 +++++++++++++--
 crates/dsl-core/src/binding_context.rs             |  31 +++-
 crates/dsl-core/src/config/dag_validator.rs        |  12 +-
 crates/dsl-core/src/config/loader.rs               | 180 ++++++++++++++++++++-
 crates/dsl-core/src/config/mod.rs                  |  13 +-
 crates/dsl-core/src/config/pack_loader.rs          | 103 ++++++++++++
 crates/dsl-core/src/config/types.rs                |   4 +-
 crates/dsl-core/src/config/validator.rs            |  31 +++-
 crates/dsl-core/src/execution_dag.rs               |   4 +-
 crates/dsl-core/src/lib.rs                         |  11 +-
 crates/dsl_types/src/lib.rs                        |   2 +-
 .../sem_os_ontology/src/constellation_map_def.rs   |   2 +-
 12 files changed, 485 insertions(+), 36 deletions(-)

```

### ob-poc workspace:
```text
 rust/Cargo.lock                                    |  1 +
 .../src/cross_workspace/dag_registry.rs            | 22 ++++-----
 rust/crates/dsl-runtime/src/cross_workspace/mod.rs |  2 +-
 rust/crates/dsl-runtime/src/lib.rs                 |  2 +-
 rust/crates/ob-poc-web/src/main.rs                 |  6 ++-
 rust/src/bin/reconcile_resolver_manifest.rs        |  2 +-
 rust/src/dsl_v2/execution_plan.rs                  |  8 ++--
 rust/src/dsl_v2/executor.rs                        |  2 +-
 rust/src/dsl_v2/mod.rs                             | 54 +++++++++++++++++++---
 rust/src/runbook/compiler.rs                       |  2 +-
 rust/src/runbook/step_executor_bridge.rs           | 14 +++---
 rust/tests/phase5_coordination_harness.rs          |  4 +-
 rust/xtask/Cargo.toml                              |  1 +
 rust/xtask/src/dag_test.rs                         |  2 +-
 14 files changed, 84 insertions(+), 38 deletions(-)

```

## 4. Provenance Verification (Verbatim Restorations from Git History)

### Restored Symbol: VerbAvailability / VerbPaletteEntry
Provenance command: `git show 9bff82f7d6b958334eba0ac3810432da10415000^:crates/sem_os_ontology/src/constellation_map_def.rs`
Verbatim extract:
```rust
pub enum VerbPaletteEntry {
    Simple(String),
    Gated {
        verb: String,
        when: VerbAvailability,
    },
}
pub enum VerbAvailability {
    One(String),
    Many(Vec<String>),
}
```

### Restored Symbol: AstNode::is_entity_ref / Span::is_synthetic / find_unresolved_ref_locations
Provenance command: `git show f286ed5b88211e35d040814edf08897eca2aac1c^:crates/dsl-core/src/ast.rs`
Verbatim extract:
```rust
    pub fn is_entity_ref(&self) -> bool {
        matches!(self, AstNode::EntityRef { .. })
    }
    pub fn is_synthetic(&self) -> bool {
        self.start == usize::MAX && self.end == usize::MAX
    }
pub struct UnresolvedRefLocation {
    /// Statement index in AST (0-based)
    pub statement_index: usize,
    /// Argument key containing the EntityRef
    pub arg_key: String,
    /// Entity type for search (e.g., "cbu", "entity", "product")
    pub entity_type: String,
    /// The search text entered by user
    pub search_text: String,
    /// Search column from lookup config (e.g., "name")
    pub search_column: Option<String>,
    /// Unique ref_id for commit targeting (span-based, e.g., "0:15-30")
    pub ref_id: Option<String>,
}
pub fn find_unresolved_ref_locations(program: &Program) -> Vec<UnresolvedRefLocation> {
    let mut results = Vec::new();

    for (stmt_idx, stmt) in program.statements.iter().enumerate() {
        if let Statement::VerbCall(vc) = stmt {
            for arg in &vc.arguments {
                collect_unresolved_from_node(&arg.value, &arg.key, stmt_idx, &mut results);
            }
        }
    }

    results
}
```

### Restored Symbol: BindingInfo::from_produces / BindingContext::to_llm_context
Provenance command: `git show f286ed5b88211e35d040814edf08897eca2aac1c^:crates/dsl-core/src/binding_context.rs`
Verbatim extract:
```rust
    pub fn from_produces(name: &str, produces: &VerbProduces) -> Self {
        Self {
            name: name.to_string(),
            produced_type: produces.produced_type.clone(),
            subtype: produces.subtype.clone(),
            entity_pk: Uuid::nil(), // Not yet executed
            resolved: produces.resolved,
        }
    }
    pub fn to_llm_context(&self) -> String {
        if self.is_empty() {
            return "No bindings available.".to_string();
        }

        let mut lines = vec!["Available bindings:".to_string()];
        for info in self.bindings.values() {
            let pk_str = if info.entity_pk.is_nil() {
                "[pending]".to_string()
            } else {
                info.entity_pk.to_string()
            };
            lines.push(format!("  {} → pk: {}", info.display(), pk_str));
        }
        lines.join("\n")
    }
```

### Restored Symbol: entity_kinds_from_taxonomy_yaml
Provenance command: `git show 5b1f944f474ad96d103bb36c338b430cb04cf73f^:crates/dsl-core/src/config/dag_validator.rs`
Verbatim extract:
```rust
pub(crate) fn entity_kinds_from_taxonomy_yaml(yaml: &str) -> Result<HashSet<String>, serde_yaml::Error> {
    #[derive(serde::Deserialize)]
    struct EntityTaxonomy {
        #[serde(default)]
        entities: BTreeMap<String, serde_yaml::Value>,
    }

    let parsed: EntityTaxonomy = serde_yaml::from_str(yaml)?;
    Ok(parsed.entities.into_keys().collect())
}
```

### Restored Symbol: LoadedPack / load_packs_from_dir / flatten_pack_entries
Provenance command: `git show 467cd4264f477291166dfd1ab217f35f3959b061^:crates/dsl-core/src/config/pack_loader.rs`
Verbatim extract:
```rust
pub(crate) struct LoadedPack {
    pub name: String,
    pub source_path: PathBuf,
    pub workspaces: Vec<String>,
    pub allowed_verbs: Vec<String>,
}
pub(crate) fn load_packs_from_dir(packs_dir: &Path) -> Result<BTreeMap<String, LoadedPack>> {
    let mut out = BTreeMap::new();
    let entries =
        fs::read_dir(packs_dir).with_context(|| format!("cannot read packs dir {packs_dir:?}"))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
            continue;
        }
        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        let raw = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(?path, "pack load: {e}");
                continue;
            }
        };
        let parsed: Result<PackYaml, _> = serde_yaml::from_str(&raw);
        let pack = match parsed {
            Ok(p) => p,
            Err(e) => {
                tracing::warn!(?path, "pack parse: {e}");
                continue;
            }
        };
        let name = pack.id.unwrap_or(file_stem);
        out.insert(
            name.clone(),
            LoadedPack {
                name,
                source_path: path,
                workspaces: pack.workspaces,
                allowed_verbs: pack.allowed_verbs,
            },
        );
    }
    Ok(out)
}
pub(crate) fn flatten_pack_entries(
    packs: &BTreeMap<String, LoadedPack>,
) -> impl Iterator<Item = (String, String)> + '_ {
    packs.values().flat_map(|p| {
        p.allowed_verbs
            .iter()
            .map(move |fqn| (p.name.clone(), fqn.clone()))
    })
}
```

### Restored Symbol: collect_declared_fqns / validate_pack_fqns
Provenance command: `git show 3b8bf216823a5468d90cd543b25a4c922e971545^:crates/dsl-core/src/config/validator.rs`
Verbatim extract:
```rust
pub(crate) fn collect_declared_fqns(config: &VerbsConfig) -> HashSet<String> {
    let mut out = HashSet::new();
    for (domain_name, domain) in &config.domains {
        for verb_name in domain.verbs.keys() {
            out.insert(format!("{domain_name}.{verb_name}"));
        }
    }
    out
}
pub(crate) fn validate_pack_fqns(
    declared_verbs: &HashSet<String>,
    macro_fqns: &HashSet<String>,
    pack_entries: impl IntoIterator<Item = (String, String)>,
) -> Vec<WellFormednessError> {
    let mut errors = Vec::new();
    for (pack_name, fqn) in pack_entries {
        if declared_verbs.contains(&fqn) || macro_fqns.contains(&fqn) {
            continue;
        }
        errors.push(WellFormednessError::PackFqnWithoutDeclaration { pack_name, fqn });
    }
    errors
}
    fn collect_declared_fqns_aggregates_across_domains() {
        let yaml = r#"
version: "1.0"
domains:
  foo:
    description: "test"
    verbs:
      bar:
        description: "test"
        behavior: crud
      baz:
        description: "test"
        behavior: crud
  qux:
    description: "test"
    verbs:
      wobble:
        description: "test"
        behavior: crud
"#;
        let cfg: VerbsConfig = serde_yaml::from_str(yaml).unwrap();
        let declared = collect_declared_fqns(&cfg);
        assert_eq!(declared.len(), 3);
        assert!(declared.contains("foo.bar"));
        assert!(declared.contains("foo.baz"));
        assert!(declared.contains("qux.wobble"));
    }
```

## 5. Changed & Why Ledger (A1–A4 / B1–B2) and Invariant Verification

| Action | Description / Why |
|---|---|
| **A1** | Rebuilt `dsl_v2` facade in `ob-poc` (`rust/src/dsl_v2/mod.rs`) by enumerating module exports from `dsl_core` root. |
| **A2** | Narrowed direct `dsl_core::<mod>::X` references in `ob-poc` to `dsl_core::X`. |
| **A3** | Relocated `DagRegistry` to `dsl_runtime::cross_workspace`. Repointed all consumers to `dsl_runtime::cross_workspace::DagRegistry`. |
| **A4** | Restored `VerbAvailability` + `VerbPaletteEntry` verbatim into `sem_os_ontology::constellation_map_def` from their deletion commit. |
| **B1** | Widened method visibility (from `pub(crate)` to `pub`) for compiler-flagged symbols (e.g. `Program::to_dsl_string`, `Span::synthetic`, `DagEdge::imposes_order`, `available_types`) as needed by external targets. |
| **B2** | Restored dead D1 code verbatim from git history as requested by compilation checks on downstream integration tests and binaries. |

### Invariant Attestation:
- Lint suppression check (`allow(dead_code)` or `allow(unused)`): **CLEAN** (dsl diff: ``, ob-poc diff: ``)
- dsl-core module visibility check (no `pub mod` additions): **CLEAN** (lib.rs diff: ``)

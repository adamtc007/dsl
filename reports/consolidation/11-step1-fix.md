# Phase 2 — Step 1 Fix: Right-sizing the Leaf Surface

This report details the implementation and verification for Phase 2 — Step 1 Fix: Reverting lint suppression, tightening type exposures, and right-sizing the leaf visibility without any `#![allow]` or `#[allow]` attributes.

---

## 1. Refactoring Actions & Diff
We reverted the module-level lint suppression in `dsl-core/src/config/dag.rs` and right-sized the public visibility of the relocated types across both `dsl_types` and `dsl-core`:

1. **Reverted Suppressions**: Removed `#![allow(unreachable_pub)]` completely from `dsl-core/src/config/dag.rs`. No lint suppressions remain in the codebase for this fix.
2. **dsl_types module tightening**: Restricted the `dag` module in `crates/dsl_types/src/lib.rs` to `pub(crate) mod dag;`, preventing all 36 types from being automatically exposed as public.
3. **Selective re-exports in dsl_types**: Added a selective `pub use` in `crates/dsl_types/src/lib.rs` exporting only the 32 types that are actually used across the crate boundary (e.g. by `dsl-core` or external callers). The remaining 8 internal types (`OverallLifecycle`, `ProgressionVerbs`, `Derivation`, `StructuredDerivationCondition`, `PredicateRequiredUniverse`, `CrossSlotConstraint`, `ExposureConfig`, `Visibility`) are not re-exported, keeping the API surface minimal.
4. **Re-export tightening in dsl-core**: Split imports inside `crates/dsl-core/src/config/dag.rs` into two distinct groups:
   * `pub use` (re-exported publicly at the facade level for downstream crates) for the 13 types reached by external crates.
   * `pub(crate) use` (internal to `dsl-core`) for the remaining 24 types.

### Git Diff
```diff
diff --git a/crates/dsl-core/src/config/dag.rs b/crates/dsl-core/src/config/dag.rs
index 233b03e..fdac6d1 100644
--- a/crates/dsl-core/src/config/dag.rs
+++ b/crates/dsl-core/src/config/dag.rs
@@ -8,28 +8,29 @@
 //! parent_slot/state_dependency, expected_lifetime, dual_lifecycle,
 //! periodic_review_cadence, evidence_types, category_gated).
 
-#![allow(unreachable_pub)]
-
 use anyhow::{Context, Result};
 use std::collections::BTreeMap;
 use std::fs;
 use std::path::{Path, PathBuf};
 
 // Re-export all relocated types from dsl_types::dag module
-pub use dsl_types::dag::{
-    Dag, OverallLifecycle, Phase, ProgressionVerbs, Derivation, DerivationCondition,
-    StructuredDerivationCondition, StateSelector, Slot, SlotStateMachine, StateMachine,
-    PredicateBinding, PredicateRequiredUniverse, PredicateBindingSourceKind, ExpectedLifetime,
-    StateDef, EntryVia, TransitionDef, CrossSlotConstraint, Severity,
-    CrossWorkspaceConstraint, DerivedCrossWorkspaceState, ExposureConfig, Visibility,
-    ParentSlot, ParentJoin, StateDependency, CascadeRule, DualLifecycle,
+// Re-exports reached by external crates (facade/config re-exports)
+pub use dsl_types::{
+    Dag, Phase, DerivationCondition, StateSelector, Slot, SlotStateMachine,
+    PredicateBinding, Severity, CrossWorkspaceConstraint, DerivedCrossWorkspaceState,
+    CascadeRule, EntryVia, LoadedDag,
+};
+
+// Re-exports reached only inside dsl-core
+pub(crate) use dsl_types::{
+    StateMachine, StateDef, TransitionDef, ParentSlot, ParentJoin, StateDependency,
     PeriodicReviewCadence, RiskTierOverride, ReviewScope, EvidenceType, CategoryGated,
     ProductModuleGates, ConditionalGate, PruneCascadeRule, PruneCascadeTarget,
-    PrunePreValidation, LoadedDag,
+    PrunePreValidation, ExpectedLifetime, PredicateBindingSourceKind, DualLifecycle,
 };
 
-// Re-export level-0 types previously imported/used in this module
-pub use dsl_types::{
+// Re-exports of level-0 types reached only inside dsl-core
+pub(crate) use dsl_types::{
     AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
 };
 
diff --git a/crates/dsl-core/src/config/green_when_coverage.rs b/crates/dsl-core/src/config/green_when_coverage.rs
index 2dc2f88..ecedbc7 100644
--- a/crates/dsl-core/src/config/green_when_coverage.rs
+++ b/crates/dsl-core/src/config/green_when_coverage.rs
@@ -4,7 +4,7 @@
 //! predicates are backfilled workspace by workspace. This module deliberately
 //! reports coverage; it does not invent predicates.
 
-use dsl_types::dag::{Dag, SlotStateMachine, StateDef, TransitionDef};
+use dsl_types::{Dag, SlotStateMachine, StateDef, TransitionDef};
 use serde_yaml::Value as YamlValue;
 use std::collections::{BTreeMap, BTreeSet, HashSet};
 
diff --git a/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs b/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs
index 5b99390..0408180 100644
--- a/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs
+++ b/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs
@@ -1,5 +1,5 @@
 use crate::config::dag::load_dags_from_dir;
-use dsl_types::dag::SlotStateMachine;
+use dsl_types::SlotStateMachine;
 use crate::config::predicate::{
     parse_green_when, CmpOp, EntityQualifier, EntityRef, EntitySetRef, Predicate, RelationScope,
     Validity,
diff --git a/crates/dsl_types/src/lib.rs b/crates/dsl_types/src/lib.rs
index 45d311a..4c568a1 100644
--- a/crates/dsl_types/src/lib.rs
+++ b/crates/dsl_types/src/lib.rs
@@ -22,7 +22,7 @@
 
 pub(crate) mod constellation_map_def;
 pub(crate) mod resolver_facts;
-pub mod dag;
+pub(crate) mod dag;
 
 pub use constellation_map_def::{
     AuditClass, Cardinality, ClosureType, CompletenessAssertionConfig, ConstellationMapDefBody,
@@ -30,3 +30,12 @@ pub use constellation_map_def::{
     VerbPaletteEntry,
 };
 pub use resolver_facts::StructuralFacts;
+
+pub use dag::{
+    Dag, LoadedDag, Slot, SlotStateMachine, StateMachine, StateDef, TransitionDef, Severity,
+    CrossWorkspaceConstraint, DerivedCrossWorkspaceState, CascadeRule, EntryVia, Phase,
+    StateSelector, PredicateBinding, DerivationCondition, ParentSlot, ParentJoin,
+    StateDependency, DualLifecycle, PeriodicReviewCadence, RiskTierOverride, ReviewScope,
+    EvidenceType, CategoryGated, ProductModuleGates, ConditionalGate, PruneCascadeRule,
+    PruneCascadeTarget, PrunePreValidation, ExpectedLifetime, PredicateBindingSourceKind,
+};
```

---

## 2. unreachable_pub Clean Result
Following the clean cargo build run, `unreachable_pub` is fully satisfied and produces **no warnings or errors** across the workspace. 

* **Command**: `cargo check --workspace --all-features` (after `cargo clean`)
* **Warnings/Errors on fixed paths**: None.
* **Allows Added**: None.

---

## 3. Pinned Test Set Verification Results
The test suite was run against the pinned `ob-poc` commit SHA (`68e9be40361a36a2f3925e83960bc07238f210b6`).

Below is the per-test pass/fail set comparison showing a 100% match of all passing and failing tests.

| Test ID | Baseline Result | Post-Fix Result | Status |
| :--- | :--- | :--- | :--- |
| `config::predicate::integration_tests::predicate_ast::confirmed_green_when_fixture_count_is_eighteen` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| `config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_is_tracked_per_workspace` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| `config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_baseline_is_explicit` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| `domain_pack::tests::all_domain_packs_reload_idempotently_and_cover_dsl_surfaces` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| *Doctest:* `crates/dsl-core/src/config/phrase_gen.rs - config::phrase_gen::generate_phrases (line 115)` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| *Doctest:* `crates/dsl-core/src/config/mod.rs - config (line 14)` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| *Doctest:* `crates/dsl-core/src/ast.rs - ast::count_entity_refs (line 800)` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| *Doctest:* `crates/dsl-core/src/viewport_parser.rs - viewport_parser (line 25)` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| *Doctest:* `crates/sem_os_core/src/frontier/hydrator.rs - frontier::hydrator::hydrate_frontier (line 17)` | **FAILED (Red)** | **FAILED (Red)** | Identical |
| *All other unit and integration tests (700+ tests)* | **PASSED** | **PASSED** | Identical |

No flips occurred; the pass/fail signature remains perfectly matched.

---

## 4. "WHAT I DID NOT DO" Ledger
In strict compliance with Phase 2 — Step 1 Fix scope limitations:
1. **No Lint Suppression**: Reverted `#![allow(unreachable_pub)]` completely. Did not introduce any other `#![allow]` or `#[allow]` attributes.
2. **No Logic or Behavior Mutations**: Did not alter any of the struct/enum definitions or structural parser/loader/validator logic.
3. **No Projections or Wire-ups**: Did not construct any projection matrices or runtime gating-status mappings.
4. **No Test Logic Modification**: Adjusted only the import paths in test-site files without modifying any test bodies or logic.
5. **No Proceeding to Step 2/3**: Stopped immediately upon completing and verifying the Step 1 Fix.

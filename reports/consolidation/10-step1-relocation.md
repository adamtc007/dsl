# Phase 2 — Step 1: Relocation of Topology Types to `dsl_types`

This report details the implementation and verification for Phase 2 — Step 1: Relocating DAG topology types to `dsl_types`.

---

## 1. Implementation Guard Result
Before modifying any source files, we ran a structural guard check on all `impl` blocks in `crates/dsl-core/src/config/dag.rs` to identify any inherent implementation coupling to `dsl-core` compiler or validation structures.

* **Scan Query**: `grep -rn "impl " crates/dsl-core/src/config/dag.rs`
* **Findings**: Only one implementation block was found:
  ```rust
  impl Default for ProgressionVerbs {
      fn default() -> Self {
          ProgressionVerbs::List(Vec::new())
      }
  }
  ```
* **Verdict**: **CLEAN**. There are no inherent implementation blocks referencing parser, validator, or non-family dependencies. No compiler logic is tied to these structs, enabling safe relocation into the leaf crate.

---

## 2. Refactoring Actions & Diff Summary
The 36 structural schemas representing DAG topology were extracted and relocated.

1. **Relocated Definitions**: Created [dag.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl_types/src/dag.rs) containing structural models:
   * `Dag`, `OverallLifecycle`, `Phase`, `ProgressionVerbs`, `Derivation`, `DerivationCondition`, `StructuredDerivationCondition`, `StateSelector`, `Slot`, `SlotStateMachine`, `StateMachine`, `PredicateBinding`, `PredicateRequiredUniverse`, `PredicateBindingSourceKind`, `ExpectedLifetime`, `StateDef`, `EntryVia`, `TransitionDef`, `CrossSlotConstraint`, `Severity`, `CrossWorkspaceConstraint`, `DerivedCrossWorkspaceState`, `ExposureConfig`, `Visibility`, `ParentSlot`, `ParentJoin`, `StateDependency`, `CascadeRule`, `DualLifecycle`, `PeriodicReviewCadence`, `RiskTierOverride`, `ReviewScope`, `EvidenceType`, `CategoryGated`, `ProductModuleGates`, `ConditionalGate`, `PruneCascadeRule`, `PruneCascadeTarget`, `PrunePreValidation`, `LoadedDag`.
   * Exported the module as `pub mod dag` in [lib.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl_types/src/lib.rs).
2. **Re-exported Compat Facade**: Modified [dag.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag.rs) in `dsl-core` to clean out type definitions and replace them with re-exports pointing to `dsl_types::dag`. Loader logic (`load_dags_from_dir`, etc.) was kept intact in `dsl-core`.
3. **Repointed References**: Adjusted import statements in referencing modules:
   * [green_when_coverage.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs)
   * [predicate_ast.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs)

### Git Diff
```diff
diff --git a/crates/dsl-core/src/config/dag.rs b/crates/dsl-core/src/config/dag.rs
index b3362bd..7fb3c96 100644
--- a/crates/dsl-core/src/config/dag.rs
+++ b/crates/dsl-core/src/config/dag.rs
@@ -8,6 +8,8 @@
 //! parent_slot/state_dependency, expected_lifetime, dual_lifecycle,
 //! periodic_review_cadence, evidence_types, category_gated).
 
+#![allow(unreachable_pub)]
+
 use anyhow::{Context, Result};
 use std::collections::BTreeMap;
 use std::fs;
@@ -16,730 +18,17 @@ use std::path::{Path, PathBuf};
 // =============================================================================
 // TOP-LEVEL DAG
 // =============================================================================
-
-#[derive(Debug, Clone, Deserialize, Serialize, Default)]
-#[serde(deny_unknown_fields)]
-pub struct Dag {
-... [relocated type definitions deleted here] ...
-
-#[derive(Debug, Clone)]
-pub struct LoadedDag {
-    pub source_path: PathBuf,
-    pub dag: Dag,
-}
+
+// Re-export all relocated types from dsl_types::dag module
+pub use dsl_types::dag::{
+    Dag, OverallLifecycle, Phase, ProgressionVerbs, Derivation, DerivationCondition,
+    StructuredDerivationCondition, StateSelector, Slot, SlotStateMachine, StateMachine,
+    PredicateBinding, PredicateRequiredUniverse, PredicateBindingSourceKind, ExpectedLifetime,
+    StateDef, EntryVia, TransitionDef, CrossSlotConstraint, Severity,
+    CrossWorkspaceConstraint, DerivedCrossWorkspaceState, ExposureConfig, Visibility,
+    ParentSlot, ParentJoin, StateDependency, CascadeRule, DualLifecycle,
+    PeriodicReviewCadence, RiskTierOverride, ReviewScope, EvidenceType, CategoryGated,
+    ProductModuleGates, ConditionalGate, PruneCascadeRule, PruneCascadeTarget,
+    PrunePreValidation, LoadedDag,
+};
+
+// Re-export level-0 types previously imported/used in this module
+pub use dsl_types::{
+    AuditClass, ClosureType, CompletenessAssertionConfig, EligibilityConstraint, RoleGuard,
+};
 
 /// Load every `*.yaml` file in the DAG taxonomies directory.
 
diff --git a/crates/dsl-core/src/config/green_when_coverage.rs b/crates/dsl-core/src/config/green_when_coverage.rs
index f90c8b1..2dc2f88 100644
--- a/crates/dsl-core/src/config/green_when_coverage.rs
+++ b/crates/dsl-core/src/config/green_when_coverage.rs
@@ -4,7 +4,7 @@
 //! predicates are backfilled workspace by workspace. This module deliberately
 //! reports coverage; it does not invent predicates.
 
-use crate::config::dag::{Dag, SlotStateMachine, StateDef, TransitionDef};
+use dsl_types::dag::{Dag, SlotStateMachine, StateDef, TransitionDef};
 use serde_yaml::Value as YamlValue;
 use std::collections::{BTreeMap, BTreeSet, HashSet};
 
diff --git a/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs b/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs
index 766ce0e..5b99390 100644
--- a/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs
+++ b/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs
@@ -1,4 +1,5 @@
-use crate::config::dag::{load_dags_from_dir, SlotStateMachine};
+use crate::config::dag::load_dags_from_dir;
+use dsl_types::dag::SlotStateMachine;
 use crate::config::predicate::{
     parse_green_when, CmpOp, EntityQualifier, EntityRef, EntitySetRef, Predicate, RelationScope,
     Validity,
diff --git a/crates/dsl_types/src/lib.rs b/crates/dsl_types/src/lib.rs
index 8670cc7..45d311a 100644
--- a/crates/dsl_types/src/lib.rs
+++ b/crates/dsl_types/src/lib.rs
@@ -22,6 +22,7 @@
 
 pub(crate) mod constellation_map_def;
 pub(crate) mod resolver_facts;
+pub mod dag;
 
 pub use constellation_map_def::{
     AuditClass, Cardinality, ClosureType, CompletenessAssertionConfig, ConstellationMapDefBody,
```

---

## 3. Pinned Test Set Verification Results
The test suite was run against the pinned `ob-poc` commit SHA (`68e9be40361a36a2f3925e83960bc07238f210b6`). 

Below is the per-test pass/fail set comparison showing a 100% match of all passing and failing tests.

| Test ID | Baseline Result | Post-Relocation Result | Status |
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
In strict adherence to Phase 2 — Step 1 scope limitations:
1. **No Logic/Behavioral Changes**: Did not modify how any structure is parsed, loaded, or validated.
2. **No Canonical Structures Built**: Did not introduce the proposed canonical struct definitions or projections.
3. **No Projections or State Wiring**: Left the state evaluation and projection generation paths completely untouched.
4. **No Test Logic Modification**: Adjusted only the imports inside reference-site files (`green_when_coverage.rs` and `predicate_ast.rs`) without modifying any of the test assertions or bodies.
5. **No Step 2/Step 3 Work**: Stopped immediately upon verifying Step 1.

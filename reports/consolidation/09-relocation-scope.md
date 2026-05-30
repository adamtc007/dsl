# Phase 2 — Step 4 / Step 1 Prep: Topology-Type Relocation Scope Report

This report scopes the relocation of DAG topology types from `dsl-core` into the leaf crate `dsl_types` to ensure clean boundaries and prevent circular dependency loops.

---

## R1: Enumeration of Topology Types

All family types are defined in [dag.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag.rs):

* **`Dag`** (struct) — Line 31
* **`OverallLifecycle`** (struct) — Line 82
* **`Phase`** (struct) — Line 95
* **`ProgressionVerbs`** (enum) — Line 112
* **`Derivation`** (struct) — Line 127
* **`DerivationCondition`** (enum) — Line 138
* **`StructuredDerivationCondition`** (struct) — Line 145
* **`StateSelector`** (enum) — Line 157
* **`Slot`** (struct) — Line 171
* **`SlotStateMachine`** (enum) — Line 273
* **`StateMachine`** (struct) — Line 279
* **`PredicateBinding`** (struct) — Line 328
* **`PredicateRequiredUniverse`** (struct) — Line 379
* **`PredicateBindingSourceKind`** (enum) — Line 409
* **`ExpectedLifetime`** (enum) — Line 425
* **`StateDef`** (struct) — Line 432
* **`EntryVia`** (enum) — Line 453
* **`TransitionDef`** (struct) — Line 462
* **`CrossSlotConstraint`** (struct) — Line 478
* **`Severity`** (enum) — Line 495
* **`CrossWorkspaceConstraint`** (struct) — Line 507
* **`DerivedCrossWorkspaceState`** (struct) — Line 540
* **`ExposureConfig`** (struct) — Line 556
* **`Visibility`** (enum) — Line 573
* **`ParentSlot`** (struct) — Line 583
* **`ParentJoin`** (struct) — Line 592
* **`StateDependency`** (struct) — Line 602
* **`CascadeRule`** (struct) — Line 610
* **`DualLifecycle`** (struct) — Line 624
* **`PeriodicReviewCadence`** (struct) — Line 645
* **`RiskTierOverride`** (struct) — Line 656
* **`ReviewScope`** (enum) — Line 663
* **`EvidenceType`** (struct) — Line 671
* **`CategoryGated`** (struct) — Line 681
* **`ProductModuleGates`** (struct) — Line 697
* **`ConditionalGate`** (struct) — Line 705
* **`PruneCascadeRule`** (struct) — Line 718
* **`PruneCascadeTarget`** (struct) — Line 727
* **`PrunePreValidation`** (struct) — Line 734
* **`LoadedDag`** (struct) — Line 746

---

## R2: Clean vs Entangled Classification

### 1. CLEAN Types
* **Classification**: **All structural models (lines 31 to 746)**.
* **Details**: They contain only std collections, standard primitives, path buffers, and serde derives. None of them reference external packages or dsl-core compiler logic. They can be moved directly to `dsl_types` with no new dependencies.

### 2. ENTANGLED Logic
* **Classification**: **Loader functions** (`load_dags_from_dir` at Line 755, `load_domain_pack_owned_dags` at Line 785, and siblings `yaml_files` / `find_dag_yaml_by_id`).
* **Details**: These are functions, not types. They perform file I/O operations (`std::fs`, `std::path::Path`) and return context-wrapped `anyhow::Result` errors.
* **Separation Plan**: These functions must remain in `dsl-core` (e.g. in `dsl_core::config::dag::loader`), importing `Dag` from the leaf crate `dsl_types`. This prevents adding file I/O or `anyhow` dependencies to the leaf crate.

---

## R3: References inside `dsl-core`

The following files inside `dsl-core` import and reference these types (and will need their imports adjusted to the new re-export):

* **[lib.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/lib.rs)**: Re-exports these types at the facade level.
* **[dag_validator.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs)**: Performs extensive schema checks on `Dag` constraints and lifecycles.
* **[green_when_coverage.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/green_when_coverage.rs)**: Computes coverage matrices.
* **[resolver/mod.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/resolver/mod.rs)**: Integrates predicate bindings.
* **[resolver/manifest.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/resolver/manifest.rs)**: Resolves closure categories.
* **[config/predicate/integration_tests/predicate_ast.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs)**: Test module imports.

---

## R4: Leaf Crate Dependencies Compatibility

We reviewed the dependencies of [dsl_types/Cargo.toml](file:///Users/adamtc007/Dev/dsl/crates/dsl_types/Cargo.toml):
* Standard dependencies (`serde`, `serde_json`, `serde_yaml`) are already present.
* There are no custom packages, and no dependencies on any other workspace crates.
* It is fully compatible with hosting the pure serialization structures.

---

## "WHAT I DID NOT DO" Ledger

In strict compliance with Phase 2 — Step 4 / Step 1 Prep read-only constraints:
1. **No Source Edits**: Did not modify any code or configuration file.
2. **No Dependency Changes**: Did not alter any Cargo configuration files.
3. **No Struct Relocation**: Left all structs defined inside `dsl-core::config::dag` as-is.
4. **No Git State Mutation (Except Report Commit)**: Staged and committed only this report file.
5. **No System Alterations**: Ran no service deployments or custom validations.

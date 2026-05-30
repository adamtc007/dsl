# Phase 2 — Step 3a: Un-Quarantine Scope Report

This report documents the scoping analysis of the `ob-poc` quarantine state, mapping the runtime requirements for `DagRegistry` and private module visibility.

---

## 1. `DagRegistry` Scope Analysis

`dsl-runtime` references the deleted `DagRegistry` structure across multiple files.

### A. References in `dsl-runtime`
1. **[src/cross_workspace/gate_checker.rs](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs)**:
   * Line 41: `use dsl_core::config::DagRegistry;`
   * Line 137, 144: Holds `registry: Arc<DagRegistry>` inside the `GateChecker` struct to query constraints during transition gate checks.
   * Line 306, 311: In tests, constructs registry via `DagRegistry::from_loaded(map)`.
2. **[src/cross_workspace/postgres_child_resolver.rs](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs)**:
   * Line 30: `use dsl_core::config::DagRegistry;`
   * Line 44, 48: Holds `registry: Arc<DagRegistry>` inside the `PostgresChildEntityResolver` struct to lookup parent/child mappings.
   * Line 67: Invokes `self.registry.parent_slot_for(child_workspace, child_slot)`.
   * Line 173, 178, 195: In tests, constructs mock registry via `DagRegistry::from_loaded(map)`.
3. **[src/cross_workspace/hierarchy_cascade.rs](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs)**:
   * Line 32: `use dsl_core::config::DagRegistry;`
   * Line 116, 121, 123: Holds `registry: Arc<DagRegistry>` inside `CascadePlanner`.
   * Line 147–149: Invokes `self.registry.children_of(parent_workspace, parent_slot)`.
   * Line 152–153: Invokes `self.registry.state_dependency_for(&child_key.workspace, &child_key.slot)`.
   * Line 243, 248, 281, 328: In tests, constructs mock registry via `DagRegistry::from_loaded(map)`.
4. **[src/cross_workspace/derived_state_projector.rs](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs)**:
   * Line 19: `use dsl_core::config::DagRegistry;`
   * Line 44, 49, 51: Holds `registry: Arc<DagRegistry>` inside `DerivedStateProjector`.
   * Line 65–66: Invokes `self.registry.derived_states_for_slot(host_workspace, host_slot)`.
   * Line 122, 127, 157, 185: In tests, constructs mock registry via `DagRegistry::from_loaded(map)`.
5. **[src/cross_workspace/test_harness/runner.rs](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs)**:
   * Line 4: `use dsl_core::config::DagRegistry;`
   * Line 30, 254, 290: Holds and propagates `Arc<DagRegistry>`.
   * Line 115: Loads real registry from file directory via `DagRegistry::from_dir(&dag_path)`.
6. **[src/cross_workspace/test_harness/live.rs](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/test_harness/live.rs)**:
   * Line 14: `use dsl_core::config::DagRegistry;`
   * Line 43, 347, 382, 383: Holds and propagates `Arc<DagRegistry>`.
   * Line 79: Loads real registry via `DagRegistry::from_dir(&dag_path)`.

### B. Summary of the Deleted `DagRegistry` Caching Structure
Based on the source retrieved from commit `06232bf1de0e40fd8f8a925266e25a012758d01a~1`:
* **What it cached**: It cached the parsed, multi-workspace constellation of `Dag` layouts (including transitions, cross-workspace constraints, parent-child slot relationships, and derived states).
* **Keying Mechanism**:
  * `TransitionKey` (workspace, slot, from_state, to_state) -> maps to list of `CrossWorkspaceConstraint` entries.
  * `SlotKey` (workspace, slot) -> maps to list of `DerivedCrossWorkspaceState` entries.
  * `SlotKey` (child slot) -> maps to `parent_slot` metadata.
  * `SlotKey` (parent slot) -> maps to list of child `SlotKey` entries (for reverse propagation lookup).
* **Evaluation**: `dsl-runtime` needs the **caching/indexing mechanism** itself rather than a static caching result. Because the constellation schema is static and parsed from YAML configuration trees at server boot, `DagRegistry` pre-computes indexing structures so lookups inside hot-path execution blocks can execute in O(1) time without filesystem parser overhead.

---

## 2. Private Submodule Symbol Trace

`dsl-runtime` imports several symbols from `dsl_core::config::*` and `dsl_core::executable_plan::*`:

| Symbol Name | File:Line | Submodule Path | Symbol Kind |
| :--- | :--- | :--- | :--- |
| `CrossWorkspaceConstraint` | `gate_checker.rs:40` | `config::dag` | Struct |
| `StateSelector` | `gate_checker.rs:40`, `derived_state.rs:24` | `config::dag` | Enum |
| `Severity` | `gate_checker.rs:276, 277`, `hierarchy_cascade.rs:207, 208` | `config::dag` | Enum |
| `Dag` | `gate_checker.rs:293`, `postgres_child_resolver.rs:161`, `hierarchy_cascade.rs:230`, `derived_state_projector.rs:109` | `config::dag` | Struct |
| `LoadedDag` | `gate_checker.rs:293`, `postgres_child_resolver.rs:161`, `hierarchy_cascade.rs:230`, `derived_state_projector.rs:109` | `config::dag` | Struct |
| `DerivationCondition` | `derived_state.rs:24` | `config::dag` | Enum |
| `DerivedCrossWorkspaceState` | `derived_state.rs:24` | `config::dag` | Struct |
| `CascadeRule` | `hierarchy_cascade.rs:31` | `config::dag` | Struct |
| `EffectClass` | `coordination.rs:21` | `executable_plan` | Enum |

---

## 3. Facade Cross-Check & Classification

We cross-checked each symbol traced in Section 2 against the root facade of the `dsl-core` crate ([dsl-core/src/lib.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/lib.rs)):

* **`CrossWorkspaceConstraint`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 63 of `lib.rs`.
* **`StateSelector`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 64 of `lib.rs`.
* **`Severity` (config::dag::Severity)** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 65 of `lib.rs` under the alias **`DagSeverity`** (to prevent conflicts with parser/compiler warnings diagnostics severity).
* **`Dag`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 44 of `lib.rs`.
* **`LoadedDag`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 47 of `lib.rs`.
* **`DerivationCondition`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 63 of `lib.rs`.
* **`DerivedCrossWorkspaceState`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 63 of `lib.rs`.
* **`CascadeRule`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 63 of `lib.rs`.
* **`EffectClass`** -> **AVAILABLE-ON-FACADE**
  * Re-exported at line 86 of `lib.rs`.

**Conclusion**: **All** private submodule symbols consumed by `dsl-runtime` are already fully public and re-exported at the root of `dsl-core`. To resolve the `E0603` visibility compilation failures, no submodules need to be re-opened; `dsl-runtime` simply needs to adjust its imports to reference the root facade (and map the `Severity` -> `DagSeverity` alias).

---

## 4. Quarantined Crates Cascade Analysis

We scanned the other 6 quarantined crates in the `ob-poc` workspace to check if they import from private `dsl-core` submodules:

* **Direct Private Submodule Consumers**:
  1. **`sem_os_postgres`**:
     * `src/ops/discovery.rs:40, 41, 281`: Imports `ConfigLoader` from `dsl_core::config::loader::ConfigLoader` and configuration structs from `dsl_core::config::types::*`.
     * **Classification**: **AVAILABLE-ON-FACADE**. `ConfigLoader` and all referenced types (`ArgConfig`, `DomainConfig`, `VerbConfig`, `VerbMetadata`, `VerbsConfig`) are re-exported at the root of `dsl-core`.
  2. **`dsl-lsp`**:
     * `tests/parser_conformance.rs:11, 12`, `src/analysis/v2_adapter.rs:31, 32, 308`, `src/handlers/diagnostics.rs:23, 194, 197`, `src/handlers/completion.rs:13, 14`, `src/handlers/code_actions.rs:13, 14, 185`: Imports AST, parser, and configuration modules via private subpaths (e.g. `dsl_core::ast::*`, `dsl_core::parser::*`, `dsl_core::diagnostics::*`, `dsl_core::config::*`).
     * **Classification**: **AVAILABLE-ON-FACADE**. Every single imported symbol is re-exported at the root of `dsl-core`.
  3. **`ob-poc-agent`**:
     * `src/repl_channel.rs:292, 353`: References `dsl_core::parser::parse_program`.
     * **Classification**: **AVAILABLE-ON-FACADE**. Re-exported at the root of `dsl-core` as `dsl_core::parse_program`.
* **Pure Cascade Failures (No Direct Internal Imports)**:
  1. **`ob-poc-web`**: 0 references to private submodules. Fails to compile solely because it depends on `dsl-runtime` (which fails build checks).
  2. **`sem_os_harness`**: 0 references to private submodules. Fails to compile due to the build failure of its dependencies (`dsl-runtime` and `sem_os_server`).
  3. **`sem_os_server`**: 0 references to private submodules. Fails to compile due to compilation failures of upstream crates.

---

## "WHAT I DID NOT DO" Ledger

In strict compliance with the Phase 2 — Step 3a read-only rules:
1. **No edits to code**: Did not touch any source code files in either the `dsl` or `ob-poc` repositories.
2. **No visibility changes**: Kept `config` and `executable_plan` private (`pub(crate)`) inside `dsl-core`.
3. **No DagRegistry restoration**: Did not add, restore, or modify any files to recreate `DagRegistry`.
4. **No other Phase 2 edits**: Conducted static traces only.
5. **No git state mutations**: Performed no branch movements or checkouts.

---
Report compiled by Antigravity on 2026-05-30.

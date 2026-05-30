# Research Phase — Leg 2 (Fact Gathering)

This report details the gathered facts across ten areas (**T1–T10**) regarding the workspace dependencies, compile-time cycles, graph structures, public APIs, module clusters, and the quarantine state of the `ob-poc` workspace.

All claims are backed by command executions and linked directly to their saved raw output artifacts.

---

## T1: Dependency Wiring

* **Primary Artifacts**:
  * [dsl-cargo-tree.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-cargo-tree.txt)
  * [sem-os-cargo-tree.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-cargo-tree.txt)
  * [ob-poc-cargo-tree.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/ob-poc-cargo-tree.txt)
  * [ob-poc-config-sha.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/ob-poc-config-sha.txt) (Locked configuration at commit `68e9be40361a36a2f3925e83960bc07238f210b6`)

### Workspace Structures and Dependencies
1. **`dsl` Workspace**:
   * Contains [dsl_types](file:///Users/adamtc007/Dev/dsl/crates/dsl_types) and [dsl-core](file:///Users/adamtc007/Dev/dsl/crates/dsl-core).
   * Declared in [dsl/Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml).
2. **`sem-os` Workspace**:
   * Contains `sem_os_types`, `sem_os_core`, `sem_os_ontology`, `sem_os_policy`, and `sem_os_taxonomy`.
   * Declared in [sem-os/Cargo.toml](file:///Users/adamtc007/dev/sem-os/Cargo.toml).
   * Declares cross-repo `git` dependencies on `dsl_types` and `dsl-core` (pinned at tag `v0.1.2`).
   * Overridden for local development in [sem-os/.cargo/config.toml](file:///Users/adamtc007/dev/sem-os/.cargo/config.toml) via `[patch."https://github.com/adamtc007/dsl"]` pointing to `../dsl/crates/dsl_types` and `../dsl/crates/dsl-core`.
3. **`ob-poc` Workspace**:
   * Declared in [ob-poc/rust/Cargo.toml](file:///Users/adamtc007/Developer/ob-poc/rust/Cargo.toml).
   * Declares cross-repo `git` dependencies on `dsl` (tag `v0.1.4`) and `sem-os` (tag `v0.1.5`).
   * Overridden globally for local development in `~/.cargo/config.toml` to point to `/Users/adamtc007/dev/dsl/crates/*` and `/Users/adamtc007/dev/sem-os/crates/*`.

### Version Skew & Patch Anomalies (Measured)
* **Taxonomy Patch Omision**: In the global patch section of `~/.cargo/config.toml`, only `sem_os_types`, `sem_os_core`, `sem_os_ontology`, and `sem_os_policy` are patched. The `sem_os_taxonomy` crate is **omitted**.
* **Impact**: Consequently, `ob-poc` resolves `sem_os_taxonomy` from the remote git tag `v0.1.5` instead of the local workspace directory, introducing hidden desynchronization and build mismatches if local taxonomy changes are made.
* **Lock Versions**: All local crates are locked at version `0.1.0` inside their respective `Cargo.lock` files via path patches.

---

## T2: Cycle, Exact Shape

The Circular dependency between `dsl-core` and `sem_os_core` is verified with the following exact shape:

### 1. `sem_os_core` normal dependency -> `dsl-core` (Normal Edge)
* **Location**: [sem-os/Cargo.toml](file:///Users/adamtc007/dev/sem-os/Cargo.toml) workspace dependencies.
* **Exact Symbols/Types Imported**:
  * [hydrator.rs](file:///Users/adamtc007/dev/sem-os/crates/sem_os_core/src/frontier/hydrator.rs#L1-L7) imports 23 symbols: `parse_green_when`, `parse_single_verb`, `AttrValue`, `ClosureType`, `CmpOp`, `CompletenessAssertionStatus`, `DiscretionaryReason`, `EntityRef`, `EntitySetRef`, `FrontierFact`, `FrontierFacts`, `GreenWhenStatus`, `HydrateFrontierError`, `InstanceFrontier`, `InvalidFact`, `InvalidFactDetail`, `MissingFact`, `Predicate`, `PredicateEntityRef`, `ReachableDestination`, `ResolvedSlot`, `ResolvedTemplate`, `Validity`.
  * [composer.rs](file:///Users/adamtc007/dev/sem-os/crates/sem_os_core/src/resolver/composer.rs#L1-L6) imports 15 symbols: `compute_version_hash`, `load_domain_pack_owned_dags`, `Dag`, `DagSlot`, `LoadedDag`, `PredicateBinding`, `ResolvedSlot`, `ResolvedSource`, `ResolvedTemplate`, `ResolvedTransition`, `ResolverProvenance`, `ShapeRef`, `SlotProvenance`, `SlotStateMachine`, `WorkspaceId`.
  * [shape_rule.rs](file:///Users/adamtc007/dev/sem-os/crates/sem_os_core/src/resolver/shape_rule.rs#L1) imports: `ClosureType`, `EligibilityConstraint`, `PredicateBinding`, `RoleGuard`.

### 2. `dsl-core` dev-dependency -> `sem_os_core` (Test Edge)
* **Location**: [dsl-core/Cargo.toml](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/Cargo.toml#L35)
* **Exact Symbols/Types Imported**:
  * [tests/resolver_lux_sicav.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/resolver_lux_sicav.rs#L4), [tests/phase2_acceptance.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/phase2_acceptance.rs#L2), [tests/eligibility_lint.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/eligibility_lint.rs#L5), [tests/closure_lint.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/closure_lint.rs#L4), and [tests/resolver_manifest.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/resolver_manifest.rs#L2) import `sem_os_core::resolver::{resolve_template, ResolverInputs}`.
  * [tests/frontier_recursive.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/frontier_recursive.rs#L8), [tests/cbu_evidence_substates.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/cbu_evidence_substates.rs#L8), [tests/frontier_skeleton.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/frontier_skeleton.rs#L9), and [tests/cbu_validity.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/cbu_validity.rs#L8) import `sem_os_core::hydrate_frontier`.
  * [tests/shape_rule_composition.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/shape_rule_composition.rs#L4-L6) imports `sem_os_core::resolver::{resolve_template, InsertBetween, ResolveError, ResolverInputs, SlotGateMetadataRefinement}`.

---

## T3: Substrate / Graph Duplication

The definitions of the DAG and graph structures across the workspaces are:

### Definitions (Measured)
* **`Dag`**: Defined at [dag.rs:L31](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag.rs#L31). Defines structural workspaces, slots, parent slot references, and cross-workspace constraints.
* **`RailwayGraph`**: Defined at [railway.rs:L371](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-bpmn-frontend/src/railway.rs#L371). Represents compiled BPMN control-flow, parallel joins, sequence flow edges, and boundary attachments.
* **`StateGraphDefBody`**: Defined at [state_graph_def.rs:L7](file:///Users/adamtc007/dev/sem-os/crates/sem_os_ontology/src/state_graph_def.rs#L7). Represents milestones, advance/revert edges, and state gates in the registry ontology.
* **`RawConstellationMap` / `RawConstellationSlot`**: Defined at [dag_validator.rs:L1670](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L1670) and [dag_validator.rs:L1678](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L1678). Used for validating map coordination without compile-time database references.

### Reused vs Distinct Types (Measured/Inferred)
* Conceptual processes such as resource constraints (`Dag`), lifecycle progression (`StateGraphDefBody`), and sequence execution flow (`RailwayGraph`) are modeled by **distinct** types with different structural requirements.
* Both `sem_os_core` and `dsl-runtime` **reuse** the exact same `Dag` data definition imported directly from `dsl-core` (no duplication).
* However, [dag_validator.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L632) parses a lightweight **duplicate** `RawConstellationMap` from YAML instead of importing `sem_os_ontology::constellation_map_def` to keep `dsl-core` free of database-dependent libraries.

### Candidate Shared-Substrate Types (Inferred)
* Moving the parser AST / YAML definition types (`Dag`, `Slot`, `SlotType`, etc.) or the resolver pipeline (`resolve_template` / `hydrate_frontier`) to a common leaf crate would isolate dependencies. Moving the integration tests (T2) to a separate `dsl-integration-tests` crate would break the dev-dep cycle.

---

## T4: Public-API Baseline

* **Full Listings Saved**:
  * [dsl_types-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl_types-public-api.txt) (648 lines)
  * [dsl-core-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-core-public-api.txt) (7,168 lines)
  * [sem_os_types-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem_os_types-public-api.txt) (2,058 lines)
  * [sem_os_core-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem_os_core-public-api.txt) (4,132 lines)
  * [sem_os_ontology-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem_os_ontology-public-api.txt) (3,929 lines)
  * [sem_os_policy-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem_os_policy-public-api.txt) (15,689 lines)
  * [sem_os_taxonomy-public-api.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem_os_taxonomy-public-api.txt) (212 lines)

The `cargo public-api` tool compiled and executed correctly for all 7 library crates.

---

## T5: Tunnel Map

* **Source Artifact**: [parsed-hits.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/parsed-hits.txt) (Categorized listing of 257 occurrences)

### Summary of Matches (Measured)
* **Non-Quarantined `sem-os` Workspace**: Reaches internal module paths (e.g. `sem_os_policy::affinity::*` in `sem_os_core/tests/discovery_pipeline.rs:6`).
* **Non-Quarantined `ob-poc` Workspace**: Reaches internal paths and globs (e.g. `sem_os_core::principal::Principal` in `ob-poc-web/src/bus_runtime.rs:39`, `sem_os_ontology::view_def::*` in `sem_os_obpoc_adapter/src/seeds/view_seeds.rs:6`).
* **Quarantined `ob-poc` Workspace (Failed Compilation)**:
  * `dsl-runtime` references `dsl_core::config::DagRegistry` in multiple files (e.g. [gate_checker.rs:L41](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs#L41)).

---

## T6: State-Model Consumer Trace

A **non-compiler live runtime consumer** of `sem_os_policy` and `sem_os_core` lifecycle states exists.

### Trace Evidence (Measured)
1. **Instantiation**: [ob-poc-web/src/main.rs:L769-788](file:///Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-web/src/main.rs#L769-L788) instantiates `sem_os_policy::service::CoreServiceImpl` wrapping local postgres stores.
2. **Client Wiring**: [ob-poc-web/src/main.rs:L794-811](file:///Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-web/src/main.rs#L794-L811) instantiates `sem_os_client::inprocess::InProcessClient` wrapping the service as a `SemOsClient`.
3. **Orchestrator Setup**: [ob-poc-web/src/main.rs:L1737-1738](file:///Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-web/src/main.rs#L1737-L1738) registers the client on the runtime intent orchestrator.
4. **Service Binding**: [ob-poc-web/src/main.rs:L1030-1037](file:///Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-web/src/main.rs#L1030-L1037) registers `ob_poc::services::ObPocSemOsContextResolver` under `dyn SemOsContextResolver`.
5. **Runtime Call**: During verb execution, the database adapter resolves this service at [affinity.rs:L517-522](file:///Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_postgres/src/ops/affinity.rs#L517-L522) and calls `resolve_context` to perform live security label checks and ABAC policy enforcement against the principal.
6. **UI Consumption**: [MissionControl.tsx](file:///Users/adamtc007/Developer/ob-poc/ob-poc-ui-react/src/features/observatory/components/MissionControl.tsx#L26-L28) calls the server endpoint to query active metrics (including `pending_changesets`).

---

## T7: Module Clusters

Module trees extracted from `lib.rs` files reveal the following clusters:

* **Observation/Rendering Plane**: `sem_os_policy::observatory`, `sem_os_policy::diagram`, `sem_os_taxonomy::builder`.
* **Stewardship/Governance Plane**: `sem_os_policy::enforce`, `sem_os_policy::abac`, `sem_os_policy::context_policy`, `sem_os_policy::grounding`, `sem_os_policy::service`.
* **Validation/Compiler Plane**: `dsl-core::compiler`, `dsl-core::parser`, `dsl-core::diagnostics`, `dsl-core::executable_plan`, `dsl-core::execution_dag`.
* **Schema/Registry Defs**: `sem_os_ontology::attribute_def`, `sem_os_ontology::entity_type_def`, `sem_os_ontology::policy_rule`, `sem_os_ontology::verb_contract`.

---

## T8: Snapshot Inventory

* **Count (Measured)**: There are **110 total snapshot tests** (using the `insta` framework), all located in the `dsl` workspace:
  * 20 in `crates/dsl-core/src/executable_plan/integration_tests/snapshots/`
  * 90 in `crates/dsl-core/tests/snapshots/`

### Encoding of Crate/Module Paths (Measured)
* **Filenames**: The unit test snapshots are named `dsl_core__executable_plan__integration_tests__plan_golden__<name>.snap`, encoding both the crate name `dsl_core` and the internal module path.
* **Content Headers**: All 110 files contain metadata headers pointing to the relative path within `dsl-core`:
  * *Example* (`tests/snapshots/ast_golden__ast_attribute_bridge_to_semos.snap`):
    ```yaml
    source: crates/dsl-core/tests/ast_golden.rs
    assertion_line: 324
    expression: program
    ```
* **Impact**: Renaming the `dsl-core` crate or reorganizing its submodules will trigger regeneration churn across all 110 files.

---

## T9: Build & Quarantine

### Current Status (Measured)
* **`dsl` workspace**: Builds successfully (`Finished dev profile` with 94 compiler warnings).
* **`sem-os` workspace**: Builds successfully (`Finished dev profile` with warnings).
* **`ob-poc` workspace**: **Fails compilation** (25 errors inside `dsl-runtime`).
  * [ob-poc-check-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/ob-poc-check-raw.txt)

### Confirm the DagRegistry Blocker (Measured)
* **Removed Item**: `dsl_core::config::DagRegistry`, a 997-line caching structure defined in `crates/dsl-core/src/config/dag_registry.rs` (deleted in commit `06232bf1de0e40fd8f8a925266e25a012758d01a`), is missing.
* **Private Modules**: In `dsl-core`'s updated `lib.rs`, `config` and `executable_plan` are declared as `pub(crate) mod`, hiding them from outside crates.
* **Errors**: `dsl-runtime` triggers `E0432` (unresolved import `dsl_core::config::DagRegistry`) and `E0603` (module `config` / `executable_plan` is private).

---

## T10: Merge-Blocker Scan

* **Circular Dependency**: `dsl-core` (dev-dep tests) <-> `sem_os_core` (normal dependency) prevents independent publishing.
* **Visibility Changes**: `pub(crate)` module attributes in `dsl-core` hide types required by `dsl-runtime`.
* **Patch Skew**: The lack of a `sem_os_taxonomy` patch in `~/.cargo/config.toml` makes `ob-poc` resolve the remote git tag instead of local changes.
* **Profile Clash**: Unified workspace profile definitions (`opt-level = 2`, custom `candle` overrides) must be coordinated when merging.
* **Version Mismatch**: `sem-os/Cargo.toml` targets `dsl` tag `v0.1.2`, whereas `ob-poc` targets `v0.1.4`, which can create resolver issues if unpatched.

---

## Facts Most Likely to Shape the Plan
1. `sem_os_taxonomy` is omitted from the global patch list in `~/.cargo/config.toml`, causing it to draw from remote git instead of the local workspace.
2. Circular dev-dependencies exist between `dsl-core` and `sem_os_core`, where integration tests in `dsl-core` import `resolve_template` and `hydrate_frontier`.
3. Making `config` and `executable_plan` private (`pub(crate)`) in `dsl-core`, alongside removing `DagRegistry`, is the direct and sole compiler blocker preventing `ob-poc` from building.
4. Active runtime policy enforcement, ABAC checks, and layout visualizations continuously consume `sem_os_policy` and `sem_os_core` lifecycle states.
5. Renaming or moving the `dsl-core` crate will require updating filenames and header sources for 110 `insta` golden snapshot files.

---

## WHAT I DID NOT DO (Ledger)
* **No Code Edits**: Did not modify any Rust source, Cargo configuration, or workspace settings.
* **No Consolidation/Merging**: Did not move or merge any crates, components, or modules.
* **No File Renaming/Moving**: Did not modify the file layout or relocate any files.
* **No Fixes**: Did not restore `DagRegistry` or adjust module visibilities to resolve compile issues.
* **No Workspace Mutations**: Did not touch branches, stashes, or mutate git history except committing this report and its artifacts.
* **No Recommendations**: Strictly avoided providing solutions, plans, or next steps.

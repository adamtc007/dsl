# Phase 2 — Step 3b: DAG-Family Map Analysis Report

This report documents the family of DAG, constellation, and state-pack representations across the consolidated workspace, tracing their structures, parse points, derivations, metadata separability, and reconciliation assertions.

---

## T1: Enumerate the Family

Below is the inventory of all structures representing constellations, DAGs, or state-packs in the consolidated workspace:

1. **`dsl_types::constellation_map_def::ConstellationMapDefBody`**
   * **Definition File/Line**: [constellation_map_def.rs:10](file:///Users/adamtc007/Dev/dsl/crates/dsl_types/src/constellation_map_def.rs#L10)
   * **Shape**: `TOPOLOGY+METADATA`. Contains slot layout constraints, recursive hierarchy (`children: BTreeMap<String, SlotDef>`), dependency vectors (`depends_on`), and metadata (`verbs: BTreeMap<String, VerbPaletteEntry>`, `role_guard`, `completeness_assertion`).
   * **Role**: Compiler parse target, schema registry definition, and serialization format for database storage.

2. **`dsl_core::config::dag::Dag`**
   * **Definition File/Line**: [dag.rs:31](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag.rs#L31)
   * **Shape**: `TOPOLOGY+METADATA`. Includes slots, lifecycle transitions, constraints (`cross_slot_constraints`, `cross_workspace_constraints`), and forward-compatible metadata (`extra`).
   * **Role**: Top-level YAML deserialization schema and validation representation in `dsl-core`.

3. **`RawConstellationMap` (in `dsl-core`'s `dag_validator`)**
   * **Definition File/Line**: [dag_validator.rs:1670](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L1670)
   * **Shape**: `TOPOLOGY+METADATA`. Minimal struct mapping slot identifiers to their configuration parameters parsed as raw `serde_yaml::Value`.
   * **Role**: Dependency-free validation of constellation map definitions against loaded DAGs inside `dsl-core`.

4. **`sem_os_ontology::constellation_map_def::ConstellationMapDefBody`**
   * **Definition File/Line**: [constellation_map_def.rs:11](file:///Users/adamtc007/Dev/dsl/crates/sem_os_ontology/src/constellation_map_def.rs#L11)
   * **Shape**: `TOPOLOGY+METADATA`.
   * **Role**: A compatibility re-export shim pointing to `dsl_types::constellation_map_def::ConstellationMapDefBody`.

5. **`sem_os_ontology::state_graph_def::StateGraphDefBody`**
   * **Definition File/Line**: [state_graph_def.rs:7](file:///Users/adamtc007/Dev/dsl/crates/sem_os_ontology/src/state_graph_def.rs#L7)
   * **Shape**: `TOPOLOGY+METADATA`. Maps `nodes: Vec<GraphNode>`, `edges: Vec<GraphEdge>`, `gates: Vec<GraphGate>`, names, and lane descriptions.
   * **Role**: Ontology definition of state graphs for Sage discovery and human documentation.

6. **`sem_os_policy::domain_pack::DomainPackManifest`**
   * **Definition File/Line**: [domain_pack.rs:18](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/domain_pack.rs#L18)
   * **Shape**: `TOPOLOGY+METADATA`. Contains transition policies (`allowed_transitions`), dry-run/mutation settings, personas, and classifications.
   * **Role**: Governance, access control policy representation, and transition gating.

7. **`DagRegistry` (Deleted)**
   * **Restored File/Line**: [deleted-dag-registry.txt:71](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/deleted-dag-registry.txt#L71) (originally `crates/dsl-core/src/config/dag_registry.rs`)
   * **Shape**: `TOPOLOGY+METADATA`.
   * **Role**: In-memory pre-indexed runtime lookup cache mapping parent/child slots and transitions for hot-path lookups.

---

## T2: Count the Parses

There are **8 independent YAML/JSON parsers** loading constellation, DAG, or state-pack configurations:

1. **`serde_yaml::from_str::<Dag>`**
   * **Location**: [dag.rs:767, 812](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag.rs#L767)
   * **Source**: `sem_os_seeds/dag_taxonomies/*.yaml`
2. **`serde_yaml::from_str::<RawConstellationMap>`**
   * **Location**: [dag_validator.rs:639](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L639)
   * **Source**: `sem_os_seeds/constellation_maps/*.yaml`
3. **`serde_yaml::from_str::<SeedConstellationMap>`**
   * **Location**: [composer.rs:164](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/src/resolver/composer.rs#L164)
   * **Source**: `sem_os_seeds/constellation_maps/*.yaml`
4. **`serde_yaml::from_value::<DomainPackManifest>`**
   * **Location**: [domain_pack.rs:171](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/domain_pack.rs#L171)
   * **Source**: `sem_os_seeds/domain_packs/*.yaml`
5. **`serde_yaml::from_str::<StateGraph>`**
   * **Location**: [stategraph/mod.rs:128](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-analysis/src/stategraph/mod.rs#L128) (in `ob-poc`)
   * **Source**: `state_graphs/*.yaml`
6. **`parse_yaml_file` -> generic `serde_yaml::Value`**
   * **Location**: [domain_pack.rs:1105](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/domain_pack.rs#L1105) (called multiple times to build hashes and reload directories)
   * **Source**: `dag_taxonomies/`, `constellation_maps/`, `state_machines/`
7. **`serde_json::from_value::<ConstellationMapDefBody>`**
   * **Location**: [service.rs:374](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/service.rs#L374)
   * **Source**: Postgres `SnapshotRow::definition` JSONB column.
8. **`serde_json::from_value::<StateMachineDefBody>`**
   * **Location**: [service.rs:366](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/service.rs#L366)
   * **Source**: Postgres `SnapshotRow::definition` JSONB column.

---

## T3: Parse-vs-Derived Map

The following map defines the derivation relationships between configurations:

```mermaid
graph TD
    DAG_YAML["dag_taxonomies/*.yaml"] -->|Parse 1| Dag["dsl_core::config::dag::Dag"]
    DAG_YAML -->|Generic Parse 6| Value1["serde_yaml::Value"]
    
    CMAP_YAML["constellation_maps/*.yaml"] -->|Parse 3| Seed["SeedConstellationMap"]
    CMAP_YAML -->|Parse 2| Raw["RawConstellationMap"]
    CMAP_YAML -->|Generic Parse 6| Value2["serde_yaml::Value"]
    
    DP_YAML["domain_packs/*.yaml"] -->|Parse 4| DomainPack["DomainPackManifest"]
    
    SG_YAML["state_graphs/*.yaml"] -->|Parse 5| SG["dsl_analysis::stategraph::StateGraph"]
    
    Seed -->|Transform| CMDefBody["ConstellationMapDefBody"]
    
    Dag -->|Derive (In-Memory Index)| DagRegistry["DagRegistry (Deleted)"]
    
    DB_JSONB["Postgres SnapshotRow::definition"] -->|Parse 7| CMDefBody2["ConstellationMapDefBody (Policy Service)"]
    DB_JSONB -->|Parse 8| SMDefBody["StateMachineDefBody (Policy Service)"]
```

---

## T4: Metadata Separability

In the rich structures:
* In `dsl_types::constellation_map_def::SlotDef`, the metadata (specifically `verbs: BTreeMap<String, VerbPaletteEntry>`, `description`, `placeholder`, `role_guard`, `completeness_assertion`, and `eligibility`) is **welded** directly into the `SlotDef` type. Because the tree is represented recursively (`children: BTreeMap<String, SlotDef>`), a lean topology cannot be cleanly extracted from `SlotDef` without carrying this metadata.
* In contrast, the lean topology representation `dsl_core::config::dag::Slot` does not natively declare a `verbs` field or a verb palette; it instead contains a generic `extra: BTreeMap<String, YamlValue>` bucket.
* Because the lean topology struct (`Slot`) and the metadata-rich struct (`SlotDef`) are defined as completely separate, independent Rust types, the rich metadata is structurally **welded** into its specific representations rather than being separate and projection-derived.

---

## T5: The Recon

We located the following reconciliation checks:

### 1. Verification of DSL Verbs Coverage
* **Location**: [domain_pack_dsl_reconciliation.rs:417](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/domain_pack_dsl_reconciliation.rs#L417)
* **Check Category**: **DRIFT Check**. It verifies that every verb allowed by the pack manifests maps to a valid transition in the independent DAG taxonomy parses, preventing mismatching definitions.
* **Verbatim Assertion Quote**:
  ```rust
  assert!(
      failures.is_empty(),
      "domain pack DSL/DAG reconciliation failures: {failures:#?}"
  );
  ```

### 2. Verification of Domain Pack Coverage
* **Location**: [domain_pack.rs:1652](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/domain_pack.rs#L1652)
* **Check Category**: **COVERAGE Check**. It scans the workspace configuration directories to ensure there are no orphaned packs or DAGs that are not owned by any domain pack manifest.
* **Verbatim Assertion Quotes**:
  ```rust
  assert!(
      missing_packs.is_empty(),
      "DSL journey packs not owned by any SemOS domain pack: {missing_packs:#?}"
  );
  assert!(
      missing_dags.is_empty(),
      "DAG taxonomies not owned by any SemOS domain pack: {missing_dags:#?}"
  );
  ```

---

## T6: Canonical-Source Candidate

* **Canonical Target**: A single unified `UnifiedConstellationMap` or `UnifiedDag` representation that encapsulates both structural topology (slots, children, dependencies, state transitions, constraints) and metadata (verb palettes, personas, human metadata).
* **Crate Location**: `dsl_types` (which is a DB-free leaf crate at Level 0, depending on nothing else in the workspace).
* **Derived Views**:
  * `dsl-core` (the runtime topology and transition gating view) would project a lean `Dag` shape by discarding verb palettes and DB fields.
  * `sem_os_ontology` would project the metadata-rich `SlotDef` hierarchy and `StateGraphDefBody` for documentation and discovery.
  * `sem_os_policy` would project access control allowed transitions.
  
Under this architecture, the 8 independent parsing paths would collapse into a single parse path at boot, resolving drift hazards by construction.

---

## "WHAT I DID NOT DO" Ledger

In strict compliance with Phase 2 — Step 3b read-only constraints:
1. **No Source Edits**: Did not modify any Rust source code or YAML configurations in the workspace.
2. **No Dependency Changes**: Did not alter any `Cargo.toml` or `Cargo.lock` files.
3. **No Parse Unification**: Made no attempt to unify the independent parsing paths or define any canonical structs in the code.
4. **No Git State Mutation (Except Report Commit)**: Did not perform checkout, branch, rebase, or clean operations on git. Only the analysis report file was added to git.
5. **No System Alterations**: Ran no service processes, test sweeps, or custom builds.

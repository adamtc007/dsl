# Phase 2 — Step 4 Prep: DAG Canonical-Source Refactor Fact-Gather Report

This report documents the static analysis, comparison, and repointing plan for the DAG and constellation configurations, satisfying the Step 4 Prep requirements.

---

## P1: Same-Key Test (The Fork)

To check for identifier alignment, we examined the `deal` domain pack configurations.

### 1. Keys Declared in Constellation Maps (→`SlotDef`)
Constellation maps are split into two files:
* **[deal_lifecycle.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_lifecycle.yaml)**:
  * Constellation Key: `deal.lifecycle`
  * Slots: `deal`, `client_group`, `group_kyc_clearance`, `kyc_case`, `cbu`, `participant`, `deal_contract`, `contract`, `deal_product`, `rate_card`, `onboarding_request`, `billing_profile`, `contract_template`
* **[deal_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_workspace.yaml)**:
  * Constellation Key: `deal.workspace`
  * Slots: `workspace_root`

### 2. Keys Declared in DAG Taxonomy (→`Dag`)
* **[deal_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/deal_dag.yaml)**:
  * Workspace Key: `deal`
  * Slots: `deal`, `client_group`, `group_kyc_clearance`, `kyc_case`, `cbu`, `contract`, `workspace_root`, `deal_participant`, `deal_contract`, `contract_template`, `deal_product`, `deal_rate_card`, `deal_onboarding_request`, `deal_document`, `deal_ubo_assessment`, `billing_profile`, `billing_period`, `rate_card_line`, `deal_sla`, `pricing_config`, `client_principal_relationship`, `billing_account_target`

### 3. Key Overlap and Mappings
* **Exact Matches**:
  `deal`, `deal_product`, `billing_profile`, `client_group`, `group_kyc_clearance`, `kyc_case`, `cbu`, `contract`, `workspace_root`, `deal_contract`, `contract_template`
* **Naming Mismatches (Aliases)**:
  * Constellation Map `rate_card` <--> DAG Taxonomy `deal_rate_card`
  * Constellation Map `onboarding_request` <--> DAG Taxonomy `deal_onboarding_request`
  * Constellation Map `participant` <--> DAG Taxonomy `deal_participant`

### Verdict: Same Identifier Space
Both configurations describe the same logical domain slots of the `deal` pack. The slight naming discrepancies are authoring variances. Unification under a single canonical configuration key map is highly viable.

---

## P2: Raw vs Seed Delta

Both structures parse the same `constellation_maps/*.yaml` files.

### 1. Citations
* **`RawConstellationMap`**: Defined at [dag_validator.rs:1670](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L1670)
* **`SeedConstellationMap`**: Defined at [composer.rs:141](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/src/resolver/composer.rs#L141)

### 2. Field-by-Field Mapping
| Field | `RawConstellationMap` | `SeedConstellationMap` |
| :--- | :--- | :--- |
| `constellation` / `fqn` | Yes (`Option<String>`) | Yes (`String`) |
| `description` | No | Yes (`Option<String>`) |
| `jurisdiction` | No | Yes (`String`) |
| `legacy_stack` | No | Yes (`SeedLegacyStack`) |
| `slots` | Yes (`RawConstellationSlot` map) | Yes (`SlotDef` map) |
| Slot: `state_machine` | Yes (`Option<String>`) | Yes (`Option<String>`) |
| Slot: `table` / `pk` / `join` | No | Yes |
| Slot: `cardinality` / `depends_on` | No | Yes |
| Slot: `verbs` (Verb Palette) | No | Yes |
| Slot: `children` (Recursive tree) | No | Yes |
| Slot: `closure` / `eligibility` | Yes (`Option<Value>`) | Yes (Fully typed structs) |
| Slot: `attachment_predicates` | Yes (`Vec<String>`) | Yes (`Vec<String>`) |
| Slot: `addition_predicates` | Yes (`Vec<String>`) | Yes (`Vec<String>`) |
| Slot: `aggregate_breach_checks` | Yes (`Vec<String>`) | Yes (`Vec<String>`) |
| Slot: `role_guard` | Yes (`Option<Value>`) | Yes (Fully typed struct) |
| Slot: `justification_required` | Yes (`Option<Value>`) | Yes (`Option<bool>`) |
| Slot: `audit_class` | Yes (`Option<Value>`) | Yes (`Option<String>`) |
| Slot: `completeness_assertion` | Yes (`Option<Value>`) | Yes (Fully typed struct) |

### Verdict: Lossless Collapse
`RawConstellationMap` parses a strict subset of `SeedConstellationMap`. The latter is the superset. A single-parse collapse using the fields of `SeedConstellationMap` (or a unified structure) is lossless.

---

## P3: Lean-View Required Fields

The runtime gating path (e.g. `DagRegistry` lookups) reads the following subset of fields from `Dag`:

* **`Dag` Fields**:
  * `workspace: String`
  * `slots: Vec<Slot>`
  * `cross_workspace_constraints: Vec<CrossWorkspaceConstraint>`
  * `derived_cross_workspace_state: Vec<DerivedCrossWorkspaceState>`
* **`Slot` Fields**:
  * `id: String`
  * `state_machine: Option<SlotStateMachine>`
  * `dual_lifecycle: Vec<DualLifecycle>`
  * `parent_slot: Option<ParentSlot>`
  * `state_dependency: Option<StateDependency>`
* **`StateMachine` (Transition-specific) Fields**:
  * `transitions: Vec<TransitionDef>`
* **`TransitionDef` Fields**:
  * `from: serde_yaml::Value`
  * `to: String`
  * `via: Option<serde_yaml::Value>`
* **`CrossWorkspaceConstraint` Fields**:
  * `id`, `source_workspace`, `source_slot`, `source_state`, `source_predicate`, `target_workspace`, `target_slot`, `target_transition`, `severity`
* **`DerivedCrossWorkspaceState` Fields**:
  * `id`, `host_workspace`, `host_slot`, `host_state`, `derivation`, `exposure`
* **`ParentSlot` Fields**:
  * `workspace`, `slot`, `join`
* **`StateDependency` Fields**:
  * `cascade_rules`, `severity`

This list represents the absolute minimum lean projection fields needed to construct `DagRegistry` and perform hot-path gateway verification.

---

## P4: Consumer Inventory Per Parse

Below is the inventory of all code modules consuming the parsed outputs:

### 1. `Dag` (and `LoadedDag`)
* **`dsl-core` (Loader & Validator)**:
  * [dag.rs:755, 785](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag.rs#L755): Loader routines.
  * [dag_validator.rs:1055, 1082, 1101, 1585](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L1055): Validates constraints, state machines, review cadences, and dual-lifecycles.
* **`sem_os_core` (Composer)**:
  * [composer.rs:225, 1015](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/src/resolver/composer.rs#L225): Composes structural slots, attributes, and resolved transitions.
* **`dsl-runtime` (Runtime)**:
  * [gate_checker.rs:41](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/gate_checker.rs#L41): Feeds cross-workspace transition constraints.
  * [postgres_child_resolver.rs:30](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs#L30): Drives parent/child mapping resolutions.
  * [hierarchy_cascade.rs:32](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs#L32): Coordinates cascading child updates.
  * [derived_state_projector.rs:19](file:///Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs#L19): Hydrates derived cross-workspace states.

### 2. `RawConstellationMap`
* **`dsl-core` (Validator)**:
  * [dag_validator.rs:639](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L639): Parses raw maps to validate schema coordination between database tables and loaded DAG configurations.

### 3. `SeedConstellationMap`
* **`sem_os_core` (Composer)**:
  * [composer.rs:164](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/src/resolver/composer.rs#L164): Parses maps and converts them directly to `ConstellationMapDefBody`.

### 4. `DomainPackManifest`
* **`sem_os_policy` (Governance)**:
  * [domain_pack.rs:171, 279, 362, 406](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/domain_pack.rs#L171): Gathers fingerprints, checks pack reloads, and validates policies.

---

## P5: Recon Mechanism

### 1. `domain_pack_dsl_reconciliation` Comparison logic
* **Comparison Code**: [domain_pack_dsl_reconciliation.rs:374, 417](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/domain_pack_dsl_reconciliation.rs#L374)
* **What it does**:
  1. Calls `owned_dag_verbs`, which loads the raw files for each owned DAG and extracts **all FQN tokens** (e.g. `cbu.read`) matching the allowed prefixes using text-based token splitting (`prefixed_tokens`/`fqn_tokens`).
  2. Compares these raw-extracted tokens against allowed journey pack verbs, macro expansions, and transition verbs.
* **Under a single source**:
  * **Unnecessary**: The lexical text scanning (`fqn_tokens`) and prefix matching of raw files to detect structural mismatches becomes obsolete. The compiler can statically guarantee that transitions and slots match since they originate from the same struct fields.
  * **Remains**: The coverage check confirming that the journey pack authorization lists cover all verbs declared in the topology remains.

### 2. `cover_dsl_surfaces` Comparison logic
* **Comparison Code**: [domain_pack.rs:1652](file:///Users/adamtc007/Dev/dsl/crates/sem_os_policy/src/domain_pack.rs#L1652)
* **What it does**:
  Compares the directory listing of `/config/packs` and `/config/sem_os_seeds/dag_taxonomies` against the files declared as owned in domain pack manifests.
* **Under a single source**:
  * **Remains**: This remains fully necessary to prevent orphan files on disk.

---

## P6: DagRegistry Derivation

* **Verification of Constructor Reads**:
  We scanned the constructors (`from_loaded`, `from_dir`) and indexing routines (`rebuild_indices`) in `deleted-dag-registry.txt`.
* **Verdict: Only Topology**:
  The registry **exclusively** reads:
  1. `slot.id`, `t.via`, `t.from`, `t.to` (transitions)
  2. `cross_workspace_constraints` (targets, transitions)
  3. `derived_cross_workspace_state` (hosts)
  4. `parent_slot` (links)
  
It does **not** read any description, owner, rationale, review cadence, or other rich metadata. Thus, `DagRegistry` can be cleanly derived entirely from the lean topology view.

---

## "WHAT I DID NOT DO" Ledger

In strict compliance with Phase 2 — Step 4 Prep constraints:
1. **No Source Edits**: Did not modify any Rust code or YAML seeds.
2. **No Refactoring**: Did not design or write any canonical structures.
3. **No Parse Adjustments**: Retained all existing 8 parser paths as-is.
4. **No Git Mutations (Except Commits)**: Staged and committed only this report file.
5. **No System Alterations**: Ran no builds or changes on user services.

# Track C Consolidation Receipt

## 1. Task Invariants and Deliverables

Track C establishes the foundations of the slot-keyed gating architecture within `dsl_types` and its associated tripwire validator inside `dsl-core`.

Four correctness criteria were met and validated:
1. **`SlotKey` Derives**: `SlotKey` derives `Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize` to ensure it functions perfectly as a `BTreeMap` key without compiler errors.
2. **C3 Resolution Mechanic (Dot-Path Descent)**: Resolving a `SlotKey` walks segment-by-segment down the nested `BTreeMap<String, SlotDef>` structure within the `SeedConstellationMap` registry. This guarantees unique resolution along nested children slots rather than performing flat-string matching.
3. **Severity Policy**: If a gating state points at a non-existent slot, it is flagged as a hard `DagError::OrphanSlotGatingKey` validation error.
4. **C4 Directional Validator Invariant**: Asserted the subset invariant `gating.keys() ⊆ topology.slots()` ensuring that:
   - A ghost key (present in gating, unresolved in topology) -> Fail.
   - A valid key (present in gating, resolved in topology) -> Pass.
   - A topology slot with no gating entry -> Pass (sparse map representation).

---

## 2. Verbatim Test Body (`test_validate_slot_gating_states_subset_direction`)

The following test case was added to the bottom of `crates/dsl-core/src/config/dag_validator.rs` inside the `mod tests` block:

```rust
    #[test]
    fn test_validate_slot_gating_states_subset_direction() {
        let constellation_yaml = r#"
constellation: my_constellation
jurisdiction: GB
slots:
  parent_slot:
    type: cbu
    cardinality: root
    children:
      child_slot:
        type: entity
        cardinality: optional
"#;
        let seed: SeedConstellationMap = serde_yaml::from_str(constellation_yaml).unwrap();
        let mut constellations = BTreeMap::new();
        constellations.insert("my_constellation".to_string(), seed);

        // 1. Ghost key (present in gating, unresolved in topology) -> Fail
        let mut gating_states = BTreeMap::new();
        gating_states.insert(
            SlotKey {
                constellation: "my_constellation".to_string(),
                path: "parent_slot.does_not_exist".to_string(),
            },
            SlotGatingState {
                status: GatingStatus::Pending,
            },
        );
        let report = validate_slot_gating_states(&constellations, &gating_states);
        assert!(!report.is_clean());
        assert_eq!(report.errors.len(), 1);
        match &report.errors[0] {
            DagError::OrphanSlotGatingKey { constellation, path } => {
                assert_eq!(constellation, "my_constellation");
                assert_eq!(path, "parent_slot.does_not_exist");
            }
            _ => panic!("Expected OrphanSlotGatingKey error"),
        }

        // 2. Valid key (present in gating, resolved in topology) -> Pass
        let mut gating_states = BTreeMap::new();
        gating_states.insert(
            SlotKey {
                constellation: "my_constellation".to_string(),
                path: "parent_slot.child_slot".to_string(),
            },
            SlotGatingState {
                status: GatingStatus::Gated,
            },
        );
        let report = validate_slot_gating_states(&constellations, &gating_states);
        assert!(report.is_clean(), "Expected clean report, got: {:?}", report.errors);

        // 3. Sparse absence (topology slot with no gating entry) -> Pass
        let gating_states = BTreeMap::new(); // Empty, meaning both parent_slot and parent_slot.child_slot are absent in gating
        let report = validate_slot_gating_states(&constellations, &gating_states);
        assert!(report.is_clean(), "Expected clean report, got: {:?}", report.errors);
        
        // 4. Unknown constellation -> Fail
        let mut gating_states = BTreeMap::new();
        gating_states.insert(
            SlotKey {
                constellation: "unknown_constellation".to_string(),
                path: "parent_slot".to_string(),
            },
            SlotGatingState {
                status: GatingStatus::Pending,
            },
        );
        let report = validate_slot_gating_states(&constellations, &gating_states);
        assert!(!report.is_clean());
        assert_eq!(report.errors.len(), 1);
        match &report.errors[0] {
            DagError::OrphanSlotGatingKey { constellation, path } => {
                assert_eq!(constellation, "unknown_constellation");
                assert_eq!(path, "parent_slot");
            }
            _ => panic!("Expected OrphanSlotGatingKey error"),
        }
    }
```

---

## 3. Test Execution Outcomes (`cargo test`)

Running `cargo test --workspace` inside `dsl` yields the following verbatim outcome summaries across all library and integration test targets (representing 892 total passed tests):

```text
test result: ok. 286 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.25s (dsl-core unit tests, including test_validate_slot_gating_states_subset_direction)
test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s (ast_golden integration tests)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (dag_gate_metadata integration tests)
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s (dag_golden integration tests)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (dep_ordering integration tests)
test result: ok. 0 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s (domain_pack_dsl_reconciliation integration tests)
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s (effect_declarations integration tests)
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (regression_baseline_health integration tests)
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (slot_binding integration tests)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (tranche_d_facade_evidence integration tests)
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s (verb_flavour_catalogue integration tests)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (cbu_evidence_substates integration tests)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (cbu_validity integration tests)
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s (closure_lint integration tests)
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s (eligibility_lint integration tests)
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (frontier_recursive integration tests)
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (frontier_skeleton integration tests)
test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s (phase2_acceptance integration tests)
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s (resolver_lux_sicav integration tests)
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s (resolver_manifest integration tests)
test result: ok. 0 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 0.00s (shape_rule_composition integration tests)
test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (sem_os_core unit tests)
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (constellation_gate_metadata integration tests)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (discovery_pipeline integration tests)
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (sem_os_ontology unit tests)
test result: ok. 347 passed; 0 failed; 7 ignored; 0 measured; 0 filtered out; finished in 0.00s (sem_os_policy unit tests)
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (sem_os_types unit tests)
test result: ok. 1 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out; finished in 0.09s (dsl_core doc-tests)
test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.11s (sem_os_core doc-tests)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s (sem_os_ontology doc-tests)
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s (sem_os_policy doc-tests)
```

---

## 4. Workspace Compilation Status (`cargo check`)

### dsl workspace:
Running `cargo check --workspace --all-targets` in `dsl` resolves cleanly:
```text
    Checking sem_os_core v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_core)
    Checking sem_os_policy v0.1.0 (/Users/adamtc007/dev/dsl/crates/sem_os_policy)
    Checking dsl-integration-tests v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl-integration-tests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.78s
```

### ob-poc workspace:
Running `cargo check --workspace --lib --bins` in `ob-poc/rust` compiles cleanly against the patched `dsl` workspace:
```text
    Checking ob-poc-compiler v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-compiler)
    Checking ob-poc-ontology v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-ontology)
    Checking dsl-semos-frontend v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-semos-frontend)
    Checking ob-agentic v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-agentic)
    Checking dsl-analysis v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-analysis)
    Checking sem_os_obpoc_adapter v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_obpoc_adapter)
    Checking dsl-runtime v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-runtime)
    Checking sem_os_client v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_client)
    Checking ob-poc-boundary v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-boundary)
    Checking sem_os_mcp v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_mcp)
    Checking sem_os_postgres v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_postgres)
    Checking dsl-lsp v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/dsl-lsp)
    Checking ob-poc-agent v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-agent)
    Checking ob-poc v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust)
    Checking sem_os_server v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_server)
    Checking sem_os_harness v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_harness)
    Checking xtask v0.0.0 (/Users/adamtc007/Developer/ob-poc/rust/xtask)
    Checking ob-poc-web v0.1.0 (/Users/adamtc007/Developer/ob-poc/rust/crates/ob-poc-web)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 46.92s
```
*(Pre-existing `sqlx` offline DB prepare schema errors in test-target remain isolated/untouched).*

---

## 5. Blast Radius (`git diff --stat`)

Running `git diff HEAD --stat` confirms the scope of changes:
```text
 crates/dsl-core/src/ast.rs                         |  128 +-
 crates/dsl-core/src/binding_context.rs             |   31 +-
 crates/dsl-core/src/config/dag_validator.rs        |  159 +-
 crates/dsl-core/src/config/loader.rs               |  180 +-
 crates/dsl-core/src/config/mod.rs                  |   14 +-
 crates/dsl-core/src/config/pack_loader.rs          |  103 +
 crates/dsl-core/src/config/types.rs                |    4 +-
 crates/dsl-core/src/config/validator.rs            |   31 +-
 crates/dsl-core/src/execution_dag.rs               |    4 +-
 crates/dsl-core/src/lib.rs                         |   14 +-
 crates/dsl_types/src/constellation_map_def.rs      |   22 +
 crates/dsl_types/src/lib.rs                        |    3 +-
 .../sem_os_ontology/src/constellation_map_def.rs   |    3 +-
 reports/consolidation/27-track-b.md                | 3643 ++++++++++++++++++++
 reports/consolidation/28-track-c.md                |   61 +
 15 files changed, 4362 insertions(+), 38 deletions(-)
```
Nothing was modified in guards, eligibility, or verb palette modules, and no default or materialized `Pending` entries are produced.

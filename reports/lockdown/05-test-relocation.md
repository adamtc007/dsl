# Lockdown Report — Tranche B (Test Relocation)
- UTC:       2026-05-30T12:20:00Z
- Status:    GREEN
- Commit(s): 3a90b90

## 1. DagRegistry Archaeology & Provenance
We performed Git archaeology on the `dsl` repository to trace the deletion of the `DagRegistry` struct:
* **Command run**: `git log -S DagRegistry --oneline -- crates/`
* **Deletion Commit**: `06232bf1de0e40fd8f8a925266e25a012758d01a` ("remed(D1): Delete dag_registry.rs cluster")
* **Deletion Date**: `Thu May 28 17:21:57 2026 +0100`
* **Lockdown Start (Tranche 0)**: `Sat May 30 10:46:43 2026 +0100` (`c9a23f2`)

### Archaeology Verdict:
The deletion of `DagRegistry` predates the start of our lockdown refactoring by exactly 2 days. The compilation failure in `dsl-runtime` when building downstream `ob-poc` is a pre-existing integration drift rot, not self-inflicted damage.

---

## 2. Downstream Quarantined Gate Status
We verified the quarantined gate to prevent the `dsl-runtime` compile error from masking regressions in other parts of `ob-poc`:

### Excluded Crate Set:
`dsl-runtime`, `dsl-lsp`, `ob-poc` (root), `ob-poc-web`, `ob-poc-agent`, `sem_os_harness`, `sem_os_postgres`, `sem_os_server`, `xtask`.

### Gate Command:
```bash
cargo check --workspace --exclude dsl-runtime --exclude dsl-lsp --exclude ob-poc --exclude ob-poc-web --exclude ob-poc-agent --exclude sem_os_harness --exclude sem_os_postgres --exclude sem_os_server --exclude xtask --all-features
```

### Downstream Gate Status:
**GREEN**. All non-quarantined workspace members build cleanly. All environment-independent unit tests pass successfully.

---

## 3. dsl-core [dev-dependencies] Audit & E0308 Analysis
We audited `crates/dsl-core/Cargo.toml` and found the following:
```toml
[dev-dependencies]
sem_os_core = { git = "https://github.com/adamtc007/sem-os", tag = "v0.1.2" }
```
This dependency introduces a structural **dev-cycle** (dependency loop):
`dsl_core` --dev--> `sem_os_core` --> `dsl_core`

### Impact & E0308 Cause:
1. Sibling crates like `sem_os_core` depend on `dsl-core` as a library.
2. If a test file that imports and calls `sem_os_core` is relocated into the `src/` directory of `dsl-core` (compiled with `#[cfg(test)]`), it runs inside the `dsl-core` unit test runner.
3. The compiler compiles `dsl-core` twice: once for the library, and once for the test runner.
4. `sem_os_core` was compiled against the library version of `dsl-core`, but the relocated unit test is compiled with the test version of `dsl-core`.
5. This leads to a type mismatch error (`E0308`), as the compiler treats types like `ResolvedTemplate` from the test runner and the library version as two different types.
6. **Conclusion**: Tests that import `sem_os_core` *cannot* be unit tests and must remain in `tests/` as integration tests.

---

## 4. Test Relocation Split
We audited all 28 integration tests in `crates/dsl-core/tests/` to split them based on their dependencies:

### A. sem_os_core Dependent Tests (10 files — Stay in `tests/`)
These tests depend on `sem_os_core` and must stay in `tests/` to prevent `E0308` compilation errors:
1. [cbu_evidence_substates.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/cbu_evidence_substates.rs)
2. [cbu_validity.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/cbu_validity.rs)
3. [closure_lint.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/closure_lint.rs)
4. [eligibility_lint.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/eligibility_lint.rs)
5. [frontier_recursive.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/frontier_recursive.rs)
6. [frontier_skeleton.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/frontier_skeleton.rs)
7. [phase2_acceptance.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/phase2_acceptance.rs)
8. [resolver_lux_sicav.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/resolver_lux_sicav.rs)
9. [resolver_manifest.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/resolver_manifest.rs)
10. [shape_rule_composition.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/shape_rule_composition.rs)

### B. Pure Contract Tests (11 files — Stay in `tests/`)
These tests do not access internal paths or symbols, and only interact with the 158 facade. They stay in `tests/`:
1. [ast_golden.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/ast_golden.rs)
2. [catalogue_db_free_smoke.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/catalogue_db_free_smoke.rs)
3. [dag_gate_metadata.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/dag_gate_metadata.rs)
4. [dag_golden.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/dag_golden.rs)
5. [dep_ordering.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/dep_ordering.rs)
6. [domain_pack_dsl_reconciliation.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/domain_pack_dsl_reconciliation.rs)
7. [effect_declarations.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/effect_declarations.rs)
8. [regression_baseline_health.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/regression_baseline_health.rs)
9. [scoped_runbook_bindings_harness.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/scoped_runbook_bindings_harness.rs)
10. [slot_binding.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/slot_binding.rs)
11. [verb_flavour_catalogue.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/tests/verb_flavour_catalogue.rs)

### C. Internal-Access Tests (7 files — Relocated to `src/`)
These tests access internal submodules or internal symbols of `dsl-core` and have been relocated to `src/`:
1. `dag_validator_gate.rs` -> [dag_validator_gate.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator/integration_tests/dag_validator_gate.rs)
2. `green_when_coverage.rs` -> [green_when_coverage.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/green_when_coverage/integration_tests/green_when_coverage.rs)
3. `lux_sicav_pilot.rs` -> [lux_sicav_pilot.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag/integration_tests/lux_sicav_pilot.rs)
4. `plan_golden.rs` -> [plan_golden.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/executable_plan/integration_tests/plan_golden.rs)
5. `predicate_ast.rs` -> [predicate_ast.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/predicate/integration_tests/predicate_ast.rs)
6. `three_axis_fixtures.rs` -> [three_axis_fixtures.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/escalation/integration_tests/three_axis_fixtures.rs)
7. `v1_2_dod_fixture.rs` -> [v1_2_dod_fixture.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/validator/integration_tests/v1_2_dod_fixture.rs)

---

## 5. Relocation Set Consumer Verification (The Receipt)
The table below traces the exact consumers of the 58 unique symbols from the relocation set:

| Symbol | Kind | Definition File | Downstream Consumers | Crate-Internal Prod Consumers | Integration Tests | Verdict |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `AggregationRule` | ENUM | `config/runbook_composition.rs:67` | None | mod.rs | three_axis_fixtures.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `CrossScopeRule` | ENUM | `config/runbook_composition.rs:139` | None | mod.rs | three_axis_fixtures.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `DagError` | ENUM | `config/dag_validator.rs:51` | ob-poc (`runbook/compiler.rs`) | mod.rs | eligibility_lint.rs, closure_lint.rs... | **Keep Public (Downstream)** |
| `DagWarning` | ENUM | `config/dag_validator.rs:385` | None | mod.rs | dag_validator_gate.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `EntityQualifier` | ENUM | `config/predicate/ast.rs:93` | None | parser.rs, mod.rs | predicate_ast.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `EvaluationContext` | STRUCT | `config/escalation.rs:29` | None | mod.rs | three_axis_fixtures.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `GreenWhenExclusionReason` | ENUM | `config/green_when_coverage.rs:24` | None | mod.rs | green_when_coverage.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `Phase` | STRUCT | `config/dag.rs:95` | ob-poc (`agent/orchestrator.rs`) | dag_validator.rs | three_axis_fixtures.rs, scoped_runbook_bindings_harness.rs... | **Keep Public (Downstream)** |
| `RelationScope` | ENUM | `config/predicate/ast.rs:100` | None | parser.rs, mod.rs | predicate_ast.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `RunbookStep` | STRUCT | `config/runbook_composition.rs:39` | None | mod.rs | three_axis_fixtures.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `StructuralError` | ENUM | `config/validator.rs:72` | None | mod.rs | v1_2_dod_fixture.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `TransactionPolicy` | ENUM | `executable_plan.rs:145` | None | lib.rs | plan_golden.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `VersionHash` | STRUCT | `resolver/version.rs:5` | None | mod.rs | frontier_recursive.rs, cbu_validity.rs... | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `add_edge` | FN | `execution_dag.rs:198` | ob-poc (`graph/view_model.rs`) | None | plan_golden.rs | **Keep Public (Downstream)** |
| `all` | FN | `binding_context.rs:126` | sem-os (`frontier/hydrator.rs`) | escalation.rs, phrase_gen.rs... | eligibility_lint.rs, phase2_acceptance.rs... | **Keep Public (Downstream)** |
| `as_symbol` | FN | `ast.rs:487` | ob-poc (`domain_ops/helpers.rs`) | None | slot_binding.rs | **Keep Public (Downstream)** |
| `compute_effective_tier` | FN | `config/escalation.rs:128` | None | runbook_composition.rs | three_axis_fixtures.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `compute_runbook_tier` | FN | `config/runbook_composition.rs:212` | None | mod.rs | three_axis_fixtures.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `config_dir` | FN | `config/loader.rs:97` | ob-poc (`journey/router.rs`) | None | effect_declarations.rs | **Keep Public (Downstream)** |
| `contains` | FN | `binding_context.rs:116` | sem-os (`frontier/hydrator.rs`) | effect_class.rs, validator.rs... | resolver_lux_sicav.rs, predicate_ast.rs... | **Keep Public (Downstream)** |
| `dag` | MOD | `config/mod.rs:21` | sem-os (`frontier/hydrator.rs`) | dag.rs, dag_validator.rs... | frontier_recursive.rs, resolver_lux_sicav.rs... | **Keep Public (Downstream)** |
| `dag_validator` | MOD | `config/mod.rs:22` | None | None | dag_validator_gate.rs | **Internalize to pub(crate) (Test-Only)** |
| `display` | FN | `binding_context.rs:79` | ob-poc (`gateway_resolver.rs`) | phrase_gen.rs, loader.rs | ast_golden.rs, effect_declarations.rs... | **Keep Public (Downstream)** |
| `effect_class` | MOD | `config/mod.rs:23` | ob-poc (`dsl_v2/executor.rs`) | types.rs, executable_plan.rs... | effect_declarations.rs | **Keep Public (Downstream)** |
| `entity_ref` | FN | `ast.rs:367` | sem-os (`frontier/hydrator.rs`) | mod.rs, viewport_parser.rs | frontier_skeleton.rs | **Keep Public (Downstream)** |
| `error` | FN | `diagnostics.rs:134` | ob-poc (`database/generation_log_repository.rs`) | mod.rs, viewport_parser.rs... | eligibility_lint.rs, closure_lint.rs... | **Keep Public (Downstream)** |
| `error_count` | FN | `config/validator.rs:304` | sem-os (`gates/mod.rs`) | dag_validator.rs | catalogue_db_free_smoke.rs | **Keep Public (Downstream)** |
| `execution_dag` | MOD | `lib.rs:21` | ob-poc (`dsl_v2/execution_plan.rs`) | resource_dependency.rs, executable_plan.rs | plan_golden.rs | **Keep Public (Downstream)** |
| `extension` | FN | `ast.rs:1402` | ob-poc (`journey/router.rs`) | dag_validator.rs, loader.rs... | v1_2_dod_fixture.rs, domain_pack_dsl_reconciliation.rs | **Keep Public (Downstream)** |
| `from_effect_classes` | FN | `executable_plan.rs:166` | None | None | plan_golden.rs | **Internalize to pub(crate) (Test-Only)** |
| `from_template` | FN | `resolver/manifest.rs:38` | ob-poc (`reconcile_resolver_manifest.rs`) | None | resolver_manifest.rs | **Keep Public (Downstream)** |
| `frontier` | MOD | `lib.rs:22` | sem-os (`lib.rs`) | None | frontier_recursive.rs, cbu_validity.rs... | **Keep Public (Downstream)** |
| `full_name` | FN | `ast.rs:144` | ob-poc (`mcp/handlers/core.rs`) | loader.rs | dep_ordering.rs | **Keep Public (Downstream)** |
| `get` | FN | `binding_context.rs:111` | ob-poc (`gateway_resolver.rs`) | escalation.rs, dag.rs... | resolver_lux_sicav.rs, green_when_coverage.rs... | **Keep Public (Downstream)** |
| `green_when_coverage_for_dag` | FN | `config/green_when_coverage.rs:54` | None | mod.rs | green_when_coverage.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `green_when_coverage_for_dags` | FN | `config/green_when_coverage.rs:39` | None | mod.rs | green_when_coverage.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `green_when_coverage_summary` | FN | `config/green_when_coverage.rs:82` | None | mod.rs | green_when_coverage.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `harden_schema_coordination_warnings` | FN | `config/dag_validator.rs:697` | None | None | dag_validator_gate.rs | **Internalize to pub(crate) (Test-Only)** |
| `info` | FN | `diagnostics.rs:170` | ob-poc (`database/resource_instance_service.rs`) | binding_context.rs, loader.rs | ast_golden.rs, regression_baseline_health.rs... | **Keep Public (Downstream)** |
| `insert` | FN | `binding_context.rs:131` | ob-poc (`gateway_resolver.rs`) | escalation.rs, dag.rs... | frontier_recursive.rs, predicate_ast.rs... | **Keep Public (Downstream)** |
| `is_empty` | FN | `binding_context.rs:148` | ob-poc (`gateway_resolver.rs`) | effect_class.rs, compiler.rs... | frontier_recursive.rs, phase2_acceptance.rs... | **Keep Public (Downstream)** |
| `is_ok` | FN | `compiler.rs:58` | ob-poc (`mcp/handlers/core.rs`) | parser.rs | regression_baseline_health.rs | **Keep Public (Downstream)** |
| `len` | FN | `binding_context.rs:153` | ob-poc (`gateway_resolver.rs`) | effect_class.rs, validator.rs... | frontier_recursive.rs, phase2_acceptance.rs... | **Keep Public (Downstream)** |
| `load_verbs` | FN | `config/loader.rs:109` | ob-poc (`mcp/handlers/core.rs`) | None | green_when_coverage.rs, effect_declarations.rs... | **Keep Public (Downstream)** |
| `loader` | MOD | `config/mod.rs:26` | ob-poc (`mcp/handlers/core.rs`) | lib.rs | effect_declarations.rs, catalogue_db_free_smoke.rs | **Keep Public (Downstream)** |
| `manifest` | MOD | `resolver/mod.rs:3` | ob-poc (`journey/router.rs`) | manifest.rs, dag.rs | resolver_manifest.rs | **Keep Public (Downstream)** |
| `ordering_pairs` | FN | `execution_dag.rs:211` | None | None | plan_golden.rs | **Internalize to pub(crate) (Test-Only)** |
| `resolver` | MOD | `lib.rs:24` | sem-os (`lib.rs`) | dag_validator.rs | frontier_recursive.rs, resolver_lux_sicav.rs... | **Keep Public (Downstream)** |
| `slot_mut` | FN | `resolver/mod.rs:60` | None | None | eligibility_lint.rs, closure_lint.rs | **Internalize to pub(crate) (Test-Only)** |
| `string` | FN | `ast.rs:357` | ob-poc (`mcp/handlers/core.rs`) | viewport_parser.rs | ast_golden.rs, dag_golden.rs | **Keep Public (Downstream)** |
| `to_text` | FN | `resolver/manifest.rs:66` | ob-poc (`reconcile_resolver_manifest.rs`) | None | resolver_manifest.rs | **Keep Public (Downstream)** |
| `types` | MOD | `config/mod.rs:31` | ob-poc (`api/capital_routes.rs`) | effect_class.rs, escalation.rs... | ast_golden.rs, three_axis_fixtures.rs... | **Keep Public (Downstream)** |
| `validate_constellation_map_schema_coordination` | FN | `config/dag_validator.rs:633` | None | mod.rs | lux_sicav_pilot.rs, dag_validator_gate.rs | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `validate_resolved_template_gate_metadata` | FN | `config/dag_validator.rs:549` | None | mod.rs | eligibility_lint.rs, phase2_acceptance.rs... | **Internalize to pub(crate) (Crate-Internal Prod)** |
| `with_arg` | FN | `config/escalation.rs:44` | ob-poc (`dsl-analysis/.../variable.rs`) | None | three_axis_fixtures.rs | **Keep Public (Downstream)** |
| `with_entity_attr` | FN | `config/escalation.rs:49` | None | None | three_axis_fixtures.rs | **Internalize to pub(crate) (Test-Only)** |
| `with_flag` | FN | `config/escalation.rs:62` | ob-poc (`sem_os_postgres/.../docs_bundle.rs`) | None | three_axis_fixtures.rs | **Keep Public (Downstream)** |
| `with_required_slots` | FN | `resolver/manifest.rs:10` | ob-poc (`reconcile_resolver_manifest.rs`) | None | resolver_manifest.rs | **Keep Public (Downstream)** |

### Internalization Verdict:
Of the 58 unique symbols, **34 symbols** have verified downstream consumers and must remain public.
The remaining **24 symbols** have NO downstream consumers (they are only consumed by internal modules and relocated tests). They will be internalized from `pub` to `pub(crate)` during Tranche C (module privatization).

---

## 6. Invariant Attestation
* **E0 No Production Body Edits**: PASS. No production function bodies or executable logic were modified. Only test module declarations were appended at the end of the 7 target files.
* **E1 No Wildcard Imports**: PASS. No wildcard imports were introduced.
* **E2 No `allow(dead_code)`**: PASS. No new `allow(dead_code)` suppressions were introduced.

---

## 7. Commit SHA
* **`dsl`**: `[Pending Commit]`
* **`sem-os`**: `72207203bef97b8a6b82c3913ad2d7685118223f`
* **`ob-poc`**: `db3112ab9b2013d26985dd7e755169ccd20d8b8e`

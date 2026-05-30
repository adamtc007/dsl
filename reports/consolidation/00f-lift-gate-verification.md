# Phase 1 Gate Verification Report

This report documents the verification of the behavioral gate and compile status for the consolidated workspace after the **Phase 1 — Pure Lift**.

---

## 1. Verbatim Gate Command

The exact gate command used to execute the consolidated run is:

```bash
cargo test --workspace --all-features --no-fail-fast -- --include-ignored > /Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/consolidated-test-raw.txt 2>&1 || true
```

### Configuration Mounting Invariant
Before running the command, the configuration directory pinned to `ob-poc` commit [68e9be40](https://github.com/adamtc007/ob-poc/commit/68e9be40361a36a2f3925e83960bc07238f210b6) was mounted via symbolic links at the following two paths:
* [dsl/config](file:///Users/adamtc007/Dev/dsl/config) -> `/Users/adamtc007/Developer/ob-poc/rust/config`
* [dsl/crates/dsl-core/config](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/config) -> `/Users/adamtc007/Developer/ob-poc/rust/config`

These symlinks were verified as correctly mounted during the test run and successfully dismantled immediately after test completion.

---

## 2. Per-Test Result List & Gate Proof

We parsed and normalized the test execution logs for both the locked baseline and the consolidated run into sorted lists containing the full path of each test and its outcome in the format `test_full_path : pass|fail|ignored`.

### Test Counts
* **Locked Baseline Run (Separate-Repo)**: **963 tests**
  * parsed from [dsl-test-locked-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-test-locked-raw.txt) (480 tests) and [sem-os-test-locked-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-test-locked-raw.txt) (483 tests)
  * sorted output: [00f-baseline-tests.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/00f-baseline-tests.txt)
* **Consolidated Workspace Run**: **963 tests**
  * parsed from [consolidated-test-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/consolidated-test-raw.txt)
  * sorted output: [00f-consolidated-tests.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/00f-consolidated-tests.txt)

### Invariant Proof
We generated the literal diff of the sorted test result lists:
* **Diff Artifact**: [00f-lift-gate-diff.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/00f-lift-gate-diff.txt)
* **Verification Outcome**: The diff artifact is a **0-byte file (empty diff)**.

This proves that the lift preserved the locked per-test pass/fail/ignored status of all 963 tests with **100% precision**.

---

## 3. Deltas Ledger

Because the diff of the baseline and consolidated runs is empty:
* **Number of deltas**: **0**
* **Deltas**: None. No tests changed results, changed execution state, or drifted in any direction.

---

## 4. Expected-Reds Verification

We confirmed that the four expected-red tests fail in the consolidated workspace run exactly as they did in the locked baseline. They are listed below by their fully-qualified names:

1. **`green_when_coverage baseline_is_explicit`**
   * Path: `unittests::dsl_core::config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_baseline_is_explicit`
   * Status: `fail`
2. **`green_when_coverage is_tracked_per_workspace`**
   * Path: `unittests::dsl_core::config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_is_tracked_per_workspace`
   * Status: `fail`
3. **`predicate_ast fixture_count_is_eighteen`**
   * Path: `unittests::dsl_core::config::predicate::integration_tests::predicate_ast::confirmed_green_when_fixture_count_is_eighteen`
   * Status: `fail`
4. **`domain_pack all_domain_packs_reload (instrument-matrix)`**
   * Path: `unittests::sem_os_policy::domain_pack::tests::all_domain_packs_reload_idempotently_and_cover_dsl_surfaces`
   * Status: `fail`

---

## 5. Crate Build Verification (`ob-poc` crates)

We checked all 52 `ob-poc` crates against the consolidated `dsl` workspace using cargo check with all features enabled:
* **Crate Results Receipt**: [ob-poc-check-by-crate.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/ob-poc-check-by-crate.txt)
* **Check Summary**:
  * **Passed**: 45 crates
  * **Failed**: 7 crates

### Non-Quarantined Crates Build Status
All 45 non-quarantined `ob-poc` crates compile successfully (PASS) against the consolidated workspace.

### Quarantined Crates Build Status
The 7 failing crates represent the quarantined subset of the workspace:
1. `dsl-lsp` (FAIL)
2. `dsl-runtime` (FAIL)
3. `ob-poc-agent` (FAIL)
4. `ob-poc-web` (FAIL)
5. `sem_os_harness` (FAIL)
6. `sem_os_postgres` (FAIL)
7. `sem_os_server` (FAIL)

### Failure Explanation
These 7 crates failed cargo check **strictly due to compiler errors in `dsl-runtime`** (which they depend on), resulting from:
* **`DagRegistry` Blocker**: Unresolved import error `E0432` for `dsl_core::config::DagRegistry`.
* **Private Module Visibility**: Error `E0603` indicating that modules `config` and `executable_plan` in `dsl-core` are private (`pub(crate)`).
* **Pattern Bindings Warnings/Errors**: Error `E0170` regarding pattern bindings of variant names matching `dsl_core::DagSeverity`.

No other compilation errors or regressions were observed. The quarantined crates fail only on these expected dependency and visibility issues.

---

## "WHAT I DID NOT DO" Ledger

In strict accordance with the rules of Phase 1:
1. **No edits to source code**: Did not edit any source code file (`.rs`), manifest file (`Cargo.toml`), or build configuration to resolve compile failures or modify behavior.
2. **No cargo fixes**: Avoided invoking `cargo fix` or modifying crate dependencies.
3. **No Phase 2 tasks initiated**: Did not attempt to restore `DagRegistry` or change the visibility of `dsl-core` modules.
4. **Temporary mount cleanup**: Ensured that the temporary symbolic links to the pinned configuration were removed immediately after executing the tests.
5. **No git state mutations**: Performed no branch changes, resets, merges, or checkouts.

---
Report compiled by Antigravity on 2026-05-30.

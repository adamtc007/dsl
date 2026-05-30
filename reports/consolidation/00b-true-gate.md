# Research Phase — Leg 1b: True Behavioral Gate Report

This report documents the findings from enabling and running the previously ignored integration and config-dependent tests across the `dsl` and `sem-os` workspaces.

---

## 1. Config Mappings and Location Mechanism

The config-dependent tests locate the `ob-poc` config directory using different mechanisms, depending on whether it is the runtime loader or the integration test files.

### A. Runtime Config Loader (`ConfigLoader`)
As defined in [loader.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/loader.rs#L25-L54), the `ConfigLoader::from_env()` uses the following path resolution order:
1. `DSL_CONFIG_DIR` environment variable (explicit override check at line 33).
2. Relative `"config"` directory check (line 38).
3. `CARGO_MANIFEST_DIR/config` check (line 45).
4. Workspace root config traversal (line 51).

### B. Integration Tests (`seed_path` & `config_root`)
Unlike the runtime loader, the integration tests themselves **do not** check environment variables. They resolve configuration directories via compile-time cargo manifest paths:
* In `dsl-core` integration tests ([lux_sicav_pilot.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag/integration_tests/lux_sicav_pilot.rs#L10-L14) & [green_when_coverage.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/green_when_coverage/integration_tests/green_when_coverage.rs#L8-L12)), the seed path helper is hardcoded to:
  ```rust
  fn seed_path(relative: &str) -> PathBuf {
      PathBuf::from(env!("CARGO_MANIFEST_DIR"))
          .join("../../config/sem_os_seeds")
          .join(relative)
  }
  ```
  Since `CARGO_MANIFEST_DIR` resolves to the crate manifest path (e.g. `/Users/adamtc007/Dev/dsl/crates/dsl-core`), it looks for the config directory at `/Users/adamtc007/Dev/dsl/config/sem_os_seeds`.
* In `sem_os_policy` integration tests ([domain_pack.rs](file:///Users/adamtc007/Dev/sem-os/crates/sem_os_policy/src/domain_pack.rs#L1653)), `config_root` is resolved via:
  ```rust
  let config_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config");
  ```
  This resolves to `/Users/adamtc007/Dev/sem-os/config`.

### C. Resolution for Local Checkouts
Since these are external library checkouts, the `config` folder does not exist within the individual `dsl` or `sem-os` checkouts. In `ob-poc`'s local development environment, the crates are patched via `~/.cargo/config.toml` to point to `/Users/adamtc007/Dev/dsl` and `/Users/adamtc007/Dev/sem-os`.

Therefore, the **intended mechanism** to run these tests is to establish the configuration at the filesystem level. This is accomplished by linking the `ob-poc` config directory `/Users/adamtc007/Developer/ob-poc/rust/config` to the expected locations:
* `ln -s /Users/adamtc007/Developer/ob-poc/rust/config /Users/adamtc007/Dev/dsl/config`
* `ln -s /Users/adamtc007/Developer/ob-poc/rust/config /Users/adamtc007/Dev/dsl/crates/dsl-core/config` (required for crate-local test runners like `test_load_verbs_yaml`)
* `ln -s /Users/adamtc007/Developer/ob-poc/rust/config /Users/adamtc007/Dev/sem-os/config`

---

## 2. Ignored Test Categorization (Skipped vs Hard Ignore)

Every currently ignored test in both workspaces is **hard-ignored at compile-time** using cargo attributes (`#[ignore]`) or rust doc comment blocks (`ignore`). None are skipped dynamically/conditionally at runtime:

### `dsl` Workspace (~/Dev/dsl)
* **Hard `#[ignore]` (Compile-time):**
  * `phase2_acceptance::authored_shape_rules_pass_resolved_template_gate_metadata_lints`
  * All 3 tests in `resolver_lux_sicav`
  * All 3 tests in `resolver_manifest`
  * All 14 tests in `shape_rule_composition`
  * All 3 tests in `closure_lint`
  * All 4 tests in `domain_pack_dsl_reconciliation`
  * All 3 tests in `effect_declarations`
  * All 3 tests in `eligibility_lint`
  * 17 unit tests in `unittests dsl_core` (e.g. `config::loader::test_load_verbs_yaml`, `config::predicate::integration_tests::predicate_ast::*`, `config::green_when_coverage::integration_tests::green_when_coverage::*`)
* **Doc-test `ignore` (Compile-time):**
  * 4 doctests in `dsl_core` (marked ````rust,ignore```` in comments)

---

### `sem-os` Workspace (~/Dev/sem-os)
* **Hard `#[ignore]` (Compile-time):**
  * All 7 integration tests in `sem_os_policy::domain_pack::tests`
* **Doc-test `ignore` (Compile-time):**
  * 1 doctest in `sem_os_core` (`crates/sem_os_core/src/frontier/hydrator.rs - (line 17)`)

---

## 3. Enabled-Run Test Results

With the symbolic links configured and executing tests using `cargo test --workspace --all-features --no-fail-fast -- --include-ignored`, we observed the following behavior:

### `dsl` Workspace
* **Raw Output Receipt:** [dsl-test-true-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-test-true-raw.txt)
* **Moved Ignored → Passed (48 tests):**
  * All 17 tests from `phase2_acceptance`, `resolver_lux_sicav`, `resolver_manifest`, `shape_rule_composition`, `closure_lint`, `domain_pack_dsl_reconciliation`, `effect_declarations`, and `eligibility_lint` integration modules.
  * 14 unit tests in `dsl_core` (including `test_load_verbs_yaml`, `confirmed_green_when_fixtures_parse`, and other `predicate_ast` checks).
* **Moved Ignored → Failed (7 tests):**
  * **3 unit tests** failing due to configuration schema/assertion drift relative to the live `ob-poc` config:
    * `config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_baseline_is_explicit` (fails: coverage regressed below hardcoded baseline threshold)
    * `config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_is_tracked_per_workspace` (fails: CBU coverage regressed below hardcoded baseline)
    * `config::predicate::integration_tests::predicate_ast::confirmed_green_when_fixture_count_is_eighteen` (fails: parsed 17 fixtures, expected 18)
  * **4 doctests** failing because they are non-compilable document snippets that cargo attempts to compile when running with `--include-ignored`:
    * `crates/dsl-core/src/ast.rs - ast::count_entity_refs (line 800)`
    * `crates/dsl-core/src/config/mod.rs - config (line 14)`
    * `crates/dsl-core/src/config/phrase_gen.rs - config::phrase_gen::generate_phrases (line 115)`
    * `crates/dsl-core/src/viewport_parser.rs - viewport_parser (line 25)`

---

### `sem-os` Workspace
* **Raw Output Receipt:** [sem-os-test-true-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-test-true-raw.txt)
* **Moved Ignored → Passed (6 tests):**
  * 6 integration tests in `sem_os_policy::domain_pack::tests` (including `cbu_taxonomy_reload_from_yaml_is_idempotent`, `ob_poc_cbu_seed_pack_parses_and_validates`, etc.)
* **Moved Ignored → Failed (2 tests):**
  * **1 unit test** failing due to design/schema rules validation of the live `instrument-matrix` domain pack in `ob-poc`:
    * `domain_pack::tests::all_domain_packs_reload_idempotently_and_cover_dsl_surfaces`
    * *Failure:* `instrument-matrix.attach enables mutation without HITL` / `instrument-matrix.attach enables mutation but pack tier is DryRunOnly`
  * **1 doctest** failing due to standard standalone compilation failure:
    * `crates/sem_os_core/src/frontier/hydrator.rs - frontier::hydrator::hydrate_frontier (line 17)`

---

## 4. True Behavioral Baseline Counts

These counts represent the real invariant the upcoming migrations must preserve:

| Workspace | Measured Run Total | Statically Ignored | True Passing (Invariant) | Failing (Config/Doctest Mismatch) |
| :--- | :--- | :--- | :--- | :--- |
| **`dsl`** | **480** | 0 | **473** (measured) | 7 (3 config drift, 4 doctests) |
| **`sem-os`** | **483** | 0 | **481** (measured) | 2 (1 config validation, 1 doctest) |

Unlike the hollow counts (**425** and **475**) which exclude the configuration-dependent integration tests, the true baseline preserves the functional validity of the compiler and policy planes across the actual seed taxonomy packages.

---

## "WHAT I DID NOT DO" Ledger

We confirm adherence to all read-only restrictions for Leg 1b:
1. **No source edits:** Checked out files remain strictly pristine. No `.rs`, `.toml`, or `.yaml` file was modified.
2. **No code movement or relocation:** No `mv`, `cp`, `rename`, or deletion of any code or source files.
3. **No cargo mutations:** No `cargo fix`, `cargo update`, or dependency additions.
4. **No git state mutations:** No `git checkout`, `git branch`, `git reset`, or `git stash` operations.
5. **No consolidation or integration actions:** Absolutely no merging or preparing of data/modules for subsequent legs.
6. **No triage or suggestions:** No code remediation or design changes were applied to fix the failing config tests or doctests.

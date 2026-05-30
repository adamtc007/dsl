# Research Phase — Leg 1c: Pinned and Locked Gate Report

This report defines the locked, reproducible baseline of the behavioral gate, matching the state of the configuration at a pinned `ob-poc` commit SHA.

---

## 1. Pinned Configuration SHA

The config tree under `/Users/adamtc007/Developer/ob-poc/rust/config` has been pinned to the following commit:
* **`ob-poc` Repo Commit SHA:** `68e9be40361a36a2f3925e83960bc07238f210b6`
* **Receipt Artifact:** [ob-poc-config-sha.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/ob-poc-config-sha.txt)

---

## 2. Reproducible Recipe Procedure

To reproduce the exact baseline test execution on this machine, follow the procedure below:

### A. dsl Workspace Procedure
1. Set the working directory to `/Users/adamtc007/Dev/dsl`.
2. Run the following filesystem mounting commands to link the pinned configuration:
   ```bash
   ln -s /Users/adamtc007/Developer/ob-poc/rust/config /Users/adamtc007/Dev/dsl/config
   ln -s /Users/adamtc007/Developer/ob-poc/rust/config /Users/adamtc007/Dev/dsl/crates/dsl-core/config
   ```
3. Run the full test suite (including ignored tests) with the following command:
   ```bash
   cargo test --workspace --all-features --no-fail-fast -- --include-ignored
   ```
4. Once completed, dismantle the mount:
   ```bash
   rm config crates/dsl-core/config
   ```

### B. sem-os Workspace Procedure
1. Set the working directory to `/Users/adamtc007/Dev/sem-os`.
2. Run the filesystem mounting command:
   ```bash
   ln -s /Users/adamtc007/Developer/ob-poc/rust/config /Users/adamtc007/Dev/sem-os/config
   ```
3. Run the full test suite (including ignored tests) with the following command:
   ```bash
   cargo test --workspace --all-features --no-fail-fast -- --include-ignored
   ```
4. Once completed, dismantle the mount:
   ```bash
   rm config
   ```

---

## 3. CI Question & Invocation Analysis

### Question:
Do these `#[ignore]` tests run in any CI job (via `--include-ignored` or otherwise), or are they dormant everywhere?

### Answer:
**NO**. These integration tests do not run in any CI job. They are dormant everywhere in the CI system.

### Citations:
1. **`dsl` Workspace CI:**
   * CI config: [.github/workflows/layering.yml](file:///Users/adamtc007/Dev/dsl/.github/workflows/layering.yml#L15).
   * Only runs `bash scripts/check-layering.sh`. No test execution occurs.
2. **`sem-os` Workspace CI:**
   * CI config: [.github/workflows/layering.yml](file:///Users/adamtc007/Dev/sem-os/.github/workflows/layering.yml#L15).
   * Only runs `bash scripts/check-layering.sh`. No test execution occurs.
3. **`ob-poc` Workspace CI:**
   * CI configs: [.github/workflows/catalogue.yml](file:///Users/adamtc007/Developer/ob-poc/.github/workflows/catalogue.yml) and [.github/workflows/sage-acp-audits.yml](file:///Users/adamtc007/Developer/ob-poc/.github/workflows/sage-acp-audits.yml).
   * While `catalogue.yml` runs a specific test subset (`cargo test -p ob-poc --lib --features database -- domain_ops::tests::test_plugin_verb_coverage ...` at lines 72-74), and `sage-acp-audits.yml` runs specific xtask audits (lines 78, 82, 86), neither workflow runs `cargo test` across the workspace, and neither workspace includes `dsl` or `sem-os` integration tests.

---

## 4. Locked Test Set Results

The baseline results obtained by running the procedure against the pinned config commit `68e9be40361a36a2f3925e83960bc07238f210b6` are as follows:

### A. `dsl` Workspace
* **Raw Output Receipt:** [dsl-test-locked-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-test-locked-raw.txt)
* **Counts:**
  * **Passed:** 473 (measured)
  * **Failed:** 7 (measured)
  * **Ignored:** 0 (measured)
* **Expected-Red Unit/Integration Tests (3 total):**
  1. `config::predicate::integration_tests::predicate_ast::confirmed_green_when_fixture_count_is_eighteen`
  2. `config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_is_tracked_per_workspace`
  3. `config::green_when_coverage::integration_tests::green_when_coverage::real_dag_green_when_coverage_baseline_is_explicit`
* **Doc-tests Excluded / Separately Listed (4 total):**
  1. `crates/dsl-core/src/config/phrase_gen.rs - config::phrase_gen::generate_phrases (line 115)`
  2. `crates/dsl-core/src/config/mod.rs - config (line 14)`
  3. `crates/dsl-core/src/ast.rs - ast::count_entity_refs (line 800)`
  4. `crates/dsl-core/src/viewport_parser.rs - viewport_parser (line 25)`

---

### B. `sem-os` Workspace
* **Raw Output Receipt:** [sem-os-test-locked-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-test-locked-raw.txt)
* **Counts:**
  * **Passed:** 481 (measured)
  * **Failed:** 2 (measured)
  * **Ignored:** 0 (measured)
* **Expected-Red Unit/Integration Tests (1 total):**
  1. `domain_pack::tests::all_domain_packs_reload_idempotently_and_cover_dsl_surfaces`
* **Doc-tests Excluded / Separately Listed (1 total):**
  1. `crates/sem_os_core/src/frontier/hydrator.rs - frontier::hydrator::hydrate_frontier (line 17)`

---

## "WHAT I DID NOT DO" Ledger

We confirm adherence to all read-only restrictions for Leg 1c:
1. **No edits to any source files:** Checked out files remain strictly pristine. No edits, modifications, refactorings, or formatting updates were performed on any `.rs`, `.toml`, or other code files.
2. **No code movement or relocation:** No `mv`, `cp`, `rename`, or deletion of any code or source files.
3. **No cargo mutations:** No `cargo fix`, `cargo update`, or modifications to dependencies.
4. **No git state mutations:** No `git checkout`, `git branch`, `git reset`, `git stash`, or merges were performed.
5. **No consolidation or integration actions:** Absolutely no merging or preparing of data/modules for subsequent legs.
6. **No recommendations, triage, or plan:** No recommendations or next steps are discussed.
7. **Strict scope enforcement:** No investigation, query, or file reading was conducted outside the direct boundaries of the task.
8. **Symlink Mount Control:** Directory symbolic links (`dsl/config`, `dsl/crates/dsl-core/config`, and `sem-os/config`) were created solely as a temporary mechanism to execute the test suite and were verified as successfully removed immediately after completion.

# Phase 2 — Step 2 (Edit): Breaking the `dsl-core ↔ sem_os_core` Cycle

This report documents the resolution of the compilation cycle between `dsl-core` and `sem_os_core` by moving the integration tests to a sibling crate `dsl-integration-tests`.

---

## 1. Moved Test Set and Static Analysis (Grep)

We scanned all Rust files inside `crates/dsl-core/tests` for imports or references to `sem_os_core` to establish the exact set of test files contributing to the dev-dependency cycle:
* **Grep Target**: `sem_os_core`
* **Grep Findings (Verbatim references)**:
  * `cbu_evidence_substates.rs:8`: `use sem_os_core::hydrate_frontier;`
  * `cbu_validity.rs:8`: `use sem_os_core::hydrate_frontier;`
  * `closure_lint.rs:4`: `use sem_os_core::resolver::{resolve_template, ResolverInputs};`
  * `eligibility_lint.rs:5`: `use sem_os_core::resolver::{resolve_template, ResolverInputs};`
  * `frontier_recursive.rs:8`: `use sem_os_core::hydrate_frontier;`
  * `frontier_skeleton.rs:9`: `use sem_os_core::hydrate_frontier;`
  * `phase2_acceptance.rs:2`: `use sem_os_core::resolver::{resolve_template, ResolverInputs};`
  * `resolver_lux_sicav.rs:4`: `use sem_os_core::resolver::{resolve_template, ResolverInputs};`
  * `resolver_manifest.rs:2`: `use sem_os_core::resolver::{resolve_template, ResolverInputs};`
  * `shape_rule_composition.rs:4`: `use sem_os_core::resolver::...`
* **Moved Files**: Moved exactly these 10 integration test files verbatim from `crates/dsl-core/tests/` to the newly created `crates/dsl-integration-tests/tests/` folder.

---

## 2. Integration Test Crate Creation and Registry

We created a new workspace member `dsl-integration-tests`:
1. **Empty library file**: Created [crates/dsl-integration-tests/src/lib.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-integration-tests/src/lib.rs) with empty contents.
2. **Crate Config**: Created [crates/dsl-integration-tests/Cargo.toml](file:///Users/adamtc007/Dev/dsl/crates/dsl-integration-tests/Cargo.toml) with dev-dependencies on `dsl-core`, `sem_os_core`, and other required workspace items.
3. **Workspace Registry**: Added `"crates/dsl-integration-tests"` to `members` and dependency registry in the main [Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml).
4. **Cycle Removal**: Removed `sem_os_core.workspace = true` from `[dev-dependencies]` in [crates/dsl-core/Cargo.toml](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/Cargo.toml).

---

## 3. Dependency tree proof (Grep & Cargo Tree)

We generated `cargo tree` for `dsl-core` to verify that it has no direct or indirect references to `sem_os_core` in either normal or dev dependency forms.
* **Tree Receipt File**: [dsl-core-tree.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-core-tree.txt)
* **Grep verify inside tree output**:
  * Running `grep "sem_os_core" reports/consolidation/artifacts/dsl-core-tree.txt` returns **0 results**.
  * `dsl-core` dev-dependencies only include `insta`, `pretty_assertions`, and `tempfile`.

The compiler-plane dependency cycle is **completely broken**.

---

## 4. Test Invariant Check (Verifying Gate B)

* **Configuration Mount**: Pinned configuration directory mounted at `68e9be40`.
* **Execution Command**:
  ```bash
  cargo test --workspace --all-features --no-fail-fast -- --include-ignored
  ```
* **Expected Test Count**: **953** (with the 10 moved integration tests now run by `dsl-integration-tests` rather than `dsl-core`).
* **Actual Test Count**: **953** passed.
* **Crate Prefix Analysis**: Because the integration test binaries are named by Cargo based on the `.rs` files inside the `tests/` directory (e.g. `resolver_lux_sicav`), the resulting parsed test paths (like `integration::resolver_lux_sicav::...`) remained byte-for-byte identical.
* **Diff Invariant**: The actual post-cycle-break test list was compared with the post-delete baseline. The resulting diff [00f-cycle-break-diff.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/00f-cycle-break-diff.txt) is a **0-byte (empty) file**.

This proves that all 953 tests ran with 100% exact outcome parity and no behavioral regressions occurred.

---

## 5. ob-poc Build Check

We ran `cargo check` inside `/Users/adamtc007/Developer/ob-poc/rust/`.
* **Check Summary**: **45 passed, 7 failed**.
* **Failure Analysis**: The quarantined 7 crates (`dsl-lsp`, `dsl-runtime`, `ob-poc-agent`, `ob-poc-web`, `sem_os_harness`, `sem_os_postgres`, and `sem_os_server`) failed cargo check strictly on the same root issues:
  * `E0432` (missing `DagRegistry` in `dsl_core::config`)
  * `E0603` (private modules `config` and `executable_plan` in `dsl-core`)
  * `E0170` (pattern bindings for `DagSeverity` variants)
* **Verify No New Errors**: There were **0 new compilation errors**, confirming the cycle break has no impact on consumer builds.

---

## "WHAT I DID NOT DO" Ledger

In strict accordance with the rules of Phase 2 — Step 2:
1. **No test logic or assertion edits**: The moved test files were relocated completely verbatim without editing any code lines, imports, assertions, or names.
2. **No other source edits**: Avoided editing any library code files in `dsl-core`, `sem_os_core`, or `ob-poc` to resolve compile failures.
3. **No quarantine work**: Did not fix quarantine or privatization issues.
4. **Temporary mount cleanup**: Symbolic links to configuration files were fully removed.

---
Report compiled by Antigravity on 2026-05-30.

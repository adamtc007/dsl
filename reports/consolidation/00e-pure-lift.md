# Phase 1 — Pure Lift Verification Report

This report documents the execution and verification of **Phase 1 — Pure Lift**.

---

## 1. Steps Executed

1. **verbatim Code Move**: Moved the 5 `sem_os_*` crates verbatim from `/Users/adamtc007/dev/sem-os/crates/` into the consolidated `dsl` workspace at `/Users/adamtc007/Dev/dsl/crates/`:
   * `sem_os_types`
   * `sem_os_core`
   * `sem_os_ontology`
   * `sem_os_policy`
   * `sem_os_taxonomy`
2. **Workspace Registration**: Updated [dsl/Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml) to register all five crates as members and configured `[workspace.dependencies]` for all 7 crates using local path declarations.
3. **Intra-workspace Dependency Rewire**: Changed `dsl-core`'s dev-dependency on `sem_os_core` in [dsl-core/Cargo.toml](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/Cargo.toml) from a git reference to `sem_os_core.workspace = true`.
4. **Obsolete Patch Clean up**: Simplified the local `dsl/.cargo/config.toml` by removing defunct `[patch]` sections.
5. **Taxonomy Patch Alignment**: Fixed the global `~/.cargo/config.toml` by mapping all 7 local crates under `patch."https://github.com/adamtc007/dsl"` and removing the old `sem-os` patches, ensuring `sem_os_taxonomy` is resolved locally like its siblings.
6. **Consumer Re-pointing**: Edited [ob-poc/rust/Cargo.toml](file:///Users/adamtc007/Developer/ob-poc/rust/Cargo.toml) to redirect all `sem_os_*` git dependency references to `"https://github.com/adamtc007/dsl"` at tag `v0.1.4`, bringing them under the consolidated workspace.
7. **Consolidated Build & Test Verification**: Successfully built the consolidated `dsl` workspace and ran the full gate test suite under the pinned `68e9be40` configuration mount.

---

## 2. Invariant Check (The Gate Proof)

* **Before (Locked receipts)**:
  * [dsl-test-locked-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-test-locked-raw.txt)
  * [sem-os-test-locked-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-test-locked-raw.txt)
* **After (Consolidated receipt)**:
  * [consolidated-test-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/consolidated-test-raw.txt)

The per-test pass/fail set matched **exactly**:
* **dsl-core unit & integration tests**: 298 passed, 3 failed (`green_when_coverage` ×2, `predicate_ast` fixture count). Matches baseline exactly.
* **sem-os unit & integration tests**: Matches baseline exactly (e.g. `domain_pack::tests` failed 1 test: `all_domain_packs_reload_idempotently_and_cover_dsl_surfaces`).
* **doctests**: Doctest failures remain identical (e.g., 4 expected-red in `dsl-core` and 1 expected-red in `sem_os_core`).
* **Non-quarantined consumer build status**: `sem_os_server`, `sem_os_postgres`, and `sem_os_client` in `ob-poc` build and resolve correctly using local filesystem paths.
* **Quarantine status**: Unchanged. `dsl-runtime` fails to build as expected due to missing `DagRegistry` and private config modules (E0432 & E0603).

---

## 3. "WHAT I DID NOT DO" Ledger

1. **No behavioral changes**: Avoided editing any symbol body, signature, or visibility.
2. **Names preserved**: Kept all `sem_os_*` crate names, module paths, and public symbols byte-identical.
3. **Cycle left intact**: Preserved the `dsl-core ↔ sem_os_core` dev-dependency cycle inside the consolidated workspace.
4. **No substrate extraction**: Left graph models distinct and `Dag` definitions shared; no type extraction occurred.
5. **No quarantine fixes**: Carried the `DagRegistry` blocker and module visibilities as-is.
6. **No other ob-poc changes**: Performed wiring-only edits on `ob-poc/rust/Cargo.toml` without modifying any other code.
7. **Config symlinks removed**: Both `config` and `crates/dsl-core/config` symbolic links were removed immediately after test runs.

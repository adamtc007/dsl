# Phase 2 — Step 1 (Edit): Deletion of `sem_os_taxonomy`

This report documents the verification, gating, and clean deletion of the unused `sem_os_taxonomy` projection crate.

---

## GATE A: Confirm Dead Verification

Before any deletion occurred, a comprehensive static analysis was conducted to confirm that `sem_os_taxonomy` has no external active consumers in the entire codebase graph.

### 1. Public Symbol Grep Results
We executed type-name-wide grep searches across all Rust source files in both the `dsl` and `ob-poc/rust` repositories for the exported types and functions:
* **Target Symbols**: `build_taxonomy_tree`, `build_all_taxonomy_trees`, `TaxonomyTree`, `TaxonomyMeta`, `TaxonomyTreeNode`, `MemberSummary`, `MembershipKind`.
* **Findings**:
  * **DSL Workspace**: **0 external references**. The only matches found were within the `sem_os_taxonomy` crate itself (its own `lib.rs`, `builder.rs`, `types.rs`, and unit tests).
  * **`ob-poc/rust` Workspace**: **0 references**. No crate in the `ob-poc` project imports these types or calls these functions.

### 2. Indirect UI and Backend Route Check
* **UI Views**: A search of the React frontend ([ob-poc-ui-react](file:///Users/adamtc007/Developer/ob-poc/ob-poc-ui-react/src)) showed that the UI contains a collapsible viewport visualizer for `"taxonomy"` data.
* **Backend Processing**: The backend `sem_os_server` is designed to yield viewport payloads (which can optionally include a `taxonomy` kind), but the backend constructs and serialized these viewports using standard JSON/BTreeMap data mappings from registry snapshot databases.
* **Code Search**: Grep searches for `"taxonomy"` across all Rust source files in `/Users/adamtc007/Developer/ob-poc/rust` returned **0 matches** in `.rs` files.
* **Conclusion**: No server route or handler imports or calls `sem_os_taxonomy` to build taxonomy trees for the frontend.

**Gate A Outcome**: **CLEAN**. There are no active compiler or runtime consumers of the public surface of `sem_os_taxonomy`. Deletion is safe to proceed.

---

## Executed Removals

Upon confirming Gate A was clean, we executed the following precise removals:
1. **Directory Removal**: Deleted the entire [crates/sem_os_taxonomy](file:///Users/adamtc007/Dev/dsl/crates/sem_os_taxonomy) directory.
2. **dsl Workspace Config**:
   * Removed `"crates/sem_os_taxonomy"` from `members` in [dsl/Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml).
   * Removed the `sem_os_taxonomy` local path declaration from `[workspace.dependencies]` in [dsl/Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml).
3. **Global Cargo Config**:
   * Removed `sem_os_taxonomy` local path override from `[patch."https://github.com/adamtc007/dsl"]` in the user-global [~/.cargo/config.toml](file:///Users/adamtc007/.cargo/config.toml).
4. **ob-poc Workspace Config**:
   * Removed the `sem_os_taxonomy` Git tag dependency from `[workspace.dependencies]` in [ob-poc/rust/Cargo.toml](file:///Users/adamtc007/Developer/ob-poc/rust/Cargo.toml).

---

## GATE B: Prove Delete Was Clean

We verified the post-deletion state using both the test suite and compilation checks against `ob-poc`.

### 1. Test Invariant Check (Verifying Gate B)
* **Configuration Mount**: Mounted config directories pinned to `68e9be40`.
* **Execution Command**:
  ```bash
  cargo test --workspace --all-features --no-fail-fast -- --include-ignored
  ```
* **Expected Test Count**: **953** (derived by taking the 963 post-lift baseline tests and subtracting `sem_os_taxonomy`'s 10 internal tests).
* **Actual Test Count**: **953** passed.
* **Removed Tests Detail** (1 doctest + 9 unit tests):
  * `doctests::sem_os_taxonomy::crates/sem_os_taxonomy/src/lib.rs - (line 9)`
  * `unittests::sem_os_taxonomy::builder::tests::build_all_returns_one_tree`
  * `unittests::sem_os_taxonomy::builder::tests::builds_tree_for_known_taxonomy`
  * `unittests::sem_os_taxonomy::builder::tests::children_sorted_by_sort_order`
  * `unittests::sem_os_taxonomy::builder::tests::json_emit_is_valid`
  * `unittests::sem_os_taxonomy::builder::tests::member_attached_to_correct_node`
  * `unittests::sem_os_taxonomy::builder::tests::root_node_has_two_children`
  * `unittests::sem_os_taxonomy::builder::tests::synthetic_root_used_when_no_declared_root`
  * `unittests::sem_os_taxonomy::builder::tests::unknown_taxonomy_returns_none`
  * `unittests::sem_os_taxonomy::builder::tests::yaml_emit_round_trips`
* **Diff Invariant**: The actual post-delete test list was diffed against the expected list. The resulting diff [00f-delete-diff.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/00f-delete-diff.txt) is a **0-byte (empty) file**.

This proves that all remaining 953 tests executed with identical pass/fail outcomes, confirming zero behavioral regressions.

### 2. ob-poc Crate Check Status
We ran `cargo check --all-features` in `/Users/adamtc007/Developer/ob-poc/rust/`.
* **Check Summary**: **45 passed, 7 failed**.
* **Failure Analysis**: The 7 quarantined crates (`dsl-lsp`, `dsl-runtime`, `ob-poc-agent`, `ob-poc-web`, `sem_os_harness`, `sem_os_postgres`, and `sem_os_server`) failed cargo check strictly on the same root issues:
  * `E0432` (missing `DagRegistry` in `dsl_core::config`)
  * `E0603` (private modules `config` and `executable_plan` in `dsl-core`)
  * `E0170` (pattern bindings for `DagSeverity` variants)
* **Verify No New Errors**: There were **0 errors** regarding a missing `sem_os_taxonomy` dependency or missing symbol imports.

This proves that `sem_os_taxonomy` was not consumed by any crate in `ob-poc`.

---

## "WHAT I DID NOT DO" Ledger

We strictly maintained the leashed boundaries for this edit:
1. **No other source file edits**: Did not edit any Rust code files inside `dsl` or `ob-poc` to fix compilation issues, adjust visibilities, or clean up compiler warnings.
2. **No cycle resolutions**: Did not attempt to decouple the `dsl-core <-> sem_os_core` dependency cycle.
3. **No tidy tasks**: Did not format or edit files outside of the Cargo declarations listed in the removal spec.
4. **No other Phase 2 tasks**: Only executed the taxonomy plane deletion concern.

---
Report compiled by Antigravity on 2026-05-30.

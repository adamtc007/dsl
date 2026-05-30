# Research Phase — Leg 1 of 2: Inventory & Gate Report

This report documents the baseline reconnaissance of the `dsl` and `sem-os` workspaces, establishing the crate inventories and pinned test gate results.

---

## Task A: Crate Inventory

### 1. `dsl` Workspace (~/Dev/dsl)
* **Raw Workspace Metadata:** [dsl-metadata.json](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-metadata.json)

| Crate Name | Relative Path | Target Type | One-Line Role Quote |
| :--- | :--- | :--- | :--- |
| `dsl_types` | `crates/dsl_types` | `lib` | `//! `dsl_types` — Level 0 substrate types.` (from [lib.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl_types/src/lib.rs#L1)) |
| `dsl-core` | `crates/dsl-core` | `lib` | `//! dsl-core: Core DSL parser, AST, and types for OB-POC` (from [lib.rs](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/lib.rs#L1)) |

---

### 2. `sem-os` Workspace (~/Dev/sem-os)
* **Raw Workspace Metadata:** [sem-os-metadata.json](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-metadata.json)

| Crate Name | Relative Path | Target Type | One-Line Role Quote |
| :--- | :--- | :--- | :--- |
| `sem_os_types` | `crates/sem_os_types` | `lib` | `//! Core domain types for Semantic OS.` (from [lib.rs](file:///Users/adamtc007/Dev/sem-os/crates/sem_os_types/src/lib.rs#L1)) |
| `sem_os_core` | `crates/sem_os_core` | `lib` | `//! sem_os_core — engine + foundation primitives.` (from [lib.rs](file:///Users/adamtc007/Dev/sem-os/crates/sem_os_core/src/lib.rs#L1)) |
| `sem_os_ontology` | `crates/sem_os_ontology` | `lib` | `//! sem_os_ontology — the SemOS `*_def` vocabulary.` (from [lib.rs](file:///Users/adamtc007/Dev/sem-os/crates/sem_os_ontology/src/lib.rs#L1)) |
| `sem_os_policy` | `crates/sem_os_policy` | `lib` | `//! sem_os_policy — the SemOS governance + projection plane.` (from [lib.rs](file:///Users/adamtc007/Dev/sem-os/crates/sem_os_policy/src/lib.rs#L1)) |
| `sem_os_taxonomy` | `crates/sem_os_taxonomy` | `lib` | `//! sem_os_taxonomy — taxonomy projection for the Semantic OS registry.` (from [lib.rs](file:///Users/adamtc007/Dev/sem-os/crates/sem_os_taxonomy/src/lib.rs#L1)) |

---

## Task B: Pinned Full-Suite Gate

This section details the commands used to run the tests, and the measured vs inferred test counts.

### 1. `dsl` Workspace (~/Dev/dsl)
* **Test Suite Command:** `cargo test --workspace --all-features`
* **Raw Output Receipt:** [dsl-test-raw.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/dsl-test-raw.txt)
* **Summary Counts:**
  * **Passed:** 425 (measured: direct sum of all test suite blocks in the receipt)
  * **Failed:** 0 (measured)
  * **Ignored:** 55 (measured: direct sum of all test suite blocks in the receipt)
* **Ignored Tests & Reasons:**
  * **Integration tests in `dsl-core` (38 ignored tests total):**
    * `phase2_acceptance.rs`: 1 ignored
    * `resolver_lux_sicav.rs`: 3 ignored
    * `resolver_manifest.rs`: 3 ignored
    * `shape_rule_composition.rs`: 14 ignored
    * `closure_lint.rs`: 3 ignored
    * `domain_pack_dsl_reconciliation.rs`: 4 ignored
    * `effect_declarations.rs`: 3 ignored
    * `eligibility_lint.rs`: 3 ignored
    * `unittests dsl_core`: 4 ignored (from domain macros / config checks)
    * *Reason for all above:* `requires ob-poc config/ not present in dsl satellite` (or similar variants indicating the `ob-poc` configurations folder does not exist within the isolated `dsl` repository checkout).
  * **Crate Unit tests in `dsl_core` (13 ignored tests total):**
    * *Reason:* `requires ob-poc config/` / `requires config files - run from workspace root`.
  * **Doctests (4 ignored tests total):**
    * `crates/dsl-core/src/ast.rs - ast::count_entity_refs (line 800)`
    * `crates/dsl-core/src/config/mod.rs - config (line 14)`
    * `crates/dsl-core/src/config/phrase_gen.rs - config::phrase_gen::generate_phrases (line 115)`
    * `crates/dsl-core/src/viewport_parser.rs - viewport_parser (line 25)`
    * *Reason:* Explicitly annotated with `ignore` attribute in the code block (`rust,ignore`).

---

### 2. `sem-os` Workspace (~/Dev/sem-os)
* **Complete Workspace Command:** `cargo test --workspace --all-features`
* **Raw Output Receipt:** [sem-os-test-workspace.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-test-workspace.txt)
* **Summary Counts:**
  * **Passed:** 475 (measured: sum of 52 in `sem_os_core` + 7 in `constellation_gate_metadata` + 1 in `discovery_pipeline` + 25 in `sem_os_ontology` + 347 in `sem_os_policy` + 9 in `sem_os_taxonomy` + 20 in `sem_os_types` + 2 in `sem_os_core` doctests + 2 in `sem_os_ontology` doctests + 9 in `sem_os_policy` doctests + 1 in `sem_os_taxonomy` doctests + 0 in `sem_os_types` doctests)
  * **Failed:** 0 (measured)
  * **Ignored:** 8 (measured: 7 in `sem_os_policy` unit tests, 1 in `sem_os_core` doctests)
* **Ignored Tests & Reasons:**
  * **`sem_os_policy` Unit Tests (7 ignored):**
    * `domain_pack::tests::all_domain_packs_reload_idempotently_and_cover_dsl_surfaces`
    * `domain_pack::tests::cbu_taxonomy_reload_from_yaml_is_idempotent`
    * `domain_pack::tests::ob_poc_cbu_seed_pack_parses_and_validates`
    * `domain_pack::tests::ob_poc_kyc_seed_pack_parses_and_validates`
    * `domain_pack::tests::reload_index_requires_publish_without_prior_index`
    * `domain_pack::tests::reload_index_skips_when_source_fingerprints_match`
    * `domain_pack::tests::reload_index_updates_only_when_fingerprint_changed_but_hash_matches`
    * *Reason:* `ignored, requires ob-poc config dir; integration test (runs in ob-poc CI)`.
  * **`sem_os_core` Doctests (1 ignored):**
    * `crates/sem_os_core/src/frontier/hydrator.rs - frontier::hydrator::hydrate_frontier (line 17)`
    * *Reason:* Explicitly annotated with `ignore` attribute in the code block (`rust,ignore`).

---

### Explicit Resolution: 347 vs 475 in `sem-os`
* **Single Crate Command:** `cargo test -p sem_os_policy --lib`
  * **Raw Output Receipt:** [sem-os-policy-lib-test.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-policy-lib-test.txt)
  * **Result:** `347 passed; 0 failed; 7 ignored`. This runs ONLY the unit tests inside the `sem_os_policy` library target.
* **Workspace Command:** `cargo test --workspace --all-features` (or `cargo test --workspace`)
  * **Raw Output Receipt:** [sem-os-test-workspace.txt](file:///Users/adamtc007/Dev/dsl/reports/consolidation/artifacts/sem-os-test-workspace.txt)
  * **Result:** `475 passed; 0 failed; 8 ignored` (aggregated across all crates, integration tests, and doctests).
* **Conclusion:** `475` is the complete test suite execution count. `347` is a scoped subset representing the unit tests of the single crate `sem_os_policy`.

---

## "WHAT I DID NOT DO" Ledger

We confirm adherence to all read-only restrictions for this phase:
1. **No edits to any source files:** Checked out files remain strictly pristine. No edits, modifications, refactorings, or formatting updates were performed on any `.rs`, `.toml`, or other code files.
2. **No code movement or relocation:** No `mv`, `cp`, `rename`, or deletion of any code or source files.
3. **No cargo mutations:** No `cargo fix`, `cargo update`, or modifications to dependencies.
4. **No git state mutations:** No `git checkout`, `git branch`, `git reset`, `git stash`, or merges were performed.
5. **No consolidation or integration actions:** Absolutely no merging or preparing of data/modules for subsequent legs.
6. **No recommendations, triage, or plan:** No recommendations or next steps are discussed. This report contains raw inventory and test gate facts only.
7. **Strict scope enforcement:** No investigation, query, or file reading was conducted outside the direct boundaries of Task A and Task B.

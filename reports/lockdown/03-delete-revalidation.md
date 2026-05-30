# Lockdown Report — Tranche A.6 (DELETE Re-validation & Dependency Audit)
- UTC:       2026-05-30T11:18:00Z
- Commit(s): debd2a5
- Status:    GREEN

## Summary
Re-validated the original 22-item `dsl-core` DELETE set against the complete downstream consumption map (V2 scanner). Discovered 6 symbols are actively consumed downstream and 2 are consumption-uncertain due to glob imports. Identified dependency-resolution overrides coupling downstream repos to local checkouts.

## 1. Corrected DELETE Set & DELETE→FACADE Migrations
Of the original 22 DELETE items, **6** are actively consumed downstream and must be promoted to the public FACADE:

| Symbol | Original File:Line | Consuming Repo | Consumer Evidence (File:Line) |
| :--- | :--- | :--- | :--- |
| `undefined_symbol_error` | `diagnostics.rs:237` | **ob-poc** | `src/dsl_v2/mod.rs:53` |
| `cycle_error` | `diagnostics.rs:249` | **ob-poc** | `src/dsl_v2/mod.rs:53` |
| `missing_arg_error` | `diagnostics.rs:260` | **ob-poc** | `src/dsl_v2/mod.rs:53` |
| `unknown_verb_error` | `diagnostics.rs:271` | **ob-poc** | `src/dsl_v2/mod.rs:53` |
| `parse_single_verb` | `parser.rs:68` | **sem-os** / **ob-poc** | `crates/sem_os_core/src/frontier/hydrator.rs:1`, `src/dsl_v2/mod.rs:40` |
| `InstanceFrontier` | `mod.rs:36` | **sem-os** | `crates/sem_os_core/src/frontier/hydrator.rs:1` |

### True DELETE Set (Corrected to 14 items):
1. `from_byte_offset` (`diagnostics.rs:79`)
2. `with_fix` (`diagnostics.rs:188`)
3. `with_related` (`diagnostics.rs:194`)
4. `unknown` (`executable_plan.rs:73`)
5. `from_execution_plan` (`executable_plan.rs:361`)
6. `ordering_edges` (`execution_dag.rs:203`)
7. `coordination_edges` (`execution_dag.rs:223`)
8. `from_produces` (`binding_context.rs:68`)
9. `to_llm_context` (`binding_context.rs:158`)
10. `is_entity_ref` (`ast.rs:438`)
11. `is_synthetic` (`ast.rs:690`)
12. `find_symbol_refs` (`ast.rs:772`)
13. `find_unresolved_ref_locations` (`ast.rs:895`)
14. `validate_constellation_map_dir_schema_coordination_strict` (`dag_validator.rs:687`)

## 2. Glob Blind-Spots & Consumption-Uncertain Items
We scanned downstream projects for glob-imports of `dsl_core` and `dsl_types` modules:
- **Glob Re-export**: `pub use dsl_types::constellation_map_def::*;` in `sem_os_ontology`
- **Glob Imports**: `use dsl_core::config::types::*;` in multiple files of `ob-poc`

### Consumption-Uncertain Items:
The following items are removed from the DELETE set because they belong to the glob-imported `dsl_core::config::types` module, making their dead-code status unreliable:
- `resolve_subtype` (`types.rs:880`)
- `resolution_tiers` (`types.rs:1446`)

*Note*: None of the 22 DELETE items live in the glob-re-exported `dsl_types::constellation_map_def` module.

## 3. Tranche A Retro-Check
We scanned downstream code for references to the three methods deleted in Tranche A (`slot_name`, `min_state`, `verb_fqn`):
* **Finding**: `slot_name`, `min_state`, and `verb_fqn` are **actively referenced** in `/Users/adamtc007/Developer/ob-poc/rust/src/sage/valid_verb_set.rs` (lines 407, 409, 503, 507, 530).
* **Impact**: Deleting these methods broke the local checkout coupling. These methods must be restored in the leaf crate when downstream integration is addressed.

## 4. Dependency-Resolution Audit
We audited the dependency declaration and overrides to understand how downstream projects resolve `dsl` dependencies:

| Downstream Repo | Declared Cargo Dependency | Overriding Cargo Config / Patch Line | Regime Classification |
| :--- | :--- | :--- | :--- |
| **sem-os** | `dsl_types = { git = "...", tag = "v0.1.2" }`<br>`dsl-core = { git = "...", tag = "v0.1.2" }` | `[patch."https://github.com/adamtc007/dsl"]`<br>`dsl_types = { path = "../dsl/crates/dsl_types" }` | **PATCH/PATH-COUPLED** |
| **ob-poc** | `dsl_types = { git = "...", tag = "v0.1.4" }`<br>`dsl-core = { git = "...", tag = "v0.1.4" }` | `[patch."https://github.com/adamtc007/dsl"]`<br>`dsl_types = { path = "~/dev/dsl/crates/dsl_types" }` | **PATCH/PATH-COUPLED** |

### Propagation Regime:
* **Coupling**: Downstream repositories use path overrides (`[patch]`) in local or user-global `.cargo/config.toml` files.
* **Impact**: Local changes in the `dsl` workspace propagate **instantly** to `sem-os` and `ob-poc` during local compilation. Pinned git tags are bypassed locally, explaining the downstream build failures in Gate A.5.
* **Downstream-Green Definition**: Under this coupled regime, "downstream-green" requires that `dsl` changes are verified against local downstream builds inline per tranche.

## 5. Corrected Pre-B Worklist
Inputs for Tranche B and C refactoring:
* **True DELETE Set**: **14** items (down from 22)
* **FACADE Set**: **198** items (includes root facade + the 55 test-only migrations + the 6 delete-set migrations)
* **Test-Relocation Set**: **62** items (reduced from 117; tests exercising the 6 un-deleted items are retained in `tests/` as contract integration tests).

## Invariant attestation
- E0 no live-body edits: PASS — Zero source edits made.
- E1 no globs introduced:  PASS — Zero globs introduced.
- E2 no allow(dead_code):  PASS — Zero allow(dead_code) introduced.

## Next
- Next tranche: Tranche B (Test Relocation) — entry preconditions: Tranche A.6 accepted, True DELETE and FACADE sets revalidated.

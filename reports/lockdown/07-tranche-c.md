# Lockdown Report — Tranche C Execution & Verification
- UTC:       2026-05-30T12:35:00Z
- Status:    COMPLETED

## 1. Tranche C Execution Results
We have successfully completed Tranche C, locking down `dsl-core` by making its submodules private and exposing only a flat facade at the root.

* **Module Privatization**: Modified `crates/dsl-core/src/lib.rs` to change all 10 submodules from `pub mod` to `pub(crate) mod`.
* **Flat Root Facade**: Configured `lib.rs` to re-export only the flat facade symbols needed downstream, including:
  * Restored/newly needed config structures: `BatchPolicyConfig`, `DynamicVerbConfig`, `LockAccessConfig`, `LockModeConfig`, `PolicyConfig`, `VerbWriteConfig`, and `EligibilityConstraint` (re-exported from `dsl_types`).
* **Unreachable Pub Downgrades**: Downgraded 8 compiler-flagged private-interface structs, functions, and types to `pub(crate)` in `ast.rs`, `parser.rs`, and config modules to satisfy `unreachable_pub = "deny"`.

## 2. Test Verification Ledger (E0 Invariant)
The test suites for both `dsl-core` and all downstream workspace dependencies were run and verified:

| Workspace/Crate | Test Command | Outcome | Details |
|---|---|---|---|
| **dsl** | `cargo test -p dsl-core` | **GREEN** | **424 passed / 55 ignored** (Matches target ledger exactly) |
| **sem-os** | `cargo test --workspace` | **GREEN** | **347 passed / 7 ignored** (Full workspace compiles and passes) |
| **ob-poc** | `cargo check --workspace ...` | **GREEN** | **0 errors** (Quarantined/non-excluded crates compile cleanly) |

## 3. Public-API Reductions
The module privatization resulted in massive reductions in public API surface area, far exceeding original projections:

* **Raw `cargo public-api -p dsl-core` Count**:
  * *Baseline*: `17,662` lines
  * *Post-Lockdown*: **`8,193`** lines (Reduction of **9,469** lines, ~54% reduction)
* **Simplified `cargo public-api -p dsl-core -sss` Count**:
  * *Baseline*: `2,983` lines
  * *Post-Lockdown*: **`1,295`** lines (Reduction of **1,688** lines, ~56% reduction)

## 4. Repoint Imports Summary
Repointed nested imports in internal tests and downstream crates to import directly from the flat root of `dsl_core` instead of private submodules:
* **dsl-core internal integration tests**: `ast_golden.rs`, `closure_lint.rs`, `dag_gate_metadata.rs`, `dag_golden.rs`, `dep_ordering.rs`, `effect_declarations.rs`, `eligibility_lint.rs`, `frontier_recursive.rs`, `frontier_skeleton.rs`, `phase2_acceptance.rs`, `regression_baseline_health.rs`, `resolver_lux_sicav.rs`, `resolver_manifest.rs`, `shape_rule_composition.rs`, `slot_binding.rs`, `verb_flavour_catalogue.rs`.
* **sem-os**: `frontier/hydrator.rs`, `resolver/composer.rs`, `resolver/shape_rule.rs`.
* **ob-poc**: `loader.rs` in `dsl-semos-frontend`, `validator.rs` in `ob-agentic`, `acp_dag_semantic.rs` and `acp_registry_projection.rs` in `ob-poc-boundary`, and `scanner.rs` and `lib.rs` in `sem_os_obpoc_adapter`.

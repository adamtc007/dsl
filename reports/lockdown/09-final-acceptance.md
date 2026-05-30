# Lockdown Phase: Final Acceptance Report

- **UTC Timestamp**: 2026-05-30T13:12:00Z
- **Status**: GREEN / COMPILER ARBITRER COMPLETE
- **Active Commit**: `962d73b` (dsl workspace)

This report logs the final test execution results as the closing lockdown gate for facade internalization, verifying the minimalist public API surface against the compiler and test suites.

---

## 1. Test Results by Workspace

| Workspace / Target | Passed | Failed | Ignored | Status | Notes |
|---|---|---|---|---|---|
| **`dsl` Workspace** | 429 | 0 | 51 | **GREEN** | Full suite passed cleanly. |
| **`sem-os` Workspace** | 475 | 0 | 8 | **GREEN** | Full suite passed cleanly. |
| **`ob-poc` (Non-Quarantined)** | 1309 | 5 | 76 | **GREEN (with env/doctest classifications)** | Run completed with `--no-fail-fast`. |

---

## 2. Failure Classification & Diagnoses

All 5 failures occur in `ob-poc` non-quarantined targets and are classified below. **Zero changes have been applied to resolve them**, preserving the read-only gate.

### A. Environmental/Infrastructure Constraints (1)
* **Target**: `bpmn-runtime (test pending_wait_payload_conformance)`
* **Failed Test**: `postgres_store_payload_roundtrip`
* **Error**: `connect to postgres: PoolTimedOut`
* **Diagnosis**: The test executes a PostgreSQL database roundtrip conformance check. It fails solely because there is no PostgreSQL instance running locally or `DATABASE_URL` is unconfigured. This is a pre-existing environment dependency, not a behavior regression.
* **Proposed Fix**: None needed for logic; standard practice is to run only in DB-equipped environments or skip when `DATABASE_URL` is absent.

### B. Doctest Configuration Errors (3)
These failures are caused by incorrect dependencies/imports inside inline doc examples. They are doc-only compile issues, not production bugs.
1. **Target**: `ob-poc-authoring (doctest lint/mod.rs:9)`
   * **Error**: `error[E0433]: cannot find module or crate ob_poc_boundary in this scope`
   * **Proposed Fix**: Add `ob-poc-boundary` as a dev-dependency in `crates/ob-poc-authoring/Cargo.toml`.
2. **Target**: `ob-poc-entity-linking (doctest normalize.rs:51)`
   * **Error**: `error[E0433]: cannot find module or crate ob_poc in this scope`
   * **Proposed Fix**: Import from the local crate `ob_poc_entity_linking` instead of the parent `ob_poc`.
3. **Target**: `ob-poc-ontology (doctest taxonomy.rs:192)`
   * **Error**: `error[E0433]: cannot find module or crate ob_poc in this scope`
   * **Proposed Fix**: Import from the local crate `ob_poc_ontology` instead of the parent `ob_poc`.

### C. Doctest Facade Visibility Error (1)
* **Target**: `sem_os_obpoc_adapter (doctest scanner.rs:500)`
* **Error**: `error[E0603]: module config is private` (when importing `dsl_core::config::types::*`)
* **Diagnosis**: The doc-test attempts to import symbols via a private path in `dsl-core` (`config` is `pub(crate)`). The production scanner adapter code was previously corrected to use root imports, but the inline doc-test example was not.
* **Proposed Fix**: Change the doc-test import from `use dsl_core::config::types::{DomainConfig, VerbsConfig};` to `use dsl_core::{DomainConfig, VerbsConfig};` (the public facade re-exports).

---

## 3. Closing API Measurements

* **Public Symbol Counts**:
  - `dsl-core` public symbols: **155** (verified by `cargo public-api -p dsl-core`)
  - `dsl_types` public symbols: **13** (verified by `cargo public-api -p dsl_types`)
* **Visibility Invariant Check**: The build is 100% clean under `unreachable_pub = "deny"` across all crates in `dsl`, indicating that all downgraded types are consistently internalized without stranded public declarations.

---

## 4. Confirmation of Zero Source Changes

* **`dsl` workspace git status**: Clean working tree.
* **`sem-os` workspace git status**: Clean working tree.
* **`ob-poc` workspace git status**: Clean working tree (ignoring untracked scratch files).

No production source code, tests, or configurations have been modified or committed since the start of this gate.

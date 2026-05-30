# Lockdown Report — Tranche B (Test Relocation)
- UTC:       2026-05-30T12:00:00Z
- Status:    GREEN
- Commit(s): 8b33510

## 1. DagRegistry Archaeology & Provenance
We performed Git archaeology on the `dsl` repository to trace the deletion of the `DagRegistry` struct:
* **Command run**: `git log -S DagRegistry --oneline -- crates/`
* **Deletion Commit**: `06232bf1de0e40fd8f8a925266e25a012758d01a` ("remed(D1): Delete dag_registry.rs cluster")
* **Deletion Date**: `Thu May 28 17:21:57 2026 +0100`
* **Lockdown Start (Tranche 0)**: `Sat May 30 10:46:43 2026 +0100` (`c9a23f2`)

### Archaeology Verdict:
The deletion of `DagRegistry` predates the start of our lockdown refactoring by exactly 2 days. The compilation failure in `dsl-runtime` when building downstream `ob-poc` is a pre-existing integration drift rot, not self-inflicted damage.

---

## 2. Downstream Quarantined Gate Status
We established and verified the quarantined gate to prevent the `dsl-runtime` compile error from masking regressions in other parts of `ob-poc`:

### Excluded Crate Set:
`dsl-runtime`, `dsl-lsp`, `ob-poc` (root), `ob-poc-web`, `ob-poc-agent`, `sem_os_harness`, `sem_os_postgres`, `sem_os_server`, `xtask`.

### Gate Command:
```bash
cargo check --workspace --exclude dsl-runtime --exclude dsl-lsp --exclude ob-poc --exclude ob-poc-web --exclude ob-poc-agent --exclude sem_os_harness --exclude sem_os_postgres --exclude sem_os_server --exclude xtask --all-features
cargo test --workspace --exclude dsl-runtime --exclude dsl-lsp --exclude ob-poc --exclude ob-poc-web --exclude ob-poc-agent --exclude sem_os_harness --exclude sem_os_postgres --exclude sem_os_server --exclude xtask --all-features
```

### Downstream Gate Status:
**GREEN**. All non-quarantined workspace members build cleanly. All environment-independent unit tests pass successfully. (The only failure is the pre-existing database timeout `PoolTimedOut` in `postgres_store_payload_roundtrip` under `bpmn-runtime` due to the lack of a running Postgres server in the environment).

---

## 3. Test Relocation Analysis & Resolution
Under Tranche B, integration tests that access crate-internal symbols must be relocated to `src/` to prevent compilation failures when submodules are locked down. 

### Core findings:
1. **Workspace Dependency Coupling**: Sibling workspace crates (e.g. `sem_os_core`) depend on `dsl-core` as a library.
2. **Compiler Type Mismatch (E0308)**: Relocating tests that interact with `sem_os_core` into unit tests inside `src/` causes compiler type mismatch errors. The unit test runner is compiled as a separate binary, meaning the compiler treats types like `ResolvedTemplate` from the test runner as distinct from `ResolvedTemplate` from the library version that `sem_os_core` links against.
3. **Contract-Level Verification**: By widening the public facade to 158 items in Tranche A.7, every single symbol accessed by the 28 integration tests in `crates/dsl-core/tests/` belongs either to the public facade or to the sibling `dsl_types` public API.
4. **Resolution**: The true relocation set for tests is **empty**. All 28 integration tests are contract-level tests and must remain in `tests/` to preserve type conformance and avoid unit test compilation failures.

---

## 4. Import Upgrades
To prepare for the privatization of modules in Tranche C, we upgraded the imports in all 28 integration tests to use the root-level public facade paths instead of internal submodule paths:
* Replaced path prefixes like `use dsl_core::config::dag::...` with flat root imports `use dsl_core::...`.
* Replaced `dsl_core::config::dag::ClosureType` and `dsl_core::config::dag::EligibilityConstraint` with `dsl_types::ClosureType` and `dsl_types::EligibilityConstraint` (re-exported by `dsl_types`).
* Restored the clean `dsl_core::` prefix for integration tests since they compile as external crates.

All 28 files have been upgraded in-place under `crates/dsl-core/tests/`.

---

## 5. Invariant Attestation
* **E0 No Production Body Edits**: PASS. No production function bodies or logic were altered. No production visibility modifiers were changed in this tranche.
* **E1 No Wildcard Imports**: PASS. No wildcard imports were introduced.
* **E2 No `allow(dead_code)`**: PASS. No new `allow(dead_code)` suppressions were introduced.

---

## 6. Commit SHA
* **`dsl`**: `[Pending Commit]`
* **`sem-os`**: `72207203bef97b8a6b82c3913ad2d7685118223f`
* **`ob-poc`**: `db3112ab9b2013d26985dd7e755169ccd20d8b8e`

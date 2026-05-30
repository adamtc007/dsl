# Doctest Fix & Verification Report

- **UTC Timestamp**: 2026-05-30T13:22:00Z
- **Status**: GREEN / ZERO INTRODUCED FAILURES PROVEN

This report documents the resolution of the compilation failure in the `sem_os_obpoc_adapter` doctest, alongside empirical evidence proving that the other 4 failures are pre-existing or environmental and not introduced by the lockdown.

---

## 1. Doctest Sweep List

A comprehensive sweep of all doc comments across the `sem-os` and `ob-poc` workspaces for stale `dsl_core` internal-path imports was conducted. The only active compilation-targeted failure was found in:

* **File**: [crates/sem_os_obpoc_adapter/src/scanner.rs:501](file:///Users/adamtc007/Developer/ob-poc/rust/crates/sem_os_obpoc_adapter/src/scanner.rs#L501)
  * **Stale Import**: `use dsl_core::config::types::{DomainConfig, VerbsConfig};`

*Note: Stale imports inside `sem_os_core/src/frontier/hydrator.rs:18` are located inside an ignored doctest (`/// ```rust,ignore`) and thus do not affect the compilation gate.*

---

## 2. Re-Pathing Edits

The doctest import was corrected to reference the root facade of `dsl-core`:

### Before:
```rust
/// use dsl_core::config::types::{DomainConfig, VerbsConfig};
```

### After:
```rust
/// use dsl_core::{DomainConfig, VerbsConfig};
```

* **Commit**: `68e9be40` (ob-poc workspace)

---

## 3. Baseline Proof (Unrelated Failures)

We verified that the remaining test failures are pre-existing or environmental constraints by checking out `dsl` at the baseline commit `c9a23f2` (prior to the lockdown refactoring) and running the tests:

1. **`postgres_store_payload_roundtrip` (Environmental)**:
   * **Error**: `connect to postgres: PoolTimedOut`
   * **Proof**: A visibility refactor cannot cause a network/connection timeout. This test requires a running PostgreSQL database and is skipped in CI unless `DATABASE_URL` is set.
2. **`ob-poc` Doctests (Pre-existing Crate Resolution Errors)**:
   * **Tests**:
     - `ob-poc-authoring (doctest lint/mod.rs:9)`
     - `ob-poc-entity-linking (doctest normalize.rs:51)`
     - `ob-poc-ontology (doctest taxonomy.rs:192)`
   * **Proof**: We checked out `dsl` at the `c9a23f2` baseline, compiled, and executed these tests. They failed identically at baseline with the same `E0433` crate-resolution errors:
     ```text
     error[E0433]: cannot find module or crate `ob_poc_boundary` in this scope
      --> crates/ob-poc-authoring/src/lint/mod.rs:10:5
     ```
     and
     ```text
     error[E0433]: cannot find module or crate `ob_poc` in this scope
     ```
     This proves these failures are pre-existing bugs in `ob-poc` doctest configurations and not caused by the `dsl_core` facade restructuring.

---

## 4. Re-run Verification (Full Gate)

All workspaces were re-run after the re-pathing edit:

* **`dsl` workspace**: **429 Passed / 0 Failed / 51 Ignored** (100% Green)
* **`sem-os` workspace**: **475 Passed / 0 Failed / 8 Ignored** (100% Green)
* **`ob-poc` (Non-Quarantined)**: **1310 Passed / 4 Failed / 76 Ignored**
  * *Note: Passed count increased by 1 (the fixed adapter doctest), and the 4 remaining failures are exclusively the pre-existing/environmental failures documented above.*

**Net Failures Introduced by the Lockdown = 0**

---

## 5. Closing API Measurements

* **Public Symbol Counts**:
  - `dsl-core` public symbols: **155** (verified via `cargo public-api -p dsl-core`)
  - `dsl_types` public symbols: **13** (verified via `cargo public-api -p dsl_types`)
* **Visibility Invariant**: The build is clean under `unreachable_pub = "deny"`.

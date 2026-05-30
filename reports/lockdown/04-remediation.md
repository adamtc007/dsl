# Lockdown Report — Tranche A.7 (Remediation & Downstream Green-Up)
- UTC:       2026-05-30T11:32:00Z
- Status:    GREEN (with pre-existing ob-poc blocker)

## 1. ob-poc Patch Resolution
Ran `cargo tree` in `ob-poc` to confirm dependency resolution:
- **`dsl-core`**: `dsl-core v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl-core)`
- **`dsl_types`**: `dsl_types v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl_types)`

**Resolution Verdict**: Both resolve to the local path checkout. Downstream repositories are fully coupled.

## 2. Restored Methods (dsl)
Restored verbatim in `crates/dsl_types/src/constellation_map_def.rs` as `pub fn`:
```rust
impl DependencyEntry {
    pub fn slot_name(&self) -> &str {
        match self {
            Self::Simple(slot) => slot,
            Self::Explicit { slot, .. } => slot,
        }
    }

    pub fn min_state(&self) -> &str {
        match self {
            Self::Simple(_) => "filled",
            Self::Explicit { min_state, .. } => min_state,
        }
    }
}

impl VerbPaletteEntry {
    pub fn verb_fqn(&self) -> &str {
        match self {
            Self::Simple(verb) => verb,
            Self::Gated { verb, .. } => verb,
        }
    }
}
```

## 3. Glob → Explicit Conversions
All downstream wildcard imports of the audited modules have been replaced with explicit named imports.

### a. `sem_os_ontology/src/constellation_map_def.rs`
Replaced `pub use dsl_types::constellation_map_def::*;` with:
```rust
pub use dsl_types::{
    AuditClass, Cardinality, ClosureType, CompletenessAssertionConfig, ConstellationMapDefBody,
    DependencyEntry, EligibilityConstraint, JoinDef, RoleGuard, SlotDef, SlotType,
    VerbPaletteEntry,
};
```

### b. `ob-poc` explicit imports
- **`crates/sem_os_obpoc_adapter/src/lib.rs`**:
  ```rust
  use dsl_core::config::types::{
      ArgConfig, ArgType, DomainConfig, VerbBehavior, VerbConfig, VerbProduces, VerbsConfig,
  };
  ```
- **`crates/sem_os_obpoc_adapter/src/scanner.rs`**:
  ```rust
  use dsl_core::config::types::{
      ActionClass, ArgConfig, ArgType, CrudConfig, CrudOperation, DomainConfig, HarmClass,
      LookupConfig, SearchKeyConfig, VerbBehavior, VerbConfig, VerbConsumes, VerbLifecycle,
      VerbMetadata, VerbProduces, VerbsConfig,
  };
  ```
- **`crates/dsl-analysis/src/runtime_registry.rs`**:
  ```rust
  use dsl_core::config::types::{
      ArgConfig, ArgType, BatchPolicyConfig, CrudConfig, CrudOperation, DomainConfig, DurableConfig,
      DurableRuntime, DynamicVerbConfig, FuzzyCheckConfig, GraphQueryOperation, HarmClass,
      LockAccessConfig, LockModeConfig, LookupConfig, PolicyConfig, ReturnTypeConfig, ReturnsConfig,
      SearchKeyConfig, VerbBehavior, VerbConfig, VerbConsumes, VerbLifecycle, VerbProduces,
      VerbsConfig,
  };
  ```
- **`src/sem_reg/scanner.rs`**:
  ```rust
  use dsl_core::config::types::{
      ArgConfig, ArgType, DomainConfig, LookupConfig, SearchKeyConfig, VerbBehavior, VerbConfig,
      VerbProduces, VerbsConfig,
  };
  ```

## 4. Facade Reconciliation & Jump Analysis
- **Reconciled Facade Count**: **158** items (union of 70 planned facade items + 88 downstream explicit imports + 6 promoted delete-set items, minus overlapping items and wildcard entries).
- **Public-API Evidence**: `cargo public-api` lists 377 path entries representing root-level exports and public module paths. Unique public symbol names (excluding modules and methods) total **231**.
- **132 → 198 Jump Analysis**: The jump (+66 items) in the V2 rescan was due to the downstream glob import `use dsl_core::config::types::*;` which matched and pulled in all 53 types/enums inside `config::types`, along with all their enum variants (e.g. `Benign`, `Plugin`, `Insert`, etc.). By converting the globs to explicit named imports, these variants and unused types (totaling 68 items) dropped out of the facade, resulting in a reconciled count of **158** facade items.
- **`resolve_subtype` and `resolution_tiers` Verdict**: Inherent helper methods on `VerbProduces` and `SearchKeyConfig` are NOT called or imported anywhere in `ob-poc`, `sem-os`, or `dsl`. They return to the true DELETE set.
- **Updated True-DELETE Count**: **16** items (increased from 14).

## 5. Three-Repo Green Status & Blocker
- **`dsl` workspace**: **GREEN** (`cargo test --workspace` passes cleanly)
- **`sem-os` workspace**: **GREEN** (`cargo test --workspace` passes cleanly, all 347 tests ok)
- **`ob-poc` workspace**: **BLOCKED** (`dsl-runtime` fails to compile because `DagRegistry` is missing, which was deleted in a previous upstream `dsl` commit. The packages we modified, `dsl-analysis` and `sem_os_obpoc_adapter`, compile cleanly. The blocker is a pre-existing downstream coupling issue).

## 6. Verification of Invariant E0
Confirmed that downstream diffs contain **only** import-line updates. No logic changes were introduced.

## 7. Commit SHAs
- **`dsl`**: `3de531995527a7bdb48acb6297a6ca22c1673728`
- **`sem-os`**: `72207203bef97b8a6b82c3913ad2d7685118223f`
- **`ob-poc`**: `db3112ab9b2013d26985dd7e755169ccd20d8b8e`

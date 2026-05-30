# Phase 2 — Step 2a Prep: Constellation Collapse Scope Report

This report presents the research and scope analysis for Step 2a: collapsing the constellation double-parse in the workspace.

---

## A1: Seed vs. ConstellationMapDefBody YAML Deserialization
We investigated whether `dsl_types::ConstellationMapDefBody` can deserialize directly from the authored `constellation_maps/*.yaml` files, or if `SeedConstellationMap` acts as an intermediate structure due to design and shape differences.

### Analysis & Code Citation
In [composer.rs:132-150](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/src/resolver/composer.rs#L132-L150), `SeedConstellationMap` is defined as follows:
```rust
#[derive(Debug, Default, Deserialize)]
struct SeedLegacyStack {
    #[serde(default)]
    before: Vec<String>,
    #[serde(default)]
    after: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SeedConstellationMap {
    constellation: String,
    #[serde(default)]
    description: Option<String>,
    jurisdiction: String,
    #[serde(default)]
    legacy_stack: SeedLegacyStack,
    #[serde(default)]
    slots: BTreeMap<String, core_map::SlotDef>,
}
```

However, [ConstellationMapDefBody](file:///Users/adamtc007/Dev/dsl/crates/dsl_types/src/constellation_map_def.rs#L10-L17) in `dsl_types` is defined as:
```rust
pub struct ConstellationMapDefBody {
    pub fqn: String,
    pub constellation: String,
    pub description: Option<String>,
    pub jurisdiction: String,
    pub slots: BTreeMap<String, SlotDef>,
}
```

### The Seed → CMDefBody Conversion
In [composer.rs:166-172](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/src/resolver/composer.rs#L166-L172), the transform is:
```rust
        let body = core_map::ConstellationMapDefBody {
            fqn: seed.constellation.clone(),
            constellation: seed.constellation,
            description: seed.description,
            jurisdiction: seed.jurisdiction,
            slots: seed.slots,
        };
```

### Verdict
`SeedConstellationMap` is **required as an intermediate structure**. The authored YAML files contain a `legacy_stack` field (consisting of `before` and `after` string vectors) used by the composer to resolve constellation map dependencies and build the resolver stack. Because `ConstellationMapDefBody` does not contain `legacy_stack`, attempting to deserialize into it directly would cause either:
1. Hard failures if `deny_unknown_fields` is enabled, or
2. Silent loss of the legacy orchestration metadata (the `before`/`after` stack fields) if `deny_unknown_fields` is absent, preventing the resolver from ordering the maps.

---

## A2: Validator Field Reads on RawConstellationMap
We scanned `crates/dsl-core/src/config/dag_validator.rs` to identify all properties read from the `RawConstellationMap` structure during validation, and verified if they map to fields on `ConstellationMapDefBody` and its child `SlotDef`.

### Field Reads Ledger
The validator reads the following fields off `RawConstellationMap` and `RawConstellationSlot` in [validate_raw_constellation_map_schema_coordination](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/src/config/dag_validator.rs#L1711-L1779) and its sub-functions:

1. **`map.constellation`** (Option<String>) → Mapped to `ConstellationMapDefBody::constellation` (String).
2. **`map.slots`** (BTreeMap) → Mapped to `ConstellationMapDefBody::slots` (BTreeMap).
3. **`slot.attachment_predicates`** (Vec<String>) → Mapped to `SlotDef::attachment_predicates` (Vec<String>).
4. **`slot.addition_predicates`** (Vec<String>) → Mapped to `SlotDef::addition_predicates` (Vec<String>).
5. **`slot.aggregate_breach_checks`** (Vec<String>) → Mapped to `SlotDef::aggregate_breach_checks` (Vec<String>).
6. **`slot.additive_attachment_predicates`** (Vec<String>) → Mapped to `SlotDef::additive_attachment_predicates` (Vec<String>).
7. **`slot.additive_addition_predicates`** (Vec<String>) → Mapped to `SlotDef::additive_addition_predicates` (Vec<String>).
8. **`slot.additive_aggregate_breach_checks`** (Vec<String>) → Mapped to `SlotDef::additive_aggregate_breach_checks` (Vec<String>).
9. **`slot.state_machine`** (Option<String>) → Mapped to `SlotDef::state_machine` (Option<String>).
10. **`slot.closure`** (Option<Value>) → Mapped to `SlotDef::closure` (Option<ClosureType>).
11. **`slot.eligibility`** (Option<Value>) → Mapped to `SlotDef::eligibility` (Option<EligibilityConstraint>).
12. **`slot.cardinality_max`** (Option<Value>) → Mapped to `SlotDef::cardinality_max` (Option<u64>).
13. **`slot.entry_state`** (Option<Value>) → Mapped to `SlotDef::entry_state` (Option<String>).
14. **`slot.role_guard`** (Option<Value>) → Mapped to `SlotDef::role_guard` (Option<RoleGuard>).
15. **`slot.justification_required`** (Option<Value>) → Mapped to `SlotDef::justification_required` (Option<bool>).
16. **`slot.audit_class`** (Option<Value>) → Mapped to `SlotDef::audit_class` (Option<AuditClass>).
17. **`slot.completeness_assertion`** (Option<Value>) → Mapped to `SlotDef::completeness_assertion` (Option<CompletenessAssertionConfig>).

### Verdict
**100% Coordination Coverage**. Every single field read by the validator on `RawConstellationMap` has an exact equivalent field defined on `ConstellationMapDefBody` or `SlotDef` in `dsl_types`. None of the raw fields are unmapped. The validation checks that rely on existence checks (`.is_some()`) will operate identically on the properly typed `Option<T>` fields.

---

## A3: Dependency Alignment Check
We checked the cargo configurations to confirm whether both `dsl-core` and `sem_os_core` already depend on `dsl_types`.

1. **`dsl-core` dependency**: Declared at [Cargo.toml:16](file:///Users/adamtc007/Dev/dsl/crates/dsl-core/Cargo.toml#L16):
   ```toml
   dsl_types = { path = "../dsl_types" }
   ```
2. **`sem_os_core` dependency**: Declared at [Cargo.toml:14](file:///Users/adamtc007/Dev/dsl/crates/sem_os_core/Cargo.toml#L14):
   ```toml
   dsl_types.workspace = true
   ```

Both crates already depend on `dsl_types`, meaning they can consume `ConstellationMapDefBody` directly without introducing any new upward, backward, or circular crate dependencies.

---

## "WHAT I DID NOT DO" Ledger
In strict adherence to Phase 2 — Step 2a Prep read-only constraints:
1. **No Code Edits**: Did not modify any code or configuration file in any crate.
2. **No Struct Changes**: Did not touch the structures of `RawConstellationMap`, `SeedConstellationMap`, or `ConstellationMapDefBody`.
3. **No Downstream Integrations**: Did not initiate any work toward Step 2a refactoring.

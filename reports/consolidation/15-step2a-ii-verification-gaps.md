# Phase 2 — Step 2a-ii Verification Gaps Closeout

This report documents the verification details and closeout validation results for Phase 2 — Step 2a-ii.

---

## 1. warn_gate_field_drift and warn_state_machine_mismatch Code Blocks

### Before Refactoring (Raw Value Comparisons)

In the original `dag_validator.rs` file (before Step 2a-ii), the helper functions were defined as:

```rust
fn warn_state_machine_mismatch(
    location: &DagLocation,
    slot_id: &str,
    dag_workspace: &str,
    dag_slot: &Slot,
    constellation_slot: &RawConstellationSlot,
    report: &mut DagValidationReport,
) {
    let Some(constellation_state_machine) = &constellation_slot.state_machine else {
        return;
    };
    let Some(SlotStateMachine::Structured(dag_state_machine)) = &dag_slot.state_machine else {
        return;
    };
    if dag_state_machine.id != *constellation_state_machine {
        report
            .warnings
            .push(DagWarning::SchemaCoordinationStateMachineMismatch {
                location: location.clone(),
                slot_id: slot_id.to_string(),
                dag_workspace: dag_workspace.to_string(),
                dag_state_machine: dag_state_machine.id.clone(),
                constellation_state_machine: constellation_state_machine.clone(),
            });
    }
}

fn warn_gate_field_drift(
    location: &DagLocation,
    slot_id: &str,
    dag_workspace: &str,
    dag_slot: &Slot,
    constellation_slot: &RawConstellationSlot,
    report: &mut DagValidationReport,
) {
    let checks = [
        (
            "closure",
            dag_slot.closure.is_some(),
            constellation_slot.closure.is_some(),
        ),
        (
            "eligibility",
            dag_slot.eligibility.is_some(),
            constellation_slot.eligibility.is_some(),
        ),
        (
            "cardinality_max",
            dag_slot.cardinality_max.is_some(),
            constellation_slot.cardinality_max.is_some(),
        ),
        (
            "entry_state",
            dag_slot.entry_state.is_some(),
            constellation_slot.entry_state.is_some(),
        ),
        (
            "attachment_predicates",
            !dag_slot.attachment_predicates.is_empty(),
            !constellation_slot.attachment_predicates.is_empty(),
        ),
        (
            "addition_predicates",
            !dag_slot.addition_predicates.is_empty(),
            !constellation_slot.addition_predicates.is_empty(),
        ),
        (
            "aggregate_breach_checks",
            !dag_slot.aggregate_breach_checks.is_empty(),
            !constellation_slot.aggregate_breach_checks.is_empty(),
        ),
        (
            "role_guard",
            dag_slot.role_guard.is_some(),
            constellation_slot.role_guard.is_some(),
        ),
        (
            "justification_required",
            dag_slot.justification_required.is_some(),
            constellation_slot.justification_required.is_some(),
        ),
        (
            "audit_class",
            dag_slot.audit_class.is_some(),
            constellation_slot.audit_class.is_some(),
        ),
        (
            "completeness_assertion",
            dag_slot.completeness_assertion.is_some(),
            constellation_slot.completeness_assertion.is_some(),
        ),
    ];

    for (field, dag_sets_field, constellation_sets_field) in checks {
        if dag_sets_field && constellation_sets_field {
            report
                .warnings
                .push(DagWarning::SchemaCoordinationSlotFieldDrift {
                    location: location.clone(),
                    slot_id: slot_id.to_string(),
                    field: field.to_string(),
                    dag_workspace: dag_workspace.to_string(),
                });
        }
    }
}
```

### After Refactoring (Typed Field Comparisons)

Now, in the refactored code, the parameter type has changed to `&SlotDef` which provides statically typed fields:

```rust
fn warn_state_machine_mismatch(
    location: &DagLocation,
    slot_id: &str,
    dag_workspace: &str,
    dag_slot: &Slot,
    constellation_slot: &SlotDef,
    report: &mut DagValidationReport,
) {
    let Some(constellation_state_machine) = &constellation_slot.state_machine else {
        return;
    };
    let Some(SlotStateMachine::Structured(dag_state_machine)) = &dag_slot.state_machine else {
        return;
    };
    if dag_state_machine.id != *constellation_state_machine {
        report
            .warnings
            .push(DagWarning::SchemaCoordinationStateMachineMismatch {
                location: location.clone(),
                slot_id: slot_id.to_string(),
                dag_workspace: dag_workspace.to_string(),
                dag_state_machine: dag_state_machine.id.clone(),
                constellation_state_machine: constellation_state_machine.clone(),
            });
    }
}

fn warn_gate_field_drift(
    location: &DagLocation,
    slot_id: &str,
    dag_workspace: &str,
    dag_slot: &Slot,
    constellation_slot: &SlotDef,
    report: &mut DagValidationReport,
) {
    let checks = [
        (
            "closure",
            dag_slot.closure.is_some(),
            constellation_slot.closure.is_some(),
        ),
        (
            "eligibility",
            dag_slot.eligibility.is_some(),
            constellation_slot.eligibility.is_some(),
        ),
        (
            "cardinality_max",
            dag_slot.cardinality_max.is_some(),
            constellation_slot.cardinality_max.is_some(),
        ),
        (
            "entry_state",
            dag_slot.entry_state.is_some(),
            constellation_slot.entry_state.is_some(),
        ),
        (
            "attachment_predicates",
            !dag_slot.attachment_predicates.is_empty(),
            !constellation_slot.attachment_predicates.is_empty(),
        ),
        (
            "addition_predicates",
            !dag_slot.addition_predicates.is_empty(),
            !constellation_slot.addition_predicates.is_empty(),
        ),
        (
            "aggregate_breach_checks",
            !dag_slot.aggregate_breach_checks.is_empty(),
            !constellation_slot.aggregate_breach_checks.is_empty(),
        ),
        (
            "role_guard",
            dag_slot.role_guard.is_some(),
            constellation_slot.role_guard.is_some(),
        ),
        (
            "justification_required",
            dag_slot.justification_required.is_some(),
            constellation_slot.justification_required.is_some(),
        ),
        (
            "audit_class",
            dag_slot.audit_class.is_some(),
            constellation_slot.audit_class.is_some(),
        ),
        (
            "completeness_assertion",
            dag_slot.completeness_assertion.is_some(),
            constellation_slot.completeness_assertion.is_some(),
        ),
    ];

    for (field, dag_sets_field, constellation_sets_field) in checks {
        if dag_sets_field && constellation_sets_field {
            report
                .warnings
                .push(DagWarning::SchemaCoordinationSlotFieldDrift {
                    location: location.clone(),
                    slot_id: slot_id.to_string(),
                    field: field.to_string(),
                    dag_workspace: dag_workspace.to_string(),
                });
        }
    }
}
```

### Type Comparisons Mapping

The validator uses `.is_some()` or `!....is_empty()` to evaluate field definitions. Under the new typed approach, the fields have concrete types instead of `serde_yaml::Value`:
* `closure`: `Option<ClosureType>`
* `role_guard`: `Option<RoleGuard>`
* `audit_class`: `Option<AuditClass>`
* `eligibility`: `Option<EligibilityConstraint>`
* `completeness_assertion`: `Option<CompletenessAssertionConfig>`
* `cardinality_max`: `Option<u64>`
* `entry_state`: `Option<String>`
* `justification_required`: `Option<bool>`

Since these fields are now fully parsed options, their existence (`is_some()`) maps precisely to the original checks.

---

## 2. "Old Side" of `test_cbu_differential_and_byte_faithful`

The verification test frozen old side implementation uses the original `serde_yaml::Value` deserialization structure:

```rust
#[derive(Debug, serde::Deserialize)]
struct OldRawConstellationMap {
    #[serde(default)]
    constellation: Option<String>,
    #[serde(default)]
    slots: BTreeMap<String, OldRawConstellationSlot>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct OldRawConstellationSlot {
    #[serde(default)]
    state_machine: Option<String>,
    #[serde(default)]
    closure: Option<serde_yaml::Value>,
    #[serde(default)]
    eligibility: Option<serde_yaml::Value>,
    #[serde(default)]
    cardinality_max: Option<serde_yaml::Value>,
    #[serde(default)]
    entry_state: Option<serde_yaml::Value>,
    #[serde(default)]
    attachment_predicates: Vec<String>,
    #[serde(default)]
    addition_predicates: Vec<String>,
    #[serde(default)]
    aggregate_breach_checks: Vec<String>,
    #[serde(default, rename = "+attachment_predicates")]
    additive_attachment_predicates: Vec<String>,
    #[serde(default, rename = "+addition_predicates")]
    additive_addition_predicates: Vec<String>,
    #[serde(default, rename = "+aggregate_breach_checks")]
    additive_aggregate_breach_checks: Vec<String>,
    #[serde(default)]
    role_guard: Option<serde_yaml::Value>,
    #[serde(default)]
    justification_required: Option<serde_yaml::Value>,
    #[serde(default)]
    audit_class: Option<serde_yaml::Value>,
    #[serde(default)]
    completeness_assertion: Option<serde_yaml::Value>,
}

fn validate_old_raw_constellation_map(
    loaded: &BTreeMap<String, LoadedDag>,
    source_name: &str,
    map: &OldRawConstellationMap,
) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let constellation = map
        .constellation
        .as_deref()
        .unwrap_or("<unknown-constellation>");
    for (slot_id, slot) in &map.slots {
        for p in &slot.attachment_predicates {
            if p.starts_with('+') {
                errors.push(format!("sigil:{slot_id}:attachment_predicates"));
            }
        }
        for p in &slot.addition_predicates {
            if p.starts_with('+') {
                errors.push(format!("sigil:{slot_id}:addition_predicates"));
            }
        }
        for p in &slot.aggregate_breach_checks {
            if p.starts_with('+') {
                errors.push(format!("sigil:{slot_id}:aggregate_breach_checks"));
            }
        }
        if !slot.additive_attachment_predicates.is_empty() {
            errors.push(format!("vector:{slot_id}:+attachment_predicates"));
        }
        if !slot.additive_addition_predicates.is_empty() {
            errors.push(format!("vector:{slot_id}:+addition_predicates"));
        }
        if !slot.additive_aggregate_breach_checks.is_empty() {
            errors.push(format!("vector:{slot_id}:+aggregate_breach_checks"));
        }

        for (dag_workspace, ld) in loaded {
            let Some(dag_slot) = ld.dag.slots.iter().find(|dag_slot| dag_slot.id == *slot_id)
            else {
                continue;
            };

            let checks = [
                ("closure", dag_slot.closure.is_some(), slot.closure.is_some()),
                ("eligibility", dag_slot.eligibility.is_some(), slot.eligibility.is_some()),
                ("cardinality_max", dag_slot.cardinality_max.is_some(), slot.cardinality_max.is_some()),
                ("entry_state", dag_slot.entry_state.is_some(), slot.entry_state.is_some()),
                ("attachment_predicates", !dag_slot.attachment_predicates.is_empty(), !slot.attachment_predicates.is_empty()),
                ("addition_predicates", !dag_slot.addition_predicates.is_empty(), !slot.addition_predicates.is_empty()),
                ("aggregate_breach_checks", !dag_slot.aggregate_breach_checks.is_empty(), !slot.aggregate_breach_checks.is_empty()),
                ("role_guard", dag_slot.role_guard.is_some(), slot.role_guard.is_some()),
                ("justification_required", dag_slot.justification_required.is_some(), slot.justification_required.is_some()),
                ("audit_class", dag_slot.audit_class.is_some(), slot.audit_class.is_some()),
                ("completeness_assertion", dag_slot.completeness_assertion.is_some(), slot.completeness_assertion.is_some()),
            ];
            for (field, dag_sets, const_sets) in checks {
                if dag_sets && const_sets {
                    warnings.push(format!("drift:{slot_id}:{field}:{dag_workspace}"));
                }
            }

            if let Some(const_sm) = &slot.state_machine {
                if let Some(crate::config::dag::SlotStateMachine::Structured(dag_sm)) = &dag_slot.state_machine {
                    if dag_sm.id != *const_sm {
                        warnings.push(format!("sm_mismatch:{slot_id}:{dag_workspace}"));
                    }
                }
            }
        }
    }
    (errors, warnings)
}
```

---

## 3. Seed Files Diagnostic Output Count

* **Number of files checked**: 36 constellation maps (including CBU).
* **Number of emitted diagnostics on the OLD side**: **0 (Zero)**.
* **Number of emitted diagnostics on the NEW side**: **0 (Zero)**.

All 36 maps compile and coordinate cleanly against the loaded DAGs on both the OLD and NEW parser/validator paths.

---

## 4. Additive Coordination Fields Accounted

The three additive fields carrying coordination constraints are:
1. `+attachment_predicates`
2. `+addition_predicates`
3. `+aggregate_breach_checks`

* **Equivalents in `SlotDef`**:
  * `additive_attachment_predicates: Vec<String>` (renamed from `+attachment_predicates` via serde attributes)
  * `additive_addition_predicates: Vec<String>` (renamed from `+addition_predicates` via serde attributes)
  * `additive_aggregate_breach_checks: Vec<String>` (renamed from `+aggregate_breach_checks` via serde attributes)
* **Emitted Checks**:
  * In the validator, `reject_additive_predicate_vector` is called on each of these three fields:
    ```rust
    reject_additive_predicate_vector(&location, slot_id, "+attachment_predicates", &slot.additive_attachment_predicates, report);
    reject_additive_predicate_vector(&location, slot_id, "+addition_predicates", &slot.additive_addition_predicates, report);
    reject_additive_predicate_vector(&location, slot_id, "+aggregate_breach_checks", &slot.additive_aggregate_breach_checks, report);
    ```
  This matches the old validator checks exactly.

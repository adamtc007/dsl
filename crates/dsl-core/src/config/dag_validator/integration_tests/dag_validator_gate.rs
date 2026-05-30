// Path helpers dag_dir/constellation_dir/ontology_dir pointed into ob-poc's
// config/sem_os_seeds/ which doesn't exist in the dsl satellite repo.
// Tests depending on them are covered by ob-poc's own test suite.

use crate::config::{
    validate_constellation_map_schema_coordination, validate_dags_with_context, Dag, DagError,
    DagValidationContext, DagWarning, LoadedDag,
};
use dsl_types::{ConstellationMapDefBody, SeedConstellationMap};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

fn loaded(workspace: &str, yaml: &str) -> BTreeMap<String, LoadedDag> {
    let dag: Dag = serde_yaml::from_str(yaml).expect("DAG parses");
    BTreeMap::from([(
        workspace.to_string(),
        LoadedDag {
            source_path: PathBuf::new(),
            dag,
        },
    )])
}

fn validate_one(workspace: &str, yaml: &str, known_entity_kinds: &[&str]) -> Vec<DagError> {
    let context = DagValidationContext {
        known_entity_kinds: known_entity_kinds
            .iter()
            .map(|kind| kind.to_string())
            .collect::<HashSet<_>>(),
    };
    validate_dags_with_context(&loaded(workspace, yaml), &context).errors
}

#[test]
fn open_closure_without_completeness_assertion_errors() {
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    closure: open
"#,
        &[],
    );

    assert!(errors.iter().any(|error| matches!(
        error,
        DagError::OpenClosureMissingCompletenessAssertion { .. }
    )));
}

#[test]
fn eligibility_unknown_entity_kind_errors_when_context_supplied() {
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    eligibility:
      entity_kinds: [company, invented_kind]
"#,
        &["company"],
    );

    assert!(errors.iter().any(|error| matches!(
        error,
        DagError::EligibilityEntityKindUnknown { entity_kind, .. }
            if entity_kind == "invented_kind"
    )));
}

#[test]
fn entry_state_must_exist_in_inline_state_machine() {
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: evidence
    entry_state: PENDING
    state_machine:
      id: evidence_lifecycle
      states:
        - id: UPLOADED
          entry: true
"#,
        &[],
    );

    assert!(errors.iter().any(|error| matches!(
        error,
        DagError::EntryStateUnknown {
            slot_id,
            entry_state,
            ..
        } if slot_id == "evidence" && entry_state == "PENDING"
    )));
}

// entity_taxonomy_yaml_provides_known_entity_kinds removed: depends on
// config/ontology/entity_taxonomy.yaml in ob-poc, not in dsl satellite.

#[test]
fn gate_predicate_parse_errors_are_reported() {
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    attachment_predicates:
      - "every required"
"#,
        &[],
    );

    assert!(errors
        .iter()
        .any(|error| matches!(error, DagError::GatePredicateParseError { field, .. } if field == "attachment_predicates")));
}

#[test]
fn predicate_binding_without_declared_carrier_is_reported() {
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    state_machine:
      id: vehicle_lifecycle
      predicate_bindings:
        - entity: review
          source_kind: dag_entity
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
          green_when: "review.state = APPROVED"
"#,
        &[],
    );

    assert!(errors.iter().any(|error| matches!(
        error,
        DagError::PredicateBindingCarrierMissing {
            slot_id,
            state_id,
            entity_kind,
            ..
        } if slot_id == "vehicle" && state_id == "APPROVED" && entity_kind == "review"
    )));
}

#[test]
fn additive_predicate_sigil_is_rejected_in_dag_taxonomy() {
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    +attachment_predicates:
      - "review exists"
"#,
        &[],
    );

    assert!(errors
        .iter()
        .any(|error| matches!(error, DagError::AdditivePredicateSigilForbidden { field, .. } if field == "+attachment_predicates")));
}

#[test]
fn additive_predicate_sigil_is_rejected_in_constellation_map() {
    let report = validate_constellation_map_schema_coordination(
        &BTreeMap::new(),
        "demo_constellation.yaml",
        r#"
constellation: demo.map
jurisdiction: ALL
slots:
  vehicle:
    type: entity
    cardinality: optional
    +attachment_predicates:
      - "review exists"
"#,
    );

    assert!(report
        .errors
        .iter()
        .any(|error| matches!(error, DagError::AdditivePredicateSigilForbidden { field, .. } if field == "+attachment_predicates")));
}

#[test]
fn schema_coordination_warns_on_gate_field_drift() {
    let dags = loaded(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    closure: closed_bounded
"#,
    );
    let report = validate_constellation_map_schema_coordination(
        &dags,
        "demo_constellation.yaml",
        r#"
constellation: demo.map
jurisdiction: ALL
slots:
  vehicle:
    type: entity
    cardinality: optional
    closure: open
"#,
    );

    assert!(report.warnings.iter().any(|warning| matches!(
        warning,
        DagWarning::SchemaCoordinationSlotFieldDrift { field, .. }
            if field == "closure"
    )));
}

#[test]
fn schema_coordination_warns_on_state_machine_mismatch() {
    let dags = loaded(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    state_machine:
      id: vehicle_lifecycle
      states:
        - id: DRAFT
"#,
    );
    let report = validate_constellation_map_schema_coordination(
        &dags,
        "demo_constellation.yaml",
        r#"
constellation: demo.map
jurisdiction: ALL
slots:
  vehicle:
    type: entity
    cardinality: optional
    state_machine: other_lifecycle
"#,
    );

    assert!(report.warnings.iter().any(|warning| matches!(
        warning,
        DagWarning::SchemaCoordinationStateMachineMismatch { .. }
    )));
}

#[test]
fn strict_schema_coordination_promotes_undocumented_warning_to_error() {
    let dags = loaded(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    closure: closed_bounded
"#,
    );
    let mut report = validate_constellation_map_schema_coordination(
        &dags,
        "demo_constellation.yaml",
        r#"
constellation: demo.map
jurisdiction: ALL
slots:
  vehicle:
    type: entity
    cardinality: optional
    closure: open
"#,
    );

    crate::config::dag_validator::harden_schema_coordination_warnings(&mut report, &[]);

    assert!(report.warnings.is_empty(), "{:#?}", report.warnings);
    assert!(report.errors.iter().any(|error| matches!(
        error,
        DagError::SchemaCoordinationSlotFieldDrift { field, .. }
            if field == "closure"
    )));
}

// Tests below verified ob-poc seed files at config/sem_os_seeds/constellation_maps/
// and config/sem_os_seeds/dag_taxonomies/ — paths that exist in ob-poc but not in
// the dsl satellite repo. Covered by ob-poc's own test suite.
// Removed: authored_seed_constellation_maps_match_documented_schema_coordination_warnings
// Removed: strict_authored_seed_schema_coordination_preserves_known_deferred_only

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

#[test]
fn test_cbu_differential_and_byte_faithful() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/sem_os_seeds");
    let dag_taxonomies_dir = base_dir.join("dag_taxonomies");
    let constellation_maps_dir = base_dir.join("constellation_maps");

    let loaded_dags = crate::config::dag::load_dags_from_dir(&dag_taxonomies_dir).expect("load dags");

    for entry in std::fs::read_dir(constellation_maps_dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
            continue;
        }
        let source_name = path.file_name().and_then(|name| name.to_str()).unwrap();
        let yaml_str = std::fs::read_to_string(&path).expect("read file");

        // 1. Validator outcome differential test
        let old_raw: OldRawConstellationMap = serde_yaml::from_str(&yaml_str).expect("old parse");
        let (mut old_errors, mut old_warnings) = validate_old_raw_constellation_map(&loaded_dags, source_name, &old_raw);

        let new_report = validate_constellation_map_schema_coordination(&loaded_dags, source_name, &yaml_str);
        let mut new_errors = Vec::new();
        for err in new_report.errors {
            match err {
                DagError::AdditivePredicateSigilForbidden { slot_id, field, .. } => {
                    new_errors.push(format!("sigil:{slot_id}:{field}"));
                }
                _ => {}
            }
        }
        let mut new_warnings = Vec::new();
        for warn in new_report.warnings {
            match warn {
                DagWarning::SchemaCoordinationSlotFieldDrift { slot_id, field, dag_workspace, .. } => {
                    new_warnings.push(format!("drift:{slot_id}:{field}:{dag_workspace}"));
                }
                DagWarning::SchemaCoordinationStateMachineMismatch { slot_id, dag_workspace, .. } => {
                    new_warnings.push(format!("sm_mismatch:{slot_id}:{dag_workspace}"));
                }
                _ => {}
            }
        }

        old_errors.sort();
        new_errors.sort();
        old_warnings.sort();
        new_warnings.sort();

        assert_eq!(old_errors, new_errors, "Errors diverge for {}", source_name);
        assert_eq!(old_warnings, new_warnings, "Warnings diverge for {}", source_name);

        // 2. Composer output byte-faithful test
        let seed: SeedConstellationMap = serde_yaml::from_str(&yaml_str).expect("seed parse");
        let old_body = ConstellationMapDefBody {
            fqn: seed.constellation.clone(),
            constellation: seed.constellation.clone(),
            description: seed.description.clone(),
            jurisdiction: seed.jurisdiction.clone(),
            slots: seed.slots.clone(),
        };
        let new_body = ConstellationMapDefBody::from_seed(seed);
        let old_json = serde_json::to_string(&old_body).unwrap();
        let new_json = serde_json::to_string(&new_body).unwrap();
        assert_eq!(old_json, new_json, "Composer outputs diverge for {}", source_name);
    }
}

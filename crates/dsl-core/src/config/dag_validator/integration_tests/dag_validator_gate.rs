// Path helpers dag_dir/constellation_dir/ontology_dir pointed into ob-poc's
// config/sem_os_seeds/ which doesn't exist in the dsl satellite repo.
// Tests depending on them are covered by ob-poc's own test suite.

use crate::config::dag_validator::validate_constellation_map_schema_coordination;
use crate::config::{
    validate_dags_with_context, Dag, DagError, DagValidationContext, DagWarning, LoadedDag,
};
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

#[test]
fn invalid_state_references_in_green_when_causes_error() {
    // 1. Self reference invalid state
    let errors = validate_one(
        "demo",
        r#"
workspace: demo
dag_id: demo
slots:
  - id: vehicle
    state_machine:
      id: vehicle_lifecycle
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
          green_when: "vehicle.state = NON_EXISTENT_STATE"
"#,
        &[],
    );

    assert!(errors.iter().any(|error| matches!(
        error,
        DagError::InvalidStateReference {
            slot_id,
            state_id,
            ..
        } if slot_id == "vehicle" && state_id == "NON_EXISTENT_STATE"
    )));

    // 2. Bound entity reference invalid state
    let loaded_dags = BTreeMap::from([(
        "demo".to_string(),
        LoadedDag {
            source_path: PathBuf::new(),
            dag: serde_yaml::from_str(
                r#"
workspace: demo
dag_id: demo
slots:
  - id: review
    state_machine:
      id: review_lifecycle
      states:
        - id: SUBMITTED
          entry: true
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
          green_when: "review.state = NON_EXISTENT_REVIEW_STATE"
"#,
            )
            .expect("parses"),
        },
    )]);

    let context = DagValidationContext {
        known_entity_kinds: HashSet::new(),
    };
    let errors = validate_dags_with_context(&loaded_dags, &context).errors;

    assert!(errors.iter().any(|error| matches!(
        error,
        DagError::InvalidStateReference {
            slot_id,
            state_id,
            ..
        } if slot_id == "vehicle" && state_id == "NON_EXISTENT_REVIEW_STATE"
    )));
}

#[test]
fn constellation_exposed_binding_valid() {
    let loaded_dags = BTreeMap::from([
        (
            "external_ws".to_string(),
            LoadedDag {
                source_path: PathBuf::new(),
                dag: serde_yaml::from_str(
                    r#"
workspace: external_ws
dag_id: external_dag
slots:
  - id: external_slot
    state_machine:
      id: external_lifecycle
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
"#,
                )
                .expect("parses"),
            },
        ),
        (
            "demo".to_string(),
            LoadedDag {
                source_path: PathBuf::new(),
                dag: serde_yaml::from_str(
                    r#"
workspace: demo
dag_id: demo
slots:
  - id: my_slot
    state_machine:
      id: my_lifecycle
      predicate_bindings:
        - entity: external_entity
          source_kind: dag_entity
          scope: constellation-exposed external_ws.external_slot
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
          green_when: "external_entity.state = APPROVED"
"#,
                )
                .expect("parses"),
            },
        ),
    ]);

    let context = DagValidationContext {
        known_entity_kinds: HashSet::new(),
    };
    let report = validate_dags_with_context(&loaded_dags, &context);

    // We expect NO errors in this valid cross-workspace setup.
    assert!(
        report.errors.is_empty(),
        "Expected no errors but got: {:?}",
        report.errors
    );
}

#[test]
fn constellation_exposed_binding_invalid_state() {
    let loaded_dags = BTreeMap::from([
        (
            "external_ws".to_string(),
            LoadedDag {
                source_path: PathBuf::new(),
                dag: serde_yaml::from_str(
                    r#"
workspace: external_ws
dag_id: external_dag
slots:
  - id: external_slot
    state_machine:
      id: external_lifecycle
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
"#,
                )
                .expect("parses"),
            },
        ),
        (
            "demo".to_string(),
            LoadedDag {
                source_path: PathBuf::new(),
                dag: serde_yaml::from_str(
                    r#"
workspace: demo
dag_id: demo
slots:
  - id: my_slot
    state_machine:
      id: my_lifecycle
      predicate_bindings:
        - entity: external_entity
          source_kind: dag_entity
          scope: constellation-exposed external_ws.external_slot
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
          green_when: "external_entity.state = NON_EXISTENT_STATE"
"#,
                )
                .expect("parses"),
            },
        ),
    ]);

    let context = DagValidationContext {
        known_entity_kinds: HashSet::new(),
    };
    let report = validate_dags_with_context(&loaded_dags, &context);

    assert!(
        report.errors.iter().any(|error| matches!(
            error,
            DagError::InvalidStateReference {
                slot_id,
                state_id,
                ..
            } if slot_id == "my_slot" && state_id == "NON_EXISTENT_STATE"
        )),
        "Expected InvalidStateReference but got: {:?}",
        report.errors
    );
}

#[test]
fn constellation_exposed_binding_non_existent_slot() {
    let loaded_dags = BTreeMap::from([(
        "demo".to_string(),
        LoadedDag {
            source_path: PathBuf::new(),
            dag: serde_yaml::from_str(
                r#"
workspace: demo
dag_id: demo
slots:
  - id: my_slot
    state_machine:
      id: my_lifecycle
      predicate_bindings:
        - entity: external_entity
          source_kind: dag_entity
          scope: constellation-exposed external_ws.non_existent_slot
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
          green_when: "external_entity.state = APPROVED"
"#,
            )
            .expect("parses"),
        },
    )]);

    let context = DagValidationContext {
        known_entity_kinds: HashSet::new(),
    };
    let report = validate_dags_with_context(&loaded_dags, &context);

    // We expect an error because the target slot does not exist in the referenced workspace (or workspace doesn't exist).
    assert!(
        !report.errors.is_empty(),
        "Expected validation errors for referencing a non-existent external slot, but got none."
    );
}

#[test]
fn constellation_exposed_binding_malformed_scope_causes_warning() {
    let loaded_dags = BTreeMap::from([(
        "demo".to_string(),
        LoadedDag {
            source_path: PathBuf::new(),
            dag: serde_yaml::from_str(
                r#"
workspace: demo
dag_id: demo
slots:
  - id: my_slot
    state_machine:
      id: my_lifecycle
      predicate_bindings:
        - entity: external_entity
          source_kind: dag_entity
          scope: constellation-exposed malformed_scope_no_dot
      states:
        - id: PENDING
          entry: true
        - id: APPROVED
          green_when: "external_entity.state = APPROVED"
"#,
            )
            .expect("parses"),
        },
    )]);

    let context = DagValidationContext {
        known_entity_kinds: HashSet::new(),
    };
    let report = validate_dags_with_context(&loaded_dags, &context);

    assert!(report.warnings.iter().any(|warning| matches!(
        warning,
        DagWarning::MalformedBindingScope {
            slot_id,
            entity_kind,
            scope,
            ..
        } if slot_id == "my_slot" && entity_kind == "external_entity" && scope == "constellation-exposed malformed_scope_no_dot"
    )), "Expected MalformedBindingScope warning but got: {:?}", report.warnings);
}

use crate::config::green_when_coverage::{
    green_when_coverage_for_dag, green_when_coverage_summary, GreenWhenExclusionReason,
};
use crate::config::Dag;
use std::collections::HashSet;

#[test]
fn synthetic_coverage_excludes_entry_source_and_discretionary_destinations() {
    let yaml = r#"
version: 1.4
workspace: demo
dag_id: demo_dag
slots:
  - id: item
    state_machine:
      id: item_lifecycle
      states:
        - id: DRAFT
          entry: true
        - id: READY
          green_when: "review exists"
        - id: REJECTED
        - id: ARCHIVED
      transitions:
        - from: DRAFT
          to: READY
          via: item.ready
        - from: READY
          to: REJECTED
          via: item.reject
        - from: READY
          to: ARCHIVED
          via: item.archive
"#;
    let dag: Dag = serde_yaml::from_str(yaml).expect("synthetic DAG parses");
    let discretionary = HashSet::from(["item.reject".to_string()]);

    let rows = green_when_coverage_for_dag("demo", &dag, &discretionary);
    let draft = rows.iter().find(|row| row.state_id == "DRAFT").unwrap();
    let ready = rows.iter().find(|row| row.state_id == "READY").unwrap();
    let rejected = rows.iter().find(|row| row.state_id == "REJECTED").unwrap();
    let archived = rows.iter().find(|row| row.state_id == "ARCHIVED").unwrap();

    assert_eq!(
        draft.exclusion_reason,
        Some(GreenWhenExclusionReason::EntryState)
    );
    assert!(ready.candidate);
    assert!(ready.has_green_when);
    assert_eq!(
        rejected.exclusion_reason,
        Some(GreenWhenExclusionReason::DiscretionaryDestination)
    );
    assert!(archived.candidate);
    assert!(!archived.has_green_when);

    let summary = green_when_coverage_summary(&rows);
    assert_eq!(summary.candidate_states, 2);
    assert_eq!(summary.covered_candidate_states, 1);
    assert_eq!(summary.missing_candidate_states, 1);
}

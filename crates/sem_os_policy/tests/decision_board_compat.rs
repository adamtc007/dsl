use std::any::TypeId;

use sem_os_policy::decision_board as compatibility;
use semantic_decision_contracts as direct;

fn accepts_direct_board(_board: &direct::SemanticDecisionBoard) {}

#[test]
fn old_and_new_paths_are_the_same_types() {
    assert_eq!(
        TypeId::of::<direct::SemanticDecisionBoard>(),
        TypeId::of::<compatibility::SemanticDecisionBoard>()
    );
    assert_eq!(
        TypeId::of::<direct::ProposalWorkbook>(),
        TypeId::of::<compatibility::ProposalWorkbook>()
    );
    assert_eq!(
        TypeId::of::<direct::InferenceEvidence>(),
        TypeId::of::<compatibility::InferenceEvidence>()
    );
    assert_eq!(
        TypeId::of::<direct::ActionClass>(),
        TypeId::of::<sem_os_ontology::verb_contract::ActionClass>()
    );
    assert_eq!(
        TypeId::of::<direct::HarmClass>(),
        TypeId::of::<sem_os_ontology::verb_contract::HarmClass>()
    );
}

#[test]
fn compatibility_value_crosses_direct_api_without_conversion() {
    let board = compatibility::SemanticDecisionBoard::new(
        1,
        compatibility::DomainIdentity::new("compatibility-test").unwrap(),
        compatibility::SnapshotIdentity::new("snapshot-v1").unwrap(),
        compatibility::GraphRevision::new("revision-v1").unwrap(),
        compatibility::ResolvedPosition {
            anchor: None,
            context_hash: "context-v1".to_string(),
        },
        Vec::new(),
        "policy-v1".to_string(),
    )
    .unwrap();

    accepts_direct_board(&board);
}

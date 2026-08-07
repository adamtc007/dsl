use semantic_decision_contracts::{
    DesignFocus, FocusAbsenceReason, GameDomainId, MoveAttemptId, MoveAttemptOutcome,
};

fn main() {
    let domain = GameDomainId::new("consumer.domain").unwrap();
    let focus = DesignFocus::absent(FocusAbsenceReason::NotProvided, None).unwrap();
    let attempt = MoveAttemptId::new("attempt-1").unwrap();
    assert_eq!(domain.as_str(), "consumer.domain");
    assert!(matches!(focus, DesignFocus::Absent { .. }));
    assert_eq!(attempt.as_str(), "attempt-1");
    let _ = MoveAttemptOutcome::CompilerRefused;
}

use semantic_decision_contracts::{
    DesignFocus, FocusAbsenceReason, GameDisposition, GameDispositionKind, GameDomainId,
    MoveAttemptId, MoveAttemptOutcome, WorkbookPositionBinding,
};

fn position_bound_facades(_: &GameDisposition, _: &WorkbookPositionBinding) {}

fn main() {
    let domain = GameDomainId::new("consumer.domain").unwrap();
    let focus = DesignFocus::absent(FocusAbsenceReason::NotProvided, None).unwrap();
    let attempt = MoveAttemptId::new("attempt-1").unwrap();
    assert_eq!(domain.as_str(), "consumer.domain");
    assert!(matches!(focus, DesignFocus::Absent { .. }));
    assert_eq!(attempt.as_str(), "attempt-1");
    let _ = MoveAttemptOutcome::CompilerRefused;
    let _ = GameDispositionKind::ClarifyMoves;
    let _ = position_bound_facades;
}

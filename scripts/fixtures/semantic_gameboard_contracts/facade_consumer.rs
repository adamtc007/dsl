use semantic_decision_contracts::{
    DesignFocus, FocusAbsenceReason, GameDisposition, GameDispositionKind, GameDomainId,
    GameSessionId, GameTurnAdjudication, GameTurnAnswer, GameTurnAnswerAbsenceReason,
    GameTurnAttempt, GameTurnCompilerResult, GameTurnJudgement, GameTurnRecord, IntendedMove,
    MoveAttemptId, MoveAttemptOutcome, SemanticFamilyId, WorkbookPositionBinding,
};

fn position_bound_facades(_: &GameDisposition, _: &WorkbookPositionBinding) {}
fn evaluation_facades(_: &GameTurnRecord, _: &GameTurnAdjudication) {}

fn main() {
    let domain = GameDomainId::new("consumer.domain").unwrap();
    let focus = DesignFocus::absent(FocusAbsenceReason::NotProvided, None).unwrap();
    let attempt = MoveAttemptId::new("attempt-1").unwrap();
    let session = GameSessionId::new("session-1").unwrap();
    let family = SemanticFamilyId::new("family.example").unwrap();
    let answer = GameTurnAnswer::not_observed(GameTurnAnswerAbsenceReason::NotRequested);
    let compiler = GameTurnCompilerResult::not_requested();
    let turn_attempt = GameTurnAttempt::not_attempted();
    assert_eq!(domain.as_str(), "consumer.domain");
    assert!(matches!(focus, DesignFocus::Absent { .. }));
    assert_eq!(attempt.as_str(), "attempt-1");
    assert_eq!(session.as_str(), "session-1");
    assert_eq!(family.as_str(), "family.example");
    let _ = MoveAttemptOutcome::CompilerRefused;
    let _ = GameDispositionKind::ClarifyMoves;
    let _ = GameTurnJudgement::SystemMisinterpretation;
    let _ = IntendedMove::None;
    let _ = answer;
    let _ = compiler;
    let _ = turn_attempt;
    let _ = position_bound_facades;
    let _ = evaluation_facades;
}

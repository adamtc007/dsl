#![no_main]

mod support;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use semantic_decision_contracts::MoveAttemptReceipt;
use support::{
    attempt, exercise_hostile_and_round_trip, observe_attempt_outcome, reference_attempt_is_valid,
    ContractTape,
};

fuzz_target!(|data: &[u8]| {
    let mut input = Unstructured::new(data);
    if let Ok(tape) = ContractTape::arbitrary(&mut input) {
        let receipt = attempt(&tape);
        reference_attempt_is_valid(&tape, &receipt);
        observe_attempt_outcome(&receipt);
        exercise_hostile_and_round_trip::<MoveAttemptReceipt>(data, receipt);
    }
});

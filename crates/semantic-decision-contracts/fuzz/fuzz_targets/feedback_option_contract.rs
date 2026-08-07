#![no_main]

mod support;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use semantic_decision_contracts::FeedbackOption;
use support::{exercise_hostile_and_round_trip, feedback, observe_disclosure_class, ContractTape};

fuzz_target!(|data: &[u8]| {
    let mut input = Unstructured::new(data);
    if let Ok(tape) = ContractTape::arbitrary(&mut input) {
        let option = feedback(&tape);
        observe_disclosure_class(option.disclosure());
        exercise_hostile_and_round_trip::<FeedbackOption>(data, option);
    }
});

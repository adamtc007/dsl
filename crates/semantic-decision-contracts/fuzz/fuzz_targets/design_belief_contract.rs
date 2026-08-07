#![no_main]

mod support;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use semantic_decision_contracts::DesignBelief;
use support::{belief, exercise_hostile_and_round_trip, ContractTape};

fuzz_target!(|data: &[u8]| {
    let mut input = Unstructured::new(data);
    if let Ok(tape) = ContractTape::arbitrary(&mut input) {
        exercise_hostile_and_round_trip::<DesignBelief>(data, belief(&tape));
    }
});

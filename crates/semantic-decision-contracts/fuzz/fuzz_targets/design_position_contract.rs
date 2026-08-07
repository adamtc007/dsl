#![no_main]

mod support;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use semantic_decision_contracts::DesignPosition;
use support::{exercise_hostile_and_round_trip, position, ContractTape};

fuzz_target!(|data: &[u8]| {
    let mut input = Unstructured::new(data);
    if let Ok(tape) = ContractTape::arbitrary(&mut input) {
        exercise_hostile_and_round_trip::<DesignPosition>(data, position(&tape));
    }
});

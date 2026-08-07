#![no_main]

mod support;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use semantic_decision_contracts::GameDisposition;
use support::{disposition, exercise_hostile_and_round_trip, observe_disposition, ContractTape};

fuzz_target!(|data: &[u8]| {
    let mut input = Unstructured::new(data);
    if let Ok(tape) = ContractTape::arbitrary(&mut input) {
        let disposition = disposition(&tape);
        observe_disposition(&disposition);
        exercise_hostile_and_round_trip::<GameDisposition>(data, disposition);
    }
});

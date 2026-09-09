#![cfg(feature = "conformance")]
//! The reference store passes its own conformance suite, and the contract's
//! invariants hold under random write sequences.

use proptest::prelude::*;
use sem_os_append_store::{
    replay, AppendBatch, AppendError, AppendStore, MemoryStore, PieceId, PieceWrite, ScopeId, Seq,
    VersionNo,
};
use std::collections::BTreeMap;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn memory_store_passes_the_conformance_suite() {
    sem_os_append_store::conformance::run_all(MemoryStore::new).await;
}

#[tokio::test]
async fn replay_folds_in_sequence_order() {
    let store = MemoryStore::new();
    let scope = ScopeId::new("s").unwrap();
    for (version, payload) in [(1, "a"), (2, "b"), (3, "c")] {
        store
            .append(AppendBatch::single(
                scope.clone(),
                PieceWrite::new(
                    PieceId::new("p").unwrap(),
                    VersionNo::new(version).unwrap(),
                    payload.as_bytes(),
                ),
            ))
            .await
            .unwrap();
    }
    let joined = replay(&store, &scope, Seq::new(2), String::new(), |acc, record| {
        acc + std::str::from_utf8(&record.payload).unwrap()
    })
    .await
    .unwrap();
    assert_eq!(joined, "ab");
}

#[derive(Debug, Clone)]
struct Step {
    piece: u8,
    /// Offset from the correct next version: 0 = correct, negative = stale, positive = gap.
    offset: i8,
}

fn steps() -> impl Strategy<Value = Vec<Step>> {
    proptest::collection::vec(
        (0u8..4, -2i8..=2).prop_map(|(piece, offset)| Step { piece, offset }),
        1..64,
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn random_write_sequences_keep_the_contract(steps in steps()) {
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        runtime.block_on(async move {
            let store = MemoryStore::new();
            let scope = ScopeId::new("prop").unwrap();
            let mut model: BTreeMap<u8, u64> = BTreeMap::new();
            let mut admitted = 0u64;
            for step in steps {
                let current = model.get(&step.piece).copied();
                let expected = current.map_or(1, |v| v + 1);
                let attempted = (expected as i64 + step.offset as i64).max(1) as u64;
                let piece = PieceId::new(format!("piece-{}", step.piece)).unwrap();
                let write = PieceWrite::new(
                    piece.clone(),
                    VersionNo::new(attempted).unwrap(),
                    attempted.to_be_bytes(),
                );
                let result = store.append(AppendBatch::single(scope.clone(), write)).await;
                match result {
                    Ok(receipt) => {
                        prop_assert_eq!(attempted, expected);
                        admitted += 1;
                        prop_assert_eq!(receipt.first_seq.get(), admitted);
                        prop_assert_eq!(receipt.last_seq.get(), admitted);
                        model.insert(step.piece, attempted);
                    }
                    Err(AppendError::LostRace { attempted: a, current: c, .. }) => {
                        prop_assert!(attempted < expected);
                        prop_assert_eq!(a.get(), attempted);
                        prop_assert_eq!(c.map(VersionNo::get), current);
                    }
                    Err(AppendError::VersionGap { attempted: a, expected: e, .. }) => {
                        prop_assert!(attempted > expected);
                        prop_assert_eq!(a.get(), attempted);
                        prop_assert_eq!(e.get(), expected);
                    }
                    Err(other) => prop_assert!(false, "unexpected error {other}"),
                }
                let latest = store.current(&scope, &piece).await.unwrap();
                prop_assert_eq!(latest.map(|r| r.version_no.get()), model.get(&step.piece).copied());
            }
            let all = store.fold(&scope, None).await.unwrap();
            prop_assert_eq!(all.len() as u64, admitted);
            for (index, record) in all.iter().enumerate() {
                prop_assert_eq!(record.seq.get(), index as u64 + 1);
            }
            Ok(())
        })?;
    }
}

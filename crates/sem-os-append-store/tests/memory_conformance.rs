#![cfg(feature = "conformance")]
//! The reference store passes its own conformance suite for two different
//! identity types — bpmn-lite-style `String` ids and CA-style `uuid::Uuid`
//! ids — proving the contract and its suite impose no string-shaped
//! requirement on a store (ledger L30). The contract's invariants also hold
//! under random write sequences.

use proptest::prelude::*;
use sem_os_append_store::conformance::StringIds;
use sem_os_append_store::{
    replay, AppendBatch, AppendError, AppendStore, MemoryStore, PieceId, PieceWrite, ScopeId, Seq,
    VersionNo,
};
use std::collections::BTreeMap;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn memory_store_passes_the_conformance_suite_with_string_ids() {
    sem_os_append_store::conformance::run_all(MemoryStore::<String>::new, &StringIds).await;
}

#[cfg(feature = "uuid")]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn memory_store_passes_the_conformance_suite_with_uuid_ids() {
    use sem_os_append_store::conformance::UuidIds;
    use uuid::Uuid;

    sem_os_append_store::conformance::run_all(MemoryStore::<Uuid>::new, &UuidIds).await;
}

#[tokio::test]
async fn replay_folds_in_sequence_order() {
    let store = MemoryStore::<String>::new();
    let scope = ScopeId::new("s".to_owned());
    for (version, payload) in [(1, "a"), (2, "b"), (3, "c")] {
        store
            .append(AppendBatch::single(
                scope.clone(),
                PieceWrite::new(
                    PieceId::new("p".to_owned()),
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

#[test]
fn validated_string_ids_reject_empty_and_whitespace() {
    assert!(ScopeId::<String>::validated("").is_err());
    assert!(ScopeId::<String>::validated("has space").is_err());
    assert!(ScopeId::<String>::validated("ok-id").is_ok());
    assert!(PieceId::<String>::validated("").is_err());
    assert!(PieceId::<String>::validated("ok-id").is_ok());
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
            let store = MemoryStore::<String>::new();
            let scope = ScopeId::new("prop".to_owned());
            let mut model: BTreeMap<u8, u64> = BTreeMap::new();
            let mut admitted = 0u64;
            for step in steps {
                let current = model.get(&step.piece).copied();
                let expected = current.map_or(1, |v| v + 1);
                let attempted = (expected as i64 + step.offset as i64).max(1) as u64;
                let piece = PieceId::new(format!("piece-{}", step.piece));
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

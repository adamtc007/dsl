//! Conformance suite for [`AppendStore`] implementations.
//!
//! Every real store (Postgres or otherwise) must pass [`run_all`] against a
//! factory that returns a fresh, empty store. Each check is also exposed on
//! its own so a failing property can be isolated. Checks panic on violation,
//! so they are meant to be called from the implementer's test binary:
//!
//! ```ignore
//! #[tokio::test(flavor = "multi_thread")]
//! async fn my_store_conforms() {
//!     sem_os_append_store::conformance::run_all(|| MyStore::connect_fresh()).await;
//! }
//! ```

use std::sync::Arc;

use tokio::sync::Barrier;

use crate::{AppendBatch, AppendError, AppendStore, PieceId, PieceWrite, ScopeId, Seq, VersionNo};

fn scope(name: &str) -> ScopeId {
    ScopeId::new(name).expect("valid scope id")
}

fn piece(name: &str) -> PieceId {
    PieceId::new(name).expect("valid piece id")
}

fn write(name: &str, version: u64, payload: &str) -> PieceWrite {
    PieceWrite::new(
        piece(name),
        VersionNo::new(version).expect("non-zero version"),
        payload.as_bytes(),
    )
}

/// Run every conformance check, each on a fresh store from `new_store`.
pub async fn run_all<S, F>(new_store: F)
where
    S: AppendStore + 'static,
    F: Fn() -> S,
{
    append_assigns_contiguous_sequence(&new_store()).await;
    current_is_the_latest_version(&new_store()).await;
    fold_up_to_is_a_prefix(&new_store()).await;
    stale_version_loses_race(&new_store()).await;
    version_gap_is_rejected(&new_store()).await;
    batch_is_atomic(&new_store()).await;
    empty_and_duplicate_batches_are_rejected(&new_store()).await;
    scopes_are_independent(&new_store()).await;
    two_writers_one_piece_exactly_one_wins(Arc::new(new_store())).await;
}

/// Records receive contiguous, increasing sequence numbers in batch order,
/// starting at [`Seq::FIRST`].
pub async fn append_assigns_contiguous_sequence<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("seq");
    let first = store
        .append(
            AppendBatch::new(s.clone(), vec![write("a", 1, "a1"), write("b", 1, "b1")]).unwrap(),
        )
        .await
        .expect("first batch admitted");
    assert_eq!(first.first_seq, Seq::FIRST);
    assert_eq!(first.last_seq, Seq::new(2).unwrap());
    assert_eq!(first.written.len(), 2);
    assert_eq!(first.written[0].seq, Seq::FIRST);
    assert_eq!(first.written[1].seq, Seq::new(2).unwrap());

    let second = store
        .append(AppendBatch::single(s.clone(), write("a", 2, "a2")))
        .await
        .expect("second batch admitted");
    assert_eq!(second.first_seq, Seq::new(3).unwrap());
    assert_eq!(second.last_seq, Seq::new(3).unwrap());

    let records = store.fold(&s, None).await.unwrap();
    let seqs: Vec<u64> = records.iter().map(|r| r.seq.get()).collect();
    assert_eq!(
        seqs,
        vec![1, 2, 3],
        "sequence must be contiguous in append order"
    );
    assert_eq!(records[2].payload, b"a2");
}

/// `current` returns the highest admitted version of a piece.
pub async fn current_is_the_latest_version<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("current");
    assert_eq!(store.current(&s, &piece("p")).await.unwrap(), None);
    for version in 1..=3 {
        store
            .append(AppendBatch::single(
                s.clone(),
                write("p", version, &format!("v{version}")),
            ))
            .await
            .unwrap();
        let current = store
            .current(&s, &piece("p"))
            .await
            .unwrap()
            .expect("present");
        assert_eq!(current.version_no.get(), version);
        assert_eq!(current.payload, format!("v{version}").into_bytes());
    }
    assert_eq!(store.current(&s, &piece("other")).await.unwrap(), None);
}

/// `fold(scope, Some(n))` is exactly the prefix of `fold(scope, None)` with
/// `seq <= n`.
pub async fn fold_up_to_is_a_prefix<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("fold");
    for version in 1..=5u64 {
        store
            .append(AppendBatch::single(s.clone(), write("p", version, "x")))
            .await
            .unwrap();
    }
    let all = store.fold(&s, None).await.unwrap();
    assert_eq!(all.len(), 5);
    for limit in 0..=6u64 {
        let up_to = Seq::new(limit);
        let subset = store.fold(&s, up_to).await.unwrap();
        let expected: Vec<_> = all
            .iter()
            .filter(|r| up_to.is_none_or(|l| r.seq <= l))
            .cloned()
            .collect();
        assert_eq!(subset, expected, "fold up to {limit}");
    }
    assert!(store
        .fold(&scope("unknown"), None)
        .await
        .unwrap()
        .is_empty());
}

/// A write asserting a version at or below the current one is a typed
/// `LostRace` and leaves the store unchanged.
pub async fn stale_version_loses_race<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("race");
    store
        .append(AppendBatch::single(s.clone(), write("p", 1, "v1")))
        .await
        .unwrap();
    let err = store
        .append(AppendBatch::single(s.clone(), write("p", 1, "v1-again")))
        .await
        .expect_err("stale version rejected");
    assert_eq!(
        err,
        AppendError::LostRace {
            piece_id: piece("p"),
            attempted: VersionNo::FIRST,
            current: Some(VersionNo::FIRST),
        }
    );
    let current = store.current(&s, &piece("p")).await.unwrap().unwrap();
    assert_eq!(
        current.payload, b"v1",
        "loser must not overwrite the winner"
    );
    assert_eq!(store.fold(&s, None).await.unwrap().len(), 1);
}

/// A write that skips a version is a typed `VersionGap`.
pub async fn version_gap_is_rejected<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("gap");
    let err = store
        .append(AppendBatch::single(s.clone(), write("p", 2, "v2")))
        .await
        .expect_err("new piece must start at version 1");
    assert_eq!(
        err,
        AppendError::VersionGap {
            piece_id: piece("p"),
            attempted: VersionNo::new(2).unwrap(),
            expected: VersionNo::FIRST,
        }
    );
    store
        .append(AppendBatch::single(s.clone(), write("p", 1, "v1")))
        .await
        .unwrap();
    let err = store
        .append(AppendBatch::single(s.clone(), write("p", 3, "v3")))
        .await
        .expect_err("skipping version 2 rejected");
    assert!(matches!(err, AppendError::VersionGap { .. }));
    assert_eq!(store.fold(&s, None).await.unwrap().len(), 1);
}

/// A batch with one losing write admits none of its writes.
pub async fn batch_is_atomic<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("atomic");
    store
        .append(AppendBatch::single(s.clone(), write("taken", 1, "v1")))
        .await
        .unwrap();
    let err = store
        .append(
            AppendBatch::new(
                s.clone(),
                vec![write("fresh", 1, "f1"), write("taken", 1, "stale")],
            )
            .unwrap(),
        )
        .await
        .expect_err("batch with a stale write rejected");
    assert!(matches!(err, AppendError::LostRace { .. }));
    assert_eq!(store.current(&s, &piece("fresh")).await.unwrap(), None);
    assert_eq!(store.fold(&s, None).await.unwrap().len(), 1);
}

/// Malformed batches are rejected before any I/O.
pub async fn empty_and_duplicate_batches_are_rejected<S: AppendStore + ?Sized>(store: &S) {
    let s = scope("shape");
    assert_eq!(
        AppendBatch::new(s.clone(), vec![]).unwrap_err(),
        AppendError::EmptyBatch
    );
    assert_eq!(
        AppendBatch::new(s.clone(), vec![write("p", 1, "a"), write("p", 2, "b")]).unwrap_err(),
        AppendError::DuplicatePieceInBatch {
            piece_id: piece("p")
        }
    );
    assert!(store.fold(&s, None).await.unwrap().is_empty());
}

/// Sequence numbers and versions are per scope.
pub async fn scopes_are_independent<S: AppendStore + ?Sized>(store: &S) {
    let a = scope("scope-a");
    let b = scope("scope-b");
    store
        .append(AppendBatch::single(a.clone(), write("p", 1, "a")))
        .await
        .unwrap();
    let receipt = store
        .append(AppendBatch::single(b.clone(), write("p", 1, "b")))
        .await
        .unwrap();
    assert_eq!(receipt.first_seq, Seq::FIRST, "each scope sequences from 1");
    assert_eq!(
        store
            .current(&a, &piece("p"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"a"
    );
    assert_eq!(
        store
            .current(&b, &piece("p"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"b"
    );
}

/// Two writers released simultaneously on the same next version of one
/// piece: exactly one is admitted, the other gets `LostRace`, and the log
/// holds exactly one new record.
pub async fn two_writers_one_piece_exactly_one_wins<S: AppendStore + 'static>(store: Arc<S>) {
    let s = scope("concurrent");
    store
        .append(AppendBatch::single(s.clone(), write("p", 1, "base")))
        .await
        .unwrap();

    for round in 0..16u64 {
        let target = VersionNo::new(round + 2).unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let mut handles = Vec::new();
        for writer in ["left", "right"] {
            let store = Arc::clone(&store);
            let barrier = Arc::clone(&barrier);
            let s = s.clone();
            handles.push(tokio::spawn(async move {
                barrier.wait().await;
                store
                    .append(AppendBatch::single(
                        s,
                        PieceWrite::new(piece("p"), target, writer.as_bytes()),
                    ))
                    .await
            }));
        }
        let mut wins = 0;
        let mut losses = 0;
        for handle in handles {
            match handle.await.expect("writer task completes") {
                Ok(receipt) => {
                    wins += 1;
                    assert_eq!(receipt.written[0].version_no, target);
                }
                Err(AppendError::LostRace {
                    attempted, current, ..
                }) => {
                    losses += 1;
                    assert_eq!(attempted, target);
                    assert_eq!(current, Some(target));
                }
                Err(other) => panic!("unexpected error: {other}"),
            }
        }
        assert_eq!(
            (wins, losses),
            (1, 1),
            "round {round}: exactly one writer wins"
        );
        let current = store.current(&s, &piece("p")).await.unwrap().unwrap();
        assert_eq!(current.version_no, target);
        assert_eq!(
            store.fold(&s, None).await.unwrap().len() as u64,
            round + 2,
            "one record per admitted version"
        );
    }
}

//! Conformance suite for [`AppendStore`] implementations.
//!
//! Every real store (Postgres or otherwise) must pass [`run_all`] against a
//! factory that returns a fresh, empty store and an [`IdFactory`] for its
//! identity type. The suite makes no assumption about the shape of `Id`
//! (ledger L30) and does not assume sequence numbers are contiguous
//! (ledger L31 — see [`AppendStore::CONTIGUOUS`]). Each check is also
//! exposed on its own so a failing property can be isolated. Checks panic on
//! violation, so they are meant to be called from the implementer's test
//! binary:
//!
//! ```ignore
//! #[tokio::test(flavor = "multi_thread")]
//! async fn my_store_conforms() {
//!     sem_os_append_store::conformance::run_all(MyStore::connect_fresh, &StringIds).await;
//! }
//! ```

use std::sync::Arc;

use tokio::sync::Barrier;

use crate::{
    AppendBatch, AppendError, AppendStore, PieceId, PieceWrite, ScopeId, Seq, StoreId, VersionNo,
};

/// Produces distinct, stable identifiers of the store's identity type `Id`
/// for the conformance suite, so the suite makes no assumption about the
/// shape of `Id` (ledger L30). The same `label` always yields the same id
/// within one factory; distinct labels yield distinct ids.
pub trait IdFactory<Id>: Send + Sync {
    /// The identifier standing in for `label` in this run of the suite.
    fn id(&self, label: &str) -> Id;
}

/// An [`IdFactory`] for `Id = String`: the label, verbatim.
#[derive(Debug, Default, Clone, Copy)]
pub struct StringIds;

impl IdFactory<String> for StringIds {
    fn id(&self, label: &str) -> String {
        label.to_owned()
    }
}

#[cfg(feature = "uuid")]
mod uuid_ids {
    use super::IdFactory;
    use uuid::Uuid;

    /// An [`IdFactory`] for `Id = uuid::Uuid`: a deterministic name-based
    /// UUID (v5) derived from the label, so the same label is stable within
    /// one run and distinct labels never collide. Proof, alongside
    /// [`super::StringIds`], that the contract and its conformance suite
    /// impose no string-shaped requirement on a store's identity type
    /// (ledger L30).
    #[derive(Debug, Default, Clone, Copy)]
    pub struct UuidIds;

    impl IdFactory<Uuid> for UuidIds {
        fn id(&self, label: &str) -> Uuid {
            Uuid::new_v5(&Uuid::NAMESPACE_OID, label.as_bytes())
        }
    }
}
#[cfg(feature = "uuid")]
pub use uuid_ids::UuidIds;

fn scope<Id, G: IdFactory<Id> + ?Sized>(ids: &G, label: &str) -> ScopeId<Id> {
    ScopeId::new(ids.id(label))
}

fn piece<Id, G: IdFactory<Id> + ?Sized>(ids: &G, label: &str) -> PieceId<Id> {
    PieceId::new(ids.id(label))
}

fn write<Id, G: IdFactory<Id> + ?Sized>(
    ids: &G,
    label: &str,
    version: u64,
    payload: &str,
) -> PieceWrite<Id> {
    PieceWrite::new(
        piece(ids, label),
        VersionNo::new(version).expect("non-zero version"),
        payload.as_bytes(),
    )
}

/// Run every conformance check, each on a fresh store from `new_store`,
/// using `ids` to mint identifiers of the store's identity type.
pub async fn run_all<S, F, Id, G>(new_store: F, ids: &G)
where
    S: AppendStore<Id> + 'static,
    F: Fn() -> S,
    Id: StoreId,
    G: IdFactory<Id>,
{
    append_assigns_strictly_increasing_sequence(&new_store(), ids).await;
    contiguous_store_has_no_gaps(&new_store(), ids).await;
    current_is_the_latest_version(&new_store(), ids).await;
    fold_up_to_is_a_prefix(&new_store(), ids).await;
    stale_version_loses_race(&new_store(), ids).await;
    version_gap_is_rejected(&new_store(), ids).await;
    batch_is_atomic(&new_store(), ids).await;
    empty_and_duplicate_batches_are_rejected(&new_store(), ids).await;
    scopes_are_independent(&new_store(), ids).await;
    two_writers_one_piece_exactly_one_wins(Arc::new(new_store()), ids).await;
    two_writers_one_scope_may_race_without_serialising(Arc::new(new_store()), ids).await;
}

/// Records receive strictly increasing `Seq` values in batch order: a total
/// order per scope. Gaps are legal (ledger L31) — a store is not required to
/// start at [`Seq::FIRST`] or to leave no gaps between values; see
/// [`contiguous_store_has_no_gaps`] for the stronger, opt-in property.
pub async fn append_assigns_strictly_increasing_sequence<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "seq");
    let first = store
        .append(
            AppendBatch::new(
                s.clone(),
                vec![write(ids, "a", 1, "a1"), write(ids, "b", 1, "b1")],
            )
            .unwrap(),
        )
        .await
        .expect("first batch admitted");
    assert!(first.first_seq <= first.last_seq);
    assert_eq!(first.written.len(), 2);
    assert!(
        first.written[0].seq < first.written[1].seq,
        "writes within a batch are strictly increasing"
    );
    assert_eq!(first.written[0].seq, first.first_seq);
    assert_eq!(first.written[1].seq, first.last_seq);

    let second = store
        .append(AppendBatch::single(s.clone(), write(ids, "a", 2, "a2")))
        .await
        .expect("second batch admitted");
    assert!(
        second.first_seq > first.last_seq,
        "later batches sequence strictly after earlier ones"
    );
    assert_eq!(second.first_seq, second.last_seq);

    let records = store.fold(&s, None).await.unwrap();
    assert_eq!(records.len(), 3);
    let seqs: Vec<Seq> = records.iter().map(|r| r.seq).collect();
    let mut sorted = seqs.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        seqs, sorted,
        "fold returns a strictly increasing total order per scope, not just a set"
    );
    assert_eq!(records[2].payload, b"a2");
}

/// A store that declares [`AppendStore::CONTIGUOUS`] must not skip sequence
/// numbers within a scope. A store that does not declare it is exempt: the
/// base contract (see [`append_assigns_strictly_increasing_sequence`]) never
/// requires contiguity (ledger L31).
pub async fn contiguous_store_has_no_gaps<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    if !S::CONTIGUOUS {
        return;
    }
    let s = scope(ids, "contiguous");
    for (label, version) in [("a", 1), ("b", 1), ("a", 2)] {
        store
            .append(AppendBatch::single(
                s.clone(),
                write(ids, label, version, "x"),
            ))
            .await
            .unwrap();
    }
    let records = store.fold(&s, None).await.unwrap();
    let seqs: Vec<u64> = records.iter().map(|r| r.seq.get()).collect();
    assert_eq!(
        seqs,
        vec![1, 2, 3],
        "a store declaring CONTIGUOUS must not skip sequence numbers"
    );
}

/// `current` returns the highest admitted version of a piece.
pub async fn current_is_the_latest_version<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "current");
    assert_eq!(store.current(&s, &piece(ids, "p")).await.unwrap(), None);
    for version in 1..=3 {
        store
            .append(AppendBatch::single(
                s.clone(),
                write(ids, "p", version, &format!("v{version}")),
            ))
            .await
            .unwrap();
        let current = store
            .current(&s, &piece(ids, "p"))
            .await
            .unwrap()
            .expect("present");
        assert_eq!(current.version_no.get(), version);
        assert_eq!(current.payload, format!("v{version}").into_bytes());
    }
    assert_eq!(store.current(&s, &piece(ids, "other")).await.unwrap(), None);
}

/// `fold(scope, Some(n))` is exactly the prefix of `fold(scope, None)` with
/// `seq <= n`.
pub async fn fold_up_to_is_a_prefix<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "fold");
    for version in 1..=5u64 {
        store
            .append(AppendBatch::single(
                s.clone(),
                write(ids, "p", version, "x"),
            ))
            .await
            .unwrap();
    }
    let all = store.fold(&s, None).await.unwrap();
    assert_eq!(all.len(), 5);
    let max_seq = all.last().unwrap().seq.get();
    for limit in 0..=(max_seq + 1) {
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
        .fold(&scope(ids, "unknown"), None)
        .await
        .unwrap()
        .is_empty());
}

/// A write asserting a version at or below the current one is a typed
/// `LostRace` and leaves the store unchanged.
pub async fn stale_version_loses_race<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "race");
    store
        .append(AppendBatch::single(s.clone(), write(ids, "p", 1, "v1")))
        .await
        .unwrap();
    let err = store
        .append(AppendBatch::single(
            s.clone(),
            write(ids, "p", 1, "v1-again"),
        ))
        .await
        .expect_err("stale version rejected");
    assert_eq!(
        err,
        AppendError::LostRace {
            piece_id: piece(ids, "p"),
            attempted: VersionNo::FIRST,
            current: Some(VersionNo::FIRST),
        }
    );
    let current = store.current(&s, &piece(ids, "p")).await.unwrap().unwrap();
    assert_eq!(
        current.payload, b"v1",
        "loser must not overwrite the winner"
    );
    assert_eq!(store.fold(&s, None).await.unwrap().len(), 1);
}

/// A write that skips a version is a typed `VersionGap`.
pub async fn version_gap_is_rejected<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "gap");
    let err = store
        .append(AppendBatch::single(s.clone(), write(ids, "p", 2, "v2")))
        .await
        .expect_err("new piece must start at version 1");
    assert_eq!(
        err,
        AppendError::VersionGap {
            piece_id: piece(ids, "p"),
            attempted: VersionNo::new(2).unwrap(),
            expected: VersionNo::FIRST,
        }
    );
    store
        .append(AppendBatch::single(s.clone(), write(ids, "p", 1, "v1")))
        .await
        .unwrap();
    let err = store
        .append(AppendBatch::single(s.clone(), write(ids, "p", 3, "v3")))
        .await
        .expect_err("skipping version 2 rejected");
    assert!(matches!(err, AppendError::VersionGap { .. }));
    assert_eq!(store.fold(&s, None).await.unwrap().len(), 1);
}

/// A batch with one losing write admits none of its writes.
pub async fn batch_is_atomic<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "atomic");
    store
        .append(AppendBatch::single(s.clone(), write(ids, "taken", 1, "v1")))
        .await
        .unwrap();
    let err = store
        .append(
            AppendBatch::new(
                s.clone(),
                vec![
                    write(ids, "fresh", 1, "f1"),
                    write(ids, "taken", 1, "stale"),
                ],
            )
            .unwrap(),
        )
        .await
        .expect_err("batch with a stale write rejected");
    assert!(matches!(err, AppendError::LostRace { .. }));
    assert_eq!(store.current(&s, &piece(ids, "fresh")).await.unwrap(), None);
    assert_eq!(store.fold(&s, None).await.unwrap().len(), 1);
}

/// Malformed batches are rejected before any I/O.
pub async fn empty_and_duplicate_batches_are_rejected<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "shape");
    assert_eq!(
        AppendBatch::new(s.clone(), vec![]).unwrap_err(),
        AppendError::EmptyBatch
    );
    assert_eq!(
        AppendBatch::new(
            s.clone(),
            vec![write(ids, "p", 1, "a"), write(ids, "p", 2, "b")]
        )
        .unwrap_err(),
        AppendError::DuplicatePieceInBatch {
            piece_id: piece(ids, "p")
        }
    );
    assert!(store.fold(&s, None).await.unwrap().is_empty());
}

/// Sequence numbers and versions are per scope.
pub async fn scopes_are_independent<S, Id, G>(store: &S, ids: &G)
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let a = scope(ids, "scope-a");
    let b = scope(ids, "scope-b");
    store
        .append(AppendBatch::single(a.clone(), write(ids, "p", 1, "a")))
        .await
        .unwrap();
    store
        .append(AppendBatch::single(b.clone(), write(ids, "p", 1, "b")))
        .await
        .unwrap();
    assert_eq!(
        store
            .current(&a, &piece(ids, "p"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"a"
    );
    assert_eq!(
        store
            .current(&b, &piece(ids, "p"))
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
pub async fn two_writers_one_piece_exactly_one_wins<S, Id, G>(store: Arc<S>, ids: &G)
where
    S: AppendStore<Id> + 'static,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "concurrent");
    store
        .append(AppendBatch::single(s.clone(), write(ids, "p", 1, "base")))
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
            let piece_id = piece(ids, "p");
            handles.push(tokio::spawn(async move {
                barrier.wait().await;
                store
                    .append(AppendBatch::single(
                        s,
                        PieceWrite::new(piece_id, target, writer.as_bytes()),
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
        let current = store.current(&s, &piece(ids, "p")).await.unwrap().unwrap();
        assert_eq!(current.version_no, target);
        assert_eq!(
            store.fold(&s, None).await.unwrap().len() as u64,
            round + 2,
            "one record per admitted version"
        );
    }
}

/// Two writers append into the *same scope* but *different pieces*, released
/// simultaneously with no coordination beyond the store itself. Neither
/// write conflicts with the other, so both must be admitted; the suite does
/// not assume, and a store must not require, that the two were serialised
/// against each other to produce that outcome (ledger L31) — a store may
/// hand out sequence numbers from any source (e.g. a shared, lock-free
/// counter) as long as no two records share one and the resulting log is a
/// strictly increasing total order per scope. Contiguity is deliberately not
/// asserted here: a store built for intra-scope fan-out may leave a gap
/// where a third, concurrently active scope consumed an intervening value.
pub async fn two_writers_one_scope_may_race_without_serialising<S, Id, G>(store: Arc<S>, ids: &G)
where
    S: AppendStore<Id> + 'static,
    Id: StoreId,
    G: IdFactory<Id> + ?Sized,
{
    let s = scope(ids, "fan-out");
    let barrier = Arc::new(Barrier::new(2));
    let mut handles = Vec::new();
    for (label, payload) in [("left", "l1"), ("right", "r1")] {
        let store = Arc::clone(&store);
        let barrier = Arc::clone(&barrier);
        let scope_id = s.clone();
        let write = write(ids, label, 1, payload);
        handles.push(tokio::spawn(async move {
            barrier.wait().await;
            store.append(AppendBatch::single(scope_id, write)).await
        }));
    }
    let mut seqs = Vec::new();
    for handle in handles {
        let receipt = handle
            .await
            .expect("writer task completes")
            .expect("writes to distinct pieces never conflict");
        seqs.push(receipt.first_seq);
    }
    assert_ne!(
        seqs[0], seqs[1],
        "two admitted records must never share a sequence number"
    );

    let records = store.fold(&s, None).await.unwrap();
    assert_eq!(records.len(), 2);
    let mut folded: Vec<Seq> = records.iter().map(|r| r.seq).collect();
    let mut sorted = folded.clone();
    sorted.sort();
    assert_eq!(
        folded, sorted,
        "fold is a strictly increasing total order even when writers were not serialised by the caller"
    );
    folded.dedup();
    assert_eq!(folded.len(), 2, "no duplicate sequence numbers");
}

//! In-memory reference implementation of [`AppendStore`].

use std::collections::BTreeMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::{
    AppendBatch, AppendError, AppendReceipt, AppendStore, PieceId, PieceRecord, ScopeId, Seq,
    StoreId, VersionNo, WrittenPiece,
};

#[derive(Debug)]
struct ScopeLog<Id: StoreId> {
    records: Vec<PieceRecord<Id>>,
    /// Index into `records` of each piece's current version.
    current: BTreeMap<PieceId<Id>, usize>,
}

impl<Id: StoreId> Default for ScopeLog<Id> {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            current: BTreeMap::new(),
        }
    }
}

/// Thread-safe in-memory store, generic over the identity type `Id`. Every
/// append takes the store lock, so the contract's race semantics hold by
/// construction and sequence numbers happen to be contiguous
/// ([`AppendStore::CONTIGUOUS`] is `true`); the conformance suite verifies
/// both properties all the same.
#[derive(Debug)]
pub struct MemoryStore<Id: StoreId> {
    scopes: Mutex<BTreeMap<ScopeId<Id>, ScopeLog<Id>>>,
}

impl<Id: StoreId> Default for MemoryStore<Id> {
    fn default() -> Self {
        Self {
            scopes: Mutex::new(BTreeMap::new()),
        }
    }
}

impl<Id: StoreId> MemoryStore<Id> {
    /// An empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl<Id: StoreId> AppendStore<Id> for MemoryStore<Id> {
    const CONTIGUOUS: bool = true;

    async fn append(&self, batch: AppendBatch<Id>) -> Result<AppendReceipt<Id>, AppendError<Id>> {
        let (scope, writes) = batch.into_parts();
        let mut scopes = self
            .scopes
            .lock()
            .map_err(|_| AppendError::Backend("store lock poisoned".to_owned()))?;
        let log = scopes.entry(scope.clone()).or_default();

        // Validate every write against the current state before touching it:
        // the batch is all-or-nothing.
        for write in &writes {
            let current = log
                .current
                .get(&write.piece_id)
                .map(|&index| log.records[index].version_no);
            let expected = current.map_or(VersionNo::FIRST, VersionNo::next);
            if write.version_no < expected {
                return Err(AppendError::LostRace {
                    piece_id: write.piece_id.clone(),
                    attempted: write.version_no,
                    current,
                });
            }
            if write.version_no > expected {
                return Err(AppendError::VersionGap {
                    piece_id: write.piece_id.clone(),
                    attempted: write.version_no,
                    expected,
                });
            }
        }

        let first_seq = Seq::new(log.records.len() as u64 + 1).expect("non-zero");
        let mut seq = first_seq;
        let mut written = Vec::with_capacity(writes.len());
        for write in writes {
            let index = log.records.len();
            log.records.push(PieceRecord {
                scope: scope.clone(),
                piece_id: write.piece_id.clone(),
                version_no: write.version_no,
                seq,
                payload: write.payload,
            });
            log.current.insert(write.piece_id.clone(), index);
            written.push(WrittenPiece {
                piece_id: write.piece_id,
                version_no: write.version_no,
                seq,
            });
            seq = seq.next();
        }
        let last_seq = written.last().map(|w| w.seq).unwrap_or(first_seq);
        Ok(AppendReceipt {
            scope,
            first_seq,
            last_seq,
            written,
        })
    }

    async fn fold(
        &self,
        scope: &ScopeId<Id>,
        up_to: Option<Seq>,
    ) -> Result<Vec<PieceRecord<Id>>, AppendError<Id>> {
        let scopes = self
            .scopes
            .lock()
            .map_err(|_| AppendError::Backend("store lock poisoned".to_owned()))?;
        Ok(scopes
            .get(scope)
            .map(|log| {
                log.records
                    .iter()
                    .filter(|record| up_to.is_none_or(|limit| record.seq <= limit))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }

    async fn current(
        &self,
        scope: &ScopeId<Id>,
        piece_id: &PieceId<Id>,
    ) -> Result<Option<PieceRecord<Id>>, AppendError<Id>> {
        let scopes = self
            .scopes
            .lock()
            .map_err(|_| AppendError::Backend("store lock poisoned".to_owned()))?;
        Ok(scopes.get(scope).and_then(|log| {
            log.current
                .get(piece_id)
                .map(|&index| log.records[index].clone())
        }))
    }
}

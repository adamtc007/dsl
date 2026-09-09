//! In-memory reference implementation of [`AppendStore`].

use std::collections::BTreeMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::{
    AppendBatch, AppendError, AppendReceipt, AppendStore, PieceId, PieceRecord, ScopeId, Seq,
    VersionNo, WrittenPiece,
};

#[derive(Debug, Default)]
struct ScopeLog {
    records: Vec<PieceRecord>,
    /// Index into `records` of each piece's current version.
    current: BTreeMap<PieceId, usize>,
}

/// Thread-safe in-memory store. Every append takes the store lock, so the
/// contract's race semantics hold by construction; the conformance suite
/// verifies them all the same.
#[derive(Debug, Default)]
pub struct MemoryStore {
    scopes: Mutex<BTreeMap<ScopeId, ScopeLog>>,
}

impl MemoryStore {
    /// An empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl AppendStore for MemoryStore {
    async fn append(&self, batch: AppendBatch) -> Result<AppendReceipt, AppendError> {
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
        scope: &ScopeId,
        up_to: Option<Seq>,
    ) -> Result<Vec<PieceRecord>, AppendError> {
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
        scope: &ScopeId,
        piece_id: &PieceId,
    ) -> Result<Option<PieceRecord>, AppendError> {
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

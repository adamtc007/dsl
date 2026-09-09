//! Conditional-append store contract for versioned pieces.
//!
//! A *piece* is an identified thing that changes only by appending a new
//! version. A *scope* is an ordered log of piece versions (a stream). Every
//! write asserts the version it intends to create; the store admits it only
//! if that is exactly the next version of the piece, so two writers racing
//! on one piece cannot both succeed and no version is ever skipped or
//! overwritten.
//!
//! The crate is domain-free: identifiers are opaque validated strings and
//! payloads are opaque bytes. No SQL lives here; [`MemoryStore`] is the
//! reference implementation and the [`conformance`] suite (feature
//! `conformance`) is what a real store must pass.
#![deny(unreachable_pub)]

use std::fmt;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod memory;

#[cfg(feature = "conformance")]
pub mod conformance;

pub use memory::MemoryStore;

/// Upper bound on identifier length in bytes.
pub const MAX_ID_LEN: usize = 256;

// ---------------------------------------------------------------------------
// Identifiers
// ---------------------------------------------------------------------------

/// Failure to construct a store identifier.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("invalid {kind} `{value}`: {reason}")]
pub struct IdError {
    pub kind: &'static str,
    pub value: String,
    pub reason: &'static str,
}

fn validate_id(kind: &'static str, value: &str) -> Result<(), IdError> {
    let invalid = |reason| IdError {
        kind,
        value: value.to_owned(),
        reason,
    };
    if value.is_empty() {
        return Err(invalid("must not be empty"));
    }
    if value.len() > MAX_ID_LEN {
        return Err(invalid("exceeds the maximum length"));
    }
    if value.chars().any(|c| c.is_control() || c.is_whitespace()) {
        return Err(invalid("must not contain whitespace or control characters"));
    }
    Ok(())
}

macro_rules! store_id {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("Validated ", $kind, ": non-empty, bounded, no whitespace or control characters.")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Construct a validated ", $kind, ".")]
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                validate_id($kind, &value)?;
                Ok(Self(value))
            }

            #[doc = concat!("Borrow the ", $kind, ".")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

store_id!(ScopeId, "scope id");
store_id!(PieceId, "piece id");

/// Version number of a piece; the first version is 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VersionNo(u64);

impl VersionNo {
    /// The version a new piece is created at.
    pub const FIRST: Self = Self(1);

    /// A version number; zero is not a version.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// The numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The version that follows this one.
    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for VersionNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Store-assigned position of a record in its scope; the first record is 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Seq(u64);

impl Seq {
    /// The first sequence number in a scope.
    pub const FIRST: Self = Self(1);

    /// A sequence number; zero is not a position.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// The numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The position that follows this one.
    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Writes and records
// ---------------------------------------------------------------------------

/// One intended piece version. `version_no` is the caller's assertion of the
/// version being created: [`VersionNo::FIRST`] for a new piece, otherwise the
/// current version plus one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PieceWrite {
    pub piece_id: PieceId,
    pub version_no: VersionNo,
    pub payload: Vec<u8>,
}

impl PieceWrite {
    /// A write of `payload` as version `version_no` of `piece_id`.
    #[must_use]
    pub fn new(piece_id: PieceId, version_no: VersionNo, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            piece_id,
            version_no,
            payload: payload.into(),
        }
    }
}

/// An atomic batch of writes to one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppendBatch {
    scope: ScopeId,
    writes: Vec<PieceWrite>,
}

impl AppendBatch {
    /// Build a batch. The batch must be non-empty and name each piece at most
    /// once (two versions of one piece belong in two batches).
    pub fn new(scope: ScopeId, writes: Vec<PieceWrite>) -> Result<Self, AppendError> {
        if writes.is_empty() {
            return Err(AppendError::EmptyBatch);
        }
        let mut seen = std::collections::BTreeSet::new();
        for write in &writes {
            if !seen.insert(&write.piece_id) {
                return Err(AppendError::DuplicatePieceInBatch {
                    piece_id: write.piece_id.clone(),
                });
            }
        }
        Ok(Self { scope, writes })
    }

    /// A batch of one write.
    pub fn single(scope: ScopeId, write: PieceWrite) -> Self {
        Self {
            scope,
            writes: vec![write],
        }
    }

    /// The scope written to.
    #[must_use]
    pub fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// The writes, in the order they will be sequenced.
    #[must_use]
    pub fn writes(&self) -> &[PieceWrite] {
        &self.writes
    }

    /// Consume the batch.
    #[must_use]
    pub fn into_parts(self) -> (ScopeId, Vec<PieceWrite>) {
        (self.scope, self.writes)
    }
}

/// One admitted piece version in a scope log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PieceRecord {
    pub scope: ScopeId,
    pub piece_id: PieceId,
    pub version_no: VersionNo,
    pub seq: Seq,
    pub payload: Vec<u8>,
}

/// One admitted write within a receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrittenPiece {
    pub piece_id: PieceId,
    pub version_no: VersionNo,
    pub seq: Seq,
}

/// Result of an admitted batch: the contiguous sequence range it occupies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppendReceipt {
    pub scope: ScopeId,
    pub first_seq: Seq,
    pub last_seq: Seq,
    pub written: Vec<WrittenPiece>,
}

/// Typed append failure. `LostRace` and `VersionGap` are contract outcomes,
/// not backend faults; a batch that fails leaves the store unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppendError {
    #[error("lost race on piece `{piece_id}`: attempted version {attempted}, current is {}", current.map_or("none".to_owned(), |v| v.to_string()))]
    LostRace {
        piece_id: PieceId,
        attempted: VersionNo,
        current: Option<VersionNo>,
    },
    #[error(
        "version gap on piece `{piece_id}`: attempted version {attempted}, expected {expected}"
    )]
    VersionGap {
        piece_id: PieceId,
        attempted: VersionNo,
        expected: VersionNo,
    },
    #[error("batch must contain at least one write")]
    EmptyBatch,
    #[error("batch names piece `{piece_id}` more than once")]
    DuplicatePieceInBatch { piece_id: PieceId },
    #[error("store backend failed: {0}")]
    Backend(String),
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

/// The conditional-append store contract.
///
/// Implementations must guarantee, per scope:
///
/// 1. `append` admits a batch only if every write's `version_no` is exactly
///    the next version of its piece (`FIRST` for an unseen piece); otherwise
///    it returns [`AppendError::LostRace`] (stale) or
///    [`AppendError::VersionGap`] (skipped) and writes nothing.
/// 2. Admitted records receive contiguous, increasing sequence numbers in
///    batch order, starting at [`Seq::FIRST`], and a batch's records are
///    contiguous.
/// 3. `fold` returns records in sequence order, all of them when `up_to` is
///    `None`, otherwise those with `seq <= up_to`.
/// 4. `current` returns the record with the highest version of the piece.
/// 5. Concurrent appends racing on one piece admit exactly one of them.
#[async_trait]
pub trait AppendStore: Send + Sync {
    /// Atomically append a batch of conditional writes.
    async fn append(&self, batch: AppendBatch) -> Result<AppendReceipt, AppendError>;

    /// The ordered replay set of a scope up to and including `up_to`.
    async fn fold(
        &self,
        scope: &ScopeId,
        up_to: Option<Seq>,
    ) -> Result<Vec<PieceRecord>, AppendError>;

    /// The latest version of one piece, if it exists.
    async fn current(
        &self,
        scope: &ScopeId,
        piece_id: &PieceId,
    ) -> Result<Option<PieceRecord>, AppendError>;
}

/// Fold a scope's replay set with `step`, starting from `init`.
pub async fn replay<S, T, F>(
    store: &S,
    scope: &ScopeId,
    up_to: Option<Seq>,
    init: T,
    step: F,
) -> Result<T, AppendError>
where
    S: AppendStore + ?Sized,
    F: FnMut(T, &PieceRecord) -> T,
{
    let records = store.fold(scope, up_to).await?;
    Ok(records.iter().fold(init, step))
}

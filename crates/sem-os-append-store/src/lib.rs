//! Conditional-append store contract for versioned pieces.
//!
//! A *piece* is an identified thing that changes only by appending a new
//! version. A *scope* is an ordered log of piece versions (a stream). Every
//! write asserts the version it intends to create; the store admits it only
//! if that is exactly the next version of the piece, so two writers racing
//! on one piece cannot both succeed and no version is ever skipped or
//! overwritten.
//!
//! The crate is domain-free: identifiers are generic over the store's own
//! identity type ([`StoreId`]) and payloads are opaque bytes. No SQL lives
//! here; [`MemoryStore`] is the reference implementation and the
//! [`conformance`] suite (feature `conformance`) is what a real store must
//! pass, for whichever identity type it uses.
#![deny(unreachable_pub)]

use std::fmt;
use std::hash::Hash;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod memory;

#[cfg(feature = "conformance")]
pub mod conformance;

pub use memory::MemoryStore;

/// Upper bound on identifier length in bytes, for the validated `String`
/// identity type ([`ScopeId::validated`], [`PieceId::validated`]).
pub const MAX_ID_LEN: usize = 256;

// ---------------------------------------------------------------------------
// Identity
// ---------------------------------------------------------------------------

/// A type usable as a store's identity type: cloneable, totally ordered,
/// hashable, printable, and safe to hold across an `await`.
///
/// This is a capability marker, not a contract to implement — it is
/// blanket-implemented for every type that already has these properties, so
/// `String`, [`uuid::Uuid`](https://docs.rs/uuid) (with the `uuid` feature),
/// or a domain's own identifier newtype all qualify with no extra code. A
/// store is not required to choose string keys to satisfy this crate's
/// contract or its conformance suite (ledger L30).
pub trait StoreId:
    Clone + Eq + Ord + Hash + fmt::Debug + fmt::Display + Send + Sync + 'static
{
}

impl<T> StoreId for T where
    T: Clone + Eq + Ord + Hash + fmt::Debug + fmt::Display + Send + Sync + 'static
{
}

/// Failure to construct a validated `String` store identifier
/// ([`ScopeId::validated`], [`PieceId::validated`]). Identity types other
/// than `String` are not validated here — validity is whatever the type
/// itself enforces.
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
        #[doc = concat!("A ", $kind, ", generic over the store's identity type `Id`.")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name<Id>(Id);

        impl<Id> $name<Id> {
            #[doc = concat!("Wrap an identifier of the store's identity type as a ", $kind, ". No validation is performed; see `validated` for the `Id = String` case.")]
            pub fn new(id: Id) -> Self {
                Self(id)
            }

            #[doc = concat!("Borrow the underlying identifier of this ", $kind, ".")]
            #[must_use]
            pub fn get(&self) -> &Id {
                &self.0
            }

            #[doc = concat!("Consume the ", $kind, ", returning the underlying identifier.")]
            #[must_use]
            pub fn into_inner(self) -> Id {
                self.0
            }
        }

        impl $name<String> {
            #[doc = concat!("Construct a validated ", $kind, " from a string: non-empty, bounded, no whitespace or control characters.")]
            pub fn validated(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                validate_id($kind, &value)?;
                Ok(Self(value))
            }

            #[doc = concat!("Borrow the ", $kind, " as a string.")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<Id: fmt::Display> fmt::Display for $name<Id> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
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

/// Store-assigned position of a record within its scope: a total order, not
/// necessarily a contiguous one.
///
/// Records in one scope receive strictly increasing `Seq` values in append
/// order — that is the entire ordering contract. Gaps between values are
/// legal and must not be relied upon to be absent: a store built on a
/// shared, lock-free counter (so that unrelated scopes can be appended to
/// concurrently with no per-scope lock) will typically leave gaps where
/// another scope's writes consumed intervening values. A store that does
/// guarantee contiguity may say so via [`AppendStore::CONTIGUOUS`] (ledger
/// L31); callers must not assume it of a store that does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Seq(u64);

impl Seq {
    /// The smallest legal sequence number. Not every scope's first record
    /// need be assigned this value — see the type documentation.
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
pub struct PieceWrite<Id> {
    pub piece_id: PieceId<Id>,
    pub version_no: VersionNo,
    pub payload: Vec<u8>,
}

impl<Id> PieceWrite<Id> {
    /// A write of `payload` as version `version_no` of `piece_id`.
    #[must_use]
    pub fn new(piece_id: PieceId<Id>, version_no: VersionNo, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            piece_id,
            version_no,
            payload: payload.into(),
        }
    }
}

/// An atomic batch of writes to one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppendBatch<Id> {
    scope: ScopeId<Id>,
    writes: Vec<PieceWrite<Id>>,
}

impl<Id: StoreId> AppendBatch<Id> {
    /// Build a batch. The batch must be non-empty and name each piece at most
    /// once (two versions of one piece belong in two batches).
    pub fn new(scope: ScopeId<Id>, writes: Vec<PieceWrite<Id>>) -> Result<Self, AppendError<Id>> {
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
    pub fn single(scope: ScopeId<Id>, write: PieceWrite<Id>) -> Self {
        Self {
            scope,
            writes: vec![write],
        }
    }

    /// The scope written to.
    #[must_use]
    pub fn scope(&self) -> &ScopeId<Id> {
        &self.scope
    }

    /// The writes, in the order they will be sequenced.
    #[must_use]
    pub fn writes(&self) -> &[PieceWrite<Id>] {
        &self.writes
    }

    /// Consume the batch.
    #[must_use]
    pub fn into_parts(self) -> (ScopeId<Id>, Vec<PieceWrite<Id>>) {
        (self.scope, self.writes)
    }
}

/// One admitted piece version in a scope log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PieceRecord<Id> {
    pub scope: ScopeId<Id>,
    pub piece_id: PieceId<Id>,
    pub version_no: VersionNo,
    pub seq: Seq,
    pub payload: Vec<u8>,
}

/// One admitted write within a receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrittenPiece<Id> {
    pub piece_id: PieceId<Id>,
    pub version_no: VersionNo,
    pub seq: Seq,
}

/// Result of an admitted batch: the sequence range it occupies. The range is
/// not necessarily contiguous with a prior batch's range — see [`Seq`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppendReceipt<Id> {
    pub scope: ScopeId<Id>,
    pub first_seq: Seq,
    pub last_seq: Seq,
    pub written: Vec<WrittenPiece<Id>>,
}

/// Typed append failure. `LostRace` and `VersionGap` are contract outcomes,
/// not backend faults; a batch that fails leaves the store unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppendError<Id: StoreId> {
    #[error("lost race on piece `{piece_id}`: attempted version {attempted}, current is {}", current.map_or("none".to_owned(), |v| v.to_string()))]
    LostRace {
        piece_id: PieceId<Id>,
        attempted: VersionNo,
        current: Option<VersionNo>,
    },
    #[error(
        "version gap on piece `{piece_id}`: attempted version {attempted}, expected {expected}"
    )]
    VersionGap {
        piece_id: PieceId<Id>,
        attempted: VersionNo,
        expected: VersionNo,
    },
    #[error("batch must contain at least one write")]
    EmptyBatch,
    #[error("batch names piece `{piece_id}` more than once")]
    DuplicatePieceInBatch { piece_id: PieceId<Id> },
    #[error("store backend failed: {0}")]
    Backend(String),
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

/// The conditional-append store contract, generic over the store's identity
/// type `Id` (ledger L30 — a store must not have to adopt string keys to
/// implement this trait or pass its conformance suite).
///
/// Implementations must guarantee, per scope:
///
/// 1. `append` admits a batch only if every write's `version_no` is exactly
///    the next version of its piece (`FIRST` for an unseen piece); otherwise
///    it returns [`AppendError::LostRace`] (stale) or
///    [`AppendError::VersionGap`] (skipped) and writes nothing.
/// 2. Admitted records receive strictly increasing `Seq` values in batch
///    order: a total order per scope, not necessarily a contiguous one
///    (ledger L31 — see [`Seq`] and [`CONTIGUOUS`](Self::CONTIGUOUS)).
/// 3. `fold` returns records in sequence order, all of them when `up_to` is
///    `None`, otherwise those with `seq <= up_to`.
/// 4. `current` returns the record with the highest version of the piece.
/// 5. Concurrent appends racing on one piece admit exactly one of them.
///    Concurrent appends to different pieces — even in the same scope — are
///    not required to be serialised against each other.
#[async_trait]
pub trait AppendStore<Id: StoreId>: Send + Sync {
    /// Whether this store additionally guarantees `Seq` is contiguous
    /// (no gaps) within a scope. Default `false`: a store opts in only if it
    /// actually provides the stronger guarantee, since it usually costs a
    /// per-scope lock or sequence and forecloses concurrent, unserialised
    /// writers into one scope (ledger L31).
    const CONTIGUOUS: bool = false;

    /// Atomically append a batch of conditional writes.
    async fn append(&self, batch: AppendBatch<Id>) -> Result<AppendReceipt<Id>, AppendError<Id>>;

    /// The ordered replay set of a scope up to and including `up_to`.
    async fn fold(
        &self,
        scope: &ScopeId<Id>,
        up_to: Option<Seq>,
    ) -> Result<Vec<PieceRecord<Id>>, AppendError<Id>>;

    /// The latest version of one piece, if it exists.
    async fn current(
        &self,
        scope: &ScopeId<Id>,
        piece_id: &PieceId<Id>,
    ) -> Result<Option<PieceRecord<Id>>, AppendError<Id>>;
}

/// Fold a scope's replay set with `step`, starting from `init`.
pub async fn replay<S, Id, T, F>(
    store: &S,
    scope: &ScopeId<Id>,
    up_to: Option<Seq>,
    init: T,
    step: F,
) -> Result<T, AppendError<Id>>
where
    S: AppendStore<Id> + ?Sized,
    Id: StoreId,
    F: FnMut(T, &PieceRecord<Id>) -> T,
{
    let records = store.fold(scope, up_to).await?;
    Ok(records.iter().fold(init, step))
}

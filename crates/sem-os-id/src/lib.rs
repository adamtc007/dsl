//! Pure identity helpers: RFC 9562 UUID v7 with monotonic-within-millisecond
//! ordering, and deterministic UUID v8 for replay-stable identities.
//!
//! Nothing here reads a clock or a random source. The caller supplies a
//! [`UnixMillis`] and, for v7, an [`Entropy`] value; the same inputs always
//! produce the same outputs, which is what makes the generator testable and
//! replayable.
//!
//! # v7 layout (RFC 9562 §5.7, counter method 1 of §6.2)
//!
//! ```text
//! unix_ts_ms (48) | ver=7 (4) | counter_hi (12) | var=10 (2) | counter_lo (30) | random (32)
//! ```
//!
//! The 42-bit counter is seeded from entropy on every new millisecond with its
//! top bit clear, then incremented for each identifier issued in the same
//! millisecond. If the counter would overflow, the timestamp advances by one
//! millisecond and the counter is reseeded. A caller clock that stands still or
//! runs backwards is clamped to the last issued millisecond, so identifiers
//! from one generator are always strictly increasing.
//!
//! # v8 layout (RFC 9562 §5.8)
//!
//! ```text
//! unix_ts_ms (48) | ver=8 (4) | hash (12) | var=10 (2) | hash (62)
//! ```
//!
//! The 74 hash bits are the leading bits of a SHA-256 over a domain tag, the
//! caller's namespace and each input, every part length-prefixed so that
//! `["ab", "c"]` and `["a", "bc"]` never collide by construction.
#![deny(unreachable_pub)]

use std::fmt;

use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::{Builder, Uuid, Variant, Version};

/// Largest millisecond timestamp that fits the 48-bit UUID field.
pub const MAX_UNIX_MILLIS: u64 = (1 << 48) - 1;

/// Number of counter bits in a v7 identifier.
pub const COUNTER_BITS: u32 = 42;

const COUNTER_MAX: u64 = (1 << COUNTER_BITS) - 1;
const V8_DOMAIN_TAG: &[u8] = b"sem-os-id/v8\0";

/// Failure to issue an identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum IdError {
    #[error("unix millisecond timestamp {unix_ms} exceeds the 48-bit UUID field")]
    TimestampOutOfRange { unix_ms: u64 },
    #[error("UUID {uuid} is not version {expected}")]
    WrongVersion { uuid: Uuid, expected: u8 },
}

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

/// A millisecond Unix timestamp that fits the 48-bit UUID field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u64", into = "u64"))]
pub struct UnixMillis(u64);

impl UnixMillis {
    /// The epoch.
    pub const ZERO: Self = Self(0);
    /// The largest representable timestamp.
    pub const MAX: Self = Self(MAX_UNIX_MILLIS);

    /// Validate a millisecond timestamp.
    pub const fn new(unix_ms: u64) -> Result<Self, IdError> {
        if unix_ms > MAX_UNIX_MILLIS {
            Err(IdError::TimestampOutOfRange { unix_ms })
        } else {
            Ok(Self(unix_ms))
        }
    }

    /// The millisecond value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for UnixMillis {
    type Error = IdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<UnixMillis> for u64 {
    fn from(value: UnixMillis) -> Self {
        value.0
    }
}

impl fmt::Display for UnixMillis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Eighty bits of caller-supplied randomness for one v7 identifier.
///
/// The top 41 bits seed the counter on a new millisecond; the low 32 bits
/// fill the random tail of every identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entropy(pub [u8; 10]);

impl Entropy {
    /// All-zero entropy. Valid but predictable; useful only for tests.
    pub const ZERO: Self = Self([0; 10]);

    fn counter_seed(self) -> u64 {
        let mut hi = [0u8; 8];
        hi[2..].copy_from_slice(&self.0[..6]);
        // 48 bits available; keep 41 so the counter has ≥ 2^41 increments left.
        (u64::from_be_bytes(hi) >> 7) & ((1 << (COUNTER_BITS - 1)) - 1)
    }

    fn tail(self) -> u32 {
        u32::from_be_bytes([self.0[6], self.0[7], self.0[8], self.0[9]])
    }
}

// ---------------------------------------------------------------------------
// v7
// ---------------------------------------------------------------------------

/// A version 7 identifier issued by a [`TimeOrderedGenerator`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct TimeOrderedId(Uuid);

impl TimeOrderedId {
    /// Borrow the UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Unwrap the UUID.
    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }

    /// The millisecond the identifier was issued for.
    #[must_use]
    pub fn unix_ms(&self) -> UnixMillis {
        UnixMillis((self.0.as_u128() >> 80) as u64)
    }

    /// The 42-bit counter value.
    #[must_use]
    pub fn counter(&self) -> u64 {
        let bits = self.0.as_u128();
        let hi = ((bits >> 64) & 0x0fff) as u64;
        let lo = ((bits >> 32) & 0x3fff_ffff) as u64;
        (hi << 30) | lo
    }
}

impl TryFrom<Uuid> for TimeOrderedId {
    type Error = IdError;

    fn try_from(uuid: Uuid) -> Result<Self, Self::Error> {
        if uuid.get_version_num() == 7 && uuid.get_variant() == Variant::RFC4122 {
            Ok(Self(uuid))
        } else {
            Err(IdError::WrongVersion { uuid, expected: 7 })
        }
    }
}

impl From<TimeOrderedId> for Uuid {
    fn from(value: TimeOrderedId) -> Self {
        value.0
    }
}

impl fmt::Display for TimeOrderedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Issues strictly increasing v7 identifiers from caller-supplied time and
/// entropy. One generator per writer; the generator carries the state that
/// makes same-millisecond ordering possible.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TimeOrderedGenerator {
    state: Option<(u64, u64)>,
}

impl TimeOrderedGenerator {
    /// A generator that has issued nothing.
    #[must_use]
    pub const fn new() -> Self {
        Self { state: None }
    }

    /// The `(unix_ms, counter)` of the last issued identifier, if any.
    #[must_use]
    pub const fn last(&self) -> Option<(u64, u64)> {
        self.state
    }

    /// Issue the next identifier for clock reading `now` using `entropy`.
    ///
    /// Errors only when the timestamp field is exhausted (the counter
    /// overflowed at [`UnixMillis::MAX`]).
    pub fn next(&mut self, now: UnixMillis, entropy: Entropy) -> Result<TimeOrderedId, IdError> {
        let (ms, counter) = match self.state {
            None => (now.0, entropy.counter_seed()),
            Some((last_ms, _)) if now.0 > last_ms => (now.0, entropy.counter_seed()),
            Some((last_ms, last_counter)) => {
                if last_counter < COUNTER_MAX {
                    (last_ms, last_counter + 1)
                } else if last_ms < MAX_UNIX_MILLIS {
                    (last_ms + 1, entropy.counter_seed())
                } else {
                    return Err(IdError::TimestampOutOfRange {
                        unix_ms: last_ms + 1,
                    });
                }
            }
        };
        self.state = Some((ms, counter));
        Ok(TimeOrderedId(build_v7(ms, counter, entropy.tail())))
    }
}

fn build_v7(unix_ms: u64, counter: u64, tail: u32) -> Uuid {
    debug_assert!(unix_ms <= MAX_UNIX_MILLIS);
    debug_assert!(counter <= COUNTER_MAX);
    let counter_hi = (counter >> 30) & 0x0fff;
    let counter_lo = counter & 0x3fff_ffff;
    let bits: u128 = (u128::from(unix_ms) << 80)
        | (u128::from(counter_hi) << 64)
        | (u128::from(counter_lo) << 32)
        | u128::from(tail);
    Builder::from_u128(bits)
        .with_version(Version::SortRand)
        .with_variant(Variant::RFC4122)
        .into_uuid()
}

// ---------------------------------------------------------------------------
// v8
// ---------------------------------------------------------------------------

/// A deterministic version 8 identifier produced by [`derive_id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DerivedId(Uuid);

impl DerivedId {
    /// Borrow the UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Unwrap the UUID.
    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }

    /// The millisecond prefix the identifier was derived for.
    #[must_use]
    pub fn unix_ms(&self) -> UnixMillis {
        UnixMillis((self.0.as_u128() >> 80) as u64)
    }
}

impl TryFrom<Uuid> for DerivedId {
    type Error = IdError;

    fn try_from(uuid: Uuid) -> Result<Self, Self::Error> {
        if uuid.get_version_num() == 8 && uuid.get_variant() == Variant::RFC4122 {
            Ok(Self(uuid))
        } else {
            Err(IdError::WrongVersion { uuid, expected: 8 })
        }
    }
}

impl From<DerivedId> for Uuid {
    fn from(value: DerivedId) -> Self {
        value.0
    }
}

impl fmt::Display for DerivedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Derive a replay-stable v8 identifier from a millisecond prefix, a
/// namespace and an ordered list of inputs.
///
/// The same `(at, namespace, inputs)` always yields the same identifier.
/// Different namespaces, or the same bytes split differently across inputs,
/// yield different identifiers.
pub fn derive_id<I, B>(at: UnixMillis, namespace: &str, inputs: I) -> DerivedId
where
    I: IntoIterator<Item = B>,
    B: AsRef<[u8]>,
{
    let mut hasher = Sha256::new();
    hasher.update(V8_DOMAIN_TAG);
    hash_part(&mut hasher, namespace.as_bytes());
    for input in inputs {
        hash_part(&mut hasher, input.as_ref());
    }
    let digest = hasher.finalize();
    let mut hash_bytes = [0u8; 16];
    hash_bytes.copy_from_slice(&digest[..16]);
    let hash = u128::from_be_bytes(hash_bytes);
    // 12 bits into custom_b (bits 64..76), 62 bits into custom_c (bits 0..62).
    let custom_b = (hash >> (128 - 12)) & 0x0fff;
    let custom_c = (hash >> (128 - 12 - 62)) & ((1u128 << 62) - 1);
    let bits: u128 = (u128::from(at.0) << 80) | (custom_b << 64) | custom_c;
    DerivedId(
        Builder::from_u128(bits)
            .with_version(Version::Custom)
            .with_variant(Variant::RFC4122)
            .into_uuid(),
    )
}

fn hash_part(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

// ---------------------------------------------------------------------------
// Arbitrary
// ---------------------------------------------------------------------------

#[cfg(feature = "proptest")]
mod proptest_impls {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for UnixMillis {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: ()) -> Self::Strategy {
            (0..=MAX_UNIX_MILLIS).prop_map(UnixMillis).boxed()
        }
    }

    impl Arbitrary for Entropy {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: ()) -> Self::Strategy {
            any::<[u8; 10]>().prop_map(Entropy).boxed()
        }
    }

    impl Arbitrary for TimeOrderedId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: ()) -> Self::Strategy {
            (any::<UnixMillis>(), any::<Entropy>())
                .prop_map(|(now, entropy)| {
                    TimeOrderedGenerator::new()
                        .next(now, entropy)
                        .expect("fresh generator never exhausts the timestamp")
                })
                .boxed()
        }
    }

    impl Arbitrary for DerivedId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: ()) -> Self::Strategy {
            (
                any::<UnixMillis>(),
                any::<String>(),
                proptest::collection::vec(any::<Vec<u8>>(), 0..4),
            )
                .prop_map(|(at, namespace, inputs)| derive_id(at, &namespace, inputs))
                .boxed()
        }
    }
}

#[cfg(feature = "arbitrary")]
mod arbitrary_impls {
    use super::*;
    use arbitrary::{Arbitrary, Result, Unstructured};

    impl<'a> Arbitrary<'a> for UnixMillis {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(UnixMillis(u.int_in_range(0..=MAX_UNIX_MILLIS)?))
        }
    }

    impl<'a> Arbitrary<'a> for Entropy {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(Entropy(<[u8; 10]>::arbitrary(u)?))
        }
    }

    impl<'a> Arbitrary<'a> for TimeOrderedId {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            let now = UnixMillis::arbitrary(u)?;
            let entropy = Entropy::arbitrary(u)?;
            Ok(TimeOrderedGenerator::new()
                .next(now, entropy)
                .expect("fresh generator never exhausts the timestamp"))
        }
    }

    impl<'a> Arbitrary<'a> for DerivedId {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            let at = UnixMillis::arbitrary(u)?;
            let namespace = <&str>::arbitrary(u)?;
            let inputs = <Vec<Vec<u8>>>::arbitrary(u)?;
            Ok(derive_id(at, namespace, inputs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v7_fields_round_trip() {
        let mut generator = TimeOrderedGenerator::new();
        let id = generator
            .next(
                UnixMillis::new(1_700_000_000_123).unwrap(),
                Entropy([0xff; 10]),
            )
            .unwrap();
        assert_eq!(id.as_uuid().get_version_num(), 7);
        assert_eq!(id.as_uuid().get_variant(), Variant::RFC4122);
        assert_eq!(id.unix_ms().get(), 1_700_000_000_123);
        assert_eq!(id.counter(), Entropy([0xff; 10]).counter_seed());
        assert!(id.counter() < 1 << (COUNTER_BITS - 1));
    }

    #[test]
    fn same_millisecond_increments_counter_and_orders() {
        let mut generator = TimeOrderedGenerator::new();
        let now = UnixMillis::new(42).unwrap();
        let a = generator.next(now, Entropy::ZERO).unwrap();
        let b = generator.next(now, Entropy([0xff; 10])).unwrap();
        assert_eq!(b.counter(), a.counter() + 1);
        assert!(b > a);
    }

    #[test]
    fn clock_regression_is_clamped() {
        let mut generator = TimeOrderedGenerator::new();
        let a = generator
            .next(UnixMillis::new(1_000).unwrap(), Entropy::ZERO)
            .unwrap();
        let b = generator
            .next(UnixMillis::new(999).unwrap(), Entropy::ZERO)
            .unwrap();
        assert_eq!(b.unix_ms().get(), 1_000);
        assert!(b > a);
    }

    #[test]
    fn counter_overflow_advances_the_millisecond() {
        let mut generator = TimeOrderedGenerator {
            state: Some((5, COUNTER_MAX)),
        };
        let id = generator
            .next(UnixMillis::new(5).unwrap(), Entropy::ZERO)
            .unwrap();
        assert_eq!(id.unix_ms().get(), 6);
        assert_eq!(id.counter(), 0);
    }

    #[test]
    fn timestamp_exhaustion_is_an_error() {
        let mut generator = TimeOrderedGenerator {
            state: Some((MAX_UNIX_MILLIS, COUNTER_MAX)),
        };
        assert_eq!(
            generator.next(UnixMillis::MAX, Entropy::ZERO),
            Err(IdError::TimestampOutOfRange {
                unix_ms: MAX_UNIX_MILLIS + 1
            })
        );
        assert!(UnixMillis::new(MAX_UNIX_MILLIS + 1).is_err());
    }

    #[test]
    fn v8_is_deterministic_and_length_prefixed() {
        let at = UnixMillis::new(7).unwrap();
        let a = derive_id(at, "ns", ["ab", "c"]);
        let b = derive_id(at, "ns", ["ab", "c"]);
        let c = derive_id(at, "ns", ["a", "bc"]);
        let d = derive_id(at, "other", ["ab", "c"]);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_eq!(a.as_uuid().get_version_num(), 8);
        assert_eq!(a.as_uuid().get_variant(), Variant::RFC4122);
        assert_eq!(a.unix_ms(), at);
        assert_eq!(DerivedId::try_from(a.into_uuid()), Ok(a));
        assert!(TimeOrderedId::try_from(a.into_uuid()).is_err());
    }
}

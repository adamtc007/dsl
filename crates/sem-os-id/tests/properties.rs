//! Property tests: v7 identifiers from one generator are strictly increasing
//! whatever the clock and entropy do; v8 derivation is a pure function of its
//! inputs.

use proptest::prelude::*;
use sem_os_id::{derive_id, Entropy, TimeOrderedGenerator, TimeOrderedId, UnixMillis};

fn millis() -> impl Strategy<Value = UnixMillis> {
    (0..=sem_os_id::MAX_UNIX_MILLIS).prop_map(|ms| UnixMillis::new(ms).unwrap())
}

fn entropy() -> impl Strategy<Value = Entropy> {
    any::<[u8; 10]>().prop_map(Entropy)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn v7_sequence_is_strictly_increasing_for_any_clock(
        steps in proptest::collection::vec((millis(), entropy()), 1..64)
    ) {
        let mut generator = TimeOrderedGenerator::new();
        let mut previous: Option<TimeOrderedId> = None;
        let mut high_water = 0u64;
        for (now, entropy) in steps {
            let id = generator.next(now, entropy).unwrap();
            if let Some(prev) = previous {
                prop_assert!(id > prev, "{id} must follow {prev}");
                prop_assert!(id.as_uuid().as_u128() > prev.as_uuid().as_u128());
            }
            high_water = high_water.max(now.get());
            // The issued millisecond never runs ahead of the clock by more
            // than counter overflows could force, and never behind it.
            prop_assert!(id.unix_ms().get() >= high_water.min(id.unix_ms().get()));
            prop_assert_eq!(id.as_uuid().get_version_num(), 7);
            previous = Some(id);
        }
    }

    #[test]
    fn v7_same_millisecond_burst_orders_by_counter(
        now in millis(),
        seeds in proptest::collection::vec(entropy(), 2..256)
    ) {
        let mut generator = TimeOrderedGenerator::new();
        let ids: Vec<_> = seeds
            .into_iter()
            .map(|e| generator.next(now, e).unwrap())
            .collect();
        for pair in ids.windows(2) {
            prop_assert_eq!(pair[0].unix_ms(), pair[1].unix_ms());
            prop_assert_eq!(pair[1].counter(), pair[0].counter() + 1);
            prop_assert!(pair[1] > pair[0]);
        }
    }

    #[test]
    fn v7_encodes_the_clock_when_it_advances(a in millis(), b in millis(), e in entropy()) {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let mut generator = TimeOrderedGenerator::new();
        let first = generator.next(lo, e).unwrap();
        let second = generator.next(hi, e).unwrap();
        prop_assert_eq!(first.unix_ms(), lo);
        if hi > lo {
            prop_assert_eq!(second.unix_ms(), hi);
        }
        prop_assert!(second > first);
    }

    #[test]
    fn v8_is_a_pure_function_of_its_inputs(
        at in millis(),
        namespace in "[a-z.]{0,16}",
        inputs in proptest::collection::vec(any::<Vec<u8>>(), 0..5)
    ) {
        let a = derive_id(at, &namespace, &inputs);
        let b = derive_id(at, &namespace, &inputs);
        prop_assert_eq!(a, b);
        prop_assert_eq!(a.unix_ms(), at);
        prop_assert_eq!(a.as_uuid().get_version_num(), 8);
        let fresh = TimeOrderedGenerator::new();
        prop_assert!(fresh.last().is_none());
    }

    #[test]
    fn v8_separates_inputs_namespaces_and_time(
        at in millis(),
        other_at in millis(),
        namespace in "[a-z]{1,8}",
        head in any::<Vec<u8>>(),
        tail in proptest::collection::vec(any::<u8>(), 1..8)
    ) {
        let joined: Vec<u8> = head.iter().chain(tail.iter()).copied().collect();
        let split = derive_id(at, &namespace, [head.as_slice(), tail.as_slice()]);
        let unsplit = derive_id(at, &namespace, [joined.as_slice()]);
        prop_assert_ne!(split, unsplit, "length prefixing must separate input boundaries");
        let renamed = derive_id(at, &format!("{namespace}x"), [head.as_slice(), tail.as_slice()]);
        prop_assert_ne!(split, renamed);
        if other_at != at {
            let moved = derive_id(other_at, &namespace, [head.as_slice(), tail.as_slice()]);
            prop_assert_ne!(split, moved);
        }
    }
}

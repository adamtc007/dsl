# sem-os-id

Pure identity helpers with no I/O and no ambient clock or randomness:

- `TimeOrderedGenerator` — RFC 9562 UUID v7 with a 42-bit dedicated counter
  (method 1) so identifiers issued within one millisecond are strictly
  increasing, and a clock that stands still or runs backwards never breaks
  the order. The caller supplies the millisecond and the entropy.
- `derive_id` — RFC 9562 UUID v8: a 48-bit millisecond prefix followed by a
  SHA-256-derived, length-prefixed hash of caller-supplied inputs, for
  replay-stable identities.

Optional `serde`, `proptest` (`Arbitrary` strategies) and `arbitrary`
(`arbitrary::Arbitrary`) features.

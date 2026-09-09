# sem-os-append-store

The store contract for versioned pieces:

- `append(batch)` — every write in the batch is conditional on
  `(piece_id, version_no)` being the next version of that piece; a stale
  version is a typed `LostRace`, a skipped version a typed `VersionGap`, and
  the batch is all-or-nothing.
- `fold(scope, up_to)` — the ordered replay set of a scope up to a sequence
  number, for the caller to fold.
- `current(scope, piece)` — the latest version of one piece.

`MemoryStore` is the reference implementation. The `conformance` feature
exposes the suite every real store must pass (sequence, atomicity, races,
and two concurrent writers on one piece). No SQL lives here; a Postgres
implementation is the owning domain's.

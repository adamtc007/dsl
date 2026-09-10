# sem-os-append-store

The store contract for versioned pieces, generic over the store's own
identity type (`AppendStore<Id>`; `String` and, with the `uuid` feature,
`uuid::Uuid` both qualify — a store is never required to adopt string keys):

- `append(batch)` — every write in the batch is conditional on
  `(piece_id, version_no)` being the next version of that piece; a stale
  version is a typed `LostRace`, a skipped version a typed `VersionGap`, and
  the batch is all-or-nothing.
- `fold(scope, up_to)` — the ordered replay set of a scope up to a sequence
  number, for the caller to fold.
- `current(scope, piece)` — the latest version of one piece.

Admitted records receive strictly increasing `Seq` values per scope — a
total order, not necessarily a contiguous one. Gaps are legal: a store built
on a shared, lock-free counter can let unrelated scopes, or unrelated pieces
within one scope, be appended to concurrently with no per-scope lock, at the
cost of gaps where another write consumed an intervening value. A store that
does guarantee contiguity may say so via `AppendStore::CONTIGUOUS`.

`MemoryStore<Id>` is the reference implementation. The `conformance` feature
exposes the suite every real store must pass, for whichever identity type it
uses (sequence ordering with gaps tolerated, contiguity when declared,
atomicity, races, two concurrent writers on one piece, and two concurrent
writers into one scope on different pieces with no serialisation assumed).
No SQL lives here; a Postgres implementation is the owning domain's.

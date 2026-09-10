# Reuse Tranche DSL-T6 — `sem-os-append-store` contract fixes

- **Programme:** EOP-PLAN-CA-REUSE-001 (core fixes found by CA's T0.3, ledger L30/L31)
- **Findings:** L30 — ids are not strings: the trait and every type in the
  contract (`ScopeId`, `PieceId`, `PieceWrite`, `AppendBatch`, `PieceRecord`,
  `WrittenPiece`, `AppendReceipt`, `AppendError`, `AppendStore`) were written
  against `String`, so a store built on a different identity type (CA:
  `Uuid`) could not implement the trait without first stringifying its keys.
  L31 — contiguity is not the contract: `Seq` was assigned from
  `records.len() + 1`, documented as "contiguous", and the conformance suite
  (`append_assigns_contiguous_sequence`) asserted exact contiguous values —
  so a store that draws sequence numbers from a source shared across scopes
  (to avoid a per-scope lock) could never pass the suite, even though
  nothing in the actual precedence law needs contiguity, only a total order.
- **Status:** GREEN

## 1. L30 — the identity type is a type parameter

`ScopeId<Id>` and `PieceId<Id>` are now generic newtypes over the store's own
identity type, and so is everything built from them: `PieceWrite<Id>`,
`AppendBatch<Id>`, `PieceRecord<Id>`, `WrittenPiece<Id>`, `AppendReceipt<Id>`,
`AppendError<Id>`, and the trait itself, `AppendStore<Id>`.

Bound: `pub trait StoreId: Clone + Eq + Ord + Hash + Debug + Display + Send +
Sync + 'static {}`, blanket-implemented for every type with these properties
— a store implements nothing extra to use its own identifier type. This
deviates from the finding's suggested `Copy + Ord + Hash + Send + Sync +
'static` in one respect: `Clone`, not `Copy`. `Copy` would exclude `String`
outright (bpmn-lite-style stores could never satisfy the trait), which is
the opposite of L30's point; `Clone` is satisfied by both `String` and
`Uuid` and by every id newtype already in this workspace. `Eq`/`Debug`/
`Display` are added because the contract's own types need them (`BTreeSet`/
`BTreeMap` keys, `thiserror` `#[error("...")]` interpolation, `assert_eq!`
failure messages) — `StoreId` names exactly what the contract already
required of `String`, generalised.

`ScopeId::new`/`PieceId::new` now wrap any `Id` (no validation — validity of
an arbitrary identity type is that type's own business). The previous
non-empty/bounded-length/no-whitespace validation is preserved as
`ScopeId::<String>::validated`/`PieceId::<String>::validated`, so a
string-keyed store keeps exactly the same guarantee it had before.

A `uuid` feature (optional `uuid` dependency, `default-features = false`,
`v5` only) adds nothing to `StoreId` — `Uuid` already satisfies every bound
in the blanket impl — and gates `conformance::UuidIds`.

### Conformance suite: no hard-coded `"seq"`/`"p"`

Every check function is now generic over `Id: StoreId` and takes an
`&impl IdFactory<Id>`:

```rust
pub trait IdFactory<Id>: Send + Sync {
    fn id(&self, label: &str) -> Id;
}
```

`conformance::StringIds` (always available) returns the label verbatim.
`conformance::UuidIds` (feature `uuid`) returns a deterministic name-based
UUID (v5, fixed namespace) per label, so the same label is stable within one
run and distinct labels never collide — no store needs to invent id text.
`crates/sem-os-append-store/tests/memory_conformance.rs` runs
`conformance::run_all` twice, once as
`run_all(MemoryStore::<String>::new, &StringIds)` and once as
`run_all(MemoryStore::<Uuid>::new, &UuidIds)`: the same store type, the same
suite, two identity types, both green — the concrete proof that nothing here
requires string keys.

## 2. L31 — `Seq` is a total order per scope, gaps are legal

`Seq`'s documentation and the trait's contract (point 2) are rewritten:
"strictly increasing per scope", not "contiguous, starting at `Seq::FIRST`".
`AppendStore` gains an associated constant:

```rust
const CONTIGUOUS: bool = false;
```

`MemoryStore<Id>` sets `CONTIGUOUS = true` (it always allocates from
`records.len() + 1`, so the stronger guarantee is true and free to state).
The default is `false`: a store opts in only if it actually provides
contiguity, since — as the finding states — contiguity forces a per-scope
lock or sequence and forecloses concurrent, unserialised writers into one
scope.

The suite changes accordingly:

- `append_assigns_contiguous_sequence` is replaced by
  `append_assigns_strictly_increasing_sequence`, which asserts a strictly
  increasing total order (no duplicate or out-of-order `Seq` values) and
  that a later batch sequences strictly after an earlier one — never that
  values are contiguous or start at `Seq::FIRST`.
- `contiguous_store_has_no_gaps` is a new, separate check that verifies the
  stronger property, but only when `S::CONTIGUOUS` is `true`; it is a no-op
  for a store that does not declare it, so the base suite never requires
  contiguity.
- `two_writers_one_scope_may_race_without_serialising` is a new check: two
  writers append to the *same scope* but *different pieces*, released
  simultaneously via a `Barrier`, with no scope-level coordination assumed.
  Both writes must be admitted (they do not conflict), the two sequence
  numbers must differ, and `fold` must return a strictly increasing total
  order — nothing more. It deliberately does not assert contiguity or that
  the sequence numbers are adjacent, so a store whose scope-fan-out strategy
  leaves gaps (e.g. a shared, lock-free global counter) can pass it.
  `MemoryStore` also passes this check (its internal mutex happens to
  serialise the two writers, and the suite's assertions hold either way,
  which is the point: the suite does not depend on how the store achieved
  the outcome).

`append_assigns_strictly_increasing_sequence`'s own two-write and
three-record checks retain the underlying `MemoryStore` behaviour (`Seq`
values 1, 2, 3 in this test, since `MemoryStore` is in fact contiguous), but
the *assertions* no longer encode that as a requirement — they check
relative order, not absolute value, so the same test body is meaningful
against a non-contiguous store too.

### "Red before, green after"

This tranche replaces the contract's types, not a single function's logic,
so there is no prior commit against which the new tests can be run
red — the new tests are not expressible against the old API at all:
`MemoryStore::<Uuid>::new` does not type-check against the pre-tranche
`MemoryStore` (it took no type parameter and its fields were `ScopeId`/
`PieceId`, hard-wired to `String`). That non-compilation *is* L30's red
state. For L31, the pre-tranche suite's own
`append_assigns_contiguous_sequence` was the failing case in spirit: it
would reject a hypothetical non-contiguous-but-correct store by construction
(asserting `seqs == vec![1, 2, 3]` unconditionally), which is exactly the
defect being fixed; the new `contiguous_store_has_no_gaps` isolates that
assertion behind the capability flag instead of baking it into the base
contract.

## Domain neutrality

No domain nouns introduced. `IdFactory`, `StringIds`, `UuidIds`, `StoreId`
are generic vocabulary; the deterministic UUID v5 namespace is a fixed,
unlabelled constant (`Uuid::NAMESPACE_OID`) used only to derive test
fixture ids.

## Public-API diff (`cargo public-api --simplified`, `eop/ca-reuse-001-dsl-2` → this tranche)

| crate | change |
|---|---|
| sem-os-append-store | Every public type in the crate gains an `Id` type parameter (`ScopeId<Id>`, `PieceId<Id>`, `PieceWrite<Id>`, `AppendBatch<Id>`, `PieceRecord<Id>`, `WrittenPiece<Id>`, `AppendReceipt<Id>`, `AppendError<Id>`, `AppendStore<Id>`, `MemoryStore<Id>`). New: `StoreId` (blanket-implemented marker trait), `AppendStore::CONTIGUOUS`, `conformance::{IdFactory, StringIds, UuidIds}`. Renamed: `conformance::append_assigns_contiguous_sequence` → `append_assigns_strictly_increasing_sequence`. New: `conformance::{contiguous_store_has_no_gaps, two_writers_one_scope_may_race_without_serialising}`. Every `conformance::*` function gains `Id`/`G: IdFactory<Id>` parameters. This is a complete break of the crate's pre-tranche API; there are no consumers of this crate anywhere else in the workspace or in `ob-poc`/`bpmn-lite` yet (verified by grep), so nothing downstream is affected. |

## Verification

- `env -u DSL_CONFIG_DIR cargo test --workspace --all-targets --all-features --locked` — green (41 test binaries, 0 failures), including `MemoryStore` passing the full conformance suite under both `Id = String` and `Id = uuid::Uuid`.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps --all-features` — clean.
- `check-layering.sh`, `check-dependencies.sh`, `check-domain-neutral.sh`, `check-domain-nouns.sh`, `check-packages.sh` (dry-run publish of `sem-os-append-store` as a leaf) — OK, unchanged.
- `check-public-api-baselines.sh --update` then re-run clean; only `scripts/baselines/sem-os-append-store-public-api-v1.txt` changed.
- `Cargo.lock`: `uuid` (already in the lock file via `sem-os-id`) gains a feature edge from `sem-os-append-store`; no new crates.io package.

## Follow-ups

None. The fix is generic (identity type parameterisation, total order per
scope) and needed no new special cases; nothing here is domain-specific.

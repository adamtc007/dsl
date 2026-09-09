# Reuse Tranche DSL-T3 — Three core pieces CA needs and bpmn-lite will share

- **Programme:** EOP-PLAN-CA-REUSE-001 (core fixes from the reuse recon)
- **UTC:** 2026-09-09
- **Findings:** `ob-poc/docs/eop/reuse-recon-ob-poc.md` §1.5 Q5 (core ids are all v4, no helper; `sem_os_policy/src/service.rs:1324,1458`, `enforce.rs:64,67`, `authoring/governance_verbs.rs:379-560`), §1.5 Q6 capability segments (prefix matching only, `semantic-pack/src/source.rs:481-495`; ob-poc `xtask/src/reconcile.rs:1150-1175`), §4.3 store traits (`KycEventStore` assigns `seq` itself, `&mut self`, table-bound; no conditional append on `(piece_id, version_no)`), §6 items 3, 4, 5
- **Status:** GREEN

## 1. `sem-os-id` (new crate, leaf)

Pure and I/O-free: the caller supplies `UnixMillis` (validated 48-bit) and,
for v7, `Entropy` (80 bits). No clock, no RNG, no `std::time`.

- `TimeOrderedGenerator::next(now, entropy) -> TimeOrderedId` — RFC 9562
  v7 with the §6.2 method-1 dedicated counter: 42 bits (12 in `rand_a`, 30
  at the top of `rand_b`), seeded from entropy on a new millisecond with the
  top bit clear, incremented within a millisecond, and rolled into the next
  millisecond on overflow. A clock that stalls or regresses is clamped to the
  last issued millisecond. Identifiers from one generator are strictly
  increasing as `u128`.
- `derive_id(at, namespace, inputs) -> DerivedId` — RFC 9562 v8: 48-bit
  millisecond prefix, then 74 bits of SHA-256 over a domain tag, the
  namespace and each input, every part length-prefixed.
- Newtypes `TimeOrderedId`/`DerivedId` carry `TryFrom<Uuid>` version checks,
  `unix_ms()` and (v7) `counter()` extraction.
- Features: `serde`, `proptest` (`Arbitrary` for `UnixMillis`, `Entropy`,
  `TimeOrderedId`, `DerivedId`), `arbitrary` (`arbitrary::Arbitrary`, for
  fuzz targets). `uuid` is used with `default-features = false`.

Property tests (`crates/sem-os-id/tests/properties.rs`, 512 cases each):
strictly increasing over any clock sequence; same-millisecond bursts order
by counter; clock advance is encoded; v8 is a pure function of its inputs;
v8 separates input boundaries, namespaces and time. Unit tests cover
counter overflow, timestamp exhaustion and field round trips.

## 2. Capability segments in `semantic-pack` (+ resolver in `sem_os_policy`)

Declared by data, no built-in segments:

```yaml
declarations:
  capability_segments:
    position: 1                 # dotted segment that names the segment
    segments:
      - { name: derive, may: [read] }
      - { name: assert, may: [append] }
      - { name: decide, may: [read, append] }
      - { name: effect, may: [effect] }
```

- Types: `CapabilitySegmentPolicySource { position, segments }`,
  `CapabilitySegmentSource { name: CapabilitySegmentName, may }`,
  `SegmentPermission::{Read, Append, Effect}`, `SegmentRuling::{NoPolicy,
  Undeclared, Permitted, Forbidden}`. `CapabilitySegmentName` is a validated
  single dotted segment.
- Each capability's `action_class` maps to one required permission
  (`SegmentPermission::required_by`: list/read/search/describe/compute/review
  → read; create/update/delete/assign/remove/import/approve/reject → append;
  execute → effect). Validation rejects, with the new
  `DiagnosticCode::InvalidSegment`, a capability whose id has no declared
  segment at `position`, or whose action class needs a permission its
  segment withholds; empty segment lists, permission-less segments and
  duplicate names are rejected; the limit is 64 segments.
- Selector: `CapabilitySelectorSource::Segment(name)` matches by declared
  segment, not prefix (`matches_in(capability, policy)`); `matches()` keeps
  its signature and never matches a segment selector. Segment selectors that
  name an undeclared segment, or appear in a pack without a segment policy,
  are validation errors.
- Artifact: `CompiledPack::{capability_segments, capability_segment,
  segment_ruling}`. Compile normalises segment order and permission sets.
  The new declaration field is `Option` with `skip_serializing_if`, so packs
  that declare nothing produce byte-identical canonical bytes and unchanged
  artifact hashes (asserted in `packs_without_a_segment_policy_are_unchanged`).
- Resolver (`sem_os_policy::pack_policy`): `evaluate_capability` honours
  segment selectors (a segment deny beats a prefix allow);
  `segment_permits(snapshot, capability, permission)`; and
  `CapabilityAdapterRegistry::resolve_for(snapshot, capability, permission)`
  refuses an adapter with typed `SegmentForbids { capability, segment,
  permission, may }` or `UndeclaredCapabilitySegment`. Packs without a
  policy resolve exactly as before.

Tests: `crates/semantic-pack/tests/capability_segments.rs` (8 tests incl. two
proptest properties: segment-name validation; position selects exactly that
dotted part), `crates/sem_os_policy/tests/pack_policy.rs::segment_rules_are_enforced_by_selectors_and_the_resolver`.

## 3. `sem-os-append-store` (new crate, leaf)

Contract (`AppendStore`, `async_trait`, `&self`):

| method | rule |
|---|---|
| `append(batch)` | every `PieceWrite.version_no` must be exactly the next version of its piece (`VersionNo::FIRST` for a new piece). Stale ⇒ typed `LostRace { piece_id, attempted, current }`; skipped ⇒ `VersionGap { piece_id, attempted, expected }`; the batch is all-or-nothing; admitted records get contiguous `Seq` in batch order. |
| `fold(scope, up_to)` | ordered replay set, all or `seq <= up_to`. |
| `current(scope, piece)` | highest version of one piece. |

`AppendBatch::new` rejects empty batches and a piece named twice
(`EmptyBatch`, `DuplicatePieceInBatch`). Identifiers (`ScopeId`, `PieceId`)
are opaque validated strings; payloads are opaque bytes; `replay` is a
free-function fold helper. No SQL; a Postgres implementation is the domain's.

`MemoryStore` is the reference implementation. The `conformance` feature
exposes `conformance::run_all(factory)` and nine individually callable
checks: contiguous sequence, latest version, fold prefix, stale loses race,
version gap, batch atomicity, malformed batches, scope independence, and
**two writers on one piece** (sixteen rounds of two `tokio::spawn`ed writers
released by a `Barrier`; exactly one wins per round). `MemoryStore` passes it
(`crates/sem-os-append-store/tests/memory_conformance.rs`), and a 256-case
property test checks the contract under random stale/gap/correct write
sequences against a model.

## Domain neutrality

All three pieces pass `scripts/check-domain-nouns.sh`; the only
segment-like words anywhere are test fixtures (`derive`, `assert`, …), which
are not domain nouns and are not built in.

## Verification

- `env -u DSL_CONFIG_DIR cargo test --workspace --all-targets --all-features --locked` — green.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps --all-features` — clean.
- `check-layering.sh`, `check-dependencies.sh` (new leaf rows), `check-domain-neutral.sh`, `check-domain-nouns.sh`, `check-packages.sh` (dry-run publishes `sem-os-id` and `sem-os-append-store` as leaves), `check-public-api-baselines.sh` (now `--all-features`; baselines recorded for the two new crates) — OK.
- CI feature matrix gains `sem-os-id` (default/all-features) and `sem-os-append-store` (default/`conformance`).
- `Cargo.lock` gains `arbitrary` (optional, MIT/Apache-2.0) only.

## Public-API diff (`cargo public-api --simplified`, `0967d0b` → this tranche)

| crate | lines | change |
|---|---|---|
| sem-os-id | 216 (new) | baseline recorded |
| sem-os-append-store | 356 (new) | baseline recorded |
| semantic-pack | 1734 → 1881 | `+ CapabilitySegmentName, CapabilitySegmentPolicySource, CapabilitySegmentSource, SegmentPermission, SegmentRuling`; `+ CapabilitySelectorSource::Segment`, `::matches_in`; `+ DiagnosticCode::InvalidSegment`; `+ DeclarationSource::capability_segments`; `+ CompiledPack::{capability_segments, capability_segment, segment_ruling}`. No removals. |
| sem_os_policy | 7001 → 7009 | `+ PackPolicyError::{SegmentForbids, UndeclaredCapabilitySegment}`; `+ CapabilityAdapterRegistry::resolve_for`; `+ pack_policy::segment_permits`. No removals. |

`CapabilitySelectorSource` gained a variant: exhaustive `match`es in hosts
must add an arm (source-breaking, not wire-breaking).

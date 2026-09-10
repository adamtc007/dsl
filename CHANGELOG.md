# Changelog

All notable changes to the shared DSL and SemOS packages are recorded here.
The workspace follows Semantic Versioning subject to the pre-1.0 rules in
`docs/versioning.md`.

## [Unreleased]

### Added

- Relocate the unified DSL atom grammar crates `dsl-diagnostics`,
  `dsl-parser`, `dsl-atoms` and `dsl-ast` into this workspace as publishable
  crates (EOP-PLAN-CA-REUSE-001 DSL-T1). The atom kind catalogue is now data:
  `dsl_atoms::KindCatalogue` is built by registration, parsed from DSL
  source, or seeded from the built-in set, and an unregistered kind is a
  typed `UnknownKind` error naming the catalogue. The closed
  `StructuralKind`/`DeclarativeKind` enums and `classify` are removed;
  `AtomBag::from_source_file` takes a catalogue and returns a `Result`.
  Keyword slots outside `flow` remain the only slot form and are preserved
  verbatim.
- Record `cargo public-api` baselines for the four crates and gate them with
  `scripts/check-public-api-baselines.sh`.
- Add `scripts/check-domain-nouns.sh`: a CI gate that fails on any domain
  noun in non-test core code, with an allowlist that must stay empty
  (EOP-PLAN-CA-REUSE-001 DSL-T2).
- Add `sem-os-id`: pure UUID v7 (RFC 9562 dedicated counter, monotonic within
  a millisecond, clock-regression safe) and deterministic UUID v8 helpers
  with optional `serde`, `proptest` and `arbitrary` support
  (EOP-PLAN-CA-REUSE-001 DSL-T3).
- Add `sem-os-append-store`: the conditional-append store contract for
  versioned pieces (`append`/`fold`/`current`, typed `LostRace` and
  `VersionGap`), the `MemoryStore` reference implementation and a
  `conformance` suite for real stores (DSL-T3).
- `semantic-pack`: packs may declare capability segments
  (`declarations.capability_segments`) and what each may do; validation
  enforces segment membership and action-class permissions, and the new
  `CapabilitySelectorSource::Segment` selector matches by declared segment.
  `sem_os_policy` adds `segment_permits` and
  `CapabilityAdapterRegistry::resolve_for` (DSL-T3). Packs without a segment
  policy keep their artifact hashes.

### Changed

- **Breaking:** `dsl_core::SourceOfTruth::KycStream` is replaced by the
  data-carried `SourceOfTruth::Stream(String)` (YAML
  `source_of_truth: { stream: <id> }`); the enum no longer derives `Copy`.
- **Breaking:** threshold predicates and bounds are fixed-point decimals.
  `EscalationPredicate::{ArgGt,ArgGte,ArgLt,ArgLte}.value` and
  `ArgValidation::{min,max}` are `rust_decimal::Decimal`; YAML accepts
  integer literals or quoted decimal text and rejects floating-point
  literals. Evaluation compares decimals; no `f64` remains on the predicate
  path.
- `sem_os_policy::domain_pack::DomainTransition` gains optional `slot_path`
  and `node_prefix`; state simulation derives its predicted advance target
  from the transition instead of a built-in name.
- `sem_os_policy::context_resolution` canonicalises entity kinds through a
  request-supplied `EntityKindAliases` table instead of a built-in alias
  `match`.
- Domain examples in shared-crate documentation are rewritten with neutral
  vocabulary.

### Fixed

- `dsl-parser`: error recovery in `parse_list`, `parse_map`,
  `parse_for_each_body` and `parse_atom_body`'s positional-value loop could
  fail to consume a token it could not turn into a value, looping forever on
  an unlexable character inside a bracketed list (e.g. `(x :a [<])`). Every
  such recovery path now always consumes a token or reaches its own
  terminator (EOP-PLAN-CA-REUSE-001 DSL-T5).
- **Breaking:** `dsl-parser` treated *any* lex error immediately following a
  `Symbol` as the `pack/atom` qualified-name separator, so `foo=bar` silently
  parsed as `QualifiedName { pack: "foo", atom: "bar" }` with no diagnostic.
  The lexer gains an explicit `Token::Slash`; only `/` forms a qualified
  name, and any other unrecognised character is left to be diagnosed where
  it is next examined (DSL-T5).
- **Breaking:** `sem-os-append-store`'s contract was written against
  `String` identifiers throughout (`ScopeId`, `PieceId`, and every type built
  from them), so a store using a different identity type (e.g. `Uuid`) could
  not implement the trait. Every type is now generic over the store's own
  identity type `Id: StoreId` (`StoreId` is blanket-implemented, so `String`
  and, with the new `uuid` feature, `uuid::Uuid` both qualify with no extra
  code); `ScopeId::<String>::validated`/`PieceId::<String>::validated`
  preserve the previous string validation. The conformance suite is
  parameterised the same way via a new `conformance::IdFactory<Id>` (with
  `StringIds` and, under `uuid`, `UuidIds`), so it makes no assumption about
  the shape of `Id` (EOP-PLAN-CA-REUSE-001 DSL-T6, ledger L30).
- **Breaking:** `sem-os-append-store`'s `Seq` was documented and enforced as
  contiguous per scope; the actual precedence law only needs a total order.
  `Seq` is now defined as strictly increasing per scope, with gaps
  explicitly legal; `AppendStore` gains an opt-in `const CONTIGUOUS: bool`
  (`MemoryStore` sets it `true`). The conformance suite's contiguous-sequence
  check is split into a base check that only requires strict ordering and a
  separate check (only run when `CONTIGUOUS` is declared) that additionally
  requires no gaps, and gains a new check that two writers may append to
  different pieces in one scope without being serialised against each other
  (DSL-T6, ledger L31).

## [0.2.2] - 2026-08-05

### Changed

- Upgrade the optional Candle inference stack from 0.8.4 to 0.9.2, replacing
  the future-incompatible Apple `block 0.1.6` dependency with the maintained
  `objc2` bindings. Public embedding contracts, model identity, dimension,
  tokenizer, weights, and query/target semantics are unchanged. Pinned-model
  parity probes observed at most `1.50e-7` absolute per-component numerical
  drift, so vectors are semantically equivalent but not byte-identical across
  the implementation upgrade.
- Repair the dependency-policy gate by explicitly admitting the permissive
  transitive licenses in the locked graph and documenting two unavoidable,
  unmaintained-only upstream advisory exceptions without relaxing vulnerability
  or yanked-crate enforcement.

## [0.2.1] - 2026-08-05

### Fixed

- Document the intentional eight-port dependency-injection constructor so the
  release graph passes the repository's warnings-as-errors Clippy gate.

## [0.2.0] - 2026-08-05

### Added

- Extracted stable decision-board, evidence, disposition, and proposal
  workbook contracts into the host-neutral `semantic-decision-contracts`
  leaf crate. Existing SemOS import paths remain compatibility re-exports.
- Extracted typed embedding contracts, a deterministic fake, and optional
  Candle inference into the host-neutral `semantic-embedder` leaf crate. Its
  default feature set is empty and remote model resolution is explicit.
- Establish MIT licensing, Rust 1.95 MSRV, and standalone package metadata.
- Add formatting, build, test, lint, documentation, package, dependency, and
  domain-boundary CI gates.
- Add the typed `semantic-pack` source, validation, compilation, registry, and
  canonical artifact API used by applications to load domain policy from YAML.
- Make SemOS decision construction consume admitted semantic-pack artifacts
  instead of embedding host command families in shared Rust source.

### Changed

- Remove the unused unsafe AST reference collector.
- Scope local Cargo patches to opted-in repositories.
- Move host qualification suites to their owning consumer repositories so the
  shared workspace tests without an `ob-poc` or BPMN checkout.

No semantic contract, serialised field, persistent identifier, or hash changes
were made without retaining their existing schema or algorithm identifiers.

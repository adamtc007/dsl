# Reuse Tranche DSL-T1 — Relocate the unified-DSL parser trio with a data-driven kind catalogue

- **Programme:** EOP-PLAN-CA-REUSE-001 (core fixes from the reuse recon)
- **UTC:** 2026-09-09
- **Findings:** `ob-poc/docs/eop/reuse-recon-ob-poc.md` R-00c (parser trio are unpublished ob-poc path crates, `rust/Cargo.toml:394-406`), §2.1 dsl-atoms Q6 (closed kind catalogue, `kinds.rs:64-96`; `atom_bag.rs:63-70`), §2.2 positional-value rule (`parser.rs:264-271`), §2.2 `guard`/`pre`/`roles`/`se`/`bind`/`reads-as` rows
- **Source:** ob-poc `5be3bbf3` (`feat/ws-2a-authoring-plane-pin`), copied read-only; ob-poc not edited
- **Status:** GREEN

## What moved

| crate | from | to | version |
|---|---|---|---|
| `dsl-diagnostics` | `ob-poc/rust/crates/dsl-diagnostics` (0.1.0, path) | `crates/dsl-diagnostics` | 0.4.0 (workspace) |
| `dsl-parser` | `ob-poc/rust/crates/dsl-parser` (0.1.0, path) | `crates/dsl-parser` | 0.4.0 (workspace) |
| `dsl-atoms` | `ob-poc/rust/crates/dsl-atoms` (0.1.0, path) | `crates/dsl-atoms` | 0.4.0 (workspace) |
| `dsl-ast` | `ob-poc/rust/crates/dsl-ast` (0.1.0, path) | `crates/dsl-ast` | 0.4.0 (workspace) |

The prompt names three crates. `dsl-parser` has a hard dependency on
`dsl-diagnostics` (`rust/crates/dsl-parser/Cargo.toml:10`), so the fourth
crate moved with it as a leaf; the recon's "trio" was never a closed set of
three. All four are publishable, share the workspace version, carry
`readme`/`license`/`repository` metadata, and use `[lints] workspace = true`
(the per-crate `dead_code = "deny"` lint was dropped because Cargo forbids
mixing it with `workspace = true`; CI's `clippy -D warnings` still fails on
dead code).

Dependency direction changed in one place: `dsl-parser` no longer depends on
`dsl-atoms` (the dependency was declared but unused at
`rust/crates/dsl-parser/Cargo.toml:9`; `parser.rs` imports only
`dsl_diagnostics`). `dsl-atoms` now depends on `dsl-parser`, because a
catalogue is parseable from DSL source. The stack is
`dsl-diagnostics → dsl-parser → dsl-atoms → dsl-ast`, acyclic, with no
dependency on `dsl-core` or any SemOS crate.

## Catalogue extension point (dsl-atoms Q6, `kinds.rs:64-96`)

The closed enums `StructuralKind` (20 variants) and `DeclarativeKind` (4) and
the `classify` `match` are gone. Replacements:

- `KindCatalogue` — a named `BTreeMap<KindName, KindRole>`. Built by
  `KindCatalogue::new(name)?.with(kind, role)?` / `register`, seeded by
  `KindCatalogue::builtin()`, parsed by `KindCatalogue::from_source(src)`
  and rendered by `to_source()` (round-trip tested).
- `KindName` — validated symbol-shaped name (`[A-Za-z_][A-Za-z0-9_-]*`,
  ≤ 64 bytes). `KindRole` — `Structural | Declarative`.
- `AtomKindClass::{Structural(KindName), Declarative(KindName)}` —
  data-carried, same two-arm shape as before so consumers keep pattern
  matching on role.
- `KindCatalogue::classify(kind) -> Result<AtomKindClass, UnknownKind>`.
  `UnknownKind { kind, catalogue }` is the typed error and names the
  catalogue. There is no default arm; the former
  `AtomKindClass::UnknownStructural`/`UnknownDeclarative` variants are
  removed.
- `dsl_atoms::builtin::*` — the 24 built-in kind names as `&str` constants
  (`NODE`, `GATEWAY`, `PROVENANCE`, …) plus `STRUCTURAL`/`DECLARATIVE`
  slices, so a frontend can name a built-in kind without a closed enum.

Catalogue source grammar (documented in `crates/dsl-atoms/src/catalogue.rs`):

```text
(kind-catalogue board-kinds :extends builtin)   ; header, optional
(kind board :role structural)
(kind attestation :role declarative)
```

`dsl-ast` changes: `AtomBag::from_source_file(source, &catalogue, &mut diag)
-> Result<AtomBag, UnknownKind>` (was infallible and defaulted unknown kinds
into the bag with a diagnostic); every unknown kind is still diagnosed with
code `E0001`, and the message names the catalogue. New `AtomParser::new
(catalogue)` / `AtomParser::builtin()` bundles the catalogue with
`dsl_parser::parse` for external consumers. `atoms_of_structural_kind` now
takes `&str`; `atoms_of_kind`, `atoms_of_declarative_kind`, `atoms_of_role`
and `atoms()` were added. Nested atoms (atoms in slot values) remain
unclassified, as before; that is documented rather than changed.

## Slot rule (`parser.rs:264-271`) and the six slots

Keyword-form-only outside `flow` is kept and now documented at
`crates/dsl-parser/src/lib.rs` ("Slot rule"). No positional forms were added.
`crates/dsl-parser/tests/keyword_slots.rs` proves that `:guard`, `:pre`,
`:roles`, `:se`, `:bind`, `:reads-as` are preserved verbatim, in source
order, with nested atoms, lists, the `nil` symbol and `{}`-bearing strings
intact, and that parse → print → parse and serde round trips are identities.
A positional value outside `flow` is asserted to remain a parse error.

## Tests that can fail

| acceptance item | test |
|---|---|
| crates build here with no ob-poc dependency | `scripts/check-dependencies.sh` (new allowed-dependency rows; `assert_no_host_sources`) + `crates/dsl-integration-tests/tests/atom_grammar_consumer.rs` |
| catalogue-extension test (register a new kind by data, parse it) | `crates/dsl-ast/tests/catalogue_extension.rs::register_new_kinds_by_data_and_parse_them`, `::catalogue_parsed_from_source_drives_the_parser`; `dsl-atoms` unit tests `register_extends_by_data`, `from_source_with_header_and_extends` |
| unknown-kind error test | `crates/dsl-ast/tests/catalogue_extension.rs::unknown_kind_is_a_typed_error_naming_the_catalogue`, `::builtin_catalogue_still_rejects_kinds_it_does_not_know`; `dsl-atoms::unknown_kind_is_a_typed_error_naming_the_catalogue` |
| six-slot round-trip | `crates/dsl-parser/tests/keyword_slots.rs` (two tests) |
| `cargo public-api` baseline recorded | `scripts/baselines/{dsl-diagnostics,dsl-parser,dsl-atoms,dsl-ast}-public-api-v1.txt`, gated by the new `scripts/check-public-api-baselines.sh` in the CI `boundaries` job |

## Verification

- `cargo test -p dsl-diagnostics -p dsl-parser -p dsl-atoms -p dsl-ast` — 7 + 4 + 11 + 3 (+ 25 parser/ast integration) passed.
- `cargo test --workspace --all-targets --all-features --locked` — all green.
- `cargo clippy … -D warnings`, `RUSTDOCFLAGS=-D warnings cargo doc` — clean for the four crates and the integration-test crate.
- `scripts/check-layering.sh`, `check-dependencies.sh`, `check-domain-neutral.sh`, `check-packages.sh` (now dry-run publishes `dsl-diagnostics` as a leaf), `check-public-api-baselines.sh` — all OK.
- `Cargo.lock` gains `logos 0.14.4` (+ codegen deps); dependency policy unchanged.

## Public surface (recorded baselines, `--simplified`)

| crate | lines |
|---|---|
| dsl-diagnostics | 114 |
| dsl-parser | 124 |
| dsl-atoms | 265 |
| dsl-ast | 89 |

Symbols removed relative to the ob-poc copies (ob-poc consumers must adapt in
Prompt C): `dsl_atoms::{StructuralKind, DeclarativeKind, classify}`,
`AtomKindClass::{UnknownStructural, UnknownDeclarative}`; signature change
`AtomBag::from_source_file` (catalogue parameter, `Result`);
`AtomBag::atoms_of_structural_kind(&str)`. Consumers touched in ob-poc (from
the recon grep): `dsl-bpmn-frontend/src/assembly.rs`,
`dsl-resolution/src/{resolve,validator,pack_registry}.rs`,
`dsl-render/src/renderer.rs`, `dsl-semos-frontend/src/loader.rs`,
`dsl-migrate-verify/src/lib.rs`, `bpmn-test-harness/src/lib.rs`, plus tests.

## Not done here (logged in `reports/reuse/FOLLOW-UPS.md`)

- Qualified-name lexing treats any lex-error token after a symbol as `/`
  (`crates/dsl-parser/src/parser.rs:472-512`).
- `RawValue::FloatLit(f64)` remains; the grammar has no decimal literal.
- Pre-existing rustfmt drift in `crates/semantic-decision-contracts/src/{gameboard,lib}.rs` at `c357082` (CI `format` job is red before this tranche).

# Changelog

All notable changes to the shared DSL and SemOS packages are recorded here.
The workspace follows Semantic Versioning subject to the pre-1.0 rules in
`docs/versioning.md`.

## [Unreleased]

### Added

- Extracted stable decision-board, evidence, disposition, and proposal
  workbook contracts into the host-neutral `semantic-decision-contracts`
  leaf crate. Existing SemOS import paths remain compatibility re-exports.
- Extracted typed embedding contracts, a deterministic fake, and optional
  Candle inference into the host-neutral `semantic-embedder` leaf crate. Its
  default feature set is empty and remote model resolution is explicit.

### Planned for 0.2.0

- Establish MIT licensing, Rust 1.95 MSRV, and standalone package metadata.
- Add formatting, build, test, lint, documentation, package, dependency, and
  domain-boundary CI gates.
- Remove the unused unsafe AST reference collector.
- Scope local Cargo patches to opted-in repositories.

No semantic contract, serialised field, persistent identifier, or hash changes
are included in this release-foundation phase.

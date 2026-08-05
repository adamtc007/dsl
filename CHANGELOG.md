# Changelog

All notable changes to the shared DSL and SemOS packages are recorded here.
The workspace follows Semantic Versioning subject to the pre-1.0 rules in
`docs/versioning.md`.

## [Unreleased]

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

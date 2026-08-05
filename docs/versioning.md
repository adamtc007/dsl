# Versioning and release policy

## Package versions

The publishable crates in this workspace use one shared version. A release tag
`vX.Y.Z` must point to a commit where every publishable package declares
`X.Y.Z`. Tags and package versions must not drift.

The workspace is pre-1.0. Minor releases may contain public API changes, but
consumer migrations must be documented and compatibility re-exports retained
for the announced window. Patch releases are backwards-compatible fixes.

## Persistent compatibility

Semantic Versioning does not replace wire and storage versioning. Canonical
hashes, UUID namespaces, serialised decision contracts, compiled artifacts, and
workbooks carry explicit schema or algorithm versions. Existing bytes are never
changed while retaining their old version identifier.

## Rust version

The minimum supported Rust version is 1.95. Raising it requires a release note,
a CI/toolchain update, and verification in both current consumer applications.
The workspace remains on Rust edition 2021 until an edition migration is
planned and reviewed independently.

## Dependency and lock policy

`Cargo.lock` is committed and must be used with `--locked` in CI and release
qualification. Normal library dependencies use compatible registry ranges;
cross-repository workspace consumers use exact Git revisions or immutable
release artifacts. Moving branches are forbidden release inputs.

Developer path patches are temporary, repository-local, and gitignored. Before
release, remove the local patch file, restore any accidental lockfile changes,
and run the immutable-pin gate.

## Release sequence

1. Make the shared workspace clean and green on its declared MSRV.
2. Update the changelog and package versions together.
3. Package and inspect every releasable crate.
4. Test consumers against the exact release commit.
5. Create an annotated tag matching the package version.
6. Record the tag, commit, dependency receipt, and rollback revision.

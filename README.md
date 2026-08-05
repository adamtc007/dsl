# DSL and Semantic OS shared workspace

This repository contains the host-neutral Rust foundations used to parse and
compile the DSL and to evaluate Semantic OS contracts and policy. Applications
provide their vocabulary, graphs, policy declarations, and capability bindings
as versioned configuration; the shared crates provide mechanisms rather than a
built-in application profile.

## Dependency direction

```text
dsl_types      sem_os_types    semantic-decision-contracts
     |           /     \               /          \
     v          v       v             v            v
  dsl-core  sem_os_core  sem_os_ontology      sem_os_policy
                  \             /                 ^
                   \___________/_________________/
```

`dsl-integration-tests` is a non-published external-consumer test crate. Host
applications depend inward on these crates. Shared crates must not depend on an
application, its database schema, or its server runtime.

## Build and test

The workspace requires Rust 1.95 or newer.

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo test --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
```

The repository tracks `Cargo.lock` so CI and release qualification resolve the
same dependency graph. Library consumers should use an immutable release tag or
exact Git revision, never a moving branch.

## Using a crate from Git

Until the crates are published to a registry, pin the repository by revision:

```toml
[dependencies]
dsl-core = { git = "https://github.com/adamtc007/dsl", rev = "<commit-sha>" }
```

Local `[patch]` overrides belong in a gitignored repository-local
`.cargo/config.toml`. Do not place shared overrides in the user-global Cargo
configuration, because they silently alter unrelated workspaces and lockfiles.

## Releases and compatibility

All public packages share one version. Persistent identifiers, canonical
encodings, and hashes require explicit schema/version migrations; a crate
version change alone never silently changes stored bytes. See
[`docs/versioning.md`](docs/versioning.md) and [`CHANGELOG.md`](CHANGELOG.md).

## Licence

Licensed under the [MIT License](LICENSE).

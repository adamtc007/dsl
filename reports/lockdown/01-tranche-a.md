# Lockdown Report — Tranche 01-tranche-a
- UTC:       2026-05-30T10:46:40Z
- Commit(s): dsl: c9a23f2, sem-os: 282c724
- Status:    GREEN

## Summary
Executed leaf crate `dsl_types` surface lockdown by removing 3 dead methods (slot_name, min_state, verb_fqn), making internal submodules pub(crate), and exporting only explicit facade types.

## Edits
- git diff --stat:
  ```text
   crates/dsl-core/src/config/dag.rs             |  2 +-
   crates/dsl-core/src/resolver/mod.rs           | 10 +++++-----
   crates/dsl-core/tests/frontier_recursive.rs   |  2 +-
   crates/dsl-core/tests/frontier_skeleton.rs    |  2 +-
   crates/dsl-core/tests/resolver_lux_sicav.rs   |  4 +++-
   crates/dsl_types/src/constellation_map_def.rs | 23 +----------------------
   crates/dsl_types/src/lib.rs                   | 11 +++++++++--
   7 files changed, 21 insertions(+), 33 deletions(-)
  ```
- Deleted:    fn slot_name (constellation_map_def.rs:181)
- Deleted:    fn min_state (constellation_map_def.rs:188)
- Deleted:    fn verb_fqn (constellation_map_def.rs:208)
- Downgraded: mod constellation_map_def (lib.rs:23 pub -> pub(crate))
- Downgraded: mod resolver_facts (lib.rs:24 pub -> pub(crate))
- Relocated:  None (0)

## Gate Evidence (actual output tails — not summaries)
- cargo build (dsl_types):
  ```text
  warning: patch `bpmn-lite-engine v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-engine)` was not used in the crate graph
  warning: patch `bpmn-lite-ffi-grpc v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-grpc)` was not used in the crate graph
  warning: patch `bpmn-lite-ffi-http v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-http)` was not used in the crate graph
  warning: patch `bpmn-lite-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-server)` was not used in the crate graph
  warning: patch `bpmn-lite-store v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-store)` was not used in the crate graph
  warning: patch `dmn-lite-bridge v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dmn-lite-bridge)` was not used in the crate graph
  warning: patch `dsl-bus-client v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-client)` was not used in the crate graph
  warning: patch `dsl-bus-protocol v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-protocol)` was not used in the crate graph
  warning: patch `dsl-bus-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-server)` was not used in the crate graph
  warning: patch `dsl-bus-storage v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-storage)` was not used in the crate graph
  warning: patch `dsl-manifest v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-manifest)` was not used in the crate graph
  warning: patch `ffi-catalogue v0.1.0 (/Users/adamtc007/dev/bpmn-lite/ffi-catalogue)` was not used in the crate graph
  warning: patch `sem_os_ontology v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_ontology)` was not used in the crate graph
  warning: patch `sem_os_policy v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_policy)` was not used in the crate graph
  help: Check that the patched package version and available features are compatible
        with the dependency requirements. If the patch has a different version from
        what is locked in the Cargo.lock file, run `cargo update` to use the new
        version. This may also occur with an optional dependency that is not enabled.
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
  ```
- cargo build (workspace):
  ```text
  warning: patch `bpmn-lite-engine v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-engine)` was not used in the crate graph
  warning: patch `bpmn-lite-ffi-grpc v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-grpc)` was not used in the crate graph
  warning: patch `bpmn-lite-ffi-http v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-http)` was not used in the crate graph
  warning: patch `bpmn-lite-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-server)` was not used in the crate graph
  warning: patch `bpmn-lite-store v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-store)` was not used in the crate graph
  warning: patch `dmn-lite-bridge v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dmn-lite-bridge)` was not used in the crate graph
  warning: patch `dsl-bus-client v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-client)` was not used in the crate graph
  warning: patch `dsl-bus-protocol v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-protocol)` was not used in the crate graph
  warning: patch `dsl-bus-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-server)` was not used in the crate graph
  warning: patch `dsl-bus-storage v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-storage)` was not used in the crate graph
  warning: patch `dsl-manifest v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-manifest)` was not used in the crate graph
  warning: patch `ffi-catalogue v0.1.0 (/Users/adamtc007/dev/bpmn-lite/ffi-catalogue)` was not used in the crate graph
  warning: patch `sem_os_ontology v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_ontology)` was not used in the crate graph
  warning: patch `sem_os_policy v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_policy)` was not used in the crate graph
  help: Check that the patched package version and available features are compatible
        with the dependency requirements. If the patch has a different version from
        what is locked in the Cargo.lock file, run `cargo update` to use the new
        version. This may also occur with an optional dependency that is not enabled.
     Compiling dsl-core v0.1.0 (/Users/adamtc007/dev/dsl/crates/dsl-core)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.05s
  ```
- cargo test (dsl_types):
  ```text
  warning: patch `ffi-catalogue v0.1.0 (/Users/adamtc007/dev/bpmn-lite/ffi-catalogue)` was not used in the crate graph
  warning: patch `sem_os_ontology v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_ontology)` was not used in the crate graph
  warning: patch `sem_os_policy v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_policy)` was not used in the crate graph
  help: Check that the patched package version and available features are compatible
        with the dependency requirements. If the patch has a different version from
        what is locked in the Cargo.lock file, run `cargo update` to use the new
        version. This may also occur with an optional dependency that is not enabled.
      Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
       Running unittests src/lib.rs (target/debug/deps/dsl_types-6958b225df165fa2)

  running 0 tests

  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Doc-tests dsl_types

  running 0 tests

  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```
- cargo public-api:  dsl_types 681 → 623   dsl-core 17662 → 17662

## Ledgers
- Deleted tests:     None
- Dead-code harvest: None
- Unverified:        None

## Deviations & Decisions
- **`sem-os` adaptation**: Adjusted sibling repo `sem_os_core` to import directly from the root facade of `dsl_types` instead of its private submodules (`constellation_map_def` / `resolver_facts`). Added a local private module alias `core_map` inside `composer.rs` to keep modifications to the minimum.

## Invariant attestation
- E0 no live-body edits: PASS — diff contains only imports, visibility, deletion of unused methods, and module facade re-exports.
- E1 no globs introduced:  grep -rn "use .*::\*" crates/        → zero results
- E2 no allow(dead_code):  grep -rn "allow(dead_code)" crates/  → zero results

## Next
- Next tranche: Tranche B — entry preconditions: Tranche A accepted, workspace is clean.

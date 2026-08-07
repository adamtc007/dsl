# Semantic Gameboard Phase 3 shared-contract review

Date: 2026-08-07

- Real external consumer: BPMN-Lite `utterance-engine`, through the named
  `semantic-decision-contracts` and `semantic-pack` facades.
- Stability contract: serialized `EvidenceLane` names and canonical order are stable;
  admitted evidence, rule, and feedback declarations remain host-neutral and are part
  of the semantic-pack artifact identity.
- Reason: Phase 3 requires one complete evidence vector per legal move and pack-owned
  fusion weights, deterministic gates, explanations, and bounded recovery links.
- Visibility: no implementation module became public, no glob export was added, and no
  unchecked constructor or tooling-only production API was introduced.
- Dependency direction: unchanged. `semantic-pack` continues to depend only on shared
  contracts and types, never on an application, server, fuzzer, or orchestration crate.
- Public API snapshots: `semantic-decision-contracts` is 759 items in default and
  all-features builds; `semantic-pack` is now permanently snapshotted at 492 items in
  both builds.

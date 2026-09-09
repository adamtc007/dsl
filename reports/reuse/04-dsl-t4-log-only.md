# Reuse Tranche DSL-T4 — Logged, not built

- **Programme:** EOP-PLAN-CA-REUSE-001 (core fixes from the reuse recon)
- **UTC:** 2026-09-09
- **Status:** LOGGED (no code; one paragraph each with finding citations, for the ledger)

## T4-1 Split `sem_os_postgres` / `sem_os_server` from the 81 ob-poc domain ops

Recon R-20 (§3): `sem_os_postgres` is 44,739 LOC whose `[dependencies]`
include `ob-poc-types`, `entity-gateway`, `dsl-runtime`, `dsl-analysis`,
`ob-poc-semantic-policy` and `tonic`; 81 files under `src/ops/` are ob-poc
domain ops (`deal.rs`, `cbu.rs`, `kyc_case.rs`, `billing.rs`, …), 141
`UPDATE`/`DELETE` sites, and the `"ob-poc".` schema literal appears hundreds
of times (`ops/deal.rs` 91, `ops/cbu.rs` 74, …). The generic port
implementations (`PgSnapshotStore`, `ObjectStore`, `ChangesetStore`,
`AuditStore` at `store.rs:135,730,774,1069`, 0 `UPDATE`/`DELETE`) are fused
into the same crate, and `sem_os_server` inherits the whole graph
(`crates/sem_os_server/Cargo.toml`). Proposed shape: a `sem_os_postgres`
crate in this repository holding only the port implementations against a
schema name supplied at construction (no literal), with the 81 ops staying in
ob-poc as `ob-poc-postgres-ops`; `sem_os_server` then depends on the generic
crate plus a host-supplied op registry. Precondition: the `sem_os_core::ports`
traits are already host-neutral (`ports.rs:11-143`), so the split is a move,
not a redesign. Blocked on ob-poc agreeing the schema-name parameter and on
the `entity-gateway` `clippy::result_large_err` failure being fixed on its
side.

## T4-2 A generic `semantic-decision-walk` enumerator over a built graph

Recon R-13 (§1.6) and §6 item 1: no enumerator exists in the core. The only
one is `enumerate_placement_set(subject, &ControlState, &TypeRegistryState,
&LexiconManifest) -> PlacementSet` in
`ob-poc-kyc-substrate/src/placement.rs:450-457`, with substrate-local output
types (`PlacementSet`, `LegalMove` at `:174-181`, `:125-165`) and hardcoded
FQN dispatch (`is_edge_scoped :205`, `is_geometry_gated :239-241`, consts
`:276-284`, `is_type_registry_move :293`). The core has the *contract* it
should fill (`semantic-decision-contracts::{DesignPosition, LegalMove}`,
`gameboard.rs:1076,796`) but nothing that fills it. Proposed shape: a
`semantic-decision-walk` crate taking (a) a compiled `semantic-pack` graph
(now with data-declared capability segments, DSL-T3), (b) an abstract fold
state supplied by the domain through a trait, and (c) a role and a clock
reached-set, and emitting a `DesignPosition` whose `legal_moves` are computed
by lexicon-declared move *shapes* rather than FQN `match`es. CA will
contribute the first implementation ("current node × clock reached-set ×
role"); it lands here only once it is proven domain-free by the DSL-T2 gate.

## T4-3 A generic pack-closure audit crate (K-G1..G7)

Recon R-14 (§1.6) and §6 item 8: the closure gates K-G1 (unreachable block)
… K-G7 (fold-blind write) are defined in
`ob-poc/docs/todo/EOP-DD-KYCUBO-KIT-T0.3_Pack-Closure-Audit_v0.1.md:34-81`
and enforced only by ob-poc scripts and tests (`rust/tests/kyc_pack_closure.rs`,
`xtask/src/kyc_alignment.rs` V1–V6, `scripts/check_kyc_substrate_deps.sh`,
`check_kyc_decide_deps.sh`, `check_no_retired_kyc_fqn.sh`). There is no
generic "is this pack closed" checker. Proposed shape: a `semantic-pack-audit`
crate consuming a `CompiledPack` (graph + capabilities + segments) and a
domain-supplied lexicon/fold declaration, producing a typed finding list keyed
by the seven gate ids, runnable as a library from any domain's CI. Depends on
T4-2's abstract fold-state trait for K-G7.

## T4-4 A proposal tier whose input is an enumerated `LegalMove` set

Recon §4.2 (ob-poc-agent): `PlanningLoop::propose_draft(&self, utterance,
existing)` (`planning.rs:131-135`) bounds the draft's verb by
`SessionIndex::allowed_verbs()` = `pack.allowed_verbs` (`index.rs:65-70`), a
pack allowlist, not a state-enumerated legal-move set, and never takes a
`PlacementSet`/`DesignPosition`. The interpretation tier is also keyed on
ob-poc's closed `WorkspaceKind` enum (`ob-poc-types/src/session/kinds.rs:20-31`)
and carries CBU/KYC prompt material (`sage/llm_sage.rs:591-592`), so it is
not importable (recon §5). Proposed shape: a host-neutral proposal tier in
this repository whose only legality input is `DesignPosition::legal_moves`
(+ `move_set_hash`) from T4-2, with the utterance→move ranking supplied as a
trait so no prompt material or workspace enum lives in the core. Sequenced
after T4-2; ob-poc's `PlanningLoop` becomes a consumer.

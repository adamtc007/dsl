# Reuse Tranche DSL-T2 — Remove domain nouns and floats from `dsl-core` and `sem_os_policy`

- **Programme:** EOP-PLAN-CA-REUSE-001 (core fixes from the reuse recon)
- **UTC:** 2026-09-09
- **Findings:** `ob-poc/docs/eop/reuse-recon-ob-poc.md` §1.3 `SourceOfTruth::KycStream` (`dsl-core/src/config/types.rs:829-833`); §1.4 `f64` threshold predicates (`types.rs:415-421`, `:1219,1222`, `:1951,1953`, `escalation.rs:84-99,160-163`, `predicate/parser.rs:511`); §1.3 hardcoded `"kyc-case/workstream"` in `sem_os_policy/src/state_simulation.rs:175-176`; R-00d (the repo's own leakage register)
- **Status:** GREEN

## 1. `SourceOfTruth` is data-carried

`SourceOfTruth::KycStream` is replaced by `SourceOfTruth::Stream(String)`: an
opaque identifier naming the domain's durable append-only verb stream. The
enum no longer derives `Copy` (it carries a `String`); `Clone`/`PartialEq`/
`Eq` are unchanged. Serde goes through a private untagged wire type
(`SourceOfTruthWire`: unit variants as `snake_case` strings, the stream as a
single-key map) so the same bytes parse under serde_yaml and serde_json
without YAML `!tag` syntax:

```yaml
source_of_truth: workflow
source_of_truth: { stream: kyc_intent_events }   # was: kyc_stream
```

The doc block that named a domain (`types.rs:806-833`) is rewritten in terms
of verb families. The repository's own fixture catalogue
(`config/verbs/kyc/dsl-kyc.yaml`, 13 verbs) is migrated to the new form so
the `catalogue_db_free_smoke` tests load it.

## 2. Thresholds are fixed-point decimals

Crate: `rust_decimal` 1.42 (already a `dsl-core` dependency and in
`Cargo.lock`; 96-bit scaled integer, exact decimal text round trip, `serde`
feature available; no new dependency and no licence change). Not chosen:
`bigdecimal` (not in the graph; arbitrary precision is not needed for
thresholds).

| site | before | after |
|---|---|---|
| `EscalationPredicate::{ArgGt,ArgGte,ArgLt,ArgLte}.value` | `f64` | `Decimal` |
| `ArgValidation::{min,max}` | `Option<f64>` | `Option<Decimal>` |
| `RuleRequirement::{greater_than,less_than}` (crate-private CSG rules) | `Option<f64>` | `Option<Decimal>` |
| `escalation.rs::as_f64` | `Value::as_f64()` | `as_decimal`: JSON numbers via their exact text, strings via `Decimal::from_str`; anything else `None` (predicate false) |
| `predicate/parser.rs:511` literal sniff | `parse::<f64>()` | `Decimal::from_str` (so `inf`/`NaN`/`1e5` are no longer numbers) |
| `ast.rs:856` resolution percentage | `as f64 … * 100.0` | integer arithmetic |

Serde is through a new crate-private `config::decimal_text` helper: values
serialise as decimal text (`"0.25"`), deserialise from integer literals or
decimal text, and **reject floating-point literals** with the message
`floating-point threshold … rejected: write it as an integer or a quoted
decimal string`. No `f64` is constructed anywhere on the parse, evaluate or
compare path. Existing YAML with integer thresholds (all of ob-poc's:
`value: 10000000`, `min: 1`, `max: 100`, `greater_than: 100`) is unchanged;
a YAML author who needs a fraction writes `value: "0.25"`.

Remaining `f64` in the touched crates, deliberately left: `FiniteScore(f64)`
and `compute_verb_prominence -> f64` are ranking scores, not predicates
(recon §1.4 rates them clean).

## 3. `sem_os_policy` simulation target is pack data

`DomainTransition` gains two optional, wire-compatible fields
(`#[serde(default, skip_serializing_if)]`, so existing manifests parse and
re-serialise byte-identically):

```yaml
allowed_transitions:
  - transition_ref: …
    slot_path: kyc-case/workstream   # optional: slot this transition advances
    node_prefix: kyc-case            # optional: predicted node is "<prefix>:<state>"
```

`simulate_transition_from_pack` now emits
`DomainTransition::advance_node(state)` / `advance_slot_path()`. When a pack
declares nothing, the target is derived uniformly from data already in the
transition: `<entity_type>:<state>` and `<entity_type>/<state_machine>`. There
is no built-in name.

**Downstream consequence (Prompt C):** ob-poc's manifests
(`rust/config/sem_os_seeds/domain_packs/*.yaml`) declare no `slot_path`, so
its dry-run output changes from `kyc-case:discovery` /
`kyc-case/workstream` to `kyc_case:discovery` / `kyc_case/kyc_case_lifecycle`
until the manifest declares the two fields. `ob-poc-boundary/src/kyc_dry_run.rs:204`
asserts the old value.

## 4. Entity-kind alias table is request data

`sem_os_policy::context_resolution::canonicalize_entity_kind` was a
hardcoded `match` over domain spellings (`"kyc_case" | "case" => "kyc-case"`,
`"client-business-unit" | … => "cbu"`, …). It is replaced by
`EntityKindAliases` (a serde-transparent `BTreeMap<String, String>`) carried
on `ContextResolutionRequest::entity_kind_aliases` (`#[serde(default)]`) and
`VerbFilterContext::entity_kind_aliases`. Canonicalisation is trim +
ASCII-lower-case, then a table lookup; the core ships no aliases.

**Downstream consequence (Prompt C):** a host that relied on the built-in
aliases must pass its own table on the request; with an empty table only
exact (case-insensitive) matches survive.

## 5. Domain-noun gate with an empty allowlist

`scripts/check-domain-nouns.sh` (wired into the CI `boundaries` job) scans
every `crates/*/src/**/*.rs` outside `tests/`, `integration_tests/`, `fuzz/`
and `#[cfg(test)] mod …` for `kyc`, `cbu`, `onboard(ing)`, `ca`, `ubo`
(case-insensitive; the mixed-case `OnBoard` in
`semantic-decision-contracts` is excluded by spelling because it is "on the
board", recon §1.3). `.ci/domain-noun-allowlist.txt` is committed empty and
the gate fails if it is ever non-empty.

Non-test hits removed to make the gate green (docs and comments rewritten
with neutral examples such as `account`, `review_case`, `owner`, `intake`):

| crate | files |
|---|---|
| dsl-core | `config/types.rs` (28 lines incl. `KycStream`), `config/escalation.rs`, `config/resource_dependency.rs`, `config/runbook_composition.rs`, `config/manifest.rs`, `config/phrase_gen.rs`, `config/predicate/ast.rs`, `executable_plan.rs`, `viewport_parser.rs`, `binding_context.rs`, `ast.rs` |
| dsl_types | `dag.rs` (5 lines) |
| sem_os_types | `lib.rs` (2) |
| sem_os_core | `ids.rs` (doctest example strings) |
| sem_os_ontology | `attribute_def.rs`, `evidence.rs`, `membership.rs`, `verb_contract.rs`, `requirement_profile_def.rs` |
| sem_os_policy | `state_simulation.rs` (code), `context_resolution.rs` (code + 3 docs), `grounding.rs` (doctest YAML), `diagram/{mermaid,model}.rs`, `observatory/orientation.rs` |

The pre-existing broader-token gate (`check-domain-neutral.sh`, 71-file
reviewed-debt allowlist for `mandate`, `steward`, `pricing`, …) is unchanged
and still green; no allowlist entry became stale because every listed file
retains test-module hits.

## Tests that can fail

| acceptance item | test |
|---|---|
| grep check green | `scripts/check-domain-nouns.sh` (CI) — fails on any hit or any allowlist entry |
| predicate tests re-run on decimal | `dsl-core` in-crate `config::escalation::tests::*` (thresholds now `Decimal`), new `decimal_thresholds_compare_exactly_without_floats`; `config::decimal_text::tests::*` (integer/quoted-decimal accepted, float rejected, JSON round trip); `crates/dsl-core/tests/decimal_thresholds.rs` (YAML `ThreeAxisDeclaration`, `ArgValidation`, float rejection, `SourceOfTruth::Stream` round trip) |
| simulation target from data | `sem_os_policy::state_simulation::tests::{undeclared_advance_target_is_derived_from_transition_data, advance_target_fields_are_optional_on_the_wire}` + existing declared-path assertions |
| catalogue still loads | `dsl-core` `catalogue_db_free_smoke::*` against the migrated fixture |

## Verification

- `env -u DSL_CONFIG_DIR cargo test --workspace --all-targets --all-features --locked` — green. (With this machine's `DSL_CONFIG_DIR=~/Developer/ob-poc/rust/config` the three smoke tests read ob-poc's un-migrated `dsl-kyc.yaml` and fail on `kyc_stream`; that is the Prompt C migration, not a repo defect, and CI sets no such variable.)
- `cargo clippy … -D warnings` and `RUSTDOCFLAGS=-D warnings cargo doc` — clean for `dsl_types`, `dsl-core`, `sem_os_types`, `sem_os_core`, `sem_os_ontology`, `sem_os_policy`.
- `check-layering.sh`, `check-dependencies.sh`, `check-domain-neutral.sh`, `check-domain-nouns.sh`, `check-public-api-baselines.sh` — OK.

## Public-API diff (`cargo public-api --simplified`, `97ac3ae` → this tranche)

`dsl-core` (3392 → 3391 lines):

```text
- pub dsl_core::EscalationPredicate::{ArgGt,ArgGte,ArgLt,ArgLte}::value: f64
+ pub dsl_core::EscalationPredicate::{ArgGt,ArgGte,ArgLt,ArgLte}::value: rust_decimal::Decimal
- pub dsl_core::SourceOfTruth::KycStream
+ pub dsl_core::SourceOfTruth::Stream(String)
- impl Copy for dsl_core::SourceOfTruth
- pub dsl_core::ArgValidation::{min,max}: Option<f64>
+ pub dsl_core::ArgValidation::{min,max}: Option<rust_decimal::Decimal>
```

`sem_os_policy` (6967 → 7001 lines):

```text
+ pub struct context_resolution::EntityKindAliases  (+ new/canonicalize/len/is_empty, Clone/Eq/Default/Debug/Serialize/Deserialize)
+ pub context_resolution::ContextResolutionRequest::entity_kind_aliases: EntityKindAliases
+ pub context_resolution::VerbFilterContext::entity_kind_aliases: &'a EntityKindAliases
+ pub domain_pack::DomainTransition::{slot_path, node_prefix}: Option<String>
+ pub fn domain_pack::DomainTransition::{advance_node(&str), advance_slot_path()} -> String
```

## Not done here (logged in `reports/reuse/FOLLOW-UPS.md`)

- ob-poc-side migrations listed in §1, §3, §4 (Prompt C).
- Ranking scores stay `f64` (not predicates).

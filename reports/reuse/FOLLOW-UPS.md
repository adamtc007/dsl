# Reuse programme — follow-ups logged, not fixed

Format: `id · file:line · severity · finding · proposed tranche`.

| id | file:line | severity | finding | proposed tranche |
|---|---|---|---|---|
| FU-T1-01 | `crates/dsl-parser/src/parser.rs:472-512` | low | Qualified names `pack/atom` are recognised by treating *any* lex-error token after a symbol as `/`; any other stray character after a symbol is mis-parsed as a qualified name. | DSL-T5 (parser hygiene): lex `/` as a token. |
| FU-T1-02 | `crates/dsl-parser/src/raw_ast.rs:32`, `lexer.rs:86` | low | `RawValue::FloatLit(f64)` lexed with `parse::<f64>()`; the grammar has no exact decimal literal. Money literals must be strings/ints at the frontend. | DSL-T5: add a `DecimalLit` token (string-preserving) and deprecate `FloatLit`. |
| FU-T1-03 | `crates/semantic-decision-contracts/src/gameboard.rs:331`, `src/lib.rs` | low | Pre-existing rustfmt drift at `c357082`; the CI `format` job is red independent of this programme. Reverted the incidental reformat out of T1 to keep the diff scoped. | Housekeeping commit on `main`. |
| FU-T2-01 | ob-poc `rust/config/verbs/kyc/dsl-kyc.yaml` (13 verbs) | medium | `source_of_truth: kyc_stream` no longer parses; write `source_of_truth: { stream: kyc_intent_events }`. | Prompt C (ob-poc rewire). |
| FU-T2-02 | ob-poc `rust/config/sem_os_seeds/domain_packs/ob_poc_kyc.yaml`; `ob-poc-boundary/src/kyc_dry_run.rs:204` | medium | Simulated advance target is now pack data; declare `slot_path: kyc-case/workstream` and `node_prefix: kyc-case` on the transition, or accept the derived `kyc_case:<state>` / `kyc_case/kyc_case_lifecycle`. | Prompt C. |
| FU-T2-03 | ob-poc callers of `ContextResolutionRequest` (14 construction sites) | low | Entity-kind aliases are request data (`entity_kind_aliases`); pass ob-poc's alias table or accept exact-match canonicalisation. | Prompt C. |
| FU-T2-04 | `crates/sem_os_policy/src/context_resolution.rs` `compute_verb_prominence -> f64`; `semantic-decision-contracts/src/lib.rs:759` `FiniteScore(f64)` | low | Ranking scores remain floats; they are not predicates or money and were rated clean by the recon. | None unless scores become comparable contract values. |


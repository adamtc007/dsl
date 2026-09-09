# Reuse programme — follow-ups logged, not fixed

Format: `id · file:line · severity · finding · proposed tranche`.

| id | file:line | severity | finding | proposed tranche |
|---|---|---|---|---|
| FU-T1-01 | `crates/dsl-parser/src/parser.rs:472-512` | low | Qualified names `pack/atom` are recognised by treating *any* lex-error token after a symbol as `/`; any other stray character after a symbol is mis-parsed as a qualified name. | DSL-T5 (parser hygiene): lex `/` as a token. |
| FU-T1-02 | `crates/dsl-parser/src/raw_ast.rs:32`, `lexer.rs:86` | low | `RawValue::FloatLit(f64)` lexed with `parse::<f64>()`; the grammar has no exact decimal literal. Money literals must be strings/ints at the frontend. | DSL-T5: add a `DecimalLit` token (string-preserving) and deprecate `FloatLit`. |
| FU-T1-03 | `crates/semantic-decision-contracts/src/gameboard.rs:331`, `src/lib.rs` | low | Pre-existing rustfmt drift at `c357082`; the CI `format` job is red independent of this programme. Reverted the incidental reformat out of T1 to keep the diff scoped. | Housekeeping commit on `main`. |

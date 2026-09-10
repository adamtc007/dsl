# Reuse Tranche DSL-T5 — Parser hang and `=` lexing

- **Programme:** EOP-PLAN-CA-REUSE-001 (core fixes found by CA's T0.2 A.12 v0.3 rewrite)
- **Findings:** ledger L23 (P1): `dsl-parser` loops forever on an unlexable
  token inside a bracketed list — `(x :a [<])`, `(x :a [= 0])`,
  `(x :a [(< y)])`. Ledger L24 (low): `=` inside a symbol lexes as the
  pack/atom qualified-name separator with no diagnostic. L23/L24 correspond
  to `reports/reuse/FOLLOW-UPS.md` FU-T1-01 (`parser.rs:472-512`, logged at
  DSL-T1: "any lex-error token after a symbol" mis-parses as `/`).
- **Status:** GREEN

## 1. L23 (P1) — error recovery must always make progress

Cause, as cited: `parse_list`'s error recovery (`parser.rs`, was
lines 536-567) broke on `]` without consuming it once `parse_value`
returned `None` for an unlexable token. The unconsumed `]` then reached
`parse_atom_body`'s positional-value branch, which calls `parse_value` again;
`parse_value`'s "cannot start a value" arm (`CloseBracket`/`CloseBrace`/
`CloseParen`/`Keyword`/`Arrow`/`Slash`) intentionally returns `None` without
consuming — correct when the caller is a dispatch loop that reads the token
itself (e.g. `CloseParen` ending an atom), wrong when the caller has no arm
for it. The atom body's `_` (positional) branch had no arm for `]`, so it
called `parse_value`, got `None`, consumed nothing, and re-peeked the same
`]` forever.

The same shape existed in three more places once a stray, unhandled closing
delimiter reached them: `parse_map`'s "missing value for `:key`" path broke
without consuming `}`; `parse_for_each_body`'s body loop and
`parse_atom_body`'s own positional-value loop had no fallback consume at
all. All four are the same defect, not four different bugs, and the fuzz
test below (written to catch exactly this class) found the `parse_atom_body`
instance independently of the three findings-cited inputs
(`"(yy}a)bf<f ay. [f{bf)\""`, found before the fix, terminates after it).

Fix, applied uniformly at all four sites: on `None` from `parse_value`,
consume the token it left behind unless it is the enclosing loop's own
terminator or EOF (those are handled correctly by looping back to the top).
This guarantees each loop iteration either parses a value, matches its
terminator, or consumes exactly one token — so every one of these loops is
now bounded by the remaining token count, not by finding a specific
character.

Regression tests (`crates/dsl-parser/tests/progress_invariant.rs`):
- The three findings-cited inputs, each run on a background thread with a
  2s timeout (`l23_minimal_regressions_terminate_with_a_diagnostic`) and
  asserted to still recover the outer atom with a diagnostic.
- A general progress-invariant fuzz test
  (`bounded_fuzz_inputs_always_terminate`): a deterministic xorshift PRNG
  (no new dependency) generates 2,000 strings up to 24 bytes from an
  alphabet covering every structural and unrecognised character in the
  grammar (`()[]{}: <>=/,@$` plus symbol/literal characters); each is parsed
  on a background thread with a 500ms timeout. Stress-run separately outside
  the suite at 50 seeds × 3,000 inputs × 40 bytes (150,000 inputs) with no
  hang, before trimming to the committed budget.

## 2. L24 — `=` is a diagnosed lex error, not a silent qualified name

Decision: **require an explicit `/` token for qualified names**, rather than
rejecting `=` in symbols — `=` was never part of the `Symbol` regex (dashes,
dots, alphanumerics only), so there was nothing to "reject in a symbol"; the
defect was in the parser, which treated *any* lex-error token following a
`Symbol` as if it were `/` (`parser.rs`, was lines 491-512, comment: "Speculatively... peek for
Error token that came from `/`"). This kept every existing fixture parsing
unchanged, because the only production use of `pack/atom` syntax anywhere in
the workspace is the parser crate's own `RawValue::QualifiedName` type and
its `{pack}/{atom}` printer in `crates/dsl-parser/tests/keyword_slots.rs:34`
— no config fixture or other crate constructs one.

Change: the lexer gains an explicit `#[token("/")] Slash` variant
(`crates/dsl-parser/src/lexer.rs`). The parser's `Token::Symbol` arm now
checks specifically for `Token::Slash`, not `Some(Err(()))`; a `/` not
followed by a `Symbol` is now a diagnosed error ("Expected symbol after '/'
in qualified name") instead of being silently swallowed. Any other
unrecognised character (`=`, `<`, `>`, …) next to a symbol is left in the
stream and diagnosed wherever it is next examined, per §1's fix, instead of
being absorbed into a `QualifiedName` with no diagnostic.

Regression tests: `l24_bare_slash_forms_a_qualified_name` (unchanged
behaviour for the one real use) and
`l24_equals_sign_is_a_diagnosed_lex_error_not_a_silent_qualified_name`
(`foo=bar` now produces a diagnostic and is never
`QualifiedName { pack: "foo", atom: "bar" }`).

## Domain neutrality

No domain nouns introduced; the change is entirely in `dsl-parser`'s lexer
and parser internals and its test fixtures.

## Public-API diff (`cargo public-api --simplified`, `5925702` → this tranche)

| crate | change |
|---|---|
| dsl-parser | `+ Token::Slash` (new lexer token variant). No removals; `Token` is `#[non_exhaustive]`-free so this is source-breaking for an exhaustive external `match Token`, of which there are none in this workspace. |

## Verification

- `env -u DSL_CONFIG_DIR cargo test --workspace --all-targets --all-features --locked` — green (all crates, including the new `progress_invariant.rs`).
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps --all-features` — clean.
- `check-layering.sh`, `check-dependencies.sh`, `check-domain-neutral.sh`, `check-domain-nouns.sh`, `check-packages.sh` — OK, unchanged.
- `check-public-api-baselines.sh --update` then re-run clean; only `scripts/baselines/dsl-parser-public-api-v1.txt` changed (`+Token::Slash`).

## Follow-ups

FU-T1-01 is resolved by this tranche and removed from `FOLLOW-UPS.md`. No
new follow-ups: the fix is generic (loop-progress invariant), not
domain-specific, and needed no new special cases.

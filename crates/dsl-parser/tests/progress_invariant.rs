//! DSL-T5 regression tests (ledger L23/L24, found by CA's T0.2 A.12 rewrite).
//!
//! L23 (P1): an unlexable token inside a bracketed list made `parse_list`'s
//! error recovery break on `]` without consuming it, so the enclosing atom
//! body's slot loop re-peeked the same unconsumed `]` forever. The fix makes
//! every value-loop's recovery path consume at least one token per iteration
//! that is not its own terminator, so it always makes progress.
//!
//! L24: `=` (or any unrecognised character) immediately after a symbol was
//! silently treated as the `pack/atom` qualified-name separator, with no
//! diagnostic. The fix requires an explicit `/` token for qualified names.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Runs `dsl_parser::parse` on a background thread and fails the test if it
/// does not return within `budget` — the direct regression check for the
/// hang in L23.
fn parse_terminates_within(src: &str, budget: Duration) {
    let owned = src.to_owned();
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let result = dsl_parser::parse(&owned);
        // Ignore a closed receiver: the main thread already timed out.
        let _ = tx.send(result);
    });
    match rx.recv_timeout(budget) {
        Ok(_) => {
            // Don't block the test on a background thread that, per the
            // above, already returned; join is just cleanup.
            let _ = handle.join();
        }
        Err(_) => panic!("parse did not terminate within {budget:?} for input: {src:?}"),
    }
}

#[test]
fn l23_minimal_regressions_terminate_with_a_diagnostic() {
    for src in ["(x :a [<])", "(x :a [= 0])", "(x :a [(< y)])"] {
        parse_terminates_within(src, Duration::from_secs(2));

        let (sf, diag) = dsl_parser::parse(src);
        assert!(diag.has_errors(), "expected a diagnostic for {src:?}");
        assert_eq!(
            sf.atoms.len(),
            1,
            "expected the outer atom to still be recovered for {src:?}"
        );
        assert_eq!(sf.atoms[0].kind, "x");
    }
}

#[test]
fn l24_bare_slash_forms_a_qualified_name() {
    let (sf, diag) = dsl_parser::parse("(x :a pack/atom)");
    assert!(
        !diag.has_errors(),
        "diagnostics: {:?}",
        diag.errors()
            .map(|d| d.message.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        sf.atoms[0].slots[0].1,
        dsl_parser::RawValue::QualifiedName {
            pack: "pack".to_owned(),
            atom: "atom".to_owned(),
        }
    );
}

#[test]
fn l24_equals_sign_is_a_diagnosed_lex_error_not_a_silent_qualified_name() {
    // Before the fix, `foo=bar` silently parsed as QualifiedName { pack:
    // "foo", atom: "bar" } with no diagnostic. Now `=` is left as an
    // unrecognised character and diagnosed wherever it is next examined.
    let (sf, diag) = dsl_parser::parse("(x :a foo=bar)");
    assert!(
        diag.has_errors(),
        "expected `=` next to a symbol to be diagnosed, not silently accepted"
    );
    assert_ne!(
        sf.atoms[0].slots.first().map(|(_, v)| v.clone()),
        Some(dsl_parser::RawValue::QualifiedName {
            pack: "foo".to_owned(),
            atom: "bar".to_owned(),
        }),
        "`=` must never be treated as the qualified-name separator"
    );
}

/// General progress invariant: `parse` must terminate on any input of
/// bounded length, however malformed. Deterministic xorshift PRNG so the
/// fuzz is reproducible without adding a fuzzing dependency.
#[test]
fn bounded_fuzz_inputs_always_terminate() {
    const ALPHABET: &[u8] = b"()[]{}: <>=/,@$abcxy01.-\"tf\n";
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next_u64 = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    for _ in 0..2000 {
        let len = (next_u64() % 24) as usize;
        let src: String = (0..len)
            .map(|_| ALPHABET[(next_u64() as usize) % ALPHABET.len()] as char)
            .collect();
        parse_terminates_within(&src, Duration::from_millis(500));
    }
}

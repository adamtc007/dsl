//! The determinism property every walk engine over this contract must satisfy: same expanded board,
//! same node (or entry), same reached-set, same role ⇒ the same legal-step list, value-equal, on
//! every call. Exposed here so a domain calls it from its own test binary — the same shape
//! `sem_os_append_store::conformance::run_all` uses for a store implementation, one level down: today
//! there is one walk engine (the row walk); if a second (a frame walk) is ever built over this same
//! contract, this is the suite both must pass, not a new one per engine.

use std::collections::BTreeSet;

use crate::expand::Expanded;
use crate::walk::{legal_entry_moves, legal_moves};

/// Calls the walk twice over identical inputs and asserts the result is exactly equal — panics on
/// violation, meant to be called from the implementer's own test.
pub fn assert_deterministic(x: &Expanded, node: &str, reached: &BTreeSet<String>, role: &str) {
    let a = legal_moves(x, node, reached, role);
    let b = legal_moves(x, node, reached, role);
    assert_eq!(
        a,
        b,
        "the walk is not deterministic for board {}, node {node}, role {role}",
        x.label()
    );
}

/// The entry-edge form of [`assert_deterministic`].
pub fn assert_entry_deterministic(x: &Expanded, reached: &BTreeSet<String>, role: &str) {
    let a = legal_entry_moves(x, reached, role);
    let b = legal_entry_moves(x, reached, role);
    assert_eq!(
        a,
        b,
        "the entry walk is not deterministic for board {}, role {role}",
        x.label()
    );
}

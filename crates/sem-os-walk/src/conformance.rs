//! The determinism property every walk engine over this contract must satisfy: same expanded board,
//! same node (or entry), same reached-set, same role ⇒ the same legal-step list, value-equal, on
//! every call. Exposed here so a domain calls it from its own test binary — the same shape
//! `sem_os_append_store::conformance::run_all` uses for a store implementation, one level down: today
//! there is one walk engine (the row walk); if a second (a frame walk) is ever built over this same
//! contract, this is the suite both must pass, not a new one per engine.

use std::collections::BTreeSet;

use crate::dialect::Pred;
use crate::expand::{Expanded, XEdge};
use crate::walk::{blocked_on, enabled_moves, legal_entry_moves, legal_moves, Step};

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

/// `blocked_on`'s determinism: the same inputs and the same precondition answers give the same blocked
/// list, value-equal, on every call.
pub fn assert_blocked_deterministic(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    holds: &dyn Fn(&XEdge, &Pred) -> bool,
) {
    let a = blocked_on(x, node, reached, role, holds);
    let b = blocked_on(x, node, reached, role, holds);
    assert_eq!(
        a,
        b,
        "blocked_on is not deterministic for board {}, node {node}, role {role}",
        x.label()
    );
}

/// The partition every walk engine over this contract must keep: every edge out of `node` is in
/// exactly one of [`enabled_moves`] and [`blocked_on`], each blocked edge names at least one reason, and
/// both lists keep the board's own edge order. With every precondition holding, the enabled set is
/// [`legal_moves`] — the walk's legality and its blocked view never disagree.
pub fn assert_partition(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    holds: &dyn Fn(&XEdge, &Pred) -> bool,
) {
    let enabled = enabled_moves(x, node, reached, role, holds);
    let blocked = blocked_on(x, node, reached, role, holds);
    let out: Vec<Step> = x.out_edges(node).map(Step::from).collect();
    assert!(
        blocked.iter().all(|b| !b.on.is_empty()),
        "a blocked edge names no reason on board {}, node {node}",
        x.label()
    );
    let mut merged: Vec<&Step> = Vec::with_capacity(out.len());
    let (mut i, mut j) = (0, 0);
    for step in &out {
        if enabled.get(i) == Some(step) {
            merged.push(&enabled[i]);
            i += 1;
        } else if blocked.get(j).map(|b| &b.step) == Some(step) {
            merged.push(&blocked[j].step);
            j += 1;
        }
    }
    assert!(
        i == enabled.len() && j == blocked.len() && merged.len() == out.len(),
        "enabled ({}) and blocked ({}) do not partition the {} edges out of {node} on board {}",
        enabled.len(),
        blocked.len(),
        out.len(),
        x.label()
    );
    assert_eq!(
        enabled_moves(x, node, reached, role, &|_, _| true),
        legal_moves(x, node, reached, role),
        "with every precondition holding, the enabled set is not the legal set on board {}, node \
         {node}, role {role}",
        x.label()
    );
}

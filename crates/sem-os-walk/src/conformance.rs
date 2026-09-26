//! The determinism property every walk engine over this contract must satisfy: same expanded board,
//! same node (or entry), same reached-set, same role ⇒ the same legal-step list, value-equal, on
//! every call. Exposed here so a domain calls it from its own test binary — the same shape
//! `sem_os_append_store::conformance::run_all` uses for a store implementation, one level down: today
//! there is one walk engine (the row walk); if a second (a frame walk) is ever built over this same
//! contract, this is the suite both must pass, not a new one per engine.

use std::collections::BTreeSet;

use crate::expand::Expanded;
use crate::walk::{
    blocked_on, classify, enabled_moves, legal_entry_moves, legal_moves, Blocked, BlockedOn,
    PreAnswer, PreEvaluator, Step,
};

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
    pre: &PreEvaluator<'_>,
) {
    let a = blocked_on(x, node, reached, role, pre);
    let b = blocked_on(x, node, reached, role, pre);
    assert_eq!(
        a,
        b,
        "blocked_on is not deterministic for board {}, node {node}, role {role}",
        x.label()
    );
}

/// The partition a walk engine's own view of a node must keep: given the `enabled` steps and `blocked`
/// edges the engine produced for (`node`, `reached`, `role`, `pre`), every edge out of `node` is in
/// exactly one of them, in board order, and each is where this contract classifies it — decided per
/// edge by position, so edges that render as equal steps are never confused. Each blocked edge names
/// at least one reason. With every precondition holding, the contract's enabled set is
/// [`legal_moves`]. `pre` must be pure.
pub fn assert_partition(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    pre: &PreEvaluator<'_>,
    enabled: &[Step],
    blocked: &[Blocked],
) {
    let classified = classify(x, node, reached, role, pre);
    let want_enabled: Vec<Step> = classified
        .iter()
        .filter(|(_, on)| on.is_empty())
        .map(|(e, _)| Step::from(*e))
        .collect();
    let want_blocked: Vec<(Step, &Vec<BlockedOn>)> = classified
        .iter()
        .filter(|(_, on)| !on.is_empty())
        .map(|(e, on)| (Step::from(*e), on))
        .collect();
    assert_eq!(
        enabled,
        want_enabled.as_slice(),
        "the enabled steps out of {node} on board {} are not the contract's",
        x.label()
    );
    assert_eq!(
        blocked.len(),
        want_blocked.len(),
        "{} blocked edges out of {node} on board {}, the contract has {}",
        blocked.len(),
        x.label(),
        want_blocked.len()
    );
    for (b, (step, on)) in blocked.iter().zip(&want_blocked) {
        assert!(
            !b.on.is_empty(),
            "a blocked edge names no reason on board {}, node {node}",
            x.label()
        );
        assert_eq!(
            &b.step, step,
            "blocked edges out of {node} out of board order"
        );
        assert_eq!(&b.on, *on, "edge {} blocked on other reasons", step.edge);
    }
    assert_eq!(
        enabled.len() + blocked.len(),
        classified.len(),
        "enabled and blocked do not cover the edges out of {node} on board {}",
        x.label()
    );
    assert_eq!(
        enabled_moves(x, node, reached, role, &|_, _, _| PreAnswer::Holds),
        legal_moves(x, node, reached, role),
        "with every precondition holding, the enabled set is not the legal set on board {}, node \
         {node}, role {role}",
        x.label()
    );
}

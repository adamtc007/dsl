//! The walk's core reads: the monotone reached-sets a clock-shape admits, and the edges available
//! from a node under a reached-set for a role. Pure functions over the expanded board — no store, no
//! clock, no domain type: legality here reads only the reached-set and the guard/role surface.

use std::collections::BTreeSet;

use crate::dialect::Pred;
use crate::expand::{Expanded, GuardKind, XEdge};

/// The clock partial order, transitively closed: `(earlier, later)`.
pub struct ClockOrder {
    before: BTreeSet<(String, String)>,
}

impl ClockOrder {
    /// `coord_order` is the declared partial order over clock coordinates, `(earlier, later)` pairs
    /// — a domain's own coordinate taxonomy, read once by the caller and handed in verbatim; this
    /// type knows nothing about where that taxonomy lives.
    pub fn new(coord_order: &[(String, String)]) -> Self {
        let mut before: BTreeSet<(String, String)> = coord_order.iter().cloned().collect();
        loop {
            let pairs: Vec<_> = before.iter().cloned().collect();
            let mut grew = false;
            for (a, b) in &pairs {
                for (c, d) in &pairs {
                    if b == c && before.insert((a.clone(), d.clone())) {
                        grew = true;
                    }
                }
            }
            if !grew {
                return Self { before };
            }
        }
    }
    pub fn is_before(&self, a: &str, b: &str) -> bool {
        self.before.contains(&(a.to_owned(), b.to_owned()))
    }
    /// Every down-closed (monotone) reached-set over `shape`: a coordinate is in the set only if every
    /// coordinate before it is too.
    pub fn reached_sets(&self, shape: &[String]) -> Vec<BTreeSet<String>> {
        let n = shape.len();
        let mut out = Vec::new();
        for mask in 0..(1u32 << n) {
            let set: BTreeSet<String> = (0..n)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| shape[i].clone())
                .collect();
            let closed = set.iter().all(|c| {
                shape
                    .iter()
                    .all(|d| !self.is_before(d, c) || set.contains(d))
            });
            if closed {
                out.push(set);
            }
        }
        out
    }
    /// A guard is satisfied by a reached-set: `before c` ⇔ c not reached; `reached c` ⇔ c reached;
    /// `has c` ⇔ c in the board's clock-shape; an unresolved binding is never satisfied.
    pub fn guard_satisfied(
        x: &Expanded,
        guard: &[(GuardKind, Result<String, String>)],
        reached: &BTreeSet<String>,
    ) -> bool {
        guard.iter().all(|(k, c)| match (k, c) {
            (GuardKind::Before, Ok(c)) => !reached.contains(c),
            (GuardKind::Reached, Ok(c)) => reached.contains(c),
            (GuardKind::Has, Ok(c)) => x.clock_shape.contains(c),
            (_, Err(_)) => false,
        })
    }
}

/// The edges available from `node` under `reached` to a principal in `role`: guard satisfied and the
/// role listed. Preconditions (`:pre`) are a piece's own facts, not the board's, so they are not
/// evaluated here — a domain's own engine reads them from its own rows at commit.
pub fn available<'a>(
    x: &'a Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
) -> Vec<&'a XEdge> {
    filtered(
        x.edges.iter().filter(|e| e.from.as_deref() == Some(node)),
        x,
        reached,
        role,
    )
}

/// The entry edges (`:from ∅`) available under `reached` to `role` — a placement.
pub fn available_entry<'a>(
    x: &'a Expanded,
    reached: &BTreeSet<String>,
    role: &str,
) -> Vec<&'a XEdge> {
    filtered(
        x.edges.iter().filter(|e| e.from.is_none()),
        x,
        reached,
        role,
    )
}

fn filtered<'a>(
    edges: impl Iterator<Item = &'a XEdge>,
    x: &'a Expanded,
    reached: &BTreeSet<String>,
    role: &str,
) -> Vec<&'a XEdge> {
    edges
        .filter(|e| ClockOrder::guard_satisfied(x, &e.guard, reached))
        .filter(|e| e.edge.roles.iter().any(|r| r == role))
        .collect()
}

/// One edge available to a role, as the walk states it: this crate's own vocabulary — not a domain's
/// own move type (each domain maps this into its own), and not an authoring-session type either (no
/// `graph_revision`/`anchor`/`preview`/`focus`/`compiler_profile` — this is the walk's answer, not a
/// design-tool's).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Step {
    pub edge: String,
    pub word: String,
    pub to: String,
    pub roles: Vec<String>,
}

impl From<&XEdge> for Step {
    fn from(e: &XEdge) -> Self {
        Step {
            edge: e.id.clone(),
            word: e.edge.word.clone(),
            to: e.to.clone(),
            roles: e.edge.roles.clone(),
        }
    }
}

/// The legal steps from `node` under `reached` to `role` (the membership set a commit re-walks).
pub fn legal_moves(x: &Expanded, node: &str, reached: &BTreeSet<String>, role: &str) -> Vec<Step> {
    available(x, node, reached, role)
        .into_iter()
        .map(Step::from)
        .collect()
}

/// The legal placements (entry edges) under `reached` to `role`.
pub fn legal_entry_moves(x: &Expanded, reached: &BTreeSet<String>, role: &str) -> Vec<Step> {
    available_entry(x, reached, role)
        .into_iter()
        .map(Step::from)
        .collect()
}

/// Why an edge out of a piece's node is on the board but not enabled for the caller now — what the
/// piece is blocked on. The walk's answer, domain-free: a coordinate is a name, a party is a role, a
/// precondition is the dialect's own [`Pred`], evaluated by the domain's callback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockedOn {
    /// `reached c`, and `c` is not reached yet — clock-shaped: the edge waits on the clock.
    Awaits(String),
    /// `before c`, and `c` is reached — too late; no later reached-set enables it again.
    Passed(String),
    /// `has c`, and `c` is not in the board's clock-shape.
    NotInShape(String),
    /// the guard names a binding the board does not resolve.
    Unbound(String),
    /// the edge is another party's move: its roles, none of them the caller's — party-shaped.
    Party(Vec<String>),
    /// a precondition the domain's evaluator says does not hold.
    Pre(Pred),
}

/// One edge out of the node that is not enabled for the caller, with every reason, in the order the
/// guard, the roles and the preconditions are declared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blocked {
    pub step: Step,
    pub on: Vec<BlockedOn>,
}

/// Every reason `e` is not enabled for `role` under `reached`; empty when it is enabled. `holds`
/// answers a precondition of the edge (the domain reads its own facts; this crate never does).
fn reasons(
    x: &Expanded,
    e: &XEdge,
    reached: &BTreeSet<String>,
    role: &str,
    holds: &dyn Fn(&XEdge, &Pred) -> bool,
) -> Vec<BlockedOn> {
    let mut on: Vec<BlockedOn> = e
        .guard
        .iter()
        .filter(|(k, c)| !ClockOrder::guard_satisfied(x, &[(*k, c.clone())], reached))
        .map(|(k, c)| match (k, c) {
            (_, Err(b)) => BlockedOn::Unbound(b.clone()),
            (GuardKind::Reached, Ok(c)) => BlockedOn::Awaits(c.clone()),
            (GuardKind::Before, Ok(c)) => BlockedOn::Passed(c.clone()),
            (GuardKind::Has, Ok(c)) => BlockedOn::NotInShape(c.clone()),
        })
        .collect();
    if !e.edge.roles.iter().any(|r| r == role) {
        on.push(BlockedOn::Party(e.edge.roles.clone()));
    }
    on.extend(
        e.edge
            .pre
            .iter()
            .filter(|p| !holds(e, p))
            .cloned()
            .map(BlockedOn::Pre),
    );
    on
}

/// The steps out of `node` enabled for `role` under `reached`: guard satisfied, role listed, and every
/// precondition holding by `holds`. With a `holds` that always answers `true` this is [`legal_moves`].
pub fn enabled_moves(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    holds: &dyn Fn(&XEdge, &Pred) -> bool,
) -> Vec<Step> {
    x.out_edges(node)
        .filter(|e| reasons(x, e, reached, role, holds).is_empty())
        .map(Step::from)
        .collect()
}

/// `blocked_on(predicate)`: every edge out of `node` that is on the board but not enabled for `role`
/// under `reached`, each with what it is blocked on — the clock (`Awaits`), another party (`Party`), a
/// precondition (`Pre`), or a guard that can no longer hold. Every edge out of `node` is in exactly one
/// of [`enabled_moves`] and this list (the conformance suite's partition property). Ordered as the
/// board declares its edges.
pub fn blocked_on(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    holds: &dyn Fn(&XEdge, &Pred) -> bool,
) -> Vec<Blocked> {
    x.out_edges(node)
        .filter_map(|e| {
            let on = reasons(x, e, reached, role, holds);
            (!on.is_empty()).then(|| Blocked {
                step: Step::from(e),
                on,
            })
        })
        .collect()
}

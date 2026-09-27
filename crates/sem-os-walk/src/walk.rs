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
    /// coordinate before it is too. Enumerates subsets, so `shape` has at most 31 coordinates.
    ///
    /// # Panics
    /// A `shape` of more than 31 coordinates.
    pub fn reached_sets(&self, shape: &[String]) -> Vec<BTreeSet<String>> {
        let n = shape.len();
        assert!(
            n < 32,
            "reached_sets enumerates subsets: {n} coordinates is too many"
        );
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
#[non_exhaustive]
pub struct Step {
    pub edge: String,
    pub word: String,
    pub to: String,
    pub roles: Vec<String>,
    /// the coordinate the move must be played before (`:by`), as [`XEdge::by`] resolved it, surfaced
    /// with every answer the walk gives about the edge — legal, enabled or blocked. `None`: no flag;
    /// `Some(Err(b))`: a flag whose binding `b` the board does not resolve (never guessed, never
    /// dropped). The walk states the flag and nothing more: a move whose `:by` has been reached is
    /// still offered as it would be without one — what a deadline passed means is the domain's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Result<String, String>>,
}

impl From<&XEdge> for Step {
    fn from(e: &XEdge) -> Self {
        Step {
            edge: e.id.clone(),
            word: e.edge.word.clone(),
            to: e.to.clone(),
            roles: e.edge.roles.clone(),
            by: e.by.clone(),
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

/// A domain's answer to one precondition (`:pre`) of an edge, from its own facts: it holds, it fails,
/// or the domain cannot say yet (a fact it has not read, a self-referential `reached`). This crate never
/// reads a fact itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreAnswer {
    Holds,
    Fails,
    Unresolved,
}

/// The domain's precondition evaluator: an edge, one of its preconditions, and the reached-set the walk
/// is reading under. Must be pure — the walk may ask the same question more than once.
pub type PreEvaluator<'a> = dyn Fn(&XEdge, &Pred, &BTreeSet<String>) -> PreAnswer + 'a;

/// Why an edge out of a piece's node is on the board but not enabled for the caller now — what the
/// piece is blocked on. The walk's answer, domain-free: a coordinate is a name, a party is a role, a
/// precondition is the dialect's own [`Pred`], answered by the domain's [`PreEvaluator`].
///
/// A consumer deciding who is on move reads [`Blocked::is_dead`] first (an edge with any permanent
/// reason never enables, whatever else it waits on), then [`BlockedOn::clock_coord`] (a coordinate the
/// clock will reach) and [`BlockedOn::Party`] (another party's move). When an edge waits on both a
/// party and the clock, the move is the party's; the coordinate says when it can next be made.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BlockedOn {
    /// `reached c`, `c` in the board's clock-shape and not reached yet — clock-shaped: the edge waits
    /// on the clock.
    Awaits(String),
    /// `before c`, and `c` is reached — too late; the reached-set only grows, so it never clears.
    Passed(String),
    /// `has c` or `reached c` with `c` not in the board's clock-shape — no reached-set the board admits
    /// ever satisfies it.
    NotInShape(String),
    /// the guard names a binding the board does not resolve.
    Unbound(String),
    /// the edge is another party's move: its roles, none of them the caller's — party-shaped. Empty:
    /// no one may make the move (permanent).
    Party(Vec<String>),
    /// a precondition the domain's evaluator says fails.
    Pre(Pred),
    /// a precondition the domain's evaluator cannot answer yet.
    PreUnresolved(Pred),
}

impl BlockedOn {
    /// A reason no later reached-set or fact clears: a coordinate passed or outside the shape, an
    /// unresolved binding, a move no role may make.
    #[must_use]
    pub fn is_permanent(&self) -> bool {
        match self {
            BlockedOn::Passed(_) | BlockedOn::NotInShape(_) | BlockedOn::Unbound(_) => true,
            BlockedOn::Party(roles) => roles.is_empty(),
            BlockedOn::Awaits(_) | BlockedOn::Pre(_) | BlockedOn::PreUnresolved(_) => false,
        }
    }

    /// The coordinate this reason waits on the clock for, if it is clock-shaped: a guard's `reached c`,
    /// or a precondition `reached c` that fails or is unanswered.
    #[must_use]
    pub fn clock_coord(&self) -> Option<&str> {
        match self {
            BlockedOn::Awaits(c)
            | BlockedOn::Pre(Pred::Reached(c))
            | BlockedOn::PreUnresolved(Pred::Reached(c)) => Some(c),
            BlockedOn::Passed(_)
            | BlockedOn::NotInShape(_)
            | BlockedOn::Unbound(_)
            | BlockedOn::Party(_)
            | BlockedOn::Pre(_)
            | BlockedOn::PreUnresolved(_) => None,
        }
    }
}

/// One edge out of the node that is not enabled for the caller, with every reason once, in the order
/// the guard, the roles and the preconditions are declared.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Blocked {
    pub step: Step,
    pub on: Vec<BlockedOn>,
}

impl Blocked {
    /// Whether any reason is permanent: the edge never enables, whatever else it waits on.
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.on.iter().any(BlockedOn::is_permanent)
    }
}

/// Every reason `e` is not enabled for `role` under `reached`, each once; empty when it is enabled.
fn reasons(
    x: &Expanded,
    e: &XEdge,
    reached: &BTreeSet<String>,
    role: &str,
    pre: &PreEvaluator<'_>,
) -> Vec<BlockedOn> {
    let mut on: Vec<BlockedOn> = Vec::new();
    let mut push = |r: BlockedOn| {
        if !on.contains(&r) {
            on.push(r);
        }
    };
    for (k, c) in &e.guard {
        if ClockOrder::guard_satisfied(x, &[(*k, c.clone())], reached) {
            continue;
        }
        push(match (k, c) {
            (_, Err(b)) => BlockedOn::Unbound(b.clone()),
            (GuardKind::Reached, Ok(c)) if x.clock_shape.contains(c) => {
                BlockedOn::Awaits(c.clone())
            }
            (GuardKind::Reached | GuardKind::Has, Ok(c)) => BlockedOn::NotInShape(c.clone()),
            (GuardKind::Before, Ok(c)) => BlockedOn::Passed(c.clone()),
        });
    }
    if !e.edge.roles.iter().any(|r| r == role) {
        push(BlockedOn::Party(e.edge.roles.clone()));
    }
    for p in &e.edge.pre {
        match pre(e, p, reached) {
            PreAnswer::Holds => {}
            PreAnswer::Fails => push(BlockedOn::Pre(p.clone())),
            PreAnswer::Unresolved => push(BlockedOn::PreUnresolved(p.clone())),
        }
    }
    on
}

/// Each edge out of `node`, in board order, with its reasons — the one classification both
/// [`enabled_moves`] and [`blocked_on`] are projections of.
pub(crate) fn classify<'a>(
    x: &'a Expanded,
    node: &'a str,
    reached: &BTreeSet<String>,
    role: &str,
    pre: &PreEvaluator<'_>,
) -> Vec<(&'a XEdge, Vec<BlockedOn>)> {
    x.out_edges(node)
        .map(|e| (e, reasons(x, e, reached, role, pre)))
        .collect()
}

/// The steps out of `node` enabled for `role` under `reached`: guard satisfied, role listed, and every
/// precondition holding by `pre`. With an evaluator that always answers [`PreAnswer::Holds`] this is
/// [`legal_moves`].
pub fn enabled_moves(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    pre: &PreEvaluator<'_>,
) -> Vec<Step> {
    classify(x, node, reached, role, pre)
        .into_iter()
        .filter(|(_, on)| on.is_empty())
        .map(|(e, _)| Step::from(e))
        .collect()
}

/// `blocked_on(predicate)`: every edge out of `node` that is on the board but not enabled for `role`
/// under `reached`, each with what it is blocked on — the clock ([`BlockedOn::Awaits`]), another party
/// ([`BlockedOn::Party`]), a precondition ([`BlockedOn::Pre`], [`BlockedOn::PreUnresolved`]), or a
/// permanent reason ([`BlockedOn::is_permanent`]). Every edge out of `node` is in exactly one of
/// [`enabled_moves`] and this list. Ordered as the board declares its edges.
pub fn blocked_on(
    x: &Expanded,
    node: &str,
    reached: &BTreeSet<String>,
    role: &str,
    pre: &PreEvaluator<'_>,
) -> Vec<Blocked> {
    classify(x, node, reached, role, pre)
        .into_iter()
        .filter(|(_, on)| !on.is_empty())
        .map(|(e, on)| Blocked {
            step: Step::from(e),
            on,
        })
        .collect()
}

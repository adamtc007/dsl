//! The walk's core reads: the monotone reached-sets a clock-shape admits, and the edges available
//! from a node under a reached-set for a role. Pure functions over the expanded board — no store, no
//! clock, no domain type: legality here reads only the reached-set and the guard/role surface.

use std::collections::BTreeSet;

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

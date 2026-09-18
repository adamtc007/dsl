//! The expanded board: a board plus its regions and inherited bindings, flattened into one explicit
//! graph with every `any`, `same`, `prior` and incident form made concrete. Building one (reading a
//! domain's own board/region/model types) is a domain concern; the type and the pure reads over it
//! are not.

use std::collections::BTreeMap;

use crate::dialect::{BindValue, Coord, Edge};

/// The reserved node name every board's incident overlay parks at — a dialect-level convention (the
/// callout pattern's own shared grammar), not any one domain's vocabulary.
pub const INCIDENT: &str = "Incident";

/// One explicit edge of an expanded board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XEdge {
    pub id: String,
    /// `None` = the entry edge (from ∅)
    pub from: Option<String>,
    pub to: String,
    pub edge: Edge,
    /// guards with bound coordinates resolved through the effective bindings; `Err(abstract)` when unresolved
    pub guard: Vec<(GuardKind, Result<String, String>)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardKind {
    Before,
    Reached,
    Has,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expanded {
    pub id: String,
    pub kind: String,
    /// the event board whose clock-shape and bindings apply (self for event boards)
    pub parent: String,
    pub nodes: Vec<String>,
    pub terminal: Vec<String>,
    /// the subset of `terminal` a board declares a *won* outcome — a terminal node absent here is a
    /// build failure at the domain's own check level, never a silent third state.
    pub outcome: Vec<String>,
    pub clock_shape: Vec<String>,
    pub bind: BTreeMap<String, BindValue>,
    pub edges: Vec<XEdge>,
}

impl Expanded {
    pub fn is_terminal(&self, n: &str) -> bool {
        self.terminal.iter().any(|t| t == n)
    }
    pub fn is_outcome(&self, n: &str) -> bool {
        self.outcome.iter().any(|t| t == n)
    }
    pub fn out_edges<'a>(&'a self, n: &'a str) -> impl Iterator<Item = &'a XEdge> + 'a {
        self.edges
            .iter()
            .filter(move |e| e.from.as_deref() == Some(n))
    }
    pub fn entry_edges(&self) -> impl Iterator<Item = &XEdge> {
        self.edges.iter().filter(|e| e.from.is_none())
    }
    pub fn label(&self) -> String {
        if self.parent == self.id {
            self.id.clone()
        } else {
            format!("{} under {}", self.id, self.parent)
        }
    }
}

/// A source coordinate reference (a literal name, or a board's own abstract binding) resolved to its
/// real coordinate name — `Err` names the unresolved binding.
pub fn resolve_coord(bind: &BTreeMap<String, BindValue>, c: &Coord) -> Result<String, String> {
    match c {
        Coord::Named(n) => Ok(n.clone()),
        Coord::Bound(b) => {
            if let Some(BindValue::Symbol(s)) = bind.get(b) {
                Ok(s.clone())
            } else {
                Err(b.clone())
            }
        }
    }
}

//! The board-source dialect's own AST: bindings, edges and their guards/preconditions/side-effect
//! expressions. Domain-free — every field is a plain string, a taxonomy-shaped enum, or one of these
//! types recursively; nothing here names a field of any one domain's own piece (a callout-pattern
//! DSL's shared grammar, not one domain's vocabulary).

use std::collections::BTreeMap;

/// A binding target on a board: a literal symbol, a nested map (by-base/by-caev/by-option style
/// bindings), or a side-effect expression bound abstractly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindValue {
    Nil,
    /// a word, a coordinate, or another symbol — resolved by the slot that binds it
    Symbol(String),
    Map(BTreeMap<String, BindValue>),
    Se(Se),
}

/// One edge of a board or region source: a legal move's declared shape before expansion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: String,
    pub word: String,
    pub coord: Option<Coord>,
    pub from: From,
    pub to: To,
    pub roles: Vec<String>,
    pub guard: Vec<Guard>,
    pub pre: Vec<Pred>,
    pub se: Option<Se>,
    pub tx: Vec<String>,
    /// the region this edge came from, if any
    pub region: Option<String>,
}

/// An edge's declared source: no node (`Entry`), any node, the incident return, or a named set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum From {
    Entry,
    Any,
    /// `(incident :prior prior)` — the return edge, from Incident back to the halted node
    Incident,
    Nodes(Vec<String>),
}

/// An edge's declared target: a named node, the source node unchanged, the halted node, or Incident.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum To {
    Node(String),
    Same,
    Prior,
    Incident,
}

/// A clock coordinate reference: a literal name, or a board's own abstract binding resolved through
/// its `:bind` map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Coord {
    Named(String),
    Bound(String),
}

/// A clock-shape guard over a reached-set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Guard {
    Before(Coord),
    Reached(Coord),
    Has(Coord),
}

/// A precondition's own comparison operator (`Pred::Fact`'s optional `:is`/`:above`/`:below`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmp {
    Is,
    Above,
    Below,
}

/// A precondition (`:pre`) — the closed surface a commit-time or walk-time evaluator answers over a
/// piece's own facts, folds and counts. What each variant reads is an engine concern (a `PreLookup`
/// the evaluator supplies); this type is only the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pred {
    Count {
        kind: String,
        not_in: Vec<String>,
        is: i64,
    },
    Fact {
        name: String,
        cmp: Option<(Cmp, String)>,
    },
    Fold(String),
    ParentNode(Vec<String>),
    Not(Box<Pred>),
    Declared(String),
    Reached(String),
}

impl Pred {
    pub fn count_kinds(&self, out: &mut Vec<String>) {
        match self {
            Pred::Count { kind, .. } => out.push(kind.clone()),
            Pred::Not(p) => p.count_kinds(out),
            Pred::Fact { .. }
            | Pred::Fold(_)
            | Pred::ParentNode(_)
            | Pred::Declared(_)
            | Pred::Reached(_) => {}
        }
    }
    pub fn folds(&self, out: &mut Vec<String>) {
        match self {
            Pred::Fold(f) => out.push(f.clone()),
            Pred::Not(p) => p.folds(out),
            Pred::Count { .. }
            | Pred::Fact { .. }
            | Pred::ParentNode(_)
            | Pred::Declared(_)
            | Pred::Reached(_) => {}
        }
    }
}

/// A side-effect expression (`:se`): the ordered, possibly-nested cascade an edge commits alongside
/// its own transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Se {
    Word(String),
    Seq(Vec<Se>),
    Par(Vec<Se>),
    Each {
        kind: String,
        word: Box<Se>,
    },
    /// `(each :fold f :word w)` — iterate the rows of a fold (not a piece kind)
    EachFold {
        fold: String,
        word: Box<Se>,
    },
    Bind(String),
}

impl Se {
    /// Every concrete word named anywhere in this side-effect expression.
    pub fn words(&self, out: &mut Vec<String>) {
        match self {
            Se::Word(w) => out.push(w.clone()),
            Se::Seq(items) | Se::Par(items) => items.iter().for_each(|i| i.words(out)),
            Se::Each { word, .. } | Se::EachFold { word, .. } => word.words(out),
            Se::Bind(_) => {}
        }
    }
    /// Every abstract binding named anywhere in this expression.
    pub fn binds(&self, out: &mut Vec<String>) {
        match self {
            Se::Bind(b) => out.push(b.clone()),
            Se::Seq(items) | Se::Par(items) => items.iter().for_each(|i| i.binds(out)),
            Se::Each { word, .. } | Se::EachFold { word, .. } => word.binds(out),
            Se::Word(_) => {}
        }
    }
    /// Every `each` kind named anywhere in this expression.
    pub fn each_kinds(&self, out: &mut Vec<String>) {
        match self {
            Se::Each { kind, word } => {
                out.push(kind.clone());
                word.each_kinds(out);
            }
            Se::EachFold { word, .. } => word.each_kinds(out),
            Se::Seq(items) | Se::Par(items) => items.iter().for_each(|i| i.each_kinds(out)),
            Se::Word(_) | Se::Bind(_) => {}
        }
    }
    /// Every fold iterated by an `(each :fold ..)` anywhere in this expression.
    pub fn each_folds(&self, out: &mut Vec<String>) {
        match self {
            Se::EachFold { fold, word } => {
                out.push(fold.clone());
                word.each_folds(out);
            }
            Se::Each { word, .. } => word.each_folds(out),
            Se::Seq(items) | Se::Par(items) => items.iter().for_each(|i| i.each_folds(out)),
            Se::Word(_) | Se::Bind(_) => {}
        }
    }
}

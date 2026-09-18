//! Board-walk contract for a callout-pattern DSL (a board is a graph of nodes and edges; a piece
//! sits at a node; a move is an edge whose guard the piece's own reached-set satisfies and whose
//! role the caller holds). Domain-free: nothing here names a field of any one domain's own piece —
//! [`dialect`] is the shared AST (bindings, edges, guards, preconditions, side-effect expressions);
//! [`expand`] is one board flattened into an explicit graph; [`walk`] is the pure guard+role read
//! over it. A domain builds its own [`expand::Expanded`] from its own board/region model (that
//! construction is a domain concern, reading domain types this crate never sees) and hands it here
//! for the walk; the domain's own richer move/step type is its own — [`conformance`] is what a second
//! walk engine over the same contract, if one is ever built, must agree with the first on.
#![deny(unreachable_pub)]

pub mod conformance;
pub mod dialect;
pub mod expand;
pub mod walk;

pub use dialect::{BindValue, Cmp, Coord, Edge, From, Guard, Pred, Se, To};
pub use expand::{resolve_coord, Expanded, GuardKind, XEdge, INCIDENT};
pub use walk::{available, available_entry, legal_entry_moves, legal_moves, ClockOrder, Step};

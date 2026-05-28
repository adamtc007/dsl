//! Machine-readable `green_when` predicate support.

pub mod ast;
pub mod parser;

pub use ast::{
    AttrValue, CmpOp, EntityQualifier, EntityRef, EntitySetRef,
    Predicate, RelationScope, State, Validity,
};
pub(crate) use ast::{AttrName, CountOp, EntityKind, StateSet};
pub use parser::{parse_green_when, ParseError};

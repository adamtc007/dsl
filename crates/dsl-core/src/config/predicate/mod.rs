//! Machine-readable `green_when` predicate support.

pub mod ast;
pub mod parser;

pub(crate) use ast::{EntityQualifier, RelationScope};
pub use ast::{
    AttrValue, CmpOp, EntityRef, EntitySetRef,
    Predicate, State, Validity,
};
pub use parser::{parse_green_when, ParseError};

#[cfg(test)]
mod integration_tests;

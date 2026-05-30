//! Machine-readable `green_when` predicate support.

pub(crate) mod ast;
pub(crate) mod parser;

pub(crate) use ast::{EntityQualifier, RelationScope};
pub use ast::{
    AttrValue, CmpOp, EntityRef, EntitySetRef,
    Predicate, Validity,
};
pub(crate) use ast::State;
pub use parser::parse_green_when;
pub(crate) use parser::ParseError;

#[cfg(test)]
mod integration_tests;

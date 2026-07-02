//! Machine-readable `green_when` predicate support.

pub(crate) mod ast;
pub(crate) mod parser;

pub use ast::{AttrValue, CmpOp, EntityRef, EntitySetRef, Predicate, Validity};
pub use parser::parse_green_when;

#[cfg(test)]
mod integration_tests;

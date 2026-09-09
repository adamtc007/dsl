//! dsl-parser: S-expression lexer and parser for the unified DSL atom grammar.
//!
//! Produces a raw untyped parse tree (`SourceFile`) from DSL source text.
//! Kind classification and typed AST construction happen in `dsl-ast`
//! against a `dsl-atoms` catalogue; this crate knows no kind names.
//!
//! # Slot rule
//!
//! Slots are keyword-form only: `(kind name? :slot value ...)`. The single
//! exception is the `flow` arrow sugar `(flow src -> tgt ...)`, whose two
//! positional values are synthesised into `:source` and `:target`. A
//! positional value in any other atom is a parse error. Positional forms are
//! deliberately not part of this grammar; a frontend that wants
//! `(seq a b)`-style forms spells them as keyword slots, for example
//! `(se :steps [(seq :of [a b])])`.
//!
//! Every keyword slot is preserved verbatim in the raw tree, whether or not
//! any frontend consumes it. Dropping a slot is a frontend's decision, never
//! the parser's.
//!
//! # Entry point
//!
//! ```rust
//! let (source_file, diagnostics) = dsl_parser::parse("(node start :label \"Start\")");
//! assert!(!diagnostics.has_errors());
//! assert_eq!(source_file.atoms[0].kind, "node");
//! ```
#![deny(unreachable_pub)]

pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod raw_ast;

pub use lexer::{lex, Token};
pub use parser::parse;
pub use raw_ast::{RawAtom, RawValue, SourceFile};

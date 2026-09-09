//! dsl-ast: catalogue-classified atoms for the unified DSL.
//!
//! Provides [`AtomBag`], which classifies raw atoms from `dsl-parser` into
//! [`TypedAtom`] values against a caller-supplied `dsl-atoms`
//! [`KindCatalogue`](dsl_atoms::KindCatalogue), and [`AtomParser`], which
//! bundles a catalogue with the parser for external consumers.
//!
//! Per-kind slot extraction and type checking are the business of the
//! frontend that owns the kind; the bag preserves every slot verbatim.
#![deny(unreachable_pub)]

pub(crate) mod atom_bag;

pub use atom_bag::{AtomBag, AtomIndex, AtomParser, TypedAtom};

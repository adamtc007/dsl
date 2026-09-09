//! dsl-atoms: data-driven atom kind catalogue for the unified DSL.
//!
//! An atom's kind (the first symbol inside its parentheses) is classified
//! against a [`KindCatalogue`] supplied by the caller. The catalogue is data:
//! it is built with [`KindCatalogue::register`], parsed from DSL source with
//! [`KindCatalogue::from_source`], or seeded from the built-in set with
//! [`KindCatalogue::builtin`]. Nothing in this crate matches on a closed list
//! of kind names, and classification never falls back to a default: an
//! unregistered kind is a typed [`UnknownKind`] error naming the catalogue.
//!
//! The built-in kind names are exposed as constants in [`builtin`] so that
//! frontends can refer to them without a closed enum.
#![deny(unreachable_pub)]

mod catalogue;
mod param_type;

pub use catalogue::{
    builtin, AtomKindClass, CatalogueError, KindCatalogue, KindName, KindRole, UnknownKind,
    CATALOGUE_HEADER_KIND, KIND_ENTRY_KIND, MAX_KIND_NAME_LEN,
};
pub use param_type::ParamType;

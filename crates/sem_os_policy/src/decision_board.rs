//! Compatibility path for semantic decision contracts.
//!
//! New consumers should import [`semantic_decision_contracts`] directly.
//! This module remains for one deprecation window and re-exports the exact
//! same Rust types; it contains no wrappers or duplicate implementations.

pub use semantic_decision_contracts::*;

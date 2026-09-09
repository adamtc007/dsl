//! Data-driven atom kind catalogue.
//!
//! A [`KindCatalogue`] maps kind names to a [`KindRole`]. Catalogues are
//! values: they are constructed by code, parsed from DSL source, or seeded
//! from the built-in set. Classification ([`KindCatalogue::classify`]) is a
//! map lookup; there is no `match` over kind names anywhere in this crate.
//!
//! # Catalogue source grammar
//!
//! A catalogue can be written in the DSL's own atom syntax and parsed with
//! [`KindCatalogue::from_source`]:
//!
//! ```text
//! (kind-catalogue board-kinds :extends builtin)   ; header, optional
//! (kind board :role structural)
//! (kind guard :role structural)
//! (kind attestation :role declarative)
//! ```
//!
//! The header names the catalogue and may seed it from the built-in set with
//! `:extends builtin`. Every other top-level atom must be a `kind` entry.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use dsl_parser::{RawAtom, RawValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Upper bound on a kind name's length in bytes.
pub const MAX_KIND_NAME_LEN: usize = 64;

/// Atom kind of a catalogue-source header: `(kind-catalogue NAME ...)`.
pub const CATALOGUE_HEADER_KIND: &str = "kind-catalogue";

/// Atom kind of a catalogue-source entry: `(kind NAME :role ROLE)`.
pub const KIND_ENTRY_KIND: &str = "kind";

const EXTENDS_SLOT: &str = "extends";
const ROLE_SLOT: &str = "role";
const BUILTIN_BASE: &str = "builtin";
const ANONYMOUS_CATALOGUE_NAME: &str = "anonymous";

// ---------------------------------------------------------------------------
// KindName
// ---------------------------------------------------------------------------

/// Validated atom kind name.
///
/// A kind name is symbol-shaped: it starts with an ASCII letter or `_` and
/// continues with ASCII letters, digits, `_` or `-`. It is bounded by
/// [`MAX_KIND_NAME_LEN`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct KindName(String);

impl KindName {
    /// Construct a validated kind name.
    pub fn new(value: impl Into<String>) -> Result<Self, CatalogueError> {
        let value = value.into();
        validate_kind_name(&value)?;
        Ok(Self(value))
    }

    /// Borrow the kind name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_kind_name(value: &str) -> Result<(), CatalogueError> {
    let invalid = |reason| CatalogueError::InvalidKindName {
        value: value.to_owned(),
        reason,
    };
    if value.is_empty() {
        return Err(invalid("must not be empty"));
    }
    if value.len() > MAX_KIND_NAME_LEN {
        return Err(invalid("exceeds the maximum length"));
    }
    let mut bytes = value.bytes();
    let first = bytes.next().expect("non-empty");
    if !(first.is_ascii_alphabetic() || first == b'_') {
        return Err(invalid("must start with an ASCII letter or '_'"));
    }
    if !bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
        return Err(invalid(
            "must contain only ASCII letters, digits, '_' or '-'",
        ));
    }
    Ok(())
}

fn validate_catalogue_name(value: &str) -> Result<(), CatalogueError> {
    let invalid = |reason| CatalogueError::InvalidCatalogueName {
        value: value.to_owned(),
        reason,
    };
    if value.is_empty() {
        return Err(invalid("must not be empty"));
    }
    if value.len() > MAX_KIND_NAME_LEN {
        return Err(invalid("exceeds the maximum length"));
    }
    let mut bytes = value.bytes();
    let first = bytes.next().expect("non-empty");
    if !(first.is_ascii_alphabetic() || first == b'_') {
        return Err(invalid("must start with an ASCII letter or '_'"));
    }
    if !bytes.all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.')) {
        return Err(invalid(
            "must contain only ASCII letters, digits, '_', '-' or '.'",
        ));
    }
    Ok(())
}

impl std::borrow::Borrow<str> for KindName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for KindName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for KindName {
    type Err = CatalogueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for KindName {
    type Error = CatalogueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<KindName> for String {
    fn from(value: KindName) -> Self {
        value.0
    }
}

impl PartialEq<str> for KindName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for KindName {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

// ---------------------------------------------------------------------------
// KindRole
// ---------------------------------------------------------------------------

/// The role a kind plays in a source file.
///
/// Structural atoms describe the artefact (nodes, edges, definitions).
/// Declarative atoms carry governance and provenance metadata about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KindRole {
    Structural,
    Declarative,
}

impl KindRole {
    /// Canonical source spelling of the role.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::Declarative => "declarative",
        }
    }

    /// Parse the canonical source spelling of a role.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "structural" => Some(Self::Structural),
            "declarative" => Some(Self::Declarative),
            _ => None,
        }
    }
}

impl fmt::Display for KindRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// AtomKindClass
// ---------------------------------------------------------------------------

/// A classified atom kind: the registered name paired with its role.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AtomKindClass {
    Structural(KindName),
    Declarative(KindName),
}

impl AtomKindClass {
    /// The registered kind name.
    #[must_use]
    pub fn name(&self) -> &KindName {
        match self {
            Self::Structural(name) | Self::Declarative(name) => name,
        }
    }

    /// The role the catalogue assigned to this kind.
    #[must_use]
    pub fn role(&self) -> KindRole {
        match self {
            Self::Structural(_) => KindRole::Structural,
            Self::Declarative(_) => KindRole::Declarative,
        }
    }

    /// Whether this is the given role and kind name.
    #[must_use]
    pub fn is(&self, role: KindRole, name: &str) -> bool {
        self.role() == role && self.name().as_str() == name
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// An atom kind that is not registered in the catalogue used to classify it.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("unknown atom kind `{kind}`: not registered in kind catalogue `{catalogue}`")]
pub struct UnknownKind {
    /// The kind string as written in source.
    pub kind: String,
    /// Name of the catalogue that was consulted.
    pub catalogue: String,
}

/// Failure to build or parse a catalogue.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CatalogueError {
    #[error("invalid kind name `{value}`: {reason}")]
    InvalidKindName { value: String, reason: &'static str },
    #[error("invalid catalogue name `{value}`: {reason}")]
    InvalidCatalogueName { value: String, reason: &'static str },
    #[error("kind `{kind}` is already registered in catalogue `{catalogue}` as {existing}")]
    Duplicate {
        kind: String,
        catalogue: String,
        existing: KindRole,
    },
    #[error("catalogue source: {message}")]
    Source { message: String },
    #[error(
        "catalogue source: atom kind `{kind}` is not part of the catalogue grammar \
         (expected `{CATALOGUE_HEADER_KIND}` or `{KIND_ENTRY_KIND}`)"
    )]
    UnexpectedAtom { kind: String },
    #[error("catalogue source: unknown base catalogue `{name}` in `:{EXTENDS_SLOT}` (only `{BUILTIN_BASE}` is available)")]
    UnknownBase { name: String },
}

// ---------------------------------------------------------------------------
// KindCatalogue
// ---------------------------------------------------------------------------

/// A named, data-driven map from kind name to role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindCatalogue {
    name: String,
    kinds: BTreeMap<KindName, KindRole>,
}

impl KindCatalogue {
    /// An empty catalogue with the given name.
    ///
    /// The name must be symbol-shaped (an ASCII letter or `_`, then ASCII
    /// letters, digits, `_`, `-` or `.`) so that [`KindCatalogue::to_source`]
    /// always renders a header that [`KindCatalogue::from_source`] accepts.
    pub fn new(name: impl Into<String>) -> Result<Self, CatalogueError> {
        let name = name.into();
        validate_catalogue_name(&name)?;
        Ok(Self {
            name,
            kinds: BTreeMap::new(),
        })
    }

    /// The built-in catalogue: the kinds the shared frontends know about.
    ///
    /// See [`builtin`] for the names. The catalogue is a starting point, not a
    /// closed set: callers extend it with [`KindCatalogue::register`] or seed
    /// from it with `:extends builtin` in catalogue source.
    #[must_use]
    pub fn builtin() -> Self {
        let mut catalogue =
            Self::new(builtin::CATALOGUE_NAME).expect("built-in catalogue name is valid");
        for name in builtin::STRUCTURAL {
            catalogue
                .register(name, KindRole::Structural)
                .expect("built-in structural kind names are valid and unique");
        }
        for name in builtin::DECLARATIVE {
            catalogue
                .register(name, KindRole::Declarative)
                .expect("built-in declarative kind names are valid and unique");
        }
        catalogue
    }

    /// Catalogue name, reported in [`UnknownKind`] errors.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Register a kind. Registering an existing name is an error, even with
    /// the same role.
    pub fn register(
        &mut self,
        name: impl AsRef<str>,
        role: KindRole,
    ) -> Result<&mut Self, CatalogueError> {
        let name = KindName::new(name.as_ref())?;
        if let Some(existing) = self.kinds.get(&name) {
            return Err(CatalogueError::Duplicate {
                kind: name.0,
                catalogue: self.name.clone(),
                existing: *existing,
            });
        }
        self.kinds.insert(name, role);
        Ok(self)
    }

    /// Builder form of [`KindCatalogue::register`].
    pub fn with(mut self, name: impl AsRef<str>, role: KindRole) -> Result<Self, CatalogueError> {
        self.register(name, role)?;
        Ok(self)
    }

    /// Register every kind of `other` into this catalogue.
    pub fn extend_from(&mut self, other: &KindCatalogue) -> Result<&mut Self, CatalogueError> {
        for (name, role) in &other.kinds {
            self.register(name.as_str(), *role)?;
        }
        Ok(self)
    }

    /// Role of a registered kind, if any.
    #[must_use]
    pub fn role_of(&self, kind: &str) -> Option<KindRole> {
        self.kinds.get(kind).copied()
    }

    /// Whether the kind is registered.
    #[must_use]
    pub fn contains(&self, kind: &str) -> bool {
        self.kinds.contains_key(kind)
    }

    /// Classify a kind string. Unregistered kinds are a typed error naming
    /// this catalogue; there is no default classification.
    pub fn classify(&self, kind: &str) -> Result<AtomKindClass, UnknownKind> {
        match self.kinds.get_key_value(kind) {
            Some((name, KindRole::Structural)) => Ok(AtomKindClass::Structural(name.clone())),
            Some((name, KindRole::Declarative)) => Ok(AtomKindClass::Declarative(name.clone())),
            None => Err(UnknownKind {
                kind: kind.to_owned(),
                catalogue: self.name.clone(),
            }),
        }
    }

    /// Registered kinds in name order.
    pub fn kinds(&self) -> impl Iterator<Item = (&KindName, KindRole)> {
        self.kinds.iter().map(|(name, role)| (name, *role))
    }

    /// Number of registered kinds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    /// Whether no kinds are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }

    /// Parse a catalogue from DSL source (see the module documentation for
    /// the grammar).
    pub fn from_source(src: &str) -> Result<Self, CatalogueError> {
        let (file, diagnostics) = dsl_parser::parse(src);
        if diagnostics.has_errors() {
            let messages: Vec<&str> = diagnostics.errors().map(|d| d.message.as_str()).collect();
            return Err(CatalogueError::Source {
                message: messages.join("; "),
            });
        }

        let mut atoms = file.atoms.iter();
        let mut catalogue = match atoms.clone().next() {
            Some(header) if header.kind == CATALOGUE_HEADER_KIND => {
                atoms.next();
                parse_header(header)?
            }
            _ => Self::new(ANONYMOUS_CATALOGUE_NAME)?,
        };

        for atom in atoms {
            if atom.kind != KIND_ENTRY_KIND {
                return Err(CatalogueError::UnexpectedAtom {
                    kind: atom.kind.clone(),
                });
            }
            let (name, role) = parse_entry(atom)?;
            catalogue.register(name, role)?;
        }
        Ok(catalogue)
    }

    /// Render the catalogue as DSL source that [`KindCatalogue::from_source`]
    /// parses back to an equal catalogue.
    #[must_use]
    pub fn to_source(&self) -> String {
        let mut out = format!("({CATALOGUE_HEADER_KIND} {})\n", self.name);
        for (name, role) in &self.kinds {
            out.push_str(&format!("({KIND_ENTRY_KIND} {name} :{ROLE_SLOT} {role})\n"));
        }
        out
    }
}

fn parse_header(header: &RawAtom) -> Result<KindCatalogue, CatalogueError> {
    let name = header.name.clone().ok_or_else(|| CatalogueError::Source {
        message: format!("`({CATALOGUE_HEADER_KIND} NAME ...)` requires a name"),
    })?;
    let mut catalogue = KindCatalogue::new(name)?;
    for (slot, value) in &header.slots {
        match (slot.as_str(), value) {
            (EXTENDS_SLOT, RawValue::Symbol(base)) if base == BUILTIN_BASE => {
                catalogue.extend_from(&KindCatalogue::builtin())?;
            }
            (EXTENDS_SLOT, RawValue::Symbol(base)) => {
                return Err(CatalogueError::UnknownBase { name: base.clone() });
            }
            (EXTENDS_SLOT, other) => {
                return Err(CatalogueError::Source {
                    message: format!("`:{EXTENDS_SLOT}` expects a symbol, got {other:?}"),
                });
            }
            (other, _) => {
                return Err(CatalogueError::Source {
                    message: format!("unknown `{CATALOGUE_HEADER_KIND}` slot `:{other}`"),
                });
            }
        }
    }
    Ok(catalogue)
}

fn parse_entry(atom: &RawAtom) -> Result<(String, KindRole), CatalogueError> {
    let name = atom.name.clone().ok_or_else(|| CatalogueError::Source {
        message: format!("`({KIND_ENTRY_KIND} NAME :{ROLE_SLOT} ROLE)` requires a name"),
    })?;
    let mut role = None;
    for (slot, value) in &atom.slots {
        match (slot.as_str(), value) {
            (ROLE_SLOT, RawValue::Symbol(spelling)) => {
                role = Some(KindRole::parse(spelling).ok_or_else(|| CatalogueError::Source {
                    message: format!(
                        "kind `{name}`: unknown role `{spelling}` (expected `structural` or `declarative`)"
                    ),
                })?);
            }
            (ROLE_SLOT, other) => {
                return Err(CatalogueError::Source {
                    message: format!(
                        "kind `{name}`: `:{ROLE_SLOT}` expects a symbol, got {other:?}"
                    ),
                });
            }
            (other, _) => {
                return Err(CatalogueError::Source {
                    message: format!("kind `{name}`: unknown slot `:{other}`"),
                });
            }
        }
    }
    let role = role.ok_or_else(|| CatalogueError::Source {
        message: format!("kind `{name}`: missing `:{ROLE_SLOT}`"),
    })?;
    Ok((name, role))
}

// ---------------------------------------------------------------------------
// Built-in kind names
// ---------------------------------------------------------------------------

/// Names of the kinds in [`KindCatalogue::builtin`].
///
/// These are constants, not an enum: a frontend that needs a kind not listed
/// here registers it in its own catalogue.
pub mod builtin {
    /// Name of the built-in catalogue.
    pub const CATALOGUE_NAME: &str = "dsl-atoms-builtin-v0.1";

    pub const VERB: &str = "verb";
    pub const INVOKE: &str = "invoke";
    pub const NODE: &str = "node";
    pub const GATEWAY: &str = "gateway";
    pub const FLOW: &str = "flow";
    pub const BOUNDARY_ATTACHMENT: &str = "boundary-attachment";
    pub const PARALLEL_JOIN: &str = "parallel-join";
    pub const ENTITY: &str = "entity";
    pub const RELATIONSHIP: &str = "relationship";
    pub const PREDICATE: &str = "predicate";
    pub const DECISION: &str = "decision";
    pub const DATA_TYPE: &str = "data-type";
    pub const MESSAGE_DEFINITION: &str = "message-definition";
    pub const TIMER_DEFINITION: &str = "timer-definition";
    pub const ERROR_DEFINITION: &str = "error-definition";
    pub const GRAPH_PACK: &str = "graph-pack";
    pub const UTTERANCE_BINDING: &str = "utterance-binding";
    pub const CONSTELLATION_ROOT: &str = "constellation-root";
    pub const WORKSPACE_CONSTRAINT: &str = "workspace-constraint";
    pub const DECISION_PACK: &str = "decision-pack";

    pub const PROVENANCE: &str = "provenance";
    pub const GOVERNANCE_STATUS: &str = "governance-status";
    pub const REVIEW_ANNOTATION: &str = "review-annotation";
    pub const JURISDICTION_TAG: &str = "jurisdiction-tag";

    /// Built-in structural kind names.
    pub const STRUCTURAL: &[&str] = &[
        VERB,
        INVOKE,
        NODE,
        GATEWAY,
        FLOW,
        BOUNDARY_ATTACHMENT,
        PARALLEL_JOIN,
        ENTITY,
        RELATIONSHIP,
        PREDICATE,
        DECISION,
        DATA_TYPE,
        MESSAGE_DEFINITION,
        TIMER_DEFINITION,
        ERROR_DEFINITION,
        GRAPH_PACK,
        UTTERANCE_BINDING,
        CONSTELLATION_ROOT,
        WORKSPACE_CONSTRAINT,
        DECISION_PACK,
    ];

    /// Built-in declarative kind names.
    pub const DECLARATIVE: &[&str] = &[
        PROVENANCE,
        GOVERNANCE_STATUS,
        REVIEW_ANNOTATION,
        JURISDICTION_TAG,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_classifies_structural_and_declarative_kinds() {
        let catalogue = KindCatalogue::builtin();
        assert_eq!(catalogue.len(), 24);
        assert!(catalogue
            .classify(builtin::GATEWAY)
            .unwrap()
            .is(KindRole::Structural, "gateway"));
        assert!(catalogue
            .classify(builtin::PROVENANCE)
            .unwrap()
            .is(KindRole::Declarative, "provenance"));
    }

    #[test]
    fn unknown_kind_is_a_typed_error_naming_the_catalogue() {
        let catalogue = KindCatalogue::builtin();
        let err = catalogue.classify("not-a-real-kind").unwrap_err();
        assert_eq!(
            err,
            UnknownKind {
                kind: "not-a-real-kind".to_owned(),
                catalogue: builtin::CATALOGUE_NAME.to_owned(),
            }
        );
        assert!(err.to_string().contains(builtin::CATALOGUE_NAME));
    }

    #[test]
    fn register_extends_by_data() {
        let catalogue = KindCatalogue::builtin()
            .with("board", KindRole::Structural)
            .unwrap();
        assert!(catalogue
            .classify("board")
            .unwrap()
            .is(KindRole::Structural, "board"));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut catalogue = KindCatalogue::new("t").unwrap();
        catalogue.register("x", KindRole::Structural).unwrap();
        let err = catalogue.register("x", KindRole::Declarative).unwrap_err();
        assert!(matches!(err, CatalogueError::Duplicate { .. }));
    }

    #[test]
    fn kind_name_validation() {
        assert!(KindName::new("gateway").is_ok());
        assert!(KindName::new("boundary-attachment").is_ok());
        assert!(KindName::new("_x1").is_ok());
        assert!(KindName::new("").is_err());
        assert!(KindName::new("1abc").is_err());
        assert!(KindName::new("has space").is_err());
        assert!(KindName::new("dotted.name").is_err());
        assert!(KindName::new("x".repeat(MAX_KIND_NAME_LEN + 1)).is_err());
    }

    #[test]
    fn from_source_with_header_and_extends() {
        let src = r#"
            (kind-catalogue board-kinds :extends builtin)
            (kind board :role structural)
            (kind attestation :role declarative)
        "#;
        let catalogue = KindCatalogue::from_source(src).unwrap();
        assert_eq!(catalogue.name(), "board-kinds");
        assert_eq!(catalogue.len(), 26);
        assert_eq!(catalogue.role_of("board"), Some(KindRole::Structural));
        assert_eq!(
            catalogue.role_of("attestation"),
            Some(KindRole::Declarative)
        );
        assert_eq!(catalogue.role_of("gateway"), Some(KindRole::Structural));
    }

    #[test]
    fn from_source_without_header_is_standalone() {
        let catalogue = KindCatalogue::from_source("(kind piece :role structural)").unwrap();
        assert_eq!(catalogue.len(), 1);
        assert!(!catalogue.contains("gateway"));
    }

    #[test]
    fn from_source_rejects_foreign_atoms_and_bad_roles() {
        assert!(matches!(
            KindCatalogue::from_source("(node x)").unwrap_err(),
            CatalogueError::UnexpectedAtom { kind } if kind == "node"
        ));
        assert!(matches!(
            KindCatalogue::from_source("(kind x :role sideways)").unwrap_err(),
            CatalogueError::Source { .. }
        ));
        assert!(matches!(
            KindCatalogue::from_source("(kind-catalogue c :extends other)").unwrap_err(),
            CatalogueError::UnknownBase { name } if name == "other"
        ));
        assert!(matches!(
            KindCatalogue::from_source("(kind x :role structural) (kind x :role structural)")
                .unwrap_err(),
            CatalogueError::Duplicate { .. }
        ));
    }

    #[test]
    fn to_source_round_trips() {
        let catalogue = KindCatalogue::builtin()
            .with("board", KindRole::Structural)
            .unwrap();
        let reparsed = KindCatalogue::from_source(&catalogue.to_source()).unwrap();
        assert_eq!(reparsed, catalogue);
    }

    #[test]
    fn catalogue_name_must_be_symbol_shaped() {
        assert!(KindCatalogue::new("board.kinds-v1").is_ok());
        assert!(matches!(
            KindCatalogue::new("has/slash").unwrap_err(),
            CatalogueError::InvalidCatalogueName { .. }
        ));
        assert!(KindCatalogue::new("").is_err());
    }

    #[test]
    fn serde_round_trip() {
        let catalogue = KindCatalogue::builtin();
        let json = serde_json::to_string(&catalogue).unwrap();
        let back: KindCatalogue = serde_json::from_str(&json).unwrap();
        assert_eq!(back, catalogue);
    }
}

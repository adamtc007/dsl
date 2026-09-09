//! Typed atom bag: classifies raw atoms against a catalogue and provides
//! name-indexed access.
//!
//! `AtomBag` is the primary output of the `dsl-ast` crate. It is built from a
//! `SourceFile` produced by `dsl-parser` by classifying each top-level atom's
//! kind string against a [`KindCatalogue`]. Nested atoms (atoms appearing as
//! slot values) are not classified: their kinds belong to the frontend that
//! reads the enclosing slot.

use std::collections::HashMap;

use dsl_atoms::{AtomKindClass, KindCatalogue, KindRole, UnknownKind};
use dsl_diagnostics::{Diagnostic, DiagnosticBag, UNKNOWN_ATOM_KIND};
use dsl_parser::{RawAtom, SourceFile};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// AtomIndex
// ---------------------------------------------------------------------------

/// A typed index into an `AtomBag`. Cheap to copy and compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AtomIndex(pub usize);

// ---------------------------------------------------------------------------
// TypedAtom
// ---------------------------------------------------------------------------

/// A raw atom paired with its classified kind.
///
/// Slot extraction is the owning frontend's business. Consumers read
/// `raw.slots` directly; every slot written in source is present verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedAtom {
    /// Classified atom kind.
    pub kind_class: AtomKindClass,
    /// Optional atom name (mirrors `raw.name`).
    pub name: Option<String>,
    /// The raw atom from the parser.
    pub raw: RawAtom,
}

// ---------------------------------------------------------------------------
// AtomBag
// ---------------------------------------------------------------------------

/// A classified, name-indexed collection of atoms from a single source file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtomBag {
    atoms: Vec<TypedAtom>,
    by_name: HashMap<String, AtomIndex>,
}

impl AtomBag {
    /// Build an `AtomBag` from a parsed `SourceFile`, classifying each
    /// top-level atom against `catalogue`.
    ///
    /// Every unknown kind produces an error diagnostic (code
    /// [`UNKNOWN_ATOM_KIND`]) naming the catalogue, and the first one is
    /// returned as a typed [`UnknownKind`] error: there is no default
    /// classification. Duplicate atom names produce a `Warning` diagnostic;
    /// the second occurrence is still added to the bag but is not indexed by
    /// name (first wins).
    pub fn from_source_file(
        source: SourceFile,
        catalogue: &KindCatalogue,
        diagnostics: &mut DiagnosticBag,
    ) -> Result<Self, UnknownKind> {
        let mut atoms: Vec<TypedAtom> = Vec::with_capacity(source.atoms.len());
        let mut by_name: HashMap<String, AtomIndex> = HashMap::new();
        let mut first_unknown: Option<UnknownKind> = None;

        for raw in source.atoms {
            let kind_class = match catalogue.classify(&raw.kind) {
                Ok(kind_class) => kind_class,
                Err(unknown) => {
                    diagnostics
                        .push(Diagnostic::error(unknown.to_string()).with_code(UNKNOWN_ATOM_KIND));
                    first_unknown.get_or_insert(unknown);
                    continue;
                }
            };

            let name = raw.name.clone();
            let index = AtomIndex(atoms.len());

            atoms.push(TypedAtom {
                kind_class,
                name: name.clone(),
                raw,
            });

            if let Some(n) = name {
                if by_name.contains_key(n.as_str()) {
                    diagnostics.push(Diagnostic::warning(format!(
                        "Duplicate atom name '{}'; first occurrence wins in name index",
                        n
                    )));
                } else {
                    by_name.insert(n, index);
                }
            }
        }

        match first_unknown {
            Some(unknown) => Err(unknown),
            None => Ok(Self { atoms, by_name }),
        }
    }

    /// Return the `TypedAtom` at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds (indices are always produced by the
    /// bag that issued them, so this should not occur in practice).
    #[must_use]
    pub fn get(&self, idx: AtomIndex) -> &TypedAtom {
        &self.atoms[idx.0]
    }

    /// Find an atom by name. Returns `None` if no atom with that name exists
    /// or if the name was a duplicate (first wins; later duplicates are not
    /// indexed).
    #[must_use]
    pub fn find(&self, name: &str) -> Option<AtomIndex> {
        self.by_name.get(name).copied()
    }

    /// Iterate over all atoms in source order.
    pub fn atoms(&self) -> impl Iterator<Item = &TypedAtom> {
        self.atoms.iter()
    }

    /// Iterate over all structural atoms.
    pub fn structural_atoms(&self) -> impl Iterator<Item = &TypedAtom> {
        self.atoms_of_role(KindRole::Structural)
    }

    /// Iterate over all declarative atoms.
    pub fn declarative_atoms(&self) -> impl Iterator<Item = &TypedAtom> {
        self.atoms_of_role(KindRole::Declarative)
    }

    /// Iterate over all atoms with the given role.
    pub fn atoms_of_role(&self, role: KindRole) -> impl Iterator<Item = &TypedAtom> {
        self.atoms
            .iter()
            .filter(move |a| a.kind_class.role() == role)
    }

    /// Return all atoms with the given kind name, whatever its role.
    #[must_use]
    pub fn atoms_of_kind(&self, kind: &str) -> Vec<&TypedAtom> {
        self.atoms
            .iter()
            .filter(|a| a.kind_class.name().as_str() == kind)
            .collect()
    }

    /// Return all structural atoms with the given kind name.
    #[must_use]
    pub fn atoms_of_structural_kind(&self, kind: &str) -> Vec<&TypedAtom> {
        self.atoms
            .iter()
            .filter(|a| a.kind_class.is(KindRole::Structural, kind))
            .collect()
    }

    /// Return all declarative atoms with the given kind name.
    #[must_use]
    pub fn atoms_of_declarative_kind(&self, kind: &str) -> Vec<&TypedAtom> {
        self.atoms
            .iter()
            .filter(|a| a.kind_class.is(KindRole::Declarative, kind))
            .collect()
    }

    /// Total number of atoms in the bag.
    #[must_use]
    pub fn len(&self) -> usize {
        self.atoms.len()
    }

    /// Returns `true` if the bag contains no atoms.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }
}

// ---------------------------------------------------------------------------
// AtomParser
// ---------------------------------------------------------------------------

/// A parser front end bound to one kind catalogue.
///
/// This is the entry point for a consumer that supplies its own catalogue:
/// construct it once with the catalogue and parse any number of sources.
#[derive(Debug, Clone)]
pub struct AtomParser {
    catalogue: KindCatalogue,
}

impl AtomParser {
    /// A parser classifying against `catalogue`.
    #[must_use]
    pub fn new(catalogue: KindCatalogue) -> Self {
        Self { catalogue }
    }

    /// A parser classifying against [`KindCatalogue::builtin`].
    #[must_use]
    pub fn builtin() -> Self {
        Self::new(KindCatalogue::builtin())
    }

    /// The catalogue this parser classifies against.
    #[must_use]
    pub fn catalogue(&self) -> &KindCatalogue {
        &self.catalogue
    }

    /// Parse `src` and classify its top-level atoms.
    ///
    /// Parse diagnostics and classification diagnostics are both appended to
    /// `diagnostics`. Unknown kinds are returned as a typed error.
    pub fn parse(
        &self,
        src: &str,
        diagnostics: &mut DiagnosticBag,
    ) -> Result<AtomBag, UnknownKind> {
        let (source, parse_diagnostics) = dsl_parser::parse(src);
        diagnostics
            .diagnostics
            .extend(parse_diagnostics.diagnostics);
        AtomBag::from_source_file(source, &self.catalogue, diagnostics)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use dsl_atoms::builtin;
    use dsl_parser::parse;

    const MINI_SOURCE: &str = r#"
        (gateway start-gate :kind exclusive)
        (provenance :author "test")
    "#;

    fn bag(src: &str) -> (AtomBag, DiagnosticBag) {
        let (sf, _) = parse(src);
        let mut diag = DiagnosticBag::new();
        let bag = AtomBag::from_source_file(sf, &KindCatalogue::builtin(), &mut diag)
            .expect("all kinds known");
        (bag, diag)
    }

    #[test]
    fn bag_from_mini_source() {
        let (bag, diag) = bag(MINI_SOURCE);

        assert_eq!(bag.len(), 2);
        assert!(
            !diag.has_errors(),
            "unexpected errors: {:?}",
            diag.diagnostics
        );

        let structural: Vec<_> = bag.structural_atoms().collect();
        assert_eq!(structural.len(), 1);
        assert_eq!(structural[0].raw.kind, "gateway");

        let declarative: Vec<_> = bag.declarative_atoms().collect();
        assert_eq!(declarative.len(), 1);
        assert_eq!(declarative[0].raw.kind, "provenance");
    }

    #[test]
    fn find_by_name() {
        let (bag, _) = bag(MINI_SOURCE);
        let idx = bag
            .find("start-gate")
            .expect("expected to find 'start-gate'");
        assert_eq!(bag.get(idx).raw.kind, "gateway");
    }

    #[test]
    fn atoms_of_structural_kind() {
        let (bag, _) =
            bag("(gateway g1 :kind exclusive) (gateway g2 :kind parallel) (node n :label \"N\")");
        assert_eq!(bag.atoms_of_structural_kind(builtin::GATEWAY).len(), 2);
        assert_eq!(bag.atoms_of_structural_kind(builtin::NODE).len(), 1);
        assert_eq!(bag.atoms_of_kind(builtin::NODE).len(), 1);
        assert!(bag.atoms_of_declarative_kind(builtin::NODE).is_empty());
    }

    #[test]
    fn duplicate_name_produces_diagnostic() {
        let (bag, diag) = bag("(node foo :x 1) (node foo :x 2)");

        assert_eq!(bag.len(), 2);
        assert!(
            !diag.has_errors(),
            "duplicate name should be a warning, not an error"
        );
        assert_eq!(diag.warnings().count(), 1);
        let idx = bag.find("foo").unwrap();
        assert_eq!(bag.get(idx).raw.slots[0].0, "x");
        match &bag.get(idx).raw.slots[0].1 {
            dsl_parser::RawValue::IntLit(v) => assert_eq!(*v, 1),
            other => panic!("expected IntLit(1), got {:?}", other),
        }
    }

    #[test]
    fn unknown_kind_is_typed_error_and_diagnostic() {
        let (sf, _) = parse("(flux-capacitor foo :speed 88)");
        let mut diag = DiagnosticBag::new();
        let err = AtomBag::from_source_file(sf, &KindCatalogue::builtin(), &mut diag).unwrap_err();

        assert_eq!(err.kind, "flux-capacitor");
        assert_eq!(err.catalogue, builtin::CATALOGUE_NAME);
        assert!(diag.has_errors(), "unknown kind should produce an error");
        let d = diag.errors().next().unwrap();
        assert_eq!(d.code.as_deref(), Some(UNKNOWN_ATOM_KIND));
        assert!(d.message.contains(builtin::CATALOGUE_NAME));
    }

    #[test]
    fn declarative_kind_coverage() {
        let (bag, diag) = bag(r#"
            (provenance :author "a")
            (governance-status :state active)
            (review-annotation :note "ok")
            (jurisdiction-tag :region EU)
        "#);

        assert!(!diag.has_errors());
        let decl: Vec<_> = bag.declarative_atoms().collect();
        assert_eq!(decl.len(), 4);
        assert!(decl.iter().any(|a| a
            .kind_class
            .is(KindRole::Declarative, builtin::JURISDICTION_TAG)));
    }

    #[test]
    fn atom_parser_merges_parse_and_classification_diagnostics() {
        let parser = AtomParser::builtin();
        let mut diag = DiagnosticBag::new();
        let err = parser
            .parse("junk (node ok :x 1) (mystery m)", &mut diag)
            .unwrap_err();
        assert_eq!(err.kind, "mystery");
        // one parse error ("junk"), one classification error ("mystery")
        assert_eq!(diag.errors().count(), 2);
    }
}

//! External-consumer view of the catalogue extension point: a frontend that
//! has never heard of the built-in kinds registers its own by data and
//! parses with them; an unregistered kind is a typed error, never a default.

use dsl_ast::{AtomBag, AtomParser};
use dsl_atoms::{builtin, KindCatalogue, KindRole, UnknownKind};
use dsl_diagnostics::{DiagnosticBag, UNKNOWN_ATOM_KIND};

const BOARD_SOURCE: &str = r#"
(board settlement :pieces [entitlement election])
(piece entitlement :holder-role custodian)
(guard record-date :deadline record-date-passed)
(attestation :by issuer-agent)
"#;

#[test]
fn register_new_kinds_by_data_and_parse_them() {
    let catalogue = KindCatalogue::new("board-frontend")
        .unwrap()
        .with("board", KindRole::Structural)
        .unwrap()
        .with("piece", KindRole::Structural)
        .unwrap()
        .with("guard", KindRole::Structural)
        .unwrap()
        .with("attestation", KindRole::Declarative)
        .unwrap();

    let parser = AtomParser::new(catalogue);
    let mut diagnostics = DiagnosticBag::new();
    let bag = parser.parse(BOARD_SOURCE, &mut diagnostics).unwrap();

    assert!(!diagnostics.has_errors());
    assert_eq!(bag.len(), 4);
    assert_eq!(bag.atoms_of_structural_kind("board").len(), 1);
    assert_eq!(bag.atoms_of_structural_kind("guard").len(), 1);
    assert_eq!(bag.atoms_of_declarative_kind("attestation").len(), 1);
    assert_eq!(bag.structural_atoms().count(), 3);
    assert!(bag.find("record-date").is_some());
    // The built-in names are not in this catalogue at all.
    assert!(!parser.catalogue().contains(builtin::NODE));
}

#[test]
fn catalogue_parsed_from_source_drives_the_parser() {
    let catalogue = KindCatalogue::from_source(
        r#"
        (kind-catalogue board-frontend :extends builtin)
        (kind board :role structural)
        (kind piece :role structural)
        (kind guard :role structural)
        (kind attestation :role declarative)
        "#,
    )
    .unwrap();

    let parser = AtomParser::new(catalogue);
    let mut diagnostics = DiagnosticBag::new();
    let bag = parser
        .parse(
            &format!("{BOARD_SOURCE}\n(node n :label \"built-in kind\")"),
            &mut diagnostics,
        )
        .unwrap();
    assert_eq!(bag.len(), 5);
    assert_eq!(bag.atoms_of_structural_kind(builtin::NODE).len(), 1);
}

#[test]
fn unknown_kind_is_a_typed_error_naming_the_catalogue() {
    let parser = AtomParser::new(
        KindCatalogue::new("tiny")
            .unwrap()
            .with("board", KindRole::Structural)
            .unwrap(),
    );
    let mut diagnostics = DiagnosticBag::new();
    let err = parser.parse(BOARD_SOURCE, &mut diagnostics).unwrap_err();

    assert_eq!(
        err,
        UnknownKind {
            kind: "piece".to_owned(),
            catalogue: "tiny".to_owned(),
        }
    );
    // Every unknown kind is diagnosed, not just the first.
    let unknown: Vec<_> = diagnostics
        .errors()
        .filter(|d| d.code.as_deref() == Some(UNKNOWN_ATOM_KIND))
        .collect();
    assert_eq!(unknown.len(), 3);
    assert!(unknown.iter().all(|d| d.message.contains("`tiny`")));
}

#[test]
fn builtin_catalogue_still_rejects_kinds_it_does_not_know() {
    let (file, _) = dsl_parser::parse("(board b)");
    let mut diagnostics = DiagnosticBag::new();
    let err =
        AtomBag::from_source_file(file, &KindCatalogue::builtin(), &mut diagnostics).unwrap_err();
    assert_eq!(err.kind, "board");
    assert_eq!(err.catalogue, builtin::CATALOGUE_NAME);
}

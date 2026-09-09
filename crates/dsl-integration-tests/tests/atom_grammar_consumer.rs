//! External-consumer proof for the unified DSL atom grammar crates: a second
//! domain builds its own kind catalogue by data and parses its own source
//! without editing any shared crate.

use dsl_ast::AtomParser;
use dsl_atoms::{KindCatalogue, KindRole};
use dsl_diagnostics::DiagnosticBag;
use dsl_parser::RawValue;

#[test]
fn second_domain_parses_its_own_kinds_with_a_data_catalogue() {
    let catalogue = KindCatalogue::from_source(
        r#"
        (kind-catalogue event-board)
        (kind board :role structural)
        (kind clock :role structural)
        (kind attested-by :role declarative)
        "#,
    )
    .expect("catalogue source parses");

    let parser = AtomParser::new(catalogue);
    let mut diagnostics = DiagnosticBag::new();
    let bag = parser
        .parse(
            r#"
            (board rights-issue :pieces [entitlement election] :roles [holder agent])
            (clock record-date :deadline record-date-at :on-expiry record-date-passed)
            (attested-by :role agent)
            "#,
            &mut diagnostics,
        )
        .expect("every kind is registered");

    assert!(!diagnostics.has_errors());
    assert_eq!(bag.structural_atoms().count(), 2);
    assert_eq!(bag.declarative_atoms().count(), 1);

    let clock = bag.get(bag.find("record-date").unwrap());
    assert!(clock.kind_class.is(KindRole::Structural, "clock"));
    assert_eq!(
        clock.raw.slots[1],
        (
            "on-expiry".to_owned(),
            RawValue::Symbol("record-date-passed".to_owned())
        )
    );
}

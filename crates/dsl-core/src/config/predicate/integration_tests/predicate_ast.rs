use crate::config::predicate::{
    parse_green_when, CmpOp, EntityRef, EntitySetRef, Predicate, Validity,
};

#[test]
fn count_predicate_fixture_is_structured() {
    let ast = parse_green_when("count(cbu_evidence where state = APPROVED) >= 2")
        .expect("count predicate parses");

    let Predicate::Count {
        set,
        condition,
        op,
        threshold,
    } = ast
    else {
        panic!("expected Count predicate");
    };

    assert_eq!(
        set,
        EntitySetRef {
            kind: "cbu_evidence".to_string(),
            qualifier: None,
            scope: None,
        }
    );
    assert_eq!(op, CmpOp::Ge);
    assert_eq!(threshold, 2);
    assert!(matches!(
        condition.as_deref(),
        Some(Predicate::StateIn {
            entity: EntityRef::This,
            state_set,
        }) if state_set == &vec!["APPROVED".to_string()]
    ));
}

#[test]
fn count_predicate_allows_nested_function_like_attr_values() {
    let ast = parse_green_when("count(evidence where evidence.source = required(foo)) >= 1")
        .expect("count predicate with nested attr value parses");

    assert!(matches!(
        ast,
        Predicate::Count {
            condition: Some(_),
            op: CmpOp::Ge,
            threshold: 1,
            ..
        }
    ));
}

#[test]
fn obtained_predicate_fixture_is_structured() {
    let ast = parse_green_when("obtained(kyc_case.state in {APPROVED, ACTIVE})")
        .expect("obtained predicate parses");

    assert!(matches!(
        ast,
        Predicate::Obtained {
            entity: EntityRef::Named(ref entity),
            validity: Validity::StateIn(ref states),
        } if entity == "kyc_case"
            && states == &vec!["APPROVED".to_string(), "ACTIVE".to_string()]
    ));
}

#[test]
fn orphaned_attached_to_scope_has_specific_parse_error() {
    let err = parse_green_when("attached_to this clearance").expect_err("orphaned scope fails");

    assert!(err.message.contains("orphaned `attached_to`"), "{err:?}");
}

//! Threshold predicates and validation bounds are fixed-point decimals on the
//! wire: integer literals and quoted decimal text are exact, floating-point
//! literals are rejected, and `SourceOfTruth::Stream` carries an opaque id.

use dsl_core::{ArgValidation, EscalationPredicate, SourceOfTruth, ThreeAxisDeclaration};
use rust_decimal::Decimal;
use std::str::FromStr;

const DECLARATION: &str = r#"
state_effect: transition
external_effects: []
consequence:
  baseline: benign
  escalation:
    - name: bulk
      when:
        op: arg_gt
        arg: amount
        value: 10000000
      tier: requires_confirmation
    - name: share
      when:
        op: arg_gte
        arg: share
        value: "0.25"
      tier: reviewable
transitions:
  dag: example_dag
  edges: []
"#;

#[test]
fn integer_and_quoted_decimal_thresholds_are_exact() {
    let decl: ThreeAxisDeclaration = serde_yaml::from_str(DECLARATION).unwrap();
    match &decl.consequence.escalation[0].when {
        EscalationPredicate::ArgGt { arg, value } => {
            assert_eq!(arg, "amount");
            assert_eq!(*value, Decimal::from(10_000_000));
        }
        other => panic!("expected ArgGt, got {other:?}"),
    }
    match &decl.consequence.escalation[1].when {
        EscalationPredicate::ArgGte { value, .. } => {
            assert_eq!(*value, Decimal::from_str("0.25").unwrap());
        }
        other => panic!("expected ArgGte, got {other:?}"),
    }
    // Round trip keeps decimal text, never a float.
    let json = serde_json::to_string(&decl.consequence.escalation[1].when).unwrap();
    assert!(json.contains(r#""value":"0.25""#), "{json}");
    let back: EscalationPredicate = serde_json::from_str(&json).unwrap();
    assert_eq!(back, decl.consequence.escalation[1].when);
}

#[test]
fn float_literal_threshold_is_rejected() {
    let yaml = DECLARATION.replace("value: \"0.25\"", "value: 0.25");
    let err = serde_yaml::from_str::<ThreeAxisDeclaration>(&yaml).unwrap_err();
    assert!(
        err.to_string().contains("floating-point threshold"),
        "{err}"
    );
}

#[test]
fn arg_validation_bounds_are_decimal() {
    let v: ArgValidation = serde_yaml::from_str("min: 0\nmax: \"100.5\"\n").unwrap();
    assert_eq!(v.min, Some(Decimal::ZERO));
    assert_eq!(v.max, Some(Decimal::from_str("100.5").unwrap()));
    let none: ArgValidation = serde_yaml::from_str("pattern: '^x'\n").unwrap();
    assert_eq!(none.min, None);
    assert!(serde_yaml::from_str::<ArgValidation>("min: 0.5\n").is_err());
}

#[test]
fn source_of_truth_stream_is_data_carried() {
    let stream: SourceOfTruth = serde_yaml::from_str("stream: settlement_events").unwrap();
    assert_eq!(
        stream,
        SourceOfTruth::Stream("settlement_events".to_owned())
    );
    let unit: SourceOfTruth = serde_yaml::from_str("workflow").unwrap();
    assert_eq!(unit, SourceOfTruth::Workflow);
    let json = serde_json::to_string(&stream).unwrap();
    assert_eq!(json, r#"{"stream":"settlement_events"}"#);
    assert_eq!(
        serde_json::from_str::<SourceOfTruth>(&json).unwrap(),
        stream
    );
}

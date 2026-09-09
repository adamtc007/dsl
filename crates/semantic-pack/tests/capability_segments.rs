//! Capability segments are pack data: a pack declares its segments and what
//! each may do; validation and the resolver enforce them by declared
//! semantics, not by prefix. Nothing here is built in.

use proptest::prelude::*;
use semantic_pack::{
    admit_pack, CapabilityId, CapabilitySegmentName, CapabilitySelectorSource, DiagnosticCode,
    PackAdmissionError, PackBytes, SegmentPermission, SegmentRuling,
};

fn pack(segments: &str, capabilities: &str, eligibility: &str) -> String {
    format!(
        r#"
schema_version: 1
pack:
  id: example.segmented
  version: 1.0.0
  domain: example
  identity_namespace: example.segmented
  canonicalization_version: 1
  dependencies: []
  provenance: {{ source: fixtures/segmented.yaml, revision: test-v1 }}
declarations:
  domain_types: []
  slot_kinds: []
  focus_kinds: []
{segments}
capabilities:
{capabilities}
policy:
  phrase_ambiguity: reject
  abstention: {{ enabled: true, candidate_id: abstain.none_of_the_above }}
  roles: []
{eligibility}
extensions: {{}}
"#
    )
}

const SEGMENTS: &str = r#"
  capability_segments:
    position: 1
    segments:
      - { name: derive, may: [read] }
      - { name: assert, may: [append] }
      - { name: decide, may: [read, append] }
      - { name: effect, may: [effect] }
"#;

fn capability(id: &str, action_class: &str) -> String {
    format!(
        r#"  - id: {id}
    adapter_binding: example.{id}
    title: {id}
    intent_summary: Exercise {id}.
    action_class: {action_class}
    applicability: Always.
    effect: Declared by its segment.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: reversible
"#
    )
}

fn good_capabilities() -> String {
    [
        capability("board.derive.entitlement", "compute"),
        capability("board.assert.election", "create"),
        capability("board.decide.settle", "approve"),
        capability("board.effect.pay", "execute"),
    ]
    .concat()
}

fn admission_codes(source: String) -> Vec<(DiagnosticCode, String)> {
    match admit_pack(PackBytes::new("segmented.yaml", source)) {
        Err(PackAdmissionError::Validation(errors)) => errors
            .diagnostics()
            .iter()
            .map(|d| (d.code, d.message.clone()))
            .collect(),
        Err(other) => panic!("expected validation failure, got {other:?}"),
        Ok(_) => panic!("expected validation failure, pack was admitted"),
    }
}

#[test]
fn declared_segments_admit_and_rule_by_declared_semantics() {
    let compiled = admit_pack(PackBytes::new(
        "segmented.yaml",
        pack(SEGMENTS, &good_capabilities(), ""),
    ))
    .expect("segmented pack admits");

    let derive = CapabilityId::new("board.derive.entitlement").unwrap();
    let effect = CapabilityId::new("board.effect.pay").unwrap();
    assert_eq!(
        compiled.capability_segment(&derive).unwrap().name.as_str(),
        "derive"
    );
    assert!(matches!(
        compiled.segment_ruling(&derive, SegmentPermission::Read),
        SegmentRuling::Permitted { .. }
    ));
    assert!(matches!(
        compiled.segment_ruling(&derive, SegmentPermission::Effect),
        SegmentRuling::Forbidden { ref segment, .. } if segment.as_str() == "derive"
    ));
    assert!(matches!(
        compiled.segment_ruling(&effect, SegmentPermission::Append),
        SegmentRuling::Forbidden { .. }
    ));
    let stranger = CapabilityId::new("board.other.thing").unwrap();
    assert_eq!(
        compiled.segment_ruling(&stranger, SegmentPermission::Read),
        SegmentRuling::Undeclared
    );

    // Segment selectors match by declared segment, not by prefix.
    let selector = CapabilitySelectorSource::Segment(CapabilitySegmentName::new("derive").unwrap());
    assert!(selector.matches_in(&derive, compiled.capability_segments()));
    assert!(!selector.matches_in(&effect, compiled.capability_segments()));
    assert!(!selector.matches(&derive), "no policy, no segment match");
}

#[test]
fn packs_without_a_segment_policy_are_unchanged() {
    let compiled = admit_pack(PackBytes::new(
        "plain.yaml",
        pack("", &good_capabilities(), ""),
    ))
    .expect("plain pack admits");
    assert!(compiled.capability_segments().is_none());
    assert_eq!(
        compiled.segment_ruling(
            &CapabilityId::new("board.derive.entitlement").unwrap(),
            SegmentPermission::Effect
        ),
        SegmentRuling::NoPolicy
    );
    let canonical = String::from_utf8_lossy(compiled.canonical_bytes()).into_owned();
    assert!(
        !canonical.contains("capability_segments"),
        "absent policy must not appear in canonical bytes, so existing hashes are stable"
    );
}

#[test]
fn undeclared_segment_is_a_typed_validation_error() {
    let codes = admission_codes(pack(
        SEGMENTS,
        &format!(
            "{}{}",
            good_capabilities(),
            capability("board.observe.x", "read")
        ),
        "",
    ));
    assert!(
        codes
            .iter()
            .any(|(code, message)| *code == DiagnosticCode::InvalidSegment
                && message.contains("board.observe.x")
                && message.contains("no declared segment")),
        "{codes:?}"
    );
}

#[test]
fn action_class_outside_segment_permission_is_rejected() {
    let codes = admission_codes(pack(
        SEGMENTS,
        &format!(
            "{}{}",
            good_capabilities(),
            capability("board.derive.mutate", "update")
        ),
        "",
    ));
    assert!(
        codes
            .iter()
            .any(|(code, message)| *code == DiagnosticCode::InvalidSegment
                && message.contains("board.derive.mutate")
                && message.contains("requires `Append`")),
        "{codes:?}"
    );
}

#[test]
fn segment_selector_must_name_a_declared_segment() {
    let eligibility = r#"
  eligibility:
    - context: mode.default
      default: deny
      allow:
        - { kind: segment, value: derive }
        - { kind: segment, value: mystery }
"#;
    let codes = admission_codes(pack(SEGMENTS, &good_capabilities(), eligibility));
    assert!(codes
        .iter()
        .any(|(code, message)| *code == DiagnosticCode::InvalidSegment
            && message.contains("`mystery`")));
    assert!(!codes
        .iter()
        .any(|(_, message)| message.contains("`derive`")));

    let without_policy = admission_codes(pack("", &good_capabilities(), eligibility));
    assert!(without_policy
        .iter()
        .any(|(code, message)| *code == DiagnosticCode::InvalidSegment
            && message.contains("requires `declarations.capability_segments`")));
}

#[test]
fn malformed_segment_policies_are_rejected() {
    let empty = r#"
  capability_segments:
    position: 1
    segments: []
"#;
    let codes = admission_codes(pack(empty, &good_capabilities(), ""));
    assert!(codes
        .iter()
        .any(|(code, _)| *code == DiagnosticCode::InvalidSegment));

    let no_permission = r#"
  capability_segments:
    position: 1
    segments:
      - { name: derive, may: [] }
"#;
    let codes = admission_codes(pack(
        no_permission,
        &capability("board.derive.entitlement", "compute"),
        "",
    ));
    assert!(codes
        .iter()
        .any(|(_, message)| message.contains("at least one permission")));
}

proptest! {
    #[test]
    fn segment_names_are_single_lowercase_segments(name in "[a-z][a-z0-9_-]{0,20}") {
        let parsed = CapabilitySegmentName::new(name.clone());
        if name.ends_with('-') || name.contains("--") {
            prop_assert!(parsed.is_err());
        } else {
            let parsed = parsed.unwrap();
            prop_assert_eq!(parsed.as_str(), name.as_str());
        }
        let dotted = format!("{name}.x");
        prop_assert!(CapabilitySegmentName::new(dotted).is_err());
        let upper_rejected = CapabilitySegmentName::new(name.to_ascii_uppercase()).is_err();
        prop_assert!(upper_rejected);
    }

    #[test]
    fn segment_position_selects_exactly_that_dotted_part(
        parts in proptest::collection::vec("[a-z][a-z0-9]{0,6}", 1..5),
        position in 0usize..6
    ) {
        let id = CapabilityId::new(parts.join(".")).unwrap();
        let policy = semantic_pack::CapabilitySegmentPolicySource {
            position,
            segments: parts
                .iter()
                .map(|part| semantic_pack::CapabilitySegmentSource {
                    name: CapabilitySegmentName::new(part.clone()).unwrap(),
                    may: vec![SegmentPermission::Read],
                })
                .collect(),
        };
        prop_assert_eq!(policy.segment_name_of(&id), parts.get(position).map(String::as_str));
        match policy.ruling(&id, SegmentPermission::Read) {
            SegmentRuling::Permitted { segment } => {
                prop_assert_eq!(segment.as_str(), parts[position].as_str());
            }
            SegmentRuling::Undeclared => prop_assert!(position >= parts.len()),
            other => prop_assert!(false, "unexpected ruling {other:?}"),
        }
        let effect_ruling = policy.ruling(&id, SegmentPermission::Effect);
        let effect_withheld = matches!(
            effect_ruling,
            SegmentRuling::Forbidden { .. } | SegmentRuling::Undeclared
        );
        prop_assert!(effect_withheld);
    }
}

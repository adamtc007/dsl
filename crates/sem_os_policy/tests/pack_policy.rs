use std::sync::Arc;

use sem_os_policy::pack_policy::{
    context_has_attribute, evaluate_capability, has_privilege, identity_namespace_uuid,
    CapabilityAdapter, CapabilityAdapterRegistry, PackPolicyError, PolicyReason, PrincipalContext,
};
use semantic_pack::{
    admit_pack, AdapterBindingId, CapabilityId, InMemoryPackRegistry, PackBytes, PackRegistry,
    PolicyAttributeId, PolicyContextId, PrivilegeId,
};

const POLICY_PACK: &str = r#"
schema_version: 1
pack:
  id: example.policy
  version: 1.0.0
  domain: example
  identity_namespace: example.semantic.v1
  identity_namespace_uuid: 7a3b9f42-e1d4-5a8b-910c-4f2d6e8a1b3c
  canonicalization_version: 1
  dependencies: []
  provenance: { source: fixtures/policy.yaml, revision: test-v1 }
declarations: { domain_types: [], slot_kinds: [], focus_kinds: [] }
capabilities:
  - id: change.add
    adapter_binding: example.change.add
    title: Add a change
    intent_summary: Add one declared change.
    action_class: create
    applicability: A change set is active.
    effect: One change is appended.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: reversible
  - id: record.read
    adapter_binding: example.record.read
    title: Read a record
    intent_summary: Read one admitted record.
    action_class: read
    applicability: The record exists.
    effect: The record is returned without mutation.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: read_only
policy:
  phrase_ambiguity: reject
  abstention: { enabled: true, candidate_id: abstain.none_of_the_above }
  roles:
    - role: administrator
      capabilities: [change.add]
  eligibility:
    - context: mode.research
      default: allow
      deny:
        - { kind: prefix, value: change. }
      attributes: [feature.full-introspection]
    - context: mode.governed
      default: deny
      allow:
        - { kind: exact, value: change.add }
        - { kind: prefix, value: record. }
  privileges:
    - privilege: evidence.review
      roles:
        - { kind: contains, value: steward }
        - { kind: exact, value: compliance_officer }
extensions: {}
"#;

fn snapshot() -> semantic_pack::SemanticSnapshot {
    let pack = admit_pack(PackBytes::new("policy.yaml", POLICY_PACK)).unwrap();
    InMemoryPackRegistry::new().install(pack).unwrap()
}

#[test]
fn selectors_defaults_role_grants_and_evidence_are_deterministic() {
    let snapshot = snapshot();
    let admin = PrincipalContext::new(["administrator"]);
    let viewer = PrincipalContext::new(["viewer"]);
    let change = CapabilityId::new("change.add").unwrap();

    let denied = evaluate_capability(
        &snapshot,
        &admin,
        &PolicyContextId::new("mode.research").unwrap(),
        &change,
    )
    .unwrap();
    assert!(!denied.allowed);
    assert!(matches!(denied.reason, PolicyReason::ExplicitDeny(_)));

    let allowed = evaluate_capability(
        &snapshot,
        &admin,
        &PolicyContextId::new("mode.governed").unwrap(),
        &change,
    )
    .unwrap();
    assert!(allowed.allowed);
    assert_eq!(allowed.reason, PolicyReason::RoleGrant);
    assert_eq!(
        allowed.evidence.artifact_hash,
        snapshot.pack().receipt().artifact_hash
    );

    let role_denied = evaluate_capability(
        &snapshot,
        &viewer,
        &PolicyContextId::new("mode.governed").unwrap(),
        &change,
    )
    .unwrap();
    assert!(!role_denied.allowed);
    assert_eq!(role_denied.reason, PolicyReason::RoleDenied);
}

#[test]
fn privilege_attributes_namespace_and_missing_context_are_typed() {
    let snapshot = snapshot();
    let privilege = PrivilegeId::new("evidence.review").unwrap();
    assert!(has_privilege(
        &snapshot,
        &PrincipalContext::new(["data_steward"]),
        &privilege
    )
    .unwrap());
    assert!(has_privilege(
        &snapshot,
        &PrincipalContext::new(["COMPLIANCE_OFFICER"]),
        &privilege
    )
    .unwrap());
    assert!(!has_privilege(&snapshot, &PrincipalContext::new(["analyst"]), &privilege).unwrap());
    assert!(context_has_attribute(
        &snapshot,
        &PolicyContextId::new("mode.research").unwrap(),
        &PolicyAttributeId::new("feature.full-introspection").unwrap(),
    )
    .unwrap());
    assert_eq!(
        identity_namespace_uuid(&snapshot).unwrap().to_string(),
        "7a3b9f42-e1d4-5a8b-910c-4f2d6e8a1b3c"
    );
    let error = evaluate_capability(
        &snapshot,
        &PrincipalContext::default(),
        &PolicyContextId::new("mode.missing").unwrap(),
        &CapabilityId::new("record.read").unwrap(),
    )
    .unwrap_err();
    assert!(matches!(error, PackPolicyError::MissingContext(_)));
}

struct ReadAdapter {
    binding: AdapterBindingId,
}

impl CapabilityAdapter for ReadAdapter {
    fn binding_id(&self) -> &AdapterBindingId {
        &self.binding
    }
}

#[test]
fn adapter_registry_resolves_only_pack_selected_bindings() {
    let snapshot = snapshot();
    let adapter = Arc::new(ReadAdapter {
        binding: AdapterBindingId::new("example.record.read").unwrap(),
    });
    let mut registry = CapabilityAdapterRegistry::default();
    registry.register(adapter.clone()).unwrap();
    let resolved = registry
        .resolve(&snapshot, &CapabilityId::new("record.read").unwrap())
        .unwrap();
    assert_eq!(resolved.binding_id(), adapter.binding_id());
}

const SEGMENTED_PACK: &str = r#"
schema_version: 1
pack:
  id: example.segmented
  version: 1.0.0
  domain: example
  identity_namespace: example.segmented
  canonicalization_version: 1
  dependencies: []
  provenance: { source: fixtures/segmented.yaml, revision: test-v1 }
declarations:
  domain_types: []
  slot_kinds: []
  focus_kinds: []
  capability_segments:
    position: 1
    segments:
      - { name: derive, may: [read] }
      - { name: effect, may: [effect] }
capabilities:
  - id: board.derive.entitlement
    adapter_binding: example.derive
    title: Derive entitlement
    intent_summary: Compute an entitlement.
    action_class: compute
    applicability: Always.
    effect: Pure.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: read_only
  - id: board.effect.pay
    adapter_binding: example.effect
    title: Pay
    intent_summary: Cause a payment.
    action_class: execute
    applicability: Always.
    effect: External.
    arguments: []
    phrases: []
    positive_examples: []
    negative_contrasts: []
    risk: reversible
policy:
  phrase_ambiguity: reject
  abstention: { enabled: true, candidate_id: abstain.none_of_the_above }
  roles: []
  eligibility:
    - context: mode.pure
      default: deny
      allow:
        - { kind: segment, value: derive }
    - context: mode.prefix
      default: deny
      allow:
        - { kind: prefix, value: board. }
      deny:
        - { kind: segment, value: effect }
extensions: {}
"#;

#[test]
fn segment_rules_are_enforced_by_selectors_and_the_resolver() {
    use sem_os_policy::pack_policy::segment_permits;
    use semantic_pack::SegmentPermission;

    let pack = admit_pack(PackBytes::new("segmented.yaml", SEGMENTED_PACK)).unwrap();
    let snapshot = InMemoryPackRegistry::new().install(pack).unwrap();
    let anyone = PrincipalContext::new(["anyone"]);
    let derive = CapabilityId::new("board.derive.entitlement").unwrap();
    let effect = CapabilityId::new("board.effect.pay").unwrap();

    // Segment selectors: `derive` allowed in mode.pure, `effect` is not.
    let pure = PolicyContextId::new("mode.pure").unwrap();
    assert!(
        evaluate_capability(&snapshot, &anyone, &pure, &derive)
            .unwrap()
            .allowed
    );
    let effect_in_pure = evaluate_capability(&snapshot, &anyone, &pure, &effect).unwrap();
    assert!(!effect_in_pure.allowed);
    assert_eq!(effect_in_pure.reason, PolicyReason::DefaultDeny);

    // A segment deny beats a prefix allow: prefix matching is not the rule.
    let prefix = PolicyContextId::new("mode.prefix").unwrap();
    let effect_in_prefix = evaluate_capability(&snapshot, &anyone, &prefix, &effect).unwrap();
    assert!(!effect_in_prefix.allowed);
    assert!(matches!(
        effect_in_prefix.reason,
        PolicyReason::ExplicitDeny(semantic_pack::CapabilitySelectorSource::Segment(_))
    ));
    assert!(
        evaluate_capability(&snapshot, &anyone, &prefix, &derive)
            .unwrap()
            .allowed
    );

    // Permissions follow the declaration.
    assert!(segment_permits(&snapshot, &derive, SegmentPermission::Read).unwrap());
    assert!(!segment_permits(&snapshot, &derive, SegmentPermission::Effect).unwrap());
    assert!(segment_permits(&snapshot, &effect, SegmentPermission::Effect).unwrap());
    assert!(matches!(
        segment_permits(
            &snapshot,
            &CapabilityId::new("board.other.x").unwrap(),
            SegmentPermission::Read
        ),
        Err(PackPolicyError::UndeclaredCapabilitySegment(_))
    ));

    // The resolver refuses an adapter for a permission the segment withholds.
    struct Adapter(AdapterBindingId);
    impl CapabilityAdapter for Adapter {
        fn binding_id(&self) -> &AdapterBindingId {
            &self.0
        }
    }
    let mut registry = CapabilityAdapterRegistry::default();
    registry
        .register(Arc::new(Adapter(
            AdapterBindingId::new("example.derive").unwrap(),
        )))
        .unwrap();
    registry
        .register(Arc::new(Adapter(
            AdapterBindingId::new("example.effect").unwrap(),
        )))
        .unwrap();
    assert!(registry
        .resolve_for(&snapshot, &derive, SegmentPermission::Read)
        .is_ok());
    let refused = match registry.resolve_for(&snapshot, &derive, SegmentPermission::Effect) {
        Err(err) => err,
        Ok(_) => panic!("derive segment must not resolve for an effect"),
    };
    assert!(matches!(
        refused,
        PackPolicyError::SegmentForbids { ref segment, permission: SegmentPermission::Effect, .. }
            if segment.as_str() == "derive"
    ));
    assert!(registry
        .resolve_for(&snapshot, &effect, SegmentPermission::Effect)
        .is_ok());
    assert!(matches!(
        registry
            .resolve_for(&snapshot, &effect, SegmentPermission::Append)
            .err(),
        Some(PackPolicyError::SegmentForbids { .. })
    ));

    // Packs without a segment policy resolve exactly as before.
    let plain = snapshot_without_segments();
    assert!(segment_permits(
        &plain,
        &CapabilityId::new("change.add").unwrap(),
        SegmentPermission::Effect
    )
    .unwrap());
}

fn snapshot_without_segments() -> semantic_pack::SemanticSnapshot {
    snapshot()
}

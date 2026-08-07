use std::collections::BTreeMap;

use proptest::prelude::*;
use semantic_pack::{
    admit_pack, parse_pack, validate_pack, ArtifactHash, CapabilityId, InMemoryPackRegistry,
    PackBytes, PackId, PackRegistry, PackRequest, PackSource, PackSourceError, PackVersion,
    RegistryError,
};

const PACK_A: &str = r#"
schema_version: 1
pack:
  id: example.workflow
  version: 1.2.0
  domain: example
  identity_namespace: example.workflow
  canonicalization_version: 1
  dependencies: []
  provenance:
    source: fixtures/example.yaml
    revision: test-v1
declarations:
  domain_types: [example.process]
  slot_kinds: [process.reference, signal.name]
  focus_kinds: [process.instance]
capabilities:
  - id: process.signal
    adapter_binding: example.process.signal
    title: Signal a process
    intent_summary: Deliver a named signal to one process instance.
    action_class: execute
    applicability: A process instance is waiting for the named signal.
    effect: The waiting process instance may advance.
    arguments:
      - name: signal.name
        kind: text
        required: true
        clarification_prompt: Which signal should be delivered?
      - name: process.reference
        kind: identifier
        required: true
        clarification_prompt: Which process instance?
    phrases:
      - text: send signal
        locale: en-GB
        role: canonical
        provenance: fixture
    positive_examples: [signal the process]
    negative_contrasts:
      - candidate_id: process.start
        distinction: Starts a new process rather than advancing an existing one.
    risk: reversible
    aliases: [process.deliver-signal]
    extensions:
      example.presentation:
        icon: bolt
  - id: process.start
    adapter_binding: example.process.start
    title: Start a process
    intent_summary: Create a new process instance.
    action_class: create
    applicability: A deployed process definition is available.
    effect: A new process instance is created.
    arguments:
      - name: process.reference
        kind: identifier
        required: true
        clarification_prompt: Which process definition?
    phrases:
      - text: start process
        locale: en-GB
        role: canonical
        provenance: fixture
    positive_examples: [launch the process]
    negative_contrasts:
      - candidate_id: process.signal
        distinction: Advances an existing process rather than creating one.
    risk: reversible
policy:
  phrase_ambiguity: reject
  abstention:
    enabled: true
    candidate_id: abstain.none_of_the_above
  roles:
    - role: process.operator
      capabilities: [process.signal, process.start]
graph:
  acyclic: true
  entry_nodes: [choose]
  nodes:
    - id: done
      candidates: [process.signal]
      required: true
      terminal: candidate
    - id: choose
      candidates: [process.signal, process.start]
      required: true
  edges:
    - from: choose
      to: done
      when: an existing process is selected
extensions: {}
"#;

const PACK_B_REORDERED: &str = r#"
schema_version: 1
pack:
  id: example.workflow
  version: 1.2.0
  domain: example
  identity_namespace: example.workflow
  canonicalization_version: 1
  provenance: { revision: test-v1, source: fixtures/example.yaml }
  dependencies: []
declarations:
  focus_kinds: [process.instance]
  slot_kinds: [signal.name, process.reference]
  domain_types: [example.process]
capabilities:
  - id: process.start
    adapter_binding: example.process.start
    title: Start a process
    intent_summary: Create a new process instance.
    action_class: create
    applicability: A deployed process definition is available.
    effect: A new process instance is created.
    arguments:
      - { name: process.reference, kind: identifier, required: true, clarification_prompt: Which process definition? }
    phrases:
      - { provenance: fixture, role: canonical, locale: en-GB, text: start process }
    positive_examples: [launch the process]
    negative_contrasts:
      - { candidate_id: process.signal, distinction: Advances an existing process rather than creating one. }
    risk: reversible
  - id: process.signal
    adapter_binding: example.process.signal
    title: Signal a process
    intent_summary: Deliver a named signal to one process instance.
    action_class: execute
    applicability: A process instance is waiting for the named signal.
    effect: The waiting process instance may advance.
    arguments:
      - { name: process.reference, kind: identifier, required: true, clarification_prompt: Which process instance? }
      - { name: signal.name, kind: text, required: true, clarification_prompt: Which signal should be delivered? }
    phrases:
      - { text: send signal, locale: en-GB, role: canonical, provenance: fixture }
    positive_examples: [signal the process]
    negative_contrasts:
      - { candidate_id: process.start, distinction: Starts a new process rather than advancing an existing one. }
    aliases: [process.deliver-signal]
    risk: reversible
    extensions: { example.presentation: { icon: bolt } }
graph:
  acyclic: true
  entry_nodes: [choose]
  nodes:
    - { id: choose, required: true, candidates: [process.start, process.signal] }
    - { id: done, required: true, candidates: [process.signal], terminal: candidate }
  edges:
    - { to: done, from: choose, when: an existing process is selected }
policy:
  roles:
    - { role: process.operator, capabilities: [process.start, process.signal] }
  abstention: { candidate_id: abstain.none_of_the_above, enabled: true }
  phrase_ambiguity: reject
extensions: {}
"#;

fn compile(name: &str, yaml: &str) -> semantic_pack::CompiledPack {
    admit_pack(PackBytes::new(name, yaml.as_bytes())).unwrap()
}

fn governed_pack() -> String {
    PACK_A.replace(
        "extensions: {}\n",
        r#"evidence:
  version: 1
  features:
    - { lane: governed_exact, weight_millis: 4000 }
    - { lane: typed_argument, weight_millis: 2500 }
  deterministic_gates:
    - { candidate_id: process.start, lane: governed_exact, effect: require }
rule_explanations:
  - rule_code: rule.process.available
    message_key: process.available
    message: A process definition must be available before it can be started.
    disclosure: public
    feedback_options: [recovery.select_process]
feedback_options:
  - id: recovery.select_process
    rule_code: rule.process.available
    kind: select_alternative
    prompt_key: process.select
    prompt: Select an available process definition.
    candidate_id: process.start
    disclosure: public
    next_options: []
extensions: {}
"#,
    )
}

#[test]
fn public_pipeline_inspects_capabilities_graph_bindings_and_provenance() {
    let parsed = parse_pack(PackBytes::new("example.yaml", PACK_A)).unwrap();
    let validated = validate_pack(parsed).unwrap();
    assert_eq!(validated.document().capabilities.len(), 2);
    let pack = semantic_pack::compile_pack(validated).unwrap();
    assert_eq!(pack.identity().id.as_str(), "example.workflow");
    assert_eq!(pack.domain().as_str(), "example");
    assert_eq!(pack.provenance().revision, "test-v1");
    assert_eq!(pack.declarations().domain_types.len(), 1);
    assert!(pack.extensions().is_empty());
    assert_eq!(pack.capabilities().len(), 2);
    let signal = CapabilityId::new("process.signal").unwrap();
    assert_eq!(
        pack.adapter_binding(&signal).unwrap().as_str(),
        "example.process.signal"
    );
    let choose = "choose".parse().unwrap();
    assert_eq!(pack.successors(&choose)[0].id.as_str(), "done");
}

#[test]
fn compilation_is_independent_of_yaml_map_and_semantic_set_order() {
    let first = compile("a.yaml", PACK_A);
    let second = compile("b.yaml", PACK_B_REORDERED);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(
        first.receipt().artifact_hash,
        second.receipt().artifact_hash
    );
    assert_ne!(first.receipt().source_hash, second.receipt().source_hash);
}

#[test]
fn canonical_hash_golden_vector() {
    let pack = compile("example.yaml", PACK_A);
    assert_eq!(
        pack.receipt().artifact_hash.as_str(),
        "1576ce316374a0f97ba3ff4368aa71eb4a1974e27aa06cfa7d3933a0195993e7"
    );
}

#[test]
fn governed_evidence_rules_and_recovery_are_admitted_and_resolved() {
    let pack = compile("governed.yaml", &governed_pack());
    assert_eq!(pack.evidence().version, 1);
    assert_eq!(pack.evidence().features.len(), 2);
    let rule = semantic_decision_contracts::RuleCode::new("rule.process.available").unwrap();
    assert_eq!(
        pack.rule_explanation(&rule).unwrap().message_key.as_str(),
        "process.available"
    );
    let option = semantic_decision_contracts::MessageKey::new("recovery.select_process").unwrap();
    assert_eq!(
        pack.feedback_option(&option)
            .unwrap()
            .candidate_id
            .as_ref()
            .unwrap()
            .as_str(),
        "process.start"
    );
}

#[test]
fn evidence_policy_refuses_unknown_invalid_dangling_and_contradictory_data() {
    let cases = [
        governed_pack().replace("weight_millis: 4000", "weight_millis: 0"),
        governed_pack().replace("candidate_id: process.start", "candidate_id: process.missing"),
        governed_pack().replace(
            "- { candidate_id: process.start, lane: governed_exact, effect: require }",
            "- { candidate_id: process.start, lane: governed_exact, effect: require }\n    - { candidate_id: process.start, lane: governed_exact, effect: forbid }",
        ),
        governed_pack().replace("next_options: []", "next_options: [recovery.select_process]"),
        governed_pack().replace(
            "feedback_options: [recovery.select_process]",
            "feedback_options: [recovery.missing]",
        ),
    ];
    for invalid in cases {
        assert!(admit_pack(PackBytes::new("invalid-governance.yaml", invalid)).is_err());
    }

    let unknown = governed_pack().replace("governed_exact", "host_private_feature");
    let error = parse_pack(PackBytes::new("unknown-feature.yaml", unknown)).unwrap_err();
    assert!(error.message.contains("unknown variant"));
}

#[test]
fn parse_errors_include_source_line_and_column_and_unknown_fields_fail() {
    let error = parse_pack(PackBytes::new(
        "broken.yaml",
        "schema_version: 1\nunknown: true\n",
    ))
    .unwrap_err();
    assert_eq!(error.source_name, "broken.yaml");
    assert!(!error.yaml_path.is_empty());
    assert!(error.line.is_some());
    assert!(error.column.is_some());
    assert!(error.message.contains("unknown field"));
}

#[test]
fn validation_accumulates_sorted_cross_reference_and_graph_errors() {
    let invalid = PACK_A
        .replace(
            "capabilities: [process.signal, process.start]",
            "capabilities: [missing.capability]",
        )
        .replace("to: done", "to: missing")
        .replace(
            "slot_kinds: [process.reference, signal.name]",
            "slot_kinds: [signal.name]",
        );
    let document = parse_pack(PackBytes::new("invalid.yaml", invalid)).unwrap();
    let error = validate_pack(document).unwrap_err();
    assert!(error.diagnostics().len() >= 3);
    assert!(error
        .diagnostics()
        .windows(2)
        .all(|pair| pair[0].yaml_path <= pair[1].yaml_path));
    assert!(error
        .diagnostics()
        .iter()
        .all(|diagnostic| diagnostic.pack_id.as_deref() == Some("example.workflow")));
}

#[test]
fn cycle_and_executable_extension_material_are_rejected() {
    let invalid = PACK_A
        .replace(
            "- from: choose\n      to: done\n      when: an existing process is selected",
            "- from: choose\n      to: done\n      when: an existing process is selected\n    - from: done\n      to: choose\n      when: retry",
        )
        .replace("extensions: {}\n", "extensions: { example.script: \"fn execute()\" }\n");
    let document = parse_pack(PackBytes::new("invalid.yaml", invalid)).unwrap();
    let error = validate_pack(document).unwrap_err();
    assert!(error
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.message.contains("cycle")));
    assert!(error.diagnostics().iter().any(|diagnostic| {
        diagnostic.code == semantic_pack::DiagnosticCode::ExecutableMaterial
    }));
}

#[test]
fn natural_language_insert_and_update_are_not_mistaken_for_sql() {
    let natural = PACK_A
        .replace("Start a process", "Insert before the process")
        .replace(
            "Create a new process instance.",
            "Update the selected process.",
        );
    compile("natural-language.yaml", &natural);

    let sql = PACK_A.replace(
        "extensions: {}\n",
        "extensions: { example.query: \"select secret from host_table\" }\n",
    );
    let document = parse_pack(PackBytes::new("sql.yaml", sql)).unwrap();
    let errors = validate_pack(document).unwrap_err();
    assert!(errors.diagnostics().iter().any(|diagnostic| {
        diagnostic.code == semantic_pack::DiagnosticCode::ExecutableMaterial
    }));
}

struct EmbeddedSource {
    values: BTreeMap<String, &'static str>,
}

impl PackSource for EmbeddedSource {
    fn load(&self, request: &PackRequest) -> Result<PackBytes, PackSourceError> {
        let yaml = self
            .values
            .get(request.id.as_str())
            .ok_or_else(|| PackSourceError::NotFound(request.id.to_string()))?;
        if request
            .version
            .as_ref()
            .is_some_and(|version| version.as_str() != "1.2.0")
        {
            return Err(PackSourceError::Incompatible("version mismatch".to_owned()));
        }
        Ok(PackBytes::new("embedded/example.yaml", yaml.as_bytes()))
    }
}

#[test]
fn host_source_and_registry_retain_versions_and_fence_stale_activation() {
    let source = EmbeddedSource {
        values: BTreeMap::from([("example.workflow".to_owned(), PACK_A)]),
    };
    let request = PackRequest {
        id: PackId::new("example.workflow").unwrap(),
        version: Some(PackVersion::new("1.2.0").unwrap()),
    };
    let pack = admit_pack(source.load(&request).unwrap()).unwrap();
    let hash = pack.receipt().artifact_hash.clone();
    let identity = pack.identity();
    let registry = InMemoryPackRegistry::new();
    registry.install(pack).unwrap();
    assert_eq!(
        registry
            .resolve(&identity)
            .unwrap()
            .pack()
            .receipt()
            .artifact_hash,
        hash
    );
    assert!(matches!(
        registry.activate(&identity, Some(&ArtifactHash::new("0".repeat(64)).unwrap())),
        Err(RegistryError::StaleActivation { .. })
    ));
}

proptest! {
    #[test]
    fn valid_identifier_segments_round_trip(
        first in "[a-z][a-z0-9_]{0,15}",
        second in "[a-z][a-z0-9_]{0,15}"
    ) {
        let value = format!("{first}.{second}");
        let id = CapabilityId::new(&value).unwrap();
        let encoded = serde_json::to_string(&id).unwrap();
        prop_assert_eq!(serde_json::from_str::<CapabilityId>(&encoded).unwrap(), id);
    }
}

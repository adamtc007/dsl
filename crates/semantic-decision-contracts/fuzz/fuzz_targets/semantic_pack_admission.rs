#![no_main]

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU8, Ordering};

use libfuzzer_sys::fuzz_target;
use semantic_pack::{admit_pack, PackBytes};

static OUTCOMES: AtomicU8 = AtomicU8::new(0);

const VALID: &str = r#"
schema_version: 1
pack:
  id: fuzz.pack
  version: 1.0.0
  domain: fuzz
  identity_namespace: fuzz.pack
  canonicalization_version: 1
  provenance: { source: fuzz, revision: fuzz }
declarations:
  slot_kinds: [item.reference]
capabilities:
  - id: item.select
    adapter_binding: fuzz.item.select
    title: Select item
    intent_summary: Select one item.
    action_class: read
    applicability: An item is available.
    effect: The item is selected.
    arguments:
      - name: item.reference
        kind: identifier
        required: true
        clarification_prompt: Which item?
        requirement_rule: rule.item.required
        feedback_options: [recovery.select_item]
    phrases: [{ text: select item, locale: en, role: canonical, provenance: fuzz }]
    positive_examples: [choose the item]
    risk: read_only
    evidence_cues:
      - { lane: governed_exact, score_millis: 1000, cues: [select item] }
    applicability_rule: rule.item.available
    feedback_options: [recovery.retry]
evidence:
  version: 1
  features:
    - { lane: governed_exact, weight_millis: 4000 }
    - { lane: typed_argument, weight_millis: 2500 }
  deterministic_gates:
    - { candidate_id: item.select, lane: governed_exact, effect: require }
rule_explanations:
  - rule_code: rule.item.available
    message_key: item.available
    message: An item must be available.
    disclosure: public
    feedback_options: [recovery.retry]
  - rule_code: rule.item.required
    message_key: item.required
    message: An item reference is required.
    disclosure: public
    feedback_options: [recovery.select_item]
feedback_options:
  - id: recovery.retry
    rule_code: rule.item.available
    kind: retry
    prompt_key: item.retry
    prompt: Retry with an available item.
    candidate_id: item.select
    disclosure: public
    next_options: []
  - id: recovery.select_item
    rule_code: rule.item.required
    kind: supply_argument
    prompt_key: item.select
    prompt: Select an item.
    candidate_id: item.select
    disclosure: public
    next_options: []
"#;

fn observe(index: u8, label: &str) {
    let bit = 1_u8 << index;
    if OUTCOMES.fetch_or(bit, Ordering::Relaxed) & bit == 0 {
        eprintln!("semantic-counter pack_admission={label}");
    }
}

fn assert_closed_references(pack: &semantic_pack::CompiledPack) {
    let lanes = pack
        .evidence()
        .features
        .iter()
        .map(|feature| feature.lane)
        .collect::<BTreeSet<_>>();
    for gate in &pack.evidence().deterministic_gates {
        assert!(lanes.contains(&gate.lane));
        assert!(pack.capability(&gate.candidate_id).is_some());
    }
    for rule in pack.rule_explanations() {
        for option in &rule.feedback_options {
            assert!(pack.feedback_option(option).is_some());
        }
    }
    for option in pack.feedback_options() {
        assert!(pack.rule_explanation(&option.rule_code).is_some());
        assert!(option
            .candidate_id
            .as_ref()
            .is_none_or(|candidate| pack.capability(candidate).is_some()));
        for next in &option.next_options {
            assert!(pack.feedback_option(next).is_some());
        }
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(pack) = admit_pack(PackBytes::new("hostile.yaml", data)) {
        observe(0, "hostile_admitted");
        assert_closed_references(&pack);
        let repeated = admit_pack(PackBytes::new("hostile.yaml", data)).unwrap();
        assert_eq!(pack.canonical_bytes(), repeated.canonical_bytes());
        assert_eq!(
            pack.receipt().artifact_hash,
            repeated.receipt().artifact_hash
        );
    } else {
        observe(1, "hostile_refused");
    }

    let selector = data.first().copied().unwrap_or_default() % 7;
    let generated = match selector {
        0 => VALID.to_owned(),
        1 => VALID.replace("weight_millis: 4000", "weight_millis: 0"),
        2 => VALID.replace("lane: governed_exact", "lane: host_private"),
        3 => VALID.replace("candidate_id: item.select", "candidate_id: item.missing"),
        4 => VALID.replace("next_options: []", "next_options: [recovery.retry]"),
        5 => VALID.replace(
            "- { candidate_id: item.select, lane: governed_exact, effect: require }",
            "- { candidate_id: item.select, lane: governed_exact, effect: require }\n    - { candidate_id: item.select, lane: governed_exact, effect: forbid }",
        ),
        _ => VALID.replace("recovery.select_item]", "recovery.missing]"),
    };
    let admitted = admit_pack(PackBytes::new("generated.yaml", generated));
    if selector == 0 {
        observe(2, "generated_valid");
        assert_closed_references(&admitted.expect("valid generated pack must be admitted"));
    } else {
        observe(3, "generated_invalid");
        assert!(admitted.is_err());
    }
});

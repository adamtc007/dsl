use std::collections::{BTreeMap, BTreeSet};

use semantic_decision_contracts::{EvidenceLane, MessageKey, PhraseRole, RuleCode};
use sha2::{Digest, Sha256};

use crate::{
    ConfigValue, Diagnostic, DiagnosticCode, GraphNodeId, PackBytes, PackDocument, PackParseError,
    PackValidationErrors, PhraseAmbiguityPolicy, SUPPORTED_CANONICALIZATION_VERSION,
    SUPPORTED_SCHEMA_VERSION,
};

const MAX_SOURCE_BYTES: usize = 1024 * 1024;
const MAX_ARTIFACT_ITEMS: usize = 4096;
const MAX_STRING_BYTES: usize = 16 * 1024;
const MAX_EXTENSION_BYTES: usize = 64 * 1024;
const MAX_EXTENSION_DEPTH: usize = 8;
const MAX_EXTENSION_NODES: usize = 2048;

/// Parse exact YAML bytes into a typed source document without activating it.
///
/// # Examples
///
/// ```
/// use semantic_pack::{parse_pack, PackBytes};
/// let yaml = br#"
/// schema_version: 1
/// pack:
///   id: example.pack
///   version: 1.0.0
///   domain: example
///   identity_namespace: example.semantic
///   canonicalization_version: 1
///   provenance: { source: fixture.yaml, revision: v1 }
/// "#;
/// let document = parse_pack(PackBytes::new("fixture.yaml", yaml)).unwrap();
/// assert_eq!(document.pack.id.as_str(), "example.pack");
/// ```
pub fn parse_pack(source: PackBytes) -> Result<PackDocument, PackParseError> {
    if source.bytes.len() > MAX_SOURCE_BYTES {
        return Err(PackParseError::new(
            source.source_name,
            "$".to_owned(),
            None,
            None,
            "source exceeds 1 MiB admission limit".to_owned(),
        ));
    }
    let mut streams = serde_yaml::Deserializer::from_slice(&source.bytes);
    let Some(deserializer) = streams.next() else {
        return Err(PackParseError::new(
            source.source_name,
            "$".to_owned(),
            None,
            None,
            "source is empty".to_owned(),
        ));
    };
    let mut document: PackDocument =
        serde_path_to_error::deserialize(deserializer).map_err(|error| {
            let yaml_path = error.path().to_string();
            let inner = error.inner();
            let location = inner.location();
            PackParseError::new(
                source.source_name.clone(),
                if yaml_path.is_empty() {
                    "$".to_owned()
                } else {
                    format!("$.{yaml_path}")
                },
                location.as_ref().map(serde_yaml::Location::line),
                location.as_ref().map(serde_yaml::Location::column),
                inner.to_string(),
            )
        })?;
    if streams.next().is_some() {
        return Err(PackParseError::new(
            source.source_name,
            "$".to_owned(),
            None,
            None,
            "multiple YAML documents are not allowed".to_owned(),
        ));
    }
    document.source_name = source.source_name;
    document.source_hash = hex::encode(Sha256::digest(&source.bytes));
    Ok(document)
}

/// Source pack proven safe for deterministic compilation.
#[derive(Debug)]
pub struct ValidatedPack(pub(crate) PackDocument);

impl ValidatedPack {
    /// Inspect the admitted source document before compilation.
    #[must_use]
    pub fn document(&self) -> &PackDocument {
        &self.0
    }
}

struct Validator<'a> {
    document: &'a PackDocument,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Validator<'a> {
    fn push(&mut self, code: DiagnosticCode, path: impl Into<String>, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::validation(
            code,
            Some(self.document.pack.id.to_string()),
            self.document.source_name.clone(),
            path,
            message,
        ));
    }

    fn text(&mut self, path: &str, value: &str) {
        if value.trim().is_empty() {
            self.push(DiagnosticCode::InvalidProvenance, path, "must not be empty");
        } else if value.len() > MAX_STRING_BYTES {
            self.push(
                DiagnosticCode::ResourceLimit,
                path,
                "string exceeds 16 KiB limit",
            );
        } else if value.chars().any(char::is_control) {
            self.push(
                DiagnosticCode::InvalidProvenance,
                path,
                "must not contain control characters",
            );
        } else if looks_executable(value) {
            self.push(
                DiagnosticCode::ExecutableMaterial,
                path,
                "semantic text contains forbidden executable or host-language material",
            );
        }
    }
}

/// Validate schema, cross-reference, graph, semantic and resource invariants.
pub fn validate_pack(document: PackDocument) -> Result<ValidatedPack, PackValidationErrors> {
    let mut validator = Validator {
        document: &document,
        diagnostics: Vec::new(),
    };
    validate_metadata(&mut validator);
    validate_declarations(&mut validator);
    validate_capabilities(&mut validator);
    validate_policy(&mut validator);
    validate_evidence_and_governed_resources(&mut validator);
    validate_graph(&mut validator);
    validate_extensions(&mut validator);
    if validator.diagnostics.is_empty() {
        Ok(ValidatedPack(document))
    } else {
        Err(PackValidationErrors::new(validator.diagnostics))
    }
}

fn validate_metadata(validator: &mut Validator<'_>) {
    let document = validator.document;
    if document.schema_version != SUPPORTED_SCHEMA_VERSION {
        validator.push(
            DiagnosticCode::UnsupportedVersion,
            "$.schema_version",
            format!(
                "unsupported schema version {}; expected {SUPPORTED_SCHEMA_VERSION}",
                document.schema_version
            ),
        );
    }
    if document.pack.canonicalization_version != SUPPORTED_CANONICALIZATION_VERSION {
        validator.push(
            DiagnosticCode::UnsupportedVersion,
            "$.pack.canonicalization_version",
            format!(
                "unsupported canonicalization version {}; expected {SUPPORTED_CANONICALIZATION_VERSION}",
                document.pack.canonicalization_version
            ),
        );
    }
    validator.text("$.pack.provenance.source", &document.pack.provenance.source);
    validator.text(
        "$.pack.provenance.revision",
        &document.pack.provenance.revision,
    );
    let mut dependencies = BTreeSet::new();
    for (index, dependency) in document.pack.dependencies.iter().enumerate() {
        if !dependencies.insert((&dependency.id, &dependency.version)) {
            validator.push(
                DiagnosticCode::Duplicate,
                format!("$.pack.dependencies[{index}]"),
                "duplicate dependency identity",
            );
        }
        if !is_hash(&dependency.artifact_hash) {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("$.pack.dependencies[{index}].artifact_hash"),
                "dependency artifact hash must be 64 lowercase hexadecimal characters",
            );
        }
    }
}

fn validate_declarations(validator: &mut Validator<'_>) {
    let declarations = &validator.document.declarations;
    duplicates(
        validator,
        "$.declarations.domain_types",
        declarations.domain_types.iter().map(ToString::to_string),
    );
    duplicates(
        validator,
        "$.declarations.slot_kinds",
        declarations.slot_kinds.iter().map(ToString::to_string),
    );
    duplicates(
        validator,
        "$.declarations.focus_kinds",
        declarations.focus_kinds.iter().map(ToString::to_string),
    );
    for (path, value) in declarations
        .domain_types
        .iter()
        .map(|value| ("$.declarations.domain_types", value.as_str()))
        .chain(
            declarations
                .slot_kinds
                .iter()
                .map(|value| ("$.declarations.slot_kinds", value.as_str())),
        )
        .chain(
            declarations
                .focus_kinds
                .iter()
                .map(|value| ("$.declarations.focus_kinds", value.as_str())),
        )
    {
        if value.starts_with("system.") {
            validator.push(
                DiagnosticCode::InvalidIdentity,
                path,
                "application declarations may not use the reserved `system.` namespace",
            );
        }
    }
}

fn validate_capabilities(validator: &mut Validator<'_>) {
    if validator.document.capabilities.len() > MAX_ARTIFACT_ITEMS {
        validator.push(
            DiagnosticCode::ResourceLimit,
            "$.capabilities",
            "capability count exceeds 4096",
        );
    }
    duplicates(
        validator,
        "$.capabilities",
        validator
            .document
            .capabilities
            .iter()
            .map(|capability| capability.id.to_string()),
    );
    let ids: BTreeSet<_> = validator
        .document
        .capabilities
        .iter()
        .map(|capability| capability.id.clone())
        .collect();
    let declared_slots: BTreeSet<_> = validator
        .document
        .declarations
        .slot_kinds
        .iter()
        .cloned()
        .collect();
    let declared_lanes = validator
        .document
        .evidence
        .features
        .iter()
        .map(|feature| feature.lane)
        .collect::<BTreeSet<_>>();
    let rule_codes = validator
        .document
        .rule_explanations
        .iter()
        .map(|explanation| explanation.rule_code.clone())
        .collect::<BTreeSet<_>>();
    let feedback_ids = validator
        .document
        .feedback_options
        .iter()
        .map(|option| option.id.clone())
        .collect::<BTreeSet<_>>();
    let mut aliases: BTreeMap<String, String> = BTreeMap::new();
    let mut phrases: BTreeMap<String, String> = BTreeMap::new();
    for (index, capability) in validator.document.capabilities.iter().enumerate() {
        let base = format!("$.capabilities[{index}]");
        if capability.adapter_binding.as_str().trim().is_empty() {
            validator.push(
                DiagnosticCode::MissingBinding,
                format!("{base}.adapter_binding"),
                "adapter binding is required",
            );
        }
        for (field, value) in [
            ("title", capability.title.as_str()),
            ("intent_summary", capability.intent_summary.as_str()),
            ("applicability", capability.applicability.as_str()),
            ("effect", capability.effect.as_str()),
        ] {
            validator.text(&format!("{base}.{field}"), value);
        }
        duplicates(
            validator,
            &format!("{base}.arguments"),
            capability
                .arguments
                .iter()
                .map(|argument| argument.name.to_string()),
        );
        for (argument_index, argument) in capability.arguments.iter().enumerate() {
            let path = format!("{base}.arguments[{argument_index}]");
            if !declared_slots.contains(&argument.name) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("{path}.name"),
                    format!("slot kind `{}` is not declared by the pack", argument.name),
                );
            }
            validator.text(
                &format!("{path}.clarification_prompt"),
                &argument.clarification_prompt,
            );
            if argument.required && argument.default.is_some() {
                validator.push(
                    DiagnosticCode::InvalidArgument,
                    format!("{path}.default"),
                    "a required argument may not declare a default",
                );
            }
            for (constraint_index, constraint) in argument.constraints.iter().enumerate() {
                use crate::ArgumentConstraint;
                let constraint_path = format!("{path}.constraints[{constraint_index}]");
                match constraint {
                    ArgumentConstraint::AllowedValues { values } if values.is_empty() => validator
                        .push(
                            DiagnosticCode::InvalidArgument,
                            constraint_path,
                            "allowed-values constraint must contain at least one value",
                        ),
                    ArgumentConstraint::IntegerRange { minimum, maximum } if minimum > maximum => {
                        validator.push(
                            DiagnosticCode::InvalidArgument,
                            constraint_path,
                            "integer range minimum exceeds maximum",
                        )
                    }
                    ArgumentConstraint::TextLength { minimum, maximum } if minimum > maximum => {
                        validator.push(
                            DiagnosticCode::InvalidArgument,
                            constraint_path,
                            "text length minimum exceeds maximum",
                        )
                    }
                    ArgumentConstraint::Pattern { pattern } => {
                        validator.text(&constraint_path, pattern)
                    }
                    _ => {}
                }
            }
            validate_governed_requirement(
                validator,
                &path,
                argument.requirement_rule.as_ref(),
                &argument.feedback_options,
                &rule_codes,
                &feedback_ids,
            );
        }
        duplicates(
            validator,
            &format!("{base}.evidence_cues"),
            capability
                .evidence_cues
                .iter()
                .map(|cue| format!("{:?}", cue.lane)),
        );
        for (cue_index, cue) in capability.evidence_cues.iter().enumerate() {
            let path = format!("{base}.evidence_cues[{cue_index}]");
            if !declared_lanes.contains(&cue.lane) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("{path}.lane"),
                    "cue references an undeclared evidence feature",
                );
            }
            if cue.score_millis == 0 || !(-1000..=1000).contains(&cue.score_millis) {
                validator.push(
                    DiagnosticCode::InvalidEvidence,
                    format!("{path}.score_millis"),
                    "cue score must be non-zero and within [-1000, 1000]",
                );
            }
            if cue.cues.is_empty() {
                validator.push(
                    DiagnosticCode::InvalidEvidence,
                    format!("{path}.cues"),
                    "evidence cue set must not be empty",
                );
            }
            duplicates(validator, &format!("{path}.cues"), cue.cues.iter().cloned());
            for (text_index, text) in cue.cues.iter().enumerate() {
                validator.text(&format!("{path}.cues[{text_index}]"), text);
            }
        }
        validate_governed_requirement(
            validator,
            &base,
            capability.applicability_rule.as_ref(),
            &capability.feedback_options,
            &rule_codes,
            &feedback_ids,
        );
        for (alias_index, alias) in capability.aliases.iter().enumerate() {
            if ids.contains(alias) {
                validator.push(
                    DiagnosticCode::AmbiguousEvidence,
                    format!("{base}.aliases[{alias_index}]"),
                    "alias conflicts with a canonical capability ID",
                );
            }
            if let Some(previous) = aliases.insert(alias.to_string(), capability.id.to_string()) {
                if previous != capability.id.as_str() {
                    validator.push(
                        DiagnosticCode::AmbiguousEvidence,
                        format!("{base}.aliases[{alias_index}]"),
                        format!("alias is already assigned to `{previous}`"),
                    );
                }
            }
        }
        for (phrase_index, phrase) in capability.phrases.iter().enumerate() {
            let path = format!("{base}.phrases[{phrase_index}]");
            validator.text(&format!("{path}.text"), &phrase.text);
            validator.text(&format!("{path}.locale"), &phrase.locale);
            validator.text(&format!("{path}.provenance"), &phrase.provenance);
            if !matches!(
                phrase.role,
                PhraseRole::NegativeExample | PhraseRole::ArgumentCue
            ) {
                let key = format!(
                    "{}\u{0}{}",
                    phrase.locale.to_ascii_lowercase(),
                    phrase.text.trim().to_lowercase()
                );
                if let Some(previous) = phrases.insert(key, capability.id.to_string()) {
                    if previous != capability.id.as_str()
                        && validator.document.policy.phrase_ambiguity
                            == PhraseAmbiguityPolicy::Reject
                    {
                        validator.push(
                            DiagnosticCode::AmbiguousEvidence,
                            path,
                            format!("phrase is already assigned to `{previous}`"),
                        );
                    }
                }
            }
        }
        for (example_index, example) in capability.positive_examples.iter().enumerate() {
            validator.text(
                &format!("{base}.positive_examples[{example_index}]"),
                example,
            );
        }
        for (contrast_index, contrast) in capability.negative_contrasts.iter().enumerate() {
            let path = format!("{base}.negative_contrasts[{contrast_index}]");
            if !ids.contains(&contrast.candidate_id) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("{path}.candidate_id"),
                    "negative contrast references an unknown capability",
                );
            }
            validator.text(&format!("{path}.distinction"), &contrast.distinction);
        }
        if let Some(deprecation) = &capability.deprecation {
            validator.text(&format!("{base}.deprecation.message"), &deprecation.message);
            if let Some(replacement) = &deprecation.replacement {
                if replacement == &capability.id || !ids.contains(replacement) {
                    validator.push(
                        DiagnosticCode::MissingReference,
                        format!("{base}.deprecation.replacement"),
                        "replacement must reference a different known capability",
                    );
                }
            }
        }
        validate_extension_map(
            validator,
            &format!("{base}.extensions"),
            &capability.extensions,
        );
    }
}

fn validate_governed_requirement(
    validator: &mut Validator<'_>,
    base: &str,
    rule: Option<&RuleCode>,
    feedback: &[MessageKey],
    rule_codes: &BTreeSet<RuleCode>,
    feedback_ids: &BTreeSet<MessageKey>,
) {
    if rule.is_some() == feedback.is_empty() {
        validator.push(
            DiagnosticCode::InvalidRecovery,
            base,
            "a governed requirement must declare both a rule and at least one feedback option",
        );
    }
    if rule.is_some_and(|code| !rule_codes.contains(code)) {
        validator.push(
            DiagnosticCode::MissingReference,
            base,
            "governed requirement references an unknown rule",
        );
    }
    for option in feedback {
        if !feedback_ids.contains(option) {
            validator.push(
                DiagnosticCode::MissingReference,
                base,
                format!(
                    "governed requirement references unknown feedback option `{}`",
                    option.as_str()
                ),
            );
        }
    }
}

fn validate_evidence_and_governed_resources(validator: &mut Validator<'_>) {
    let evidence = &validator.document.evidence;
    if !evidence.is_empty() && evidence.version != 1 {
        validator.push(
            DiagnosticCode::UnsupportedVersion,
            "$.evidence.version",
            "unsupported evidence policy version; expected 1",
        );
    }
    if evidence.features.len() > EvidenceLane::ALL.len() {
        validator.push(
            DiagnosticCode::ResourceLimit,
            "$.evidence.features",
            "evidence feature count exceeds the closed lane vocabulary",
        );
    }
    duplicates(
        validator,
        "$.evidence.features",
        evidence
            .features
            .iter()
            .map(|feature| format!("{:?}", feature.lane)),
    );
    let lanes = evidence
        .features
        .iter()
        .map(|feature| feature.lane)
        .collect::<BTreeSet<_>>();
    for (index, feature) in evidence.features.iter().enumerate() {
        if !(1..=100_000).contains(&feature.weight_millis) {
            validator.push(
                DiagnosticCode::InvalidEvidence,
                format!("$.evidence.features[{index}].weight_millis"),
                "fusion weight must be within [1, 100000]",
            );
        }
    }

    let capability_ids = validator
        .document
        .capabilities
        .iter()
        .map(|capability| capability.id.clone())
        .collect::<BTreeSet<_>>();
    let mut gates = BTreeMap::new();
    for (index, gate) in evidence.deterministic_gates.iter().enumerate() {
        let path = format!("$.evidence.deterministic_gates[{index}]");
        if !capability_ids.contains(&gate.candidate_id) {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("{path}.candidate_id"),
                "deterministic gate references an unknown capability",
            );
        }
        if !lanes.contains(&gate.lane) {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("{path}.lane"),
                "deterministic gate references an undeclared evidence feature",
            );
        }
        let key = (gate.candidate_id.clone(), gate.lane);
        if let Some(previous) = gates.insert(key, gate.effect) {
            let message = if previous == gate.effect {
                "duplicate deterministic gate"
            } else {
                "contradictory deterministic gates"
            };
            validator.push(DiagnosticCode::InvalidEvidence, path, message);
        }
    }

    duplicates(
        validator,
        "$.rule_explanations",
        validator
            .document
            .rule_explanations
            .iter()
            .map(|explanation| explanation.rule_code.as_str().to_owned()),
    );
    let rule_codes = validator
        .document
        .rule_explanations
        .iter()
        .map(|explanation| explanation.rule_code.clone())
        .collect::<BTreeSet<_>>();
    let feedback_ids = validator
        .document
        .feedback_options
        .iter()
        .map(|option| option.id.clone())
        .collect::<BTreeSet<_>>();
    for (index, explanation) in validator.document.rule_explanations.iter().enumerate() {
        let base = format!("$.rule_explanations[{index}]");
        validator.text(&format!("{base}.message"), &explanation.message);
        duplicates(
            validator,
            &format!("{base}.feedback_options"),
            explanation
                .feedback_options
                .iter()
                .map(|option| option.as_str().to_owned()),
        );
        for option in &explanation.feedback_options {
            if !feedback_ids.contains(option) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("{base}.feedback_options"),
                    format!(
                        "rule references unknown feedback option `{}`",
                        option.as_str()
                    ),
                );
            }
        }
    }

    duplicates(
        validator,
        "$.feedback_options",
        validator
            .document
            .feedback_options
            .iter()
            .map(|option| option.id.as_str().to_owned()),
    );
    let mut adjacency = BTreeMap::<MessageKey, Vec<MessageKey>>::new();
    for (index, option) in validator.document.feedback_options.iter().enumerate() {
        let base = format!("$.feedback_options[{index}]");
        validator.text(&format!("{base}.prompt"), &option.prompt);
        if !rule_codes.contains(&option.rule_code) {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("{base}.rule_code"),
                "feedback option references an unknown rule",
            );
        }
        if option
            .candidate_id
            .as_ref()
            .is_some_and(|candidate| !capability_ids.contains(candidate))
        {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("{base}.candidate_id"),
                "feedback option references an unknown capability",
            );
        }
        duplicates(
            validator,
            &format!("{base}.next_options"),
            option
                .next_options
                .iter()
                .map(|next| next.as_str().to_owned()),
        );
        for next in &option.next_options {
            if !feedback_ids.contains(next) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("{base}.next_options"),
                    format!(
                        "feedback option references unknown successor `{}`",
                        next.as_str()
                    ),
                );
            }
        }
        adjacency.insert(option.id.clone(), option.next_options.clone());
    }
    if contains_feedback_cycle(&adjacency) {
        validator.push(
            DiagnosticCode::InvalidRecovery,
            "$.feedback_options",
            "feedback option links must be acyclic",
        );
    }
}

fn contains_feedback_cycle(adjacency: &BTreeMap<MessageKey, Vec<MessageKey>>) -> bool {
    fn visit<'a>(
        node: &'a MessageKey,
        adjacency: &'a BTreeMap<MessageKey, Vec<MessageKey>>,
        visiting: &mut BTreeSet<&'a MessageKey>,
        complete: &mut BTreeSet<&'a MessageKey>,
    ) -> bool {
        if complete.contains(node) {
            return false;
        }
        if !visiting.insert(node) {
            return true;
        }
        if adjacency.get(node).is_some_and(|next| {
            next.iter()
                .any(|child| visit(child, adjacency, visiting, complete))
        }) {
            return true;
        }
        visiting.remove(node);
        complete.insert(node);
        false
    }

    let mut visiting = BTreeSet::new();
    let mut complete = BTreeSet::new();
    adjacency
        .keys()
        .any(|node| visit(node, adjacency, &mut visiting, &mut complete))
}

fn validate_policy(validator: &mut Validator<'_>) {
    let capability_ids: BTreeSet<_> = validator
        .document
        .capabilities
        .iter()
        .map(|capability| capability.id.clone())
        .collect();
    duplicates(
        validator,
        "$.policy.roles",
        validator
            .document
            .policy
            .roles
            .iter()
            .map(|grant| grant.role.to_string()),
    );
    for (index, grant) in validator.document.policy.roles.iter().enumerate() {
        duplicates(
            validator,
            &format!("$.policy.roles[{index}].capabilities"),
            grant.capabilities.iter().map(ToString::to_string),
        );
        for capability in &grant.capabilities {
            if !capability_ids.contains(capability) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("$.policy.roles[{index}].capabilities"),
                    format!("grant references unknown capability `{capability}`"),
                );
            }
        }
    }

    duplicates(
        validator,
        "$.policy.eligibility",
        validator
            .document
            .policy
            .eligibility
            .iter()
            .map(|policy| policy.context.to_string()),
    );
    if validator.document.policy.eligibility.len() > MAX_ARTIFACT_ITEMS {
        validator.push(
            DiagnosticCode::ResourceLimit,
            "$.policy.eligibility",
            "eligibility context count exceeds 4096",
        );
    }
    for (index, policy) in validator.document.policy.eligibility.iter().enumerate() {
        let base = format!("$.policy.eligibility[{index}]");
        let allow = policy
            .allow
            .iter()
            .map(selector_key)
            .collect::<BTreeSet<_>>();
        let deny = policy
            .deny
            .iter()
            .map(selector_key)
            .collect::<BTreeSet<_>>();
        duplicates(
            validator,
            &format!("{base}.allow"),
            policy.allow.iter().map(selector_key),
        );
        duplicates(
            validator,
            &format!("{base}.deny"),
            policy.deny.iter().map(selector_key),
        );
        duplicates(
            validator,
            &format!("{base}.attributes"),
            policy.attributes.iter().map(ToString::to_string),
        );
        if let Some(selector) = allow.intersection(&deny).next() {
            validator.push(
                DiagnosticCode::InvalidPolicy,
                base,
                format!("selector `{selector}` appears in both allow and deny lists"),
            );
        }
    }

    duplicates(
        validator,
        "$.policy.privileges",
        validator
            .document
            .policy
            .privileges
            .iter()
            .map(|grant| grant.privilege.to_string()),
    );
    for (index, grant) in validator.document.policy.privileges.iter().enumerate() {
        duplicates(
            validator,
            &format!("$.policy.privileges[{index}].roles"),
            grant.roles.iter().map(role_selector_key),
        );
        if grant.roles.is_empty() {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("$.policy.privileges[{index}].roles"),
                "privilege must declare at least one role selector",
            );
        }
    }
}

fn selector_key(selector: &crate::CapabilitySelectorSource) -> String {
    serde_json::to_string(selector).expect("typed capability selector serializes")
}

fn role_selector_key(selector: &crate::RoleSelectorSource) -> String {
    serde_json::to_string(selector).expect("typed role selector serializes")
}

fn validate_graph(validator: &mut Validator<'_>) {
    let Some(graph) = &validator.document.graph else {
        return;
    };
    duplicates(
        validator,
        "$.graph.nodes",
        graph.nodes.iter().map(|node| node.id.to_string()),
    );
    duplicates(
        validator,
        "$.graph.entry_nodes",
        graph.entry_nodes.iter().map(ToString::to_string),
    );
    let nodes: BTreeMap<_, _> = graph.nodes.iter().map(|node| (&node.id, node)).collect();
    let capabilities: BTreeSet<_> = validator
        .document
        .capabilities
        .iter()
        .map(|capability| &capability.id)
        .collect();
    for (index, entry) in graph.entry_nodes.iter().enumerate() {
        if !nodes.contains_key(entry) {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("$.graph.entry_nodes[{index}]"),
                "entry references an unknown graph node",
            );
        }
    }
    let mut adjacency: BTreeMap<&GraphNodeId, Vec<&GraphNodeId>> = graph
        .nodes
        .iter()
        .map(|node| (&node.id, Vec::new()))
        .collect();
    for (index, edge) in graph.edges.iter().enumerate() {
        if !nodes.contains_key(&edge.from) || !nodes.contains_key(&edge.to) {
            validator.push(
                DiagnosticCode::MissingReference,
                format!("$.graph.edges[{index}]"),
                "edge references an unknown graph node",
            );
        } else {
            adjacency.entry(&edge.from).or_default().push(&edge.to);
        }
        validator.text(&format!("$.graph.edges[{index}].when"), &edge.when);
    }
    for (index, node) in graph.nodes.iter().enumerate() {
        for capability in &node.candidates {
            if !capabilities.contains(capability) {
                validator.push(
                    DiagnosticCode::MissingReference,
                    format!("$.graph.nodes[{index}].candidates"),
                    format!("node references unknown capability `{capability}`"),
                );
            }
        }
        if node.terminal.is_some()
            && adjacency
                .get(&node.id)
                .is_some_and(|successors| !successors.is_empty())
        {
            validator.push(
                DiagnosticCode::InvalidGraph,
                format!("$.graph.nodes[{index}].terminal"),
                "terminal graph node may not have outgoing edges",
            );
        }
    }
    let reachable = reachable_nodes(&graph.entry_nodes, &adjacency);
    for (index, node) in graph.nodes.iter().enumerate() {
        if node.required && !reachable.contains(&node.id) {
            validator.push(
                DiagnosticCode::InvalidGraph,
                format!("$.graph.nodes[{index}]"),
                "required graph node is unreachable from every entry",
            );
        }
    }
    if graph.acyclic && contains_cycle(&adjacency) {
        validator.push(
            DiagnosticCode::InvalidGraph,
            "$.graph",
            "graph declares `acyclic: true` but contains a cycle",
        );
    }
}

fn reachable_nodes<'a>(
    entries: &'a [GraphNodeId],
    adjacency: &BTreeMap<&'a GraphNodeId, Vec<&'a GraphNodeId>>,
) -> BTreeSet<&'a GraphNodeId> {
    let mut reached = BTreeSet::new();
    let mut pending: Vec<_> = entries.iter().collect();
    while let Some(node) = pending.pop() {
        if reached.insert(node) {
            pending.extend(adjacency.get(node).into_iter().flatten().copied());
        }
    }
    reached
}

fn contains_cycle(adjacency: &BTreeMap<&GraphNodeId, Vec<&GraphNodeId>>) -> bool {
    fn visit<'a>(
        node: &'a GraphNodeId,
        adjacency: &BTreeMap<&'a GraphNodeId, Vec<&'a GraphNodeId>>,
        visiting: &mut BTreeSet<&'a GraphNodeId>,
        complete: &mut BTreeSet<&'a GraphNodeId>,
    ) -> bool {
        if complete.contains(node) {
            return false;
        }
        if !visiting.insert(node) {
            return true;
        }
        if adjacency.get(node).is_some_and(|next| {
            next.iter()
                .any(|child| visit(child, adjacency, visiting, complete))
        }) {
            return true;
        }
        visiting.remove(node);
        complete.insert(node);
        false
    }

    let mut visiting = BTreeSet::new();
    let mut complete = BTreeSet::new();
    adjacency
        .keys()
        .any(|node| visit(node, adjacency, &mut visiting, &mut complete))
}

fn validate_extensions(validator: &mut Validator<'_>) {
    validate_extension_map(validator, "$.extensions", &validator.document.extensions);
}

fn validate_extension_map(
    validator: &mut Validator<'_>,
    path: &str,
    extensions: &BTreeMap<String, ConfigValue>,
) {
    for key in extensions.keys() {
        if !key.contains('.') || key.starts_with('.') || key.ends_with('.') {
            validator.push(
                DiagnosticCode::InvalidExtension,
                format!("{path}.{key}"),
                "extension key must be namespace-qualified",
            );
        }
    }
    let mut nodes = 0;
    for (key, value) in extensions {
        validate_config_value(validator, &format!("{path}.{key}"), value, 1, &mut nodes);
    }
    if nodes > MAX_EXTENSION_NODES {
        validator.push(
            DiagnosticCode::ResourceLimit,
            path,
            "extension node count exceeds 2048",
        );
    }
    if serde_json::to_vec(extensions).is_ok_and(|bytes| bytes.len() > MAX_EXTENSION_BYTES) {
        validator.push(
            DiagnosticCode::ResourceLimit,
            path,
            "canonical extension data exceeds 64 KiB",
        );
    }
}

fn validate_config_value(
    validator: &mut Validator<'_>,
    path: &str,
    value: &ConfigValue,
    depth: usize,
    nodes: &mut usize,
) {
    *nodes += 1;
    if depth > MAX_EXTENSION_DEPTH {
        validator.push(
            DiagnosticCode::ResourceLimit,
            path,
            "extension nesting exceeds 8 levels",
        );
        return;
    }
    match value {
        ConfigValue::Text(text) => validator.text(path, text),
        ConfigValue::List(values) => {
            for (index, value) in values.iter().enumerate() {
                validate_config_value(
                    validator,
                    &format!("{path}[{index}]"),
                    value,
                    depth + 1,
                    nodes,
                );
            }
        }
        ConfigValue::Map(values) => {
            for (key, value) in values {
                validator.text(path, key);
                validate_config_value(validator, &format!("{path}.{key}"), value, depth + 1, nodes);
            }
        }
        ConfigValue::Null | ConfigValue::Boolean(_) | ConfigValue::Integer(_) => {}
    }
}

fn duplicates(validator: &mut Validator<'_>, path: &str, values: impl IntoIterator<Item = String>) {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            validator.push(
                DiagnosticCode::Duplicate,
                path,
                format!("duplicate value `{value}`"),
            );
        }
    }
}

fn is_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn looks_executable(value: &str) -> bool {
    let normalized = value.trim().to_ascii_lowercase();
    value.contains("::")
        || value.contains("#!/")
        || value.contains("```rust")
        || normalized.starts_with("fn ")
        || normalized.starts_with("bash -c")
        || (normalized.starts_with("select ") && normalized.contains(" from "))
        || normalized.starts_with("insert into ")
        || (normalized.starts_with("update ") && normalized.contains(" set "))
        || normalized.starts_with("delete from ")
}

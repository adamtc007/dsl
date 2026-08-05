use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::{
    ArtifactHash, CompiledPack, PackAdmissionError, PackCompileError, PackDocument, PackReceipt,
    SourceHash, ValidatedPack, COMPILER_VERSION,
};

const MAX_ARTIFACT_BYTES: usize = 2 * 1024 * 1024;

/// Deterministically normalize and compile a validated source pack.
pub fn compile_pack(pack: ValidatedPack) -> Result<CompiledPack, PackCompileError> {
    let mut document = pack.0;
    normalize(&mut document)?;
    let canonical_bytes = serde_json::to_vec(&document)
        .map_err(|error| PackCompileError::CanonicalSerialization(error.to_string()))?;
    if canonical_bytes.len() > MAX_ARTIFACT_BYTES {
        return Err(PackCompileError::ArtifactTooLarge);
    }
    let mut hasher = Sha256::new();
    hasher.update(b"semantic-pack-canonical-v1\0");
    hasher.update(&canonical_bytes);
    let artifact_hash = ArtifactHash(hex::encode(hasher.finalize()));
    let mut adapter_bindings: Vec<_> = document
        .capabilities
        .iter()
        .map(|capability| capability.adapter_binding.to_string())
        .collect();
    adapter_bindings.sort();
    adapter_bindings.dedup();
    let receipt = PackReceipt {
        identity: crate::PackIdentity::new(document.pack.id.clone(), document.pack.version.clone()),
        schema_version: document.schema_version,
        canonicalization_version: document.pack.canonicalization_version,
        compiler_version: COMPILER_VERSION.to_owned(),
        source_name: document.source_name.clone(),
        source_hash: SourceHash(document.source_hash.clone()),
        dependencies: document.pack.dependencies.clone(),
        adapter_bindings,
        artifact_hash,
    };
    Ok(CompiledPack {
        document: Arc::new(document),
        canonical_bytes: Arc::from(canonical_bytes),
        receipt: Arc::new(receipt),
    })
}

/// Parse, validate and compile source bytes without installing them.
pub fn admit_pack(source: crate::PackBytes) -> Result<CompiledPack, PackAdmissionError> {
    let document = crate::parse_pack(source)?;
    let validated = crate::validate_pack(document)?;
    Ok(compile_pack(validated)?)
}

fn normalize(document: &mut PackDocument) -> Result<(), PackCompileError> {
    document.pack.dependencies.sort();
    document.pack.dependencies.dedup();
    document.declarations.domain_types.sort();
    document.declarations.domain_types.dedup();
    document.declarations.slot_kinds.sort();
    document.declarations.slot_kinds.dedup();
    document.declarations.focus_kinds.sort();
    document.declarations.focus_kinds.dedup();
    for capability in &mut document.capabilities {
        capability
            .arguments
            .sort_by(|left, right| left.name.cmp(&right.name));
        for argument in &mut capability.arguments {
            sort_by_json(&mut argument.constraints)?;
        }
        capability.phrases.sort();
        capability.phrases.dedup();
        capability.positive_examples.sort();
        capability.positive_examples.dedup();
        capability.negative_contrasts.sort();
        capability.negative_contrasts.dedup();
        capability.aliases.sort();
        capability.aliases.dedup();
    }
    document
        .capabilities
        .sort_by(|left, right| left.id.cmp(&right.id));
    document
        .policy
        .roles
        .sort_by(|left, right| left.role.cmp(&right.role));
    for role in &mut document.policy.roles {
        role.capabilities.sort();
        role.capabilities.dedup();
    }
    document
        .policy
        .eligibility
        .sort_by(|left, right| left.context.cmp(&right.context));
    for policy in &mut document.policy.eligibility {
        policy.allow.sort();
        policy.allow.dedup();
        policy.deny.sort();
        policy.deny.dedup();
        policy.attributes.sort();
        policy.attributes.dedup();
    }
    document
        .policy
        .privileges
        .sort_by(|left, right| left.privilege.cmp(&right.privilege));
    for privilege in &mut document.policy.privileges {
        privilege.roles.sort();
        privilege.roles.dedup();
    }
    if let Some(graph) = &mut document.graph {
        graph.entry_nodes.sort();
        graph.entry_nodes.dedup();
        graph.nodes.sort_by(|left, right| left.id.cmp(&right.id));
        for node in &mut graph.nodes {
            node.candidates.sort();
            node.candidates.dedup();
        }
        graph.edges.sort_by(|left, right| {
            (&left.from, &left.to, &left.when).cmp(&(&right.from, &right.to, &right.when))
        });
        graph.edges.dedup();
    }
    Ok(())
}

fn sort_by_json<T: serde::Serialize>(values: &mut [T]) -> Result<(), PackCompileError> {
    let mut keyed = values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            serde_json::to_string(value)
                .map(|key| (key, index))
                .map_err(|error| PackCompileError::CanonicalSerialization(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    keyed.sort_by(|left, right| left.0.cmp(&right.0));
    let mut positions = vec![0; values.len()];
    for (new_index, (_, old_index)) in keyed.into_iter().enumerate() {
        positions[old_index] = new_index;
    }
    for old_index in 0..values.len() {
        let mut current = old_index;
        while positions[current] != current {
            let next = positions[current];
            values.swap(current, next);
            positions.swap(current, next);
            current = next;
        }
    }
    Ok(())
}

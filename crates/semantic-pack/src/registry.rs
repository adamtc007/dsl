use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use crate::{ArtifactHash, CompiledPack, PackId, PackIdentity, RegistryError, SemanticSnapshot};

/// Immutable semantic pack installation and exact-resolution port.
pub trait PackRegistry {
    /// Install and atomically activate a pack when its logical identity is new.
    fn install(&self, pack: CompiledPack) -> Result<SemanticSnapshot, RegistryError>;

    /// Activate an installed exact identity, optionally fencing on the active hash.
    fn activate(
        &self,
        identity: &PackIdentity,
        expected_current: Option<&ArtifactHash>,
    ) -> Result<SemanticSnapshot, RegistryError>;

    /// Resolve an exact logical identity, including immutable prior versions.
    fn resolve(&self, identity: &PackIdentity) -> Result<SemanticSnapshot, RegistryError>;

    /// Resolve an exact content hash.
    fn resolve_hash(&self, hash: &ArtifactHash) -> Result<SemanticSnapshot, RegistryError>;

    /// Resolve the currently active version of a pack ID.
    fn resolve_active(&self, id: &PackId) -> Result<SemanticSnapshot, RegistryError>;
}

#[derive(Default)]
struct RegistryState {
    by_hash: BTreeMap<ArtifactHash, Arc<CompiledPack>>,
    by_identity: BTreeMap<PackIdentity, ArtifactHash>,
    active: BTreeMap<PackId, ArtifactHash>,
}

/// Thread-safe in-memory registry retaining every immutable installed version.
#[derive(Default)]
pub struct InMemoryPackRegistry {
    state: RwLock<RegistryState>,
}

impl InMemoryPackRegistry {
    /// Construct an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PackRegistry for InMemoryPackRegistry {
    fn install(&self, pack: CompiledPack) -> Result<SemanticSnapshot, RegistryError> {
        let identity = pack.identity();
        let hash = pack.receipt().artifact_hash.clone();
        let mut state = self.state.write().map_err(|_| RegistryError::Poisoned)?;
        if let Some(existing) = state.by_identity.get(&identity).cloned() {
            if existing != hash {
                return Err(RegistryError::IdentityConflict(format!(
                    "{}@{}",
                    identity.id, identity.version
                )));
            }
            let installed = state
                .by_hash
                .get(&existing)
                .cloned()
                .ok_or(RegistryError::Poisoned)?;
            state.active.insert(identity.id, existing);
            return Ok(SemanticSnapshot::new(installed));
        }
        let pack = Arc::new(pack);
        state.by_hash.insert(hash.clone(), Arc::clone(&pack));
        state.by_identity.insert(identity.clone(), hash.clone());
        state.active.insert(identity.id, hash);
        Ok(SemanticSnapshot::new(pack))
    }

    fn activate(
        &self,
        identity: &PackIdentity,
        expected_current: Option<&ArtifactHash>,
    ) -> Result<SemanticSnapshot, RegistryError> {
        let mut state = self.state.write().map_err(|_| RegistryError::Poisoned)?;
        let hash = state.by_identity.get(identity).cloned().ok_or_else(|| {
            RegistryError::NotFound(format!("{}@{}", identity.id, identity.version))
        })?;
        if let Some(expected) = expected_current {
            let actual = state
                .active
                .get(&identity.id)
                .ok_or_else(|| RegistryError::NotFound(identity.id.to_string()))?;
            if actual != expected {
                return Err(RegistryError::StaleActivation {
                    expected: expected.as_str().to_owned(),
                    actual: actual.as_str().to_owned(),
                });
            }
        }
        let pack = state
            .by_hash
            .get(&hash)
            .cloned()
            .ok_or(RegistryError::Poisoned)?;
        state.active.insert(identity.id.clone(), hash);
        Ok(SemanticSnapshot::new(pack))
    }

    fn resolve(&self, identity: &PackIdentity) -> Result<SemanticSnapshot, RegistryError> {
        let state = self.state.read().map_err(|_| RegistryError::Poisoned)?;
        let hash = state.by_identity.get(identity).ok_or_else(|| {
            RegistryError::NotFound(format!("{}@{}", identity.id, identity.version))
        })?;
        let pack = state
            .by_hash
            .get(hash)
            .cloned()
            .ok_or(RegistryError::Poisoned)?;
        Ok(SemanticSnapshot::new(pack))
    }

    fn resolve_hash(&self, hash: &ArtifactHash) -> Result<SemanticSnapshot, RegistryError> {
        let state = self.state.read().map_err(|_| RegistryError::Poisoned)?;
        let pack = state
            .by_hash
            .get(hash)
            .cloned()
            .ok_or_else(|| RegistryError::HashNotFound(hash.as_str().to_owned()))?;
        Ok(SemanticSnapshot::new(pack))
    }

    fn resolve_active(&self, id: &PackId) -> Result<SemanticSnapshot, RegistryError> {
        let state = self.state.read().map_err(|_| RegistryError::Poisoned)?;
        let hash = state
            .active
            .get(id)
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))?;
        let pack = state
            .by_hash
            .get(hash)
            .cloned()
            .ok_or(RegistryError::Poisoned)?;
        Ok(SemanticSnapshot::new(pack))
    }
}

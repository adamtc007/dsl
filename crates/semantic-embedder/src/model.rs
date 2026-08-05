use std::path::{Path, PathBuf};

use crate::{EmbeddingError, ModelArtifact};

/// Default BGE model repository used by the compatibility constructors.
pub const DEFAULT_MODEL_REPOSITORY: &str = "BAAI/bge-small-en-v1.5";

/// Immutable upstream revision used by the compatibility constructors.
pub const DEFAULT_MODEL_REVISION: &str = "5c38ec7c405ec4b44b94cc5a9bb96e735b38267a";

/// Output dimension of the default BGE-small-en-v1.5 model.
pub const DEFAULT_EMBEDDING_DIMENSION: usize = 384;

/// Maximum token sequence accepted by the compatibility BERT model.
pub const MAX_SEQUENCE_LENGTH: usize = 512;

/// Retrieval instruction applied to queries and never to targets.
pub const QUERY_PREFIX: &str = "Represent this sentence for searching relevant passages: ";

/// Stable identity of an embedding model and its output shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelIdentity {
    repository: String,
    revision: String,
    dimension: usize,
}

impl ModelIdentity {
    /// Construct and validate a model identity.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_embedder::ModelIdentity;
    /// let identity = ModelIdentity::new("models/example", "abc123", 16)?;
    /// assert_eq!(identity.dimension(), 16);
    /// # Ok::<(), semantic_embedder::EmbeddingError>(())
    /// ```
    pub fn new(
        repository: impl Into<String>,
        revision: impl Into<String>,
        dimension: usize,
    ) -> Result<Self, EmbeddingError> {
        let repository = repository.into();
        if repository.trim().is_empty() {
            return Err(EmbeddingError::InvalidModelIdentity {
                field: "repository",
                reason: "must not be empty",
            });
        }
        let revision = revision.into();
        if revision.trim().is_empty() {
            return Err(EmbeddingError::InvalidModelIdentity {
                field: "revision",
                reason: "must not be empty",
            });
        }
        if dimension == 0 {
            return Err(EmbeddingError::InvalidDimension { dimension });
        }
        Ok(Self {
            repository,
            revision,
            dimension,
        })
    }

    /// Return the immutable default BGE model identity.
    #[must_use]
    pub fn default_bge() -> Self {
        Self {
            repository: DEFAULT_MODEL_REPOSITORY.to_string(),
            revision: DEFAULT_MODEL_REVISION.to_string(),
            dimension: DEFAULT_EMBEDDING_DIMENSION,
        }
    }

    /// Model repository or caller-defined local identity.
    #[must_use]
    pub fn repository(&self) -> &str {
        &self.repository
    }

    /// Immutable model revision or caller-defined bundle revision.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Declared output vector dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Paths and identity for a caller-provided local model bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelBundle {
    identity: ModelIdentity,
    config: PathBuf,
    tokenizer: PathBuf,
    weights: PathBuf,
}

impl ModelBundle {
    /// Resolve the conventional three artifact names below a directory.
    ///
    /// This constructor performs no filesystem access; call [`Self::validate`]
    /// or load the bundle through an inference implementation to validate it.
    #[must_use]
    pub fn from_directory(root: impl AsRef<Path>, identity: ModelIdentity) -> Self {
        let root = root.as_ref();
        Self {
            identity,
            config: root.join("config.json"),
            tokenizer: root.join("tokenizer.json"),
            weights: root.join("model.safetensors"),
        }
    }

    /// Construct a bundle from explicit artifact paths.
    #[must_use]
    pub fn from_paths(
        identity: ModelIdentity,
        config: impl Into<PathBuf>,
        tokenizer: impl Into<PathBuf>,
        weights: impl Into<PathBuf>,
    ) -> Self {
        Self {
            identity,
            config: config.into(),
            tokenizer: tokenizer.into(),
            weights: weights.into(),
        }
    }

    /// Require all three artifacts to exist as files.
    pub fn validate(&self) -> Result<(), EmbeddingError> {
        for (artifact, path) in [
            (ModelArtifact::Config, &self.config),
            (ModelArtifact::Tokenizer, &self.tokenizer),
            (ModelArtifact::Weights, &self.weights),
        ] {
            if !path.is_file() {
                return Err(EmbeddingError::MissingArtifact {
                    artifact,
                    path: path.clone(),
                });
            }
        }
        Ok(())
    }

    /// Model identity carried by the bundle.
    #[must_use]
    pub const fn identity(&self) -> &ModelIdentity {
        &self.identity
    }

    /// BERT configuration path.
    #[must_use]
    pub fn config_path(&self) -> &Path {
        &self.config
    }

    /// Tokenizer JSON path.
    #[must_use]
    pub fn tokenizer_path(&self) -> &Path {
        &self.tokenizer
    }

    /// Safetensors weights path.
    #[must_use]
    pub fn weights_path(&self) -> &Path {
        &self.weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_typed_and_validated() {
        assert!(matches!(
            ModelIdentity::new("", "rev", 1),
            Err(EmbeddingError::InvalidModelIdentity {
                field: "repository",
                ..
            })
        ));
        assert!(matches!(
            ModelIdentity::new("repo", "rev", 0),
            Err(EmbeddingError::InvalidDimension { dimension: 0 })
        ));
    }

    #[test]
    fn missing_bundle_artifact_has_a_typed_failure() {
        let root =
            std::env::temp_dir().join(format!("semantic-embedder-missing-{}", std::process::id()));
        let bundle = ModelBundle::from_directory(root, ModelIdentity::default_bge());
        assert!(matches!(
            bundle.validate(),
            Err(EmbeddingError::MissingArtifact {
                artifact: ModelArtifact::Config,
                ..
            })
        ));
    }
}

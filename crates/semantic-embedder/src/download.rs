use std::path::Path;

use hf_hub::{api::sync::Api, Repo, RepoType};

use crate::{
    CandleEmbedder, EmbeddingError, ModelBundle, ModelIdentity, DEFAULT_EMBEDDING_DIMENSION,
    DEFAULT_MODEL_REPOSITORY, DEFAULT_MODEL_REVISION,
};

impl CandleEmbedder {
    /// Load the pinned default model, resolving it through Hugging Face Hub.
    pub fn new() -> Result<Self, EmbeddingError> {
        Self::with_model_and_revision(DEFAULT_MODEL_REPOSITORY, DEFAULT_MODEL_REVISION)
    }

    /// Load a local directory or a named Hugging Face model at the pinned default revision.
    pub fn with_model(model: &str) -> Result<Self, EmbeddingError> {
        Self::with_model_and_revision(model, DEFAULT_MODEL_REVISION)
    }

    /// Load a local directory or resolve an exact Hugging Face model revision.
    pub fn with_model_and_revision(model: &str, revision: &str) -> Result<Self, EmbeddingError> {
        let identity = ModelIdentity::new(model, revision, DEFAULT_EMBEDDING_DIMENSION)?;
        if Path::new(model).is_dir() {
            return Self::from_directory(model, identity);
        }
        let api = Api::new().map_err(|error| EmbeddingError::Download {
            artifact: "hub client",
            message: error.to_string(),
        })?;
        let repository = api.repo(Repo::with_revision(
            model.to_string(),
            RepoType::Model,
            revision.to_string(),
        ));
        let config = repository
            .get("config.json")
            .map_err(|error| EmbeddingError::Download {
                artifact: "config.json",
                message: error.to_string(),
            })?;
        let tokenizer =
            repository
                .get("tokenizer.json")
                .map_err(|error| EmbeddingError::Download {
                    artifact: "tokenizer.json",
                    message: error.to_string(),
                })?;
        let weights =
            repository
                .get("model.safetensors")
                .map_err(|error| EmbeddingError::Download {
                    artifact: "model.safetensors",
                    message: error.to_string(),
                })?;
        Self::from_bundle(&ModelBundle::from_paths(
            identity, config, tokenizer, weights,
        ))
    }
}

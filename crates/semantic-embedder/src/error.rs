use std::path::PathBuf;

use thiserror::Error;

/// The three files required by the supported local model bundle format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelArtifact {
    /// Candle BERT configuration JSON.
    Config,
    /// Hugging Face tokenizer JSON.
    Tokenizer,
    /// Safetensors model weights.
    Weights,
}

impl std::fmt::Display for ModelArtifact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Config => "config",
            Self::Tokenizer => "tokenizer",
            Self::Weights => "weights",
        })
    }
}

/// Typed failures produced by bundle validation, model loading, or inference.
#[derive(Debug, Error)]
pub enum EmbeddingError {
    /// A required model identity field was empty or otherwise invalid.
    #[error("invalid model identity field {field}: {reason}")]
    InvalidModelIdentity {
        /// Name of the invalid field.
        field: &'static str,
        /// Stable human-readable validation reason.
        reason: &'static str,
    },
    /// The requested embedding dimension cannot be represented.
    #[error("invalid embedding dimension {dimension}; it must be greater than zero")]
    InvalidDimension {
        /// Invalid dimension supplied by the caller or bundle.
        dimension: usize,
    },
    /// A required bundle artifact is not a readable file.
    #[error("missing {artifact} model artifact: {path}")]
    MissingArtifact {
        /// Kind of artifact that was absent.
        artifact: ModelArtifact,
        /// Expected artifact path.
        path: PathBuf,
    },
    /// A bundle artifact could not be read.
    #[error("failed to read {artifact} model artifact {path}: {source}")]
    ReadArtifact {
        /// Kind of artifact that could not be read.
        artifact: ModelArtifact,
        /// Artifact path.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
    /// The BERT configuration was not valid for this runtime.
    #[error("invalid model config {path}: {message}")]
    InvalidConfig {
        /// Configuration path.
        path: PathBuf,
        /// Parser or validation detail.
        message: String,
    },
    /// The tokenizer file was malformed or incompatible.
    #[error("invalid tokenizer {path}: {message}")]
    InvalidTokenizer {
        /// Tokenizer path.
        path: PathBuf,
        /// Parser or compatibility detail.
        message: String,
    },
    /// The safetensors data was malformed or incompatible.
    #[error("invalid model weights {path}: {message}")]
    InvalidWeights {
        /// Weights path.
        path: PathBuf,
        /// Loader detail.
        message: String,
    },
    /// The loaded model dimension does not match its declared identity.
    #[error("model dimension mismatch: identity declares {expected}, config declares {actual}")]
    IncompatibleDimension {
        /// Dimension declared by the model identity.
        expected: usize,
        /// Hidden size declared by the BERT configuration.
        actual: usize,
    },
    /// Candle could not construct the model.
    #[error("failed to construct embedding model: {message}")]
    Model {
        /// Candle model construction detail.
        message: String,
    },
    /// Tokenization or model inference failed.
    #[error("embedding inference failed: {message}")]
    Inference {
        /// Tokenization or Candle inference detail.
        message: String,
    },
    /// An explicitly requested remote model artifact could not be resolved.
    #[error("failed to resolve remote {artifact} artifact: {message}")]
    Download {
        /// Remote artifact name.
        artifact: &'static str,
        /// Hub client or transport detail.
        message: String,
    },
}

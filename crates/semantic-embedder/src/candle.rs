use std::fs;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use tokenizers::Tokenizer;
use tracing::info;

use crate::{
    Embedder, EmbeddingError, ModelArtifact, ModelBundle, ModelIdentity, MAX_SEQUENCE_LENGTH,
    QUERY_PREFIX,
};

/// Candle CPU implementation of the host-neutral embedding contract.
pub struct CandleEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    identity: ModelIdentity,
}

impl CandleEmbedder {
    /// Load a local model bundle without network access.
    pub fn from_bundle(bundle: &ModelBundle) -> Result<Self, EmbeddingError> {
        bundle.validate()?;
        let start = std::time::Instant::now();
        let device = Device::Cpu;

        let config_bytes = read_artifact(ModelArtifact::Config, bundle.config_path())?;
        let config: Config = serde_json::from_slice(&config_bytes).map_err(|error| {
            EmbeddingError::InvalidConfig {
                path: bundle.config_path().to_path_buf(),
                message: error.to_string(),
            }
        })?;
        if config.hidden_size != bundle.identity().dimension() {
            return Err(EmbeddingError::IncompatibleDimension {
                expected: bundle.identity().dimension(),
                actual: config.hidden_size,
            });
        }

        let tokenizer = load_tokenizer(bundle.tokenizer_path())?;
        let weights = read_artifact(ModelArtifact::Weights, bundle.weights_path())?;
        let variables =
            VarBuilder::from_buffered_safetensors(weights, DTYPE, &device).map_err(|error| {
                EmbeddingError::InvalidWeights {
                    path: bundle.weights_path().to_path_buf(),
                    message: error.to_string(),
                }
            })?;
        let model = BertModel::load(variables, &config).map_err(|error| EmbeddingError::Model {
            message: error.to_string(),
        })?;

        let embedder = Self {
            model,
            tokenizer,
            device,
            identity: bundle.identity().clone(),
        };
        let warmup_text = "This is a warmup sentence to initialize the embedding model and trigger any lazy loading of weights and computation kernels for optimal performance.";
        embedder.forward(warmup_text)?;
        info!(
            model = embedder.identity.repository(),
            revision = embedder.identity.revision(),
            dimension = embedder.identity.dimension(),
            init_ms = start.elapsed().as_millis(),
            "semantic embedder ready"
        );
        Ok(embedder)
    }

    /// Load conventional model artifacts from a local directory.
    pub fn from_directory(
        directory: impl AsRef<std::path::Path>,
        identity: ModelIdentity,
    ) -> Result<Self, EmbeddingError> {
        Self::from_bundle(&ModelBundle::from_directory(directory, identity))
    }

    fn forward(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        let mut embeddings = self.forward_batch(&[text])?;
        embeddings.pop().ok_or_else(|| EmbeddingError::Inference {
            message: "model returned no embedding for one input".to_string(),
        })
    }

    fn forward_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let encodings = self
            .tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|error| EmbeddingError::Inference {
                message: format!("tokenization failed: {error}"),
            })?;
        let max_length = encodings
            .iter()
            .map(|encoding| encoding.get_ids().len())
            .max()
            .unwrap_or(0)
            .min(MAX_SEQUENCE_LENGTH);
        if max_length == 0 {
            return Err(EmbeddingError::Inference {
                message: "tokenizer produced an empty sequence".to_string(),
            });
        }

        let mut input_ids = Vec::new();
        let mut attention_masks = Vec::new();
        let mut token_type_ids = Vec::new();
        for encoding in &encodings {
            let length = encoding.get_ids().len().min(max_length);
            let mut ids = encoding.get_ids()[..length].to_vec();
            let mut attention = encoding.get_attention_mask()[..length].to_vec();
            let mut type_ids = encoding.get_type_ids()[..length].to_vec();
            ids.resize(max_length, 0);
            attention.resize(max_length, 0);
            type_ids.resize(max_length, 0);
            input_ids.extend(ids);
            attention_masks.extend(attention);
            token_type_ids.extend(type_ids);
        }

        let batch_size = texts.len();
        let input_ids = tensor(input_ids, batch_size, max_length, &self.device)?
            .to_dtype(DType::U32)
            .map_err(inference)?;
        let attention_mask = tensor(attention_masks, batch_size, max_length, &self.device)?;
        let token_type_ids = tensor(token_type_ids, batch_size, max_length, &self.device)?
            .to_dtype(DType::U32)
            .map_err(inference)?;
        let output = self
            .model
            .forward(&input_ids, &token_type_ids, Some(&attention_mask))
            .map_err(inference)?;
        let cls = output
            .narrow(1, 0, 1)
            .and_then(|tensor| tensor.squeeze(1))
            .map_err(inference)?;
        let norm = cls
            .sqr()
            .and_then(|tensor| tensor.sum_keepdim(1))
            .and_then(|tensor| tensor.sqrt())
            .and_then(|tensor| tensor.clamp(1e-12, f64::MAX))
            .map_err(inference)?;
        let normalized = cls.broadcast_div(&norm).map_err(inference)?;
        normalized.to_vec2::<f32>().map_err(inference)
    }

    /// Embed a query with the pinned retrieval instruction prefix.
    pub fn embed_query(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        <Self as Embedder>::embed_query(self, text)
    }

    /// Embed a target without a retrieval prefix.
    pub fn embed_target(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        <Self as Embedder>::embed_target(self, text)
    }

    /// Batch-embed queries with the pinned retrieval instruction prefix.
    pub fn embed_batch_queries(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        <Self as Embedder>::embed_batch_queries(self, texts)
    }

    /// Batch-embed targets without a retrieval prefix.
    pub fn embed_batch_targets(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        <Self as Embedder>::embed_batch_targets(self, texts)
    }

    /// Legacy target embedding retained for source compatibility.
    #[deprecated(note = "use embed_target for explicit query/target semantics")]
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        self.embed_target(text)
    }

    /// Legacy target batch embedding retained for source compatibility.
    #[deprecated(note = "use embed_batch_targets for explicit query/target semantics")]
    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        self.embed_batch_targets(texts)
    }

    /// Output dimension declared by the loaded model identity.
    #[must_use]
    pub fn embedding_dim(&self) -> usize {
        self.identity.dimension()
    }

    /// Model repository or caller-defined local identity.
    #[must_use]
    pub fn model_name(&self) -> &str {
        self.identity.repository()
    }

    /// Full immutable identity of the loaded model.
    #[must_use]
    pub const fn model_identity(&self) -> &ModelIdentity {
        &self.identity
    }
}

impl Embedder for CandleEmbedder {
    fn embed_query(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        self.forward(&format!("{QUERY_PREFIX}{text}"))
    }

    fn embed_target(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        self.forward(text)
    }

    fn embed_batch_queries(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let prefixed = texts
            .iter()
            .map(|text| format!("{QUERY_PREFIX}{text}"))
            .collect::<Vec<_>>();
        let references = prefixed.iter().map(String::as_str).collect::<Vec<_>>();
        self.forward_batch(&references)
    }

    fn embed_batch_targets(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        self.forward_batch(texts)
    }

    fn embedding_dim(&self) -> usize {
        self.identity.dimension()
    }

    fn model_identity(&self) -> &ModelIdentity {
        &self.identity
    }
}

fn read_artifact(
    artifact: ModelArtifact,
    path: &std::path::Path,
) -> Result<Vec<u8>, EmbeddingError> {
    fs::read(path).map_err(|source| EmbeddingError::ReadArtifact {
        artifact,
        path: path.to_path_buf(),
        source,
    })
}

fn load_tokenizer(path: &std::path::Path) -> Result<Tokenizer, EmbeddingError> {
    Tokenizer::from_file(path).map_err(|error| EmbeddingError::InvalidTokenizer {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn tensor(
    values: Vec<u32>,
    rows: usize,
    columns: usize,
    device: &Device,
) -> Result<Tensor, EmbeddingError> {
    Tensor::from_vec(values, (rows, columns), device).map_err(inference)
}

fn inference(error: candle_core::Error) -> EmbeddingError {
    EmbeddingError::Inference {
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_tokenizer_has_a_typed_failure() {
        let path = std::env::temp_dir().join(format!(
            "semantic-embedder-tokenizer-{}.json",
            std::process::id()
        ));
        fs::write(&path, b"not tokenizer json").unwrap();
        let result = load_tokenizer(&path);
        fs::remove_file(&path).unwrap();
        assert!(matches!(
            result,
            Err(EmbeddingError::InvalidTokenizer { .. })
        ));
    }

    #[test]
    #[ignore = "requires SEMANTIC_EMBEDDER_TEST_BUNDLE with the pinned model"]
    fn cached_bundle_inference_is_deterministic() {
        let directory = std::env::var("SEMANTIC_EMBEDDER_TEST_BUNDLE").unwrap();
        let embedder = CandleEmbedder::from_directory(directory, ModelIdentity::default_bge())
            .expect("load pinned bundle");
        let first = embedder.embed_query("add a timer boundary event").unwrap();
        let second = embedder.embed_query("add a timer boundary event").unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), crate::DEFAULT_EMBEDDING_DIMENSION);
    }
}

//! Host-neutral semantic embedding contracts and optional Candle inference.
//!
//! The default feature set is deliberately empty. Applications may use the
//! deterministic fake or implement [`Embedder`] without compiling a model
//! runtime. The `candle` feature admits local bundles; the separate
//! `huggingface-download` feature adds explicit remote/cache resolution.

#![forbid(unsafe_code)]

mod error;
mod model;

#[cfg(feature = "candle")]
mod candle;
#[cfg(feature = "huggingface-download")]
mod download;

#[cfg(feature = "candle")]
pub use candle::CandleEmbedder;
pub use error::{EmbeddingError, ModelArtifact};
pub use model::{
    ModelBundle, ModelIdentity, DEFAULT_EMBEDDING_DIMENSION, DEFAULT_MODEL_REPOSITORY,
    DEFAULT_MODEL_REVISION, MAX_SEQUENCE_LENGTH, QUERY_PREFIX,
};

/// Host-neutral interface implemented by semantic embedding providers.
pub trait Embedder: Send + Sync {
    /// Embed a user query using the provider's declared query semantics.
    fn embed_query(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;

    /// Embed a retrieval target without query-only instructions.
    fn embed_target(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;

    /// Embed query inputs while preserving input order.
    fn embed_batch_queries(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError>;

    /// Embed target inputs while preserving input order.
    fn embed_batch_targets(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError>;

    /// Return the fixed output dimension.
    fn embedding_dim(&self) -> usize;

    /// Return the immutable model/provider identity.
    fn model_identity(&self) -> &ModelIdentity;
}

/// Deterministic, dependency-free embedder for tests and application fallback wiring.
#[derive(Clone, Debug)]
pub struct DeterministicFakeEmbedder {
    identity: ModelIdentity,
}

impl DeterministicFakeEmbedder {
    /// Construct a deterministic fake with the requested non-zero dimension.
    pub fn new(dimension: usize) -> Result<Self, EmbeddingError> {
        Ok(Self {
            identity: ModelIdentity::new("semantic-embedder/fake", "fnv1a-v1", dimension)?,
        })
    }

    fn embed_bytes(&self, bytes: &[u8]) -> Vec<f32> {
        let mut vector = Vec::with_capacity(self.identity.dimension());
        for index in 0..self.identity.dimension() {
            let mut state = 0xcbf2_9ce4_8422_2325_u64 ^ index as u64;
            for byte in bytes {
                state ^= u64::from(*byte);
                state = state.wrapping_mul(0x0000_0100_0000_01b3);
            }
            let unit = (state as f64 / u64::MAX as f64) * 2.0 - 1.0;
            vector.push(unit as f32);
        }
        let norm = vector
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt()
            .max(1e-12);
        for value in &mut vector {
            *value /= norm;
        }
        vector
    }
}

impl Embedder for DeterministicFakeEmbedder {
    fn embed_query(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Ok(self.embed_bytes(format!("{QUERY_PREFIX}{text}").as_bytes()))
    }

    fn embed_target(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Ok(self.embed_bytes(text.as_bytes()))
    }

    fn embed_batch_queries(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        texts.iter().map(|text| self.embed_query(text)).collect()
    }

    fn embed_batch_targets(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        texts.iter().map(|text| self.embed_target(text)).collect()
    }

    fn embedding_dim(&self) -> usize {
        self.identity.dimension()
    }

    fn model_identity(&self) -> &ModelIdentity {
        &self.identity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_is_deterministic_normalized_and_asymmetric() {
        let fake = DeterministicFakeEmbedder::new(16).unwrap();
        let target = fake.embed_target("add timer").unwrap();
        assert_eq!(target, fake.embed_target("add timer").unwrap());
        assert_ne!(target, fake.embed_query("add timer").unwrap());
        assert_eq!(target.len(), 16);
        let norm = target.iter().map(|value| value * value).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }

    #[test]
    fn fake_batch_preserves_single_item_semantics() {
        let fake = DeterministicFakeEmbedder::new(8).unwrap();
        let texts = ["one", "two"];
        assert_eq!(
            fake.embed_batch_targets(&texts).unwrap(),
            texts
                .iter()
                .map(|text| fake.embed_target(text).unwrap())
                .collect::<Vec<_>>()
        );
    }
}

# semantic-embedder

`semantic-embedder` is the host-neutral embedding boundary for the shared DSL
workspace. Its default build contains only typed contracts, errors, model-bundle
paths, and a deterministic fake. It has no database, runtime, network, or host
application dependency.

Optional features are explicit:

- `candle` loads caller-provided local BERT/safetensors bundles and performs
  CPU inference;
- `huggingface-download` additionally resolves an exact model revision through
  Hugging Face Hub;
- `metal` enables Candle's Metal backend dependencies without changing the
  default CPU device selection.

The compatibility model is `BAAI/bge-small-en-v1.5` at revision
`5c38ec7c405ec4b44b94cc5a9bb96e735b38267a`. Model artifacts are not bundled
with this crate. The upstream pinned model card declares the released model
under the MIT licence.

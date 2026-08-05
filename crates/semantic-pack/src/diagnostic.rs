use thiserror::Error;

/// Stable machine-readable diagnostic category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticCode {
    UnsupportedVersion,
    InvalidIdentity,
    Duplicate,
    MissingReference,
    InvalidGraph,
    InvalidArgument,
    InvalidPolicy,
    AmbiguousEvidence,
    MissingBinding,
    InvalidProvenance,
    InvalidExtension,
    ResourceLimit,
    ExecutableMaterial,
}

/// One deterministic validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub pack_id: Option<String>,
    pub source_name: String,
    pub yaml_path: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
}

impl Diagnostic {
    pub(crate) fn validation(
        code: DiagnosticCode,
        pack_id: Option<String>,
        source_name: impl Into<String>,
        yaml_path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            pack_id,
            source_name: source_name.into(),
            yaml_path: yaml_path.into(),
            line: None,
            column: None,
            message: message.into(),
        }
    }
}

/// Failure to retrieve source bytes from a host adapter.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PackSourceError {
    #[error("pack source does not contain the requested pack: {0}")]
    NotFound(String),
    #[error("pack source failed: {0}")]
    Unavailable(String),
    #[error("pack source returned incompatible bytes: {0}")]
    Incompatible(String),
}

/// YAML syntax/schema parse failure with source location.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("failed to parse semantic pack {source_name} at {yaml_path}: {message}")]
pub struct PackParseError {
    pub source_name: String,
    pub yaml_path: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
}

impl PackParseError {
    pub(crate) fn new(
        source_name: String,
        yaml_path: String,
        line: Option<usize>,
        column: Option<usize>,
        message: String,
    ) -> Self {
        Self {
            source_name,
            yaml_path,
            line,
            column,
            message,
        }
    }
}

/// Complete deterministic set of independent admission diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("semantic pack validation failed with {} diagnostic(s)", .diagnostics.len())]
pub struct PackValidationErrors {
    diagnostics: Vec<Diagnostic>,
}

impl PackValidationErrors {
    pub(crate) fn new(mut diagnostics: Vec<Diagnostic>) -> Self {
        diagnostics.sort_by(|left, right| {
            (&left.yaml_path, left.code, &left.message).cmp(&(
                &right.yaml_path,
                right.code,
                &right.message,
            ))
        });
        diagnostics.dedup();
        Self { diagnostics }
    }

    /// Borrow all diagnostics in stable YAML-path order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

/// Failure while producing canonical bytes or hashes after validation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PackCompileError {
    #[error("canonical serialization failed: {0}")]
    CanonicalSerialization(String),
    #[error("compiled artifact exceeds the size limit")]
    ArtifactTooLarge,
}

/// End-to-end parse, validation or compile failure.
#[derive(Debug, Error)]
pub enum PackAdmissionError {
    #[error(transparent)]
    Parse(#[from] PackParseError),
    #[error(transparent)]
    Validation(#[from] PackValidationErrors),
    #[error(transparent)]
    Compile(#[from] PackCompileError),
}

/// Immutable registry failure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistryError {
    #[error("pack is not installed: {0}")]
    NotFound(String),
    #[error("artifact hash is not installed: {0}")]
    HashNotFound(String),
    #[error("pack identity already maps to a different immutable artifact: {0}")]
    IdentityConflict(String),
    #[error("stale activation: expected {expected}, current is {actual}")]
    StaleActivation { expected: String, actual: String },
    #[error("registry lock is poisoned")]
    Poisoned,
}

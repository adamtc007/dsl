use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use thiserror::Error;

use super::{
    ArgumentKind, CandidateSemanticSlice, CanonicalCandidateId, DecisionRecord, FiniteScore,
    GraphRevision, InferenceDisposition, InferenceEvidence, ProposalStatus, ProposalWorkbook,
    SemanticDecisionBoard, SlotValue, SnapshotIdentity,
};

/// Current canonical wire schema for the reusable gameboard contracts.
pub const GAMEBOARD_SCHEMA_VERSION: u32 = 1;

/// Maximum byte length of any single contract text field (identifiers,
/// provenance, explanation prose). Bounds decode allocation for
/// attacker-supplied strings; not a product-facing policy limit.
pub const MAX_CONTRACT_TEXT_BYTES: usize = 64 * 1024;

/// Maximum arguments admitted onto one legal move.
pub const MAX_MOVE_ARGUMENTS: usize = 64;

/// Maximum applicability facts admitted onto one legal move.
pub const MAX_APPLICABILITY_FACTS: usize = 64;

/// Maximum legal moves admitted onto one design position. Bounds
/// anchor x candidate enumeration amplification at the contract boundary.
pub const MAX_LEGAL_MOVES: usize = 512;

/// Maximum effect operations admitted onto one graph delta preview.
pub const MAX_DELTA_OPERATIONS: usize = 256;

/// Maximum attempt receipts a single correction-history validation walks.
/// A generic contract-layer safety backstop; product code may enforce a
/// tighter policy limit of its own on top of this one.
pub const MAX_VALIDATED_ATTEMPTS: usize = 1024;

/// Refusal returned while constructing or decoding a gameboard contract.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GameboardContractError {
    /// A wire value used an unsupported schema revision.
    #[error("unsupported gameboard schema version {actual}; expected {expected}")]
    UnsupportedSchema { expected: u32, actual: u32 },
    /// A required textual identity was malformed.
    #[error("invalid {field}: {reason}")]
    InvalidText { field: &'static str, reason: String },
    /// A content hash was not canonical lowercase SHA-256.
    #[error("invalid {field}: expected 64 lowercase hexadecimal characters")]
    InvalidHash { field: &'static str },
    /// A collection violated a canonical contract invariant.
    #[error("invalid {contract}: {reason}")]
    InvalidContract {
        contract: &'static str,
        reason: String,
    },
    /// A correction target was absent or formed a cycle.
    #[error("invalid correction history: {0}")]
    InvalidCorrection(String),
    /// A compatibility source could not be projected without guessing authority.
    #[error("legacy compatibility projection refused: {0}")]
    Compatibility(String),
    /// A collection or text field exceeded its resource-safety bound. Distinct
    /// from `InvalidContract` so callers can react to a resource refusal
    /// (e.g. leave the session usable, do not retry) without string matching.
    #[error("{field} exceeds the resource limit of {limit} ({actual} supplied)")]
    ResourceLimitExceeded {
        field: &'static str,
        limit: usize,
        actual: usize,
    },
}

fn validate_schema(actual: u32) -> Result<(), GameboardContractError> {
    if actual == GAMEBOARD_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(GameboardContractError::UnsupportedSchema {
            expected: GAMEBOARD_SCHEMA_VERSION,
            actual,
        })
    }
}

fn validate_text(field: &'static str, value: &str) -> Result<(), GameboardContractError> {
    if value.is_empty() {
        return Err(GameboardContractError::InvalidText {
            field,
            reason: "must not be empty".to_string(),
        });
    }
    if value.chars().any(char::is_control) {
        return Err(GameboardContractError::InvalidText {
            field,
            reason: "must not contain control characters".to_string(),
        });
    }
    if value.len() > MAX_CONTRACT_TEXT_BYTES {
        return Err(GameboardContractError::ResourceLimitExceeded {
            field,
            limit: MAX_CONTRACT_TEXT_BYTES,
            actual: value.len(),
        });
    }
    Ok(())
}

fn validate_hash(field: &'static str, value: &str) -> Result<(), GameboardContractError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(GameboardContractError::InvalidHash { field })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ContractText(String);

impl ContractText {
    fn new(field: &'static str, value: impl Into<String>) -> Result<Self, GameboardContractError> {
        let value = value.into();
        validate_text(field, &value)?;
        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for ContractText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ContractText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new("contract text", value).map_err(serde::de::Error::custom)
    }
}

macro_rules! text_identity {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Validated ", $field, ".")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(ContractText);

        impl $name {
            #[doc = concat!("Construct a validated ", $field, ".")]
            pub fn new(value: impl Into<String>) -> Result<Self, GameboardContractError> {
                ContractText::new($field, value).map(Self)
            }

            #[doc = concat!("Borrow the validated ", $field, ".")]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
            }
        }
    };
}

macro_rules! hash_identity {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Canonical SHA-256 ", $field, ".")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Admit a canonical lowercase SHA-256 ", $field, ".")]
            pub fn new(value: impl Into<String>) -> Result<Self, GameboardContractError> {
                let value = value.into();
                validate_hash($field, &value)?;
                Ok(Self(value))
            }

            #[doc = concat!("Borrow the ", $field, ".")]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
            }
        }
    };
}

text_identity!(GameDomainId, "game domain identity");
text_identity!(MoveAttemptId, "move attempt identity");
text_identity!(DesignTurnId, "design turn identity");
text_identity!(GameSessionId, "game session identity");
text_identity!(SemanticFamilyId, "semantic family identity");
text_identity!(GraphElementRef, "graph element reference");
text_identity!(RuleCode, "rule code");
text_identity!(MessageKey, "message key");
text_identity!(ProducerIdentity, "producer identity");
text_identity!(MoveArgumentName, "move argument name");

hash_identity!(DesignStateId, "design state identity");
hash_identity!(LegalMoveId, "legal move identity");
hash_identity!(MoveSetHash, "move-set hash");
hash_identity!(GraphContentHash, "graph content hash");
hash_identity!(GraphDeltaHash, "graph delta hash");
hash_identity!(HistoryHash, "history hash");
hash_identity!(RuleExplanationId, "rule explanation identity");
hash_identity!(BeliefHash, "belief hash");
hash_identity!(DesignTurnHash, "design turn hash");
hash_identity!(GameTurnRecordHash, "game turn record hash");
hash_identity!(GameTurnAdjudicationHash, "game turn adjudication hash");

fn hash_fields(domain: &str, fields: impl IntoIterator<Item = (String, String)>) -> String {
    let mut hasher = Sha256::new();
    framed_field(&mut hasher, "contract", domain);
    for (tag, value) in fields {
        framed_field(&mut hasher, &tag, &value);
    }
    hex::encode(hasher.finalize())
}

fn framed_field(hasher: &mut Sha256, tag: &str, value: &str) {
    hasher.update((tag.len() as u64).to_be_bytes());
    hasher.update(tag.as_bytes());
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value.as_bytes());
}

/// Hierarchical path selecting one governed board within a game domain.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct BoardPath(Vec<ContractText>);

impl BoardPath {
    /// Construct a non-empty board path from validated domain-neutral segments.
    pub fn new(segments: Vec<String>) -> Result<Self, GameboardContractError> {
        if segments.is_empty() {
            return Err(GameboardContractError::InvalidContract {
                contract: "board path",
                reason: "must contain at least one segment".to_string(),
            });
        }
        let segments = segments
            .into_iter()
            .map(|segment| ContractText::new("board path segment", segment))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(segments))
    }

    /// Iterate over the validated path segments.
    pub fn segments(&self) -> impl ExactSizeIterator<Item = &str> {
        self.0.iter().map(ContractText::as_str)
    }
}

impl<'de> Deserialize<'de> for BoardPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(Vec::<String>::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Why a position deliberately carries no graph focus. `NotProvided` is the
/// only producible reason today — no code path clears a sticky focus,
/// downgrades an unresolved reference, records an auto-selected default, or
/// projects a legacy (pre-gameboard) board, so those reasons were removed
/// rather than kept as unreachable scaffolding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusAbsenceReason {
    NotProvided,
}

/// Explicit current graph focus; absence is never silently inferred.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DesignFocus {
    Absent {
        reason: FocusAbsenceReason,
    },
    Element {
        element: GraphElementRef,
    },
    Unknown {
        reference: GraphElementRef,
    },
}

impl DesignFocus {
    /// Record an explicit absence.
    pub fn absent(reason: FocusAbsenceReason) -> Self {
        Self::Absent { reason }
    }

    /// Focus one known graph element.
    pub fn element(element: GraphElementRef) -> Self {
        Self::Element { element }
    }

    /// Preserve an unresolved focus reference without selecting another element.
    pub fn unknown(reference: GraphElementRef) -> Self {
        Self::Unknown { reference }
    }
}

impl<'de> Deserialize<'de> for DesignFocus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case")]
        enum Wire {
            Absent { reason: FocusAbsenceReason },
            Element { element: GraphElementRef },
            Unknown { reference: GraphElementRef },
        }
        Ok(match Wire::deserialize(deserializer)? {
            Wire::Absent { reason } => Self::absent(reason),
            Wire::Element { element } => Self::element(element),
            Wire::Unknown { reference } => Self::unknown(reference),
        })
    }
}

/// One typed argument binding on a legal move.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MoveArgument {
    name: ContractText,
    kind: ArgumentKind,
    required: bool,
    value: Option<SlotValue>,
    provenance: Option<ContractText>,
}

impl<'de> Deserialize<'de> for MoveArgument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            name: String,
            kind: ArgumentKind,
            required: bool,
            value: Option<SlotValue>,
            provenance: Option<String>,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.name,
            wire.kind,
            wire.required,
            wire.value,
            wire.provenance,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl MoveArgument {
    /// Construct a typed, optionally resolved argument.
    pub fn new(
        name: impl Into<String>,
        kind: ArgumentKind,
        required: bool,
        value: Option<SlotValue>,
        provenance: Option<String>,
    ) -> Result<Self, GameboardContractError> {
        if let Some(value) = &value {
            if !super::slot_value_matches(kind, value) {
                return Err(GameboardContractError::InvalidContract {
                    contract: "move argument",
                    reason: "value kind does not match the declared argument kind".to_string(),
                });
            }
        }
        if value.is_some() != provenance.is_some() {
            return Err(GameboardContractError::InvalidContract {
                contract: "move argument",
                reason: "a resolved value and its provenance must appear together".to_string(),
            });
        }
        Ok(Self {
            name: ContractText::new("move argument name", name)?,
            kind,
            required,
            value,
            provenance: provenance
                .map(|value| ContractText::new("move argument provenance", value))
                .transpose()?,
        })
    }

    /// Borrow the argument name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the declared argument kind.
    pub fn kind(&self) -> ArgumentKind {
        self.kind
    }

    /// Whether the argument is required for a complete binding.
    pub fn required(&self) -> bool {
        self.required
    }

    /// Borrow the resolved value, when one was explicitly bound.
    pub fn value(&self) -> Option<&SlotValue> {
        self.value.as_ref()
    }

    /// Borrow the binding provenance.
    pub fn provenance(&self) -> Option<&str> {
        self.provenance.as_ref().map(ContractText::as_str)
    }
}

/// Deterministic binding completeness for a position-bound move.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum MoveBindingState {
    Complete,
    Incomplete {
        missing_arguments: Vec<MoveArgumentName>,
        missing_anchor: bool,
    },
}

impl<'de> Deserialize<'de> for MoveBindingState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "state", rename_all = "snake_case")]
        enum Wire {
            Complete,
            Incomplete {
                missing_arguments: Vec<MoveArgumentName>,
                missing_anchor: bool,
            },
        }
        match Wire::deserialize(deserializer)? {
            Wire::Complete => Ok(Self::Complete),
            Wire::Incomplete {
                mut missing_arguments,
                missing_anchor,
            } => {
                let original = missing_arguments.clone();
                missing_arguments.sort();
                missing_arguments.dedup();
                if missing_arguments != original {
                    return Err(serde::de::Error::custom(
                        "missing arguments must be unique and canonically ordered",
                    ));
                }
                if missing_arguments.is_empty() && !missing_anchor {
                    return Err(serde::de::Error::custom(
                        "an incomplete binding must identify a missing binding",
                    ));
                }
                Ok(Self::Incomplete {
                    missing_arguments,
                    missing_anchor,
                })
            }
        }
    }
}

impl MoveBindingState {
    /// Names of required arguments still unresolved.
    pub fn missing_arguments(&self) -> impl Iterator<Item = &str> {
        let values = match self {
            Self::Complete => &[][..],
            Self::Incomplete {
                missing_arguments, ..
            } => missing_arguments.as_slice(),
        };
        values.iter().map(MoveArgumentName::as_str)
    }

    /// Whether a required graph anchor remains unresolved.
    pub fn missing_anchor(&self) -> bool {
        matches!(
            self,
            Self::Incomplete {
                missing_anchor: true,
                ..
            }
        )
    }

    /// Whether all required bindings are present.
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Deterministic rule applicability classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicabilityState {
    Applicable,
    Incomplete,
    Inapplicable,
    PolicyHidden,
}

/// One governed, provenance-bearing applicability fact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ApplicabilityFact {
    rule_code: RuleCode,
    state: ApplicabilityState,
    explanation_id: Option<RuleExplanationId>,
    provenance: ContractText,
}

impl ApplicabilityFact {
    /// Construct an applicability fact without embedding application wording.
    pub fn new(
        rule_code: RuleCode,
        state: ApplicabilityState,
        explanation_id: Option<RuleExplanationId>,
        provenance: impl Into<String>,
    ) -> Result<Self, GameboardContractError> {
        Ok(Self {
            rule_code,
            state,
            explanation_id,
            provenance: ContractText::new("applicability provenance", provenance)?,
        })
    }

    /// Governed rule code supporting the fact.
    pub fn rule_code(&self) -> &RuleCode {
        &self.rule_code
    }

    /// Applicability classification.
    pub fn state(&self) -> ApplicabilityState {
        self.state
    }

    /// Optional retrievable governed explanation.
    pub fn explanation_id(&self) -> Option<&RuleExplanationId> {
        self.explanation_id.as_ref()
    }

    /// Borrow the fact provenance.
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }
}

/// One domain-neutral effect in a previewed graph delta.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GraphDeltaOperation {
    effect_code: ContractText,
    target: Option<GraphElementRef>,
    payload_hash: GraphContentHash,
}

impl GraphDeltaOperation {
    /// Construct a content-addressed effect without application vocabulary.
    pub fn new(
        effect_code: impl Into<String>,
        target: Option<GraphElementRef>,
        payload_hash: GraphContentHash,
    ) -> Result<Self, GameboardContractError> {
        Ok(Self {
            effect_code: ContractText::new("graph delta effect code", effect_code)?,
            target,
            payload_hash,
        })
    }

    /// Borrow the adapter-owned effect code.
    pub fn effect_code(&self) -> &str {
        self.effect_code.as_str()
    }

    /// Borrow the affected element, when the effect is target-bound.
    pub fn target(&self) -> Option<&GraphElementRef> {
        self.target.as_ref()
    }

    /// Hash of the effect payload held by the domain adapter.
    pub fn payload_hash(&self) -> &GraphContentHash {
        &self.payload_hash
    }
}

/// Canonical, non-mutating preview of a proposed graph transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GraphDeltaPreview {
    schema_version: u32,
    from_graph: GraphContentHash,
    operations: Vec<GraphDeltaOperation>,
    delta_hash: GraphDeltaHash,
}

impl GraphDeltaPreview {
    /// Construct a canonical preview. Operation order is semantically irrelevant.
    pub fn new(
        schema_version: u32,
        from_graph: GraphContentHash,
        mut operations: Vec<GraphDeltaOperation>,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        if operations.len() > MAX_DELTA_OPERATIONS {
            return Err(GameboardContractError::ResourceLimitExceeded {
                field: "graph delta preview operations",
                limit: MAX_DELTA_OPERATIONS,
                actual: operations.len(),
            });
        }
        operations.sort();
        if operations.is_empty() {
            return Err(GameboardContractError::InvalidContract {
                contract: "graph delta preview",
                reason: "must contain at least one effect".to_string(),
            });
        }
        if operations.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(GameboardContractError::InvalidContract {
                contract: "graph delta preview",
                reason: "duplicate effects are not canonical".to_string(),
            });
        }
        let fields = std::iter::once(("schema_version".to_string(), schema_version.to_string()))
            .chain(std::iter::once((
                "from_graph".to_string(),
                from_graph.as_str().to_string(),
            )))
            .chain(
                operations
                    .iter()
                    .enumerate()
                    .flat_map(|(index, operation)| {
                        [
                            (
                                format!("operation.{index}.effect"),
                                operation.effect_code().to_string(),
                            ),
                            (
                                format!("operation.{index}.target"),
                                operation
                                    .target()
                                    .map_or_else(String::new, |value| value.as_str().to_string()),
                            ),
                            (
                                format!("operation.{index}.payload"),
                                operation.payload_hash().as_str().to_string(),
                            ),
                        ]
                    }),
            );
        let delta_hash = GraphDeltaHash::new(hash_fields("semantic-gameboard-delta-v1", fields))?;
        Ok(Self {
            schema_version,
            from_graph,
            operations,
            delta_hash,
        })
    }

    /// Canonical schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Graph content from which this preview was calculated.
    pub fn from_graph(&self) -> &GraphContentHash {
        &self.from_graph
    }

    /// Canonically ordered effects.
    pub fn operations(&self) -> &[GraphDeltaOperation] {
        &self.operations
    }

    /// Content identity of the complete preview.
    pub fn delta_hash(&self) -> &GraphDeltaHash {
        &self.delta_hash
    }
}

impl<'de> Deserialize<'de> for GraphDeltaPreview {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            from_graph: GraphContentHash,
            operations: Vec<GraphDeltaOperation>,
            delta_hash: GraphDeltaHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(wire.schema_version, wire.from_graph, wire.operations)
            .map_err(serde::de::Error::custom)?;
        if admitted.delta_hash != wire.delta_hash {
            return Err(serde::de::Error::custom(
                "graph delta hash does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

fn binding_state(
    requires_anchor: bool,
    anchor: Option<&GraphElementRef>,
    arguments: &[MoveArgument],
) -> MoveBindingState {
    let mut missing_arguments = arguments
        .iter()
        .filter(|argument| argument.required && argument.value.is_none())
        .map(|argument| MoveArgumentName(argument.name.clone()))
        .collect::<Vec<_>>();
    missing_arguments.sort();
    let missing_anchor = requires_anchor && anchor.is_none();
    if missing_arguments.is_empty() && !missing_anchor {
        MoveBindingState::Complete
    } else {
        MoveBindingState::Incomplete {
            missing_arguments,
            missing_anchor,
        }
    }
}

/// One typed, position-bound move admitted to a deterministic move set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LegalMove {
    schema_version: u32,
    move_id: LegalMoveId,
    candidate_id: CanonicalCandidateId,
    graph_revision: GraphRevision,
    requires_anchor: bool,
    anchor: Option<GraphElementRef>,
    arguments: Vec<MoveArgument>,
    binding_state: MoveBindingState,
    applicability: Vec<ApplicabilityFact>,
    preview: Option<GraphDeltaPreview>,
    semantic_hash: LegalMoveId,
}

impl LegalMove {
    /// Construct and content-address a legal move.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        candidate_id: CanonicalCandidateId,
        graph_revision: GraphRevision,
        requires_anchor: bool,
        anchor: Option<GraphElementRef>,
        mut arguments: Vec<MoveArgument>,
        mut applicability: Vec<ApplicabilityFact>,
        preview: Option<GraphDeltaPreview>,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        if arguments.len() > MAX_MOVE_ARGUMENTS {
            return Err(GameboardContractError::ResourceLimitExceeded {
                field: "legal move arguments",
                limit: MAX_MOVE_ARGUMENTS,
                actual: arguments.len(),
            });
        }
        if applicability.len() > MAX_APPLICABILITY_FACTS {
            return Err(GameboardContractError::ResourceLimitExceeded {
                field: "legal move applicability facts",
                limit: MAX_APPLICABILITY_FACTS,
                actual: applicability.len(),
            });
        }
        arguments.sort_by(|left, right| left.name.cmp(&right.name));
        if arguments
            .windows(2)
            .any(|pair| pair[0].name == pair[1].name)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "legal move",
                reason: "duplicate argument names are refused".to_string(),
            });
        }
        applicability.sort();
        if applicability.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(GameboardContractError::InvalidContract {
                contract: "legal move",
                reason: "duplicate applicability facts are refused".to_string(),
            });
        }
        if preview.is_some()
            && !binding_state(requires_anchor, anchor.as_ref(), &arguments).is_complete()
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "legal move",
                reason: "an incomplete move cannot carry a graph delta preview".to_string(),
            });
        }
        let binding_state = binding_state(requires_anchor, anchor.as_ref(), &arguments);
        let mut fields = vec![
            ("schema_version".to_string(), schema_version.to_string()),
            (
                "candidate_id".to_string(),
                candidate_id.as_str().to_string(),
            ),
            (
                "graph_revision".to_string(),
                graph_revision.as_str().to_string(),
            ),
            ("requires_anchor".to_string(), requires_anchor.to_string()),
            (
                "anchor".to_string(),
                anchor
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "binding_state".to_string(),
                match &binding_state {
                    MoveBindingState::Complete => "complete".to_string(),
                    MoveBindingState::Incomplete { missing_anchor, .. } => {
                        format!("incomplete:{missing_anchor}")
                    }
                },
            ),
        ];
        for (index, argument) in arguments.iter().enumerate() {
            fields.extend([
                (
                    format!("argument.{index}.name"),
                    argument.name().to_string(),
                ),
                (
                    format!("argument.{index}.kind"),
                    format!("{:?}", argument.kind()),
                ),
                (
                    format!("argument.{index}.required"),
                    argument.required().to_string(),
                ),
                (
                    format!("argument.{index}.value"),
                    argument
                        .value()
                        .map_or_else(String::new, canonical_slot_value),
                ),
                (
                    format!("argument.{index}.provenance"),
                    argument.provenance().unwrap_or_default().to_string(),
                ),
            ]);
        }
        for (index, fact) in applicability.iter().enumerate() {
            fields.extend([
                (
                    format!("applicability.{index}.rule"),
                    fact.rule_code().as_str().to_string(),
                ),
                (
                    format!("applicability.{index}.state"),
                    format!("{:?}", fact.state()),
                ),
                (
                    format!("applicability.{index}.explanation"),
                    fact.explanation_id()
                        .map_or_else(String::new, |value| value.as_str().to_string()),
                ),
                (
                    format!("applicability.{index}.provenance"),
                    fact.provenance().to_string(),
                ),
            ]);
        }
        fields.push((
            "preview".to_string(),
            preview
                .as_ref()
                .map_or_else(String::new, |value| value.delta_hash().as_str().to_string()),
        ));
        let semantic_hash =
            LegalMoveId::new(hash_fields("semantic-gameboard-legal-move-v1", fields))?;
        Ok(Self {
            schema_version,
            move_id: semantic_hash.clone(),
            candidate_id,
            graph_revision,
            requires_anchor,
            anchor,
            arguments,
            binding_state,
            applicability,
            preview,
            semantic_hash,
        })
    }

    /// Schema version of the move wire contract.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Stable identity bound to every authority-bearing move field.
    pub fn move_id(&self) -> &LegalMoveId {
        &self.move_id
    }

    /// Semantic candidate projected into this concrete move.
    pub fn candidate_id(&self) -> &CanonicalCandidateId {
        &self.candidate_id
    }

    /// Graph revision against which this move was enumerated.
    pub fn graph_revision(&self) -> &GraphRevision {
        &self.graph_revision
    }

    /// Whether a concrete graph anchor is required.
    pub fn requires_anchor(&self) -> bool {
        self.requires_anchor
    }

    /// Concrete graph anchor when selected.
    pub fn anchor(&self) -> Option<&GraphElementRef> {
        self.anchor.as_ref()
    }

    /// Canonically ordered typed bindings.
    pub fn arguments(&self) -> &[MoveArgument] {
        &self.arguments
    }

    /// Deterministically calculated binding state.
    pub fn binding_state(&self) -> &MoveBindingState {
        &self.binding_state
    }

    /// Governed applicability facts.
    pub fn applicability(&self) -> &[ApplicabilityFact] {
        &self.applicability
    }

    /// Non-mutating preview, present only for sufficiently bound moves.
    pub fn preview(&self) -> Option<&GraphDeltaPreview> {
        self.preview.as_ref()
    }

    /// Semantic content hash; identical to the move identity in schema v1.
    pub fn semantic_hash(&self) -> &LegalMoveId {
        &self.semantic_hash
    }
}

impl<'de> Deserialize<'de> for LegalMove {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            move_id: LegalMoveId,
            candidate_id: CanonicalCandidateId,
            graph_revision: GraphRevision,
            requires_anchor: bool,
            anchor: Option<GraphElementRef>,
            arguments: Vec<MoveArgument>,
            binding_state: MoveBindingState,
            applicability: Vec<ApplicabilityFact>,
            preview: Option<GraphDeltaPreview>,
            semantic_hash: LegalMoveId,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.candidate_id,
            wire.graph_revision,
            wire.requires_anchor,
            wire.anchor,
            wire.arguments,
            wire.applicability,
            wire.preview,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.move_id != wire.move_id
            || admitted.semantic_hash != wire.semantic_hash
            || admitted.binding_state != wire.binding_state
        {
            return Err(serde::de::Error::custom(
                "legal move derived fields do not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

fn canonical_slot_value(value: &SlotValue) -> String {
    match value {
        SlotValue::Text(value) => format!("text:{value}"),
        SlotValue::Identifier(value) => format!("identifier:{value}"),
        SlotValue::NodeReference(value) => format!("node_reference:{value}"),
        SlotValue::DataReference(value) => format!("data_reference:{value}"),
        SlotValue::Count(value) => format!("count:{value}"),
        SlotValue::DurationMillis(value) => format!("duration_millis:{value}"),
        SlotValue::Condition(value) => format!("condition:{value}"),
        SlotValue::SubprocessReference(value) => format!("subprocess_reference:{value}"),
        SlotValue::Boolean(value) => format!("boolean:{value}"),
    }
}

/// Canonical projection of authoritative graph state and its current legal moves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DesignPosition {
    schema_version: u32,
    state_id: DesignStateId,
    domain: GameDomainId,
    board_path: BoardPath,
    semantic_snapshot: SnapshotIdentity,
    graph_revision: GraphRevision,
    graph_hash: GraphContentHash,
    compiler_profile: ContractText,
    policy_identity: ContractText,
    current_proposal_hash: Option<GraphContentHash>,
    focus: DesignFocus,
    history_hash: HistoryHash,
    legal_moves: Vec<LegalMove>,
    move_set_hash: MoveSetHash,
}

impl DesignPosition {
    /// Construct a position without consulting time, randomness, storage or a server.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        domain: GameDomainId,
        board_path: BoardPath,
        semantic_snapshot: SnapshotIdentity,
        graph_revision: GraphRevision,
        graph_hash: GraphContentHash,
        compiler_profile: impl Into<String>,
        policy_identity: impl Into<String>,
        current_proposal_hash: Option<GraphContentHash>,
        focus: DesignFocus,
        history_hash: HistoryHash,
        mut legal_moves: Vec<LegalMove>,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        if legal_moves.len() > MAX_LEGAL_MOVES {
            return Err(GameboardContractError::ResourceLimitExceeded {
                field: "design position legal moves",
                limit: MAX_LEGAL_MOVES,
                actual: legal_moves.len(),
            });
        }
        let compiler_profile = ContractText::new("compiler profile", compiler_profile)?;
        let policy_identity = ContractText::new("policy identity", policy_identity)?;
        for legal_move in &legal_moves {
            if legal_move.graph_revision() != &graph_revision {
                return Err(GameboardContractError::InvalidContract {
                    contract: "design position",
                    reason: "every move must be bound to the position graph revision".to_string(),
                });
            }
        }
        legal_moves.sort_by(|left, right| left.move_id.cmp(&right.move_id));
        if legal_moves
            .windows(2)
            .any(|pair| pair[0].move_id == pair[1].move_id)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "design position",
                reason: "duplicate legal moves are refused".to_string(),
            });
        }
        let focus_hash = hash_focus(&focus);
        let move_set_fields = [
            (
                "graph_revision".to_string(),
                graph_revision.as_str().to_string(),
            ),
            (
                "semantic_snapshot".to_string(),
                semantic_snapshot.as_str().to_string(),
            ),
            ("focus".to_string(), focus_hash.clone()),
            ("policy".to_string(), policy_identity.as_str().to_string()),
            (
                "compiler_profile".to_string(),
                compiler_profile.as_str().to_string(),
            ),
        ]
        .into_iter()
        .chain(legal_moves.iter().enumerate().map(|(index, legal_move)| {
            (
                format!("move.{index}"),
                legal_move.move_id().as_str().to_string(),
            )
        }));
        let move_set_hash = MoveSetHash::new(hash_fields(
            "semantic-gameboard-move-set-v1",
            move_set_fields,
        ))?;
        let state_fields = [
            ("schema_version".to_string(), schema_version.to_string()),
            ("domain".to_string(), domain.as_str().to_string()),
            (
                "board_path".to_string(),
                board_path.segments().collect::<Vec<_>>().join("/"),
            ),
            (
                "semantic_snapshot".to_string(),
                semantic_snapshot.as_str().to_string(),
            ),
            (
                "graph_revision".to_string(),
                graph_revision.as_str().to_string(),
            ),
            ("graph_hash".to_string(), graph_hash.as_str().to_string()),
            (
                "compiler_profile".to_string(),
                compiler_profile.as_str().to_string(),
            ),
            ("policy".to_string(), policy_identity.as_str().to_string()),
            (
                "current_proposal".to_string(),
                current_proposal_hash
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            ("focus".to_string(), focus_hash),
            ("history".to_string(), history_hash.as_str().to_string()),
            ("move_set".to_string(), move_set_hash.as_str().to_string()),
        ];
        let state_id = DesignStateId::new(hash_fields(
            "semantic-gameboard-design-position-v1",
            state_fields,
        ))?;
        Ok(Self {
            schema_version,
            state_id,
            domain,
            board_path,
            semantic_snapshot,
            graph_revision,
            graph_hash,
            compiler_profile,
            policy_identity,
            current_proposal_hash,
            focus,
            history_hash,
            legal_moves,
            move_set_hash,
        })
    }

    /// Project a legacy semantic board into an explicitly qualified position.
    ///
    /// The adapter requires authority-bearing values absent from the legacy board;
    /// it never fabricates graph, compiler, history or focus identities.
    #[allow(clippy::too_many_arguments)]
    pub fn from_semantic_board(
        board: &SemanticDecisionBoard,
        board_path: BoardPath,
        graph_hash: GraphContentHash,
        compiler_profile: impl Into<String>,
        policy_identity: impl Into<String>,
        history_hash: HistoryHash,
        focus: DesignFocus,
        current_proposal_hash: Option<GraphContentHash>,
    ) -> Result<Self, GameboardContractError> {
        let moves = board
            .candidates
            .iter()
            .map(|candidate| legacy_candidate_move(candidate, &board.graph_revision))
            .collect::<Result<Vec<_>, _>>()?;
        Self::new(
            GAMEBOARD_SCHEMA_VERSION,
            GameDomainId::new(board.domain.as_str())?,
            board_path,
            board.semantic_snapshot.clone(),
            board.graph_revision.clone(),
            graph_hash,
            compiler_profile,
            policy_identity,
            current_proposal_hash,
            focus,
            history_hash,
            moves,
        )
    }

    /// Canonical schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Content identity of this complete position.
    pub fn state_id(&self) -> &DesignStateId {
        &self.state_id
    }
    /// Active game domain.
    pub fn domain(&self) -> &GameDomainId {
        &self.domain
    }
    /// Active hierarchical board path.
    pub fn board_path(&self) -> &BoardPath {
        &self.board_path
    }
    /// Admitted semantic snapshot.
    pub fn semantic_snapshot(&self) -> &SnapshotIdentity {
        &self.semantic_snapshot
    }
    /// Authoritative graph revision.
    pub fn graph_revision(&self) -> &GraphRevision {
        &self.graph_revision
    }
    /// Canonical graph content identity.
    pub fn graph_hash(&self) -> &GraphContentHash {
        &self.graph_hash
    }
    /// Compiler/profile identity owning legality and admission.
    pub fn compiler_profile(&self) -> &str {
        self.compiler_profile.as_str()
    }
    /// Deterministic policy identity.
    pub fn policy_identity(&self) -> &str {
        self.policy_identity.as_str()
    }
    /// Current proposal identity, when one exists.
    pub fn current_proposal_hash(&self) -> Option<&GraphContentHash> {
        self.current_proposal_hash.as_ref()
    }
    /// Explicit current focus.
    pub fn focus(&self) -> &DesignFocus {
        &self.focus
    }
    /// Append-only history identity observed while deriving this position.
    pub fn history_hash(&self) -> &HistoryHash {
        &self.history_hash
    }
    /// Canonically ordered legal moves.
    pub fn legal_moves(&self) -> &[LegalMove] {
        &self.legal_moves
    }
    /// Identity of the complete ordered move set and its authority inputs.
    pub fn move_set_hash(&self) -> &MoveSetHash {
        &self.move_set_hash
    }
}

impl<'de> Deserialize<'de> for DesignPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            state_id: DesignStateId,
            domain: GameDomainId,
            board_path: BoardPath,
            semantic_snapshot: SnapshotIdentity,
            graph_revision: GraphRevision,
            graph_hash: GraphContentHash,
            compiler_profile: String,
            policy_identity: String,
            current_proposal_hash: Option<GraphContentHash>,
            focus: DesignFocus,
            history_hash: HistoryHash,
            legal_moves: Vec<LegalMove>,
            move_set_hash: MoveSetHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.domain,
            wire.board_path,
            wire.semantic_snapshot,
            wire.graph_revision,
            wire.graph_hash,
            wire.compiler_profile,
            wire.policy_identity,
            wire.current_proposal_hash,
            wire.focus,
            wire.history_hash,
            wire.legal_moves,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.state_id != wire.state_id || admitted.move_set_hash != wire.move_set_hash {
            return Err(serde::de::Error::custom(
                "design position identities do not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

fn hash_focus(focus: &DesignFocus) -> String {
    let fields = match focus {
        DesignFocus::Absent { reason } => vec![
            ("kind".to_string(), "absent".to_string()),
            ("reason".to_string(), format!("{reason:?}")),
        ],
        DesignFocus::Element { element } => vec![
            ("kind".to_string(), "element".to_string()),
            ("element".to_string(), element.as_str().to_string()),
        ],
        DesignFocus::Unknown { reference } => vec![
            ("kind".to_string(), "unknown".to_string()),
            ("reference".to_string(), reference.as_str().to_string()),
        ],
    };
    hash_fields("semantic-gameboard-focus-v1", fields)
}

fn legacy_candidate_move(
    candidate: &CandidateSemanticSlice,
    graph_revision: &GraphRevision,
) -> Result<LegalMove, GameboardContractError> {
    let arguments = candidate
        .arguments
        .iter()
        .map(|argument| {
            MoveArgument::new(&argument.name, argument.kind, argument.required, None, None)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let provenance = hash_fields(
        "semantic-gameboard-legacy-applicability-v1",
        [("source".to_string(), candidate.applicability.clone())],
    );
    let applicability = vec![ApplicabilityFact::new(
        RuleCode::new("compat.semantic_board.applicability")?,
        ApplicabilityState::Applicable,
        None,
        provenance,
    )?];
    LegalMove::new(
        GAMEBOARD_SCHEMA_VERSION,
        candidate.canonical_id.clone(),
        graph_revision.clone(),
        false,
        None,
        arguments,
        applicability,
        None,
    )
}

/// Policy-controlled disclosure class for governed semantic resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureClass {
    Public,
    Authenticated,
    Restricted,
    PolicyHidden,
    Technical,
}

/// One governed parameter supplied to an explanation message.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExplanationParameter {
    name: ContractText,
    value: ContractText,
}

impl ExplanationParameter {
    /// Construct a validated named explanation parameter.
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, GameboardContractError> {
        Ok(Self {
            name: ContractText::new("explanation parameter name", name)?,
            value: ContractText::new("explanation parameter value", value)?,
        })
    }

    /// Parameter name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// Governed parameter value.
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

/// Retrievable governed explanation; never parsed from an error string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuleExplanation {
    schema_version: u32,
    explanation_id: RuleExplanationId,
    rule_code: RuleCode,
    message_key: MessageKey,
    parameters: Vec<ExplanationParameter>,
    provenance: ContractText,
    disclosure: DisclosureClass,
}

impl RuleExplanation {
    /// Construct and content-address a governed rule explanation.
    pub fn new(
        schema_version: u32,
        rule_code: RuleCode,
        message_key: MessageKey,
        mut parameters: Vec<ExplanationParameter>,
        provenance: impl Into<String>,
        disclosure: DisclosureClass,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        parameters.sort();
        if parameters
            .windows(2)
            .any(|pair| pair[0].name == pair[1].name)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "rule explanation",
                reason: "duplicate parameter names are refused".to_string(),
            });
        }
        let provenance = ContractText::new("rule explanation provenance", provenance)?;
        let fields = [
            ("schema_version".to_string(), schema_version.to_string()),
            ("rule_code".to_string(), rule_code.as_str().to_string()),
            ("message_key".to_string(), message_key.as_str().to_string()),
            ("provenance".to_string(), provenance.as_str().to_string()),
            ("disclosure".to_string(), format!("{disclosure:?}")),
        ]
        .into_iter()
        .chain(
            parameters
                .iter()
                .enumerate()
                .flat_map(|(index, parameter)| {
                    [
                        (
                            format!("parameter.{index}.name"),
                            parameter.name().to_string(),
                        ),
                        (
                            format!("parameter.{index}.value"),
                            parameter.value().to_string(),
                        ),
                    ]
                }),
        );
        let explanation_id = RuleExplanationId::new(hash_fields(
            "semantic-gameboard-rule-explanation-v1",
            fields,
        ))?;
        Ok(Self {
            schema_version,
            explanation_id,
            rule_code,
            message_key,
            parameters,
            provenance,
            disclosure,
        })
    }

    /// Schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Content identity.
    pub fn explanation_id(&self) -> &RuleExplanationId {
        &self.explanation_id
    }
    /// Governed rule code.
    pub fn rule_code(&self) -> &RuleCode {
        &self.rule_code
    }
    /// Governed localization/message key.
    pub fn message_key(&self) -> &MessageKey {
        &self.message_key
    }
    /// Canonically ordered message parameters.
    pub fn parameters(&self) -> &[ExplanationParameter] {
        &self.parameters
    }
    /// Pack/snapshot provenance.
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }
    /// Disclosure policy class.
    pub fn disclosure(&self) -> DisclosureClass {
        self.disclosure
    }
}

impl<'de> Deserialize<'de> for RuleExplanation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            explanation_id: RuleExplanationId,
            rule_code: RuleCode,
            message_key: MessageKey,
            parameters: Vec<ExplanationParameter>,
            provenance: String,
            disclosure: DisclosureClass,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.rule_code,
            wire.message_key,
            wire.parameters,
            wire.provenance,
            wire.disclosure,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.explanation_id != wire.explanation_id {
            return Err(serde::de::Error::custom(
                "rule explanation identity does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

/// Governed recovery or feedback action presented after an attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackOptionKind {
    Retry,
    SupplyArgument,
    SelectAlternative,
    ChangeFocus,
    ChangeBoard,
    ExplainRule,
    Reject,
    Undo,
    Replace,
    Escalate,
    TechnicalRetry,
}

/// One governed, typed feedback option.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FeedbackOption {
    kind: FeedbackOptionKind,
    move_id: Option<LegalMoveId>,
    prompt_key: MessageKey,
    rule_explanation: Option<RuleExplanationId>,
    disclosure: DisclosureClass,
}

impl FeedbackOption {
    /// Construct an option. Application wording remains in the admitted pack.
    pub fn new(
        kind: FeedbackOptionKind,
        move_id: Option<LegalMoveId>,
        prompt_key: MessageKey,
        rule_explanation: Option<RuleExplanationId>,
        disclosure: DisclosureClass,
    ) -> Self {
        Self {
            kind,
            move_id,
            prompt_key,
            rule_explanation,
            disclosure,
        }
    }

    /// Option kind.
    pub fn kind(&self) -> FeedbackOptionKind {
        self.kind
    }
    /// Currently legal move linked by this option.
    pub fn move_id(&self) -> Option<&LegalMoveId> {
        self.move_id.as_ref()
    }
    /// Governed rendering key.
    pub fn prompt_key(&self) -> &MessageKey {
        &self.prompt_key
    }
    /// Governed explanation supporting the option.
    pub fn rule_explanation(&self) -> Option<&RuleExplanationId> {
        self.rule_explanation.as_ref()
    }
    /// Disclosure policy class.
    pub fn disclosure(&self) -> DisclosureClass {
        self.disclosure
    }
}

/// Closed set of deterministic interactions a semantic gameboard may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameDispositionKind {
    ProposeMove,
    ClarifyMoves,
    RequestMoveArguments,
    ExplainAttempt,
    OfferRecoveryMoves,
    OfferCorrection,
    OutOfScope,
    ChangeFocusOrContext,
    Escalate,
    CompoundPlan,
}

/// Semantic dimension resolved by one governed clarification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameClarificationDimension {
    Move,
    Focus,
    Argument,
}

/// Position-bound deterministic interaction packet.
///
/// Its representation is private so applications cannot manufacture a proposal,
/// clarification or recovery action without passing the position-membership checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameDisposition {
    schema_version: u32,
    kind: GameDispositionKind,
    position_id: DesignStateId,
    move_set_hash: MoveSetHash,
    selected_moves: Vec<LegalMoveId>,
    clarification_dimension: Option<GameClarificationDimension>,
    governed_prompt: Option<ContractText>,
    missing_arguments: Vec<ContractText>,
    attempt_receipt: Option<MoveAttemptReceipt>,
    feedback_options: Vec<FeedbackOption>,
    disposition_hash: GraphContentHash,
}

impl GameDisposition {
    #[allow(clippy::too_many_arguments)]
    fn admit(
        position: &DesignPosition,
        kind: GameDispositionKind,
        mut selected_moves: Vec<LegalMoveId>,
        clarification_dimension: Option<GameClarificationDimension>,
        governed_prompt: Option<String>,
        missing_arguments: Vec<String>,
        attempt_receipt: Option<MoveAttemptReceipt>,
        mut feedback_options: Vec<FeedbackOption>,
    ) -> Result<Self, GameboardContractError> {
        selected_moves.sort();
        if selected_moves.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(GameboardContractError::InvalidContract {
                contract: "game disposition",
                reason: "selected moves must be unique".to_string(),
            });
        }
        for move_id in &selected_moves {
            if !position
                .legal_moves()
                .iter()
                .any(|legal_move| legal_move.move_id() == move_id)
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game disposition",
                    reason: format!(
                        "selected move '{}' is absent from the position",
                        move_id.as_str()
                    ),
                });
            }
        }
        feedback_options.sort();
        feedback_options.dedup();
        for option in &feedback_options {
            if let Some(move_id) = option.move_id() {
                if !position
                    .legal_moves()
                    .iter()
                    .any(|legal_move| legal_move.move_id() == move_id)
                {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game disposition",
                        reason: format!(
                            "feedback move '{}' is absent from the position",
                            move_id.as_str()
                        ),
                    });
                }
            }
        }
        if let Some(receipt) = &attempt_receipt {
            if kind != GameDispositionKind::OfferCorrection
                && receipt.position_id() != position.state_id()
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game disposition",
                    reason: "attempt receipt belongs to a different position".to_string(),
                });
            }
        }
        let governed_prompt = governed_prompt
            .map(|prompt| ContractText::new("game disposition prompt", prompt))
            .transpose()?;
        let mut missing_arguments = missing_arguments
            .into_iter()
            .map(|argument| ContractText::new("missing move argument", argument))
            .collect::<Result<Vec<_>, _>>()?;
        missing_arguments.sort();
        missing_arguments.dedup();

        let invalid = |reason: &str| GameboardContractError::InvalidContract {
            contract: "game disposition",
            reason: reason.to_string(),
        };
        match kind {
            GameDispositionKind::ProposeMove
                if selected_moves.len() != 1
                    || clarification_dimension.is_some()
                    || governed_prompt.is_some()
                    || !missing_arguments.is_empty()
                    || attempt_receipt.is_some() =>
            {
                return Err(invalid(
                    "a proposal must name exactly one move and no terminal attempt",
                ));
            }
            GameDispositionKind::ClarifyMoves
                if !(2..=3).contains(&selected_moves.len())
                    || clarification_dimension.is_none()
                    || governed_prompt.is_none()
                    || attempt_receipt.is_none() =>
            {
                return Err(invalid(
                    "a clarification requires two or three moves, one dimension, governed prompt and attempt",
                ));
            }
            GameDispositionKind::RequestMoveArguments
                if selected_moves.len() != 1
                    || missing_arguments.is_empty()
                    || governed_prompt.is_none()
                    || attempt_receipt.is_none() =>
            {
                return Err(invalid(
                    "an argument request requires one move, missing arguments, governed prompt and attempt",
                ));
            }
            GameDispositionKind::ExplainAttempt
                if !selected_moves.is_empty() || attempt_receipt.is_none() =>
            {
                return Err(invalid(
                    "an explanation requires one terminal attempt and no selected move",
                ));
            }
            GameDispositionKind::OfferRecoveryMoves
                if selected_moves.is_empty()
                    || selected_moves.len() > 3
                    || attempt_receipt.is_none() =>
            {
                return Err(invalid(
                    "recovery requires one to three current legal moves and a terminal attempt",
                ));
            }
            GameDispositionKind::OfferCorrection
                if selected_moves.is_empty()
                    || selected_moves.len() > 3
                    || attempt_receipt.is_none() =>
            {
                return Err(invalid(
                    "correction requires one to three current legal moves and the retained attempt",
                ));
            }
            GameDispositionKind::OutOfScope
                if !selected_moves.is_empty() || attempt_receipt.is_none() =>
            {
                return Err(invalid(
                    "out-of-scope requires a terminal attempt and no selected move",
                ));
            }
            GameDispositionKind::ChangeFocusOrContext
                if !selected_moves.is_empty()
                    || attempt_receipt.is_none()
                    || feedback_options.is_empty() =>
            {
                return Err(invalid(
                    "focus/context change requires a terminal attempt and governed feedback",
                ));
            }
            GameDispositionKind::Escalate
                if !selected_moves.is_empty() || governed_prompt.is_none() =>
            {
                return Err(invalid(
                    "escalation requires a governed prompt and no selected move",
                ));
            }
            GameDispositionKind::CompoundPlan
                if selected_moves.is_empty()
                    || selected_moves.len() > 8
                    || governed_prompt.is_none() =>
            {
                return Err(invalid(
                    "a compound plan requires one to eight non-authoritative legal steps and a governed prompt",
                ));
            }
            _ => {}
        }

        let mut disposition = Self {
            schema_version: GAMEBOARD_SCHEMA_VERSION,
            kind,
            position_id: position.state_id().clone(),
            move_set_hash: position.move_set_hash().clone(),
            selected_moves,
            clarification_dimension,
            governed_prompt,
            missing_arguments,
            attempt_receipt,
            feedback_options,
            disposition_hash: GraphContentHash::new("0".repeat(64))?,
        };
        disposition.disposition_hash = disposition.canonical_hash()?;
        Ok(disposition)
    }

    fn canonical_hash(&self) -> Result<GraphContentHash, GameboardContractError> {
        let fields = [
            (
                "schema_version".to_string(),
                self.schema_version.to_string(),
            ),
            ("kind".to_string(), format!("{:?}", self.kind)),
            (
                "position_id".to_string(),
                self.position_id.as_str().to_string(),
            ),
            (
                "move_set_hash".to_string(),
                self.move_set_hash.as_str().to_string(),
            ),
            (
                "clarification_dimension".to_string(),
                self.clarification_dimension
                    .map_or_else(String::new, |dimension| format!("{dimension:?}")),
            ),
            (
                "governed_prompt".to_string(),
                self.governed_prompt
                    .as_ref()
                    .map_or_else(String::new, |prompt| prompt.as_str().to_string()),
            ),
            (
                "attempt_receipt".to_string(),
                self.attempt_receipt
                    .as_ref()
                    .map_or_else(String::new, |receipt| {
                        receipt.receipt_hash().as_str().to_string()
                    }),
            ),
        ]
        .into_iter()
        .chain(
            self.selected_moves
                .iter()
                .enumerate()
                .map(|(index, move_id)| (format!("move.{index}"), move_id.as_str().to_string())),
        )
        .chain(
            self.missing_arguments
                .iter()
                .enumerate()
                .map(|(index, argument)| {
                    (format!("argument.{index}"), argument.as_str().to_string())
                }),
        )
        .chain(
            self.feedback_options
                .iter()
                .enumerate()
                .flat_map(|(index, option)| {
                    [
                        (
                            format!("feedback.{index}.kind"),
                            format!("{:?}", option.kind()),
                        ),
                        (
                            format!("feedback.{index}.move"),
                            option
                                .move_id()
                                .map_or_else(String::new, |move_id| move_id.as_str().to_string()),
                        ),
                        (
                            format!("feedback.{index}.prompt"),
                            option.prompt_key().as_str().to_string(),
                        ),
                    ]
                }),
        );
        GraphContentHash::new(hash_fields("semantic-gameboard-disposition-v1", fields))
    }

    /// Construct a single-move proposal.
    pub fn propose_move(
        position: &DesignPosition,
        move_id: LegalMoveId,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::ProposeMove,
            vec![move_id],
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
        )
    }

    /// Construct a governed clarification over two or three current legal moves.
    pub fn clarify_moves(
        position: &DesignPosition,
        moves: Vec<LegalMoveId>,
        dimension: GameClarificationDimension,
        governed_prompt: impl Into<String>,
        attempt: MoveAttemptReceipt,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::ClarifyMoves,
            moves,
            Some(dimension),
            Some(governed_prompt.into()),
            Vec::new(),
            Some(attempt),
            Vec::new(),
        )
    }

    /// Construct a typed request for unresolved arguments of one legal move.
    pub fn request_move_arguments(
        position: &DesignPosition,
        move_id: LegalMoveId,
        missing_arguments: Vec<String>,
        governed_prompt: impl Into<String>,
        attempt: MoveAttemptReceipt,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::RequestMoveArguments,
            vec![move_id],
            None,
            Some(governed_prompt.into()),
            missing_arguments,
            Some(attempt),
            Vec::new(),
        )
    }

    /// Construct a terminal attempt explanation without inventing a legal move.
    pub fn explain_attempt(
        position: &DesignPosition,
        attempt: MoveAttemptReceipt,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::ExplainAttempt,
            Vec::new(),
            None,
            None,
            Vec::new(),
            Some(attempt.clone()),
            attempt.feedback_options().to_vec(),
        )
    }

    /// Construct a bounded set of legal recovery moves for an unsuccessful attempt.
    pub fn offer_recovery_moves(
        position: &DesignPosition,
        moves: Vec<LegalMoveId>,
        attempt: MoveAttemptReceipt,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::OfferRecoveryMoves,
            moves,
            None,
            None,
            Vec::new(),
            Some(attempt.clone()),
            attempt.feedback_options().to_vec(),
        )
    }

    /// Construct a bounded set of legal correction moves linked to a retained attempt.
    pub fn offer_correction(
        position: &DesignPosition,
        moves: Vec<LegalMoveId>,
        attempt: MoveAttemptReceipt,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::OfferCorrection,
            moves,
            None,
            None,
            Vec::new(),
            Some(attempt.clone()),
            attempt.feedback_options().to_vec(),
        )
    }

    /// Construct a truthful out-of-scope response.
    pub fn out_of_scope(
        position: &DesignPosition,
        attempt: MoveAttemptReceipt,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::OutOfScope,
            Vec::new(),
            None,
            None,
            Vec::new(),
            Some(attempt.clone()),
            attempt.feedback_options().to_vec(),
        )
    }

    /// Construct a governed focus/context change response.
    pub fn change_focus_or_context(
        position: &DesignPosition,
        attempt: MoveAttemptReceipt,
        feedback_options: Vec<FeedbackOption>,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::ChangeFocusOrContext,
            Vec::new(),
            None,
            None,
            Vec::new(),
            Some(attempt),
            feedback_options,
        )
    }

    /// Construct an explicit collaborative escalation.
    pub fn escalate(
        position: &DesignPosition,
        governed_prompt: impl Into<String>,
        attempt: Option<MoveAttemptReceipt>,
        feedback_options: Vec<FeedbackOption>,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::Escalate,
            Vec::new(),
            None,
            Some(governed_prompt.into()),
            Vec::new(),
            attempt,
            feedback_options,
        )
    }

    /// Construct a non-authoritative plan of individually ratified legal moves.
    pub fn compound_plan(
        position: &DesignPosition,
        steps: Vec<LegalMoveId>,
        governed_prompt: impl Into<String>,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            position,
            GameDispositionKind::CompoundPlan,
            steps,
            None,
            Some(governed_prompt.into()),
            Vec::new(),
            None,
            Vec::new(),
        )
    }

    /// Revalidate all move references against the current position.
    pub fn validate_for_position(
        &self,
        position: &DesignPosition,
    ) -> Result<(), GameboardContractError> {
        if self.position_id != *position.state_id()
            || self.move_set_hash != *position.move_set_hash()
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game disposition",
                reason: "position or move-set identity is stale".to_string(),
            });
        }
        for move_id in self.selected_moves.iter().chain(
            self.feedback_options
                .iter()
                .filter_map(FeedbackOption::move_id),
        ) {
            if !position
                .legal_moves()
                .iter()
                .any(|legal_move| legal_move.move_id() == move_id)
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game disposition",
                    reason: format!(
                        "referenced move '{}' is not currently legal",
                        move_id.as_str()
                    ),
                });
            }
        }
        Ok(())
    }

    pub fn kind(&self) -> GameDispositionKind {
        self.kind
    }
    pub fn position_id(&self) -> &DesignStateId {
        &self.position_id
    }
    pub fn move_set_hash(&self) -> &MoveSetHash {
        &self.move_set_hash
    }
    pub fn selected_moves(&self) -> &[LegalMoveId] {
        &self.selected_moves
    }
    pub fn clarification_dimension(&self) -> Option<GameClarificationDimension> {
        self.clarification_dimension
    }
    pub fn governed_prompt(&self) -> Option<&str> {
        self.governed_prompt.as_ref().map(ContractText::as_str)
    }
    pub fn missing_arguments(&self) -> impl Iterator<Item = &str> {
        self.missing_arguments.iter().map(ContractText::as_str)
    }
    pub fn attempt_receipt(&self) -> Option<&MoveAttemptReceipt> {
        self.attempt_receipt.as_ref()
    }
    pub fn feedback_options(&self) -> &[FeedbackOption] {
        &self.feedback_options
    }
    pub fn disposition_hash(&self) -> &GraphContentHash {
        &self.disposition_hash
    }
}

impl<'de> Deserialize<'de> for GameDisposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            kind: GameDispositionKind,
            position_id: DesignStateId,
            move_set_hash: MoveSetHash,
            selected_moves: Vec<LegalMoveId>,
            clarification_dimension: Option<GameClarificationDimension>,
            governed_prompt: Option<ContractText>,
            missing_arguments: Vec<ContractText>,
            attempt_receipt: Option<MoveAttemptReceipt>,
            feedback_options: Vec<FeedbackOption>,
            disposition_hash: GraphContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        validate_schema(wire.schema_version).map_err(serde::de::Error::custom)?;
        let admitted = Self {
            schema_version: wire.schema_version,
            kind: wire.kind,
            position_id: wire.position_id,
            move_set_hash: wire.move_set_hash,
            selected_moves: wire.selected_moves,
            clarification_dimension: wire.clarification_dimension,
            governed_prompt: wire.governed_prompt,
            missing_arguments: wire.missing_arguments,
            attempt_receipt: wire.attempt_receipt,
            feedback_options: wire.feedback_options,
            disposition_hash: wire.disposition_hash,
        };
        let canonical = admitted
            .canonical_hash()
            .map_err(serde::de::Error::custom)?;
        if canonical != admitted.disposition_hash {
            return Err(serde::de::Error::custom(
                "game disposition identity does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

/// Every typed terminal result of one attempted interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveAttemptOutcome {
    Applied,
    Incomplete,
    Ambiguous,
    Inapplicable,
    DisclosureSafeRefusal,
    Stale,
    CompilerRefused,
    RejectedByUser,
    Corrected,
    SystemFailure,
}

/// Attribution for a correction linked to a retained earlier attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrectionKind {
    Undo,
    Replacement,
    FollowUp,
}

/// Position-bound observed attempt before deterministic policy assigns an outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MoveAttempt {
    schema_version: u32,
    attempt_id: MoveAttemptId,
    position_id: DesignStateId,
    attempted_move: Option<LegalMoveId>,
    observed_intent_hash: GraphContentHash,
}

impl MoveAttempt {
    /// Construct an attempt from explicit session identities and observed content.
    pub fn new(
        schema_version: u32,
        attempt_id: MoveAttemptId,
        position_id: DesignStateId,
        attempted_move: Option<LegalMoveId>,
        observed_intent_hash: GraphContentHash,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        Ok(Self {
            schema_version,
            attempt_id,
            position_id,
            attempted_move,
            observed_intent_hash,
        })
    }

    /// Schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Session-supplied attempt identity.
    pub fn attempt_id(&self) -> &MoveAttemptId {
        &self.attempt_id
    }
    /// Position observed by the attempt.
    pub fn position_id(&self) -> &DesignStateId {
        &self.position_id
    }
    /// Move identity when the observation resolved to a move.
    pub fn attempted_move(&self) -> Option<&LegalMoveId> {
        self.attempted_move.as_ref()
    }
    /// Content identity of the observed input.
    pub fn observed_intent_hash(&self) -> &GraphContentHash {
        &self.observed_intent_hash
    }
}

impl<'de> Deserialize<'de> for MoveAttempt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            attempt_id: MoveAttemptId,
            position_id: DesignStateId,
            attempted_move: Option<LegalMoveId>,
            observed_intent_hash: GraphContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.schema_version,
            wire.attempt_id,
            wire.position_id,
            wire.attempted_move,
            wire.observed_intent_hash,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Receipt for every attempted interaction, including non-transition outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MoveAttemptReceipt {
    schema_version: u32,
    attempt_id: MoveAttemptId,
    position_id: DesignStateId,
    attempted_move: Option<LegalMoveId>,
    observed_intent_hash: GraphContentHash,
    outcome: MoveAttemptOutcome,
    rule_explanations: Vec<RuleExplanationId>,
    feedback_options: Vec<FeedbackOption>,
    correction_of: Option<MoveAttemptId>,
    correction_kind: Option<CorrectionKind>,
    receipt_hash: GraphContentHash,
}

impl MoveAttemptReceipt {
    /// Assign a typed terminal outcome to a recorded attempt.
    pub fn from_attempt(
        attempt: MoveAttempt,
        outcome: MoveAttemptOutcome,
        rule_explanations: Vec<RuleExplanationId>,
        feedback_options: Vec<FeedbackOption>,
        correction_of: Option<MoveAttemptId>,
        correction_kind: Option<CorrectionKind>,
    ) -> Result<Self, GameboardContractError> {
        Self::new(
            attempt.schema_version,
            attempt.attempt_id,
            attempt.position_id,
            attempt.attempted_move,
            attempt.observed_intent_hash,
            outcome,
            rule_explanations,
            feedback_options,
            correction_of,
            correction_kind,
        )
    }

    /// Construct an append-only attempt receipt without mutating domain state.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        attempt_id: MoveAttemptId,
        position_id: DesignStateId,
        attempted_move: Option<LegalMoveId>,
        observed_intent_hash: GraphContentHash,
        outcome: MoveAttemptOutcome,
        mut rule_explanations: Vec<RuleExplanationId>,
        mut feedback_options: Vec<FeedbackOption>,
        correction_of: Option<MoveAttemptId>,
        correction_kind: Option<CorrectionKind>,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        if (correction_of.is_some() || correction_kind.is_some())
            != (correction_of.is_some() && correction_kind.is_some())
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "move attempt receipt",
                reason: "correction target and correction kind must appear together".to_string(),
            });
        }
        if outcome == MoveAttemptOutcome::Corrected && correction_of.is_none() {
            return Err(GameboardContractError::InvalidContract {
                contract: "move attempt receipt",
                reason: "a corrected outcome must link the retained original attempt".to_string(),
            });
        }
        if correction_of.as_ref() == Some(&attempt_id) {
            return Err(GameboardContractError::InvalidCorrection(
                "an attempt cannot correct itself".to_string(),
            ));
        }
        rule_explanations.sort();
        rule_explanations.dedup();
        feedback_options.sort();
        feedback_options.dedup();
        let fields = [
            ("schema_version".to_string(), schema_version.to_string()),
            ("attempt_id".to_string(), attempt_id.as_str().to_string()),
            ("position_id".to_string(), position_id.as_str().to_string()),
            (
                "attempted_move".to_string(),
                attempted_move
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "observed_intent".to_string(),
                observed_intent_hash.as_str().to_string(),
            ),
            ("outcome".to_string(), format!("{outcome:?}")),
            (
                "correction_of".to_string(),
                correction_of
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "correction_kind".to_string(),
                correction_kind.map_or_else(String::new, |value| format!("{value:?}")),
            ),
        ]
        .into_iter()
        .chain(
            rule_explanations
                .iter()
                .enumerate()
                .map(|(index, value)| (format!("explanation.{index}"), value.as_str().to_string())),
        )
        .chain(
            feedback_options
                .iter()
                .enumerate()
                .flat_map(|(index, option)| {
                    [
                        (
                            format!("feedback.{index}.kind"),
                            format!("{:?}", option.kind()),
                        ),
                        (
                            format!("feedback.{index}.move"),
                            option
                                .move_id()
                                .map_or_else(String::new, |value| value.as_str().to_string()),
                        ),
                        (
                            format!("feedback.{index}.prompt"),
                            option.prompt_key().as_str().to_string(),
                        ),
                        (
                            format!("feedback.{index}.explanation"),
                            option
                                .rule_explanation()
                                .map_or_else(String::new, |value| value.as_str().to_string()),
                        ),
                        (
                            format!("feedback.{index}.disclosure"),
                            format!("{:?}", option.disclosure()),
                        ),
                    ]
                }),
        );
        let receipt_hash =
            GraphContentHash::new(hash_fields("semantic-gameboard-attempt-receipt-v1", fields))?;
        Ok(Self {
            schema_version,
            attempt_id,
            position_id,
            attempted_move,
            observed_intent_hash,
            outcome,
            rule_explanations,
            feedback_options,
            correction_of,
            correction_kind,
            receipt_hash,
        })
    }

    /// Project a legacy deterministic decision after the caller explicitly supplies
    /// the typed terminal outcome and governed response resources.
    #[allow(clippy::too_many_arguments)]
    pub fn from_decision_record(
        position: &DesignPosition,
        decision: &DecisionRecord,
        attempt_id: MoveAttemptId,
        observed_intent_hash: GraphContentHash,
        outcome: MoveAttemptOutcome,
        rule_explanations: Vec<RuleExplanationId>,
        feedback_options: Vec<FeedbackOption>,
    ) -> Result<Self, GameboardContractError> {
        let attempted_move = match &decision.disposition {
            InferenceDisposition::Candidate { candidate_id } => Some(
                position
                    .legal_moves()
                    .iter()
                    .find(|legal_move| legal_move.candidate_id() == candidate_id)
                    .ok_or_else(|| {
                        GameboardContractError::Compatibility(format!(
                            "decision candidate '{}' is absent from the position",
                            candidate_id.as_str()
                        ))
                    })?
                    .move_id()
                    .clone(),
            ),
            InferenceDisposition::ClarifyCandidates { .. }
            | InferenceDisposition::OutOfScope
            | InferenceDisposition::Escalate { .. } => None,
        };
        Self::new(
            GAMEBOARD_SCHEMA_VERSION,
            attempt_id,
            position.state_id().clone(),
            attempted_move,
            observed_intent_hash,
            outcome,
            rule_explanations,
            feedback_options,
            None,
            None,
        )
    }

    /// Project a terminal legacy workbook state without turning ready/provisional
    /// workbook states into successful attempts.
    pub fn from_proposal_workbook(
        position: &DesignPosition,
        workbook: &ProposalWorkbook,
        attempt_id: MoveAttemptId,
        observed_intent_hash: GraphContentHash,
        rule_explanations: Vec<RuleExplanationId>,
        feedback_options: Vec<FeedbackOption>,
    ) -> Result<Self, GameboardContractError> {
        let outcome = match workbook.status() {
            ProposalStatus::NeedsArguments => MoveAttemptOutcome::Incomplete,
            ProposalStatus::DryRunRefused => MoveAttemptOutcome::CompilerRefused,
            ProposalStatus::Ratified => MoveAttemptOutcome::Applied,
            ProposalStatus::Rejected => MoveAttemptOutcome::RejectedByUser,
            ProposalStatus::Expired => MoveAttemptOutcome::Stale,
            ProposalStatus::ReadyForDryRun | ProposalStatus::ReadyForRatification => {
                return Err(GameboardContractError::Compatibility(
                    "a provisional workbook state is not a terminal attempt outcome".to_string(),
                ));
            }
        };
        let attempted_move = position
            .legal_moves()
            .iter()
            .find(|legal_move| legal_move.candidate_id() == &workbook.candidate_id)
            .ok_or_else(|| {
                GameboardContractError::Compatibility(format!(
                    "workbook candidate '{}' is absent from the position",
                    workbook.candidate_id.as_str()
                ))
            })?
            .move_id()
            .clone();
        Self::new(
            GAMEBOARD_SCHEMA_VERSION,
            attempt_id,
            position.state_id().clone(),
            Some(attempted_move),
            observed_intent_hash,
            outcome,
            rule_explanations,
            feedback_options,
            None,
            None,
        )
    }

    /// Schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Attributable attempt identity supplied by the session composition root.
    pub fn attempt_id(&self) -> &MoveAttemptId {
        &self.attempt_id
    }
    /// Position on which the attempt was made.
    pub fn position_id(&self) -> &DesignStateId {
        &self.position_id
    }
    /// Referenced legal move when one was identified.
    pub fn attempted_move(&self) -> Option<&LegalMoveId> {
        self.attempted_move.as_ref()
    }
    /// Content identity of the observed input, not its raw private content.
    pub fn observed_intent_hash(&self) -> &GraphContentHash {
        &self.observed_intent_hash
    }
    /// Typed terminal outcome.
    pub fn outcome(&self) -> MoveAttemptOutcome {
        self.outcome
    }
    /// Governed explanations associated with the outcome.
    pub fn rule_explanations(&self) -> &[RuleExplanationId] {
        &self.rule_explanations
    }
    /// Governed recovery and feedback options.
    pub fn feedback_options(&self) -> &[FeedbackOption] {
        &self.feedback_options
    }
    /// Retained original attempt corrected by this one.
    pub fn correction_of(&self) -> Option<&MoveAttemptId> {
        self.correction_of.as_ref()
    }
    /// Correction mechanism.
    pub fn correction_kind(&self) -> Option<CorrectionKind> {
        self.correction_kind
    }
    /// Content identity of the complete receipt.
    pub fn receipt_hash(&self) -> &GraphContentHash {
        &self.receipt_hash
    }
}

impl<'de> Deserialize<'de> for MoveAttemptReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            attempt_id: MoveAttemptId,
            position_id: DesignStateId,
            attempted_move: Option<LegalMoveId>,
            observed_intent_hash: GraphContentHash,
            outcome: MoveAttemptOutcome,
            rule_explanations: Vec<RuleExplanationId>,
            feedback_options: Vec<FeedbackOption>,
            correction_of: Option<MoveAttemptId>,
            correction_kind: Option<CorrectionKind>,
            receipt_hash: GraphContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.attempt_id,
            wire.position_id,
            wire.attempted_move,
            wire.observed_intent_hash,
            wire.outcome,
            wire.rule_explanations,
            wire.feedback_options,
            wire.correction_of,
            wire.correction_kind,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.receipt_hash != wire.receipt_hash {
            return Err(serde::de::Error::custom(
                "attempt receipt hash does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

/// Validate append-only correction links and reject missing targets or cycles.
pub fn validate_attempt_history(
    receipts: &[MoveAttemptReceipt],
) -> Result<(), GameboardContractError> {
    if receipts.len() > MAX_VALIDATED_ATTEMPTS {
        return Err(GameboardContractError::ResourceLimitExceeded {
            field: "attempt history",
            limit: MAX_VALIDATED_ATTEMPTS,
            actual: receipts.len(),
        });
    }
    let by_id = receipts
        .iter()
        .map(|receipt| (receipt.attempt_id(), receipt))
        .collect::<BTreeMap<_, _>>();
    if by_id.len() != receipts.len() {
        return Err(GameboardContractError::InvalidCorrection(
            "attempt identities must be unique".to_string(),
        ));
    }
    for receipt in receipts {
        let mut seen = BTreeSet::new();
        let mut cursor = receipt;
        while let Some(target) = cursor.correction_of() {
            if !seen.insert(cursor.attempt_id()) {
                return Err(GameboardContractError::InvalidCorrection(
                    "correction links must be acyclic".to_string(),
                ));
            }
            cursor = by_id.get(target).copied().ok_or_else(|| {
                GameboardContractError::InvalidCorrection(format!(
                    "attempt '{}' corrects an unknown attempt '{}'",
                    receipt.attempt_id().as_str(),
                    target.as_str()
                ))
            })?;
        }
    }
    Ok(())
}

/// Filter explanations to explicitly admitted disclosure classes while retaining
/// canonical input order. Hidden explanations are omitted, never paraphrased.
pub fn filter_rule_explanations<'a>(
    explanations: &'a [RuleExplanation],
    allowed: &[DisclosureClass],
) -> Vec<&'a RuleExplanation> {
    let allowed = allowed.iter().copied().collect::<BTreeSet<_>>();
    explanations
        .iter()
        .filter(|explanation| allowed.contains(&explanation.disclosure()))
        .collect()
}

/// Complete deterministic and statistical evidence attached to one legal move.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MoveEvidence {
    schema_version: u32,
    move_id: LegalMoveId,
    lanes: Vec<super::LaneScore>,
    final_score: FiniteScore,
    probability: FiniteScore,
    explanation_codes: Vec<RuleCode>,
    producer: ProducerIdentity,
    evidence_hash: GraphContentHash,
}

impl MoveEvidence {
    /// Construct finite, canonically ordered evidence. Evidence never authorizes a move.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        move_id: LegalMoveId,
        mut lanes: Vec<super::LaneScore>,
        final_score: FiniteScore,
        probability: FiniteScore,
        mut explanation_codes: Vec<RuleCode>,
        producer: ProducerIdentity,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        if !(0.0..=1.0).contains(&probability.get()) {
            return Err(GameboardContractError::InvalidContract {
                contract: "move evidence",
                reason: "probability must be finite and within [0, 1]".to_string(),
            });
        }
        lanes.sort_by_key(|lane| lane.lane);
        if lanes.windows(2).any(|pair| pair[0].lane == pair[1].lane) {
            return Err(GameboardContractError::InvalidContract {
                contract: "move evidence",
                reason: "duplicate evidence lanes are refused".to_string(),
            });
        }
        explanation_codes.sort();
        explanation_codes.dedup();
        let fields = [
            ("schema_version".to_string(), schema_version.to_string()),
            ("move_id".to_string(), move_id.as_str().to_string()),
            (
                "final_score".to_string(),
                final_score.get().to_bits().to_string(),
            ),
            (
                "probability".to_string(),
                probability.get().to_bits().to_string(),
            ),
            ("producer".to_string(), producer.as_str().to_string()),
        ]
        .into_iter()
        .chain(lanes.iter().enumerate().flat_map(|(index, lane)| {
            [
                (format!("lane.{index}.kind"), format!("{:?}", lane.lane)),
                (
                    format!("lane.{index}.score"),
                    lane.score.get().to_bits().to_string(),
                ),
            ]
        }))
        .chain(
            explanation_codes
                .iter()
                .enumerate()
                .map(|(index, code)| (format!("explanation.{index}"), code.as_str().to_string())),
        );
        let evidence_hash =
            GraphContentHash::new(hash_fields("semantic-gameboard-move-evidence-v1", fields))?;
        Ok(Self {
            schema_version,
            move_id,
            lanes,
            final_score,
            probability,
            explanation_codes,
            producer,
            evidence_hash,
        })
    }

    /// Project matching evidence from the legacy board-bound inference record.
    pub fn from_inference_evidence(
        move_id: LegalMoveId,
        candidate_id: &CanonicalCandidateId,
        evidence: &InferenceEvidence,
        producer: ProducerIdentity,
    ) -> Result<Self, GameboardContractError> {
        let candidate = evidence
            .ranked
            .iter()
            .find(|candidate| &candidate.candidate_id == candidate_id)
            .ok_or_else(|| {
                GameboardContractError::Compatibility(format!(
                    "legacy evidence has no candidate '{}'",
                    candidate_id.as_str()
                ))
            })?;
        Self::new(
            GAMEBOARD_SCHEMA_VERSION,
            move_id,
            candidate.lane_scores.clone(),
            candidate.final_score,
            FiniteScore::new(0.0)
                .map_err(|error| GameboardContractError::Compatibility(error.to_string()))?,
            Vec::new(),
            producer,
        )
    }

    /// Schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Move receiving the evidence.
    pub fn move_id(&self) -> &LegalMoveId {
        &self.move_id
    }
    /// Canonically ordered lanes.
    pub fn lanes(&self) -> &[super::LaneScore] {
        &self.lanes
    }
    /// Deterministic fused score.
    pub fn final_score(&self) -> FiniteScore {
        self.final_score
    }
    /// Calibrated probability.
    pub fn probability(&self) -> FiniteScore {
        self.probability
    }
    /// Governed explanation codes.
    pub fn explanation_codes(&self) -> &[RuleCode] {
        &self.explanation_codes
    }
    /// Evidence producer identity.
    pub fn producer(&self) -> &ProducerIdentity {
        &self.producer
    }
    /// Content identity.
    pub fn evidence_hash(&self) -> &GraphContentHash {
        &self.evidence_hash
    }
}

impl<'de> Deserialize<'de> for MoveEvidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            move_id: LegalMoveId,
            lanes: Vec<super::LaneScore>,
            final_score: FiniteScore,
            probability: FiniteScore,
            explanation_codes: Vec<RuleCode>,
            producer: ProducerIdentity,
            evidence_hash: GraphContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.move_id,
            wire.lanes,
            wire.final_score,
            wire.probability,
            wire.explanation_codes,
            wire.producer,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.evidence_hash != wire.evidence_hash {
            return Err(serde::de::Error::custom(
                "move evidence hash does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

/// Probability assigned to one currently legal move.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MoveProbability {
    move_id: LegalMoveId,
    probability: FiniteScore,
}

impl<'de> Deserialize<'de> for MoveProbability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            move_id: LegalMoveId,
            probability: FiniteScore,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.move_id, wire.probability).map_err(serde::de::Error::custom)
    }
}

impl MoveProbability {
    /// Construct a bounded probability.
    pub fn new(
        move_id: LegalMoveId,
        probability: FiniteScore,
    ) -> Result<Self, GameboardContractError> {
        if !(0.0..=1.0).contains(&probability.get()) {
            return Err(GameboardContractError::InvalidContract {
                contract: "move probability",
                reason: "probability must be within [0, 1]".to_string(),
            });
        }
        Ok(Self {
            move_id,
            probability,
        })
    }

    /// Move identity.
    pub fn move_id(&self) -> &LegalMoveId {
        &self.move_id
    }
    /// Bounded probability.
    pub fn probability(&self) -> FiniteScore {
        self.probability
    }
}

/// Non-authoritative governed motif hypothesis.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MotifHypothesis {
    motif_id: ContractText,
    probability: FiniteScore,
    provenance: ContractText,
}

impl<'de> Deserialize<'de> for MotifHypothesis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            motif_id: String,
            probability: FiniteScore,
            provenance: String,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.motif_id, wire.probability, wire.provenance)
            .map_err(serde::de::Error::custom)
    }
}

impl MotifHypothesis {
    /// Construct a bounded motif hypothesis.
    pub fn new(
        motif_id: impl Into<String>,
        probability: FiniteScore,
        provenance: impl Into<String>,
    ) -> Result<Self, GameboardContractError> {
        if !(0.0..=1.0).contains(&probability.get()) {
            return Err(GameboardContractError::InvalidContract {
                contract: "motif hypothesis",
                reason: "probability must be within [0, 1]".to_string(),
            });
        }
        Ok(Self {
            motif_id: ContractText::new("motif identity", motif_id)?,
            probability,
            provenance: ContractText::new("motif provenance", provenance)?,
        })
    }

    /// Governed motif identity.
    pub fn motif_id(&self) -> &str {
        self.motif_id.as_str()
    }
    /// Bounded probability.
    pub fn probability(&self) -> FiniteScore {
        self.probability
    }
    /// Producer or semantic-pack provenance.
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }
}

/// One unresolved, decision-relevant dimension in statistical belief.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct UnresolvedDimension {
    dimension_id: ContractText,
    option_codes: Vec<ContractText>,
}

impl<'de> Deserialize<'de> for UnresolvedDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            dimension_id: String,
            option_codes: Vec<String>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let original = wire.option_codes.clone();
        let admitted =
            Self::new(wire.dimension_id, wire.option_codes).map_err(serde::de::Error::custom)?;
        if admitted.option_codes().collect::<Vec<_>>() != original {
            return Err(serde::de::Error::custom(
                "unresolved options must be unique and canonically ordered",
            ));
        }
        Ok(admitted)
    }
}

impl UnresolvedDimension {
    /// Construct a dimension with at least two canonical alternatives.
    pub fn new(
        dimension_id: impl Into<String>,
        options: Vec<String>,
    ) -> Result<Self, GameboardContractError> {
        let mut option_codes = options
            .into_iter()
            .map(|value| ContractText::new("unresolved option code", value))
            .collect::<Result<Vec<_>, _>>()?;
        option_codes.sort();
        option_codes.dedup();
        if option_codes.len() < 2 {
            return Err(GameboardContractError::InvalidContract {
                contract: "unresolved dimension",
                reason: "must contain at least two distinct alternatives".to_string(),
            });
        }
        Ok(Self {
            dimension_id: ContractText::new("unresolved dimension identity", dimension_id)?,
            option_codes,
        })
    }

    /// Dimension identity.
    pub fn dimension_id(&self) -> &str {
        self.dimension_id.as_str()
    }
    /// Canonical alternatives.
    pub fn option_codes(&self) -> impl ExactSizeIterator<Item = &str> {
        self.option_codes.iter().map(ContractText::as_str)
    }
}

/// Non-authoritative, content-addressed belief about a recorded position.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DesignBelief {
    schema_version: u32,
    position_id: DesignStateId,
    likely_moves: Vec<MoveProbability>,
    motifs: Vec<MotifHypothesis>,
    unresolved_dimensions: Vec<UnresolvedDimension>,
    producer: ProducerIdentity,
    belief_hash: BeliefHash,
}

impl DesignBelief {
    /// Construct a canonical belief record. This type carries no transition authority.
    pub fn new(
        schema_version: u32,
        position_id: DesignStateId,
        mut likely_moves: Vec<MoveProbability>,
        mut motifs: Vec<MotifHypothesis>,
        mut unresolved_dimensions: Vec<UnresolvedDimension>,
        producer: ProducerIdentity,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        likely_moves.sort_by(|left, right| left.move_id.cmp(&right.move_id));
        if likely_moves
            .windows(2)
            .any(|pair| pair[0].move_id == pair[1].move_id)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "design belief",
                reason: "duplicate likely moves are refused".to_string(),
            });
        }
        let probability_sum = likely_moves
            .iter()
            .map(|value| value.probability.get())
            .sum::<f64>();
        if probability_sum > 1.000_000_001 {
            return Err(GameboardContractError::InvalidContract {
                contract: "design belief",
                reason: "likely-move probabilities must not sum above one".to_string(),
            });
        }
        motifs.sort_by(|left, right| left.motif_id.cmp(&right.motif_id));
        if motifs
            .windows(2)
            .any(|pair| pair[0].motif_id == pair[1].motif_id)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "design belief",
                reason: "duplicate motif hypotheses are refused".to_string(),
            });
        }
        unresolved_dimensions.sort();
        if unresolved_dimensions
            .windows(2)
            .any(|pair| pair[0].dimension_id == pair[1].dimension_id)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "design belief",
                reason: "duplicate unresolved dimensions are refused".to_string(),
            });
        }
        let fields = [
            ("schema_version".to_string(), schema_version.to_string()),
            ("position_id".to_string(), position_id.as_str().to_string()),
            ("producer".to_string(), producer.as_str().to_string()),
        ]
        .into_iter()
        .chain(likely_moves.iter().enumerate().flat_map(|(index, value)| {
            [
                (
                    format!("move.{index}.id"),
                    value.move_id().as_str().to_string(),
                ),
                (
                    format!("move.{index}.probability"),
                    value.probability().get().to_bits().to_string(),
                ),
            ]
        }))
        .chain(motifs.iter().enumerate().flat_map(|(index, value)| {
            [
                (format!("motif.{index}.id"), value.motif_id().to_string()),
                (
                    format!("motif.{index}.probability"),
                    value.probability().get().to_bits().to_string(),
                ),
                (
                    format!("motif.{index}.provenance"),
                    value.provenance().to_string(),
                ),
            ]
        }))
        .chain(
            unresolved_dimensions
                .iter()
                .enumerate()
                .flat_map(|(index, value)| {
                    std::iter::once((
                        format!("dimension.{index}.id"),
                        value.dimension_id().to_string(),
                    ))
                    .chain(value.option_codes().enumerate().map(
                        move |(option_index, option)| {
                            (
                                format!("dimension.{index}.option.{option_index}"),
                                option.to_string(),
                            )
                        },
                    ))
                }),
        );
        let belief_hash = BeliefHash::new(hash_fields("semantic-gameboard-belief-v1", fields))?;
        Ok(Self {
            schema_version,
            position_id,
            likely_moves,
            motifs,
            unresolved_dimensions,
            producer,
            belief_hash,
        })
    }

    /// Schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Position observed by the producer.
    pub fn position_id(&self) -> &DesignStateId {
        &self.position_id
    }
    /// Canonical legal-move probabilities.
    pub fn likely_moves(&self) -> &[MoveProbability] {
        &self.likely_moves
    }
    /// Canonical motif hypotheses.
    pub fn motifs(&self) -> &[MotifHypothesis] {
        &self.motifs
    }
    /// Canonical unresolved dimensions.
    pub fn unresolved_dimensions(&self) -> &[UnresolvedDimension] {
        &self.unresolved_dimensions
    }
    /// Producer identity.
    pub fn producer(&self) -> &ProducerIdentity {
        &self.producer
    }
    /// Content identity.
    pub fn belief_hash(&self) -> &BeliefHash {
        &self.belief_hash
    }
}

impl<'de> Deserialize<'de> for DesignBelief {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            position_id: DesignStateId,
            likely_moves: Vec<MoveProbability>,
            motifs: Vec<MotifHypothesis>,
            unresolved_dimensions: Vec<UnresolvedDimension>,
            producer: ProducerIdentity,
            belief_hash: BeliefHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.position_id,
            wire.likely_moves,
            wire.motifs,
            wire.unresolved_dimensions,
            wire.producer,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.belief_hash != wire.belief_hash {
            return Err(serde::de::Error::custom(format!(
                "belief hash does not match canonical content: expected {}, observed {}",
                admitted.belief_hash.as_str(),
                wire.belief_hash.as_str()
            )));
        }
        Ok(admitted)
    }
}

/// Why no user answer was observed for a recorded game interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameTurnAnswerAbsenceReason {
    NotRequested,
    NoResponse,
    Abandoned,
    SystemInterrupted,
}

/// Closed shape of an answer captured at the semantic-game boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameTurnAnswerKind {
    NotObserved,
    Clarification,
    MoveArguments,
    Feedback,
    Unstructured,
}

/// Typed user answer bound to the board on which it was observed.
///
/// Raw/private content remains in the charter-governed capture sink. This stable
/// contract carries its content identity and the semantic facts needed for replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameTurnAnswer {
    kind: GameTurnAnswerKind,
    answer_hash: Option<GraphContentHash>,
    absence_reason: Option<GameTurnAnswerAbsenceReason>,
    clarification_dimension: Option<GameClarificationDimension>,
    selected_moves: Vec<LegalMoveId>,
    focus: Option<DesignFocus>,
    move_id: Option<LegalMoveId>,
    arguments: Vec<MoveArgument>,
    feedback_kind: Option<FeedbackOptionKind>,
}

impl GameTurnAnswer {
    #[allow(clippy::too_many_arguments)]
    fn admit(
        kind: GameTurnAnswerKind,
        answer_hash: Option<GraphContentHash>,
        absence_reason: Option<GameTurnAnswerAbsenceReason>,
        clarification_dimension: Option<GameClarificationDimension>,
        mut selected_moves: Vec<LegalMoveId>,
        focus: Option<DesignFocus>,
        move_id: Option<LegalMoveId>,
        mut arguments: Vec<MoveArgument>,
        feedback_kind: Option<FeedbackOptionKind>,
    ) -> Result<Self, GameboardContractError> {
        selected_moves.sort();
        if selected_moves.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn answer",
                reason: "selected moves must be unique".to_string(),
            });
        }
        arguments.sort_by(|left, right| left.name.cmp(&right.name));
        if arguments
            .windows(2)
            .any(|pair| pair[0].name == pair[1].name)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn answer",
                reason: "answered arguments must have unique names".to_string(),
            });
        }
        let invalid = |reason: &str| GameboardContractError::InvalidContract {
            contract: "game turn answer",
            reason: reason.to_string(),
        };
        match kind {
            GameTurnAnswerKind::NotObserved
                if answer_hash.is_some()
                    || absence_reason.is_none()
                    || clarification_dimension.is_some()
                    || !selected_moves.is_empty()
                    || focus.is_some()
                    || move_id.is_some()
                    || !arguments.is_empty()
                    || feedback_kind.is_some() =>
            {
                return Err(invalid(
                    "an absent answer may carry only its absence reason",
                ));
            }
            GameTurnAnswerKind::Clarification
                if answer_hash.is_none()
                    || absence_reason.is_some()
                    || clarification_dimension.is_none()
                    || (selected_moves.is_empty() && focus.is_none())
                    || move_id.is_some()
                    || !arguments.is_empty()
                    || feedback_kind.is_some() =>
            {
                return Err(invalid(
                    "a clarification answer requires content, one dimension and a move or focus",
                ));
            }
            GameTurnAnswerKind::MoveArguments
                if answer_hash.is_none()
                    || absence_reason.is_some()
                    || clarification_dimension.is_some()
                    || !selected_moves.is_empty()
                    || focus.is_some()
                    || move_id.is_none()
                    || arguments.is_empty()
                    || feedback_kind.is_some() =>
            {
                return Err(invalid(
                    "a move-argument answer requires content, one move and typed arguments",
                ));
            }
            GameTurnAnswerKind::Feedback
                if answer_hash.is_none()
                    || absence_reason.is_some()
                    || clarification_dimension.is_some()
                    || !selected_moves.is_empty()
                    || focus.is_some()
                    || !arguments.is_empty()
                    || feedback_kind.is_none() =>
            {
                return Err(invalid(
                    "a feedback answer requires content and a typed feedback option",
                ));
            }
            GameTurnAnswerKind::Unstructured
                if answer_hash.is_none()
                    || absence_reason.is_some()
                    || clarification_dimension.is_some()
                    || !selected_moves.is_empty()
                    || focus.is_some()
                    || move_id.is_some()
                    || !arguments.is_empty()
                    || feedback_kind.is_some() =>
            {
                return Err(invalid(
                    "an unstructured answer may carry only its content identity",
                ));
            }
            _ => {}
        }
        Ok(Self {
            kind,
            answer_hash,
            absence_reason,
            clarification_dimension,
            selected_moves,
            focus,
            move_id,
            arguments,
            feedback_kind,
        })
    }

    pub fn not_observed(reason: GameTurnAnswerAbsenceReason) -> Self {
        Self::admit(
            GameTurnAnswerKind::NotObserved,
            None,
            Some(reason),
            None,
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
        )
        .expect("the fixed not-observed answer shape is valid")
    }

    pub fn clarification(
        answer_hash: GraphContentHash,
        dimension: GameClarificationDimension,
        selected_moves: Vec<LegalMoveId>,
        focus: Option<DesignFocus>,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            GameTurnAnswerKind::Clarification,
            Some(answer_hash),
            None,
            Some(dimension),
            selected_moves,
            focus,
            None,
            Vec::new(),
            None,
        )
    }

    pub fn move_arguments(
        answer_hash: GraphContentHash,
        move_id: LegalMoveId,
        arguments: Vec<MoveArgument>,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            GameTurnAnswerKind::MoveArguments,
            Some(answer_hash),
            None,
            None,
            Vec::new(),
            None,
            Some(move_id),
            arguments,
            None,
        )
    }

    pub fn feedback(
        answer_hash: GraphContentHash,
        kind: FeedbackOptionKind,
        move_id: Option<LegalMoveId>,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            GameTurnAnswerKind::Feedback,
            Some(answer_hash),
            None,
            None,
            Vec::new(),
            None,
            move_id,
            Vec::new(),
            Some(kind),
        )
    }

    pub fn unstructured(answer_hash: GraphContentHash) -> Self {
        Self::admit(
            GameTurnAnswerKind::Unstructured,
            Some(answer_hash),
            None,
            None,
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
        )
        .expect("the fixed unstructured answer shape is valid")
    }

    fn validate_for_position(
        &self,
        position: &DesignPosition,
    ) -> Result<(), GameboardContractError> {
        let on_board = |move_id: &LegalMoveId| {
            position
                .legal_moves()
                .iter()
                .find(|legal_move| legal_move.move_id() == move_id)
        };
        for move_id in self.selected_moves.iter().chain(self.move_id.iter()) {
            if on_board(move_id).is_none() {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn answer",
                    reason: format!(
                        "move '{}' is absent from the recorded position",
                        move_id.as_str()
                    ),
                });
            }
        }
        if self.kind == GameTurnAnswerKind::MoveArguments {
            let legal_move = on_board(self.move_id.as_ref().expect("shape validated"))
                .expect("board membership checked above");
            for answer in &self.arguments {
                let Some(declared) = legal_move
                    .arguments()
                    .iter()
                    .find(|argument| argument.name() == answer.name())
                else {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn answer",
                        reason: format!(
                            "argument '{}' is absent from the selected move",
                            answer.name()
                        ),
                    });
                };
                if declared.kind() != answer.kind() {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn answer",
                        reason: format!("argument '{}' has the wrong value kind", answer.name()),
                    });
                }
            }
        }
        Ok(())
    }

    fn canonical_hash(&self) -> String {
        let fields = [
            ("kind".to_string(), format!("{:?}", self.kind)),
            (
                "answer_hash".to_string(),
                self.answer_hash
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "absence_reason".to_string(),
                self.absence_reason
                    .map_or_else(String::new, |value| format!("{value:?}")),
            ),
            (
                "clarification_dimension".to_string(),
                self.clarification_dimension
                    .map_or_else(String::new, |value| format!("{value:?}")),
            ),
            (
                "focus".to_string(),
                self.focus.as_ref().map_or_else(String::new, hash_focus),
            ),
            (
                "move_id".to_string(),
                self.move_id
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "feedback_kind".to_string(),
                self.feedback_kind
                    .map_or_else(String::new, |value| format!("{value:?}")),
            ),
        ]
        .into_iter()
        .chain(
            self.selected_moves
                .iter()
                .enumerate()
                .map(|(index, value)| (format!("selected.{index}"), value.as_str().to_string())),
        )
        .chain(
            self.arguments
                .iter()
                .enumerate()
                .flat_map(|(index, value)| {
                    [
                        (format!("argument.{index}.name"), value.name().to_string()),
                        (
                            format!("argument.{index}.kind"),
                            format!("{:?}", value.kind()),
                        ),
                        (
                            format!("argument.{index}.value"),
                            value.value().map_or_else(String::new, canonical_slot_value),
                        ),
                        (
                            format!("argument.{index}.provenance"),
                            value.provenance().unwrap_or_default().to_string(),
                        ),
                    ]
                }),
        );
        hash_fields("semantic-gameboard-turn-answer-v1", fields)
    }

    pub fn kind(&self) -> GameTurnAnswerKind {
        self.kind
    }
    pub fn answer_hash(&self) -> Option<&GraphContentHash> {
        self.answer_hash.as_ref()
    }
    pub fn absence_reason(&self) -> Option<GameTurnAnswerAbsenceReason> {
        self.absence_reason
    }
    pub fn clarification_dimension(&self) -> Option<GameClarificationDimension> {
        self.clarification_dimension
    }
    pub fn selected_moves(&self) -> &[LegalMoveId] {
        &self.selected_moves
    }
    pub fn focus(&self) -> Option<&DesignFocus> {
        self.focus.as_ref()
    }
    pub fn move_id(&self) -> Option<&LegalMoveId> {
        self.move_id.as_ref()
    }
    pub fn arguments(&self) -> &[MoveArgument] {
        &self.arguments
    }
    pub fn feedback_kind(&self) -> Option<FeedbackOptionKind> {
        self.feedback_kind
    }
}

impl<'de> Deserialize<'de> for GameTurnAnswer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: GameTurnAnswerKind,
            answer_hash: Option<GraphContentHash>,
            absence_reason: Option<GameTurnAnswerAbsenceReason>,
            clarification_dimension: Option<GameClarificationDimension>,
            selected_moves: Vec<LegalMoveId>,
            focus: Option<DesignFocus>,
            move_id: Option<LegalMoveId>,
            arguments: Vec<MoveArgument>,
            feedback_kind: Option<FeedbackOptionKind>,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::admit(
            wire.kind,
            wire.answer_hash,
            wire.absence_reason,
            wire.clarification_dimension,
            wire.selected_moves,
            wire.focus,
            wire.move_id,
            wire.arguments,
            wire.feedback_kind,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Typed compiler result recorded without granting the model transition authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameTurnCompilerResultKind {
    NotRequested,
    Admitted,
    Refused,
    SystemFailure,
}

/// Whether this observed turn had reached a terminal move attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameTurnAttemptKind {
    NotAttempted,
    Terminal,
}

/// Explicit attempt state for proposal/clarification turns and terminal attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameTurnAttempt {
    kind: GameTurnAttemptKind,
    receipt: Option<MoveAttemptReceipt>,
}

impl GameTurnAttempt {
    pub fn not_attempted() -> Self {
        Self {
            kind: GameTurnAttemptKind::NotAttempted,
            receipt: None,
        }
    }

    pub fn terminal(receipt: MoveAttemptReceipt) -> Self {
        Self {
            kind: GameTurnAttemptKind::Terminal,
            receipt: Some(receipt),
        }
    }

    fn admit(
        kind: GameTurnAttemptKind,
        receipt: Option<MoveAttemptReceipt>,
    ) -> Result<Self, GameboardContractError> {
        if (kind == GameTurnAttemptKind::Terminal) != receipt.is_some() {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn attempt",
                reason: "only a terminal attempt may carry exactly one receipt".to_string(),
            });
        }
        Ok(Self { kind, receipt })
    }

    pub fn kind(&self) -> GameTurnAttemptKind {
        self.kind
    }

    pub fn receipt(&self) -> Option<&MoveAttemptReceipt> {
        self.receipt.as_ref()
    }
}

impl<'de> Deserialize<'de> for GameTurnAttempt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: GameTurnAttemptKind,
            receipt: Option<MoveAttemptReceipt>,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::admit(wire.kind, wire.receipt).map_err(serde::de::Error::custom)
    }
}

/// Validated compiler result for one game turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameTurnCompilerResult {
    kind: GameTurnCompilerResultKind,
    delta_hash: Option<GraphDeltaHash>,
    result_graph_hash: Option<GraphContentHash>,
    receipt_hash: Option<GraphContentHash>,
    diagnostic_hash: Option<GraphContentHash>,
}

impl GameTurnCompilerResult {
    fn admit(
        kind: GameTurnCompilerResultKind,
        delta_hash: Option<GraphDeltaHash>,
        result_graph_hash: Option<GraphContentHash>,
        receipt_hash: Option<GraphContentHash>,
        diagnostic_hash: Option<GraphContentHash>,
    ) -> Result<Self, GameboardContractError> {
        let invalid = |reason: &str| GameboardContractError::InvalidContract {
            contract: "game turn compiler result",
            reason: reason.to_string(),
        };
        match kind {
            GameTurnCompilerResultKind::NotRequested
                if delta_hash.is_some()
                    || result_graph_hash.is_some()
                    || receipt_hash.is_some()
                    || diagnostic_hash.is_some() =>
            {
                return Err(invalid("not-requested may not carry compiler output"));
            }
            GameTurnCompilerResultKind::Admitted
                if delta_hash.is_none()
                    || result_graph_hash.is_none()
                    || receipt_hash.is_none()
                    || diagnostic_hash.is_some() =>
            {
                return Err(invalid(
                    "admission requires delta, resulting graph and compiler receipt identities",
                ));
            }
            GameTurnCompilerResultKind::Refused | GameTurnCompilerResultKind::SystemFailure
                if delta_hash.is_some()
                    || result_graph_hash.is_some()
                    || receipt_hash.is_some()
                    || diagnostic_hash.is_none() =>
            {
                return Err(invalid(
                    "refusal/failure requires one diagnostic identity and no admitted result",
                ));
            }
            _ => {}
        }
        Ok(Self {
            kind,
            delta_hash,
            result_graph_hash,
            receipt_hash,
            diagnostic_hash,
        })
    }

    pub fn not_requested() -> Self {
        Self::admit(
            GameTurnCompilerResultKind::NotRequested,
            None,
            None,
            None,
            None,
        )
        .expect("the fixed not-requested compiler result is valid")
    }

    pub fn admitted(
        delta_hash: GraphDeltaHash,
        result_graph_hash: GraphContentHash,
        receipt_hash: GraphContentHash,
    ) -> Result<Self, GameboardContractError> {
        Self::admit(
            GameTurnCompilerResultKind::Admitted,
            Some(delta_hash),
            Some(result_graph_hash),
            Some(receipt_hash),
            None,
        )
    }

    pub fn refused(diagnostic_hash: GraphContentHash) -> Self {
        Self::admit(
            GameTurnCompilerResultKind::Refused,
            None,
            None,
            None,
            Some(diagnostic_hash),
        )
        .expect("the fixed refused compiler result is valid")
    }

    pub fn system_failure(failure_hash: GraphContentHash) -> Self {
        Self::admit(
            GameTurnCompilerResultKind::SystemFailure,
            None,
            None,
            None,
            Some(failure_hash),
        )
        .expect("the fixed failed compiler result is valid")
    }

    fn canonical_hash(&self) -> String {
        hash_fields(
            "semantic-gameboard-compiler-result-v1",
            [
                ("kind".to_string(), format!("{:?}", self.kind)),
                (
                    "delta".to_string(),
                    self.delta_hash
                        .as_ref()
                        .map_or_else(String::new, |value| value.as_str().to_string()),
                ),
                (
                    "result_graph".to_string(),
                    self.result_graph_hash
                        .as_ref()
                        .map_or_else(String::new, |value| value.as_str().to_string()),
                ),
                (
                    "receipt".to_string(),
                    self.receipt_hash
                        .as_ref()
                        .map_or_else(String::new, |value| value.as_str().to_string()),
                ),
                (
                    "diagnostic".to_string(),
                    self.diagnostic_hash
                        .as_ref()
                        .map_or_else(String::new, |value| value.as_str().to_string()),
                ),
            ],
        )
    }

    pub fn kind(&self) -> GameTurnCompilerResultKind {
        self.kind
    }
    pub fn delta_hash(&self) -> Option<&GraphDeltaHash> {
        self.delta_hash.as_ref()
    }
    pub fn result_graph_hash(&self) -> Option<&GraphContentHash> {
        self.result_graph_hash.as_ref()
    }
    pub fn receipt_hash(&self) -> Option<&GraphContentHash> {
        self.receipt_hash.as_ref()
    }
    pub fn diagnostic_hash(&self) -> Option<&GraphContentHash> {
        self.diagnostic_hash.as_ref()
    }
}

impl<'de> Deserialize<'de> for GameTurnCompilerResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: GameTurnCompilerResultKind,
            delta_hash: Option<GraphDeltaHash>,
            result_graph_hash: Option<GraphContentHash>,
            receipt_hash: Option<GraphContentHash>,
            diagnostic_hash: Option<GraphContentHash>,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::admit(
            wire.kind,
            wire.delta_hash,
            wire.result_graph_hash,
            wire.receipt_hash,
            wire.diagnostic_hash,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Complete, consented game-level record for one attempted interaction.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GameTurnRecord {
    schema_version: u32,
    session_id: GameSessionId,
    turn_id: DesignTurnId,
    sequence: u64,
    observed_at_epoch_ms: u64,
    semantic_family: SemanticFamilyId,
    risk_class: super::HarmClass,
    input_hash: GraphContentHash,
    position: DesignPosition,
    evidence: Vec<MoveEvidence>,
    belief: DesignBelief,
    disposition: GameDisposition,
    answer: GameTurnAnswer,
    chosen_move: Option<LegalMoveId>,
    delta: Option<GraphDeltaPreview>,
    attempt: GameTurnAttempt,
    compiler_result: GameTurnCompilerResult,
    related_attempts: Vec<MoveAttemptReceipt>,
    record_hash: GameTurnRecordHash,
}

impl GameTurnRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        session_id: GameSessionId,
        turn_id: DesignTurnId,
        sequence: u64,
        observed_at_epoch_ms: u64,
        semantic_family: SemanticFamilyId,
        risk_class: super::HarmClass,
        input_hash: GraphContentHash,
        position: DesignPosition,
        mut evidence: Vec<MoveEvidence>,
        belief: DesignBelief,
        disposition: GameDisposition,
        answer: GameTurnAnswer,
        chosen_move: Option<LegalMoveId>,
        delta: Option<GraphDeltaPreview>,
        attempt: GameTurnAttempt,
        compiler_result: GameTurnCompilerResult,
        mut related_attempts: Vec<MoveAttemptReceipt>,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        evidence.sort_by(|left, right| left.move_id.cmp(&right.move_id));
        let evidence_moves = evidence
            .iter()
            .map(|value| value.move_id().clone())
            .collect::<Vec<_>>();
        let position_moves = position
            .legal_moves()
            .iter()
            .map(|value| value.move_id().clone())
            .collect::<Vec<_>>();
        if evidence_moves != position_moves {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn record",
                reason: "evidence must cover every legal move exactly once".to_string(),
            });
        }
        if belief.position_id() != position.state_id()
            || belief
                .likely_moves()
                .iter()
                .any(|value| !position_moves.contains(value.move_id()))
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn record",
                reason: "belief is stale or names a move absent from the position".to_string(),
            });
        }
        disposition.validate_for_position(&position)?;
        answer.validate_for_position(&position)?;
        let chosen_legal = chosen_move
            .as_ref()
            .map(|move_id| {
                position
                    .legal_moves()
                    .iter()
                    .find(|legal_move| legal_move.move_id() == move_id)
                    .ok_or_else(|| GameboardContractError::InvalidContract {
                        contract: "game turn record",
                        reason: format!(
                            "chosen move '{}' is absent from the position",
                            move_id.as_str()
                        ),
                    })
            })
            .transpose()?;
        if let Some(receipt) = attempt.receipt() {
            if receipt.position_id() != position.state_id() {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "attempt receipt belongs to a different position".to_string(),
                });
            }
            if let (Some(attempted), Some(chosen)) =
                (receipt.attempted_move(), chosen_move.as_ref())
            {
                if attempted != chosen {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn record",
                        reason: "chosen move and attempted move disagree".to_string(),
                    });
                }
            }
        }
        if let Some(disposition_attempt) = disposition.attempt_receipt() {
            if attempt.receipt().map(MoveAttemptReceipt::receipt_hash)
                != Some(disposition_attempt.receipt_hash())
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "disposition and turn name different terminal attempts".to_string(),
                });
            }
        }
        if let Some(delta) = &delta {
            if delta.from_graph() != position.graph_hash() {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "delta preview starts from a different graph".to_string(),
                });
            }
            let Some(chosen_legal) = chosen_legal else {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "a delta preview requires a chosen legal move".to_string(),
                });
            };
            if chosen_legal.preview().map(GraphDeltaPreview::delta_hash) != Some(delta.delta_hash())
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "captured delta differs from the legal move preview".to_string(),
                });
            }
        }
        match compiler_result.kind() {
            GameTurnCompilerResultKind::Admitted => {
                let Some(attempt_receipt) = attempt.receipt() else {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn record",
                        reason: "compiler admission requires a terminal attempt".to_string(),
                    });
                };
                let Some(delta) = &delta else {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn record",
                        reason: "compiler admission requires the exact previewed delta".to_string(),
                    });
                };
                if compiler_result.delta_hash() != Some(delta.delta_hash())
                    || compiler_result.result_graph_hash() == Some(position.graph_hash())
                    || attempt_receipt.outcome() != MoveAttemptOutcome::Applied
                {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn record",
                        reason: "compiler admission, graph transition and attempt outcome disagree"
                            .to_string(),
                    });
                }
            }
            GameTurnCompilerResultKind::Refused
                if attempt.receipt().map(MoveAttemptReceipt::outcome)
                    != Some(MoveAttemptOutcome::CompilerRefused) =>
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "compiler refusal requires a compiler-refused attempt".to_string(),
                });
            }
            GameTurnCompilerResultKind::SystemFailure
                if attempt.receipt().map(MoveAttemptReceipt::outcome)
                    != Some(MoveAttemptOutcome::SystemFailure) =>
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "compiler failure requires a system-failure attempt".to_string(),
                });
            }
            GameTurnCompilerResultKind::NotRequested
                if attempt.receipt().is_some_and(|receipt| {
                    matches!(
                        receipt.outcome(),
                        MoveAttemptOutcome::Applied
                            | MoveAttemptOutcome::CompilerRefused
                            | MoveAttemptOutcome::SystemFailure
                    )
                }) =>
            {
                return Err(GameboardContractError::InvalidContract {
                    contract: "game turn record",
                    reason: "terminal compiler outcome cannot be recorded as not requested"
                        .to_string(),
                });
            }
            _ => {}
        }
        related_attempts.sort_by(|left, right| left.attempt_id.cmp(&right.attempt_id));
        if related_attempts
            .windows(2)
            .any(|pair| pair[0].attempt_id() == pair[1].attempt_id())
            || related_attempts.iter().any(|receipt| {
                attempt
                    .receipt()
                    .is_some_and(|attempt| receipt.attempt_id() == attempt.attempt_id())
            })
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn record",
                reason: "related attempts must have unique identities".to_string(),
            });
        }
        if !related_attempts.is_empty() && attempt.receipt().is_none() {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn record",
                reason: "a turn without a terminal attempt cannot carry related attempt history"
                    .to_string(),
            });
        }
        let history = related_attempts
            .iter()
            .cloned()
            .chain(attempt.receipt().cloned())
            .collect::<Vec<_>>();
        validate_attempt_history(&history)?;

        let mut record = Self {
            schema_version,
            session_id,
            turn_id,
            sequence,
            observed_at_epoch_ms,
            semantic_family,
            risk_class,
            input_hash,
            position,
            evidence,
            belief,
            disposition,
            answer,
            chosen_move,
            delta,
            attempt,
            compiler_result,
            related_attempts,
            record_hash: GameTurnRecordHash::new("0".repeat(64))?,
        };
        record.record_hash = record.canonical_hash()?;
        Ok(record)
    }

    fn canonical_hash(&self) -> Result<GameTurnRecordHash, GameboardContractError> {
        let fields = [
            (
                "schema_version".to_string(),
                self.schema_version.to_string(),
            ),
            ("session".to_string(), self.session_id.as_str().to_string()),
            ("turn".to_string(), self.turn_id.as_str().to_string()),
            ("sequence".to_string(), self.sequence.to_string()),
            (
                "observed_at_epoch_ms".to_string(),
                self.observed_at_epoch_ms.to_string(),
            ),
            (
                "semantic_family".to_string(),
                self.semantic_family.as_str().to_string(),
            ),
            ("risk_class".to_string(), format!("{:?}", self.risk_class)),
            ("input".to_string(), self.input_hash.as_str().to_string()),
            (
                "position".to_string(),
                self.position.state_id().as_str().to_string(),
            ),
            (
                "belief".to_string(),
                self.belief.belief_hash().as_str().to_string(),
            ),
            (
                "disposition".to_string(),
                self.disposition.disposition_hash().as_str().to_string(),
            ),
            ("answer".to_string(), self.answer.canonical_hash()),
            (
                "chosen_move".to_string(),
                self.chosen_move
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "delta".to_string(),
                self.delta
                    .as_ref()
                    .map_or_else(String::new, |value| value.delta_hash().as_str().to_string()),
            ),
            (
                "attempt_kind".to_string(),
                format!("{:?}", self.attempt.kind()),
            ),
            (
                "attempt_receipt".to_string(),
                self.attempt.receipt().map_or_else(String::new, |value| {
                    value.receipt_hash().as_str().to_string()
                }),
            ),
            (
                "compiler".to_string(),
                self.compiler_result.canonical_hash(),
            ),
        ]
        .into_iter()
        .chain(self.evidence.iter().enumerate().map(|(index, value)| {
            (
                format!("evidence.{index}"),
                value.evidence_hash().as_str().to_string(),
            )
        }))
        .chain(
            self.related_attempts
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    (
                        format!("related_attempt.{index}"),
                        value.receipt_hash().as_str().to_string(),
                    )
                }),
        );
        GameTurnRecordHash::new(hash_fields("semantic-gameboard-turn-record-v1", fields))
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    pub fn session_id(&self) -> &GameSessionId {
        &self.session_id
    }
    pub fn turn_id(&self) -> &DesignTurnId {
        &self.turn_id
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn observed_at_epoch_ms(&self) -> u64 {
        self.observed_at_epoch_ms
    }
    pub fn semantic_family(&self) -> &SemanticFamilyId {
        &self.semantic_family
    }
    pub fn risk_class(&self) -> super::HarmClass {
        self.risk_class
    }
    pub fn input_hash(&self) -> &GraphContentHash {
        &self.input_hash
    }
    pub fn position(&self) -> &DesignPosition {
        &self.position
    }
    pub fn evidence(&self) -> &[MoveEvidence] {
        &self.evidence
    }
    pub fn belief(&self) -> &DesignBelief {
        &self.belief
    }
    pub fn disposition(&self) -> &GameDisposition {
        &self.disposition
    }
    pub fn answer(&self) -> &GameTurnAnswer {
        &self.answer
    }
    pub fn chosen_move(&self) -> Option<&LegalMoveId> {
        self.chosen_move.as_ref()
    }
    pub fn delta(&self) -> Option<&GraphDeltaPreview> {
        self.delta.as_ref()
    }
    pub fn attempt(&self) -> &GameTurnAttempt {
        &self.attempt
    }
    pub fn compiler_result(&self) -> &GameTurnCompilerResult {
        &self.compiler_result
    }
    /// Bounded retained history needed to validate correction links for this turn.
    pub fn related_attempts(&self) -> &[MoveAttemptReceipt] {
        &self.related_attempts
    }
    pub fn record_hash(&self) -> &GameTurnRecordHash {
        &self.record_hash
    }
}

impl<'de> Deserialize<'de> for GameTurnRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            session_id: GameSessionId,
            turn_id: DesignTurnId,
            sequence: u64,
            observed_at_epoch_ms: u64,
            semantic_family: SemanticFamilyId,
            risk_class: super::HarmClass,
            input_hash: GraphContentHash,
            position: DesignPosition,
            evidence: Vec<MoveEvidence>,
            belief: DesignBelief,
            disposition: GameDisposition,
            answer: GameTurnAnswer,
            chosen_move: Option<LegalMoveId>,
            delta: Option<GraphDeltaPreview>,
            attempt: GameTurnAttempt,
            compiler_result: GameTurnCompilerResult,
            related_attempts: Vec<MoveAttemptReceipt>,
            record_hash: GameTurnRecordHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let record_hash = wire.record_hash;
        let admitted = Self::new(
            wire.schema_version,
            wire.session_id,
            wire.turn_id,
            wire.sequence,
            wire.observed_at_epoch_ms,
            wire.semantic_family,
            wire.risk_class,
            wire.input_hash,
            wire.position,
            wire.evidence,
            wire.belief,
            wire.disposition,
            wire.answer,
            wire.chosen_move,
            wire.delta,
            wire.attempt,
            wire.compiler_result,
            wire.related_attempts,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.record_hash != record_hash {
            return Err(serde::de::Error::custom(
                "game turn record hash does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

/// Human judgement of the role an observed interaction played.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameTurnJudgement {
    ExploratoryHumanAttempt,
    AcceptedMove,
    AccidentalMove,
    SystemMisinterpretation,
}

/// Structured intended move, including explicitly unrepresentable intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum IntendedMove {
    None,
    OnBoard { move_id: LegalMoveId },
    OffBoard { semantic_code: MessageKey },
}

/// Separate, append-only operator judgement linked to a captured game turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GameTurnAdjudication {
    schema_version: u32,
    record_hash: GameTurnRecordHash,
    adjudicator: ContractText,
    adjudicated_at_epoch_ms: u64,
    judgement: GameTurnJudgement,
    intended_move: IntendedMove,
    intended_anchor: Option<GraphElementRef>,
    intended_arguments: Vec<MoveArgument>,
    intended_motif: Option<ContractText>,
    acceptable_clarifications: Vec<GameClarificationDimension>,
    acceptable_feedback: Vec<FeedbackOptionKind>,
    note_hash: Option<GraphContentHash>,
    adjudication_hash: GameTurnAdjudicationHash,
}

impl GameTurnAdjudication {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        record: &GameTurnRecord,
        adjudicator: impl Into<String>,
        adjudicated_at_epoch_ms: u64,
        judgement: GameTurnJudgement,
        intended_move: IntendedMove,
        intended_anchor: Option<GraphElementRef>,
        intended_arguments: Vec<MoveArgument>,
        intended_motif: Option<String>,
        acceptable_clarifications: Vec<GameClarificationDimension>,
        acceptable_feedback: Vec<FeedbackOptionKind>,
        note_hash: Option<GraphContentHash>,
    ) -> Result<Self, GameboardContractError> {
        let admitted = Self::admit(
            GAMEBOARD_SCHEMA_VERSION,
            record.record_hash().clone(),
            adjudicator.into(),
            adjudicated_at_epoch_ms,
            judgement,
            intended_move,
            intended_anchor,
            intended_arguments,
            intended_motif,
            acceptable_clarifications,
            acceptable_feedback,
            note_hash,
        )?;
        admitted.validate_for_record(record)?;
        Ok(admitted)
    }

    #[allow(clippy::too_many_arguments)]
    fn admit(
        schema_version: u32,
        record_hash: GameTurnRecordHash,
        adjudicator: String,
        adjudicated_at_epoch_ms: u64,
        judgement: GameTurnJudgement,
        intended_move: IntendedMove,
        intended_anchor: Option<GraphElementRef>,
        mut intended_arguments: Vec<MoveArgument>,
        intended_motif: Option<String>,
        mut acceptable_clarifications: Vec<GameClarificationDimension>,
        mut acceptable_feedback: Vec<FeedbackOptionKind>,
        note_hash: Option<GraphContentHash>,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        intended_arguments.sort_by(|left, right| left.name.cmp(&right.name));
        if intended_arguments
            .windows(2)
            .any(|pair| pair[0].name == pair[1].name)
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn adjudication",
                reason: "intended arguments must have unique names".to_string(),
            });
        }
        acceptable_clarifications.sort();
        acceptable_clarifications.dedup();
        acceptable_feedback.sort();
        acceptable_feedback.dedup();
        if judgement == GameTurnJudgement::AcceptedMove
            && !matches!(intended_move, IntendedMove::OnBoard { .. })
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn adjudication",
                reason: "an accepted move requires an explicit on-board intended move".to_string(),
            });
        }
        if judgement == GameTurnJudgement::SystemMisinterpretation
            && matches!(intended_move, IntendedMove::None)
            && intended_anchor.is_none()
            && intended_arguments.is_empty()
            && intended_motif.is_none()
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn adjudication",
                reason: "a system misinterpretation must state the corrected intent".to_string(),
            });
        }
        let mut admitted = Self {
            schema_version,
            record_hash,
            adjudicator: ContractText::new("game turn adjudicator", adjudicator)?,
            adjudicated_at_epoch_ms,
            judgement,
            intended_move,
            intended_anchor,
            intended_arguments,
            intended_motif: intended_motif
                .map(|value| ContractText::new("intended motif", value))
                .transpose()?,
            acceptable_clarifications,
            acceptable_feedback,
            note_hash,
            adjudication_hash: GameTurnAdjudicationHash::new("0".repeat(64))?,
        };
        admitted.adjudication_hash = admitted.canonical_hash()?;
        Ok(admitted)
    }

    pub fn validate_for_record(
        &self,
        record: &GameTurnRecord,
    ) -> Result<(), GameboardContractError> {
        if self.record_hash != *record.record_hash() {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn adjudication",
                reason: "adjudication belongs to a different captured turn".to_string(),
            });
        }
        let intended_legal = match &self.intended_move {
            IntendedMove::OnBoard { move_id } => Some(
                record
                    .position()
                    .legal_moves()
                    .iter()
                    .find(|legal_move| legal_move.move_id() == move_id)
                    .ok_or_else(|| GameboardContractError::InvalidContract {
                        contract: "game turn adjudication",
                        reason: format!(
                            "intended move '{}' was not on the recorded board",
                            move_id.as_str()
                        ),
                    })?,
            ),
            IntendedMove::None | IntendedMove::OffBoard { .. } => None,
        };
        if self.judgement == GameTurnJudgement::AcceptedMove
            && intended_legal.map(LegalMove::move_id) != record.chosen_move()
        {
            return Err(GameboardContractError::InvalidContract {
                contract: "game turn adjudication",
                reason: "accepted judgement does not match the chosen move".to_string(),
            });
        }
        if let Some(legal_move) = intended_legal {
            for answer in &self.intended_arguments {
                let Some(declared) = legal_move
                    .arguments()
                    .iter()
                    .find(|argument| argument.name() == answer.name())
                else {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn adjudication",
                        reason: format!(
                            "intended argument '{}' is absent from the intended move",
                            answer.name()
                        ),
                    });
                };
                if declared.kind() != answer.kind() {
                    return Err(GameboardContractError::InvalidContract {
                        contract: "game turn adjudication",
                        reason: format!(
                            "intended argument '{}' has the wrong value kind",
                            answer.name()
                        ),
                    });
                }
            }
        }
        Ok(())
    }

    fn canonical_hash(&self) -> Result<GameTurnAdjudicationHash, GameboardContractError> {
        let intended_move = match &self.intended_move {
            IntendedMove::None => "none".to_string(),
            IntendedMove::OnBoard { move_id } => format!("on_board:{}", move_id.as_str()),
            IntendedMove::OffBoard { semantic_code } => {
                format!("off_board:{}", semantic_code.as_str())
            }
        };
        let fields = [
            (
                "schema_version".to_string(),
                self.schema_version.to_string(),
            ),
            ("record".to_string(), self.record_hash.as_str().to_string()),
            (
                "adjudicator".to_string(),
                self.adjudicator.as_str().to_string(),
            ),
            (
                "adjudicated_at_epoch_ms".to_string(),
                self.adjudicated_at_epoch_ms.to_string(),
            ),
            ("judgement".to_string(), format!("{:?}", self.judgement)),
            ("intended_move".to_string(), intended_move),
            (
                "intended_anchor".to_string(),
                self.intended_anchor
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "intended_motif".to_string(),
                self.intended_motif
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            (
                "note".to_string(),
                self.note_hash
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
        ]
        .into_iter()
        .chain(
            self.intended_arguments
                .iter()
                .enumerate()
                .flat_map(|(index, value)| {
                    [
                        (format!("argument.{index}.name"), value.name().to_string()),
                        (
                            format!("argument.{index}.kind"),
                            format!("{:?}", value.kind()),
                        ),
                        (
                            format!("argument.{index}.value"),
                            value.value().map_or_else(String::new, canonical_slot_value),
                        ),
                        (
                            format!("argument.{index}.provenance"),
                            value.provenance().unwrap_or_default().to_string(),
                        ),
                    ]
                }),
        )
        .chain(
            self.acceptable_clarifications
                .iter()
                .enumerate()
                .map(|(index, value)| (format!("clarification.{index}"), format!("{value:?}"))),
        )
        .chain(
            self.acceptable_feedback
                .iter()
                .enumerate()
                .map(|(index, value)| (format!("feedback.{index}"), format!("{value:?}"))),
        );
        GameTurnAdjudicationHash::new(hash_fields(
            "semantic-gameboard-turn-adjudication-v1",
            fields,
        ))
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    pub fn record_hash(&self) -> &GameTurnRecordHash {
        &self.record_hash
    }
    pub fn adjudicator(&self) -> &str {
        self.adjudicator.as_str()
    }
    pub fn adjudicated_at_epoch_ms(&self) -> u64 {
        self.adjudicated_at_epoch_ms
    }
    pub fn judgement(&self) -> GameTurnJudgement {
        self.judgement
    }
    pub fn intended_move(&self) -> &IntendedMove {
        &self.intended_move
    }
    pub fn intended_anchor(&self) -> Option<&GraphElementRef> {
        self.intended_anchor.as_ref()
    }
    pub fn intended_arguments(&self) -> &[MoveArgument] {
        &self.intended_arguments
    }
    pub fn intended_motif(&self) -> Option<&str> {
        self.intended_motif.as_ref().map(ContractText::as_str)
    }
    pub fn acceptable_clarifications(&self) -> &[GameClarificationDimension] {
        &self.acceptable_clarifications
    }
    pub fn acceptable_feedback(&self) -> &[FeedbackOptionKind] {
        &self.acceptable_feedback
    }
    pub fn note_hash(&self) -> Option<&GraphContentHash> {
        self.note_hash.as_ref()
    }
    pub fn adjudication_hash(&self) -> &GameTurnAdjudicationHash {
        &self.adjudication_hash
    }

    /// Return a positive structured-choice label only when the operator explicitly
    /// adjudicated one. Exploratory and accidental interactions can never label training.
    pub fn positive_label(&self) -> Option<&LegalMoveId> {
        if matches!(
            self.judgement,
            GameTurnJudgement::AcceptedMove | GameTurnJudgement::SystemMisinterpretation
        ) {
            if let IntendedMove::OnBoard { move_id } = &self.intended_move {
                return Some(move_id);
            }
        }
        None
    }
}

impl<'de> Deserialize<'de> for GameTurnAdjudication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            record_hash: GameTurnRecordHash,
            adjudicator: String,
            adjudicated_at_epoch_ms: u64,
            judgement: GameTurnJudgement,
            intended_move: IntendedMove,
            intended_anchor: Option<GraphElementRef>,
            intended_arguments: Vec<MoveArgument>,
            intended_motif: Option<String>,
            acceptable_clarifications: Vec<GameClarificationDimension>,
            acceptable_feedback: Vec<FeedbackOptionKind>,
            note_hash: Option<GraphContentHash>,
            adjudication_hash: GameTurnAdjudicationHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let adjudication_hash = wire.adjudication_hash;
        let admitted = Self::admit(
            wire.schema_version,
            wire.record_hash,
            wire.adjudicator,
            wire.adjudicated_at_epoch_ms,
            wire.judgement,
            wire.intended_move,
            wire.intended_anchor,
            wire.intended_arguments,
            wire.intended_motif,
            wire.acceptable_clarifications,
            wire.acceptable_feedback,
            wire.note_hash,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.adjudication_hash != adjudication_hash {
            return Err(serde::de::Error::custom(
                "game turn adjudication hash does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

/// Append-only session event covering observations, decisions and corrections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum DesignTurnEvent {
    InputObserved {
        input_hash: GraphContentHash,
    },
    FocusChanged {
        focus: DesignFocus,
    },
    BoardConstructed {
        position_id: DesignStateId,
        move_set_hash: MoveSetHash,
    },
    EvidenceRecorded {
        evidence_hash: GraphContentHash,
    },
    ClarificationAsked {
        message_key: MessageKey,
    },
    ClarificationAnswered {
        answer_hash: GraphContentHash,
    },
    MoveProposed {
        move_id: LegalMoveId,
    },
    MoveRejected {
        attempt_id: MoveAttemptId,
    },
    MoveRatified {
        attempt_id: MoveAttemptId,
    },
    AttemptIncomplete {
        attempt_id: MoveAttemptId,
    },
    AttemptInapplicable {
        attempt_id: MoveAttemptId,
    },
    AttemptStale {
        attempt_id: MoveAttemptId,
    },
    AttemptRefused {
        attempt_id: MoveAttemptId,
    },
    FeedbackOptionsPresented {
        attempt_id: MoveAttemptId,
    },
    FeedbackOptionSelected {
        attempt_id: MoveAttemptId,
        kind: FeedbackOptionKind,
    },
    CompileRefused {
        attempt_id: MoveAttemptId,
        diagnostic_hash: GraphContentHash,
    },
    GraphRevisionCommitted {
        from: GraphRevision,
        to: GraphRevision,
        delta_hash: GraphDeltaHash,
    },
    MoveUndone {
        attempt_id: MoveAttemptId,
        correction_of: MoveAttemptId,
    },
    MoveCorrected {
        attempt_id: MoveAttemptId,
        correction_of: MoveAttemptId,
    },
}

/// One attributable event in a bounded append-only design history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DesignTurn {
    schema_version: u32,
    turn_id: DesignTurnId,
    sequence: u64,
    actor: ContractText,
    previous_turn_hash: Option<DesignTurnHash>,
    event: DesignTurnEvent,
    turn_hash: DesignTurnHash,
}

impl DesignTurn {
    /// Construct one explicit event. Time is deliberately supplied outside this kernel.
    pub fn new(
        schema_version: u32,
        turn_id: DesignTurnId,
        sequence: u64,
        actor: impl Into<String>,
        previous_turn_hash: Option<DesignTurnHash>,
        event: DesignTurnEvent,
    ) -> Result<Self, GameboardContractError> {
        validate_schema(schema_version)?;
        if sequence == 0 && previous_turn_hash.is_some() {
            return Err(GameboardContractError::InvalidContract {
                contract: "design turn",
                reason: "the first sequence cannot carry a previous hash".to_string(),
            });
        }
        if sequence > 0 && previous_turn_hash.is_none() {
            return Err(GameboardContractError::InvalidContract {
                contract: "design turn",
                reason: "a non-initial sequence requires a previous hash".to_string(),
            });
        }
        if let DesignTurnEvent::GraphRevisionCommitted { from, to, .. } = &event {
            if from == to {
                return Err(GameboardContractError::InvalidContract {
                    contract: "design turn",
                    reason: "a committed graph revision must advance".to_string(),
                });
            }
        }
        if let DesignTurnEvent::MoveUndone {
            attempt_id,
            correction_of,
        }
        | DesignTurnEvent::MoveCorrected {
            attempt_id,
            correction_of,
        } = &event
        {
            if attempt_id == correction_of {
                return Err(GameboardContractError::InvalidCorrection(
                    "a correction event cannot link an attempt to itself".to_string(),
                ));
            }
        }
        let actor = ContractText::new("design turn actor", actor)?;
        let event_hash = hash_event(&event);
        let fields = [
            ("schema_version".to_string(), schema_version.to_string()),
            ("turn_id".to_string(), turn_id.as_str().to_string()),
            ("sequence".to_string(), sequence.to_string()),
            ("actor".to_string(), actor.as_str().to_string()),
            (
                "previous".to_string(),
                previous_turn_hash
                    .as_ref()
                    .map_or_else(String::new, |value| value.as_str().to_string()),
            ),
            ("event".to_string(), event_hash),
        ];
        let turn_hash =
            DesignTurnHash::new(hash_fields("semantic-gameboard-design-turn-v1", fields))?;
        Ok(Self {
            schema_version,
            turn_id,
            sequence,
            actor,
            previous_turn_hash,
            event,
            turn_hash,
        })
    }

    /// Schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    /// Session-supplied attributable identity.
    pub fn turn_id(&self) -> &DesignTurnId {
        &self.turn_id
    }
    /// Explicit monotonic session sequence.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    /// Actor/authority identity.
    pub fn actor(&self) -> &str {
        self.actor.as_str()
    }
    /// Previous append-only event hash.
    pub fn previous_turn_hash(&self) -> Option<&DesignTurnHash> {
        self.previous_turn_hash.as_ref()
    }
    /// Typed event.
    pub fn event(&self) -> &DesignTurnEvent {
        &self.event
    }
    /// Content identity of this event and predecessor link.
    pub fn turn_hash(&self) -> &DesignTurnHash {
        &self.turn_hash
    }
}

impl<'de> Deserialize<'de> for DesignTurn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: u32,
            turn_id: DesignTurnId,
            sequence: u64,
            actor: String,
            previous_turn_hash: Option<DesignTurnHash>,
            event: DesignTurnEvent,
            turn_hash: DesignTurnHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let admitted = Self::new(
            wire.schema_version,
            wire.turn_id,
            wire.sequence,
            wire.actor,
            wire.previous_turn_hash,
            wire.event,
        )
        .map_err(serde::de::Error::custom)?;
        if admitted.turn_hash != wire.turn_hash {
            return Err(serde::de::Error::custom(
                "design turn hash does not match canonical content",
            ));
        }
        Ok(admitted)
    }
}

fn hash_event(event: &DesignTurnEvent) -> String {
    let fields = match event {
        DesignTurnEvent::InputObserved { input_hash } => {
            vec![("input".to_string(), input_hash.as_str().to_string())]
        }
        DesignTurnEvent::FocusChanged { focus } => vec![("focus".to_string(), hash_focus(focus))],
        DesignTurnEvent::BoardConstructed {
            position_id,
            move_set_hash,
        } => vec![
            ("position".to_string(), position_id.as_str().to_string()),
            ("move_set".to_string(), move_set_hash.as_str().to_string()),
        ],
        DesignTurnEvent::EvidenceRecorded { evidence_hash } => {
            vec![("evidence".to_string(), evidence_hash.as_str().to_string())]
        }
        DesignTurnEvent::ClarificationAsked { message_key } => {
            vec![("message".to_string(), message_key.as_str().to_string())]
        }
        DesignTurnEvent::ClarificationAnswered { answer_hash } => {
            vec![("answer".to_string(), answer_hash.as_str().to_string())]
        }
        DesignTurnEvent::MoveProposed { move_id } => {
            vec![("move".to_string(), move_id.as_str().to_string())]
        }
        DesignTurnEvent::MoveRejected { attempt_id }
        | DesignTurnEvent::MoveRatified { attempt_id }
        | DesignTurnEvent::AttemptIncomplete { attempt_id }
        | DesignTurnEvent::AttemptInapplicable { attempt_id }
        | DesignTurnEvent::AttemptStale { attempt_id }
        | DesignTurnEvent::AttemptRefused { attempt_id }
        | DesignTurnEvent::FeedbackOptionsPresented { attempt_id } => {
            vec![("attempt".to_string(), attempt_id.as_str().to_string())]
        }
        DesignTurnEvent::FeedbackOptionSelected { attempt_id, kind } => vec![
            ("attempt".to_string(), attempt_id.as_str().to_string()),
            ("kind".to_string(), format!("{kind:?}")),
        ],
        DesignTurnEvent::CompileRefused {
            attempt_id,
            diagnostic_hash,
        } => vec![
            ("attempt".to_string(), attempt_id.as_str().to_string()),
            (
                "diagnostic".to_string(),
                diagnostic_hash.as_str().to_string(),
            ),
        ],
        DesignTurnEvent::GraphRevisionCommitted {
            from,
            to,
            delta_hash,
        } => vec![
            ("from".to_string(), from.as_str().to_string()),
            ("to".to_string(), to.as_str().to_string()),
            ("delta".to_string(), delta_hash.as_str().to_string()),
        ],
        DesignTurnEvent::MoveUndone {
            attempt_id,
            correction_of,
        }
        | DesignTurnEvent::MoveCorrected {
            attempt_id,
            correction_of,
        } => vec![
            ("attempt".to_string(), attempt_id.as_str().to_string()),
            (
                "correction_of".to_string(),
                correction_of.as_str().to_string(),
            ),
        ],
    };
    let variant = match event {
        DesignTurnEvent::InputObserved { .. } => "input_observed",
        DesignTurnEvent::FocusChanged { .. } => "focus_changed",
        DesignTurnEvent::BoardConstructed { .. } => "board_constructed",
        DesignTurnEvent::EvidenceRecorded { .. } => "evidence_recorded",
        DesignTurnEvent::ClarificationAsked { .. } => "clarification_asked",
        DesignTurnEvent::ClarificationAnswered { .. } => "clarification_answered",
        DesignTurnEvent::MoveProposed { .. } => "move_proposed",
        DesignTurnEvent::MoveRejected { .. } => "move_rejected",
        DesignTurnEvent::MoveRatified { .. } => "move_ratified",
        DesignTurnEvent::AttemptIncomplete { .. } => "attempt_incomplete",
        DesignTurnEvent::AttemptInapplicable { .. } => "attempt_inapplicable",
        DesignTurnEvent::AttemptStale { .. } => "attempt_stale",
        DesignTurnEvent::AttemptRefused { .. } => "attempt_refused",
        DesignTurnEvent::FeedbackOptionsPresented { .. } => "feedback_presented",
        DesignTurnEvent::FeedbackOptionSelected { .. } => "feedback_selected",
        DesignTurnEvent::CompileRefused { .. } => "compile_refused",
        DesignTurnEvent::GraphRevisionCommitted { .. } => "revision_committed",
        DesignTurnEvent::MoveUndone { .. } => "move_undone",
        DesignTurnEvent::MoveCorrected { .. } => "move_corrected",
    };
    hash_fields(
        "semantic-gameboard-design-turn-event-v1",
        std::iter::once(("variant".to_string(), variant.to_string())).chain(fields),
    )
}

/// Domain-neutral description retrievable from a recorded position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardDescription {
    position_id: DesignStateId,
    domain: GameDomainId,
    board_path: BoardPath,
    focus: DesignFocus,
    move_set_hash: MoveSetHash,
}

impl BoardDescription {
    /// Describe the authoritative identities already present in a position.
    pub fn from_position(position: &DesignPosition) -> Self {
        Self {
            position_id: position.state_id.clone(),
            domain: position.domain.clone(),
            board_path: position.board_path.clone(),
            focus: position.focus.clone(),
            move_set_hash: position.move_set_hash.clone(),
        }
    }

    /// Position identity.
    pub fn position_id(&self) -> &DesignStateId {
        &self.position_id
    }
    /// Active domain.
    pub fn domain(&self) -> &GameDomainId {
        &self.domain
    }
    /// Active board path.
    pub fn board_path(&self) -> &BoardPath {
        &self.board_path
    }
    /// Explicit focus.
    pub fn focus(&self) -> &DesignFocus {
        &self.focus
    }
    /// Legal move-set identity.
    pub fn move_set_hash(&self) -> &MoveSetHash {
        &self.move_set_hash
    }
}

/// Typed applicability response for a move on a recorded position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveApplicabilityExplanation {
    position_id: DesignStateId,
    move_id: LegalMoveId,
    facts: Vec<ApplicabilityFact>,
    binding_state: MoveBindingState,
}

impl MoveApplicabilityExplanation {
    /// Retrieve the facts already carried by a move; no rule is invented.
    pub fn for_move(position: &DesignPosition, move_id: &LegalMoveId) -> Option<Self> {
        position
            .legal_moves
            .iter()
            .find(|legal_move| legal_move.move_id() == move_id)
            .map(|legal_move| Self {
                position_id: position.state_id.clone(),
                move_id: legal_move.move_id.clone(),
                facts: legal_move.applicability.clone(),
                binding_state: legal_move.binding_state.clone(),
            })
    }

    /// Position identity.
    pub fn position_id(&self) -> &DesignStateId {
        &self.position_id
    }
    /// Move identity.
    pub fn move_id(&self) -> &LegalMoveId {
        &self.move_id
    }
    /// Governed applicability facts.
    pub fn facts(&self) -> &[ApplicabilityFact] {
        &self.facts
    }
    /// Binding completeness.
    pub fn binding_state(&self) -> &MoveBindingState {
        &self.binding_state
    }
}

/// Typed result of requesting a non-mutating preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum TransitionPreview {
    Available {
        position_id: DesignStateId,
        move_id: LegalMoveId,
        delta: GraphDeltaPreview,
    },
    Incomplete {
        position_id: DesignStateId,
        move_id: LegalMoveId,
        binding_state: MoveBindingState,
    },
    UnknownMove {
        position_id: DesignStateId,
        move_id: LegalMoveId,
    },
}

impl TransitionPreview {
    /// Retrieve a preview already admitted on the position.
    pub fn for_move(position: &DesignPosition, move_id: LegalMoveId) -> Self {
        match position
            .legal_moves
            .iter()
            .find(|legal_move| legal_move.move_id() == &move_id)
        {
            Some(legal_move) => match legal_move.preview() {
                Some(delta) => Self::Available {
                    position_id: position.state_id.clone(),
                    move_id,
                    delta: delta.clone(),
                },
                None => Self::Incomplete {
                    position_id: position.state_id.clone(),
                    move_id,
                    binding_state: legal_move.binding_state.clone(),
                },
            },
            None => Self::UnknownMove {
                position_id: position.state_id.clone(),
                move_id,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionClass, ArgumentSpec, DomainIdentity, HarmClass, ResolvedPosition};

    fn digest(byte: char) -> String {
        std::iter::repeat_n(byte, 64).collect()
    }

    fn graph_hash(byte: char) -> GraphContentHash {
        GraphContentHash::new(digest(byte)).unwrap()
    }

    fn history_hash(byte: char) -> HistoryHash {
        HistoryHash::new(digest(byte)).unwrap()
    }

    fn fact(code: &str) -> ApplicabilityFact {
        ApplicabilityFact::new(
            RuleCode::new(code).unwrap(),
            ApplicabilityState::Applicable,
            None,
            "pack@sha256:admitted",
        )
        .unwrap()
    }

    fn argument(name: &str, required: bool) -> MoveArgument {
        MoveArgument::new(name, ArgumentKind::Identifier, required, None, None).unwrap()
    }

    fn legal_move(candidate: &str, revision: &str) -> LegalMove {
        LegalMove::new(
            GAMEBOARD_SCHEMA_VERSION,
            CanonicalCandidateId::new(candidate).unwrap(),
            GraphRevision::new(revision).unwrap(),
            false,
            None,
            vec![argument("name", true)],
            vec![fact("rule.allowed")],
            None,
        )
        .unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    fn position_with(
        domain: &str,
        path: Vec<String>,
        snapshot: &str,
        revision: &str,
        graph: char,
        compiler: &str,
        policy: &str,
        proposal: Option<char>,
        focus: DesignFocus,
        history: char,
        moves: Vec<LegalMove>,
    ) -> DesignPosition {
        DesignPosition::new(
            GAMEBOARD_SCHEMA_VERSION,
            GameDomainId::new(domain).unwrap(),
            BoardPath::new(path).unwrap(),
            SnapshotIdentity::new(snapshot).unwrap(),
            GraphRevision::new(revision).unwrap(),
            graph_hash(graph),
            compiler,
            policy,
            proposal.map(graph_hash),
            focus,
            history_hash(history),
            moves,
        )
        .unwrap()
    }

    fn position() -> DesignPosition {
        position_with(
            "domain.example",
            vec!["root".to_string(), "bounded-board".to_string()],
            "snapshot-v1",
            "revision-1",
            'a',
            "compiler-profile-v1",
            "policy-v1",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            'b',
            vec![legal_move("op.example", "revision-1")],
        )
    }

    fn incomplete_game_turn() -> GameTurnRecord {
        let position = position();
        let move_id = position.legal_moves()[0].move_id().clone();
        let evidence = MoveEvidence::new(
            GAMEBOARD_SCHEMA_VERSION,
            move_id.clone(),
            Vec::new(),
            FiniteScore::new(0.75).unwrap(),
            FiniteScore::new(1.0).unwrap(),
            vec![RuleCode::new("evidence.complete-board").unwrap()],
            ProducerIdentity::new("deterministic-fusion-v1").unwrap(),
        )
        .unwrap();
        let belief = DesignBelief::new(
            GAMEBOARD_SCHEMA_VERSION,
            position.state_id().clone(),
            vec![MoveProbability::new(move_id.clone(), FiniteScore::new(1.0).unwrap()).unwrap()],
            Vec::new(),
            Vec::new(),
            ProducerIdentity::new("belief-policy-v1").unwrap(),
        )
        .unwrap();
        let attempt = MoveAttemptReceipt::new(
            GAMEBOARD_SCHEMA_VERSION,
            MoveAttemptId::new("attempt-incomplete").unwrap(),
            position.state_id().clone(),
            Some(move_id.clone()),
            graph_hash('c'),
            MoveAttemptOutcome::Incomplete,
            vec![RuleExplanationId::new(digest('d')).unwrap()],
            Vec::new(),
            None,
            None,
        )
        .unwrap();
        let disposition = GameDisposition::request_move_arguments(
            &position,
            move_id.clone(),
            vec!["name".to_string()],
            "prompt.argument.name",
            attempt.clone(),
        )
        .unwrap();
        GameTurnRecord::new(
            GAMEBOARD_SCHEMA_VERSION,
            GameSessionId::new("session-1").unwrap(),
            DesignTurnId::new("turn-1").unwrap(),
            7,
            1_786_128_000_000,
            SemanticFamilyId::new("family.example").unwrap(),
            HarmClass::Reversible,
            graph_hash('c'),
            position,
            vec![evidence],
            belief,
            disposition,
            GameTurnAnswer::not_observed(GameTurnAnswerAbsenceReason::NotRequested),
            Some(move_id),
            None,
            GameTurnAttempt::terminal(attempt),
            GameTurnCompilerResult::not_requested(),
            Vec::new(),
        )
        .unwrap()
    }

    #[test]
    fn position_round_trip_is_canonical_and_has_golden_bytes() {
        let position = position();
        let encoded = serde_json::to_vec(&position).unwrap();
        let decoded: DesignPosition = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, position);
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), encoded);
        assert_eq!(
            hex::encode(Sha256::digest(&encoded)),
            "09b4187ef408930418bcd84073e7ff69b0a53cb1502e5adf6056d47ea701d1c5"
        );
        assert_eq!(
            position.state_id().as_str(),
            "35f29f806dd75245aef00c2f24196d83e1c431a4845710684cba141849804d27"
        );
        assert_eq!(
            position.move_set_hash().as_str(),
            "e3f56ad4ce9be1c1615134c28e9e6d0eca1cf9d2672eb083ec36ce6454af101a"
        );
    }

    #[test]
    fn complete_game_turn_is_content_addressed_and_round_trips() {
        let record = incomplete_game_turn();
        assert_eq!(
            record.evidence().len(),
            record.position().legal_moves().len()
        );
        assert_eq!(
            record.attempt().receipt().unwrap().outcome(),
            MoveAttemptOutcome::Incomplete
        );
        assert_eq!(
            record.compiler_result().kind(),
            GameTurnCompilerResultKind::NotRequested
        );
        let encoded = serde_json::to_vec(&record).unwrap();
        let decoded: GameTurnRecord = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, record);

        let mut tampered: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        tampered["sequence"] = serde_json::json!(8);
        assert!(serde_json::from_value::<GameTurnRecord>(tampered).is_err());
    }

    #[test]
    fn proposal_turn_records_explicitly_that_no_terminal_attempt_exists() {
        let terminal = incomplete_game_turn();
        let move_id = terminal.chosen_move().unwrap().clone();
        let proposal = GameTurnRecord::new(
            terminal.schema_version(),
            terminal.session_id().clone(),
            DesignTurnId::new("turn-proposal").unwrap(),
            terminal.sequence() + 1,
            terminal.observed_at_epoch_ms() + 1,
            terminal.semantic_family().clone(),
            terminal.risk_class(),
            terminal.input_hash().clone(),
            terminal.position().clone(),
            terminal.evidence().to_vec(),
            terminal.belief().clone(),
            GameDisposition::propose_move(terminal.position(), move_id.clone()).unwrap(),
            GameTurnAnswer::not_observed(GameTurnAnswerAbsenceReason::NotRequested),
            Some(move_id),
            None,
            GameTurnAttempt::not_attempted(),
            GameTurnCompilerResult::not_requested(),
            Vec::new(),
        )
        .unwrap();
        assert_eq!(proposal.attempt().kind(), GameTurnAttemptKind::NotAttempted);
        assert_eq!(proposal.attempt().receipt(), None);
        assert!(proposal.related_attempts().is_empty());
        let encoded = serde_json::to_vec(&proposal).unwrap();
        assert_eq!(
            serde_json::from_slice::<GameTurnRecord>(&encoded).unwrap(),
            proposal
        );
    }

    #[test]
    fn correction_turn_retains_the_prior_attempt_needed_by_its_link() {
        let prior_turn = incomplete_game_turn();
        let prior = prior_turn.attempt().receipt().unwrap().clone();
        let current = MoveAttemptReceipt::new(
            GAMEBOARD_SCHEMA_VERSION,
            MoveAttemptId::new("attempt-correction").unwrap(),
            prior_turn.position().state_id().clone(),
            prior_turn.chosen_move().cloned(),
            graph_hash('f'),
            MoveAttemptOutcome::Corrected,
            Vec::new(),
            Vec::new(),
            Some(prior.attempt_id().clone()),
            Some(CorrectionKind::Replacement),
        )
        .unwrap();
        let record = GameTurnRecord::new(
            prior_turn.schema_version(),
            prior_turn.session_id().clone(),
            DesignTurnId::new("turn-correction").unwrap(),
            prior_turn.sequence() + 1,
            prior_turn.observed_at_epoch_ms() + 1,
            prior_turn.semantic_family().clone(),
            prior_turn.risk_class(),
            graph_hash('e'),
            prior_turn.position().clone(),
            prior_turn.evidence().to_vec(),
            prior_turn.belief().clone(),
            GameDisposition::explain_attempt(prior_turn.position(), current.clone()).unwrap(),
            GameTurnAnswer::not_observed(GameTurnAnswerAbsenceReason::NotRequested),
            prior_turn.chosen_move().cloned(),
            None,
            GameTurnAttempt::terminal(current),
            GameTurnCompilerResult::not_requested(),
            vec![prior.clone()],
        )
        .unwrap();
        assert_eq!(record.related_attempts(), &[prior]);
    }

    #[test]
    fn game_turn_refuses_incomplete_evidence_and_off_board_answers() {
        let record = incomplete_game_turn();
        assert!(GameTurnRecord::new(
            record.schema_version(),
            record.session_id().clone(),
            record.turn_id().clone(),
            record.sequence(),
            record.observed_at_epoch_ms(),
            record.semantic_family().clone(),
            record.risk_class(),
            record.input_hash().clone(),
            record.position().clone(),
            Vec::new(),
            record.belief().clone(),
            record.disposition().clone(),
            record.answer().clone(),
            record.chosen_move().cloned(),
            record.delta().cloned(),
            record.attempt().clone(),
            record.compiler_result().clone(),
            record.related_attempts().to_vec(),
        )
        .is_err());

        let off_board = GameTurnAnswer::feedback(
            graph_hash('e'),
            FeedbackOptionKind::SelectAlternative,
            Some(LegalMoveId::new(digest('f')).unwrap()),
        )
        .unwrap();
        assert!(GameTurnRecord::new(
            record.schema_version(),
            record.session_id().clone(),
            record.turn_id().clone(),
            record.sequence(),
            record.observed_at_epoch_ms(),
            record.semantic_family().clone(),
            record.risk_class(),
            record.input_hash().clone(),
            record.position().clone(),
            record.evidence().to_vec(),
            record.belief().clone(),
            record.disposition().clone(),
            off_board,
            record.chosen_move().cloned(),
            record.delta().cloned(),
            record.attempt().clone(),
            record.compiler_result().clone(),
            record.related_attempts().to_vec(),
        )
        .is_err());
    }

    #[test]
    fn adjudication_separates_exploration_accidents_and_explicit_labels() {
        let record = incomplete_game_turn();
        let intended = IntendedMove::OnBoard {
            move_id: record.chosen_move().unwrap().clone(),
        };
        let exploratory = GameTurnAdjudication::new(
            &record,
            "operator-1",
            1_786_128_001_000,
            GameTurnJudgement::ExploratoryHumanAttempt,
            intended.clone(),
            None,
            Vec::new(),
            None,
            vec![GameClarificationDimension::Move],
            vec![FeedbackOptionKind::SelectAlternative],
            None,
        )
        .unwrap();
        assert_eq!(exploratory.positive_label(), None);

        let accidental = GameTurnAdjudication::new(
            &record,
            "operator-1",
            1_786_128_002_000,
            GameTurnJudgement::AccidentalMove,
            IntendedMove::None,
            None,
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            None,
        )
        .unwrap();
        assert_eq!(accidental.positive_label(), None);

        let corrected = GameTurnAdjudication::new(
            &record,
            "operator-1",
            1_786_128_003_000,
            GameTurnJudgement::SystemMisinterpretation,
            intended,
            None,
            Vec::new(),
            Some("motif.example".to_string()),
            vec![GameClarificationDimension::Argument],
            vec![FeedbackOptionKind::Replace],
            Some(graph_hash('f')),
        )
        .unwrap();
        assert_eq!(corrected.positive_label(), record.chosen_move());
        corrected.validate_for_record(&record).unwrap();
        let encoded = serde_json::to_vec(&corrected).unwrap();
        let decoded: GameTurnAdjudication = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, corrected);
        decoded.validate_for_record(&record).unwrap();
    }

    #[test]
    fn adjudication_refuses_off_board_move_as_an_on_board_label() {
        let record = incomplete_game_turn();
        let result = GameTurnAdjudication::new(
            &record,
            "operator-1",
            1_786_128_004_000,
            GameTurnJudgement::SystemMisinterpretation,
            IntendedMove::OnBoard {
                move_id: LegalMoveId::new(digest('f')).unwrap(),
            },
            None,
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            None,
        );
        assert!(result.is_err());

        let unrepresentable = GameTurnAdjudication::new(
            &record,
            "operator-1",
            1_786_128_005_000,
            GameTurnJudgement::SystemMisinterpretation,
            IntendedMove::OffBoard {
                semantic_code: MessageKey::new("intent.not-represented").unwrap(),
            },
            None,
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            None,
        )
        .unwrap();
        assert_eq!(unrepresentable.positive_label(), None);
    }

    #[test]
    fn every_position_authority_field_moves_the_state_identity() {
        let baseline = position();
        let cases = vec![
            position_with(
                "domain.other",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["other".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v2",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-2",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-2")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'c',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v2",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v2",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                Some('d'),
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::unknown(GraphElementRef::new("unknown-element").unwrap()),
                'b',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'c',
                vec![legal_move("op.example", "revision-1")],
            ),
            position_with(
                "domain.example",
                vec!["root".into(), "bounded-board".into()],
                "snapshot-v1",
                "revision-1",
                'a',
                "compiler-profile-v1",
                "policy-v1",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                'b',
                vec![legal_move("op.other", "revision-1")],
            ),
        ];
        for changed in cases {
            assert_ne!(baseline.state_id(), changed.state_id());
        }
        assert!(matches!(
            DesignPosition::new(
                2,
                GameDomainId::new("domain.example").unwrap(),
                BoardPath::new(vec!["root".into()]).unwrap(),
                SnapshotIdentity::new("snapshot-v1").unwrap(),
                GraphRevision::new("revision-1").unwrap(),
                graph_hash('a'),
                "compiler",
                "policy",
                None,
                DesignFocus::absent(FocusAbsenceReason::NotProvided),
                history_hash('b'),
                vec![],
            ),
            Err(GameboardContractError::UnsupportedSchema { .. })
        ));
    }

    #[test]
    fn semantically_unordered_inputs_are_permutation_invariant() {
        let operation_a = GraphDeltaOperation::new("effect.a", None, graph_hash('a')).unwrap();
        let operation_b = GraphDeltaOperation::new("effect.b", None, graph_hash('b')).unwrap();
        let left = GraphDeltaPreview::new(
            GAMEBOARD_SCHEMA_VERSION,
            graph_hash('c'),
            vec![operation_a.clone(), operation_b.clone()],
        )
        .unwrap();
        let right = GraphDeltaPreview::new(
            GAMEBOARD_SCHEMA_VERSION,
            graph_hash('c'),
            vec![operation_b, operation_a],
        )
        .unwrap();
        assert_eq!(left, right);

        let move_a = legal_move("op.a", "revision-1");
        let move_b = legal_move("op.b", "revision-1");
        let first = position_with(
            "domain.example",
            vec!["root".into()],
            "snapshot-v1",
            "revision-1",
            'a',
            "compiler",
            "policy",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            'b',
            vec![move_a.clone(), move_b.clone()],
        );
        let second = position_with(
            "domain.example",
            vec!["root".into()],
            "snapshot-v1",
            "revision-1",
            'a',
            "compiler",
            "policy",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            'b',
            vec![move_b, move_a],
        );
        assert_eq!(first, second);
    }

    #[test]
    fn duplicate_moves_arguments_and_preview_effects_are_refused() {
        let duplicate = legal_move("op.a", "revision-1");
        let position = DesignPosition::new(
            GAMEBOARD_SCHEMA_VERSION,
            GameDomainId::new("domain.example").unwrap(),
            BoardPath::new(vec!["root".into()]).unwrap(),
            SnapshotIdentity::new("snapshot").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            graph_hash('a'),
            "compiler",
            "policy",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            history_hash('b'),
            vec![duplicate.clone(), duplicate],
        );
        assert!(position.is_err());

        let arguments = vec![argument("same", false), argument("same", true)];
        assert!(LegalMove::new(
            GAMEBOARD_SCHEMA_VERSION,
            CanonicalCandidateId::new("op.a").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            false,
            None,
            arguments,
            vec![fact("rule.allowed")],
            None,
        )
        .is_err());

        let operation = GraphDeltaOperation::new("effect", None, graph_hash('a')).unwrap();
        assert!(GraphDeltaPreview::new(
            GAMEBOARD_SCHEMA_VERSION,
            graph_hash('b'),
            vec![operation.clone(), operation]
        )
        .is_err());
    }

    #[test]
    fn oversized_contract_text_is_a_typed_resource_limit_refusal_not_a_generic_one() {
        let oversized = "x".repeat(MAX_CONTRACT_TEXT_BYTES + 1);
        let error = ContractText::new("compiler profile", oversized).unwrap_err();
        assert_eq!(
            error,
            GameboardContractError::ResourceLimitExceeded {
                field: "compiler profile",
                limit: MAX_CONTRACT_TEXT_BYTES,
                actual: MAX_CONTRACT_TEXT_BYTES + 1,
            }
        );
        // At the limit is admitted; the session stays usable for a legitimate call.
        let at_limit = "x".repeat(MAX_CONTRACT_TEXT_BYTES);
        assert!(ContractText::new("compiler profile", at_limit).is_ok());
    }

    #[test]
    fn oversized_move_argument_count_is_a_typed_resource_limit_refusal() {
        let too_many = (0..=MAX_MOVE_ARGUMENTS)
            .map(|index| argument(&format!("arg-{index}"), false))
            .collect::<Vec<_>>();
        let error = LegalMove::new(
            GAMEBOARD_SCHEMA_VERSION,
            CanonicalCandidateId::new("op.a").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            false,
            None,
            too_many,
            vec![fact("rule.allowed")],
            None,
        )
        .unwrap_err();
        assert_eq!(
            error,
            GameboardContractError::ResourceLimitExceeded {
                field: "legal move arguments",
                limit: MAX_MOVE_ARGUMENTS,
                actual: MAX_MOVE_ARGUMENTS + 1,
            }
        );
        // The session stays usable: a legal move at the limit still constructs.
        let at_limit = (0..MAX_MOVE_ARGUMENTS)
            .map(|index| argument(&format!("arg-{index}"), false))
            .collect::<Vec<_>>();
        assert!(LegalMove::new(
            GAMEBOARD_SCHEMA_VERSION,
            CanonicalCandidateId::new("op.a").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            false,
            None,
            at_limit,
            vec![fact("rule.allowed")],
            None,
        )
        .is_ok());
    }

    #[test]
    fn oversized_applicability_fact_count_is_a_typed_resource_limit_refusal() {
        let too_many = (0..=MAX_APPLICABILITY_FACTS)
            .map(|index| fact(&format!("rule.allowed-{index}")))
            .collect::<Vec<_>>();
        let error = LegalMove::new(
            GAMEBOARD_SCHEMA_VERSION,
            CanonicalCandidateId::new("op.a").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            false,
            None,
            vec![argument("name", false)],
            too_many,
            None,
        )
        .unwrap_err();
        assert_eq!(
            error,
            GameboardContractError::ResourceLimitExceeded {
                field: "legal move applicability facts",
                limit: MAX_APPLICABILITY_FACTS,
                actual: MAX_APPLICABILITY_FACTS + 1,
            }
        );
    }

    #[test]
    fn oversized_legal_move_set_is_a_typed_resource_limit_refusal() {
        let too_many = (0..=MAX_LEGAL_MOVES)
            .map(|index| legal_move(&format!("op.candidate-{index}"), "revision-1"))
            .collect::<Vec<_>>();
        let error = DesignPosition::new(
            GAMEBOARD_SCHEMA_VERSION,
            GameDomainId::new("domain.example").unwrap(),
            BoardPath::new(vec!["root".into()]).unwrap(),
            SnapshotIdentity::new("snapshot").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            graph_hash('a'),
            "compiler",
            "policy",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            history_hash('b'),
            too_many,
        )
        .unwrap_err();
        assert_eq!(
            error,
            GameboardContractError::ResourceLimitExceeded {
                field: "design position legal moves",
                limit: MAX_LEGAL_MOVES,
                actual: MAX_LEGAL_MOVES + 1,
            }
        );
        // The session stays usable: a normal, small legal-move set still constructs.
        assert!(DesignPosition::new(
            GAMEBOARD_SCHEMA_VERSION,
            GameDomainId::new("domain.example").unwrap(),
            BoardPath::new(vec!["root".into()]).unwrap(),
            SnapshotIdentity::new("snapshot").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            graph_hash('a'),
            "compiler",
            "policy",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            history_hash('b'),
            vec![legal_move("op.solo", "revision-1")],
        )
        .is_ok());
    }

    #[test]
    fn oversized_delta_operation_count_is_a_typed_resource_limit_refusal() {
        let too_many = (0..=MAX_DELTA_OPERATIONS)
            .map(|index| GraphDeltaOperation::new(format!("effect.{index}"), None, graph_hash('a')).unwrap())
            .collect::<Vec<_>>();
        let error =
            GraphDeltaPreview::new(GAMEBOARD_SCHEMA_VERSION, graph_hash('b'), too_many).unwrap_err();
        assert_eq!(
            error,
            GameboardContractError::ResourceLimitExceeded {
                field: "graph delta preview operations",
                limit: MAX_DELTA_OPERATIONS,
                actual: MAX_DELTA_OPERATIONS + 1,
            }
        );
    }

    #[test]
    fn oversized_attempt_history_is_a_typed_resource_limit_refusal() {
        let too_many = (0..=MAX_VALIDATED_ATTEMPTS)
            .map(|index| {
                MoveAttemptReceipt::new(
                    GAMEBOARD_SCHEMA_VERSION,
                    MoveAttemptId::new(format!("attempt-{index}")).unwrap(),
                    DesignStateId::new(digest('a')).unwrap(),
                    None,
                    graph_hash('b'),
                    MoveAttemptOutcome::Applied,
                    Vec::new(),
                    Vec::new(),
                    None,
                    None,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let error = validate_attempt_history(&too_many).unwrap_err();
        assert_eq!(
            error,
            GameboardContractError::ResourceLimitExceeded {
                field: "attempt history",
                limit: MAX_VALIDATED_ATTEMPTS,
                actual: MAX_VALIDATED_ATTEMPTS + 1,
            }
        );
        // The session stays usable: a normal, bounded attempt window still validates.
        assert!(validate_attempt_history(&too_many[..MAX_VALIDATED_ATTEMPTS]).is_ok());
    }

    #[test]
    fn every_move_authority_field_moves_its_identity() {
        fn build(
            candidate: &str,
            revision: &str,
            requires_anchor: bool,
            anchor: Option<&str>,
            argument_name: &str,
            rule: &str,
            preview: Option<GraphDeltaPreview>,
        ) -> LegalMove {
            LegalMove::new(
                GAMEBOARD_SCHEMA_VERSION,
                CanonicalCandidateId::new(candidate).unwrap(),
                GraphRevision::new(revision).unwrap(),
                requires_anchor,
                anchor.map(|value| GraphElementRef::new(value).unwrap()),
                vec![argument(argument_name, false)],
                vec![fact(rule)],
                preview,
            )
            .unwrap()
        }
        let baseline = build("op.a", "rev-a", false, None, "name", "rule.a", None);
        let preview = GraphDeltaPreview::new(
            GAMEBOARD_SCHEMA_VERSION,
            graph_hash('a'),
            vec![GraphDeltaOperation::new("effect", None, graph_hash('b')).unwrap()],
        )
        .unwrap();
        let changes = [
            build("op.b", "rev-a", false, None, "name", "rule.a", None),
            build("op.a", "rev-b", false, None, "name", "rule.a", None),
            build(
                "op.a",
                "rev-a",
                true,
                Some("element-a"),
                "name",
                "rule.a",
                None,
            ),
            build(
                "op.a",
                "rev-a",
                false,
                Some("element-a"),
                "name",
                "rule.a",
                None,
            ),
            build("op.a", "rev-a", false, None, "other", "rule.a", None),
            build("op.a", "rev-a", false, None, "name", "rule.b", None),
            build(
                "op.a",
                "rev-a",
                false,
                None,
                "name",
                "rule.a",
                Some(preview),
            ),
        ];
        for changed in changes {
            assert_ne!(baseline.move_id(), changed.move_id());
        }
    }

    #[test]
    fn unknown_focus_round_trips_without_selecting_an_element() {
        let focus = DesignFocus::unknown(GraphElementRef::new("not-in-current-graph").unwrap());
        let encoded = serde_json::to_string(&focus).unwrap();
        assert_eq!(
            serde_json::from_str::<DesignFocus>(&encoded).unwrap(),
            focus
        );
        assert!(matches!(focus, DesignFocus::Unknown { .. }));
    }

    fn attempt_receipt(
        id: &str,
        outcome: MoveAttemptOutcome,
        correction_of: Option<&str>,
    ) -> MoveAttemptReceipt {
        MoveAttemptReceipt::new(
            GAMEBOARD_SCHEMA_VERSION,
            MoveAttemptId::new(id).unwrap(),
            DesignStateId::new(digest('a')).unwrap(),
            None,
            graph_hash('b'),
            outcome,
            Vec::new(),
            Vec::new(),
            correction_of.map(|value| MoveAttemptId::new(value).unwrap()),
            correction_of.map(|_| CorrectionKind::Replacement),
        )
        .unwrap()
    }

    #[test]
    fn every_attempt_outcome_including_non_transitions_round_trips() {
        let outcomes = [
            MoveAttemptOutcome::Applied,
            MoveAttemptOutcome::Incomplete,
            MoveAttemptOutcome::Ambiguous,
            MoveAttemptOutcome::Inapplicable,
            MoveAttemptOutcome::DisclosureSafeRefusal,
            MoveAttemptOutcome::Stale,
            MoveAttemptOutcome::CompilerRefused,
            MoveAttemptOutcome::RejectedByUser,
            MoveAttemptOutcome::Corrected,
            MoveAttemptOutcome::SystemFailure,
        ];
        for (index, outcome) in outcomes.into_iter().enumerate() {
            let target = (outcome == MoveAttemptOutcome::Corrected).then_some("original");
            let receipt = attempt_receipt(&format!("attempt-{index}"), outcome, target);
            let bytes = serde_json::to_vec(&receipt).unwrap();
            let decoded: MoveAttemptReceipt = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(decoded, receipt);
            assert_eq!(decoded.outcome(), outcome);
        }
    }

    #[test]
    fn attempt_receipt_hash_tracks_outcome_and_governed_response() {
        let baseline = attempt_receipt("attempt", MoveAttemptOutcome::Incomplete, None);
        let changed_outcome = attempt_receipt("attempt", MoveAttemptOutcome::Stale, None);
        assert_ne!(baseline.receipt_hash(), changed_outcome.receipt_hash());

        let explanation = RuleExplanation::new(
            GAMEBOARD_SCHEMA_VERSION,
            RuleCode::new("rule.a").unwrap(),
            MessageKey::new("message.a").unwrap(),
            vec![],
            "pack@admitted",
            DisclosureClass::Public,
        )
        .unwrap();
        let with_response = MoveAttemptReceipt::new(
            GAMEBOARD_SCHEMA_VERSION,
            MoveAttemptId::new("attempt").unwrap(),
            DesignStateId::new(digest('a')).unwrap(),
            None,
            graph_hash('b'),
            MoveAttemptOutcome::Incomplete,
            vec![explanation.explanation_id().clone()],
            vec![FeedbackOption::new(
                FeedbackOptionKind::SupplyArgument,
                None,
                MessageKey::new("feedback.supply").unwrap(),
                Some(explanation.explanation_id().clone()),
                DisclosureClass::Public,
            )],
            None,
            None,
        )
        .unwrap();
        assert_ne!(baseline.receipt_hash(), with_response.receipt_hash());
    }

    #[test]
    fn game_dispositions_are_position_bound_and_off_board_moves_fail_closed() {
        let position = position_with(
            "domain.example",
            vec!["root".to_string()],
            "snapshot-v1",
            "revision-1",
            'a',
            "compiler-profile-v1",
            "policy-v1",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            'b',
            vec![
                legal_move("op.alpha", "revision-1"),
                legal_move("op.beta", "revision-1"),
                legal_move("op.gamma", "revision-1"),
            ],
        );
        let moves = position
            .legal_moves()
            .iter()
            .map(|legal_move| legal_move.move_id().clone())
            .collect::<Vec<_>>();
        let attempt = MoveAttemptReceipt::new(
            GAMEBOARD_SCHEMA_VERSION,
            MoveAttemptId::new("attempt-clarify").unwrap(),
            position.state_id().clone(),
            None,
            graph_hash('c'),
            MoveAttemptOutcome::Ambiguous,
            Vec::new(),
            Vec::new(),
            None,
            None,
        )
        .unwrap();
        let clarify = GameDisposition::clarify_moves(
            &position,
            moves.clone(),
            GameClarificationDimension::Move,
            "governed clarification",
            attempt,
        )
        .unwrap();
        assert_eq!(clarify.kind(), GameDispositionKind::ClarifyMoves);
        assert_eq!(clarify.selected_moves().len(), 3);
        clarify.validate_for_position(&position).unwrap();
        let encoded = serde_json::to_vec(&clarify).unwrap();
        let decoded: GameDisposition = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, clarify);

        let off_board = LegalMoveId::new(digest('f')).unwrap();
        assert!(GameDisposition::propose_move(&position, off_board).is_err());
        let retained_attempt = MoveAttemptReceipt::new(
            GAMEBOARD_SCHEMA_VERSION,
            MoveAttemptId::new("retained-prior-attempt").unwrap(),
            DesignStateId::new(digest('e')).unwrap(),
            None,
            graph_hash('f'),
            MoveAttemptOutcome::RejectedByUser,
            Vec::new(),
            Vec::new(),
            None,
            None,
        )
        .unwrap();
        assert!(GameDisposition::offer_correction(
            &position,
            moves[..2].to_vec(),
            retained_attempt,
        )
        .is_ok());
        let stale = position_with(
            "domain.example",
            vec!["root".to_string()],
            "snapshot-v1",
            "revision-2",
            'd',
            "compiler-profile-v1",
            "policy-v1",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            'b',
            vec![legal_move("op.alpha", "revision-2")],
        );
        assert!(clarify.validate_for_position(&stale).is_err());
    }

    #[test]
    fn proposal_workbook_preserves_legal_move_position_and_move_set() {
        let position = position();
        let selected = position.legal_moves()[0].move_id().clone();
        let workbook = crate::ProposalWorkbook::new_position_bound(
            GAMEBOARD_SCHEMA_VERSION,
            crate::WorkbookId::new("position-bound-workbook").unwrap(),
            1,
            crate::BoardHash::new(digest('c')).unwrap(),
            &position,
            selected.clone(),
            Vec::new(),
            crate::EvidenceRecordHash::new(digest('d')).unwrap(),
        )
        .unwrap();
        let binding = workbook.position_binding().unwrap();
        assert_eq!(binding.move_id(), &selected);
        assert_eq!(binding.position_id(), position.state_id());
        assert_eq!(binding.move_set_hash(), position.move_set_hash());
        workbook.validate_position(&position).unwrap();

        let stale = position_with(
            "domain.example",
            vec!["root".to_string(), "bounded-board".to_string()],
            "snapshot-v1",
            "revision-2",
            'e',
            "compiler-profile-v1",
            "policy-v1",
            None,
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            'b',
            vec![legal_move("op.example", "revision-2")],
        );
        assert!(workbook.validate_position(&stale).is_err());
    }

    #[test]
    fn correction_links_require_known_acyclic_retained_attempts() {
        let original = attempt_receipt("original", MoveAttemptOutcome::RejectedByUser, None);
        let correction = attempt_receipt(
            "correction",
            MoveAttemptOutcome::Corrected,
            Some("original"),
        );
        assert!(validate_attempt_history(&[original.clone(), correction]).is_ok());
        let missing = attempt_receipt(
            "missing-link",
            MoveAttemptOutcome::Corrected,
            Some("absent"),
        );
        assert!(validate_attempt_history(&[original, missing]).is_err());

        let left = attempt_receipt("left", MoveAttemptOutcome::Corrected, Some("right"));
        let right = attempt_receipt("right", MoveAttemptOutcome::Corrected, Some("left"));
        assert!(validate_attempt_history(&[left, right]).is_err());
    }

    #[test]
    fn disclosure_classes_round_trip_and_filter_without_invention() {
        let classes = [
            DisclosureClass::Public,
            DisclosureClass::Authenticated,
            DisclosureClass::Restricted,
            DisclosureClass::PolicyHidden,
            DisclosureClass::Technical,
        ];
        let explanations = classes
            .into_iter()
            .enumerate()
            .map(|(index, disclosure)| {
                RuleExplanation::new(
                    GAMEBOARD_SCHEMA_VERSION,
                    RuleCode::new(format!("rule.{index}")).unwrap(),
                    MessageKey::new(format!("message.{index}")).unwrap(),
                    Vec::new(),
                    "pack@admitted",
                    disclosure,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        for explanation in &explanations {
            let encoded = serde_json::to_vec(explanation).unwrap();
            assert_eq!(
                serde_json::from_slice::<RuleExplanation>(&encoded).unwrap(),
                *explanation
            );
        }
        let visible = filter_rule_explanations(
            &explanations,
            &[DisclosureClass::Public, DisclosureClass::Technical],
        );
        assert_eq!(visible.len(), 2);
        assert_eq!(visible[0].disclosure(), DisclosureClass::Public);
        assert_eq!(visible[1].disclosure(), DisclosureClass::Technical);
    }

    #[test]
    fn non_finite_and_out_of_range_evidence_is_refused() {
        assert!(FiniteScore::new(f64::NAN).is_err());
        let move_id = LegalMoveId::new(digest('a')).unwrap();
        assert!(MoveEvidence::new(
            GAMEBOARD_SCHEMA_VERSION,
            move_id,
            Vec::new(),
            FiniteScore::new(0.5).unwrap(),
            FiniteScore::new(1.1).unwrap(),
            Vec::new(),
            ProducerIdentity::new("producer-v1").unwrap(),
        )
        .is_err());
    }

    #[test]
    fn finite_scores_preserve_bits_through_the_canonical_json_boundary() {
        let score = FiniteScore::new(122.0 / 255.0).unwrap();
        let encoded = serde_json::to_vec(&score).unwrap();
        let decoded: FiniteScore = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded.get().to_bits(), score.get().to_bits());
    }

    fn legacy_board() -> SemanticDecisionBoard {
        SemanticDecisionBoard::new(
            1,
            DomainIdentity::new("domain.example").unwrap(),
            SnapshotIdentity::new("snapshot-v1").unwrap(),
            GraphRevision::new("revision-1").unwrap(),
            ResolvedPosition {
                anchor: None,
                context_hash: "context".into(),
            },
            vec![CandidateSemanticSlice {
                canonical_id: CanonicalCandidateId::new("op.example").unwrap(),
                schema_version: 1,
                title: "Example".into(),
                intent_summary: "Example governed action".into(),
                action_class: ActionClass::Create,
                applicability: "governed by the admitted pack".into(),
                effect: "domain adapter effect".into(),
                arguments: vec![ArgumentSpec {
                    name: "name".into(),
                    kind: ArgumentKind::Identifier,
                    required: true,
                    clarification_prompt: "governed.prompt.name".into(),
                }],
                phrases: vec![],
                positive_examples: vec![],
                negative_contrasts: vec![],
                risk: HarmClass::Reversible,
                adapter_payload_hash: "adapter-payload".into(),
            }],
            "policy-v1".into(),
        )
        .unwrap()
    }

    #[test]
    fn legacy_semantic_board_compatibility_requires_missing_authority_inputs() {
        let board = legacy_board();
        let position = DesignPosition::from_semantic_board(
            &board,
            BoardPath::new(vec!["root".into()]).unwrap(),
            graph_hash('a'),
            "compiler-profile-v1",
            "policy-v1",
            history_hash('b'),
            DesignFocus::absent(FocusAbsenceReason::NotProvided),
            None,
        )
        .unwrap();
        assert_eq!(position.legal_moves().len(), board.candidates.len());
        assert!(position
            .legal_moves()
            .iter()
            .any(|legal_move| legal_move.candidate_id().as_str() == "op.example"));
        let encoded = serde_json::to_vec(&position).unwrap();
        assert_eq!(
            serde_json::from_slice::<DesignPosition>(&encoded).unwrap(),
            position
        );
    }
}

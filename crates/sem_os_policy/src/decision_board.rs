//! Content-addressed semantic decision boards and proposal workbooks.
//!
//! This module is host-agnostic. Adapters decide which actions are legal and
//! map their domain types into these contracts; a model may only attach finite
//! evidence to the resulting closed board.

use std::collections::{BTreeMap, BTreeSet};

use sem_os_ontology::verb_contract::{ActionClass, HarmClass};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Framework-owned abstention identity present exactly once on every board.
pub const ABSTENTION_CANDIDATE_ID: &str = "abstain.none_of_the_above";

/// Errors raised while constructing or transitioning semantic contracts.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DecisionBoardError {
    /// A required string is empty or contains a control character.
    #[error("invalid {field}: {reason}")]
    InvalidText { field: &'static str, reason: String },
    /// A hash wrapper was not a lowercase 64-character hex digest.
    #[error("invalid {field}: expected 64 lowercase hexadecimal characters")]
    InvalidHash { field: &'static str },
    /// The adapter supplied a duplicate or reserved candidate.
    #[error("invalid candidate set: {0}")]
    InvalidCandidateSet(String),
    /// Evidence was not a complete one-to-one projection of a board.
    #[error("invalid evidence: {0}")]
    InvalidEvidence(String),
    /// A score was NaN or infinite.
    #[error("score must be finite")]
    NonFiniteScore,
    /// A workbook state transition is not in the closed transition table.
    #[error("illegal workbook transition from {from:?} to {to:?}")]
    IllegalWorkbookTransition {
        from: ProposalStatus,
        to: ProposalStatus,
    },
    /// A slot answer does not conform to the declared slot.
    #[error("invalid slot answer: {0}")]
    InvalidSlotAnswer(String),
    /// A disposition policy is internally inconsistent.
    #[error("invalid disposition policy: {0}")]
    InvalidPolicy(String),
}

fn validate_text(field: &'static str, value: &str) -> Result<(), DecisionBoardError> {
    if value.is_empty() {
        return Err(DecisionBoardError::InvalidText {
            field,
            reason: "must not be empty".to_string(),
        });
    }
    if value.chars().any(char::is_control) {
        return Err(DecisionBoardError::InvalidText {
            field,
            reason: "must not contain control characters".to_string(),
        });
    }
    Ok(())
}

macro_rules! text_identity {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Validated ", $field, " identity.")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Construct a validated ", $field, ".\n\n# Examples\n\n```\n# use sem_os_policy::decision_board::", stringify!($name), ";\nlet value = ", stringify!($name), "::new(\"example\").unwrap();\nassert_eq!(value.as_str(), \"example\");\n```")]
            pub fn new(value: impl Into<String>) -> Result<Self, DecisionBoardError> {
                let value = value.into();
                validate_text($field, &value)?;
                Ok(Self(value))
            }

            #[doc = concat!("Borrow the validated ", $field, ".\n\n# Examples\n\n```\n# use sem_os_policy::decision_board::", stringify!($name), ";\nassert_eq!(", stringify!($name), "::new(\"x\").unwrap().as_str(), \"x\");\n```")]
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
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

text_identity!(CanonicalCandidateId, "candidate id");
text_identity!(DomainIdentity, "domain");
text_identity!(SnapshotIdentity, "semantic snapshot");
text_identity!(GraphRevision, "graph revision");
text_identity!(WorkbookId, "workbook id");

/// Lowercase SHA-256 digest of the full semantic board preimage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoardHash(String);

impl BoardHash {
    /// Construct a validated board hash.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::BoardHash;
    /// let hash = BoardHash::new("0".repeat(64)).unwrap();
    /// assert_eq!(hash.as_str().len(), 64);
    /// ```
    pub fn new(value: impl Into<String>) -> Result<Self, DecisionBoardError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(DecisionBoardError::InvalidHash {
                field: "board hash",
            });
        }
        Ok(Self(value))
    }

    /// Borrow the lowercase digest.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::BoardHash;
    /// assert!(BoardHash::new("a".repeat(64)).unwrap().as_str().starts_with('a'));
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for BoardHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for BoardHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Lowercase SHA-256 digest of a complete evidence or decision record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceRecordHash(String);

impl EvidenceRecordHash {
    /// Construct a validated evidence record hash.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::EvidenceRecordHash;
    /// let hash = EvidenceRecordHash::new("0".repeat(64)).unwrap();
    /// assert_eq!(hash.as_str().len(), 64);
    /// ```
    pub fn new(value: impl Into<String>) -> Result<Self, DecisionBoardError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(DecisionBoardError::InvalidHash {
                field: "evidence record hash",
            });
        }
        Ok(Self(value))
    }

    /// Borrow the lowercase digest.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::EvidenceRecordHash;
    /// assert_eq!(EvidenceRecordHash::new("a".repeat(64)).unwrap().as_str().len(), 64);
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for EvidenceRecordHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EvidenceRecordHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Role played by a governed phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhraseRole {
    Canonical,
    Paraphrase,
    Shorthand,
    ArgumentCue,
    NegativeExample,
}

/// Governed phrase evidence attached to a candidate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PhraseEvidence {
    pub text: String,
    pub locale: String,
    pub role: PhraseRole,
    pub provenance: String,
}

/// Generic argument kind understood by a deterministic workbook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArgumentKind {
    Identifier,
    NodeReference,
    DataReference,
    Count,
    Duration,
    Condition,
    SubprocessReference,
    Text,
    Boolean,
}

/// Candidate argument contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ArgumentSpec {
    pub name: String,
    pub kind: ArgumentKind,
    pub required: bool,
    pub clarification_prompt: String,
}

/// Explicit contrast against a semantically nearby candidate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NegativeContrast {
    pub candidate_id: CanonicalCandidateId,
    pub distinction: String,
}

/// Model-visible, host-projected semantics for one legal candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateSemanticSlice {
    pub canonical_id: CanonicalCandidateId,
    pub schema_version: u32,
    pub title: String,
    pub intent_summary: String,
    pub action_class: ActionClass,
    pub applicability: String,
    pub effect: String,
    pub arguments: Vec<ArgumentSpec>,
    pub phrases: Vec<PhraseEvidence>,
    pub positive_examples: Vec<String>,
    pub negative_contrasts: Vec<NegativeContrast>,
    pub risk: HarmClass,
    pub adapter_payload_hash: String,
}

impl CandidateSemanticSlice {
    fn canonicalize(&mut self) -> Result<(), DecisionBoardError> {
        for (field, value) in [
            ("title", self.title.as_str()),
            ("intent summary", self.intent_summary.as_str()),
            ("applicability", self.applicability.as_str()),
            ("effect", self.effect.as_str()),
            ("adapter payload hash", self.adapter_payload_hash.as_str()),
        ] {
            validate_text(field, value)?;
        }
        for argument in &self.arguments {
            validate_text("argument name", &argument.name)?;
            validate_text("clarification prompt", &argument.clarification_prompt)?;
        }
        for phrase in &self.phrases {
            validate_text("phrase", &phrase.text)?;
            validate_text("phrase locale", &phrase.locale)?;
            validate_text("phrase provenance", &phrase.provenance)?;
        }
        for value in &self.positive_examples {
            validate_text("positive example", value)?;
        }
        for contrast in &self.negative_contrasts {
            validate_text("negative contrast", &contrast.distinction)?;
        }
        self.arguments.sort();
        self.arguments.dedup();
        self.phrases.sort();
        self.phrases.dedup();
        self.positive_examples.sort();
        self.positive_examples.dedup();
        self.negative_contrasts.sort();
        self.negative_contrasts.dedup();
        Ok(())
    }
}

/// Resolved position to which a semantic board is bound.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedPosition {
    pub anchor: Option<String>,
    pub context_hash: String,
}

/// Closed, canonical and content-addressed action board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticDecisionBoard {
    pub schema_version: u32,
    pub domain: DomainIdentity,
    pub semantic_snapshot: SnapshotIdentity,
    pub graph_revision: GraphRevision,
    pub position: ResolvedPosition,
    pub candidates: Vec<CandidateSemanticSlice>,
    pub policy_fingerprint: String,
    pub board_hash: BoardHash,
}

impl<'de> Deserialize<'de> for SemanticDecisionBoard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireBoard {
            schema_version: u32,
            domain: DomainIdentity,
            semantic_snapshot: SnapshotIdentity,
            graph_revision: GraphRevision,
            position: ResolvedPosition,
            candidates: Vec<CandidateSemanticSlice>,
            policy_fingerprint: String,
            board_hash: BoardHash,
        }

        let mut wire = WireBoard::deserialize(deserializer)?;
        let abstention = wire.candidates.pop().ok_or_else(|| {
            serde::de::Error::custom("semantic board must end with framework abstention")
        })?;
        if abstention.canonical_id.as_str() != ABSTENTION_CANDIDATE_ID {
            return Err(serde::de::Error::custom(
                "semantic board must end with framework abstention",
            ));
        }
        let board = SemanticDecisionBoard::new(
            wire.schema_version,
            wire.domain,
            wire.semantic_snapshot,
            wire.graph_revision,
            wire.position,
            wire.candidates,
            wire.policy_fingerprint,
        )
        .map_err(serde::de::Error::custom)?;
        if board.board_hash != wire.board_hash {
            return Err(serde::de::Error::custom(
                "semantic board hash does not match canonical content",
            ));
        }
        Ok(board)
    }
}

impl SemanticDecisionBoard {
    /// Construct the only valid semantic board shape.
    ///
    /// The constructor canonicalizes model-visible collections, rejects
    /// duplicate/reserved ids, appends abstention, and hashes every field.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let candidates: Vec<CandidateSemanticSlice> = vec![];
    /// let board = SemanticDecisionBoard::new(
    ///     1,
    ///     DomainIdentity::new("demo").unwrap(),
    ///     SnapshotIdentity::new("snapshot").unwrap(),
    ///     GraphRevision::new("revision").unwrap(),
    ///     ResolvedPosition { anchor: None, context_hash: "context".into() },
    ///     candidates,
    ///     "policy-v1".into(),
    /// ).unwrap();
    /// assert_eq!(board.candidates.last().unwrap().canonical_id.as_str(), ABSTENTION_CANDIDATE_ID);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        domain: DomainIdentity,
        semantic_snapshot: SnapshotIdentity,
        graph_revision: GraphRevision,
        position: ResolvedPosition,
        mut candidates: Vec<CandidateSemanticSlice>,
        policy_fingerprint: String,
    ) -> Result<Self, DecisionBoardError> {
        validate_text("position context hash", &position.context_hash)?;
        if let Some(anchor) = &position.anchor {
            validate_text("position anchor", anchor)?;
        }
        validate_text("policy fingerprint", &policy_fingerprint)?;
        for candidate in &mut candidates {
            candidate.canonicalize()?;
            if candidate.canonical_id.as_str() == ABSTENTION_CANDIDATE_ID {
                return Err(DecisionBoardError::InvalidCandidateSet(
                    "adapter supplied the reserved abstention id".to_string(),
                ));
            }
        }
        candidates.sort_by(|left, right| left.canonical_id.cmp(&right.canonical_id));
        if candidates
            .windows(2)
            .any(|pair| pair[0].canonical_id == pair[1].canonical_id)
        {
            return Err(DecisionBoardError::InvalidCandidateSet(
                "duplicate canonical candidate id".to_string(),
            ));
        }
        candidates.push(abstention_slice()?);
        let digest = hash_board(
            schema_version,
            &domain,
            &semantic_snapshot,
            &graph_revision,
            &position,
            &candidates,
            &policy_fingerprint,
        );
        Ok(Self {
            schema_version,
            domain,
            semantic_snapshot,
            graph_revision,
            position,
            candidates,
            policy_fingerprint,
            board_hash: BoardHash::new(digest)?,
        })
    }

    /// Resolve a candidate by canonical id.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let board: SemanticDecisionBoard = todo!();
    /// assert!(board.candidate(ABSTENTION_CANDIDATE_ID).is_some());
    /// ```
    pub fn candidate(&self, id: &str) -> Option<&CandidateSemanticSlice> {
        self.candidates
            .iter()
            .find(|candidate| candidate.canonical_id.as_str() == id)
    }
}

fn abstention_slice() -> Result<CandidateSemanticSlice, DecisionBoardError> {
    let mut slice = CandidateSemanticSlice {
        canonical_id: CanonicalCandidateId::new(ABSTENTION_CANDIDATE_ID)?,
        schema_version: 1,
        title: "None of the above".to_string(),
        intent_summary: "No available legal action matches the request".to_string(),
        action_class: ActionClass::Review,
        applicability: "Always available as explicit abstention".to_string(),
        effect: "No operation and no mutation".to_string(),
        arguments: Vec::new(),
        phrases: Vec::new(),
        positive_examples: vec!["That is not what I meant".to_string()],
        negative_contrasts: Vec::new(),
        risk: HarmClass::ReadOnly,
        adapter_payload_hash: hex::encode(Sha256::digest(b"framework-abstention-v1")),
    };
    slice.canonicalize()?;
    Ok(slice)
}

fn field(hasher: &mut Sha256, tag: &str, value: &str) {
    hasher.update((tag.len() as u64).to_be_bytes());
    hasher.update(tag.as_bytes());
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value.as_bytes());
}

#[allow(clippy::too_many_arguments)]
fn hash_board(
    schema_version: u32,
    domain: &DomainIdentity,
    snapshot: &SnapshotIdentity,
    revision: &GraphRevision,
    position: &ResolvedPosition,
    candidates: &[CandidateSemanticSlice],
    policy: &str,
) -> String {
    let mut hasher = Sha256::new();
    field(&mut hasher, "domain", "sem-os-semantic-decision-board-v1");
    field(&mut hasher, "schema_version", &schema_version.to_string());
    field(&mut hasher, "domain_identity", domain.as_str());
    field(&mut hasher, "semantic_snapshot", snapshot.as_str());
    field(&mut hasher, "graph_revision", revision.as_str());
    match &position.anchor {
        Some(anchor) => field(&mut hasher, "position.anchor.some", anchor),
        None => field(&mut hasher, "position.anchor.none", ""),
    }
    field(&mut hasher, "position.context_hash", &position.context_hash);
    field(&mut hasher, "policy_fingerprint", policy);
    field(
        &mut hasher,
        "candidate_count",
        &candidates.len().to_string(),
    );
    for (candidate_index, candidate) in candidates.iter().enumerate() {
        let prefix = format!("candidate.{candidate_index}");
        field(
            &mut hasher,
            &format!("{prefix}.id"),
            candidate.canonical_id.as_str(),
        );
        field(
            &mut hasher,
            &format!("{prefix}.schema_version"),
            &candidate.schema_version.to_string(),
        );
        field(&mut hasher, &format!("{prefix}.title"), &candidate.title);
        field(
            &mut hasher,
            &format!("{prefix}.intent"),
            &candidate.intent_summary,
        );
        field(
            &mut hasher,
            &format!("{prefix}.action_class"),
            &format!("{:?}", candidate.action_class),
        );
        field(
            &mut hasher,
            &format!("{prefix}.applicability"),
            &candidate.applicability,
        );
        field(&mut hasher, &format!("{prefix}.effect"), &candidate.effect);
        field(
            &mut hasher,
            &format!("{prefix}.risk"),
            &format!("{:?}", candidate.risk),
        );
        field(
            &mut hasher,
            &format!("{prefix}.adapter_payload_hash"),
            &candidate.adapter_payload_hash,
        );
        field(
            &mut hasher,
            &format!("{prefix}.argument_count"),
            &candidate.arguments.len().to_string(),
        );
        for argument in &candidate.arguments {
            field(
                &mut hasher,
                &format!("{prefix}.argument.name"),
                &argument.name,
            );
            field(
                &mut hasher,
                &format!("{prefix}.argument.kind"),
                &format!("{:?}", argument.kind),
            );
            field(
                &mut hasher,
                &format!("{prefix}.argument.requirement"),
                if argument.required {
                    "required"
                } else {
                    "optional"
                },
            );
            field(
                &mut hasher,
                &format!("{prefix}.argument.prompt"),
                &argument.clarification_prompt,
            );
        }
        field(
            &mut hasher,
            &format!("{prefix}.phrase_count"),
            &candidate.phrases.len().to_string(),
        );
        for phrase in &candidate.phrases {
            field(&mut hasher, &format!("{prefix}.phrase.text"), &phrase.text);
            field(
                &mut hasher,
                &format!("{prefix}.phrase.locale"),
                &phrase.locale,
            );
            field(
                &mut hasher,
                &format!("{prefix}.phrase.role"),
                &format!("{:?}", phrase.role),
            );
            field(
                &mut hasher,
                &format!("{prefix}.phrase.provenance"),
                &phrase.provenance,
            );
        }
        field(
            &mut hasher,
            &format!("{prefix}.example_count"),
            &candidate.positive_examples.len().to_string(),
        );
        for example in &candidate.positive_examples {
            field(&mut hasher, &format!("{prefix}.example"), example);
        }
        field(
            &mut hasher,
            &format!("{prefix}.contrast_count"),
            &candidate.negative_contrasts.len().to_string(),
        );
        for contrast in &candidate.negative_contrasts {
            field(
                &mut hasher,
                &format!("{prefix}.contrast.id"),
                contrast.candidate_id.as_str(),
            );
            field(
                &mut hasher,
                &format!("{prefix}.contrast.distinction"),
                &contrast.distinction,
            );
        }
    }
    hex::encode(hasher.finalize())
}

/// Evidence producer lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLane {
    GovernedExact,
    DeterministicGrammar,
    Lexical,
    Embedding,
    CandleCrossEncoder,
}

/// Finite score wrapper.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FiniteScore(f64);

impl FiniteScore {
    /// Construct a score, refusing NaN and infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::FiniteScore;
    /// assert_eq!(FiniteScore::new(0.5).unwrap().get(), 0.5);
    /// assert!(FiniteScore::new(f64::NAN).is_err());
    /// ```
    pub fn new(value: f64) -> Result<Self, DecisionBoardError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(DecisionBoardError::NonFiniteScore)
        }
    }

    /// Return the finite scalar.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::FiniteScore;
    /// assert_eq!(FiniteScore::new(1.0).unwrap().get(), 1.0);
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}

impl Serialize for FiniteScore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for FiniteScore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(f64::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Score from one evidence lane.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneScore {
    pub lane: EvidenceLane,
    pub score: FiniteScore,
}

/// Complete evidence for one candidate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateEvidence {
    pub candidate_id: CanonicalCandidateId,
    pub lane_scores: Vec<LaneScore>,
    pub final_score: FiniteScore,
}

/// Complete board-bound inference evidence.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InferenceEvidence {
    pub board_hash: BoardHash,
    pub turn_context_hash: String,
    pub candidate_serializer_hash: String,
    pub producer_bundle_hash: String,
    pub ranked: Vec<CandidateEvidence>,
    pub evidence_record_hash: EvidenceRecordHash,
}

impl<'de> Deserialize<'de> for InferenceEvidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireEvidence {
            board_hash: BoardHash,
            turn_context_hash: String,
            candidate_serializer_hash: String,
            producer_bundle_hash: String,
            ranked: Vec<CandidateEvidence>,
            evidence_record_hash: EvidenceRecordHash,
        }

        let wire = WireEvidence::deserialize(deserializer)?;
        // `::new()` enforces two structural invariants before it ever
        // hashes: no candidate carries a duplicate evidence lane, and
        // `ranked` is sorted by descending final score (ties broken by
        // ascending candidate id). The hash-equality check below only
        // proves the wire bytes are self-consistent with *whatever* lane
        // set and order they happen to carry -- a duplicate-lane or
        // out-of-order payload can still compute a hash over itself that
        // matches its own `evidence_record_hash` field. Re-run both checks
        // here so a deserialized record carries the same guarantees as one
        // built through `::new()`, not just a self-consistent hash.
        for candidate in &wire.ranked {
            let mut lanes = candidate
                .lane_scores
                .iter()
                .map(|score| score.lane)
                .collect::<Vec<_>>();
            lanes.sort();
            if lanes.windows(2).any(|pair| pair[0] == pair[1]) {
                return Err(serde::de::Error::custom(format!(
                    "candidate '{}' has duplicate evidence lanes",
                    candidate.candidate_id.as_str()
                )));
            }
        }
        let is_canonically_ordered = wire.ranked.windows(2).all(|pair| {
            let [left, right] = pair else {
                unreachable!("windows(2) always yields two-element slices")
            };
            match right.final_score.partial_cmp(&left.final_score) {
                Some(std::cmp::Ordering::Less) => true,
                Some(std::cmp::Ordering::Equal) => left.candidate_id <= right.candidate_id,
                _ => false,
            }
        });
        if !is_canonically_ordered {
            return Err(serde::de::Error::custom(
                "inference evidence is not canonically ordered (descending final score, ties by ascending candidate id)",
            ));
        }
        let expected = hash_evidence(
            &wire.board_hash,
            &wire.turn_context_hash,
            &wire.candidate_serializer_hash,
            &wire.producer_bundle_hash,
            &wire.ranked,
        )
        .map_err(serde::de::Error::custom)?;
        if expected != wire.evidence_record_hash {
            return Err(serde::de::Error::custom(
                "inference evidence hash does not match canonical content",
            ));
        }
        Ok(Self {
            board_hash: wire.board_hash,
            turn_context_hash: wire.turn_context_hash,
            candidate_serializer_hash: wire.candidate_serializer_hash,
            producer_bundle_hash: wire.producer_bundle_hash,
            ranked: wire.ranked,
            evidence_record_hash: wire.evidence_record_hash,
        })
    }
}

impl InferenceEvidence {
    /// Validate and construct complete evidence for a board.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let board: SemanticDecisionBoard = todo!();
    /// # let ranked: Vec<CandidateEvidence> = todo!();
    /// let evidence = InferenceEvidence::new(&board, "turn", "serializer", "producer", ranked);
    /// assert!(evidence.is_ok());
    /// ```
    pub fn new(
        board: &SemanticDecisionBoard,
        turn_context_hash: impl Into<String>,
        candidate_serializer_hash: impl Into<String>,
        producer_bundle_hash: impl Into<String>,
        mut ranked: Vec<CandidateEvidence>,
    ) -> Result<Self, DecisionBoardError> {
        let turn_context_hash = turn_context_hash.into();
        let candidate_serializer_hash = candidate_serializer_hash.into();
        let producer_bundle_hash = producer_bundle_hash.into();
        validate_text("turn context hash", &turn_context_hash)?;
        validate_text("candidate serializer hash", &candidate_serializer_hash)?;
        validate_text("producer bundle hash", &producer_bundle_hash)?;
        for candidate in &mut ranked {
            candidate.lane_scores.sort_by_key(|score| score.lane);
            if candidate
                .lane_scores
                .windows(2)
                .any(|pair| pair[0].lane == pair[1].lane)
            {
                return Err(DecisionBoardError::InvalidEvidence(format!(
                    "candidate '{}' has duplicate evidence lanes",
                    candidate.candidate_id.as_str()
                )));
            }
        }
        let board_ids = board
            .candidates
            .iter()
            .map(|candidate| candidate.canonical_id.clone())
            .collect::<BTreeSet<_>>();
        let evidence_ids = ranked
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect::<BTreeSet<_>>();
        if ranked.len() != evidence_ids.len() || board_ids != evidence_ids {
            return Err(DecisionBoardError::InvalidEvidence(
                "ranking must contain every board id exactly once".to_string(),
            ));
        }
        ranked.sort_by(|left, right| {
            right
                .final_score
                .partial_cmp(&left.final_score)
                .expect("FiniteScore ordering is total")
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
        });
        let evidence_record_hash = hash_evidence(
            &board.board_hash,
            &turn_context_hash,
            &candidate_serializer_hash,
            &producer_bundle_hash,
            &ranked,
        )?;
        Ok(Self {
            board_hash: board.board_hash.clone(),
            turn_context_hash,
            candidate_serializer_hash,
            producer_bundle_hash,
            ranked,
            evidence_record_hash,
        })
    }
}

fn hash_evidence(
    board_hash: &BoardHash,
    turn_context_hash: &str,
    candidate_serializer_hash: &str,
    producer_bundle_hash: &str,
    ranked: &[CandidateEvidence],
) -> Result<EvidenceRecordHash, DecisionBoardError> {
    let mut hasher = Sha256::new();
    field(&mut hasher, "domain", "sem-os-inference-evidence-v1");
    field(&mut hasher, "board_hash", board_hash.as_str());
    field(&mut hasher, "turn_context_hash", turn_context_hash);
    field(
        &mut hasher,
        "candidate_serializer_hash",
        candidate_serializer_hash,
    );
    field(&mut hasher, "producer_bundle_hash", producer_bundle_hash);
    field(&mut hasher, "ranked_count", &ranked.len().to_string());
    for (candidate_index, candidate) in ranked.iter().enumerate() {
        let prefix = format!("ranked.{candidate_index}");
        field(
            &mut hasher,
            &format!("{prefix}.candidate_id"),
            candidate.candidate_id.as_str(),
        );
        field(
            &mut hasher,
            &format!("{prefix}.final_score"),
            &candidate.final_score.get().to_bits().to_string(),
        );
        field(
            &mut hasher,
            &format!("{prefix}.lane_count"),
            &candidate.lane_scores.len().to_string(),
        );
        for lane in &candidate.lane_scores {
            field(
                &mut hasher,
                &format!("{prefix}.lane"),
                &format!("{:?}", lane.lane),
            );
            field(
                &mut hasher,
                &format!("{prefix}.lane_score"),
                &lane.score.get().to_bits().to_string(),
            );
        }
    }
    EvidenceRecordHash::new(hex::encode(hasher.finalize()))
}

/// Versioned deterministic disposition thresholds.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DispositionPolicy {
    schema_version: u32,
    candidate_threshold: FiniteScore,
    clarify_margin: FiniteScore,
    fingerprint: String,
}

impl<'de> Deserialize<'de> for DispositionPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WirePolicy {
            schema_version: u32,
            candidate_threshold: FiniteScore,
            clarify_margin: FiniteScore,
            fingerprint: String,
        }

        let wire = WirePolicy::deserialize(deserializer)?;
        let rebuilt = Self::new(
            wire.schema_version,
            wire.candidate_threshold.get(),
            wire.clarify_margin.get(),
        )
        .map_err(serde::de::Error::custom)?;
        if rebuilt.fingerprint != wire.fingerprint {
            return Err(serde::de::Error::custom(
                "disposition policy fingerprint does not match its content",
            ));
        }
        Ok(rebuilt)
    }
}

impl DispositionPolicy {
    /// Construct a content-addressed disposition policy.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::DispositionPolicy;
    /// let policy = DispositionPolicy::new(1, 0.7, 0.05).unwrap();
    /// assert_eq!(policy.schema_version(), 1);
    /// ```
    pub fn new(
        schema_version: u32,
        candidate_threshold: f64,
        clarify_margin: f64,
    ) -> Result<Self, DecisionBoardError> {
        if !(0.0..=1.0).contains(&candidate_threshold) || !(0.0..=1.0).contains(&clarify_margin) {
            return Err(DecisionBoardError::InvalidPolicy(
                "threshold and margin must be between zero and one".to_string(),
            ));
        }
        let candidate_threshold = FiniteScore::new(candidate_threshold)?;
        let clarify_margin = FiniteScore::new(clarify_margin)?;
        let mut hasher = Sha256::new();
        field(&mut hasher, "domain", "sem-os-disposition-policy-v1");
        field(&mut hasher, "schema_version", &schema_version.to_string());
        field(
            &mut hasher,
            "candidate_threshold",
            &candidate_threshold.get().to_bits().to_string(),
        );
        field(
            &mut hasher,
            "clarify_margin",
            &clarify_margin.get().to_bits().to_string(),
        );
        Ok(Self {
            schema_version,
            candidate_threshold,
            clarify_margin,
            fingerprint: hex::encode(hasher.finalize()),
        })
    }

    /// Return the policy schema version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::DispositionPolicy;
    /// assert_eq!(DispositionPolicy::new(2, 0.7, 0.05).unwrap().schema_version(), 2);
    /// ```
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Borrow the content fingerprint.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sem_os_policy::decision_board::DispositionPolicy;
    /// assert_eq!(DispositionPolicy::new(1, 0.7, 0.05).unwrap().fingerprint().len(), 64);
    /// ```
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

/// Deterministic inference result, separate from binding state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InferenceDisposition {
    Candidate {
        candidate_id: CanonicalCandidateId,
    },
    ClarifyCandidates {
        candidates: Vec<CanonicalCandidateId>,
    },
    OutOfScope,
    Escalate {
        reason: String,
    },
}

/// Reproducible output of deterministic disposition policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub board_hash: BoardHash,
    pub evidence_record_hash: EvidenceRecordHash,
    pub policy_fingerprint: String,
    pub disposition: InferenceDisposition,
    pub decision_record_hash: EvidenceRecordHash,
}

/// Apply deterministic policy to complete board-bound evidence.
///
/// # Examples
///
/// ```no_run
/// # use sem_os_policy::decision_board::*;
/// # let board: SemanticDecisionBoard = todo!();
/// # let evidence: InferenceEvidence = todo!();
/// let policy = DispositionPolicy::new(1, 0.7, 0.05).unwrap();
/// let record = decide(&policy, &board, &evidence).unwrap();
/// assert_eq!(record.board_hash, board.board_hash);
/// ```
pub fn decide(
    policy: &DispositionPolicy,
    board: &SemanticDecisionBoard,
    evidence: &InferenceEvidence,
) -> Result<DecisionRecord, DecisionBoardError> {
    if evidence.board_hash != board.board_hash {
        return Err(DecisionBoardError::InvalidEvidence(
            "evidence board hash does not match the decision board".to_string(),
        ));
    }
    let top = evidence.ranked.first().ok_or_else(|| {
        DecisionBoardError::InvalidEvidence("ranking must not be empty".to_string())
    })?;
    let disposition = if top.candidate_id.as_str() == ABSTENTION_CANDIDATE_ID
        || top.final_score.get() < policy.candidate_threshold.get()
    {
        InferenceDisposition::OutOfScope
    } else if let Some(second) = evidence.ranked.get(1) {
        if top.final_score.get() - second.final_score.get() <= policy.clarify_margin.get() {
            InferenceDisposition::ClarifyCandidates {
                candidates: vec![top.candidate_id.clone(), second.candidate_id.clone()],
            }
        } else {
            InferenceDisposition::Candidate {
                candidate_id: top.candidate_id.clone(),
            }
        }
    } else {
        InferenceDisposition::Candidate {
            candidate_id: top.candidate_id.clone(),
        }
    };
    let mut hasher = Sha256::new();
    field(&mut hasher, "domain", "sem-os-decision-record-v1");
    field(&mut hasher, "board_hash", board.board_hash.as_str());
    field(
        &mut hasher,
        "evidence_record_hash",
        evidence.evidence_record_hash.as_str(),
    );
    field(&mut hasher, "policy_fingerprint", policy.fingerprint());
    match &disposition {
        InferenceDisposition::Candidate { candidate_id } => {
            field(&mut hasher, "disposition.candidate", candidate_id.as_str());
        }
        InferenceDisposition::ClarifyCandidates { candidates } => {
            field(
                &mut hasher,
                "disposition.clarify_count",
                &candidates.len().to_string(),
            );
            for candidate in candidates {
                field(&mut hasher, "disposition.clarify", candidate.as_str());
            }
        }
        InferenceDisposition::OutOfScope => {
            field(&mut hasher, "disposition.out_of_scope", "");
        }
        InferenceDisposition::Escalate { reason } => {
            field(&mut hasher, "disposition.escalate", reason);
        }
    }
    Ok(DecisionRecord {
        board_hash: board.board_hash.clone(),
        evidence_record_hash: evidence.evidence_record_hash.clone(),
        policy_fingerprint: policy.fingerprint().to_string(),
        disposition,
        decision_record_hash: EvidenceRecordHash::new(hex::encode(hasher.finalize()))?,
    })
}

/// Closed proposal-workbook state set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    NeedsArguments,
    ReadyForDryRun,
    DryRunRefused,
    ReadyForRatification,
    Ratified,
    Rejected,
    Expired,
}

/// Slot requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotRequirement {
    Required,
    Optional,
}

/// Typed value accepted by a workbook slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SlotValue {
    Text(String),
    Identifier(String),
    NodeReference(String),
    DataReference(String),
    Count(u32),
    DurationMillis(u64),
    Condition(String),
    SubprocessReference(String),
    Boolean(bool),
}

/// Resolution state of one slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", content = "value", rename_all = "snake_case")]
pub enum SlotValueState {
    Missing,
    Resolved(SlotValue),
}

/// Provenance for a deterministic binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingProvenance {
    pub source: String,
    pub detail: String,
}

/// One typed proposal slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkbookSlot {
    pub name: String,
    pub kind: ArgumentKind,
    pub requirement: SlotRequirement,
    pub value: SlotValueState,
    pub provenance: Option<BindingProvenance>,
    pub clarification_prompt: String,
}

/// Resumable, graph- and evidence-bound proposal workbook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposalWorkbook {
    pub schema_version: u32,
    pub workbook_id: WorkbookId,
    pub source_utterance_seq: u64,
    pub board_hash: BoardHash,
    pub graph_revision: GraphRevision,
    pub candidate_id: CanonicalCandidateId,
    slots: Vec<WorkbookSlot>,
    status: ProposalStatus,
    pub evidence_record_hash: EvidenceRecordHash,
}

impl ProposalWorkbook {
    /// Construct a validated workbook whose initial status reflects slot completeness.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// let workbook = ProposalWorkbook::new(
    ///     1, WorkbookId::new("workbook").unwrap(), 1,
    ///     BoardHash::new("0".repeat(64)).unwrap(), GraphRevision::new("rev").unwrap(),
    ///     CanonicalCandidateId::new("op.append_node").unwrap(), vec![],
    ///     EvidenceRecordHash::new("e".repeat(64)).unwrap(),
    /// ).unwrap();
    /// assert_eq!(workbook.status(), ProposalStatus::ReadyForDryRun);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_version: u32,
        workbook_id: WorkbookId,
        source_utterance_seq: u64,
        board_hash: BoardHash,
        graph_revision: GraphRevision,
        candidate_id: CanonicalCandidateId,
        mut slots: Vec<WorkbookSlot>,
        evidence_record_hash: EvidenceRecordHash,
    ) -> Result<Self, DecisionBoardError> {
        let mut names = BTreeSet::new();
        for slot in &mut slots {
            validate_text("workbook slot name", &slot.name)?;
            validate_text("workbook clarification prompt", &slot.clarification_prompt)?;
            if !names.insert(slot.name.clone()) {
                return Err(DecisionBoardError::InvalidSlotAnswer(format!(
                    "duplicate workbook slot '{}'",
                    slot.name
                )));
            }
            if let SlotValueState::Resolved(value) = &slot.value {
                if !slot_value_matches(slot.kind, value) {
                    return Err(DecisionBoardError::InvalidSlotAnswer(format!(
                        "resolved value does not match slot '{}'",
                        slot.name
                    )));
                }
            }
        }
        slots.sort_by(|left, right| left.name.cmp(&right.name));
        let complete = required_slots_resolved(&slots);
        Ok(Self {
            schema_version,
            workbook_id,
            source_utterance_seq,
            board_hash,
            graph_revision,
            candidate_id,
            slots,
            status: if complete {
                ProposalStatus::ReadyForDryRun
            } else {
                ProposalStatus::NeedsArguments
            },
            evidence_record_hash,
        })
    }

    /// Return the current closed workbook state.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let workbook: ProposalWorkbook = todo!();
    /// let _ = workbook.status();
    /// ```
    pub fn status(&self) -> ProposalStatus {
        self.status
    }

    /// Borrow the declared slots.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let workbook: ProposalWorkbook = todo!();
    /// let _ = workbook.slots();
    /// ```
    pub fn slots(&self) -> &[WorkbookSlot] {
        &self.slots
    }

    /// Apply a legal state transition from the closed transition table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let mut workbook: ProposalWorkbook = todo!();
    /// workbook.transition(ProposalStatus::ReadyForDryRun).unwrap();
    /// ```
    pub fn transition(&mut self, to: ProposalStatus) -> Result<(), DecisionBoardError> {
        let legal = matches!(
            (self.status, to),
            (
                ProposalStatus::NeedsArguments,
                ProposalStatus::ReadyForDryRun
            ) | (ProposalStatus::NeedsArguments, ProposalStatus::Rejected)
                | (ProposalStatus::NeedsArguments, ProposalStatus::Expired)
                | (
                    ProposalStatus::ReadyForDryRun,
                    ProposalStatus::DryRunRefused
                )
                | (
                    ProposalStatus::ReadyForDryRun,
                    ProposalStatus::ReadyForRatification
                )
                | (ProposalStatus::ReadyForDryRun, ProposalStatus::Expired)
                | (ProposalStatus::DryRunRefused, ProposalStatus::Rejected)
                | (ProposalStatus::DryRunRefused, ProposalStatus::Expired)
                | (
                    ProposalStatus::ReadyForRatification,
                    ProposalStatus::Ratified
                )
                | (
                    ProposalStatus::ReadyForRatification,
                    ProposalStatus::Rejected
                )
                | (
                    ProposalStatus::ReadyForRatification,
                    ProposalStatus::Expired
                )
        );
        if !legal {
            return Err(DecisionBoardError::IllegalWorkbookTransition {
                from: self.status,
                to,
            });
        }
        self.status = to;
        Ok(())
    }

    /// Apply explicit answers atomically after type and name validation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sem_os_policy::decision_board::*;
    /// # let mut workbook: ProposalWorkbook = todo!();
    /// workbook.apply_answers(vec![("name".into(), SlotValue::Identifier("task".into()))]).unwrap();
    /// ```
    pub fn apply_answers(
        &mut self,
        answers: Vec<(String, SlotValue)>,
    ) -> Result<(), DecisionBoardError> {
        let mut seen = BTreeSet::new();
        let by_name = self
            .slots
            .iter()
            .enumerate()
            .map(|(index, slot)| (slot.name.clone(), index))
            .collect::<BTreeMap<_, _>>();
        let mut updates = Vec::new();
        for (name, value) in answers {
            if !seen.insert(name.clone()) {
                return Err(DecisionBoardError::InvalidSlotAnswer(format!(
                    "duplicate answer for '{name}'"
                )));
            }
            let index = by_name.get(&name).copied().ok_or_else(|| {
                DecisionBoardError::InvalidSlotAnswer(format!("unknown slot '{name}'"))
            })?;
            if !slot_value_matches(self.slots[index].kind, &value) {
                return Err(DecisionBoardError::InvalidSlotAnswer(format!(
                    "answer kind does not match slot '{name}'"
                )));
            }
            if matches!(self.slots[index].value, SlotValueState::Resolved(_)) {
                return Err(DecisionBoardError::InvalidSlotAnswer(format!(
                    "slot '{name}' is already resolved"
                )));
            }
            updates.push((index, value));
        }
        for (index, value) in updates {
            self.slots[index].value = SlotValueState::Resolved(value);
            self.slots[index].provenance = Some(BindingProvenance {
                source: "explicit_answer".to_string(),
                detail: "typed user answer".to_string(),
            });
        }
        if required_slots_resolved(&self.slots) && self.status == ProposalStatus::NeedsArguments {
            self.transition(ProposalStatus::ReadyForDryRun)?;
        }
        Ok(())
    }
}

fn required_slots_resolved(slots: &[WorkbookSlot]) -> bool {
    slots.iter().all(|slot| {
        slot.requirement == SlotRequirement::Optional
            || matches!(slot.value, SlotValueState::Resolved(_))
    })
}

fn slot_value_matches(kind: ArgumentKind, value: &SlotValue) -> bool {
    matches!(
        (kind, value),
        (ArgumentKind::Text, SlotValue::Text(_))
            | (ArgumentKind::Identifier, SlotValue::Identifier(_))
            | (ArgumentKind::NodeReference, SlotValue::NodeReference(_))
            | (ArgumentKind::DataReference, SlotValue::DataReference(_))
            | (ArgumentKind::Count, SlotValue::Count(_))
            | (ArgumentKind::Duration, SlotValue::DurationMillis(_))
            | (ArgumentKind::Condition, SlotValue::Condition(_))
            | (
                ArgumentKind::SubprocessReference,
                SlotValue::SubprocessReference(_)
            )
            | (ArgumentKind::Boolean, SlotValue::Boolean(_))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(id: &str) -> CandidateSemanticSlice {
        CandidateSemanticSlice {
            canonical_id: CanonicalCandidateId::new(id).unwrap(),
            schema_version: 1,
            title: "Append task".into(),
            intent_summary: "Append one task".into(),
            action_class: ActionClass::Create,
            applicability: "At a sequence end".into(),
            effect: "Adds one task".into(),
            arguments: vec![],
            phrases: vec![],
            positive_examples: vec!["Add a task".into()],
            negative_contrasts: vec![],
            risk: HarmClass::Reversible,
            adapter_payload_hash: "adapter-v1".into(),
        }
    }

    fn board() -> SemanticDecisionBoard {
        SemanticDecisionBoard::new(
            1,
            DomainIdentity::new("bpmn-authoring").unwrap(),
            SnapshotIdentity::new("pack-v1").unwrap(),
            GraphRevision::new("rev-1").unwrap(),
            ResolvedPosition {
                anchor: Some("task-1".into()),
                context_hash: "context-1".into(),
            },
            vec![candidate("op.append_node")],
            "policy-v1".into(),
        )
        .unwrap()
    }

    fn complete_evidence(board: &SemanticDecisionBoard) -> InferenceEvidence {
        let ranked = board
            .candidates
            .iter()
            .map(|candidate| CandidateEvidence {
                candidate_id: candidate.canonical_id.clone(),
                lane_scores: vec![LaneScore {
                    lane: EvidenceLane::Lexical,
                    score: FiniteScore::new(
                        if candidate.canonical_id.as_str() == "op.append_node" {
                            0.9
                        } else {
                            0.1
                        },
                    )
                    .unwrap(),
                }],
                final_score: FiniteScore::new(
                    if candidate.canonical_id.as_str() == "op.append_node" {
                        0.9
                    } else {
                        0.1
                    },
                )
                .unwrap(),
            })
            .collect();
        InferenceEvidence::new(board, "turn", "serializer", "producer", ranked).unwrap()
    }

    #[test]
    fn board_hash_tracks_every_semantic_change() {
        let first = board();
        let mut changed = candidate("op.append_node");
        changed.effect = "Adds two tasks".into();
        let second = SemanticDecisionBoard::new(
            1,
            first.domain.clone(),
            first.semantic_snapshot.clone(),
            first.graph_revision.clone(),
            first.position.clone(),
            vec![changed],
            first.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(first.board_hash, second.board_hash);
    }

    #[test]
    fn duplicate_and_reserved_candidates_are_refused() {
        assert!(SemanticDecisionBoard::new(
            1,
            DomainIdentity::new("bpmn").unwrap(),
            SnapshotIdentity::new("pack").unwrap(),
            GraphRevision::new("rev").unwrap(),
            ResolvedPosition {
                anchor: None,
                context_hash: "ctx".into()
            },
            vec![candidate("op.x"), candidate("op.x")],
            "policy".into(),
        )
        .is_err());
        assert!(SemanticDecisionBoard::new(
            1,
            DomainIdentity::new("bpmn").unwrap(),
            SnapshotIdentity::new("pack").unwrap(),
            GraphRevision::new("rev").unwrap(),
            ResolvedPosition {
                anchor: None,
                context_hash: "ctx".into()
            },
            vec![candidate(ABSTENTION_CANDIDATE_ID)],
            "policy".into(),
        )
        .is_err());
    }

    #[test]
    fn off_board_and_incomplete_evidence_are_refused() {
        let board = board();
        let only = CandidateEvidence {
            candidate_id: CanonicalCandidateId::new("op.append_node").unwrap(),
            lane_scores: vec![],
            final_score: FiniteScore::new(1.0).unwrap(),
        };
        assert!(
            InferenceEvidence::new(&board, "turn", "serializer", "producer", vec![only]).is_err()
        );
    }

    #[test]
    fn direct_needs_arguments_to_ratified_is_refused() {
        let mut workbook = ProposalWorkbook::new(
            1,
            WorkbookId::new("workbook").unwrap(),
            1,
            board().board_hash,
            GraphRevision::new("rev").unwrap(),
            CanonicalCandidateId::new("op.append_node").unwrap(),
            vec![WorkbookSlot {
                name: "task_name".to_string(),
                kind: ArgumentKind::Identifier,
                requirement: SlotRequirement::Required,
                value: SlotValueState::Missing,
                provenance: None,
                clarification_prompt: "What should the task be called?".to_string(),
            }],
            EvidenceRecordHash::new("e".repeat(64)).unwrap(),
        )
        .unwrap();
        assert_eq!(workbook.status(), ProposalStatus::NeedsArguments);
        assert!(workbook.transition(ProposalStatus::Ratified).is_err());
    }

    #[test]
    fn workbook_transition_table_is_closed_for_every_status_pair() {
        let statuses = [
            ProposalStatus::NeedsArguments,
            ProposalStatus::ReadyForDryRun,
            ProposalStatus::DryRunRefused,
            ProposalStatus::ReadyForRatification,
            ProposalStatus::Ratified,
            ProposalStatus::Rejected,
            ProposalStatus::Expired,
        ];
        let base = ProposalWorkbook::new(
            1,
            WorkbookId::new("workbook").unwrap(),
            1,
            board().board_hash,
            GraphRevision::new("rev").unwrap(),
            CanonicalCandidateId::new("op.append_node").unwrap(),
            vec![],
            EvidenceRecordHash::new("e".repeat(64)).unwrap(),
        )
        .unwrap();

        for from in statuses {
            for to in statuses {
                let expected = matches!(
                    (from, to),
                    (
                        ProposalStatus::NeedsArguments,
                        ProposalStatus::ReadyForDryRun
                    ) | (ProposalStatus::NeedsArguments, ProposalStatus::Rejected)
                        | (ProposalStatus::NeedsArguments, ProposalStatus::Expired)
                        | (
                            ProposalStatus::ReadyForDryRun,
                            ProposalStatus::DryRunRefused
                        )
                        | (
                            ProposalStatus::ReadyForDryRun,
                            ProposalStatus::ReadyForRatification
                        )
                        | (ProposalStatus::ReadyForDryRun, ProposalStatus::Expired)
                        | (ProposalStatus::DryRunRefused, ProposalStatus::Rejected)
                        | (ProposalStatus::DryRunRefused, ProposalStatus::Expired)
                        | (
                            ProposalStatus::ReadyForRatification,
                            ProposalStatus::Ratified
                        )
                        | (
                            ProposalStatus::ReadyForRatification,
                            ProposalStatus::Rejected
                        )
                        | (
                            ProposalStatus::ReadyForRatification,
                            ProposalStatus::Expired
                        )
                );
                let mut workbook = base.clone();
                workbook.status = from;
                assert_eq!(
                    workbook.transition(to).is_ok(),
                    expected,
                    "unexpected transition result for {from:?} -> {to:?}"
                );
            }
        }
    }

    #[test]
    fn disposition_is_board_bound_and_policy_content_addressed() {
        let board = board();
        let evidence = complete_evidence(&board);
        let policy = DispositionPolicy::new(1, 0.7, 0.05).unwrap();
        let changed_policy = DispositionPolicy::new(1, 0.8, 0.05).unwrap();
        assert_ne!(policy.fingerprint(), changed_policy.fingerprint());

        let record = decide(&policy, &board, &evidence).unwrap();
        assert_eq!(
            record.disposition,
            InferenceDisposition::Candidate {
                candidate_id: CanonicalCandidateId::new("op.append_node").unwrap()
            }
        );

        let mut wrong = evidence.clone();
        wrong.board_hash = BoardHash::new("f".repeat(64)).unwrap();
        assert!(decide(&policy, &board, &wrong).is_err());
    }

    #[test]
    fn serde_revalidates_content_hashes() {
        let board = board();
        let encoded = serde_json::to_value(&board).unwrap();
        let decoded: SemanticDecisionBoard = serde_json::from_value(encoded.clone()).unwrap();
        assert_eq!(decoded, board);

        let mut tampered = encoded;
        tampered["candidates"][0]["effect"] = serde_json::json!("tampered effect");
        assert!(serde_json::from_value::<SemanticDecisionBoard>(tampered).is_err());

        let evidence = complete_evidence(&board);
        let encoded = serde_json::to_value(&evidence).unwrap();
        let decoded: InferenceEvidence = serde_json::from_value(encoded.clone()).unwrap();
        assert_eq!(decoded, evidence);
        let mut tampered = encoded;
        tampered["ranked"][0]["final_score"] = serde_json::json!(0.2);
        assert!(serde_json::from_value::<InferenceEvidence>(tampered).is_err());

        let policy = DispositionPolicy::new(1, 0.7, 0.05).unwrap();
        let encoded = serde_json::to_value(&policy).unwrap();
        let decoded: DispositionPolicy = serde_json::from_value(encoded.clone()).unwrap();
        assert_eq!(decoded, policy);
        let mut tampered = encoded;
        tampered["candidate_threshold"] = serde_json::json!(0.6);
        assert!(serde_json::from_value::<DispositionPolicy>(tampered).is_err());
    }

    /// A candidate with every model-visible list populated (one argument,
    /// one phrase, one positive example, one negative contrast) so the
    /// per-field hash-sensitivity test below can mutate each field
    /// independently.
    fn rich_candidate() -> CandidateSemanticSlice {
        CandidateSemanticSlice {
            canonical_id: CanonicalCandidateId::new("op.append_node").unwrap(),
            schema_version: 1,
            title: "Append task".into(),
            intent_summary: "Append one task".into(),
            action_class: ActionClass::Create,
            applicability: "At a sequence end".into(),
            effect: "Adds one task".into(),
            arguments: vec![ArgumentSpec {
                name: "task_name".into(),
                kind: ArgumentKind::Identifier,
                required: true,
                clarification_prompt: "What should the task be called?".into(),
            }],
            phrases: vec![PhraseEvidence {
                text: "add a task".into(),
                locale: "en".into(),
                role: PhraseRole::Canonical,
                provenance: "authored".into(),
            }],
            positive_examples: vec!["Add a task at the end".into()],
            negative_contrasts: vec![NegativeContrast {
                candidate_id: CanonicalCandidateId::new("op.insert_after").unwrap(),
                distinction: "append always targets the sequence end".into(),
            }],
            risk: HarmClass::Reversible,
            adapter_payload_hash: "adapter-v1".into(),
        }
    }

    fn board_with(candidates: Vec<CandidateSemanticSlice>) -> SemanticDecisionBoard {
        SemanticDecisionBoard::new(
            1,
            DomainIdentity::new("bpmn-authoring").unwrap(),
            SnapshotIdentity::new("pack-v1").unwrap(),
            GraphRevision::new("rev-1").unwrap(),
            ResolvedPosition {
                anchor: Some("task-1".into()),
                context_hash: "context-1".into(),
            },
            candidates,
            "policy-v1".into(),
        )
        .unwrap()
    }

    fn rich_board() -> SemanticDecisionBoard {
        board_with(vec![rich_candidate()])
    }

    #[test]
    fn board_hash_moves_for_every_model_visible_field() {
        let baseline = rich_board();

        // Board-level fields (everything but candidates/schema_version,
        // exercised separately below).
        let domain_changed = SemanticDecisionBoard::new(
            baseline.schema_version,
            DomainIdentity::new("other-domain").unwrap(),
            baseline.semantic_snapshot.clone(),
            baseline.graph_revision.clone(),
            baseline.position.clone(),
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(baseline.board_hash, domain_changed.board_hash, "domain");

        let snapshot_changed = SemanticDecisionBoard::new(
            baseline.schema_version,
            baseline.domain.clone(),
            SnapshotIdentity::new("other-snapshot").unwrap(),
            baseline.graph_revision.clone(),
            baseline.position.clone(),
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, snapshot_changed.board_hash,
            "semantic_snapshot"
        );

        let revision_changed = SemanticDecisionBoard::new(
            baseline.schema_version,
            baseline.domain.clone(),
            baseline.semantic_snapshot.clone(),
            GraphRevision::new("other-rev").unwrap(),
            baseline.position.clone(),
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, revision_changed.board_hash,
            "graph_revision"
        );

        let schema_version_changed = SemanticDecisionBoard::new(
            baseline.schema_version + 1,
            baseline.domain.clone(),
            baseline.semantic_snapshot.clone(),
            baseline.graph_revision.clone(),
            baseline.position.clone(),
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, schema_version_changed.board_hash,
            "board schema_version"
        );

        let anchor_none = SemanticDecisionBoard::new(
            baseline.schema_version,
            baseline.domain.clone(),
            baseline.semantic_snapshot.clone(),
            baseline.graph_revision.clone(),
            ResolvedPosition {
                anchor: None,
                context_hash: baseline.position.context_hash.clone(),
            },
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, anchor_none.board_hash,
            "position.anchor Some(_) vs None"
        );

        let another_anchor = SemanticDecisionBoard::new(
            baseline.schema_version,
            baseline.domain.clone(),
            baseline.semantic_snapshot.clone(),
            baseline.graph_revision.clone(),
            ResolvedPosition {
                anchor: Some("task-2".into()),
                context_hash: baseline.position.context_hash.clone(),
            },
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, another_anchor.board_hash,
            "position.anchor value"
        );

        let context_hash_changed = SemanticDecisionBoard::new(
            baseline.schema_version,
            baseline.domain.clone(),
            baseline.semantic_snapshot.clone(),
            baseline.graph_revision.clone(),
            ResolvedPosition {
                anchor: baseline.position.anchor.clone(),
                context_hash: "other-context".into(),
            },
            vec![rich_candidate()],
            baseline.policy_fingerprint.clone(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, context_hash_changed.board_hash,
            "position.context_hash"
        );

        let policy_changed = SemanticDecisionBoard::new(
            baseline.schema_version,
            baseline.domain.clone(),
            baseline.semantic_snapshot.clone(),
            baseline.graph_revision.clone(),
            baseline.position.clone(),
            vec![rich_candidate()],
            "other-policy".into(),
        )
        .unwrap();
        assert_ne!(
            baseline.board_hash, policy_changed.board_hash,
            "policy_fingerprint"
        );

        // Per-candidate fields: apply one mutation at a time to a clone of
        // the rich candidate and rebuild the board.
        let mutations: Vec<(&str, Box<dyn Fn(&mut CandidateSemanticSlice)>)> = vec![
            (
                "candidate schema_version",
                Box::new(|c| c.schema_version += 1),
            ),
            ("title", Box::new(|c| c.title = "Other title".into())),
            (
                "intent_summary",
                Box::new(|c| c.intent_summary = "Other intent".into()),
            ),
            (
                "action_class",
                Box::new(|c| c.action_class = ActionClass::Delete),
            ),
            (
                "applicability",
                Box::new(|c| c.applicability = "Elsewhere".into()),
            ),
            ("risk", Box::new(|c| c.risk = HarmClass::Destructive)),
            (
                "adapter_payload_hash",
                Box::new(|c| c.adapter_payload_hash = "adapter-v2".into()),
            ),
            (
                "argument name",
                Box::new(|c| c.arguments[0].name = "other_name".into()),
            ),
            (
                "argument kind",
                Box::new(|c| c.arguments[0].kind = ArgumentKind::Count),
            ),
            (
                "argument requirement",
                Box::new(|c| c.arguments[0].required = false),
            ),
            (
                "argument clarification_prompt",
                Box::new(|c| c.arguments[0].clarification_prompt = "Other prompt?".into()),
            ),
            (
                "phrase text",
                Box::new(|c| c.phrases[0].text = "other phrase".into()),
            ),
            (
                "phrase locale",
                Box::new(|c| c.phrases[0].locale = "fr".into()),
            ),
            (
                "phrase role",
                Box::new(|c| c.phrases[0].role = PhraseRole::Paraphrase),
            ),
            (
                "phrase provenance",
                Box::new(|c| c.phrases[0].provenance = "other-provenance".into()),
            ),
            (
                "positive_examples",
                Box::new(|c| c.positive_examples[0] = "Other example".into()),
            ),
            (
                "negative_contrasts candidate_id",
                Box::new(|c| {
                    c.negative_contrasts[0].candidate_id =
                        CanonicalCandidateId::new("op.replace_node").unwrap()
                }),
            ),
            (
                "negative_contrasts distinction",
                Box::new(|c| c.negative_contrasts[0].distinction = "other distinction".into()),
            ),
        ];
        for (label, mutate) in mutations {
            let mut mutated = rich_candidate();
            mutate(&mut mutated);
            let moved = board_with(vec![mutated]);
            assert_ne!(
                baseline.board_hash, moved.board_hash,
                "expected board hash to move when {label} changes"
            );
        }
    }

    #[test]
    fn board_hash_is_deterministic_and_input_order_independent() {
        let a = candidate("op.append_node");
        let b = candidate("op.insert_after");
        let forward = board_with(vec![a.clone(), b.clone()]);
        let reversed = board_with(vec![b, a]);
        assert_eq!(
            forward.board_hash, reversed.board_hash,
            "candidate input order must not affect the board hash (constructor sorts by canonical_id)"
        );

        let again = board_with(vec![
            candidate("op.append_node"),
            candidate("op.insert_after"),
        ]);
        assert_eq!(
            forward.board_hash, again.board_hash,
            "identical inputs must reproduce the identical hash"
        );
    }

    #[test]
    fn length_prefix_framing_prevents_field_boundary_collisions() {
        // Directly exercise `field()`: without a length prefix,
        // ("ab", "c") and ("a", "bc") hash identically once concatenated.
        // The length-prefixed encoder must distinguish them.
        let mut left = Sha256::new();
        field(&mut left, "ab", "c");
        let mut right = Sha256::new();
        field(&mut right, "a", "bc");
        assert_ne!(left.finalize(), right.finalize());

        // Same collision shape at the board level: a candidate whose
        // title/intent_summary boundary shifts by one character must move
        // the hash even though the naive concatenation of the two fields
        // is identical.
        let mut shifted = rich_candidate();
        shifted.title = "Append tas".into();
        shifted.intent_summary = "kAppend one task".into();
        assert_eq!(
            format!("{}{}", rich_candidate().title, rich_candidate().intent_summary),
            format!("{}{}", shifted.title, shifted.intent_summary),
            "fixture must actually collide under naive concatenation for this test to be meaningful"
        );
        let baseline = rich_board();
        let moved = board_with(vec![shifted]);
        assert_ne!(baseline.board_hash, moved.board_hash);
    }

    #[test]
    fn identity_constructors_reject_empty_and_control_characters() {
        for empty in [""] {
            assert!(CanonicalCandidateId::new(empty).is_err());
            assert!(DomainIdentity::new(empty).is_err());
            assert!(SnapshotIdentity::new(empty).is_err());
            assert!(GraphRevision::new(empty).is_err());
            assert!(WorkbookId::new(empty).is_err());
        }
        for control in ["a\nb", "a\tb", "a\0b", "a\x1bb"] {
            assert!(
                CanonicalCandidateId::new(control).is_err(),
                "{control:?} must be rejected"
            );
            assert!(DomainIdentity::new(control).is_err());
            assert!(SnapshotIdentity::new(control).is_err());
            assert!(GraphRevision::new(control).is_err());
            assert!(WorkbookId::new(control).is_err());
        }
        assert!(CanonicalCandidateId::new("op.append_node").is_ok());
        assert!(DomainIdentity::new("bpmn").is_ok());
    }

    #[test]
    fn hash_wrappers_reject_malformed_hex() {
        let valid = "a".repeat(64);
        assert!(BoardHash::new(valid.clone()).is_ok());
        assert!(EvidenceRecordHash::new(valid.clone()).is_ok());

        for malformed in [
            String::new(),                  // empty
            "a".repeat(63),                 // too short
            "a".repeat(65),                 // too long
            "A".repeat(64),                 // uppercase hex
            format!("{}g", "a".repeat(63)), // non-hex character
        ] {
            assert!(
                BoardHash::new(malformed.clone()).is_err(),
                "{malformed:?} must be rejected as a board hash"
            );
            assert!(
                EvidenceRecordHash::new(malformed.clone()).is_err(),
                "{malformed:?} must be rejected as an evidence record hash"
            );
        }
    }

    #[test]
    fn finite_score_rejects_non_finite_via_serde() {
        use serde::de::value::Error as ValueError;
        use serde::de::IntoDeserializer;

        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let deserializer: serde::de::value::F64Deserializer<ValueError> =
                bad.into_deserializer();
            assert!(
                FiniteScore::deserialize(deserializer).is_err(),
                "{bad} must be rejected by FiniteScore's Deserialize impl, not just its constructor"
            );
        }

        let deserializer: serde::de::value::F64Deserializer<ValueError> =
            0.5_f64.into_deserializer();
        assert!(FiniteScore::deserialize(deserializer).is_ok());
    }

    #[test]
    fn evidence_tie_break_is_canonical_not_insertion_order() {
        let board = board_with(vec![
            candidate("op.insert_after"),
            candidate("op.append_node"),
        ]);
        // Both candidates tie on final_score. Supply them in descending
        // canonical-id order so a stable-but-insertion-order-dependent
        // sort would leave "op.insert_after" first; the canonical rule
        // (descending score, ties by ASCENDING candidate id) must place
        // "op.append_node" first regardless of input order.
        // `InferenceEvidence::new` requires ranked to cover every board id
        // exactly once (including the framework abstention appended by
        // `board_with`), so score the two named candidates tied and give
        // abstention a losing score rather than omitting it.
        let tied = vec![
            CandidateEvidence {
                candidate_id: CanonicalCandidateId::new("op.insert_after").unwrap(),
                lane_scores: vec![LaneScore {
                    lane: EvidenceLane::Lexical,
                    score: FiniteScore::new(0.5).unwrap(),
                }],
                final_score: FiniteScore::new(0.5).unwrap(),
            },
            CandidateEvidence {
                candidate_id: CanonicalCandidateId::new("op.append_node").unwrap(),
                lane_scores: vec![LaneScore {
                    lane: EvidenceLane::Lexical,
                    score: FiniteScore::new(0.5).unwrap(),
                }],
                final_score: FiniteScore::new(0.5).unwrap(),
            },
            CandidateEvidence {
                candidate_id: CanonicalCandidateId::new(ABSTENTION_CANDIDATE_ID).unwrap(),
                lane_scores: vec![LaneScore {
                    lane: EvidenceLane::Lexical,
                    score: FiniteScore::new(0.0).unwrap(),
                }],
                final_score: FiniteScore::new(0.0).unwrap(),
            },
        ];
        let evidence =
            InferenceEvidence::new(&board, "turn", "serializer", "producer", tied).unwrap();
        assert_eq!(
            evidence.ranked[0].candidate_id.as_str(),
            "op.append_node",
            "tied scores must break by ascending candidate id, not input order"
        );
        assert_eq!(evidence.ranked[1].candidate_id.as_str(), "op.insert_after");

        // Determinism: rebuilding from the already-canonical order
        // reproduces the identical hash.
        let rebuilt = InferenceEvidence::new(
            &board,
            "turn",
            "serializer",
            "producer",
            evidence.ranked.clone(),
        )
        .unwrap();
        assert_eq!(evidence.evidence_record_hash, rebuilt.evidence_record_hash);
    }

    #[test]
    fn deserialize_rejects_duplicate_lanes_and_out_of_order_ranking() {
        let board = rich_board();
        let evidence = complete_evidence(&board);
        let mut encoded = serde_json::to_value(&evidence).unwrap();

        // Duplicate lane within one candidate's lane_scores, hand-adjusted
        // so the record hash still matches the tampered bytes -- proving
        // the reject comes from the structural check added to
        // `Deserialize`, not merely from the hash-equality check.
        let mut duplicate_lane_ranked = evidence.ranked.clone();
        let extra_lane = duplicate_lane_ranked[0].lane_scores[0].clone();
        duplicate_lane_ranked[0].lane_scores.push(extra_lane);
        let duplicate_hash = hash_evidence(
            &evidence.board_hash,
            &evidence.turn_context_hash,
            &evidence.candidate_serializer_hash,
            &evidence.producer_bundle_hash,
            &duplicate_lane_ranked,
        )
        .unwrap();
        encoded["ranked"] = serde_json::to_value(&duplicate_lane_ranked).unwrap();
        encoded["evidence_record_hash"] = serde_json::json!(duplicate_hash.as_str());
        assert!(
            serde_json::from_value::<InferenceEvidence>(encoded).is_err(),
            "a self-consistent hash over duplicate-lane evidence must still be refused"
        );

        // Out-of-order ranking (ascending score instead of descending),
        // again with a hash recomputed over that same wrong order so only
        // the order check -- not the hash check -- can catch it.
        let mut encoded = serde_json::to_value(&evidence).unwrap();
        let mut reversed_ranked = evidence.ranked.clone();
        reversed_ranked.reverse();
        let reversed_hash = hash_evidence(
            &evidence.board_hash,
            &evidence.turn_context_hash,
            &evidence.candidate_serializer_hash,
            &evidence.producer_bundle_hash,
            &reversed_ranked,
        )
        .unwrap();
        encoded["ranked"] = serde_json::to_value(&reversed_ranked).unwrap();
        encoded["evidence_record_hash"] = serde_json::json!(reversed_hash.as_str());
        assert!(
            serde_json::from_value::<InferenceEvidence>(encoded).is_err(),
            "a self-consistent hash over non-canonically-ordered ranking must still be refused"
        );
    }
}

//! Stable SemOS execution-mode value contract.
//!
//! Capability eligibility and mode attributes are application policy. They are
//! declared by semantic packs and evaluated by `sem_os_policy::pack_policy`;
//! this foundational crate intentionally contains no verb or role tables.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// Stable execution mode carried by principals, sessions and audit records.
///
/// Default and serialized spellings are retained for persistent compatibility.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AgentMode {
    Research,
    #[default]
    Governed,
    Maintenance,
}

impl AgentMode {
    /// Parse the stable mode spelling case-insensitively.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "research" => Some(Self::Research),
            "governed" => Some(Self::Governed),
            "maintenance" => Some(Self::Maintenance),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_parse_display_and_serde_remain_stable() {
        assert_eq!(AgentMode::default(), AgentMode::Governed);
        assert_eq!(AgentMode::parse("Research"), Some(AgentMode::Research));
        assert_eq!(AgentMode::parse("GOVERNED"), Some(AgentMode::Governed));
        assert_eq!(AgentMode::parse("invalid"), None);
        assert_eq!(AgentMode::Research.to_string(), "research");
        assert_eq!(
            serde_json::from_str::<AgentMode>("\"maintenance\"").unwrap(),
            AgentMode::Maintenance
        );
        assert_eq!(
            serde_json::to_string(&AgentMode::Governed).unwrap(),
            "\"governed\""
        );
    }
}

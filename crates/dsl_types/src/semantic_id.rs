use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

const MAX_ID_LEN: usize = 128;

/// Error returned when a pack-declared semantic identifier is malformed.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("invalid {kind} `{value}`: {reason}")]
pub struct SemanticIdError {
    kind: &'static str,
    value: String,
    reason: &'static str,
}

fn validate(kind: &'static str, value: &str) -> Result<(), SemanticIdError> {
    let invalid = |reason| SemanticIdError {
        kind,
        value: value.to_owned(),
        reason,
    };
    if value.is_empty() {
        return Err(invalid("must not be empty"));
    }
    if value.len() > MAX_ID_LEN {
        return Err(invalid("exceeds 128 bytes"));
    }
    if value.starts_with('.')
        || value.ends_with('.')
        || value.starts_with('-')
        || value.ends_with('-')
        || value.contains("..")
        || value.contains("--")
        || value.contains(".-")
        || value.contains("-.")
    {
        return Err(invalid("contains an empty or malformed segment"));
    }
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-' | b'_')
    }) {
        return Err(invalid(
            "must contain only lowercase ASCII letters, digits, '.', '-' or '_'",
        ));
    }
    Ok(())
}

macro_rules! semantic_id {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("Validated pack-declared ", $kind, ".")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Construct a validated ", $kind, ".")]
            pub fn new(value: impl Into<String>) -> Result<Self, SemanticIdError> {
                let value = value.into();
                validate($kind, &value)?;
                Ok(Self(value))
            }

            #[doc = concat!("Borrow the ", $kind, ".")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl FromStr for $name {
            type Err = SemanticIdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
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

semantic_id!(DomainTypeId, "domain type id");
semantic_id!(SlotKind, "slot kind");
semantic_id!(FocusKind, "focus kind");
semantic_id!(CapabilityId, "capability id");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_normalized_but_domain_neutral() {
        assert_eq!(SlotKind::new("graph.root").unwrap().as_str(), "graph.root");
        assert!(FocusKind::new("HostSpecific").is_err());
        assert!(CapabilityId::new("a..b").is_err());
    }
}

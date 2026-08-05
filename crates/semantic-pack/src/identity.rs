use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

const MAX_ID_LEN: usize = 128;

/// Error returned when a semantic pack identity is malformed.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("invalid {kind} `{value}`: {reason}")]
pub struct IdentityError {
    kind: &'static str,
    value: String,
    reason: &'static str,
}

fn validate_id(kind: &'static str, value: &str) -> Result<(), IdentityError> {
    let invalid = |reason| IdentityError {
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
        #[doc = concat!("Validated ", $kind, ".")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Construct a validated ", $kind, ".")]
            pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
                let value = value.into();
                validate_id($kind, &value)?;
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
            type Err = IdentityError;

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

semantic_id!(PackId, "pack id");
semantic_id!(DomainIdentity, "domain identity");
semantic_id!(AdapterBindingId, "adapter binding id");
semantic_id!(RoleId, "role id");
semantic_id!(GraphNodeId, "graph node id");
semantic_id!(IdentityNamespace, "identity namespace");

/// Validated pack version. Versions are intentionally opaque but stable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PackVersion(String);

impl PackVersion {
    /// Construct a version containing only stable ASCII version characters.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'+'))
        {
            return Err(IdentityError {
                kind: "pack version",
                value,
                reason: "must be 1-64 ASCII alphanumeric, '.', '-' or '+' characters",
            });
        }
        Ok(Self(value))
    }

    /// Borrow the exact version text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PackVersion {
    type Error = IdentityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PackVersion> for String {
    fn from(value: PackVersion) -> Self {
        value.0
    }
}

impl fmt::Display for PackVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable logical identity of one versioned pack.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackIdentity {
    /// Pack identifier.
    pub id: PackId,
    /// Exact source-declared version.
    pub version: PackVersion,
}

impl PackIdentity {
    /// Construct a logical pack identity.
    #[must_use]
    pub fn new(id: PackId, version: PackVersion) -> Self {
        Self { id, version }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_strict_and_round_trip() {
        let id = PackId::new("process.start_v2").unwrap();
        assert_eq!(serde_json::to_string(&id).unwrap(), "\"process.start_v2\"");
        assert_eq!(
            serde_json::from_str::<PackId>("\"process.start_v2\"").unwrap(),
            id
        );
        assert!(PackId::new("Process::Start").is_err());
        assert!(PackId::new("a..b").is_err());
    }
}

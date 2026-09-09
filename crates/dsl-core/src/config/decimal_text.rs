//! Serde helpers for fixed-point decimal thresholds.
//!
//! Thresholds are serialised as decimal text (`"1000.50"`) and deserialised
//! from decimal text or an integer literal. A floating-point literal is
//! rejected with a typed message: the value would already have been rounded
//! by the time it reached us, so the only exact spellings are an integer or a
//! quoted decimal string. No `f64` is ever constructed on this path.
//!
//! Use as `#[serde(with = "crate::config::decimal_text")]` on a `Decimal`
//! field, or `with = "crate::config::decimal_text::option"` on an
//! `Option<Decimal>` field.

use std::fmt;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};

struct DecimalTextVisitor;

impl Visitor<'_> for DecimalTextVisitor {
    type Value = Decimal;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an integer literal or a quoted decimal string")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Decimal, E> {
        Ok(Decimal::from(value))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Decimal, E> {
        Ok(Decimal::from(value))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Decimal, E> {
        Err(E::custom(format!(
            "floating-point threshold `{value}` rejected: write it as an integer or a quoted decimal string"
        )))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Decimal, E> {
        Decimal::from_str(value.trim())
            .map_err(|err| E::custom(format!("invalid decimal `{value}`: {err}")))
    }
}

/// Serialize a decimal as its exact text form.
pub(crate) fn serialize<S: Serializer>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.normalize().to_string())
}

/// Deserialize a decimal from an integer literal or decimal text.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Decimal, D::Error> {
    deserializer.deserialize_any(DecimalTextVisitor)
}

/// `Option<Decimal>` form of this module.
pub(crate) mod option {
    use rust_decimal::Decimal;
    use serde::de::{self, Visitor};
    use serde::{Deserializer, Serializer};
    use std::fmt;

    struct OptionVisitor;

    impl<'de> Visitor<'de> for OptionVisitor {
        type Value = Option<Decimal>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("null, an integer literal or a quoted decimal string")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            super::deserialize(deserializer).map(Some)
        }
    }

    /// Serialize an optional decimal as exact text or null.
    pub(crate) fn serialize<S: Serializer>(
        value: &Option<Decimal>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_some(&value.normalize().to_string()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserialize an optional decimal.
    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Decimal>, D::Error> {
        deserializer.deserialize_option(OptionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Threshold {
        #[serde(with = "super")]
        value: Decimal,
        #[serde(default, with = "super::option")]
        limit: Option<Decimal>,
    }

    #[test]
    fn yaml_integer_and_quoted_decimal_are_exact() {
        let t: Threshold = serde_yaml::from_str("value: 1000\nlimit: \"0.10\"\n").unwrap();
        assert_eq!(t.value, Decimal::from(1000));
        assert_eq!(t.limit, Some(Decimal::from_str("0.10").unwrap()));
    }

    #[test]
    fn yaml_float_literal_is_rejected() {
        let err = serde_yaml::from_str::<Threshold>("value: 0.1\n").unwrap_err();
        assert!(
            err.to_string().contains("floating-point threshold"),
            "{err}"
        );
    }

    #[test]
    fn json_round_trip_uses_text() {
        let t = Threshold {
            value: Decimal::from_str("1000.50").unwrap(),
            limit: None,
        };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, r#"{"value":"1000.5","limit":null}"#);
        let back: Threshold = serde_json::from_str(&json).unwrap();
        assert_eq!(back, t);
        let from_int: Threshold = serde_json::from_str(r#"{"value": 7}"#).unwrap();
        assert_eq!(from_int.value, Decimal::from(7));
    }
}

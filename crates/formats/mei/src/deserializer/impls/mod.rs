//! Deserialization trait implementations for MEI types.
//!
//! Attribute class impls (`ExtractAttributes for Att*`) are auto-generated in
//! `generated_att_impls.rs`. Element impls (`MeiDeserialize`) are auto-generated in
//! `generated_element_impls.rs`.

use super::{AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader};
use serde::Deserialize;
use std::io::BufRead;

/// Parse a value using serde_json from XML attribute string.
/// Tries multiple JSON formats to handle different serde derives:
/// - For numbers/booleans: parse as-is (e.g., "4" -> 4)
/// - For strings/enums: wrap in quotes (e.g., "c" -> "c")
pub(crate) fn from_attr_string<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, String> {
    // First try parsing as-is (for numbers, booleans)
    if let Ok(v) = serde_json::from_str(s) {
        return Ok(v);
    }
    // Then try as a quoted string (for strings, enums)
    let json = format!("\"{}\"", s);
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

/// Helper macro to extract an optional attribute using serde deserialization.
macro_rules! extract_attr {
    ($attrs:expr, $name:expr, $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            match $crate::deserializer::impls::from_attr_string(&value) {
                Ok(v) => $field = Some(v),
                Err(_) => {
                    // In lenient mode, we can skip invalid values
                    // For strict mode, we'd return an error
                }
            }
        }
    };
    // For String fields (no serde parsing needed)
    ($attrs:expr, $name:expr, string $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            $field = Some(value);
        }
    };
    // For Vec fields that need serde parsing
    ($attrs:expr, $name:expr, vec $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let mut items = Vec::new();
            for part in value.split_whitespace() {
                if let Ok(v) = $crate::deserializer::impls::from_attr_string(part) {
                    items.push(v);
                }
            }
            $field = items;
        }
    };
    // For Vec<String> fields (no serde parsing needed). Use std::string::String so this
    // works when macro is expanded in generated_element_impls where String is shadowed by the MEI element.
    ($attrs:expr, $name:expr, vec_string $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let items: Vec<std::string::String> =
                value.split_whitespace().map(|s| s.to_string()).collect();
            if !items.is_empty() {
                $field = items;
            }
        }
    };
    // For SpaceSeparated<T> fields — parses each whitespace-separated token via serde
    ($attrs:expr, $name:expr, space_separated $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let mut items = Vec::new();
            for part in value.split_whitespace() {
                if let Ok(v) = $crate::deserializer::impls::from_attr_string(part) {
                    items.push(v);
                }
            }
            $field = Some(tusk_model::generated::SpaceSeparated::new(items));
        }
    };
}
pub(crate) use extract_attr;

mod generated_att_impls;
mod generated_element_impls;

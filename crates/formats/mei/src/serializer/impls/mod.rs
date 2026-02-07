//! Serialization trait implementations for MEI types.
//!
//! Attribute class impls (`CollectAttributes for Att*`) are auto-generated in
//! `generated_att_impls.rs`. Element impls (`MeiSerialize`) are hand-written below.

use serde::Serialize;

mod analysis;
mod biblio;
mod chords;
mod cmn_core;
mod control;
mod defs;
mod drama;
mod editorial;
mod facsimile;
mod header;
mod mensural;
mod metadata_text;
mod midi;
mod misc;
mod neumes;
mod note;
mod structure;
mod symbols;
mod tablature;
mod text;
mod text_containers;

// Header module is now a directory with submodules

/// Serialize any serde-serializable value to a JSON string and strip quotes.
/// This is used for all MEI data types that have serde derives.
/// For floats that are whole numbers (e.g., 25.0), outputs them without decimal (e.g., "25").
pub(crate) fn to_attr_string<T: Serialize>(v: &T) -> Option<String> {
    serde_json::to_string(v).ok().map(|s| {
        let s = s.trim_matches('"').to_string();
        // If it looks like a float that's actually a whole number (e.g., "25.0"),
        // strip the ".0" suffix to match MEI conventions
        if let Some(stripped) = s.strip_suffix(".0") {
            // Make sure the stripped part is a valid integer
            if stripped.parse::<i64>().is_ok() {
                return stripped.to_string();
            }
        }
        s
    })
}

/// Serialize a Vec of serde-serializable values to space-separated string.
pub(crate) fn serialize_vec_serde<T: Serialize>(vec: &[T]) -> Option<String> {
    if vec.is_empty() {
        None
    } else {
        let parts: Vec<String> = vec.iter().filter_map(to_attr_string).collect();
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}

/// Helper macro to push attribute if value is Some and serializes successfully.
macro_rules! push_attr {
    ($attrs:expr, $name:expr, $opt_val:expr) => {
        if let Some(ref v) = $opt_val {
            if let Some(s) = $crate::serializer::impls::to_attr_string(v) {
                $attrs.push(($name, s));
            }
        }
    };
    // For String fields (no serde parsing needed)
    ($attrs:expr, $name:expr, string $opt_val:expr) => {
        if let Some(ref v) = $opt_val {
            $attrs.push(($name, v.clone()));
        }
    };
    // For Vec types
    ($attrs:expr, $name:expr, vec $vec_val:expr) => {
        if let Some(v) = $crate::serializer::impls::serialize_vec_serde(&$vec_val) {
            $attrs.push(($name, v));
        }
    };
}
pub(crate) use push_attr;

mod generated_att_impls;

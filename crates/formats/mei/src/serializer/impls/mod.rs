//! Manual implementations of serialization traits for MEI types.
//!
//! This module contains hand-written implementations for key attribute classes
//! and elements to demonstrate and test the serialization pattern.
//!
//! In the future, these implementations should be code-generated from the MEI ODD
//! specification to cover all types.

use serde::Serialize;

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
mod tablature;
mod text;
mod text_containers;

// Header module is now a directory with submodules

/// Serialize any serde-serializable value to a JSON string and strip quotes.
/// This is used for all MEI data types that have serde derives.
pub(crate) fn to_attr_string<T: Serialize>(v: &T) -> Option<String> {
    serde_json::to_string(v)
        .ok()
        .map(|s| s.trim_matches('"').to_string())
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
    // For String/clone types
    ($attrs:expr, $name:expr, clone $opt_val:expr) => {
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

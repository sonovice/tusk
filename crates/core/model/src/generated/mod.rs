//! Generated types from MEI ODD specification.
//!
//! This module contains Rust types that map 1:1 to MEI constructs.
//!
//! DO NOT EDIT - regenerate with: cargo run -p mei-codegen
pub mod att;
pub mod data;
pub mod elements;
pub mod model;
pub mod pattern_entities;
pub mod validation;

#[cfg(test)]
mod att_tests;
#[cfg(test)]
mod data_tests;
pub use data::*;
pub use elements::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
pub use validation::{Validate, ValidationContext, ValidationError, ValidationResult};
/// Wrapper for space-separated list values in MEI attributes.
///
/// MEI uses space-separated lists for some attributes (e.g., bezier coordinates).
/// This wrapper handles serialization/deserialization of such values.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceSeparated<T>(pub Vec<T>);
impl<T> SpaceSeparated<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self(items)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<T: fmt::Display> Serialize for SpaceSeparated<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: std::string::String = self
            .0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        serializer.serialize_str(&s)
    }
}
impl<'de, T> Deserialize<'de> for SpaceSeparated<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = std::string::String::deserialize(deserializer)?;
        let items: Result<Vec<T>, _> = s
            .split_whitespace()
            .map(|part| part.parse::<T>().map_err(serde::de::Error::custom))
            .collect();
        Ok(SpaceSeparated(items?))
    }
}

//! MusicXML 4.0 data types (simple types from the XSD schema).
//!
//! These types represent the fundamental data types used throughout MusicXML.
//! They map to xs:simpleType definitions in the MusicXML XSD.

mod formatting;
mod notation;
mod pitch;

pub use formatting::*;
pub use notation::*;
pub use pitch::*;

use std::fmt;

// ============================================================================
// Error Type
// ============================================================================

/// Error type for parsing MusicXML data types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The value string couldn't be parsed.
    InvalidValue(String),
    /// A numeric value couldn't be parsed.
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidValue(s) => write!(f, "invalid value: {}", s),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

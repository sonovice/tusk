//! MusicXML 4.0 element types (complex types from the XSD schema).
//!
//! This module contains struct definitions for MusicXML elements,
//! starting with the score-partwise document structure.

pub mod barline;
pub mod measure;
pub mod score;

pub use barline::*;
pub use measure::*;
pub use score::*;

//! MusicXML 4.0 data model.
//!
//! This module contains the intermediate MusicXML model types used for
//! parsing MusicXML files and converting to/from MEI.
//!
//! # Module Organization
//!
//! - `data` - Simple types (xs:simpleType definitions from XSD)
//!
//! Types follow MusicXML 4.0 specification naming conventions.

pub mod data;

// Re-export commonly used types
pub use data::*;

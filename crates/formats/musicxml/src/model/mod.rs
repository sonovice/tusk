//! MusicXML 4.0 data model.
//!
//! This module contains the intermediate MusicXML model types used for
//! parsing MusicXML files and converting to/from MEI.
//!
//! # Module Organization
//!
//! - `data` - Simple types (xs:simpleType definitions from XSD)
//! - `elements` - Complex types/element structs (xs:complexType definitions)
//! - `note` - Note, rest, pitch, and related types
//!
//! Types follow MusicXML 4.0 specification naming conventions.

pub mod data;
pub mod elements;
pub mod note;

// Re-export commonly used types
pub use data::*;
pub use elements::*;
pub use note::*;

//! MusicXML XSD → Rust code generator.
//!
//! Parses the MusicXML 4.1 XSD schema and generates Rust model types
//! for use by the tusk-musicxml crate.

mod ast;
mod generator;
mod xsd;

pub use ast::*;
pub use generator::generate;
pub use xsd::parse_xsd;

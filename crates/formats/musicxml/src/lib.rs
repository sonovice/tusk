//! MusicXML parsing and serialization for Tusk.
//!
//! This crate handles reading and writing MusicXML files.
//!
//! # Supported Versions
//!
//! - MusicXML 4.0 (primary target)
//! - MusicXML 3.1 (with upgrade to 4.0)
//! - MusicXML 3.0 (with upgrade to 4.0)
//! - MusicXML 2.0 (with upgrade to 4.0)
//!
//! # Document Types
//!
//! Both `score-partwise` and `score-timewise` formats are supported.
//! Timewise documents are converted to partwise internally.

pub mod versions;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        assert!(true);
    }
}

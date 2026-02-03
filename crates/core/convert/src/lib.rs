//! Conversion layer between MusicXML and MEI for Tusk.
//!
//! This crate provides bidirectional conversion:
//! - MusicXML → MEI (lossless)
//! - MEI → MusicXML (lossy, MEI-specific features documented)
//!
//! # Conversion Context
//!
//! The conversion process maintains state via `ConversionContext` to track:
//! - Division calculations
//! - Pending ties/slurs
//! - ID mappings between formats

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        assert!(true);
    }
}

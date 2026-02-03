//! MEI-based internal model for Tusk (MusicXML ↔ MEI converter).
//!
//! This crate contains the internal representation of music notation, modeled
//! directly after MEI (Music Encoding Initiative) elements and attribute classes.
//! The types map 1:1 to MEI constructs for lossless MEI round-trips.
//!
//! # Module Organization
//!
//! - `generated/` - Types generated from MEI ODD specification
//!   - `data` - Data types from `macroSpec type="dt"`
//!   - `att/` - Attribute classes from `classSpec type="atts"`
//!   - `elements/` - Elements from `elementSpec`

pub mod generated;

// Re-export commonly used types
pub use generated::*;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        // Basic smoke test to verify the crate structure is valid
        assert!(true);
    }
}

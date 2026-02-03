//! MEI parsing and serialization for Tusk.
//!
//! This crate handles reading and writing MEI (Music Encoding Initiative) files.
//!
//! # Supported Versions
//!
//! - MEI 5.1 (primary target)
//! - MEI 5.0 (with migration to 5.1)
//! - MEI 4.0.1 (with migration to 5.1)
//! - MEI 3.0.0 (with migration to 5.1)
//!
//! # Streaming Support
//!
//! For large files (100+ MB operas), use `MeiReader` for chunked processing
//! by `<mdiv>` elements to maintain constant memory usage.

pub mod versions;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        assert!(true);
    }
}

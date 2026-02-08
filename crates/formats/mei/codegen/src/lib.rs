// Allow clippy lints that are widespread in this crate (to be cleaned up separately)
#![allow(
    unused_imports,
    clippy::cmp_owned,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::type_complexity,
    clippy::manual_pattern_char_comparison,
    clippy::unnecessary_map_or,
    clippy::for_kv_map
)]

//! MEI ODD → Rust Code Generator library (tusk-mei format codegen).
//!
//! Parses MEI ODD or RNG specification files and generates Rust model types
//! for 1:1 MEI mapping. Can be used as a build-dependency for automated code
//! generation during `cargo build`.
//!
//! # Usage
//!
//! ```no_run
//! use std::path::Path;
//! use tusk_mei_codegen::{rng, generator};
//!
//! let defs = rng::parse_rng_file(Path::new("specs/mei/validation/mei-all.rng")).unwrap();
//! generator::generate_all(&defs, Path::new("crates/core/model/src/generated")).unwrap();
//! ```

pub mod ast;
pub mod generator;
pub mod parser;
pub mod rng;

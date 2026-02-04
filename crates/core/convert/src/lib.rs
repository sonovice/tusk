//! Conversion layer between MusicXML and MEI for Tusk.
//!
//! This crate provides bidirectional conversion:
//! - MusicXML → MEI (lossless)
//! - MEI → MusicXML (lossy, MEI-specific features documented)
//!
//! # Conversion Context
//!
//! The conversion process maintains state via [`ConversionContext`] to track:
//! - Division calculations (MusicXML divisions per quarter note)
//! - Pending ties/slurs that need to be resolved
//! - ID mappings between formats
//! - Key signature state for accidental determination
//! - Current position in the document (part, measure, staff, voice, layer)
//!
//! # Example
//!
//! ```
//! use tusk_convert::context::{ConversionContext, ConversionDirection};
//!
//! let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
//!
//! // Set divisions when parsing MusicXML attributes
//! ctx.set_divisions(4.0);
//!
//! // Map IDs between formats
//! ctx.map_id("P1", "staff-1");
//! assert_eq!(ctx.get_mei_id("P1"), Some("staff-1"));
//!
//! // Track pending ties
//! use tusk_convert::context::PendingTie;
//! ctx.add_pending_tie(PendingTie {
//!     start_id: "note-1".to_string(),
//!     staff: 1,
//!     voice: 1,
//!     step: 'C',
//!     octave: 4,
//!     alter: None,
//! });
//!
//! // Resolve tie when end note is found
//! let tie = ctx.resolve_tie(1, 1, 'C', 4);
//! assert!(tie.is_some());
//! ```
//!
//! # Errors
//!
//! The [`ConversionError`] type provides detailed error information:
//!
//! ```
//! use tusk_convert::error::{ConversionError, ConversionResult};
//!
//! fn example() -> ConversionResult<()> {
//!     // Return specific errors
//!     Err(ConversionError::missing_required("attribute", "dur", "note element"))
//! }
//! ```

pub mod context;
pub mod error;

pub use context::{ConversionContext, ConversionDirection, PendingSlur, PendingTie};
pub use error::{ConversionError, ConversionResult};

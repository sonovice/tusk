//! Barline types for MusicXML documents.
//!
//! This module contains barline and ending types.

use serde::{Deserialize, Serialize};

/// Barline placeholder.
/// Will be expanded in Phase 4.2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BarlinePlaceholder;

//! MusicXML 4.0 lyric types.
//!
//! Lyrics appear as children of `<note>` elements and contain syllable text,
//! elision markers, extend lines, and special types like humming/laughing.

use serde::{Deserialize, Serialize};

use super::data::*;

/// Syllabic type indicating the position of a syllable within a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Syllabic {
    /// Single-syllable word.
    Single,
    /// First syllable of a multi-syllable word.
    Begin,
    /// Middle syllable of a multi-syllable word.
    Middle,
    /// Last syllable of a multi-syllable word.
    End,
}

/// Extend type for melisma/extender lines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extend {
    /// Whether this is the start, stop, or continuation of an extend line.
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub extend_type: Option<StartStopContinue>,
}

/// Elision element connecting two syllables on one note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elision {
    /// The elision text (typically a Unicode connector character like ‿).
    pub value: String,

    /// Font family.
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size.
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font style.
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font weight.
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Text element within a lyric (the actual syllable text).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LyricText {
    /// The text content.
    pub value: String,

    /// Font family.
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size.
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font style.
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font weight.
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Content of a lyric element — one of the four XSD choice branches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LyricContent {
    /// Text-based lyric: optional syllabic + text, possibly repeated with elision,
    /// plus optional extend.
    Text {
        /// Sequence of (optional syllabic, text) pairs.
        /// First pair may omit syllabic; subsequent pairs are preceded by an elision.
        syllable_groups: Vec<SyllableGroup>,
        /// Optional extend (melisma) line.
        extend: Option<Extend>,
    },
    /// Extend-only lyric (continuation line without text).
    ExtendOnly(Extend),
    /// Laughing vocal indicator.
    Laughing,
    /// Humming vocal indicator.
    Humming,
}

/// A group of syllabic + text, optionally preceded by an elision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyllableGroup {
    /// Elision connector before this syllable (None for the first group).
    pub elision: Option<Elision>,
    /// Syllabic type (single/begin/middle/end).
    pub syllabic: Option<Syllabic>,
    /// The syllable text content.
    pub text: LyricText,
}

/// A complete lyric element on a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lyric {
    /// Verse number (e.g., "1", "2").
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Lyric name (e.g., "verse", "chorus").
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Justification (left/center/right).
    #[serde(rename = "@justify", skip_serializing_if = "Option::is_none")]
    pub justify: Option<LeftCenterRight>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position.
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position.
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Placement (above/below).
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Print-object (yes/no).
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Time-only restriction.
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The lyric content.
    pub content: LyricContent,

    /// End-line indicator (for karaoke).
    pub end_line: bool,

    /// End-paragraph indicator (for karaoke).
    pub end_paragraph: bool,
}

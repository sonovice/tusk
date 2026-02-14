//! Typed ornament/notation detail data for lossless MusicXML roundtrip.

use serde::{Deserialize, Serialize};

/// Ornament and notation detail data for lossless roundtrip of MusicXML
/// ornament, notation, and articulation types that have no direct MEI equivalent.
///
/// Each variant captures the data for one ornament/notation element that is
/// stored on an MEI `<ornam>`, `<arpeg>`, `<gliss>`, or `<dynam>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrnamentDetailData {
    /// `<vertical-turn>` — placement-only ornament.
    VerticalTurn,
    /// `<inverted-vertical-turn>` — placement-only ornament.
    InvertedVerticalTurn,
    /// `<shake>` — placement-only ornament.
    Shake,
    /// `<schleifer>` — placement-only ornament.
    Schleifer,
    /// `<haydn>` — placement-only ornament.
    Haydn,
    /// `<tremolo type="unmeasured">` — unmeasured tremolo (single/start/stop use pending context).
    UnmeasuredTremolo {
        /// Tremolo type: "single", "start", "stop", "unmeasured".
        #[serde(rename = "ty")]
        tremolo_type: String,
        /// Number of tremolo marks.
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<u8>,
    },
    /// `<wavy-line>` — ornamental wavy line spanning.
    WavyLine {
        /// Start/stop/continue type.
        #[serde(rename = "ty")]
        wavy_line_type: String,
        /// Wavy-line number for disambiguation.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<u8>,
    },
    /// `<other-ornament>` — free-text ornament.
    OtherOrnament {
        /// Text content.
        text: String,
    },
    /// `<accidental-mark>` within `<ornaments>` container.
    OrnamentAccidentalMark {
        /// Accidental value (e.g. "sharp", "flat", "natural").
        value: String,
        /// Placement above/below.
        #[serde(skip_serializing_if = "Option::is_none")]
        placement: Option<String>,
    },
    /// Standalone `<accidental-mark>` in `<notations>`.
    AccidentalMark {
        /// Accidental value.
        value: String,
        /// Placement above/below.
        #[serde(skip_serializing_if = "Option::is_none")]
        placement: Option<String>,
    },
    /// `<other-notation>` — free-text notation element.
    OtherNotation {
        /// Start/stop/single type.
        #[serde(rename = "ty")]
        notation_type: String,
        /// Notation number for disambiguation.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<u8>,
        /// SMuFL glyph name.
        #[serde(skip_serializing_if = "Option::is_none")]
        smufl: Option<String>,
        /// Text content.
        #[serde(default)]
        text: String,
    },
    /// `<non-arpeggiate>` — explicit non-arpeggiation marker (stored on `<arpeg>`).
    NonArpeggiate,
    /// `<slide>` — portamento/glissando variant (stored on `<gliss>`).
    Slide,
    /// `<dynamics>` within `<notations>` (not direction-level).
    NotationDynamics,
}

//! Typed technical detail data for lossless MusicXML roundtrip.

use serde::{Deserialize, Serialize};

/// Technical notation detail data for lossless roundtrip of MusicXML
/// `<notations><technical>` types that have no direct MEI equivalent.
///
/// Each variant captures the data for one technical element that is
/// stored on an MEI `<ornam>` or `<fing>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TechnicalDetailData {
    // ----- Simple placement-only types (stored on <ornam>) -----
    /// `<up-bow>` — V-shaped bowing mark (also mapped to @artic).
    UpBow,
    /// `<down-bow>` — reversed-V bowing mark (also mapped to @artic).
    DownBow,
    /// `<open-string>` — zero-shaped symbol.
    OpenString,
    /// `<thumb-position>` — thumb position symbol.
    ThumbPosition,
    /// `<double-tongue>` — wind technique.
    DoubleTongue,
    /// `<triple-tongue>` — wind technique.
    TripleTongue,
    /// `<snap-pizzicato>` — Bartók pizz (also mapped to @artic).
    SnapPizzicato,
    /// `<fingernails>` — harp/plucked technique.
    Fingernails,
    /// `<brass-bend>` — U-shaped symbol.
    BrassBend,
    /// `<flip>` — brass technique.
    Flip,
    /// `<smear>` — tilde symbol, brass.
    Smear,
    /// `<golpe>` — guitar pick guard tap.
    Golpe,

    // ----- Placement + SMuFL types -----
    /// `<stopped>` — plus sign (also mapped to @artic when no smufl).
    Stopped {
        #[serde(skip_serializing_if = "Option::is_none")]
        smufl: Option<String>,
    },
    /// `<open>` — circle symbol (distinct from open-string).
    Open {
        #[serde(skip_serializing_if = "Option::is_none")]
        smufl: Option<String>,
    },
    /// `<half-muted>` — circle with plus.
    HalfMuted {
        #[serde(skip_serializing_if = "Option::is_none")]
        smufl: Option<String>,
    },

    // ----- Text content types -----
    /// `<pluck>` — pluck fingering (p, i, m, a for guitar).
    Pluck {
        /// Pluck text value.
        #[serde(default)]
        value: String,
    },
    /// `<fret>` — fret number (0 = open).
    Fret {
        /// Fret number.
        value: u32,
    },
    /// `<string>` — string number (1 = highest).
    StringNum {
        /// String number.
        value: u32,
    },
    /// `<handbell>` — handbell technique text.
    Handbell {
        /// Technique value text.
        #[serde(default)]
        value: String,
    },

    // ----- Parameterized types -----
    /// `<hammer-on>` — fretted instrument technique.
    HammerOn {
        /// Start/stop type.
        #[serde(rename = "ty")]
        ho_type: String,
        /// Number for disambiguation.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<u8>,
        /// Display text (e.g. "H").
        #[serde(default, skip_serializing_if = "String::is_empty")]
        text: String,
    },
    /// `<pull-off>` — fretted instrument technique.
    PullOff {
        /// Start/stop type.
        #[serde(rename = "ty")]
        po_type: String,
        /// Number for disambiguation.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<u8>,
        /// Display text (e.g. "P").
        #[serde(default, skip_serializing_if = "String::is_empty")]
        text: String,
    },
    /// `<tap>` — fretboard tap.
    Tap {
        /// Which hand performs the tap.
        #[serde(skip_serializing_if = "Option::is_none")]
        hand: Option<String>,
        /// Display text (e.g. "+", "T").
        #[serde(default, skip_serializing_if = "String::is_empty")]
        value: String,
    },
    /// `<heel>` — organ pedal technique.
    Heel {
        /// Whether this is a substitution.
        #[serde(skip_serializing_if = "Option::is_none")]
        substitution: Option<bool>,
    },
    /// `<toe>` — organ pedal technique.
    Toe {
        /// Whether this is a substitution.
        #[serde(skip_serializing_if = "Option::is_none")]
        substitution: Option<bool>,
    },

    // ----- Complex types -----
    /// `<bend>` — guitar bend notation.
    Bend {
        /// Semitones to bend.
        alter: f64,
        /// Pre-bend before sounding.
        #[serde(skip_serializing_if = "Option::is_none")]
        pre_bend: Option<bool>,
        /// Release the bend (Some(None) = plain release, Some(Some(v)) = with offset).
        #[serde(skip_serializing_if = "Option::is_none")]
        release: Option<Option<f64>>,
        /// Bend shape.
        #[serde(skip_serializing_if = "Option::is_none")]
        shape: Option<String>,
        /// With-bar text (e.g. "dip", "scoop").
        #[serde(rename = "wb", skip_serializing_if = "Option::is_none")]
        with_bar: Option<String>,
    },
    /// `<hole>` — woodwind/brass fingering hole.
    Hole {
        /// Hole closed value: "yes", "no", "half".
        closed: String,
        /// Location of closed portion.
        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<String>,
        /// Descriptive hole type text.
        #[serde(rename = "ht", skip_serializing_if = "Option::is_none")]
        hole_type: Option<String>,
        /// Hole shape text.
        #[serde(rename = "hs", skip_serializing_if = "Option::is_none")]
        hole_shape: Option<String>,
    },
    /// `<arrow>` — directional or circular arrow.
    Arrow {
        /// Arrow content — either directional or circular.
        content: ArrowContentData,
    },
    /// `<harmon-mute>` — brass harmon mute.
    HarmonMute {
        /// Closed value: "yes", "no", "half".
        closed: String,
        /// Location of closed portion.
        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<String>,
    },
    /// `<harmonic>` — natural/artificial harmonic.
    Harmonic {
        /// Natural harmonic flag.
        #[serde(skip_serializing_if = "Option::is_none")]
        natural: Option<bool>,
        /// Artificial harmonic flag.
        #[serde(skip_serializing_if = "Option::is_none")]
        artificial: Option<bool>,
        /// Base pitch indicator.
        #[serde(rename = "bp", skip_serializing_if = "Option::is_none")]
        base_pitch: Option<bool>,
        /// Touching pitch indicator.
        #[serde(rename = "tp", skip_serializing_if = "Option::is_none")]
        touching_pitch: Option<bool>,
        /// Sounding pitch indicator.
        #[serde(rename = "sp", skip_serializing_if = "Option::is_none")]
        sounding_pitch: Option<bool>,
    },

    // ----- Extensibility -----
    /// `<other-technical>` — free-text technical notation.
    OtherTechnical {
        /// SMuFL glyph name.
        #[serde(skip_serializing_if = "Option::is_none")]
        smufl: Option<String>,
        /// Text content.
        #[serde(default, skip_serializing_if = "String::is_empty")]
        text: String,
    },

    // ----- Fingering (stored on <fing>) -----
    /// `<fingering>` — fingering with substitution/alternate attrs.
    Fingering {
        /// Whether this is a substitution fingering.
        #[serde(skip_serializing_if = "Option::is_none")]
        substitution: Option<bool>,
        /// Whether this is an alternate fingering.
        #[serde(skip_serializing_if = "Option::is_none")]
        alternate: Option<bool>,
    },

    // ----- Tech-artic (stored on note @artic via label) -----
    /// Technical types mapped to MEI @artic that need a roundtrip marker.
    TechArticulation {
        /// Articulation name: "upbow", "dnbow", "snap", "stop".
        name: String,
    },
}

/// Arrow content — directional or circular.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArrowContentData {
    /// Directional arrow with direction, optional style, and arrowhead flag.
    Directional {
        direction: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<String>,
        #[serde(default)]
        arrowhead: bool,
    },
    /// Circular arrow.
    Circular(String),
}

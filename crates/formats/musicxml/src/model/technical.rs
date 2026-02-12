//! MusicXML 4.0 technical notation types.
//!
//! Contains all 31 technical element types that can appear within a note's
//! `<notations><technical>` container. These include string/bow marks,
//! fretted instrument notations, keyboard pedaling, wind/brass techniques,
//! and general extensions.

use serde::{Deserialize, Serialize};

use super::data::{AboveBelow, StartStop, YesNo};
use super::notations::EmptyPlacement;

// ============================================================================
// Technical Container
// ============================================================================

/// Container for technical indication elements within `<notations>`.
///
/// Per the MusicXML schema, `<technical>` contains a choice of 0 or more
/// child elements in any order. We store each as a Vec to support multiple
/// instances (e.g. multiple fingerings on a chord note), except for types
/// where only one makes semantic sense.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Technical {
    /// Up-bow (V-shaped bowing mark).
    #[serde(rename = "up-bow", default, skip_serializing_if = "Vec::is_empty")]
    pub up_bow: Vec<EmptyPlacement>,

    /// Down-bow (reversed-V bowing mark).
    #[serde(rename = "down-bow", default, skip_serializing_if = "Vec::is_empty")]
    pub down_bow: Vec<EmptyPlacement>,

    /// Harmonic notation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub harmonic: Vec<Harmonic>,

    /// Open string (zero-shaped symbol).
    #[serde(rename = "open-string", default, skip_serializing_if = "Vec::is_empty")]
    pub open_string: Vec<EmptyPlacement>,

    /// Thumb position symbol.
    #[serde(
        rename = "thumb-position",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub thumb_position: Vec<EmptyPlacement>,

    /// Fingering (1-5 or custom text).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fingering: Vec<Fingering>,

    /// Pluck fingering (p, i, m, a for guitar).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pluck: Vec<PlacementText>,

    /// Double tongue symbol.
    #[serde(
        rename = "double-tongue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub double_tongue: Vec<EmptyPlacement>,

    /// Triple tongue symbol.
    #[serde(
        rename = "triple-tongue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub triple_tongue: Vec<EmptyPlacement>,

    /// Stopped symbol (plus sign).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stopped: Vec<EmptyPlacementSmufl>,

    /// Snap pizzicato symbol.
    #[serde(
        rename = "snap-pizzicato",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub snap_pizzicato: Vec<EmptyPlacement>,

    /// Fret number (0 = open, 1+ = fret).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fret: Vec<Fret>,

    /// String number (1 = highest).
    #[serde(rename = "string", default, skip_serializing_if = "Vec::is_empty")]
    pub string: Vec<TechString>,

    /// Hammer-on indication (fretted instruments).
    #[serde(rename = "hammer-on", default, skip_serializing_if = "Vec::is_empty")]
    pub hammer_on: Vec<HammerOnPullOff>,

    /// Pull-off indication (fretted instruments).
    #[serde(rename = "pull-off", default, skip_serializing_if = "Vec::is_empty")]
    pub pull_off: Vec<HammerOnPullOff>,

    /// Bend notation (guitar).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bend: Vec<Bend>,

    /// Tap on fretboard.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tap: Vec<Tap>,

    /// Heel (organ pedal technique).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub heel: Vec<HeelToe>,

    /// Toe (organ pedal technique).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub toe: Vec<HeelToe>,

    /// Fingernails (harp/plucked instruments).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fingernails: Vec<EmptyPlacement>,

    /// Hole (woodwind/brass fingering).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hole: Vec<Hole>,

    /// Arrow (technical indication direction).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arrow: Vec<Arrow>,

    /// Handbell technique.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handbell: Vec<Handbell>,

    /// Brass bend (U-shaped symbol).
    #[serde(rename = "brass-bend", default, skip_serializing_if = "Vec::is_empty")]
    pub brass_bend: Vec<EmptyPlacement>,

    /// Flip symbol (brass).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flip: Vec<EmptyPlacement>,

    /// Smear symbol (tilde, brass).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub smear: Vec<EmptyPlacement>,

    /// Open symbol (circle, distinct from open-string).
    #[serde(rename = "open", default, skip_serializing_if = "Vec::is_empty")]
    pub open: Vec<EmptyPlacementSmufl>,

    /// Half-muted symbol (circle with plus).
    #[serde(rename = "half-muted", default, skip_serializing_if = "Vec::is_empty")]
    pub half_muted: Vec<EmptyPlacementSmufl>,

    /// Harmon mute symbols (brass).
    #[serde(rename = "harmon-mute", default, skip_serializing_if = "Vec::is_empty")]
    pub harmon_mute: Vec<HarmonMute>,

    /// Golpe (guitar pick guard tap).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub golpe: Vec<EmptyPlacement>,

    /// Other technical indications (extensibility).
    #[serde(
        rename = "other-technical",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_technical: Vec<OtherTechnical>,
}

impl Technical {
    /// Returns true if the technical container has any content.
    pub fn is_empty(&self) -> bool {
        self.up_bow.is_empty()
            && self.down_bow.is_empty()
            && self.harmonic.is_empty()
            && self.open_string.is_empty()
            && self.thumb_position.is_empty()
            && self.fingering.is_empty()
            && self.pluck.is_empty()
            && self.double_tongue.is_empty()
            && self.triple_tongue.is_empty()
            && self.stopped.is_empty()
            && self.snap_pizzicato.is_empty()
            && self.fret.is_empty()
            && self.string.is_empty()
            && self.hammer_on.is_empty()
            && self.pull_off.is_empty()
            && self.bend.is_empty()
            && self.tap.is_empty()
            && self.heel.is_empty()
            && self.toe.is_empty()
            && self.fingernails.is_empty()
            && self.hole.is_empty()
            && self.arrow.is_empty()
            && self.handbell.is_empty()
            && self.brass_bend.is_empty()
            && self.flip.is_empty()
            && self.smear.is_empty()
            && self.open.is_empty()
            && self.half_muted.is_empty()
            && self.harmon_mute.is_empty()
            && self.golpe.is_empty()
            && self.other_technical.is_empty()
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Empty placement with optional SMuFL glyph name.
/// Used by: stopped, open, half-muted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyPlacementSmufl {
    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

/// Text content with placement (used by pluck).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementText {
    /// Text content.
    #[serde(rename = "$value", default)]
    pub value: String,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Font style (italic, normal).
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Fingering notation (text content with substitution/alternate attributes).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fingering {
    /// Fingering text (1-5 or custom).
    #[serde(rename = "$value", default)]
    pub value: String,

    /// Whether this is a substitution fingering.
    #[serde(rename = "@substitution", skip_serializing_if = "Option::is_none")]
    pub substitution: Option<YesNo>,

    /// Whether this is an alternate fingering.
    #[serde(rename = "@alternate", skip_serializing_if = "Option::is_none")]
    pub alternate: Option<YesNo>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Fret number for fretted instruments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fret {
    /// Fret number (0 = open string).
    #[serde(rename = "$value")]
    pub value: u32,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// String number for string instruments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechString {
    /// String number (1 = highest pitched).
    #[serde(rename = "$value")]
    pub value: u32,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Hammer-on or pull-off notation (shared type).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HammerOnPullOff {
    /// Start or stop.
    #[serde(rename = "@type")]
    pub ho_type: StartStop,

    /// Number (1-6) for distinguishing concurrent instances.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Text content (e.g. "H" for hammer-on, "P" for pull-off).
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

/// Bend notation for guitar/fretted instruments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bend {
    /// Amount to bend in semitones (required, e.g. 0.5, 1, 2).
    pub bend_alter: f64,

    /// Pre-bend (bend before sounding the note).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_bend: Option<bool>,

    /// Release (release the bend).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<BendRelease>,

    /// With-bar indication (e.g. "dip", "scoop").
    #[serde(rename = "with-bar", skip_serializing_if = "Option::is_none")]
    pub with_bar: Option<PlacementText>,

    /// Bend shape (straight or curved).
    #[serde(rename = "@shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<BendShape>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Bend release with optional offset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BendRelease {
    /// Offset in divisions where the release starts.
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

/// Shape of a bend (straight or curved).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BendShape {
    Straight,
    Curved,
}

/// Tap notation on fretboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Tap {
    /// Text content (e.g. "+", "T").
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub value: String,

    /// Which hand performs the tap.
    #[serde(rename = "@hand", skip_serializing_if = "Option::is_none")]
    pub hand: Option<TapHand>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Hand for tap notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TapHand {
    Left,
    Right,
}

/// Heel or toe notation (organ pedal technique).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HeelToe {
    /// Whether this is a substitution.
    #[serde(rename = "@substitution", skip_serializing_if = "Option::is_none")]
    pub substitution: Option<YesNo>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Hole notation for woodwind/brass instruments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hole {
    /// Descriptive text for hole type (e.g. "thumb").
    #[serde(rename = "hole-type", skip_serializing_if = "Option::is_none")]
    pub hole_type: Option<String>,

    /// Whether the hole is open, closed, or half-closed.
    #[serde(rename = "hole-closed")]
    pub hole_closed: HoleClosed,

    /// Shape of the hole (e.g. "circle").
    #[serde(rename = "hole-shape", skip_serializing_if = "Option::is_none")]
    pub hole_shape: Option<String>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Hole-closed element with value and optional location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoleClosed {
    /// Whether the hole is open, closed, or half.
    #[serde(rename = "$value")]
    pub value: HoleClosedValue,

    /// Location of the closed portion.
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<HoleClosedLocation>,
}

/// Hole closed values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HoleClosedValue {
    Yes,
    No,
    Half,
}

/// Location for hole closed indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HoleClosedLocation {
    Right,
    Bottom,
    Left,
    Top,
}

/// Arrow notation for technical indication.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arrow {
    /// Arrow content — either directional or circular.
    pub content: ArrowContent,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

/// Arrow content — either directional (arrow-direction + optional arrow-style + arrowhead)
/// or circular (circular-arrow).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArrowContent {
    /// Directional arrow.
    Directional {
        direction: String,
        style: Option<String>,
        arrowhead: bool,
    },
    /// Circular arrow.
    Circular(String),
}

/// Handbell technique notation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Handbell {
    /// Handbell technique value.
    #[serde(rename = "$value")]
    pub value: String,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Harmon mute notation (brass instruments).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmonMute {
    /// The harmon-closed child element.
    #[serde(rename = "harmon-closed")]
    pub harmon_closed: HarmonClosed,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Harmon-closed element with value and optional location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmonClosed {
    /// Open, closed, or half.
    #[serde(rename = "$value")]
    pub value: HarmonClosedValue,

    /// Location of the closed portion.
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<HarmonClosedLocation>,
}

/// Harmon closed values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonClosedValue {
    Yes,
    No,
    Half,
}

/// Location for harmon closed indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonClosedLocation {
    Right,
    Bottom,
    Left,
    Top,
}

/// Harmonic notation (natural/artificial with pitch information).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Harmonic {
    /// Natural harmonic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural: Option<bool>,

    /// Artificial harmonic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artificial: Option<bool>,

    /// Base pitch indicator.
    #[serde(rename = "base-pitch", skip_serializing_if = "Option::is_none")]
    pub base_pitch: Option<bool>,

    /// Touching pitch indicator.
    #[serde(rename = "touching-pitch", skip_serializing_if = "Option::is_none")]
    pub touching_pitch: Option<bool>,

    /// Sounding pitch indicator.
    #[serde(rename = "sounding-pitch", skip_serializing_if = "Option::is_none")]
    pub sounding_pitch: Option<bool>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Print-object (yes/no).
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Other technical notation (extensibility).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherTechnical {
    /// Text content.
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub value: String,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technical_default_empty() {
        let tech = Technical::default();
        assert!(tech.is_empty());
    }

    #[test]
    fn test_technical_with_up_bow() {
        let mut tech = Technical::default();
        tech.up_bow.push(EmptyPlacement {
            placement: Some(AboveBelow::Above),
            ..Default::default()
        });
        assert!(!tech.is_empty());
    }

    #[test]
    fn test_bend_shape() {
        let bend = Bend {
            bend_alter: 2.0,
            pre_bend: None,
            release: None,
            with_bar: None,
            shape: Some(BendShape::Curved),
            default_x: None,
            default_y: None,
            color: None,
        };
        assert_eq!(bend.shape, Some(BendShape::Curved));
    }
}

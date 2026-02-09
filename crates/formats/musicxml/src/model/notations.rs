//! MusicXML 4.0 notations element types.
//!
//! Contains slurs, tied elements, articulations, tuplets, ornaments, fermatas,
//! arpeggiate, glissando, slide, accidental marks, and other notation markings
//! that appear within a note's <notations> element.

use serde::{Deserialize, Serialize};

use super::data::{
    AboveBelow, LineShape, LineType, OverUnder, StartNote, StartStop, StartStopContinue,
    StartStopSingle, TopBottom, TremoloType, TrillStep, TwoNoteTurn, UpDown, UprightInverted,
};
use super::note::NoteTypeValue;

/// Container for notation elements on a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Notations {
    /// Slur elements (curved lines connecting notes).
    #[serde(rename = "slur", default, skip_serializing_if = "Vec::is_empty")]
    pub slurs: Vec<Slur>,

    /// Tied elements (graphic representation of ties).
    #[serde(rename = "tied", default, skip_serializing_if = "Vec::is_empty")]
    pub tied: Vec<Tied>,

    /// Tuplet display notations (start/stop).
    #[serde(rename = "tuplet", default, skip_serializing_if = "Vec::is_empty")]
    pub tuplets: Vec<Tuplet>,

    /// Articulation markings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articulations: Option<Articulations>,

    /// Ornament markings (trills, mordents, turns, tremolos, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ornaments: Option<Ornaments>,

    /// Technical markings (bowing, fingering, fret, string, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical: Option<super::technical::Technical>,

    /// Fermata markings (up to 2 per note).
    #[serde(rename = "fermata", default, skip_serializing_if = "Vec::is_empty")]
    pub fermatas: Vec<Fermata>,

    /// Arpeggiate notation on chord notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arpeggiate: Option<Arpeggiate>,

    /// Non-arpeggiate bracket on chord notes.
    #[serde(rename = "non-arpeggiate", skip_serializing_if = "Option::is_none")]
    pub non_arpeggiate: Option<NonArpeggiate>,

    /// Glissando notations (start/stop pairs).
    #[serde(rename = "glissando", default, skip_serializing_if = "Vec::is_empty")]
    pub glissandos: Vec<Glissando>,

    /// Slide notations (start/stop pairs).
    #[serde(rename = "slide", default, skip_serializing_if = "Vec::is_empty")]
    pub slides: Vec<Slide>,

    /// Standalone accidental marks (not within ornaments).
    #[serde(
        rename = "accidental-mark",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub accidental_marks: Vec<AccidentalMark>,

    /// Other notations not covered by specific types.
    #[serde(
        rename = "other-notation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_notations: Vec<OtherNotation>,
}

// ============================================================================
// Tuplet Types
// ============================================================================

/// How to display tuplet numbers or types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShowTuplet {
    /// Show only the actual value.
    Actual,
    /// Show both actual and normal values.
    Both,
    /// Show neither.
    None,
}

/// Tuplet notation element — describes how a tuplet is displayed.
///
/// While `time-modification` describes the sounding effect of a tuplet,
/// the tuplet notation element controls its visual display (bracket,
/// number, type). Multiple tuplet elements on the same note support
/// nested tuplets via the `number` attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tuplet {
    /// Start or stop.
    #[serde(rename = "@type")]
    pub tuplet_type: StartStop,

    /// Tuplet number (1-6) for distinguishing nested/overlapping tuplets.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Whether to show a bracket.
    #[serde(rename = "@bracket", skip_serializing_if = "Option::is_none")]
    pub bracket: Option<YesNo>,

    /// Whether to show the tuplet number (actual, both, or none).
    #[serde(rename = "@show-number", skip_serializing_if = "Option::is_none")]
    pub show_number: Option<ShowTuplet>,

    /// Whether to show the tuplet note type (actual, both, or none).
    #[serde(rename = "@show-type", skip_serializing_if = "Option::is_none")]
    pub show_type: Option<ShowTuplet>,

    /// Line shape (straight or curved) for the bracket.
    #[serde(rename = "@line-shape", skip_serializing_if = "Option::is_none")]
    pub line_shape: Option<LineShape>,

    /// Placement above or below the staff.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Optional full control over how the actual part is displayed.
    #[serde(rename = "tuplet-actual", skip_serializing_if = "Option::is_none")]
    pub tuplet_actual: Option<TupletPortion>,

    /// Optional full control over how the normal part is displayed.
    #[serde(rename = "tuplet-normal", skip_serializing_if = "Option::is_none")]
    pub tuplet_normal: Option<TupletPortion>,
}

impl Tuplet {
    /// Create a tuplet start notation.
    pub fn start() -> Self {
        Self {
            tuplet_type: StartStop::Start,
            number: None,
            bracket: None,
            show_number: None,
            show_type: None,
            line_shape: None,
            placement: None,
            tuplet_actual: None,
            tuplet_normal: None,
        }
    }

    /// Create a tuplet stop notation.
    pub fn stop() -> Self {
        Self {
            tuplet_type: StartStop::Stop,
            number: None,
            bracket: None,
            show_number: None,
            show_type: None,
            line_shape: None,
            placement: None,
            tuplet_actual: None,
            tuplet_normal: None,
        }
    }
}

/// A portion of a tuplet display (actual or normal), providing optional
/// full control over number, note type, and dots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupletPortion {
    /// The displayed number for this portion.
    #[serde(rename = "tuplet-number", skip_serializing_if = "Option::is_none")]
    pub tuplet_number: Option<TupletNumber>,

    /// The displayed note type for this portion.
    #[serde(rename = "tuplet-type", skip_serializing_if = "Option::is_none")]
    pub tuplet_type: Option<TupletType>,

    /// Dots on the displayed note type.
    #[serde(rename = "tuplet-dot", default, skip_serializing_if = "Vec::is_empty")]
    pub tuplet_dots: Vec<TupletDot>,
}

/// The number displayed for a tuplet portion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupletNumber {
    /// The number value.
    #[serde(rename = "$value")]
    pub value: u32,
}

/// The note type displayed for a tuplet portion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupletType {
    /// The note type value (quarter, eighth, etc.).
    #[serde(rename = "$value")]
    pub value: NoteTypeValue,
}

/// A dot on a tuplet portion's note type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TupletDot;

// Re-export YesNo here for convenience since it's used in Tuplet.
use super::data::YesNo;

/// Slur notation element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slur {
    /// Start, stop, or continue.
    #[serde(rename = "@type")]
    pub slur_type: StartStopContinue,

    /// Slur number (1-6) for distinguishing concurrent slurs.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Placement above or below the staff.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Orientation (over/under) for the curve.
    #[serde(rename = "@orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<OverUnder>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Bezier X offset.
    #[serde(rename = "@bezier-x", skip_serializing_if = "Option::is_none")]
    pub bezier_x: Option<f64>,

    /// Bezier Y offset.
    #[serde(rename = "@bezier-y", skip_serializing_if = "Option::is_none")]
    pub bezier_y: Option<f64>,

    /// Bezier X2 offset.
    #[serde(rename = "@bezier-x2", skip_serializing_if = "Option::is_none")]
    pub bezier_x2: Option<f64>,

    /// Bezier Y2 offset.
    #[serde(rename = "@bezier-y2", skip_serializing_if = "Option::is_none")]
    pub bezier_y2: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Slur {
    /// Create a new slur with the given type.
    pub fn new(slur_type: StartStopContinue) -> Self {
        Self {
            slur_type,
            number: None,
            placement: None,
            orientation: None,
            default_x: None,
            default_y: None,
            bezier_x: None,
            bezier_y: None,
            bezier_x2: None,
            bezier_y2: None,
            color: None,
            id: None,
        }
    }
}

/// Tied type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TiedType {
    Start,
    Stop,
    Continue,
    #[serde(rename = "let-ring")]
    LetRing,
}

/// Tied notation element (graphic representation of ties).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tied {
    /// Start, stop, continue, or let-ring.
    #[serde(rename = "@type")]
    pub tied_type: TiedType,

    /// Tie number (1-6) for distinguishing concurrent ties.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Orientation (over/under) for the curve.
    #[serde(rename = "@orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<OverUnder>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Bezier X offset.
    #[serde(rename = "@bezier-x", skip_serializing_if = "Option::is_none")]
    pub bezier_x: Option<f64>,

    /// Bezier Y offset.
    #[serde(rename = "@bezier-y", skip_serializing_if = "Option::is_none")]
    pub bezier_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Tied {
    /// Create a new tied with the given type.
    pub fn new(tied_type: TiedType) -> Self {
        Self {
            tied_type,
            number: None,
            orientation: None,
            default_x: None,
            default_y: None,
            bezier_x: None,
            bezier_y: None,
            color: None,
            id: None,
        }
    }
}

/// Empty placement type for articulation marks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyPlacement {
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

/// Strong accent with type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StrongAccent {
    /// Type (up/down).
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub accent_type: Option<UpDown>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,
}

/// Breath mark values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BreathMarkValue {
    /// No breath mark value (empty element).
    #[serde(rename = "")]
    Empty,
    Comma,
    Tick,
    Upbow,
    Salzedo,
}

impl Default for BreathMarkValue {
    fn default() -> Self {
        Self::Empty
    }
}

/// Breath mark element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BreathMark {
    /// Breath mark type.
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<BreathMarkValue>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,
}

/// Caesura values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaesuraValue {
    /// Normal caesura.
    #[serde(rename = "")]
    Normal,
    Short,
    Thick,
    Curved,
}

impl Default for CaesuraValue {
    fn default() -> Self {
        Self::Normal
    }
}

/// Caesura element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Caesura {
    /// Caesura type.
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CaesuraValue>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,
}

/// Articulations container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Articulations {
    /// Accent (>).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<EmptyPlacement>,

    /// Strong accent (marcato, ^).
    #[serde(rename = "strong-accent", skip_serializing_if = "Option::is_none")]
    pub strong_accent: Option<StrongAccent>,

    /// Staccato (.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staccato: Option<EmptyPlacement>,

    /// Tenuto (-).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenuto: Option<EmptyPlacement>,

    /// Detached legato (tenuto + staccato).
    #[serde(rename = "detached-legato", skip_serializing_if = "Option::is_none")]
    pub detached_legato: Option<EmptyPlacement>,

    /// Staccatissimo (wedge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staccatissimo: Option<EmptyPlacement>,

    /// Spiccato.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spiccato: Option<EmptyPlacement>,

    /// Scoop (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoop: Option<EmptyPlacement>,

    /// Plop (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plop: Option<EmptyPlacement>,

    /// Doit (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doit: Option<EmptyPlacement>,

    /// Falloff (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falloff: Option<EmptyPlacement>,

    /// Breath mark.
    #[serde(rename = "breath-mark", skip_serializing_if = "Option::is_none")]
    pub breath_mark: Option<BreathMark>,

    /// Caesura.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caesura: Option<Caesura>,

    /// Stress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress: Option<EmptyPlacement>,

    /// Unstress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstress: Option<EmptyPlacement>,

    /// Soft accent.
    #[serde(rename = "soft-accent", skip_serializing_if = "Option::is_none")]
    pub soft_accent: Option<EmptyPlacement>,
}

// ============================================================================
// Ornament Types
// ============================================================================

/// Trill-sound attribute group — shared by trill-mark, turns, mordent, shake, haydn.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TrillSound {
    /// Starting note (upper, main, below).
    #[serde(rename = "@start-note", skip_serializing_if = "Option::is_none")]
    pub start_note: Option<StartNote>,

    /// Trill step (whole, half, unison).
    #[serde(rename = "@trill-step", skip_serializing_if = "Option::is_none")]
    pub trill_step: Option<TrillStep>,

    /// Two-note turn (whole, half, none).
    #[serde(rename = "@two-note-turn", skip_serializing_if = "Option::is_none")]
    pub two_note_turn: Option<TwoNoteTurn>,

    /// Whether the trill accelerates.
    #[serde(rename = "@accelerate", skip_serializing_if = "Option::is_none")]
    pub accelerate: Option<YesNo>,

    /// Number of beats (minimum 2).
    #[serde(rename = "@beats", skip_serializing_if = "Option::is_none")]
    pub beats: Option<f64>,

    /// Percentage of the way through the trill where the second beat falls.
    #[serde(rename = "@second-beat", skip_serializing_if = "Option::is_none")]
    pub second_beat: Option<f64>,

    /// Percentage of the way through the trill where the last beat falls.
    #[serde(rename = "@last-beat", skip_serializing_if = "Option::is_none")]
    pub last_beat: Option<f64>,
}

/// Empty trill-sound type (used by trill-mark, vertical-turn, inverted-vertical-turn,
/// shake, haydn).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyTrillSound {
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

    /// Trill-sound attributes.
    #[serde(flatten)]
    pub trill_sound: TrillSound,
}

/// Horizontal turn type (used by turn, delayed-turn, inverted-turn,
/// delayed-inverted-turn). Extends empty-trill-sound with a slash attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HorizontalTurn {
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

    /// Whether to show a slash through the turn.
    #[serde(rename = "@slash", skip_serializing_if = "Option::is_none")]
    pub slash: Option<YesNo>,

    /// Trill-sound attributes.
    #[serde(flatten)]
    pub trill_sound: TrillSound,
}

/// Mordent type. Extends empty-trill-sound with long, approach, departure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Mordent {
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

    /// Long mordent (shows more than one wave).
    #[serde(rename = "@long", skip_serializing_if = "Option::is_none")]
    pub long: Option<YesNo>,

    /// Approach direction (above/below).
    #[serde(rename = "@approach", skip_serializing_if = "Option::is_none")]
    pub approach: Option<AboveBelow>,

    /// Departure direction (above/below).
    #[serde(rename = "@departure", skip_serializing_if = "Option::is_none")]
    pub departure: Option<AboveBelow>,

    /// Trill-sound attributes.
    #[serde(flatten)]
    pub trill_sound: TrillSound,
}

/// Tremolo ornament element (single/start/stop/unmeasured, value 0-8 for measured).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tremolo {
    /// Tremolo type (single, start, stop, unmeasured).
    #[serde(rename = "@type")]
    pub tremolo_type: TremoloType,

    /// Number of tremolo marks (0-8). For measured tremolos.
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u8>,

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

/// Wavy-line element for trill/vibrato lines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WavyLine {
    /// Start, stop, or continue.
    #[serde(rename = "@type")]
    pub wavy_line_type: StartStopContinue,

    /// Number (1-6) for distinguishing concurrent wavy lines.
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

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,

    /// Trill-sound attributes.
    #[serde(flatten)]
    pub trill_sound: TrillSound,
}

/// Accidental mark within ornaments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccidentalMark {
    /// The accidental value.
    #[serde(rename = "$value")]
    pub value: String,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,
}

/// Other ornament with text content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherOrnament {
    /// Text content.
    #[serde(rename = "$value", default)]
    pub value: String,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,
}

/// Container for all ornament types within <ornaments>.
///
/// Per the MusicXML schema, <ornaments> contains a sequence of ornament choices,
/// each optionally followed by accidental-marks. We store each ornament type
/// as an Option since at most one of each type typically appears, except
/// accidental-marks which can appear multiple times.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Ornaments {
    /// Trill mark.
    #[serde(rename = "trill-mark", skip_serializing_if = "Option::is_none")]
    pub trill_mark: Option<EmptyTrillSound>,

    /// Turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<HorizontalTurn>,

    /// Delayed turn.
    #[serde(rename = "delayed-turn", skip_serializing_if = "Option::is_none")]
    pub delayed_turn: Option<HorizontalTurn>,

    /// Inverted turn.
    #[serde(rename = "inverted-turn", skip_serializing_if = "Option::is_none")]
    pub inverted_turn: Option<HorizontalTurn>,

    /// Delayed inverted turn.
    #[serde(
        rename = "delayed-inverted-turn",
        skip_serializing_if = "Option::is_none"
    )]
    pub delayed_inverted_turn: Option<HorizontalTurn>,

    /// Vertical turn.
    #[serde(rename = "vertical-turn", skip_serializing_if = "Option::is_none")]
    pub vertical_turn: Option<EmptyTrillSound>,

    /// Inverted vertical turn.
    #[serde(
        rename = "inverted-vertical-turn",
        skip_serializing_if = "Option::is_none"
    )]
    pub inverted_vertical_turn: Option<EmptyTrillSound>,

    /// Shake.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shake: Option<EmptyTrillSound>,

    /// Wavy line (trill extension or vibrato).
    #[serde(rename = "wavy-line", skip_serializing_if = "Option::is_none")]
    pub wavy_line: Option<WavyLine>,

    /// Mordent (with vertical line).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mordent: Option<Mordent>,

    /// Inverted mordent (without vertical line).
    #[serde(rename = "inverted-mordent", skip_serializing_if = "Option::is_none")]
    pub inverted_mordent: Option<Mordent>,

    /// Schleifer (German ornament).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schleifer: Option<EmptyPlacement>,

    /// Tremolo (single, double, or unmeasured).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tremolo: Option<Tremolo>,

    /// Haydn ornament.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haydn: Option<EmptyTrillSound>,

    /// Other ornament.
    #[serde(rename = "other-ornament", skip_serializing_if = "Option::is_none")]
    pub other_ornament: Option<OtherOrnament>,

    /// Accidental marks within ornaments.
    #[serde(
        rename = "accidental-mark",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub accidental_marks: Vec<AccidentalMark>,
}

// ============================================================================
// Fermata, Arpeggiate, Glissando, Slide, AccidentalMark, OtherNotation
// ============================================================================

/// Fermata shape values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FermataShape {
    /// Normal (default when empty).
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "angled")]
    Angled,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "double-angled")]
    DoubleAngled,
    #[serde(rename = "double-square")]
    DoubleSquare,
    #[serde(rename = "double-dot")]
    DoubleDot,
    #[serde(rename = "half-curve")]
    HalfCurve,
    #[serde(rename = "curlew")]
    Curlew,
    /// Empty value (equivalent to normal).
    #[serde(rename = "")]
    Empty,
}

impl Default for FermataShape {
    fn default() -> Self {
        Self::Empty
    }
}

/// Fermata notation element.
///
/// The text content represents the shape. The type attribute indicates
/// upright or inverted. Up to 2 fermatas can appear on a single note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Fermata {
    /// Fermata shape (text content).
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub shape: Option<FermataShape>,

    /// Upright or inverted.
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub fermata_type: Option<UprightInverted>,

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

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Arpeggiate notation on chord notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Arpeggiate {
    /// Number (1-16) for distinguishing simultaneous arpeggios.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Direction (up/down) for the arpeggio arrow.
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<UpDown>,

    /// Whether the arpeggio continues onto another staff.
    #[serde(rename = "@unbroken", skip_serializing_if = "Option::is_none")]
    pub unbroken: Option<YesNo>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Non-arpeggiate bracket notation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonArpeggiate {
    /// Top or bottom of the bracket (required).
    #[serde(rename = "@type")]
    pub non_arpeggiate_type: TopBottom,

    /// Number (1-16) for distinguishing simultaneous non-arpeggiate brackets.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Glissando notation (wavy line between pitches).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Glissando {
    /// Start or stop (required).
    #[serde(rename = "@type")]
    pub glissando_type: StartStop,

    /// Number (1-16) for distinguishing concurrent glissandos.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Line type (solid, dashed, dotted, wavy).
    #[serde(rename = "@line-type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<LineType>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Text content printed alongside the line (e.g. "gliss.").
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

/// Slide notation (solid line between pitches, portamento).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slide {
    /// Start or stop (required).
    #[serde(rename = "@type")]
    pub slide_type: StartStop,

    /// Number (1-16) for distinguishing concurrent slides.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Line type (solid, dashed, dotted, wavy).
    #[serde(rename = "@line-type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<LineType>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Text content printed alongside the line.
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

/// Other notation type (extensibility).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherNotation {
    /// Start, stop, or single (required).
    #[serde(rename = "@type")]
    pub notation_type: StartStopSingle,

    /// Number (1-16).
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,

    /// Text content.
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slur_creation() {
        let slur = Slur::new(StartStopContinue::Start);
        assert_eq!(slur.slur_type, StartStopContinue::Start);
        assert_eq!(slur.number, None);
    }

    #[test]
    fn test_tied_creation() {
        let tied = Tied::new(TiedType::Start);
        assert_eq!(tied.tied_type, TiedType::Start);
    }

    #[test]
    fn test_articulations_default() {
        let artics = Articulations::default();
        assert!(artics.accent.is_none());
        assert!(artics.staccato.is_none());
    }

    #[test]
    fn test_tuplet_start() {
        let t = Tuplet::start();
        assert_eq!(t.tuplet_type, StartStop::Start);
        assert!(t.number.is_none());
        assert!(t.bracket.is_none());
        assert!(t.tuplet_actual.is_none());
    }

    #[test]
    fn test_tuplet_stop() {
        let t = Tuplet::stop();
        assert_eq!(t.tuplet_type, StartStop::Stop);
    }

    #[test]
    fn test_tuplet_with_portion() {
        let mut t = Tuplet::start();
        t.number = Some(1);
        t.bracket = Some(YesNo::Yes);
        t.show_number = Some(ShowTuplet::Both);
        t.tuplet_actual = Some(TupletPortion {
            tuplet_number: Some(TupletNumber { value: 3 }),
            tuplet_type: Some(TupletType {
                value: NoteTypeValue::Eighth,
            }),
            tuplet_dots: Vec::new(),
        });
        assert_eq!(
            t.tuplet_actual
                .as_ref()
                .unwrap()
                .tuplet_number
                .as_ref()
                .unwrap()
                .value,
            3
        );
    }

    #[test]
    fn test_notations_with_tuplets() {
        let mut n = Notations::default();
        n.tuplets.push(Tuplet::start());
        assert_eq!(n.tuplets.len(), 1);
    }
}

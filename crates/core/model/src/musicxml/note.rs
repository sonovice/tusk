//! MusicXML 4.0 note, rest, and related element types.
//!
//! This module contains types for representing musical notes, rests,
//! chords, pitch, duration, and related elements.

use serde::{Deserialize, Serialize};

use super::data::*;
use super::elements::Empty;

// ============================================================================
// Pitch and Rest Types
// ============================================================================

/// Pitch information for a note.
///
/// Contains the step (A-G), optional chromatic alteration, and octave.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pitch {
    /// The step in the diatonic scale (A-G).
    pub step: Step,

    /// Chromatic alteration in semitones (e.g., -1 for flat, 1 for sharp).
    /// Supports microtones with fractional values (e.g., 0.5 for quarter-tone sharp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alter: Option<f64>,

    /// Octave number (0-9). Octave 4 starts with middle C.
    pub octave: u8,
}

impl Pitch {
    /// Create a new pitch with the given step and octave.
    pub fn new(step: Step, octave: u8) -> Self {
        Self {
            step,
            alter: None,
            octave,
        }
    }

    /// Create a new pitch with chromatic alteration.
    pub fn with_alter(step: Step, alter: f64, octave: u8) -> Self {
        Self {
            step,
            alter: Some(alter),
            octave,
        }
    }
}

/// Unpitched note information (for percussion instruments).
///
/// Contains optional display step and octave for staff positioning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Unpitched {
    /// Display step for staff positioning.
    #[serde(rename = "display-step", skip_serializing_if = "Option::is_none")]
    pub display_step: Option<Step>,

    /// Display octave for staff positioning.
    #[serde(rename = "display-octave", skip_serializing_if = "Option::is_none")]
    pub display_octave: Option<u8>,
}

/// Rest information.
///
/// Rests can optionally specify display step and octave for staff positioning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Rest {
    /// Display step for staff positioning.
    #[serde(rename = "display-step", skip_serializing_if = "Option::is_none")]
    pub display_step: Option<Step>,

    /// Display octave for staff positioning.
    #[serde(rename = "display-octave", skip_serializing_if = "Option::is_none")]
    pub display_octave: Option<u8>,

    /// Indicates a whole-measure rest.
    #[serde(rename = "@measure", skip_serializing_if = "Option::is_none")]
    pub measure: Option<YesNo>,
}

impl Rest {
    /// Create a new simple rest.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a whole-measure rest.
    pub fn measure_rest() -> Self {
        Self {
            measure: Some(YesNo::Yes),
            ..Default::default()
        }
    }

    /// Create a rest with display position.
    pub fn with_display(step: Step, octave: u8) -> Self {
        Self {
            display_step: Some(step),
            display_octave: Some(octave),
            measure: None,
        }
    }
}

// ============================================================================
// Note Content Types
// ============================================================================

/// The pitch/rest/unpitched content of a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FullNoteContent {
    /// A pitched note.
    Pitch(Pitch),
    /// An unpitched note (percussion).
    Unpitched(Unpitched),
    /// A rest.
    Rest(Rest),
}

/// Grace note attributes.
///
/// Grace notes "steal" time from adjacent notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Grace {
    /// Percentage of time stolen from the previous note.
    #[serde(
        rename = "@steal-time-previous",
        skip_serializing_if = "Option::is_none"
    )]
    pub steal_time_previous: Option<f64>,

    /// Percentage of time stolen from the following note.
    #[serde(
        rename = "@steal-time-following",
        skip_serializing_if = "Option::is_none"
    )]
    pub steal_time_following: Option<f64>,

    /// The time to add to the duration of grace notes (in divisions).
    #[serde(rename = "@make-time", skip_serializing_if = "Option::is_none")]
    pub make_time: Option<f64>,

    /// Whether the grace note has a slash through the stem.
    #[serde(rename = "@slash", skip_serializing_if = "Option::is_none")]
    pub slash: Option<YesNo>,
}

/// Tie information (start/stop).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tie {
    /// Whether this is the start or stop of a tie.
    #[serde(rename = "@type")]
    pub tie_type: StartStop,

    /// Time-only restriction for the tie.
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,
}

impl Tie {
    /// Create a tie start.
    pub fn start() -> Self {
        Self {
            tie_type: StartStop::Start,
            time_only: None,
        }
    }

    /// Create a tie stop.
    pub fn stop() -> Self {
        Self {
            tie_type: StartStop::Stop,
            time_only: None,
        }
    }
}

// ============================================================================
// Note Type (Duration Appearance)
// ============================================================================

/// The graphical note type (duration appearance).
///
/// This represents how the note looks, not its actual duration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteType {
    /// The note type value.
    #[serde(rename = "$value")]
    pub value: NoteTypeValue,

    /// Symbol size (cue, grace-cue, large, full).
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<SymbolSize>,
}

impl NoteType {
    /// Create a new note type.
    pub fn new(value: NoteTypeValue) -> Self {
        Self { value, size: None }
    }
}

/// Note type values (graphical duration).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoteTypeValue {
    /// 1024th note.
    #[serde(rename = "1024th")]
    N1024th,
    /// 512th note.
    #[serde(rename = "512th")]
    N512th,
    /// 256th note.
    #[serde(rename = "256th")]
    N256th,
    /// 128th note.
    #[serde(rename = "128th")]
    N128th,
    /// 64th note.
    #[serde(rename = "64th")]
    N64th,
    /// 32nd note.
    #[serde(rename = "32nd")]
    N32nd,
    /// 16th note.
    #[serde(rename = "16th")]
    N16th,
    /// Eighth note.
    #[serde(rename = "eighth")]
    Eighth,
    /// Quarter note.
    #[serde(rename = "quarter")]
    Quarter,
    /// Half note.
    #[serde(rename = "half")]
    Half,
    /// Whole note.
    #[serde(rename = "whole")]
    Whole,
    /// Breve (double whole note).
    #[serde(rename = "breve")]
    Breve,
    /// Long (quadruple whole note).
    #[serde(rename = "long")]
    Long,
    /// Maxima.
    #[serde(rename = "maxima")]
    Maxima,
}

impl std::fmt::Display for NoteTypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteTypeValue::N1024th => write!(f, "1024th"),
            NoteTypeValue::N512th => write!(f, "512th"),
            NoteTypeValue::N256th => write!(f, "256th"),
            NoteTypeValue::N128th => write!(f, "128th"),
            NoteTypeValue::N64th => write!(f, "64th"),
            NoteTypeValue::N32nd => write!(f, "32nd"),
            NoteTypeValue::N16th => write!(f, "16th"),
            NoteTypeValue::Eighth => write!(f, "eighth"),
            NoteTypeValue::Quarter => write!(f, "quarter"),
            NoteTypeValue::Half => write!(f, "half"),
            NoteTypeValue::Whole => write!(f, "whole"),
            NoteTypeValue::Breve => write!(f, "breve"),
            NoteTypeValue::Long => write!(f, "long"),
            NoteTypeValue::Maxima => write!(f, "maxima"),
        }
    }
}

// ============================================================================
// Accidental
// ============================================================================

/// Accidental value types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccidentalValue {
    Sharp,
    Natural,
    Flat,
    DoubleSharp,
    SharpSharp,
    FlatFlat,
    NaturalSharp,
    NaturalFlat,
    QuarterFlat,
    QuarterSharp,
    ThreeQuartersFlat,
    ThreeQuartersSharp,
    SharpDown,
    SharpUp,
    NaturalDown,
    NaturalUp,
    FlatDown,
    FlatUp,
    DoubleSharpDown,
    DoubleSharpUp,
    FlatFlatDown,
    FlatFlatUp,
    ArrowDown,
    ArrowUp,
    TripleSharp,
    TripleFlat,
    SlashQuarterSharp,
    SlashSharp,
    SlashFlat,
    DoubleSlashFlat,
    Sharp1,
    Sharp2,
    Sharp3,
    Sharp5,
    Flat1,
    Flat2,
    Flat3,
    Flat4,
    Sori,
    Koron,
    Other,
}

/// Accidental element for a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accidental {
    /// The accidental value.
    #[serde(rename = "$value")]
    pub value: AccidentalValue,

    /// Whether the accidental is cautionary.
    #[serde(rename = "@cautionary", skip_serializing_if = "Option::is_none")]
    pub cautionary: Option<YesNo>,

    /// Whether the accidental is editorial.
    #[serde(rename = "@editorial", skip_serializing_if = "Option::is_none")]
    pub editorial: Option<YesNo>,

    /// Whether the accidental is in parentheses.
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Whether the accidental is in brackets.
    #[serde(rename = "@bracket", skip_serializing_if = "Option::is_none")]
    pub bracket: Option<YesNo>,

    /// Size of the accidental symbol.
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<SymbolSize>,

    /// SMuFL glyph name for the accidental.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

impl Accidental {
    /// Create a new accidental.
    pub fn new(value: AccidentalValue) -> Self {
        Self {
            value,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: None,
            size: None,
            smufl: None,
        }
    }
}

// ============================================================================
// Time Modification (Tuplets)
// ============================================================================

/// Time modification for tuplets.
///
/// Indicates that a note's duration is modified for tuplet notation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeModification {
    /// The number of notes actually played.
    #[serde(rename = "actual-notes")]
    pub actual_notes: u32,

    /// The number of notes normally in this time.
    #[serde(rename = "normal-notes")]
    pub normal_notes: u32,

    /// The note type of the normal notes.
    #[serde(rename = "normal-type", skip_serializing_if = "Option::is_none")]
    pub normal_type: Option<NoteTypeValue>,

    /// Dots on the normal notes.
    #[serde(rename = "normal-dot", default, skip_serializing_if = "Vec::is_empty")]
    pub normal_dots: Vec<Empty>,
}

impl TimeModification {
    /// Create a simple tuplet (e.g., triplet = 3 in time of 2).
    pub fn new(actual: u32, normal: u32) -> Self {
        Self {
            actual_notes: actual,
            normal_notes: normal,
            normal_type: None,
            normal_dots: Vec::new(),
        }
    }

    /// Create a triplet (3 in time of 2).
    pub fn triplet() -> Self {
        Self::new(3, 2)
    }
}

// ============================================================================
// Stem
// ============================================================================

/// Stem direction and display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stem {
    /// Stem direction value.
    #[serde(rename = "$value")]
    pub value: StemValue,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative Y position.
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Color of the stem.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl Stem {
    /// Create a new stem.
    pub fn new(value: StemValue) -> Self {
        Self {
            value,
            default_y: None,
            relative_y: None,
            color: None,
        }
    }
}

/// Stem direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StemValue {
    /// Stem points down.
    Down,
    /// Stem points up.
    Up,
    /// Double stem (both up and down).
    Double,
    /// No stem.
    None,
}

// ============================================================================
// Beam
// ============================================================================

/// Beam element for note grouping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beam {
    /// Beam value (begin, continue, end, etc.).
    #[serde(rename = "$value")]
    pub value: BeamValue,

    /// Beam number (1-8).
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Whether to repeat the beam.
    #[serde(rename = "@repeater", skip_serializing_if = "Option::is_none")]
    pub repeater: Option<YesNo>,

    /// Fan acceleration/deceleration.
    #[serde(rename = "@fan", skip_serializing_if = "Option::is_none")]
    pub fan: Option<Fan>,

    /// Color of the beam.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Beam {
    /// Create a new beam.
    pub fn new(value: BeamValue) -> Self {
        Self {
            value,
            number: None,
            repeater: None,
            fan: None,
            color: None,
            id: None,
        }
    }

    /// Create a beam with a specific number.
    pub fn with_number(value: BeamValue, number: u8) -> Self {
        Self {
            value,
            number: Some(number),
            repeater: None,
            fan: None,
            color: None,
            id: None,
        }
    }
}

/// Beam value types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BeamValue {
    /// Begin a beam.
    Begin,
    /// Continue a beam.
    Continue,
    /// End a beam.
    End,
    /// Forward hook.
    ForwardHook,
    /// Backward hook.
    BackwardHook,
}

/// Fan type for beams (acceleration/deceleration).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Fan {
    Accel,
    Rit,
    None,
}

// ============================================================================
// Notehead
// ============================================================================

/// Notehead element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Notehead {
    /// Notehead value.
    #[serde(rename = "$value")]
    pub value: NoteheadValue,

    /// Whether the notehead is filled.
    #[serde(rename = "@filled", skip_serializing_if = "Option::is_none")]
    pub filled: Option<YesNo>,

    /// Whether the notehead has parentheses.
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Font family.
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style.
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font size.
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font weight.
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

impl Default for Notehead {
    fn default() -> Self {
        Self {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: None,
            font_family: None,
            font_style: None,
            font_size: None,
            font_weight: None,
            color: None,
            smufl: None,
        }
    }
}

impl Notehead {
    /// Create a new notehead.
    pub fn new(value: NoteheadValue) -> Self {
        Self {
            value,
            filled: None,
            parentheses: None,
            font_family: None,
            font_style: None,
            font_size: None,
            font_weight: None,
            color: None,
            smufl: None,
        }
    }
}

/// Notehead value types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NoteheadValue {
    Slash,
    Triangle,
    Diamond,
    Square,
    Cross,
    X,
    CircleX,
    InvertedTriangle,
    ArrowDown,
    ArrowUp,
    Circled,
    Slashed,
    BackSlashed,
    Normal,
    Cluster,
    CircleDot,
    LeftTriangle,
    Rectangle,
    None,
    Do,
    Re,
    Mi,
    Fa,
    FaUp,
    So,
    La,
    Ti,
    Other,
}

// ============================================================================
// Notehead Text
// ============================================================================

/// Text displayed inside a notehead (for educational music).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteheadText {
    /// Sequence of display-text and accidental-text children.
    pub children: Vec<NoteheadTextChild>,
}

/// Child element of notehead-text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NoteheadTextChild {
    /// Display text with optional formatting.
    DisplayText(FormattedText),
    /// Accidental text.
    AccidentalText(AccidentalText),
}

/// Accidental text element used in notehead-text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccidentalText {
    /// The accidental value.
    #[serde(rename = "$value")]
    pub value: AccidentalValue,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

impl Default for AccidentalText {
    fn default() -> Self {
        Self {
            value: AccidentalValue::Natural,
            smufl: None,
        }
    }
}

// ============================================================================
// Dot
// ============================================================================

/// Empty placement for augmentation dots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Dot {
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

    /// Placement (above or below).
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

// ============================================================================
// Instrument Reference
// ============================================================================

/// Instrument reference within a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instrument {
    /// The instrument ID reference.
    #[serde(rename = "@id")]
    pub id: String,
}

impl Instrument {
    /// Create a new instrument reference.
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

// ============================================================================
// Note Element
// ============================================================================

/// A complete note element.
///
/// Notes can be regular notes, grace notes, or cue notes.
/// They contain pitch/rest/unpitched information, duration, and various
/// display elements like stems, beams, and articulations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    // --- Note kind (grace/cue/regular) ---
    /// Grace note information (present for grace notes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace: Option<Grace>,

    /// Cue note indicator (empty element).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cue: Option<Empty>,

    // --- Full note content ---
    /// Chord indicator - if present, note is part of a chord.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chord: Option<Empty>,

    /// The pitch, unpitched, or rest content.
    #[serde(flatten)]
    pub content: FullNoteContent,

    // --- Duration ---
    /// Note duration in divisions (not present for grace notes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Ties (start/stop, max 2).
    #[serde(rename = "tie", default, skip_serializing_if = "Vec::is_empty")]
    pub ties: Vec<Tie>,

    // --- Editorial/Voice ---
    /// Footnote text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnote: Option<FormattedText>,

    /// Level information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    /// Voice for the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,

    // --- Instrument ---
    /// Instrument references.
    #[serde(rename = "instrument", default, skip_serializing_if = "Vec::is_empty")]
    pub instruments: Vec<Instrument>,

    // --- Display ---
    /// Note type (graphical duration).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub note_type: Option<NoteType>,

    /// Augmentation dots.
    #[serde(rename = "dot", default, skip_serializing_if = "Vec::is_empty")]
    pub dots: Vec<Dot>,

    /// Accidental.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accidental: Option<Accidental>,

    /// Time modification (tuplets).
    #[serde(rename = "time-modification", skip_serializing_if = "Option::is_none")]
    pub time_modification: Option<TimeModification>,

    /// Stem direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem: Option<Stem>,

    /// Notehead type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notehead: Option<Notehead>,

    /// Notehead text (display text/accidental text inside notehead).
    #[serde(rename = "notehead-text", skip_serializing_if = "Option::is_none")]
    pub notehead_text: Option<NoteheadText>,

    /// Staff number (for multi-staff parts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<u32>,

    /// Beams (max 8).
    #[serde(rename = "beam", default, skip_serializing_if = "Vec::is_empty")]
    pub beams: Vec<Beam>,

    /// Notations (slurs, ties, articulations, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notations: Option<super::notations::Notations>,

    /// Lyrics (verse text, syllables).
    #[serde(rename = "lyric", default, skip_serializing_if = "Vec::is_empty")]
    pub lyrics: Vec<super::lyric::Lyric>,

    /// Play (playback techniques: IPA, mute, semi-pitched, other).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<super::direction::Play>,

    /// Listen (interactive performance note assessment/waiting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<super::listening::Listen>,

    // --- Attributes ---
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

    /// Whether to print the note.
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Whether to print leger lines.
    #[serde(rename = "@print-leger", skip_serializing_if = "Option::is_none")]
    pub print_leger: Option<YesNo>,

    /// Whether to print spacing dots.
    #[serde(rename = "@print-spacing", skip_serializing_if = "Option::is_none")]
    pub print_spacing: Option<YesNo>,

    /// Dynamics value (0-100+).
    #[serde(rename = "@dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f64>,

    /// End dynamics value.
    #[serde(rename = "@end-dynamics", skip_serializing_if = "Option::is_none")]
    pub end_dynamics: Option<f64>,

    /// Attack offset in divisions.
    #[serde(rename = "@attack", skip_serializing_if = "Option::is_none")]
    pub attack: Option<f64>,

    /// Release offset in divisions.
    #[serde(rename = "@release", skip_serializing_if = "Option::is_none")]
    pub release: Option<f64>,

    /// Pizzicato indication.
    #[serde(rename = "@pizzicato", skip_serializing_if = "Option::is_none")]
    pub pizzicato: Option<YesNo>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Note {
    /// Create a new pitched note.
    pub fn pitched(pitch: Pitch, duration: f64) -> Self {
        Self {
            grace: None,
            cue: None,
            chord: None,
            content: FullNoteContent::Pitch(pitch),
            duration: Some(duration),
            ties: Vec::new(),
            footnote: None,
            level: None,
            voice: None,
            instruments: Vec::new(),
            note_type: None,
            dots: Vec::new(),
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            notehead_text: None,
            staff: None,
            beams: Vec::new(),
            notations: None,
            lyrics: Vec::new(),
            play: None,
            listen: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            print_object: None,
            print_leger: None,
            print_spacing: None,
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            color: None,
            id: None,
        }
    }

    /// Create a new rest.
    pub fn rest(rest: Rest, duration: f64) -> Self {
        Self {
            grace: None,
            cue: None,
            chord: None,
            content: FullNoteContent::Rest(rest),
            duration: Some(duration),
            ties: Vec::new(),
            footnote: None,
            level: None,
            voice: None,
            instruments: Vec::new(),
            note_type: None,
            dots: Vec::new(),
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            notehead_text: None,
            staff: None,
            beams: Vec::new(),
            notations: None,
            lyrics: Vec::new(),
            play: None,
            listen: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            print_object: None,
            print_leger: None,
            print_spacing: None,
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            color: None,
            id: None,
        }
    }

    /// Create a grace note.
    pub fn grace_note(pitch: Pitch, grace: Grace) -> Self {
        Self {
            grace: Some(grace),
            cue: None,
            chord: None,
            content: FullNoteContent::Pitch(pitch),
            duration: None, // Grace notes have no duration
            ties: Vec::new(),
            footnote: None,
            level: None,
            voice: None,
            instruments: Vec::new(),
            note_type: None,
            dots: Vec::new(),
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            notehead_text: None,
            staff: None,
            beams: Vec::new(),
            notations: None,
            lyrics: Vec::new(),
            play: None,
            listen: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            print_object: None,
            print_leger: None,
            print_spacing: None,
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            color: None,
            id: None,
        }
    }

    /// Create an unpitched note (for percussion).
    pub fn unpitched(unpitched: Unpitched, duration: f64) -> Self {
        Self {
            grace: None,
            cue: None,
            chord: None,
            content: FullNoteContent::Unpitched(unpitched),
            duration: Some(duration),
            ties: Vec::new(),
            footnote: None,
            level: None,
            voice: None,
            instruments: Vec::new(),
            note_type: None,
            dots: Vec::new(),
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            notehead_text: None,
            staff: None,
            beams: Vec::new(),
            notations: None,
            lyrics: Vec::new(),
            play: None,
            listen: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            print_object: None,
            print_leger: None,
            print_spacing: None,
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            color: None,
            id: None,
        }
    }

    /// Create an unpitched grace note (for percussion).
    pub fn unpitched_grace(unpitched: Unpitched, grace: Grace) -> Self {
        Self {
            grace: Some(grace),
            cue: None,
            chord: None,
            content: FullNoteContent::Unpitched(unpitched),
            duration: None, // Grace notes have no duration
            ties: Vec::new(),
            footnote: None,
            level: None,
            voice: None,
            instruments: Vec::new(),
            note_type: None,
            dots: Vec::new(),
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            notehead_text: None,
            staff: None,
            beams: Vec::new(),
            notations: None,
            lyrics: Vec::new(),
            play: None,
            listen: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            print_object: None,
            print_leger: None,
            print_spacing: None,
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            color: None,
            id: None,
        }
    }

    /// Check if this is a grace note.
    pub fn is_grace(&self) -> bool {
        self.grace.is_some()
    }

    /// Check if this is a cue note.
    pub fn is_cue(&self) -> bool {
        self.cue.is_some()
    }

    /// Check if this is part of a chord.
    pub fn is_chord(&self) -> bool {
        self.chord.is_some()
    }

    /// Check if this is a rest.
    pub fn is_rest(&self) -> bool {
        matches!(self.content, FullNoteContent::Rest(_))
    }

    /// Get the pitch if this is a pitched note.
    pub fn pitch(&self) -> Option<&Pitch> {
        match &self.content {
            FullNoteContent::Pitch(p) => Some(p),
            _ => None,
        }
    }

    /// Check if this is an unpitched note (percussion).
    pub fn is_unpitched(&self) -> bool {
        matches!(self.content, FullNoteContent::Unpitched(_))
    }

    /// Get the unpitched content if this is an unpitched note.
    pub fn unpitched_content(&self) -> Option<&Unpitched> {
        match &self.content {
            FullNoteContent::Unpitched(u) => Some(u),
            _ => None,
        }
    }
}

// ============================================================================
// Backup and Forward
// ============================================================================

/// Backup element - moves the cursor backward in time.
///
/// Used for multi-voice notation to go back and notate another voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    /// Duration to move backward (in divisions).
    pub duration: f64,

    /// Footnote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnote: Option<FormattedText>,

    /// Level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
}

impl Backup {
    /// Create a new backup element.
    pub fn new(duration: f64) -> Self {
        Self {
            duration,
            footnote: None,
            level: None,
        }
    }
}

/// Forward element - moves the cursor forward in time.
///
/// Used to skip space in a voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forward {
    /// Duration to move forward (in divisions).
    pub duration: f64,

    /// Footnote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnote: Option<FormattedText>,

    /// Level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    /// Voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,

    /// Staff number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<u32>,
}

impl Forward {
    /// Create a new forward element.
    pub fn new(duration: f64) -> Self {
        Self {
            duration,
            footnote: None,
            level: None,
            voice: None,
            staff: None,
        }
    }
}

// ============================================================================
// Editorial Types
// ============================================================================

/// Formatted text element.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FormattedText {
    /// Text content.
    #[serde(rename = "$value")]
    pub value: String,

    /// Language.
    #[serde(rename = "@xml:lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Text direction.
    #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
    pub dir: Option<TextDirection>,

    /// Enclosure shape.
    #[serde(rename = "@enclosure", skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<EnclosureShape>,
}

/// Level element for editorial markup.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Level {
    /// Level text.
    #[serde(rename = "$value")]
    pub value: String,

    /// Whether to use parentheses.
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Whether to use brackets.
    #[serde(rename = "@bracket", skip_serializing_if = "Option::is_none")]
    pub bracket: Option<YesNo>,

    /// Reference for editorial levels.
    #[serde(rename = "@reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<YesNo>,
}

// ============================================================================
// Empty marker (reused from elements.rs but needed here too)
// ============================================================================

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Pitch Tests
    // ========================================================================

    #[test]
    fn test_pitch_new() {
        let pitch = Pitch::new(Step::C, 4);
        assert_eq!(pitch.step, Step::C);
        assert_eq!(pitch.octave, 4);
        assert!(pitch.alter.is_none());
    }

    #[test]
    fn test_pitch_with_alter() {
        let pitch = Pitch::with_alter(Step::F, 1.0, 4);
        assert_eq!(pitch.step, Step::F);
        assert_eq!(pitch.alter, Some(1.0));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_pitch_microtone() {
        let pitch = Pitch::with_alter(Step::C, 0.5, 4);
        assert_eq!(pitch.alter, Some(0.5)); // Quarter-tone sharp
    }

    // ========================================================================
    // Rest Tests
    // ========================================================================

    #[test]
    fn test_rest_new() {
        let rest = Rest::new();
        assert!(rest.display_step.is_none());
        assert!(rest.display_octave.is_none());
        assert!(rest.measure.is_none());
    }

    #[test]
    fn test_rest_measure() {
        let rest = Rest::measure_rest();
        assert_eq!(rest.measure, Some(YesNo::Yes));
    }

    #[test]
    fn test_rest_with_display() {
        let rest = Rest::with_display(Step::B, 4);
        assert_eq!(rest.display_step, Some(Step::B));
        assert_eq!(rest.display_octave, Some(4));
    }

    // ========================================================================
    // Tie Tests
    // ========================================================================

    #[test]
    fn test_tie_start() {
        let tie = Tie::start();
        assert_eq!(tie.tie_type, StartStop::Start);
    }

    #[test]
    fn test_tie_stop() {
        let tie = Tie::stop();
        assert_eq!(tie.tie_type, StartStop::Stop);
    }

    // ========================================================================
    // Note Type Tests
    // ========================================================================

    #[test]
    fn test_note_type_values() {
        assert_eq!(NoteTypeValue::Quarter.to_string(), "quarter");
        assert_eq!(NoteTypeValue::Eighth.to_string(), "eighth");
        assert_eq!(NoteTypeValue::Half.to_string(), "half");
        assert_eq!(NoteTypeValue::Whole.to_string(), "whole");
        assert_eq!(NoteTypeValue::N16th.to_string(), "16th");
        assert_eq!(NoteTypeValue::N32nd.to_string(), "32nd");
    }

    // ========================================================================
    // Accidental Tests
    // ========================================================================

    #[test]
    fn test_accidental_new() {
        let acc = Accidental::new(AccidentalValue::Sharp);
        assert_eq!(acc.value, AccidentalValue::Sharp);
        assert!(acc.cautionary.is_none());
        assert!(acc.editorial.is_none());
    }

    // ========================================================================
    // Time Modification Tests
    // ========================================================================

    #[test]
    fn test_time_modification_triplet() {
        let tm = TimeModification::triplet();
        assert_eq!(tm.actual_notes, 3);
        assert_eq!(tm.normal_notes, 2);
    }

    #[test]
    fn test_time_modification_new() {
        let tm = TimeModification::new(5, 4);
        assert_eq!(tm.actual_notes, 5);
        assert_eq!(tm.normal_notes, 4);
    }

    // ========================================================================
    // Stem Tests
    // ========================================================================

    #[test]
    fn test_stem_new() {
        let stem = Stem::new(StemValue::Up);
        assert_eq!(stem.value, StemValue::Up);
    }

    #[test]
    fn test_stem_values() {
        assert!(matches!(StemValue::Up, StemValue::Up));
        assert!(matches!(StemValue::Down, StemValue::Down));
        assert!(matches!(StemValue::Double, StemValue::Double));
        assert!(matches!(StemValue::None, StemValue::None));
    }

    // ========================================================================
    // Beam Tests
    // ========================================================================

    #[test]
    fn test_beam_new() {
        let beam = Beam::new(BeamValue::Begin);
        assert_eq!(beam.value, BeamValue::Begin);
        assert!(beam.number.is_none());
    }

    #[test]
    fn test_beam_with_number() {
        let beam = Beam::with_number(BeamValue::Continue, 1);
        assert_eq!(beam.value, BeamValue::Continue);
        assert_eq!(beam.number, Some(1));
    }

    // ========================================================================
    // Note Tests
    // ========================================================================

    #[test]
    fn test_note_pitched() {
        let note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        assert!(!note.is_grace());
        assert!(!note.is_cue());
        assert!(!note.is_chord());
        assert!(!note.is_rest());
        assert!(note.pitch().is_some());
        assert_eq!(note.duration, Some(4.0));
    }

    #[test]
    fn test_note_rest() {
        let note = Note::rest(Rest::new(), 4.0);
        assert!(note.is_rest());
        assert!(note.pitch().is_none());
    }

    #[test]
    fn test_note_grace() {
        let note = Note::grace_note(Pitch::new(Step::D, 5), Grace::default());
        assert!(note.is_grace());
        assert!(note.duration.is_none()); // Grace notes have no duration
    }

    #[test]
    fn test_note_with_voice() {
        let mut note = Note::pitched(Pitch::new(Step::E, 4), 2.0);
        note.voice = Some("1".to_string());
        assert_eq!(note.voice, Some("1".to_string()));
    }

    #[test]
    fn test_note_with_staff() {
        let mut note = Note::pitched(Pitch::new(Step::G, 3), 4.0);
        note.staff = Some(2);
        assert_eq!(note.staff, Some(2));
    }

    #[test]
    fn test_note_with_ties() {
        let mut note = Note::pitched(Pitch::new(Step::A, 4), 4.0);
        note.ties.push(Tie::start());
        assert_eq!(note.ties.len(), 1);
        assert_eq!(note.ties[0].tie_type, StartStop::Start);
    }

    #[test]
    fn test_note_with_beams() {
        let mut note = Note::pitched(Pitch::new(Step::B, 4), 1.0);
        note.beams.push(Beam::with_number(BeamValue::Begin, 1));
        assert_eq!(note.beams.len(), 1);
    }

    // ========================================================================
    // Backup/Forward Tests
    // ========================================================================

    #[test]
    fn test_backup_new() {
        let backup = Backup::new(4.0);
        assert_eq!(backup.duration, 4.0);
        assert!(backup.footnote.is_none());
    }

    #[test]
    fn test_forward_new() {
        let forward = Forward::new(2.0);
        assert_eq!(forward.duration, 2.0);
        assert!(forward.voice.is_none());
        assert!(forward.staff.is_none());
    }

    #[test]
    fn test_forward_with_voice() {
        let mut forward = Forward::new(4.0);
        forward.voice = Some("2".to_string());
        forward.staff = Some(1);
        assert_eq!(forward.voice, Some("2".to_string()));
        assert_eq!(forward.staff, Some(1));
    }

    // ========================================================================
    // Instrument Tests
    // ========================================================================

    #[test]
    fn test_instrument_new() {
        let inst = Instrument::new("P1-I1");
        assert_eq!(inst.id, "P1-I1");
    }

    // ========================================================================
    // Grace Tests
    // ========================================================================

    #[test]
    fn test_grace_default() {
        let grace = Grace::default();
        assert!(grace.steal_time_previous.is_none());
        assert!(grace.steal_time_following.is_none());
        assert!(grace.slash.is_none());
    }

    #[test]
    fn test_grace_with_slash() {
        let grace = Grace {
            slash: Some(YesNo::Yes),
            ..Default::default()
        };
        assert_eq!(grace.slash, Some(YesNo::Yes));
    }

    // ========================================================================
    // Notehead Tests
    // ========================================================================

    #[test]
    fn test_notehead_new() {
        let nh = Notehead::new(NoteheadValue::Diamond);
        assert_eq!(nh.value, NoteheadValue::Diamond);
    }

    #[test]
    fn test_notehead_values() {
        assert!(matches!(NoteheadValue::Normal, NoteheadValue::Normal));
        assert!(matches!(NoteheadValue::X, NoteheadValue::X));
        assert!(matches!(NoteheadValue::Diamond, NoteheadValue::Diamond));
    }
}

// ============================================================================
// Visual Attribute Roundtrip
// ============================================================================

/// Lightweight struct capturing note-level visual/position/print attributes
/// for lossless JSON-in-label roundtrip. Only serialized when at least one
/// field is non-None.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NoteVisualAttrs {
    #[serde(rename = "dx", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,
    #[serde(rename = "dy", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,
    #[serde(rename = "rx", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,
    #[serde(rename = "ry", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,
    #[serde(rename = "po", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub print_leger: Option<YesNo>,
    #[serde(rename = "ps", skip_serializing_if = "Option::is_none")]
    pub print_spacing: Option<YesNo>,
    #[serde(rename = "col", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "dyn", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f64>,
    #[serde(rename = "ed", skip_serializing_if = "Option::is_none")]
    pub end_dynamics: Option<f64>,
    #[serde(rename = "att", skip_serializing_if = "Option::is_none")]
    pub attack: Option<f64>,
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub release: Option<f64>,
    #[serde(rename = "piz", skip_serializing_if = "Option::is_none")]
    pub pizzicato: Option<YesNo>,
}

impl NoteVisualAttrs {
    /// Build from a MusicXML Note's visual attributes.
    pub fn from_note(note: &Note) -> Self {
        Self {
            default_x: note.default_x,
            default_y: note.default_y,
            relative_x: note.relative_x,
            relative_y: note.relative_y,
            print_object: note.print_object,
            print_leger: note.print_leger,
            print_spacing: note.print_spacing,
            color: note.color.clone(),
            dynamics: note.dynamics,
            end_dynamics: note.end_dynamics,
            attack: note.attack,
            release: note.release,
            pizzicato: note.pizzicato,
        }
    }

    /// Returns true if all fields are None (nothing to store).
    pub fn is_empty(&self) -> bool {
        self.default_x.is_none()
            && self.default_y.is_none()
            && self.relative_x.is_none()
            && self.relative_y.is_none()
            && self.print_object.is_none()
            && self.print_leger.is_none()
            && self.print_spacing.is_none()
            && self.color.is_none()
            && self.dynamics.is_none()
            && self.end_dynamics.is_none()
            && self.attack.is_none()
            && self.release.is_none()
            && self.pizzicato.is_none()
    }

    /// Apply stored attributes back to a MusicXML note.
    pub fn apply_to_note(&self, note: &mut Note) {
        if self.default_x.is_some() {
            note.default_x = self.default_x;
        }
        if self.default_y.is_some() {
            note.default_y = self.default_y;
        }
        if self.relative_x.is_some() {
            note.relative_x = self.relative_x;
        }
        if self.relative_y.is_some() {
            note.relative_y = self.relative_y;
        }
        if self.print_object.is_some() {
            note.print_object = self.print_object;
        }
        if self.print_leger.is_some() {
            note.print_leger = self.print_leger;
        }
        if self.print_spacing.is_some() {
            note.print_spacing = self.print_spacing;
        }
        if self.color.is_some() {
            note.color.clone_from(&self.color);
        }
        if self.dynamics.is_some() {
            note.dynamics = self.dynamics;
        }
        if self.end_dynamics.is_some() {
            note.end_dynamics = self.end_dynamics;
        }
        if self.attack.is_some() {
            note.attack = self.attack;
        }
        if self.release.is_some() {
            note.release = self.release;
        }
        if self.pizzicato.is_some() {
            note.pizzicato = self.pizzicato;
        }
    }
}

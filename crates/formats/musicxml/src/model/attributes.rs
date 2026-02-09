//! MusicXML 4.0 attributes element types.
//!
//! This module contains types for representing musical attributes that typically
//! change on measure boundaries: key signatures, time signatures, clefs,
//! divisions, transpositions, and staving information.

use serde::{Deserialize, Serialize};

use super::data::*;

// ============================================================================
// Attributes Element
// ============================================================================

/// Musical attributes that typically change on measure boundaries.
///
/// This includes key and time signatures, clefs, transpositions, and staving.
/// When attributes are changed mid-measure, it affects the music in score order,
/// not in MusicXML document order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Attributes {
    /// Musical notation divisions per quarter note for duration calculations.
    ///
    /// For example, if duration = 1 and divisions = 2, this is an eighth note.
    /// Must be positive. For maximum MIDI 1.0 compatibility, should not exceed 16383.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisions: Option<f64>,

    /// Key signatures. Multiple keys allowed for polymodal/polytonal music.
    #[serde(rename = "key", default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<Key>,

    /// Time signatures. Multiple times allowed for composite signatures.
    #[serde(rename = "time", default, skip_serializing_if = "Vec::is_empty")]
    pub times: Vec<Time>,

    /// Number of staves in this part (default 1 if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staves: Option<u32>,

    /// Part symbol for multi-staff parts.
    #[serde(rename = "part-symbol", skip_serializing_if = "Option::is_none")]
    pub part_symbol: Option<PartSymbol>,

    /// Number of instruments in this part (default 1 if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruments: Option<u32>,

    /// Clefs for the staves.
    #[serde(rename = "clef", default, skip_serializing_if = "Vec::is_empty")]
    pub clefs: Vec<Clef>,

    /// Staff details for special staff types.
    #[serde(
        rename = "staff-details",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub staff_details: Vec<StaffDetails>,

    /// Transposition information for transposing instruments.
    #[serde(rename = "transpose", default, skip_serializing_if = "Vec::is_empty")]
    pub transposes: Vec<Transpose>,

    /// Measure style (multiple rests, repeats, slash notation).
    #[serde(
        rename = "measure-style",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub measure_styles: Vec<MeasureStyle>,
}

impl Attributes {
    /// Create new attributes with the given divisions.
    pub fn with_divisions(divisions: f64) -> Self {
        Self {
            divisions: Some(divisions),
            ..Default::default()
        }
    }

    /// Create new attributes with a key signature.
    pub fn with_key(key: Key) -> Self {
        Self {
            keys: vec![key],
            ..Default::default()
        }
    }

    /// Create new attributes with a time signature.
    pub fn with_time(time: Time) -> Self {
        Self {
            times: vec![time],
            ..Default::default()
        }
    }

    /// Create new attributes with a clef.
    pub fn with_clef(clef: Clef) -> Self {
        Self {
            clefs: vec![clef],
            ..Default::default()
        }
    }
}

// ============================================================================
// Key Signature
// ============================================================================

/// A key signature.
///
/// Supports both traditional key signatures using the cycle of fifths
/// and non-traditional key signatures with explicit alterations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    /// Staff number this key applies to (absent = all staves).
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Whether to print this key signature.
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The key content (traditional or non-traditional).
    #[serde(flatten)]
    pub content: KeyContent,

    /// Key octave specifications.
    #[serde(rename = "key-octave", default, skip_serializing_if = "Vec::is_empty")]
    pub key_octaves: Vec<KeyOctave>,
}

impl Key {
    /// Create a traditional key signature.
    pub fn traditional(fifths: i8, mode: Option<Mode>) -> Self {
        Self {
            number: None,
            print_object: None,
            id: None,
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths,
                mode,
            }),
            key_octaves: Vec::new(),
        }
    }

    /// Create a C major key signature.
    pub fn c_major() -> Self {
        Self::traditional(0, Some(Mode::Major))
    }

    /// Create an A minor key signature.
    pub fn a_minor() -> Self {
        Self::traditional(0, Some(Mode::Minor))
    }

    /// Create a key signature with the given number of sharps (positive) or flats (negative).
    pub fn from_fifths(fifths: i8) -> Self {
        Self::traditional(fifths, None)
    }
}

/// Key signature content - either traditional or non-traditional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyContent {
    /// Traditional key signature using circle of fifths.
    Traditional(TraditionalKey),
    /// Non-traditional key signature with explicit alterations.
    NonTraditional(NonTraditionalKey),
}

/// Traditional key signature using the cycle of fifths.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraditionalKey {
    /// Cancellation of previous key signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Cancel>,

    /// Number of flats (negative) or sharps (positive).
    pub fifths: i8,

    /// Mode (major, minor, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

/// Non-traditional key signature with explicit alterations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NonTraditionalKey {
    /// The alterations in this key signature.
    #[serde(rename = "$value", default)]
    pub alterations: Vec<KeyAlteration>,
}

/// A single alteration in a non-traditional key signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAlteration {
    /// The pitch step to be altered.
    #[serde(rename = "key-step")]
    pub key_step: Step,

    /// The alteration value in semitones.
    #[serde(rename = "key-alter")]
    pub key_alter: f64,

    /// The accidental to display (optional).
    #[serde(rename = "key-accidental", skip_serializing_if = "Option::is_none")]
    pub key_accidental: Option<KeyAccidental>,
}

/// Key accidental for non-traditional key signatures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAccidental {
    /// The accidental value.
    #[serde(rename = "$value")]
    pub value: AccidentalValue,

    /// SMuFL glyph name.
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

/// Key octave specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyOctave {
    /// The octave value.
    #[serde(rename = "$value")]
    pub octave: u8,

    /// Which key signature element this applies to (1-indexed, left-to-right).
    #[serde(rename = "@number")]
    pub number: u32,

    /// Whether this refers to a cancellation (default: no).
    #[serde(rename = "@cancel", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<YesNo>,
}

/// Cancellation of a previous key signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cancel {
    /// The fifths value of the cancelled key signature.
    #[serde(rename = "$value")]
    pub fifths: i8,

    /// Where the cancellation appears relative to the new key.
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<CancelLocation>,
}

/// Location of key signature cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CancelLocation {
    Left,
    Right,
    BeforeBarline,
}

/// Mode of a key signature.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Major,
    Minor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Ionian,
    Locrian,
    None,
    /// Other mode (free text).
    #[serde(untagged)]
    Other(String),
}

// ============================================================================
// Time Signature
// ============================================================================

/// A time signature.
///
/// Time signatures can use standard beat/beat-type notation, common/cut symbols,
/// or senza misura (no time signature).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    /// Staff number this time signature applies to (absent = all staves).
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Symbol for display (common, cut, single-number, etc.).
    #[serde(rename = "@symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<TimeSymbol>,

    /// Separator between compound time signature components.
    #[serde(rename = "@separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<TimeSeparator>,

    /// Whether to print this time signature.
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The time signature content.
    #[serde(flatten)]
    pub content: TimeContent,
}

impl Time {
    /// Create a standard time signature (e.g., 4/4, 3/4).
    pub fn new(beats: &str, beat_type: &str) -> Self {
        Self {
            number: None,
            symbol: None,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::Standard(StandardTime {
                signatures: vec![TimeSignature {
                    beats: beats.to_string(),
                    beat_type: beat_type.to_string(),
                }],
                interchangeable: None,
            }),
        }
    }

    /// Create common time (4/4 with C symbol).
    pub fn common() -> Self {
        Self {
            symbol: Some(TimeSymbol::Common),
            ..Self::new("4", "4")
        }
    }

    /// Create cut time (2/2 with cut C symbol).
    pub fn cut() -> Self {
        Self {
            symbol: Some(TimeSymbol::Cut),
            ..Self::new("2", "2")
        }
    }

    /// Create a compound time signature (e.g., 3+2/8).
    pub fn compound(beats: &str, beat_type: &str) -> Self {
        Self::new(beats, beat_type)
    }

    /// Create senza misura (no time signature).
    pub fn senza_misura() -> Self {
        Self {
            number: None,
            symbol: None,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::SenzaMisura(SenzaMisura { symbol: None }),
        }
    }
}

/// Time signature content - standard or senza misura.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimeContent {
    /// Standard time signature with beats and beat-type.
    Standard(StandardTime),
    /// No time signature (free meter).
    SenzaMisura(SenzaMisura),
}

/// Standard time signature with beat/beat-type pairs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardTime {
    /// The time signature components.
    #[serde(rename = "$value")]
    pub signatures: Vec<TimeSignature>,

    /// Interchangeable time signature for dual-time notation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interchangeable: Option<Interchangeable>,
}

/// A single time signature component (beats/beat-type pair).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSignature {
    /// The beats (numerator). Can include "+" for additive meters.
    pub beats: String,

    /// The beat type (denominator).
    #[serde(rename = "beat-type")]
    pub beat_type: String,
}

/// Interchangeable time signature for dual-time notation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interchangeable {
    /// Symbol for the interchangeable time.
    #[serde(rename = "@symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<TimeSymbol>,

    /// Separator.
    #[serde(rename = "@separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<TimeSeparator>,

    /// The time relation.
    #[serde(rename = "time-relation", skip_serializing_if = "Option::is_none")]
    pub time_relation: Option<TimeRelation>,

    /// The interchangeable signatures.
    #[serde(rename = "$value")]
    pub signatures: Vec<TimeSignature>,
}

/// Senza misura (no time signature).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SenzaMisura {
    /// Optional symbol to display (e.g., "X").
    #[serde(rename = "senza-misura", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Time signature display symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimeSymbol {
    /// Standard fractional display.
    Normal,
    /// Common time (C symbol).
    Common,
    /// Cut time (cut C symbol).
    Cut,
    /// Single number display.
    SingleNumber,
    /// Note symbol instead of number.
    Note,
    /// Dotted note symbol.
    DottedNote,
}

/// Separator between time signature components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeSeparator {
    None,
    Horizontal,
    Diagonal,
    Vertical,
    Adjacent,
}

/// Time relation for interchangeable time signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeRelation {
    Parentheses,
    Bracket,
    Equals,
    Slash,
    Space,
    Hyphen,
}

// ============================================================================
// Clef
// ============================================================================

/// A clef specification.
///
/// Clefs are represented by a combination of sign, line, and optional
/// octave-change elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clef {
    /// Staff number this clef applies to (default 1).
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Whether this is an additional clef (for cue passages, etc.).
    #[serde(rename = "@additional", skip_serializing_if = "Option::is_none")]
    pub additional: Option<YesNo>,

    /// Size for additional clefs.
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<SymbolSize>,

    /// Whether this clef appears after the barline.
    #[serde(rename = "@after-barline", skip_serializing_if = "Option::is_none")]
    pub after_barline: Option<YesNo>,

    /// Whether to print this clef.
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The clef sign (G, F, C, percussion, TAB, jianpu, none).
    pub sign: ClefSign,

    /// The line on which the clef sits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    /// Octave change for transposing clefs (-1 for tenor, +1 for 8va, etc.).
    #[serde(rename = "clef-octave-change", skip_serializing_if = "Option::is_none")]
    pub clef_octave_change: Option<i32>,
}

impl Clef {
    /// Create a new clef.
    pub fn new(sign: ClefSign, line: Option<u32>) -> Self {
        Self {
            number: None,
            additional: None,
            size: None,
            after_barline: None,
            print_object: None,
            id: None,
            sign,
            line,
            clef_octave_change: None,
        }
    }

    /// Create a treble (G) clef.
    pub fn treble() -> Self {
        Self::new(ClefSign::G, Some(2))
    }

    /// Create a bass (F) clef.
    pub fn bass() -> Self {
        Self::new(ClefSign::F, Some(4))
    }

    /// Create an alto (C) clef.
    pub fn alto() -> Self {
        Self::new(ClefSign::C, Some(3))
    }

    /// Create a tenor (C) clef.
    pub fn tenor() -> Self {
        Self::new(ClefSign::C, Some(4))
    }

    /// Create a percussion clef.
    pub fn percussion() -> Self {
        Self::new(ClefSign::Percussion, None)
    }

    /// Create a TAB clef.
    pub fn tab() -> Self {
        Self::new(ClefSign::Tab, None)
    }

    /// Create a treble clef 8va (one octave higher).
    pub fn treble_8va() -> Self {
        Self {
            clef_octave_change: Some(1),
            ..Self::treble()
        }
    }

    /// Create a treble clef 8vb (one octave lower, like tenor voice).
    pub fn treble_8vb() -> Self {
        Self {
            clef_octave_change: Some(-1),
            ..Self::treble()
        }
    }

    /// Create a bass clef 8vb (one octave lower).
    pub fn bass_8vb() -> Self {
        Self {
            clef_octave_change: Some(-1),
            ..Self::bass()
        }
    }
}

/// Clef sign type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClefSign {
    /// G clef (treble).
    G,
    /// F clef (bass).
    F,
    /// C clef (alto/tenor).
    C,
    /// Percussion clef.
    #[serde(rename = "percussion")]
    Percussion,
    /// Tablature.
    #[serde(rename = "TAB")]
    Tab,
    /// Jianpu numbered notation.
    #[serde(rename = "jianpu")]
    Jianpu,
    /// No clef (deprecated in MusicXML 4.0).
    #[serde(rename = "none")]
    None,
}

// ============================================================================
// Transpose
// ============================================================================

/// Transposition information for transposing instruments.
///
/// Indicates what must be added to a written pitch to get the correct
/// sounding pitch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transpose {
    /// Staff number this transposition applies to (absent = all staves).
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Diatonic steps for correct spelling of enharmonic transpositions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diatonic: Option<i32>,

    /// Chromatic semitones from written to sounding pitch.
    pub chromatic: f64,

    /// Octave change (for transpositions of an octave or more).
    #[serde(rename = "octave-change", skip_serializing_if = "Option::is_none")]
    pub octave_change: Option<i32>,

    /// Whether to double the music one octave.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double: Option<Double>,
}

impl Transpose {
    /// Create a new transposition.
    pub fn new(chromatic: f64) -> Self {
        Self {
            number: None,
            id: None,
            diatonic: None,
            chromatic,
            octave_change: None,
            double: None,
        }
    }

    /// Create a transposition for Bb instruments (clarinet, trumpet, etc.).
    /// Written C sounds Bb (down a major 2nd = -2 semitones).
    pub fn bb_instrument() -> Self {
        Self {
            diatonic: Some(-1),
            chromatic: -2.0,
            ..Self::new(-2.0)
        }
    }

    /// Create a transposition for Eb instruments (alto sax, etc.).
    /// Written C sounds Eb (down a major 6th = -9 semitones).
    pub fn eb_instrument() -> Self {
        Self {
            diatonic: Some(-5),
            chromatic: -9.0,
            octave_change: Some(1), // Up an octave to compensate
            ..Self::new(-9.0)
        }
    }

    /// Create a transposition for F instruments (horn, English horn).
    /// Written C sounds F (down a perfect 5th = -7 semitones).
    pub fn f_instrument() -> Self {
        Self {
            diatonic: Some(-4),
            chromatic: -7.0,
            ..Self::new(-7.0)
        }
    }
}

/// Double indication for octave doubling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Double {
    /// Whether the doubling is above (yes) or below (no/absent).
    #[serde(rename = "@above", skip_serializing_if = "Option::is_none")]
    pub above: Option<YesNo>,
}

// ============================================================================
// Staff Details
// ============================================================================

/// Staff details for special staff types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StaffDetails {
    /// Staff number this applies to.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Whether to show frets as numbers or letters.
    #[serde(rename = "@show-frets", skip_serializing_if = "Option::is_none")]
    pub show_frets: Option<ShowFrets>,

    /// Whether to print this staff.
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Whether to print spacing for this staff.
    #[serde(rename = "@print-spacing", skip_serializing_if = "Option::is_none")]
    pub print_spacing: Option<YesNo>,

    /// Staff type.
    #[serde(rename = "staff-type", skip_serializing_if = "Option::is_none")]
    pub staff_type: Option<StaffType>,

    /// Number of staff lines.
    #[serde(rename = "staff-lines", skip_serializing_if = "Option::is_none")]
    pub staff_lines: Option<u32>,

    /// Individual line details.
    #[serde(rename = "line-detail", default, skip_serializing_if = "Vec::is_empty")]
    pub line_details: Vec<LineDetail>,

    /// Staff tuning for tablature.
    #[serde(
        rename = "staff-tuning",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub staff_tunings: Vec<StaffTuning>,

    /// Capo position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capo: Option<u32>,

    /// Staff size as a percentage with optional scaling attribute.
    #[serde(rename = "staff-size", skip_serializing_if = "Option::is_none")]
    pub staff_size: Option<StaffSize>,
}

/// Staff size with optional scaling attribute.
///
/// The staff-size element text content is a non-negative decimal representing
/// the percentage of the regular staff size. The scaling attribute specifies
/// a separate scaling factor (also non-negative decimal).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffSize {
    /// The staff size percentage value.
    #[serde(rename = "$value")]
    pub value: f64,

    /// Scaling factor.
    #[serde(rename = "@scaling", skip_serializing_if = "Option::is_none")]
    pub scaling: Option<f64>,
}

/// Staff type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaffType {
    Ossia,
    Editorial,
    Cue,
    Regular,
    Alternate,
}

/// How to show frets in tablature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShowFrets {
    Numbers,
    Letters,
}

/// Individual staff line detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineDetail {
    /// Line number (bottom to top).
    #[serde(rename = "@line")]
    pub line: u32,

    /// Line width.
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Line color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Line type.
    #[serde(rename = "@line-type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<LineType>,

    /// Whether to print this line.
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,
}

/// Staff tuning for tablature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffTuning {
    /// Line number this tuning applies to.
    #[serde(rename = "@line")]
    pub line: u32,

    /// Tuning step (pitch name).
    #[serde(rename = "tuning-step")]
    pub tuning_step: Step,

    /// Tuning alter (semitones).
    #[serde(rename = "tuning-alter", skip_serializing_if = "Option::is_none")]
    pub tuning_alter: Option<f64>,

    /// Tuning octave.
    #[serde(rename = "tuning-octave")]
    pub tuning_octave: u8,
}

// ============================================================================
// Part Symbol
// ============================================================================

/// Symbol for multi-staff parts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartSymbol {
    /// The symbol type.
    #[serde(rename = "$value")]
    pub value: PartSymbolValue,

    /// Top staff of the symbol.
    #[serde(rename = "@top-staff", skip_serializing_if = "Option::is_none")]
    pub top_staff: Option<u32>,

    /// Bottom staff of the symbol.
    #[serde(rename = "@bottom-staff", skip_serializing_if = "Option::is_none")]
    pub bottom_staff: Option<u32>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Part symbol value (same options as group symbol).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartSymbolValue {
    None,
    Brace,
    Line,
    Bracket,
    Square,
}

// ============================================================================
// Measure Style
// ============================================================================

/// Measure style for special measure notation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasureStyle {
    /// Staff number this applies to.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// The measure style content.
    #[serde(flatten)]
    pub content: MeasureStyleContent,
}

/// Measure style content types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MeasureStyleContent {
    /// Multiple-measure rest.
    MultipleRest(MultipleRest),
    /// Measure repeat.
    MeasureRepeat(MeasureRepeat),
    /// Beat repeat.
    BeatRepeat(BeatRepeat),
    /// Slash notation.
    Slash(Slash),
}

/// Multiple-measure rest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipleRest {
    /// Number of measures.
    #[serde(rename = "$value")]
    pub value: u32,

    /// Whether to use symbols instead of numbers.
    #[serde(rename = "@use-symbols", skip_serializing_if = "Option::is_none")]
    pub use_symbols: Option<YesNo>,
}

/// Measure repeat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasureRepeat {
    /// Number of measures in the pattern.
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u32>,

    /// Start or stop.
    #[serde(rename = "@type")]
    pub repeat_type: StartStop,

    /// Number of slashes in the symbol.
    #[serde(rename = "@slashes", skip_serializing_if = "Option::is_none")]
    pub slashes: Option<u32>,
}

/// Beat repeat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatRepeat {
    /// Start or stop.
    #[serde(rename = "@type")]
    pub repeat_type: StartStopContinue,

    /// Number of slashes.
    #[serde(rename = "@slashes", skip_serializing_if = "Option::is_none")]
    pub slashes: Option<u32>,

    /// Whether to use dots.
    #[serde(rename = "@use-dots", skip_serializing_if = "Option::is_none")]
    pub use_dots: Option<YesNo>,

    /// Slash type (for start).
    #[serde(rename = "slash-type", skip_serializing_if = "Option::is_none")]
    pub slash_type: Option<NoteTypeValue>,

    /// Slash dots.
    #[serde(rename = "slash-dot", default, skip_serializing_if = "Vec::is_empty")]
    pub slash_dots: Vec<EmptyElement>,

    /// Except voice (not to be slashed).
    #[serde(
        rename = "except-voice",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub except_voices: Vec<String>,
}

/// Slash notation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slash {
    /// Start or stop.
    #[serde(rename = "@type")]
    pub slash_type: StartStop,

    /// Whether to use stems.
    #[serde(rename = "@use-stems", skip_serializing_if = "Option::is_none")]
    pub use_stems: Option<YesNo>,

    /// Whether to use dots.
    #[serde(rename = "@use-dots", skip_serializing_if = "Option::is_none")]
    pub use_dots: Option<YesNo>,

    /// Slash type (for start).
    #[serde(rename = "slash-type", skip_serializing_if = "Option::is_none")]
    pub slash_type_element: Option<NoteTypeValue>,

    /// Slash dots.
    #[serde(rename = "slash-dot", default, skip_serializing_if = "Vec::is_empty")]
    pub slash_dots: Vec<EmptyElement>,

    /// Except voice (not to be slashed).
    #[serde(
        rename = "except-voice",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub except_voices: Vec<String>,
}

/// Empty element marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct EmptyElement;

// ============================================================================
// Re-export AccidentalValue from note module
// ============================================================================
use super::note::{AccidentalValue, NoteTypeValue};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Key Tests
    // ========================================================================

    #[test]
    fn test_key_c_major() {
        let key = Key::c_major();
        match &key.content {
            KeyContent::Traditional(trad) => {
                assert_eq!(trad.fifths, 0);
                assert_eq!(trad.mode, Some(Mode::Major));
            }
            _ => panic!("Expected traditional key"),
        }
    }

    #[test]
    fn test_key_a_minor() {
        let key = Key::a_minor();
        match &key.content {
            KeyContent::Traditional(trad) => {
                assert_eq!(trad.fifths, 0);
                assert_eq!(trad.mode, Some(Mode::Minor));
            }
            _ => panic!("Expected traditional key"),
        }
    }

    #[test]
    fn test_key_from_fifths() {
        // G major (1 sharp)
        let key = Key::from_fifths(1);
        match &key.content {
            KeyContent::Traditional(trad) => {
                assert_eq!(trad.fifths, 1);
                assert!(trad.mode.is_none());
            }
            _ => panic!("Expected traditional key"),
        }

        // Bb major (2 flats)
        let key = Key::from_fifths(-2);
        match &key.content {
            KeyContent::Traditional(trad) => {
                assert_eq!(trad.fifths, -2);
            }
            _ => panic!("Expected traditional key"),
        }
    }

    #[test]
    fn test_key_with_cancel() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: Some(Cancel {
                    fifths: -2,
                    location: Some(CancelLocation::Left),
                }),
                fifths: 0,
                mode: Some(Mode::Major),
            }),
            ..Key::c_major()
        };

        match &key.content {
            KeyContent::Traditional(trad) => {
                assert!(trad.cancel.is_some());
                let cancel = trad.cancel.as_ref().unwrap();
                assert_eq!(cancel.fifths, -2);
                assert_eq!(cancel.location, Some(CancelLocation::Left));
            }
            _ => panic!("Expected traditional key"),
        }
    }

    // ========================================================================
    // Time Tests
    // ========================================================================

    #[test]
    fn test_time_4_4() {
        let time = Time::new("4", "4");
        match &time.content {
            TimeContent::Standard(std) => {
                assert_eq!(std.signatures.len(), 1);
                assert_eq!(std.signatures[0].beats, "4");
                assert_eq!(std.signatures[0].beat_type, "4");
            }
            _ => panic!("Expected standard time"),
        }
        assert!(time.symbol.is_none());
    }

    #[test]
    fn test_time_common() {
        let time = Time::common();
        assert_eq!(time.symbol, Some(TimeSymbol::Common));
        match &time.content {
            TimeContent::Standard(std) => {
                assert_eq!(std.signatures[0].beats, "4");
                assert_eq!(std.signatures[0].beat_type, "4");
            }
            _ => panic!("Expected standard time"),
        }
    }

    #[test]
    fn test_time_cut() {
        let time = Time::cut();
        assert_eq!(time.symbol, Some(TimeSymbol::Cut));
        match &time.content {
            TimeContent::Standard(std) => {
                assert_eq!(std.signatures[0].beats, "2");
                assert_eq!(std.signatures[0].beat_type, "2");
            }
            _ => panic!("Expected standard time"),
        }
    }

    #[test]
    fn test_time_compound() {
        let time = Time::compound("3+2", "8");
        match &time.content {
            TimeContent::Standard(std) => {
                assert_eq!(std.signatures[0].beats, "3+2");
                assert_eq!(std.signatures[0].beat_type, "8");
            }
            _ => panic!("Expected standard time"),
        }
    }

    #[test]
    fn test_time_senza_misura() {
        let time = Time::senza_misura();
        match &time.content {
            TimeContent::SenzaMisura(_) => {}
            _ => panic!("Expected senza misura"),
        }
    }

    // ========================================================================
    // Clef Tests
    // ========================================================================

    #[test]
    fn test_clef_treble() {
        let clef = Clef::treble();
        assert_eq!(clef.sign, ClefSign::G);
        assert_eq!(clef.line, Some(2));
        assert!(clef.clef_octave_change.is_none());
    }

    #[test]
    fn test_clef_bass() {
        let clef = Clef::bass();
        assert_eq!(clef.sign, ClefSign::F);
        assert_eq!(clef.line, Some(4));
    }

    #[test]
    fn test_clef_alto() {
        let clef = Clef::alto();
        assert_eq!(clef.sign, ClefSign::C);
        assert_eq!(clef.line, Some(3));
    }

    #[test]
    fn test_clef_tenor() {
        let clef = Clef::tenor();
        assert_eq!(clef.sign, ClefSign::C);
        assert_eq!(clef.line, Some(4));
    }

    #[test]
    fn test_clef_percussion() {
        let clef = Clef::percussion();
        assert_eq!(clef.sign, ClefSign::Percussion);
        assert!(clef.line.is_none());
    }

    #[test]
    fn test_clef_tab() {
        let clef = Clef::tab();
        assert_eq!(clef.sign, ClefSign::Tab);
        assert!(clef.line.is_none());
    }

    #[test]
    fn test_clef_treble_8va() {
        let clef = Clef::treble_8va();
        assert_eq!(clef.sign, ClefSign::G);
        assert_eq!(clef.line, Some(2));
        assert_eq!(clef.clef_octave_change, Some(1));
    }

    #[test]
    fn test_clef_treble_8vb() {
        let clef = Clef::treble_8vb();
        assert_eq!(clef.sign, ClefSign::G);
        assert_eq!(clef.line, Some(2));
        assert_eq!(clef.clef_octave_change, Some(-1));
    }

    #[test]
    fn test_clef_bass_8vb() {
        let clef = Clef::bass_8vb();
        assert_eq!(clef.sign, ClefSign::F);
        assert_eq!(clef.line, Some(4));
        assert_eq!(clef.clef_octave_change, Some(-1));
    }

    // ========================================================================
    // Transpose Tests
    // ========================================================================

    #[test]
    fn test_transpose_bb_instrument() {
        let transpose = Transpose::bb_instrument();
        assert_eq!(transpose.chromatic, -2.0);
        assert_eq!(transpose.diatonic, Some(-1));
    }

    #[test]
    fn test_transpose_eb_instrument() {
        let transpose = Transpose::eb_instrument();
        assert_eq!(transpose.chromatic, -9.0);
        assert_eq!(transpose.diatonic, Some(-5));
        assert_eq!(transpose.octave_change, Some(1));
    }

    #[test]
    fn test_transpose_f_instrument() {
        let transpose = Transpose::f_instrument();
        assert_eq!(transpose.chromatic, -7.0);
        assert_eq!(transpose.diatonic, Some(-4));
    }

    // ========================================================================
    // Attributes Tests
    // ========================================================================

    #[test]
    fn test_attributes_default() {
        let attrs = Attributes::default();
        assert!(attrs.divisions.is_none());
        assert!(attrs.keys.is_empty());
        assert!(attrs.times.is_empty());
        assert!(attrs.clefs.is_empty());
    }

    #[test]
    fn test_attributes_with_divisions() {
        let attrs = Attributes::with_divisions(4.0);
        assert_eq!(attrs.divisions, Some(4.0));
    }

    #[test]
    fn test_attributes_with_key() {
        let attrs = Attributes::with_key(Key::c_major());
        assert_eq!(attrs.keys.len(), 1);
    }

    #[test]
    fn test_attributes_with_time() {
        let attrs = Attributes::with_time(Time::common());
        assert_eq!(attrs.times.len(), 1);
    }

    #[test]
    fn test_attributes_with_clef() {
        let attrs = Attributes::with_clef(Clef::treble());
        assert_eq!(attrs.clefs.len(), 1);
    }

    #[test]
    fn test_complete_attributes() {
        let attrs = Attributes {
            divisions: Some(4.0),
            keys: vec![Key::c_major()],
            times: vec![Time::common()],
            staves: Some(2),
            clefs: vec![Clef::treble(), Clef::bass()],
            ..Default::default()
        };

        assert_eq!(attrs.divisions, Some(4.0));
        assert_eq!(attrs.keys.len(), 1);
        assert_eq!(attrs.times.len(), 1);
        assert_eq!(attrs.staves, Some(2));
        assert_eq!(attrs.clefs.len(), 2);
    }

    // ========================================================================
    // Mode Tests
    // ========================================================================

    #[test]
    fn test_modes() {
        assert!(matches!(Mode::Major, Mode::Major));
        assert!(matches!(Mode::Minor, Mode::Minor));
        assert!(matches!(Mode::Dorian, Mode::Dorian));
        assert!(matches!(Mode::Phrygian, Mode::Phrygian));
        assert!(matches!(Mode::Lydian, Mode::Lydian));
        assert!(matches!(Mode::Mixolydian, Mode::Mixolydian));
        assert!(matches!(Mode::Aeolian, Mode::Aeolian));
        assert!(matches!(Mode::Ionian, Mode::Ionian));
        assert!(matches!(Mode::Locrian, Mode::Locrian));
    }

    // ========================================================================
    // TimeSymbol Tests
    // ========================================================================

    #[test]
    fn test_time_symbols() {
        assert!(matches!(TimeSymbol::Normal, TimeSymbol::Normal));
        assert!(matches!(TimeSymbol::Common, TimeSymbol::Common));
        assert!(matches!(TimeSymbol::Cut, TimeSymbol::Cut));
        assert!(matches!(TimeSymbol::SingleNumber, TimeSymbol::SingleNumber));
    }

    // ========================================================================
    // ClefSign Tests
    // ========================================================================

    #[test]
    fn test_clef_signs() {
        assert!(matches!(ClefSign::G, ClefSign::G));
        assert!(matches!(ClefSign::F, ClefSign::F));
        assert!(matches!(ClefSign::C, ClefSign::C));
        assert!(matches!(ClefSign::Percussion, ClefSign::Percussion));
        assert!(matches!(ClefSign::Tab, ClefSign::Tab));
        assert!(matches!(ClefSign::Jianpu, ClefSign::Jianpu));
        assert!(matches!(ClefSign::None, ClefSign::None));
    }

    // ========================================================================
    // Staff Details Tests
    // ========================================================================

    #[test]
    fn test_staff_details_default() {
        let details = StaffDetails::default();
        assert!(details.number.is_none());
        assert!(details.staff_type.is_none());
        assert!(details.staff_lines.is_none());
    }

    #[test]
    fn test_staff_type() {
        assert!(matches!(StaffType::Regular, StaffType::Regular));
        assert!(matches!(StaffType::Ossia, StaffType::Ossia));
        assert!(matches!(StaffType::Cue, StaffType::Cue));
        assert!(matches!(StaffType::Editorial, StaffType::Editorial));
        assert!(matches!(StaffType::Alternate, StaffType::Alternate));
    }

    // ========================================================================
    // Measure Style Tests
    // ========================================================================

    #[test]
    fn test_multiple_rest() {
        let mr = MultipleRest {
            value: 4,
            use_symbols: Some(YesNo::No),
        };
        assert_eq!(mr.value, 4);
        assert_eq!(mr.use_symbols, Some(YesNo::No));
    }

    #[test]
    fn test_measure_repeat() {
        let mr = MeasureRepeat {
            value: Some(2),
            repeat_type: StartStop::Start,
            slashes: Some(2),
        };
        assert_eq!(mr.value, Some(2));
        assert_eq!(mr.repeat_type, StartStop::Start);
    }
}

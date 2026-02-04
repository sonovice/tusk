//! MusicXML 4.0 direction types.
//!
//! This module contains types for the `<direction>` element and its children,
//! including dynamics, tempo, pedals, wedges, and other musical directions.

use serde::{Deserialize, Serialize};

use super::data::*;

// ============================================================================
// Direction Element
// ============================================================================

/// A musical direction that is not necessarily attached to a specific note.
///
/// Directions include dynamics, tempo markings, pedal markings, wedges (crescendo/diminuendo),
/// and other performance instructions. Multiple direction-type elements can be combined
/// (e.g., words followed by the start of a dashed line).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Direction {
    /// One or more direction types (required)
    #[serde(rename = "direction-type")]
    pub direction_types: Vec<DirectionType>,

    /// Offset from current position in divisions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Offset>,

    /// Staff number if different from default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<u32>,

    /// Sound/playback information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,

    /// Placement above or below the staff
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Whether this is a directive (performance instruction)
    #[serde(rename = "@directive", skip_serializing_if = "Option::is_none")]
    pub directive: Option<YesNo>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Direction {
    /// Create a new direction with the given direction types.
    pub fn new(direction_types: Vec<DirectionType>) -> Self {
        Self {
            direction_types,
            offset: None,
            staff: None,
            sound: None,
            placement: None,
            directive: None,
            id: None,
        }
    }

    /// Create a direction with a single wedge.
    pub fn wedge(wedge: Wedge) -> Self {
        Self::new(vec![DirectionType {
            content: DirectionTypeContent::Wedge(wedge),
            id: None,
        }])
    }

    /// Create a direction with dynamics.
    pub fn dynamics(dynamics: Vec<DynamicsValue>) -> Self {
        Self::new(vec![DirectionType {
            content: DirectionTypeContent::Dynamics(Dynamics { values: dynamics }),
            id: None,
        }])
    }
}

// ============================================================================
// Direction Type Container
// ============================================================================

/// Container for the type of direction.
///
/// A direction can contain one or more of these types. The direction-type
/// element groups together elements that represent different kinds of directions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionType {
    /// The content of this direction type
    #[serde(rename = "$value")]
    pub content: DirectionTypeContent,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// The actual content of a direction type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirectionTypeContent {
    /// Rehearsal marks (letters, numbers, section names)
    Rehearsal(Vec<Rehearsal>),
    /// Segno sign
    Segno(Vec<Segno>),
    /// Coda sign
    Coda(Vec<Coda>),
    /// Text direction (words)
    Words(Vec<Words>),
    /// Musical symbol using SMuFL glyph
    Symbol(Vec<Symbol>),
    /// Crescendo/diminuendo wedge
    Wedge(Wedge),
    /// Dynamic markings
    Dynamics(Dynamics),
    /// Dashes (for cresc./dim. text)
    Dashes(Dashes),
    /// Bracket line
    Bracket(Bracket),
    /// Piano pedal marks
    Pedal(Pedal),
    /// Metronome/tempo marking
    Metronome(Metronome),
    /// Octave shift (8va, 8vb, 15ma, etc.)
    OctaveShift(OctaveShift),
    /// Harp pedal diagram
    HarpPedals(HarpPedals),
    /// Harp damping mark
    Damp(Damp),
    /// Damp all strings
    DampAll(DampAll),
    /// Eyeglasses symbol (commercial music)
    Eyeglasses(Eyeglasses),
    /// String mute on/off
    StringMute(StringMute),
    /// Scordatura (string tuning changes)
    Scordatura(Scordatura),
    /// Embedded image
    Image(DirectionImage),
    /// Principal voice marking
    PrincipalVoice(PrincipalVoice),
    /// Percussion pictogram
    Percussion(Vec<Percussion>),
    /// Accordion registration diagram
    AccordionRegistration(AccordionRegistration),
    /// Staff division symbol
    StaffDivide(StaffDivide),
    /// Other direction not covered
    OtherDirection(OtherDirection),
}

// ============================================================================
// Text Directions
// ============================================================================

/// Rehearsal mark (letters, numbers, section names).
///
/// The enclosure is square by default if not specified.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rehearsal {
    /// The rehearsal text content
    #[serde(rename = "$value")]
    pub value: String,

    /// Enclosure shape (default: square)
    #[serde(rename = "@enclosure", skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<EnclosureShape>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Rehearsal {
    /// Create a new rehearsal mark.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            enclosure: None,
            font_family: None,
            font_style: None,
            font_size: None,
            font_weight: None,
            color: None,
            halign: None,
            valign: None,
            id: None,
        }
    }
}

/// Text direction (words).
///
/// The enclosure is none by default if not specified.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Words {
    /// The text content
    #[serde(rename = "$value")]
    pub value: String,

    /// Enclosure shape (default: none)
    #[serde(rename = "@enclosure", skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<EnclosureShape>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Text justification
    #[serde(rename = "@justify", skip_serializing_if = "Option::is_none")]
    pub justify: Option<LeftCenterRight>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Words {
    /// Create new words direction.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            enclosure: None,
            font_family: None,
            font_style: None,
            font_size: None,
            font_weight: None,
            color: None,
            halign: None,
            valign: None,
            justify: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            id: None,
        }
    }
}

/// Musical symbol using SMuFL glyph name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    /// The SMuFL glyph name
    #[serde(rename = "$value")]
    pub value: String,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Symbol {
    /// Create a new symbol.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            font_family: None,
            font_size: None,
            color: None,
            halign: None,
            valign: None,
            id: None,
        }
    }
}

// ============================================================================
// Signs (Segno, Coda)
// ============================================================================

/// Segno sign for navigation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Segno {
    /// SMuFL glyph name
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Coda sign for navigation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Coda {
    /// SMuFL glyph name
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Wedge (Crescendo/Diminuendo)
// ============================================================================

/// Wedge type for crescendo/diminuendo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WedgeType {
    /// Crescendo (closed at left, open at right)
    Crescendo,
    /// Diminuendo (open at left, closed at right)
    Diminuendo,
    /// Stop the wedge
    Stop,
    /// Continue across system break
    Continue,
}

/// Crescendo or diminuendo wedge.
///
/// Spread is measured in tenths of staff line space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wedge {
    /// Type of wedge (required)
    #[serde(rename = "@type")]
    pub wedge_type: WedgeType,

    /// Number level for distinguishing overlapping wedges
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Spread in tenths
    #[serde(rename = "@spread", skip_serializing_if = "Option::is_none")]
    pub spread: Option<f64>,

    /// Circle at point indicating crescendo from nothing or diminuendo to nothing
    #[serde(rename = "@niente", skip_serializing_if = "Option::is_none")]
    pub niente: Option<YesNo>,

    /// Line type (solid, dashed, dotted, wavy)
    #[serde(rename = "@line-type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<LineType>,

    /// Dash length in tenths
    #[serde(rename = "@dash-length", skip_serializing_if = "Option::is_none")]
    pub dash_length: Option<f64>,

    /// Space length in tenths
    #[serde(rename = "@space-length", skip_serializing_if = "Option::is_none")]
    pub space_length: Option<f64>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Wedge {
    /// Create a new wedge of the given type.
    pub fn new(wedge_type: WedgeType) -> Self {
        Self {
            wedge_type,
            number: None,
            spread: None,
            niente: None,
            line_type: None,
            dash_length: None,
            space_length: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            color: None,
            id: None,
        }
    }

    /// Create a crescendo wedge.
    pub fn crescendo() -> Self {
        Self::new(WedgeType::Crescendo)
    }

    /// Create a diminuendo wedge.
    pub fn diminuendo() -> Self {
        Self::new(WedgeType::Diminuendo)
    }

    /// Create a stop wedge.
    pub fn stop() -> Self {
        Self::new(WedgeType::Stop)
    }
}

// ============================================================================
// Dynamics
// ============================================================================

/// Dynamic marking container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dynamics {
    /// Dynamic values (ppp, pp, p, mp, mf, f, ff, fff, etc.)
    #[serde(rename = "$value")]
    pub values: Vec<DynamicsValue>,
}

/// Individual dynamic marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DynamicsValue {
    /// Pianississimo (ppp)
    Ppp,
    /// Pianissimo (pp)
    Pp,
    /// Piano (p)
    P,
    /// Mezzo-piano (mp)
    Mp,
    /// Mezzo-forte (mf)
    Mf,
    /// Forte (f)
    F,
    /// Fortissimo (ff)
    Ff,
    /// Fortississimo (fff)
    Fff,
    /// Forte-piano (fp)
    Fp,
    /// Sforzando (sf)
    Sf,
    /// Sforzando-forte (sfz)
    Sfz,
    /// Sforzando-piano (sfp)
    Sfp,
    /// Sforzando-pianissimo (sfpp)
    Sfpp,
    /// Sforzando-fortissimo (sffz)
    Sffz,
    /// Sforzando-forte-piano (sfzp) - MusicXML 4.0
    Sfzp,
    /// Rinforzando (rf)
    Rf,
    /// Rinforzando-forte (rfz)
    Rfz,
    /// Fortepiano (fz)
    Fz,
    /// Niente (n)
    N,
    /// Pianissississimo (pppp) - very rare
    Pppp,
    /// Fortissississimo (ffff) - very rare
    Ffff,
    /// Pianississississimo (ppppp) - very rare
    Ppppp,
    /// Fortississississimo (fffff) - very rare
    Fffff,
    /// Pianissississississimo (pppppp) - very rare
    Pppppp,
    /// Fortississississimo (ffffff) - very rare
    Ffffff,
    /// Other dynamics not in the standard list
    #[serde(rename = "other-dynamics")]
    OtherDynamics(String),
}

// ============================================================================
// Dashes and Brackets
// ============================================================================

/// Dashes used with cresc. and dim. text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dashes {
    /// Start, stop, or continue
    #[serde(rename = "@type")]
    pub dash_type: StartStopContinue,

    /// Number level for distinguishing overlapping dashes
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Dash length in tenths
    #[serde(rename = "@dash-length", skip_serializing_if = "Option::is_none")]
    pub dash_length: Option<f64>,

    /// Space length in tenths
    #[serde(rename = "@space-length", skip_serializing_if = "Option::is_none")]
    pub space_length: Option<f64>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Dashes {
    /// Create new dashes.
    pub fn new(dash_type: StartStopContinue) -> Self {
        Self {
            dash_type,
            number: None,
            dash_length: None,
            space_length: None,
            default_x: None,
            default_y: None,
            color: None,
            id: None,
        }
    }
}

/// Line ending type for brackets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineEnd {
    Up,
    Down,
    Both,
    Arrow,
    None,
}

/// Bracket line used with directions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bracket {
    /// Start, stop, or continue
    #[serde(rename = "@type")]
    pub bracket_type: StartStopContinue,

    /// Number level for distinguishing overlapping brackets
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Line ending type (required)
    #[serde(rename = "@line-end")]
    pub line_end: LineEnd,

    /// End length in tenths (for up/down line-end)
    #[serde(rename = "@end-length", skip_serializing_if = "Option::is_none")]
    pub end_length: Option<f64>,

    /// Line type (solid, dashed, dotted, wavy)
    #[serde(rename = "@line-type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<LineType>,

    /// Dash length in tenths
    #[serde(rename = "@dash-length", skip_serializing_if = "Option::is_none")]
    pub dash_length: Option<f64>,

    /// Space length in tenths
    #[serde(rename = "@space-length", skip_serializing_if = "Option::is_none")]
    pub space_length: Option<f64>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Bracket {
    /// Create a new bracket.
    pub fn new(bracket_type: StartStopContinue, line_end: LineEnd) -> Self {
        Self {
            bracket_type,
            number: None,
            line_end,
            end_length: None,
            line_type: None,
            dash_length: None,
            space_length: None,
            default_x: None,
            default_y: None,
            color: None,
            id: None,
        }
    }
}

// ============================================================================
// Pedal
// ============================================================================

/// Pedal type for piano pedal marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PedalType {
    /// Start of damper pedal
    Start,
    /// Release pedal
    Stop,
    /// Start of sostenuto pedal
    Sostenuto,
    /// Pedal lift and retake (inverted V)
    Change,
    /// Continue across system break
    Continue,
    /// End of pedal line without explicit lift
    Discontinue,
    /// Resume pedal without downstroke symbol
    Resume,
}

/// Piano pedal mark.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pedal {
    /// Type of pedal action
    #[serde(rename = "@type")]
    pub pedal_type: PedalType,

    /// Number level for distinguishing overlapping pedals
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Whether to use pedal lines
    #[serde(rename = "@line", skip_serializing_if = "Option::is_none")]
    pub line: Option<YesNo>,

    /// Whether to use Ped/Sost/* signs
    #[serde(rename = "@sign", skip_serializing_if = "Option::is_none")]
    pub sign: Option<YesNo>,

    /// Whether to use abbreviated P/S signs
    #[serde(rename = "@abbreviated", skip_serializing_if = "Option::is_none")]
    pub abbreviated: Option<YesNo>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Pedal {
    /// Create a new pedal mark.
    pub fn new(pedal_type: PedalType) -> Self {
        Self {
            pedal_type,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            halign: None,
            valign: None,
            color: None,
            id: None,
        }
    }

    /// Create a pedal start.
    pub fn start() -> Self {
        Self::new(PedalType::Start)
    }

    /// Create a pedal stop.
    pub fn stop() -> Self {
        Self::new(PedalType::Stop)
    }
}

// ============================================================================
// Metronome
// ============================================================================

/// Metronome/tempo marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metronome {
    /// Metronome content (beat-unit based or metric modulation)
    #[serde(flatten)]
    pub content: MetronomeContent,

    /// Whether to display parentheses
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Whether to print
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Text justification
    #[serde(rename = "@justify", skip_serializing_if = "Option::is_none")]
    pub justify: Option<LeftCenterRight>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Content of a metronome marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetronomeContent {
    /// Standard beat-unit = per-minute format
    BeatUnit {
        /// The beat unit (e.g., "quarter", "eighth")
        #[serde(rename = "beat-unit")]
        beat_unit: String,
        /// Dots on the beat unit
        #[serde(
            rename = "beat-unit-dot",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        beat_unit_dots: Vec<()>,
        /// Per-minute value (number or text)
        #[serde(rename = "per-minute")]
        per_minute: String,
    },
    /// Beat-unit = beat-unit format (metric modulation)
    BeatUnitEquivalent(MetricModulation),
}

/// Metric modulation (beat-unit = beat-unit).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricModulation {
    /// The first beat unit
    #[serde(rename = "beat-unit")]
    pub beat_unit_1: String,
    /// Dots on the first beat unit
    #[serde(
        rename = "beat-unit-dot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_dots_1: Vec<()>,
    /// The second beat unit
    #[serde(rename = "beat-unit-2")]
    pub beat_unit_2: String,
    /// Dots on the second beat unit
    #[serde(
        rename = "beat-unit-dot-2",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_dots_2: Vec<()>,
}

impl Metronome {
    /// Create a simple metronome marking (e.g., quarter = 120).
    pub fn simple(beat_unit: impl Into<String>, per_minute: u32) -> Self {
        Self {
            content: MetronomeContent::BeatUnit {
                beat_unit: beat_unit.into(),
                beat_unit_dots: Vec::new(),
                per_minute: per_minute.to_string(),
            },
            parentheses: None,
            print_object: None,
            justify: None,
            default_x: None,
            default_y: None,
            halign: None,
            valign: None,
            id: None,
        }
    }
}

// ============================================================================
// Octave Shift
// ============================================================================

/// Octave shift type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OctaveShiftType {
    /// Shift up (8va, 15ma)
    Up,
    /// Shift down (8vb, 15mb)
    Down,
    /// Stop the shift
    Stop,
    /// Continue across system break
    Continue,
}

/// Octave shift (8va, 8vb, 15ma, 15mb, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OctaveShift {
    /// Type of shift
    #[serde(rename = "@type")]
    pub shift_type: OctaveShiftType,

    /// Number level for distinguishing overlapping shifts
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Size of shift (8 = one octave, 15 = two octaves)
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u8>,

    /// Dash length in tenths
    #[serde(rename = "@dash-length", skip_serializing_if = "Option::is_none")]
    pub dash_length: Option<f64>,

    /// Space length in tenths
    #[serde(rename = "@space-length", skip_serializing_if = "Option::is_none")]
    pub space_length: Option<f64>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl OctaveShift {
    /// Create a new octave shift.
    pub fn new(shift_type: OctaveShiftType) -> Self {
        Self {
            shift_type,
            number: None,
            size: Some(8), // Default is 8 (one octave)
            dash_length: None,
            space_length: None,
            default_x: None,
            default_y: None,
            font_family: None,
            font_size: None,
            color: None,
            id: None,
        }
    }

    /// Create an 8va (up one octave).
    pub fn ottava_alta() -> Self {
        Self::new(OctaveShiftType::Down) // "down" means notated an octave lower than sounding
    }

    /// Create an 8vb (down one octave).
    pub fn ottava_bassa() -> Self {
        Self::new(OctaveShiftType::Up) // "up" means notated an octave higher than sounding
    }
}

// ============================================================================
// Harp Pedals
// ============================================================================

/// Harp pedal diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarpPedals {
    /// Pedal tunings
    #[serde(rename = "pedal-tuning")]
    pub pedal_tunings: Vec<PedalTuning>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Individual pedal tuning in a harp pedal diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PedalTuning {
    /// The step (D, C, B, E, F, G, A)
    #[serde(rename = "pedal-step")]
    pub pedal_step: String,

    /// The alteration (-1, 0, 1)
    #[serde(rename = "pedal-alter")]
    pub pedal_alter: f64,
}

// ============================================================================
// Simple Direction Types
// ============================================================================

/// Harp damping mark.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Damp {
    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Damp all strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DampAll {
    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Eyeglasses symbol (common in commercial music).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Eyeglasses {
    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// String mute type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StringMuteType {
    On,
    Off,
}

/// String mute on/off symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringMute {
    /// On or off
    #[serde(rename = "@type")]
    pub mute_type: StringMuteType,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Scordatura
// ============================================================================

/// Scordatura (alternative string tuning).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scordatura {
    /// Accord elements for each string
    #[serde(rename = "accord")]
    pub accords: Vec<Accord>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Individual string tuning in scordatura.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accord {
    /// String number
    #[serde(rename = "@string")]
    pub string: u32,

    /// Tuning step (C, D, E, F, G, A, B)
    #[serde(rename = "tuning-step")]
    pub tuning_step: String,

    /// Tuning alteration
    #[serde(rename = "tuning-alter", skip_serializing_if = "Option::is_none")]
    pub tuning_alter: Option<f64>,

    /// Tuning octave
    #[serde(rename = "tuning-octave")]
    pub tuning_octave: u8,
}

// ============================================================================
// Image
// ============================================================================

/// Embedded image in direction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionImage {
    /// Image source URL
    #[serde(rename = "@source")]
    pub source: String,

    /// MIME type
    #[serde(rename = "@type")]
    pub image_type: String,

    /// Height in tenths
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// Width in tenths
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment for image
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<ValignImage>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Principal Voice
// ============================================================================

/// Principal voice symbol type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrincipalVoiceSymbol {
    /// Hauptstimme (main voice)
    Hauptstimme,
    /// Nebenstimme (secondary voice)
    Nebenstimme,
    /// Plain square bracket
    #[serde(rename = "plain")]
    Plain,
    /// No symbol (analysis only)
    #[serde(rename = "none")]
    None,
}

/// Principal voice marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrincipalVoice {
    /// Analysis text content
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Start or stop
    #[serde(rename = "@type")]
    pub voice_type: StartStop,

    /// Symbol type
    #[serde(rename = "@symbol")]
    pub symbol: PrincipalVoiceSymbol,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Percussion
// ============================================================================

/// Percussion pictogram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percussion {
    /// The percussion type
    #[serde(rename = "$value")]
    pub content: PercussionContent,

    /// Enclosure shape
    #[serde(rename = "@enclosure", skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<EnclosureShape>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Content of a percussion element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PercussionContent {
    /// Glass percussion (glass harmonica, glass harp, wind chimes)
    Glass(String),
    /// Metal percussion (cowbell, triangle, etc.)
    Metal(String),
    /// Wood percussion (wood block, claves, etc.)
    Wood(String),
    /// Pitched percussion (marimba, vibraphone, etc.)
    Pitched(String),
    /// Membrane percussion (drums)
    Membrane(String),
    /// Effect sounds (thunder sheet, etc.)
    Effect(String),
    /// Timpani
    Timpani,
    /// Beater pictogram
    Beater(Beater),
    /// Stick pictogram
    Stick(Stick),
    /// Stick location
    StickLocation(String),
    /// Other percussion
    OtherPercussion(String),
}

/// Beater pictogram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beater {
    /// Beater type
    #[serde(rename = "$value")]
    pub value: String,

    /// Tip direction
    #[serde(rename = "@tip", skip_serializing_if = "Option::is_none")]
    pub tip: Option<TipDirection>,
}

/// Stick pictogram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stick {
    /// Stick type
    #[serde(rename = "stick-type")]
    pub stick_type: String,

    /// Stick material
    #[serde(rename = "stick-material")]
    pub stick_material: String,

    /// Tip direction
    #[serde(rename = "@tip", skip_serializing_if = "Option::is_none")]
    pub tip: Option<TipDirection>,

    /// Show parentheses
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Show dashed circle
    #[serde(rename = "@dashed-circle", skip_serializing_if = "Option::is_none")]
    pub dashed_circle: Option<YesNo>,
}

/// Tip direction for beaters/sticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TipDirection {
    Up,
    Down,
    Left,
    Right,
    Northwest,
    Northeast,
    Southeast,
    Southwest,
}

// ============================================================================
// Accordion Registration
// ============================================================================

/// Accordion registration diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccordionRegistration {
    /// High section dot present
    #[serde(rename = "accordion-high", skip_serializing_if = "Option::is_none")]
    pub accordion_high: Option<()>,

    /// Middle section (1, 2, or 3 dots)
    #[serde(rename = "accordion-middle", skip_serializing_if = "Option::is_none")]
    pub accordion_middle: Option<u8>,

    /// Low section dot present
    #[serde(rename = "accordion-low", skip_serializing_if = "Option::is_none")]
    pub accordion_low: Option<()>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Staff Divide
// ============================================================================

/// Staff divide type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StaffDivideType {
    Down,
    Up,
    UpDown,
}

/// Staff division symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffDivide {
    /// Division type
    #[serde(rename = "@type")]
    pub divide_type: StaffDivideType,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Other Direction
// ============================================================================

/// Other direction type for extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherDirection {
    /// Text content
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Whether to print
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// SMuFL glyph name
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Offset
// ============================================================================

/// Offset from current position in divisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    /// Offset value in divisions
    #[serde(rename = "$value")]
    pub value: f64,

    /// Whether the offset affects sound/playback
    #[serde(rename = "@sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<YesNo>,
}

impl Offset {
    /// Create a new offset.
    pub fn new(value: f64) -> Self {
        Self { value, sound: None }
    }
}

// ============================================================================
// Sound
// ============================================================================

/// Sound/playback information.
///
/// This is a simplified version; full implementation would include
/// MIDI device changes, swing, and other playback parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Sound {
    /// Tempo in quarter notes per minute
    #[serde(rename = "@tempo", skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f64>,

    /// Dynamics (percentage of default forte = 90)
    #[serde(rename = "@dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f64>,

    /// Da capo
    #[serde(rename = "@dacapo", skip_serializing_if = "Option::is_none")]
    pub dacapo: Option<YesNo>,

    /// Segno target
    #[serde(rename = "@segno", skip_serializing_if = "Option::is_none")]
    pub segno: Option<String>,

    /// Dal segno target
    #[serde(rename = "@dalsegno", skip_serializing_if = "Option::is_none")]
    pub dalsegno: Option<String>,

    /// Coda target
    #[serde(rename = "@coda", skip_serializing_if = "Option::is_none")]
    pub coda: Option<String>,

    /// To coda target
    #[serde(rename = "@tocoda", skip_serializing_if = "Option::is_none")]
    pub tocoda: Option<String>,

    /// Fine
    #[serde(rename = "@fine", skip_serializing_if = "Option::is_none")]
    pub fine: Option<String>,

    /// Forward repeat
    #[serde(rename = "@forward-repeat", skip_serializing_if = "Option::is_none")]
    pub forward_repeat: Option<YesNo>,

    /// Pizzicato
    #[serde(rename = "@pizzicato", skip_serializing_if = "Option::is_none")]
    pub pizzicato: Option<YesNo>,

    /// Damper pedal (yes/no or percentage)
    #[serde(rename = "@damper-pedal", skip_serializing_if = "Option::is_none")]
    pub damper_pedal: Option<String>,

    /// Soft pedal (yes/no or percentage)
    #[serde(rename = "@soft-pedal", skip_serializing_if = "Option::is_none")]
    pub soft_pedal: Option<String>,

    /// Sostenuto pedal (yes/no or percentage)
    #[serde(rename = "@sostenuto-pedal", skip_serializing_if = "Option::is_none")]
    pub sostenuto_pedal: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Sound {
    /// Create a sound element with tempo.
    pub fn with_tempo(tempo: f64) -> Self {
        Self {
            tempo: Some(tempo),
            ..Default::default()
        }
    }

    /// Create a sound element with dynamics.
    pub fn with_dynamics(dynamics: f64) -> Self {
        Self {
            dynamics: Some(dynamics),
            ..Default::default()
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Wedge Tests
    // ========================================================================

    #[test]
    fn test_wedge_creation() {
        let wedge = Wedge::crescendo();
        assert_eq!(wedge.wedge_type, WedgeType::Crescendo);
        assert!(wedge.number.is_none());
        assert!(wedge.spread.is_none());
        assert!(wedge.niente.is_none());
    }

    #[test]
    fn test_wedge_diminuendo() {
        let wedge = Wedge::diminuendo();
        assert_eq!(wedge.wedge_type, WedgeType::Diminuendo);
    }

    #[test]
    fn test_wedge_stop() {
        let wedge = Wedge::stop();
        assert_eq!(wedge.wedge_type, WedgeType::Stop);
    }

    #[test]
    fn test_wedge_with_attributes() {
        let mut wedge = Wedge::crescendo();
        wedge.number = Some(1);
        wedge.spread = Some(15.0);
        wedge.niente = Some(YesNo::Yes);
        wedge.line_type = Some(LineType::Dashed);

        assert_eq!(wedge.number, Some(1));
        assert_eq!(wedge.spread, Some(15.0));
        assert_eq!(wedge.niente, Some(YesNo::Yes));
        assert_eq!(wedge.line_type, Some(LineType::Dashed));
    }

    // ========================================================================
    // Dynamics Tests
    // ========================================================================

    #[test]
    fn test_dynamics_values() {
        let dynamics = Dynamics {
            values: vec![DynamicsValue::Mf],
        };
        assert_eq!(dynamics.values.len(), 1);
    }

    #[test]
    fn test_dynamics_multiple_values() {
        let dynamics = Dynamics {
            values: vec![DynamicsValue::Sf, DynamicsValue::P],
        };
        assert_eq!(dynamics.values.len(), 2);
    }

    #[test]
    fn test_dynamics_other() {
        let dynamics = Dynamics {
            values: vec![DynamicsValue::OtherDynamics("sfffz".to_string())],
        };
        if let DynamicsValue::OtherDynamics(s) = &dynamics.values[0] {
            assert_eq!(s, "sfffz");
        } else {
            panic!("Expected OtherDynamics");
        }
    }

    // ========================================================================
    // Pedal Tests
    // ========================================================================

    #[test]
    fn test_pedal_start() {
        let pedal = Pedal::start();
        assert_eq!(pedal.pedal_type, PedalType::Start);
    }

    #[test]
    fn test_pedal_stop() {
        let pedal = Pedal::stop();
        assert_eq!(pedal.pedal_type, PedalType::Stop);
    }

    #[test]
    fn test_pedal_with_line() {
        let mut pedal = Pedal::start();
        pedal.line = Some(YesNo::Yes);
        pedal.sign = Some(YesNo::No);
        assert_eq!(pedal.line, Some(YesNo::Yes));
        assert_eq!(pedal.sign, Some(YesNo::No));
    }

    #[test]
    fn test_pedal_types() {
        assert_eq!(format!("{:?}", PedalType::Sostenuto), "Sostenuto");
        assert_eq!(format!("{:?}", PedalType::Change), "Change");
        assert_eq!(format!("{:?}", PedalType::Continue), "Continue");
        assert_eq!(format!("{:?}", PedalType::Discontinue), "Discontinue");
        assert_eq!(format!("{:?}", PedalType::Resume), "Resume");
    }

    // ========================================================================
    // Metronome Tests
    // ========================================================================

    #[test]
    fn test_metronome_simple() {
        let metronome = Metronome::simple("quarter", 120);
        if let MetronomeContent::BeatUnit {
            beat_unit,
            per_minute,
            ..
        } = &metronome.content
        {
            assert_eq!(beat_unit, "quarter");
            assert_eq!(per_minute, "120");
        } else {
            panic!("Expected BeatUnit content");
        }
    }

    #[test]
    fn test_metronome_with_parentheses() {
        let mut metronome = Metronome::simple("half", 60);
        metronome.parentheses = Some(YesNo::Yes);
        assert_eq!(metronome.parentheses, Some(YesNo::Yes));
    }

    // ========================================================================
    // Octave Shift Tests
    // ========================================================================

    #[test]
    fn test_octave_shift() {
        let shift = OctaveShift::new(OctaveShiftType::Up);
        assert_eq!(shift.shift_type, OctaveShiftType::Up);
        assert_eq!(shift.size, Some(8));
    }

    #[test]
    fn test_octave_shift_15ma() {
        let mut shift = OctaveShift::new(OctaveShiftType::Down);
        shift.size = Some(15);
        assert_eq!(shift.size, Some(15));
    }

    // ========================================================================
    // Dashes and Bracket Tests
    // ========================================================================

    #[test]
    fn test_dashes() {
        let dashes = Dashes::new(StartStopContinue::Start);
        assert_eq!(dashes.dash_type, StartStopContinue::Start);
    }

    #[test]
    fn test_bracket() {
        let bracket = Bracket::new(StartStopContinue::Start, LineEnd::Up);
        assert_eq!(bracket.bracket_type, StartStopContinue::Start);
        assert_eq!(bracket.line_end, LineEnd::Up);
    }

    #[test]
    fn test_line_end_values() {
        assert_eq!(format!("{:?}", LineEnd::Down), "Down");
        assert_eq!(format!("{:?}", LineEnd::Both), "Both");
        assert_eq!(format!("{:?}", LineEnd::Arrow), "Arrow");
        assert_eq!(format!("{:?}", LineEnd::None), "None");
    }

    // ========================================================================
    // Words and Rehearsal Tests
    // ========================================================================

    #[test]
    fn test_words() {
        let words = Words::new("dolce");
        assert_eq!(words.value, "dolce");
        assert!(words.enclosure.is_none());
    }

    #[test]
    fn test_rehearsal() {
        let rehearsal = Rehearsal::new("A");
        assert_eq!(rehearsal.value, "A");
    }

    #[test]
    fn test_rehearsal_with_enclosure() {
        let mut rehearsal = Rehearsal::new("1");
        rehearsal.enclosure = Some(EnclosureShape::Circle);
        assert_eq!(rehearsal.enclosure, Some(EnclosureShape::Circle));
    }

    // ========================================================================
    // Direction Tests
    // ========================================================================

    #[test]
    fn test_direction_with_wedge() {
        let direction = Direction::wedge(Wedge::crescendo());
        assert_eq!(direction.direction_types.len(), 1);
    }

    #[test]
    fn test_direction_with_dynamics() {
        let direction = Direction::dynamics(vec![DynamicsValue::F]);
        assert_eq!(direction.direction_types.len(), 1);
    }

    #[test]
    fn test_direction_with_placement() {
        let mut direction = Direction::dynamics(vec![DynamicsValue::P]);
        direction.placement = Some(AboveBelow::Below);
        assert_eq!(direction.placement, Some(AboveBelow::Below));
    }

    // ========================================================================
    // Offset and Sound Tests
    // ========================================================================

    #[test]
    fn test_offset() {
        let offset = Offset::new(4.0);
        assert_eq!(offset.value, 4.0);
        assert!(offset.sound.is_none());
    }

    #[test]
    fn test_sound_with_tempo() {
        let sound = Sound::with_tempo(120.0);
        assert_eq!(sound.tempo, Some(120.0));
    }

    #[test]
    fn test_sound_with_dynamics() {
        let sound = Sound::with_dynamics(80.0);
        assert_eq!(sound.dynamics, Some(80.0));
    }

    // ========================================================================
    // Harp Pedals Tests
    // ========================================================================

    #[test]
    fn test_harp_pedals() {
        let pedals = HarpPedals {
            pedal_tunings: vec![
                PedalTuning {
                    pedal_step: "D".to_string(),
                    pedal_alter: 0.0,
                },
                PedalTuning {
                    pedal_step: "C".to_string(),
                    pedal_alter: 1.0,
                },
            ],
            default_x: None,
            default_y: None,
            halign: None,
            valign: None,
            id: None,
        };
        assert_eq!(pedals.pedal_tunings.len(), 2);
        assert_eq!(pedals.pedal_tunings[0].pedal_step, "D");
        assert_eq!(pedals.pedal_tunings[1].pedal_alter, 1.0);
    }

    // ========================================================================
    // Scordatura Tests
    // ========================================================================

    #[test]
    fn test_scordatura() {
        let scordatura = Scordatura {
            accords: vec![Accord {
                string: 6,
                tuning_step: "D".to_string(),
                tuning_alter: None,
                tuning_octave: 2,
            }],
            id: None,
        };
        assert_eq!(scordatura.accords.len(), 1);
        assert_eq!(scordatura.accords[0].string, 6);
        assert_eq!(scordatura.accords[0].tuning_step, "D");
    }

    // ========================================================================
    // String Mute Tests
    // ========================================================================

    #[test]
    fn test_string_mute() {
        let mute = StringMute {
            mute_type: StringMuteType::On,
            default_x: None,
            default_y: None,
            halign: None,
            valign: None,
            id: None,
        };
        assert_eq!(mute.mute_type, StringMuteType::On);
    }

    // ========================================================================
    // Accordion Registration Tests
    // ========================================================================

    #[test]
    fn test_accordion_registration() {
        let reg = AccordionRegistration {
            accordion_high: Some(()),
            accordion_middle: Some(2),
            accordion_low: None,
            ..Default::default()
        };
        assert!(reg.accordion_high.is_some());
        assert_eq!(reg.accordion_middle, Some(2));
        assert!(reg.accordion_low.is_none());
    }

    // ========================================================================
    // Staff Divide Tests
    // ========================================================================

    #[test]
    fn test_staff_divide() {
        let divide = StaffDivide {
            divide_type: StaffDivideType::UpDown,
            default_x: None,
            default_y: None,
            halign: None,
            valign: None,
            id: None,
        };
        assert_eq!(divide.divide_type, StaffDivideType::UpDown);
    }

    // ========================================================================
    // Symbol Tests
    // ========================================================================

    #[test]
    fn test_symbol() {
        let symbol = Symbol::new("segno");
        assert_eq!(symbol.value, "segno");
    }

    // ========================================================================
    // Segno and Coda Tests
    // ========================================================================

    #[test]
    fn test_segno() {
        let segno = Segno::default();
        assert!(segno.smufl.is_none());
    }

    #[test]
    fn test_coda() {
        let coda = Coda::default();
        assert!(coda.smufl.is_none());
    }
}

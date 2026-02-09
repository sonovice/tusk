//! MusicXML 4.0 harmony types.
//!
//! This module contains types for the `<harmony>` element and its children,
//! including chord symbols, Roman numeral analysis, and fretboard diagrams.

use serde::{Deserialize, Serialize};

use super::data::{AboveBelow, LeftCenterRight, LeftRight, StartStop, Step, Valign, YesNo};
use super::direction::Offset;

// ============================================================================
// Harmony Element
// ============================================================================

/// A harmony element indicating a chord, Roman numeral, or function symbol.
///
/// Contains one or more harmony-chord groups (root/kind/bass/degree),
/// an optional frame (fretboard diagram), and optional offset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Harmony {
    /// One or more harmony-chord groups (root+kind or numeral+kind or function+kind)
    pub chords: Vec<HarmonyChord>,

    /// Fretboard diagram
    pub frame: Option<Frame>,

    /// Offset from current position in divisions
    pub offset: Option<Offset>,

    /// Staff number
    pub staff: Option<u32>,

    /// Harmony type (explicit/implied/alternate)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub harmony_type: Option<HarmonyType>,

    /// Whether to print this object
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Whether to print the frame
    #[serde(rename = "@print-frame", skip_serializing_if = "Option::is_none")]
    pub print_frame: Option<YesNo>,

    /// Arrangement of root and function
    #[serde(rename = "@arrangement", skip_serializing_if = "Option::is_none")]
    pub arrangement: Option<HarmonyArrangement>,

    /// Placement above or below
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,

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

// ============================================================================
// Harmony Type and Arrangement enums
// ============================================================================

/// Type of harmony analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonyType {
    Explicit,
    Implied,
    Alternate,
}

/// Arrangement of root/function and kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonyArrangement {
    Vertical,
    Horizontal,
    Diagonal,
}

// ============================================================================
// Harmony Chord Group
// ============================================================================

/// A chord within a harmony element.
///
/// Each chord has a root (or numeral or function), a kind, and optional
/// inversion, bass, and degrees.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmonyChord {
    /// Root, numeral, or function (the chord identifier)
    pub root_type: HarmonyChordRoot,

    /// Kind of chord (major, minor, dominant, etc.)
    pub kind: Kind,

    /// Inversion number
    pub inversion: Option<Inversion>,

    /// Bass note
    pub bass: Option<Bass>,

    /// Degree modifications (add, alter, subtract)
    pub degrees: Vec<Degree>,
}

/// The root identifier of a chord — root pitch, Roman numeral, or function text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HarmonyChordRoot {
    /// Root pitch (e.g., C, D#, Bb)
    Root(Root),
    /// Roman numeral analysis (e.g., IV, V7)
    Numeral(Numeral),
    /// Function text (deprecated in MusicXML 4.0)
    Function(StyleText),
}

// ============================================================================
// Root
// ============================================================================

/// Root of a chord.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    /// The root step (A-G)
    pub root_step: RootStep,
    /// Optional chromatic alteration
    pub root_alter: Option<HarmonyAlter>,
}

/// Root step with optional display text and print-style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootStep {
    /// The step value (A-G)
    pub value: Step,
    /// Override display text
    pub text: Option<String>,
}

/// Chromatic alteration in semitones with optional display attributes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmonyAlter {
    /// Alteration in semitones (e.g., -1 = flat, 1 = sharp)
    pub value: f64,
    /// Whether to print this alteration
    pub print_object: Option<YesNo>,
    /// Location relative to root/bass (left or right)
    pub location: Option<LeftRight>,
}

// ============================================================================
// Bass
// ============================================================================

/// Bass note of a chord (slash chord notation).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bass {
    /// Optional separator text (e.g., "on", "/")
    pub bass_separator: Option<StyleText>,
    /// The bass step (A-G)
    pub bass_step: BassStep,
    /// Optional chromatic alteration
    pub bass_alter: Option<HarmonyAlter>,
    /// Arrangement of bass relative to root
    pub arrangement: Option<HarmonyArrangement>,
}

/// Bass step with optional display text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BassStep {
    /// The step value (A-G)
    pub value: Step,
    /// Override display text
    pub text: Option<String>,
}

// ============================================================================
// Kind
// ============================================================================

/// The kind (quality) of a chord.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kind {
    /// The kind value
    pub value: KindValue,
    /// Override display text
    pub text: Option<String>,
    /// Use standard chord symbols (triangle, minus, etc.)
    pub use_symbols: Option<YesNo>,
    /// Stack degree alterations
    pub stack_degrees: Option<YesNo>,
    /// Parenthesize degree alterations
    pub parentheses_degrees: Option<YesNo>,
    /// Bracket degree alterations
    pub bracket_degrees: Option<YesNo>,
    /// Horizontal alignment
    pub halign: Option<LeftCenterRight>,
    /// Vertical alignment
    pub valign: Option<Valign>,
}

/// All possible chord kind values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KindValue {
    // Triads
    Major,
    Minor,
    Augmented,
    Diminished,
    // Sevenths
    Dominant,
    MajorSeventh,
    MinorSeventh,
    DiminishedSeventh,
    AugmentedSeventh,
    HalfDiminished,
    MajorMinor,
    // Sixths
    MajorSixth,
    MinorSixth,
    // Ninths
    DominantNinth,
    MajorNinth,
    MinorNinth,
    // 11ths
    Dominant11th,
    Major11th,
    Minor11th,
    // 13ths
    Dominant13th,
    Major13th,
    Minor13th,
    // Suspended
    SuspendedSecond,
    SuspendedFourth,
    // Functional sixths
    Neapolitan,
    Italian,
    French,
    German,
    // Other
    Pedal,
    Power,
    Tristan,
    Other,
    None,
}

impl KindValue {
    /// Parse from MusicXML string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "major" => Some(KindValue::Major),
            "minor" => Some(KindValue::Minor),
            "augmented" => Some(KindValue::Augmented),
            "diminished" => Some(KindValue::Diminished),
            "dominant" => Some(KindValue::Dominant),
            "major-seventh" => Some(KindValue::MajorSeventh),
            "minor-seventh" => Some(KindValue::MinorSeventh),
            "diminished-seventh" => Some(KindValue::DiminishedSeventh),
            "augmented-seventh" => Some(KindValue::AugmentedSeventh),
            "half-diminished" => Some(KindValue::HalfDiminished),
            "major-minor" => Some(KindValue::MajorMinor),
            "major-sixth" => Some(KindValue::MajorSixth),
            "minor-sixth" => Some(KindValue::MinorSixth),
            "dominant-ninth" => Some(KindValue::DominantNinth),
            "major-ninth" => Some(KindValue::MajorNinth),
            "minor-ninth" => Some(KindValue::MinorNinth),
            "dominant-11th" => Some(KindValue::Dominant11th),
            "major-11th" => Some(KindValue::Major11th),
            "minor-11th" => Some(KindValue::Minor11th),
            "dominant-13th" => Some(KindValue::Dominant13th),
            "major-13th" => Some(KindValue::Major13th),
            "minor-13th" => Some(KindValue::Minor13th),
            "suspended-second" => Some(KindValue::SuspendedSecond),
            "suspended-fourth" => Some(KindValue::SuspendedFourth),
            "Neapolitan" => Some(KindValue::Neapolitan),
            "Italian" => Some(KindValue::Italian),
            "French" => Some(KindValue::French),
            "German" => Some(KindValue::German),
            "pedal" => Some(KindValue::Pedal),
            "power" => Some(KindValue::Power),
            "Tristan" => Some(KindValue::Tristan),
            "other" => Some(KindValue::Other),
            "none" => Some(KindValue::None),
            _ => Option::None,
        }
    }

    /// Convert to MusicXML string.
    pub fn as_str(&self) -> &'static str {
        match self {
            KindValue::Major => "major",
            KindValue::Minor => "minor",
            KindValue::Augmented => "augmented",
            KindValue::Diminished => "diminished",
            KindValue::Dominant => "dominant",
            KindValue::MajorSeventh => "major-seventh",
            KindValue::MinorSeventh => "minor-seventh",
            KindValue::DiminishedSeventh => "diminished-seventh",
            KindValue::AugmentedSeventh => "augmented-seventh",
            KindValue::HalfDiminished => "half-diminished",
            KindValue::MajorMinor => "major-minor",
            KindValue::MajorSixth => "major-sixth",
            KindValue::MinorSixth => "minor-sixth",
            KindValue::DominantNinth => "dominant-ninth",
            KindValue::MajorNinth => "major-ninth",
            KindValue::MinorNinth => "minor-ninth",
            KindValue::Dominant11th => "dominant-11th",
            KindValue::Major11th => "major-11th",
            KindValue::Minor11th => "minor-11th",
            KindValue::Dominant13th => "dominant-13th",
            KindValue::Major13th => "major-13th",
            KindValue::Minor13th => "minor-13th",
            KindValue::SuspendedSecond => "suspended-second",
            KindValue::SuspendedFourth => "suspended-fourth",
            KindValue::Neapolitan => "Neapolitan",
            KindValue::Italian => "Italian",
            KindValue::French => "French",
            KindValue::German => "German",
            KindValue::Pedal => "pedal",
            KindValue::Power => "power",
            KindValue::Tristan => "Tristan",
            KindValue::Other => "other",
            KindValue::None => "none",
        }
    }
}

// ============================================================================
// Inversion
// ============================================================================

/// Inversion of a chord.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inversion {
    /// Inversion number (0 = root position, 1 = first inversion, etc.)
    pub value: u32,
    /// Override display text
    pub text: Option<String>,
}

// ============================================================================
// Degree
// ============================================================================

/// A degree modification of a chord (add, alter, or subtract).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Degree {
    /// The scale degree number
    pub degree_value: DegreeValue,
    /// Chromatic alteration of the degree
    pub degree_alter: DegreeAlter,
    /// Type of modification
    pub degree_type: DegreeType,
    /// Whether to print this degree
    pub print_object: Option<YesNo>,
}

/// Scale degree value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DegreeValue {
    /// The degree number (1 = root, 3 = third, etc.)
    pub value: u32,
    /// Symbol to use for display
    pub symbol: Option<DegreeSymbolValue>,
    /// Override display text
    pub text: Option<String>,
}

/// Symbols for displaying degree modifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DegreeSymbolValue {
    Major,
    Minor,
    Augmented,
    Diminished,
    HalfDiminished,
}

impl DegreeSymbolValue {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "major" => Some(DegreeSymbolValue::Major),
            "minor" => Some(DegreeSymbolValue::Minor),
            "augmented" => Some(DegreeSymbolValue::Augmented),
            "diminished" => Some(DegreeSymbolValue::Diminished),
            "half-diminished" => Some(DegreeSymbolValue::HalfDiminished),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DegreeSymbolValue::Major => "major",
            DegreeSymbolValue::Minor => "minor",
            DegreeSymbolValue::Augmented => "augmented",
            DegreeSymbolValue::Diminished => "diminished",
            DegreeSymbolValue::HalfDiminished => "half-diminished",
        }
    }
}

/// Chromatic alteration of a degree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DegreeAlter {
    /// Alteration in semitones
    pub value: f64,
    /// Whether to use plus/minus instead of sharp/flat
    pub plus_minus: Option<YesNo>,
}

/// Type of degree modification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DegreeType {
    /// The modification type
    pub value: DegreeTypeValue,
    /// Override display text
    pub text: Option<String>,
}

/// Degree modification type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DegreeTypeValue {
    Add,
    Alter,
    Subtract,
}

impl DegreeTypeValue {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "add" => Some(DegreeTypeValue::Add),
            "alter" => Some(DegreeTypeValue::Alter),
            "subtract" => Some(DegreeTypeValue::Subtract),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DegreeTypeValue::Add => "add",
            DegreeTypeValue::Alter => "alter",
            DegreeTypeValue::Subtract => "subtract",
        }
    }
}

// ============================================================================
// Numeral
// ============================================================================

/// Roman numeral analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Numeral {
    /// The numeral root (1-7)
    pub numeral_root: NumeralRoot,
    /// Optional chromatic alteration
    pub numeral_alter: Option<HarmonyAlter>,
    /// Optional key context
    pub numeral_key: Option<NumeralKey>,
}

/// Roman numeral root value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumeralRoot {
    /// The numeral value (1-7)
    pub value: u32,
    /// Override display text (e.g., "IV", "V")
    pub text: Option<String>,
}

/// Key context for Roman numeral analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumeralKey {
    /// Position on circle of fifths
    pub numeral_fifths: i8,
    /// Mode
    pub numeral_mode: NumeralMode,
    /// Whether to print this key
    pub print_object: Option<YesNo>,
}

/// Mode for numeral key context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NumeralMode {
    Major,
    Minor,
    NaturalMinor,
    MelodicMinor,
    HarmonicMinor,
}

impl NumeralMode {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "major" => Some(NumeralMode::Major),
            "minor" => Some(NumeralMode::Minor),
            "natural minor" => Some(NumeralMode::NaturalMinor),
            "melodic minor" => Some(NumeralMode::MelodicMinor),
            "harmonic minor" => Some(NumeralMode::HarmonicMinor),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            NumeralMode::Major => "major",
            NumeralMode::Minor => "minor",
            NumeralMode::NaturalMinor => "natural minor",
            NumeralMode::MelodicMinor => "melodic minor",
            NumeralMode::HarmonicMinor => "harmonic minor",
        }
    }
}

// ============================================================================
// Frame (Fretboard Diagram)
// ============================================================================

/// A fretboard diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    /// Number of strings
    pub frame_strings: u32,
    /// Number of frets shown
    pub frame_frets: u32,
    /// First fret shown (if not 1)
    pub first_fret: Option<FirstFret>,
    /// Notes on the fretboard
    pub frame_notes: Vec<FrameNote>,
    /// Height in tenths
    pub height: Option<f64>,
    /// Width in tenths
    pub width: Option<f64>,
    /// Default X position
    pub default_x: Option<f64>,
    /// Default Y position
    pub default_y: Option<f64>,
    /// Horizontal alignment
    pub halign: Option<LeftCenterRight>,
    /// Vertical alignment
    pub valign: Option<Valign>,
    /// Unplayed string symbol
    pub unplayed: Option<String>,
    /// Color
    pub color: Option<String>,
    /// Optional unique ID
    pub id: Option<String>,
}

/// First fret indicator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstFret {
    /// The fret number
    pub value: u32,
    /// Display text
    pub text: Option<String>,
    /// Location (left or right of diagram)
    pub location: Option<LeftRight>,
}

/// A note on the fretboard diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameNote {
    /// String number (1 = highest pitched)
    pub string: FrameString,
    /// Fret number (0 = open string)
    pub fret: Fret,
    /// Optional fingering
    pub fingering: Option<FrameFingering>,
    /// Optional barre indication
    pub barre: Option<Barre>,
}

/// String number in a frame note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameString {
    /// String number
    pub value: u32,
    /// Placement
    pub placement: Option<AboveBelow>,
}

/// Fret number in a frame note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fret {
    /// Fret number (0 = open)
    pub value: u32,
}

/// Fingering in a frame note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameFingering {
    /// Fingering text
    pub value: String,
    /// Substitution fingering
    pub substitution: Option<YesNo>,
    /// Alternate fingering
    pub alternate: Option<YesNo>,
}

/// Barre indication in a frame note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Barre {
    /// Start or stop of barre
    pub barre_type: StartStop,
    /// Color
    pub color: Option<String>,
}

// ============================================================================
// Style Text (for function and bass-separator)
// ============================================================================

/// Text with optional font style attributes (used for function and bass-separator).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleText {
    /// The text content
    pub value: String,
    /// Font family
    pub font_family: Option<String>,
    /// Font style (italic/normal)
    pub font_style: Option<String>,
    /// Font size
    pub font_size: Option<f64>,
    /// Font weight (bold/normal)
    pub font_weight: Option<String>,
    /// Color
    pub color: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_value_roundtrip() {
        let values = [
            "major",
            "minor",
            "augmented",
            "diminished",
            "dominant",
            "major-seventh",
            "minor-seventh",
            "diminished-seventh",
            "augmented-seventh",
            "half-diminished",
            "major-minor",
            "major-sixth",
            "minor-sixth",
            "dominant-ninth",
            "major-ninth",
            "minor-ninth",
            "dominant-11th",
            "major-11th",
            "minor-11th",
            "dominant-13th",
            "major-13th",
            "minor-13th",
            "suspended-second",
            "suspended-fourth",
            "Neapolitan",
            "Italian",
            "French",
            "German",
            "pedal",
            "power",
            "Tristan",
            "other",
            "none",
        ];
        for v in &values {
            let parsed = KindValue::from_str(v).unwrap_or_else(|| panic!("failed to parse: {v}"));
            assert_eq!(parsed.as_str(), *v);
        }
    }

    #[test]
    fn test_degree_type_value_roundtrip() {
        for v in &["add", "alter", "subtract"] {
            let parsed =
                DegreeTypeValue::from_str(v).unwrap_or_else(|| panic!("failed to parse: {v}"));
            assert_eq!(parsed.as_str(), *v);
        }
    }

    #[test]
    fn test_numeral_mode_roundtrip() {
        for v in &[
            "major",
            "minor",
            "natural minor",
            "melodic minor",
            "harmonic minor",
        ] {
            let parsed = NumeralMode::from_str(v).unwrap_or_else(|| panic!("failed to parse: {v}"));
            assert_eq!(parsed.as_str(), *v);
        }
    }
}

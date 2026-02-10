//! Clef, key signature, time signature, tempo, and mark types for the LilyPond AST.
//!
//! These correspond to the `\clef`, `\key`, `\time`, `\tempo`, and `\mark` music functions.

use super::duration::Duration;
use super::markup::Markup;
use super::pitch::Pitch;

/// `\clef "name"` — clef specification.
///
/// The clef name is stored as the raw string argument (e.g. `"treble"`,
/// `"bass"`, `"alto"`, `"G_8"`). LilyPond supports many clef names plus
/// octave transposition suffixes (`_8`, `^15`, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct Clef {
    /// The clef name string, e.g. `"treble"`, `"bass"`, `"G_8"`.
    pub name: String,
}

/// Standard clef names recognized by LilyPond.
pub const KNOWN_CLEF_NAMES: &[&str] = &[
    "treble",
    "violin",
    "G",
    "G2",
    "french",
    "GG",
    "tenorG",
    "soprano",
    "mezzosoprano",
    "alto",
    "C",
    "tenor",
    "baritone",
    "varbaritone",
    "bass",
    "F",
    "subbass",
    "percussion",
    "varpercussion",
    "tab",
    // Variant clefs
    "varC",
    "altovarC",
    "tenorvarC",
    "baritonevarC",
    // Mensural
    "mensural-c1",
    "mensural-c2",
    "mensural-c3",
    "mensural-c4",
    "mensural-c5",
    "mensural-f",
    "mensural-g",
    // Petrucci
    "petrucci-c1",
    "petrucci-c2",
    "petrucci-c3",
    "petrucci-c4",
    "petrucci-c5",
    "petrucci-f",
    "petrucci-f2",
    "petrucci-f3",
    "petrucci-f4",
    "petrucci-f5",
    "petrucci-g",
    "petrucci-g1",
    "petrucci-g2",
    // Vaticana
    "vaticana-do1",
    "vaticana-do2",
    "vaticana-do3",
    "vaticana-fa1",
    "vaticana-fa2",
    // Medicaea
    "medicaea-do1",
    "medicaea-do2",
    "medicaea-do3",
    "medicaea-fa1",
    "medicaea-fa2",
    // Hufnagel
    "hufnagel-do1",
    "hufnagel-do2",
    "hufnagel-do3",
    "hufnagel-fa1",
    "hufnagel-fa2",
    "hufnagel-do-fa",
    // Kievan
    "kievan-do",
    // Blackmensural
    "blackmensural-c3",
];

impl Clef {
    /// Returns the base clef name (before any `_` or `^` transposition suffix).
    pub fn base_name(&self) -> &str {
        if let Some(idx) = self.name.find(['_', '^']) {
            &self.name[..idx]
        } else {
            &self.name
        }
    }

    /// Returns `true` if the base clef name is recognized.
    pub fn is_known(&self) -> bool {
        KNOWN_CLEF_NAMES.contains(&self.base_name())
    }
}

/// Key mode — the scale type for `\key pitch \mode`.
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Major,
    Minor,
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    /// Parse a mode name (without backslash).
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "major" => Some(Mode::Major),
            "minor" => Some(Mode::Minor),
            "ionian" => Some(Mode::Ionian),
            "dorian" => Some(Mode::Dorian),
            "phrygian" => Some(Mode::Phrygian),
            "lydian" => Some(Mode::Lydian),
            "mixolydian" => Some(Mode::Mixolydian),
            "aeolian" => Some(Mode::Aeolian),
            "locrian" => Some(Mode::Locrian),
            _ => None,
        }
    }

    /// Serialize as the mode name (without backslash).
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Major => "major",
            Mode::Minor => "minor",
            Mode::Ionian => "ionian",
            Mode::Dorian => "dorian",
            Mode::Phrygian => "phrygian",
            Mode::Lydian => "lydian",
            Mode::Mixolydian => "mixolydian",
            Mode::Aeolian => "aeolian",
            Mode::Locrian => "locrian",
        }
    }
}

/// `\key pitch \mode` — key signature specification.
#[derive(Debug, Clone, PartialEq)]
pub struct KeySignature {
    /// The tonic pitch (e.g. `c`, `bes`, `fis`).
    pub pitch: Pitch,
    /// The mode (e.g. `\major`, `\minor`, `\dorian`).
    pub mode: Mode,
}

/// `\time n/m` — time signature specification.
///
/// Supports simple time signatures (`\time 4/4`) and compound/additive
/// numerators (`\time 3+2/8`).
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSignature {
    /// Numerator components. A simple time signature like `4/4` has
    /// `numerators = [4]`. A compound signature like `3+2/8` has
    /// `numerators = [3, 2]`.
    pub numerators: Vec<u32>,
    /// Denominator (beat unit).
    pub denominator: u32,
}

// ---------------------------------------------------------------------------
// Tempo
// ---------------------------------------------------------------------------

/// `\tempo` — tempo indication.
///
/// Supports three forms from the grammar:
/// - `\tempo 4 = 120` — metronome mark only
/// - `\tempo "Allegro" 4 = 120` — text + metronome mark
/// - `\tempo "Allegro"` — text only (markup or string)
///
/// The BPM can be a single value or a range (e.g. `120-132`).
#[derive(Debug, Clone, PartialEq)]
pub struct Tempo {
    /// Optional text label (string or markup), e.g. `"Allegro"`.
    pub text: Option<Markup>,
    /// Optional beat duration for metronome mark (e.g. quarter note = `4`).
    pub duration: Option<Duration>,
    /// Optional BPM value or range.
    pub bpm: Option<TempoRange>,
}

/// BPM specification for `\tempo`.
#[derive(Debug, Clone, PartialEq)]
pub enum TempoRange {
    /// Single BPM value, e.g. `120`.
    Single(u32),
    /// BPM range, e.g. `120-132`.
    Range(u32, u32),
}

// ---------------------------------------------------------------------------
// Mark
// ---------------------------------------------------------------------------

/// The label argument to `\mark`.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkLabel {
    /// `\mark \default` — auto-incremented mark.
    Default,
    /// `\mark N` — numeric mark (integer argument).
    Number(u32),
    /// `\mark "A"` or `\mark \markup { ... }` — string/markup label.
    Markup(Markup),
}

/// `\mark` — rehearsal or ad-hoc mark.
///
/// Grammar forms:
/// - `\mark \default` — auto-incremented rehearsal mark
/// - `\mark N` — mark with specific number
/// - `\mark "text"` or `\mark \markup { ... }` — mark with text/markup
#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub label: MarkLabel,
}

/// `\textMark` — text mark (LilyPond 2.24+).
///
/// Places a text mark at the current position.
/// - `\textMark "text"` or `\textMark \markup { ... }`
#[derive(Debug, Clone, PartialEq)]
pub struct TextMark {
    pub text: Markup,
}

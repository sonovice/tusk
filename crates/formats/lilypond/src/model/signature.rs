//! Clef, key signature, and time signature types for the LilyPond AST.
//!
//! These correspond to the `\clef`, `\key`, and `\time` music functions.

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

//! Note, rest, and skip event types for LilyPond AST.
//!
//! These mirror the `simple_element` and `pitch_or_music` productions
//! in the grammar.

use super::duration::Duration;
use super::pitch::Pitch;

/// Direction placement for articulations, dynamics, and other post-events.
///
/// Mirrors the `script_dir` production in the grammar:
/// `^` = Up, `_` = Down, `-` = Neutral (default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// `^` — place above
    Up,
    /// `_` — place below
    Down,
    /// `-` — default/neutral placement
    Neutral,
}

/// Script abbreviation characters and their articulation names.
///
/// Mirrors `script_abbreviation` in the grammar and `script-init.ly` mappings:
/// `.` = staccato, `-` = tenuto, `>` = accent, `^` = marcato,
/// `+` = stopped, `!` = staccatissimo, `_` = portato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptAbbreviation {
    /// `.` → staccato (`dashDot`)
    Dot,
    /// `-` → tenuto (`dashDash`)
    Dash,
    /// `>` → accent (`dashLarger`)
    Accent,
    /// `^` → marcato (`dashHat`)
    Marcato,
    /// `+` → stopped (`dashPlus`)
    Stopped,
    /// `!` → staccatissimo (`dashBang`)
    Staccatissimo,
    /// `_` → portato (`dashUnderscore`)
    Portato,
}

impl ScriptAbbreviation {
    /// Convert a character to a script abbreviation, if valid.
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '.' => Some(Self::Dot),
            '-' => Some(Self::Dash),
            '>' => Some(Self::Accent),
            '^' => Some(Self::Marcato),
            '+' => Some(Self::Stopped),
            '!' => Some(Self::Staccatissimo),
            '_' => Some(Self::Portato),
            _ => None,
        }
    }

    /// The character used in LilyPond source for this abbreviation.
    pub fn as_char(self) -> char {
        match self {
            Self::Dot => '.',
            Self::Dash => '-',
            Self::Accent => '>',
            Self::Marcato => '^',
            Self::Stopped => '+',
            Self::Staccatissimo => '!',
            Self::Portato => '_',
        }
    }

    /// The LilyPond articulation command name (without backslash).
    pub fn articulation_name(self) -> &'static str {
        match self {
            Self::Dot => "staccato",
            Self::Dash => "tenuto",
            Self::Accent => "accent",
            Self::Marcato => "marcato",
            Self::Stopped => "stopped",
            Self::Staccatissimo => "staccatissimo",
            Self::Portato => "portato",
        }
    }
}

/// A post-event attached to a note, chord, rest, or skip.
///
/// Mirrors the `post_event` production in the grammar. Post-events appear
/// after the duration and include ties, slurs, phrasing slurs, beaming,
/// dynamics, hairpins, articulations, and fingerings.
#[derive(Debug, Clone, PartialEq)]
pub enum PostEvent {
    /// Tie: `~`
    Tie,
    /// Slur start: `(`
    SlurStart,
    /// Slur end: `)`
    SlurEnd,
    /// Phrasing slur start: `\(`
    PhrasingSlurStart,
    /// Phrasing slur end: `\)`
    PhrasingSlurEnd,
    /// Beam start: `[`
    BeamStart,
    /// Beam end: `]`
    BeamEnd,
    /// Crescendo hairpin start: `\<`
    Crescendo,
    /// Decrescendo hairpin start: `\>`
    Decrescendo,
    /// Hairpin end: `\!`
    HairpinEnd,
    /// Absolute dynamic marking: `\p`, `\ff`, `\sfz`, etc.
    Dynamic(String),
    /// Script abbreviation with optional direction: `-. ^> _-` etc.
    Articulation {
        direction: Direction,
        script: ScriptAbbreviation,
    },
    /// Fingering with optional direction: `-1`, `^3`, `_2`, etc.
    Fingering { direction: Direction, digit: u8 },
    /// Named articulation with direction: `-\staccato`, `^\accent`, etc.
    NamedArticulation { direction: Direction, name: String },
    /// String number with direction: `\N` (where N is a digit).
    StringNumber { direction: Direction, number: u8 },
    /// Single-note tremolo: `:N` (e.g. `:32`, `:16`).
    ///
    /// The value is the subdivision (8, 16, 32, 64, etc.).
    /// `:` alone (no number) means "default" tremolo, stored as 0.
    Tremolo(u32),
    /// Lyric hyphen: `--` (syllable continuation).
    LyricHyphen,
    /// Lyric extender: `__` (melisma/note hold).
    LyricExtender,
}

/// Known LilyPond dynamic marking names (from `dynamic-scripts-init.ly`).
pub const KNOWN_DYNAMICS: &[&str] = &[
    "ppppp", "pppp", "ppp", "pp", "p", "mp", "mf", "f", "ff", "fff", "ffff", "fffff", "fp", "sf",
    "sfp", "sff", "sfz", "fz", "sp", "spp", "rfz", "n",
];

/// Returns `true` if the given name is a known LilyPond dynamic marking.
pub fn is_dynamic_marking(name: &str) -> bool {
    KNOWN_DYNAMICS.contains(&name)
}

/// Known LilyPond ornament / script names that can appear as direction-less
/// post-events (e.g. `c4\trill` without a `-`/`^`/`_` prefix).
///
/// From `scm/script.scm` and the grammar's `direction_less_event` production.
pub const KNOWN_ORNAMENTS: &[&str] = &[
    // Trills
    "trill",
    // Mordents
    "mordent",
    "prall",
    "prallprall",
    "prallmordent",
    "upprall",
    "downprall",
    "upmordent",
    "downmordent",
    "pralldown",
    "prallup",
    "lineprall",
    // Turns
    "turn",
    "reverseturn",
    // Fermatas
    "fermata",
    "shortfermata",
    "longfermata",
    "verylongfermata",
    // Bowing
    "upbow",
    "downbow",
    // Harmonics
    "flageolet",
    "open",
    "harmonic",
    // Articulation scripts
    "espressivo",
    "staccatissimo",
    "staccato",
    "tenuto",
    "portato",
    "marcato",
    "accent",
    "stopped",
    "snappizzicato",
    // Segno/Coda
    "segno",
    "coda",
    "varcoda",
];

/// Returns `true` if the given name is a known LilyPond ornament/script name
/// that can appear as a direction-less post-event.
pub fn is_ornament_or_script(name: &str) -> bool {
    KNOWN_ORNAMENTS.contains(&name)
}

/// A note event: pitch + optional duration + post-events.
///
/// Corresponds to the `pitch_or_music` production when it produces a
/// NoteEvent (not a RestEvent or chord).
#[derive(Debug, Clone, PartialEq)]
pub struct NoteEvent {
    pub pitch: Pitch,
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// `\rest` after a pitched note makes it a pitched rest.
    pub pitched_rest: bool,
    /// Post-events (ties, slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A chord event: `< pitch1 pitch2 ... > duration post_events`.
///
/// Corresponds to the `note_chord_element` production in the grammar:
/// `chord_body optional_notemode_duration post_events`.
///
/// Each element in the chord body is a pitch with optional accidental markers;
/// the duration is shared across all pitches.
#[derive(Debug, Clone, PartialEq)]
pub struct ChordEvent {
    /// Pitches in the chord body (at least one).
    pub pitches: Vec<Pitch>,
    /// Shared duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (ties, slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A rest event (`r` with optional duration + post-events).
///
/// Corresponds to `simple_element` with RESTNAME="r".
#[derive(Debug, Clone, PartialEq)]
pub struct RestEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A skip event (`s` with optional duration + post-events).
///
/// Corresponds to `simple_element` with RESTNAME="s".
#[derive(Debug, Clone, PartialEq)]
pub struct SkipEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A multi-measure rest event (`R` with optional duration + post-events).
///
/// Corresponds to `MULTI_MEASURE_REST` in the grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiMeasureRestEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A lyric event: a syllable with optional duration and post-events.
///
/// In lyric mode, words/strings become lyric elements with optional duration,
/// hyphen (`--`), and extender (`__`) post-events.
#[derive(Debug, Clone, PartialEq)]
pub struct LyricEvent {
    /// The syllable text (word or string).
    pub text: String,
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (hyphens, extenders, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A chord repetition event (`q` with optional duration + post-events).
///
/// Repeats the pitches of the most recent chord. Corresponds to
/// `EVENT_CHORD → note_chord_element` when the element is `q`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChordRepetitionEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (ties, slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

// ---------------------------------------------------------------------------
// Chord mode types
// ---------------------------------------------------------------------------

/// A chord-mode event: `root[:quality][/inversion][/+bass]`.
///
/// Corresponds to the `new_chord` production in chord mode. The root pitch
/// is combined with quality modifiers, step additions/removals, inversion,
/// and optional bass note to fully specify a chord symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct ChordModeEvent {
    /// Root pitch of the chord (e.g. `c`, `fis`).
    pub root: Pitch,
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Quality items after `:` — modifiers and step numbers.
    pub quality: Vec<ChordQualityItem>,
    /// Removals after `^` — step numbers to omit.
    pub removals: Vec<ChordStep>,
    /// Inversion pitch after `/` (chord inversion).
    pub inversion: Option<Pitch>,
    /// Added bass note after `/+`.
    pub bass: Option<Pitch>,
    /// Post-events (ties, slurs, etc.) attached after the chord.
    pub post_events: Vec<PostEvent>,
}

/// An item in the chord quality chain after `:`.
#[derive(Debug, Clone, PartialEq)]
pub enum ChordQualityItem {
    /// A named modifier: `m`, `min`, `aug`, `dim`, `maj`, `sus`.
    Modifier(ChordModifier),
    /// A step number with optional alteration: `7`, `9+`, `5-`.
    Step(ChordStep),
}

/// A step number with optional alteration.
#[derive(Debug, Clone, PartialEq)]
pub struct ChordStep {
    /// Step number (1–13 typically).
    pub number: u8,
    /// Alteration of the step.
    pub alteration: StepAlteration,
}

/// Named chord quality modifiers.
///
/// From LilyPond's `chordmodifiers` table (see `chord-entry.scm`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordModifier {
    /// `m` or `min` — minor
    Minor,
    /// `aug` — augmented
    Augmented,
    /// `dim` — diminished
    Diminished,
    /// `maj` — major (typically `maj7`)
    Major,
    /// `sus` — suspended
    Suspended,
}

impl ChordModifier {
    /// Parse a modifier from its LilyPond name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "m" | "min" => Some(Self::Minor),
            "aug" => Some(Self::Augmented),
            "dim" => Some(Self::Diminished),
            "maj" => Some(Self::Major),
            "sus" => Some(Self::Suspended),
            _ => None,
        }
    }

    /// The canonical LilyPond name for this modifier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Minor => "m",
            Self::Augmented => "aug",
            Self::Diminished => "dim",
            Self::Major => "maj",
            Self::Suspended => "sus",
        }
    }

    /// All known chord modifier names.
    pub const KNOWN_NAMES: &[&str] = &["m", "min", "aug", "dim", "maj", "sus"];
}

/// Alteration on a chord step number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAlteration {
    /// No alteration (natural).
    Natural,
    /// `+` — raised (sharp).
    Sharp,
    /// `-` — lowered (flat).
    Flat,
}

impl StepAlteration {
    /// The LilyPond suffix character for this alteration.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Natural => "",
            Self::Sharp => "+",
            Self::Flat => "-",
        }
    }
}

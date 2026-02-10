//! LilyPond AST types mirroring the grammar in `specs/lilypond/repo/lily/parser.yy`.
//!
//! Types are added incrementally per phase (score, music, note, pitch, duration, etc.).

pub mod duration;
pub mod note;
pub mod pitch;
pub mod signature;

pub use duration::Duration;
pub use note::{
    ChordEvent, Direction, KNOWN_DYNAMICS, KNOWN_ORNAMENTS, MultiMeasureRestEvent, NoteEvent,
    PostEvent, RestEvent, ScriptAbbreviation, SkipEvent,
};
pub use pitch::Pitch;
pub use signature::{Clef, KeySignature, Mode, TimeSignature};

// Re-export repeat types (defined in this file)
// RepeatType is already available as model::RepeatType

// ---------------------------------------------------------------------------
// Top-level file
// ---------------------------------------------------------------------------

/// Root AST node for a `.ly` file.
///
/// Mirrors the `lilypond` production: a sequence of `toplevel_expression` and
/// `assignment` items, preceded by an optional `\version`.
#[derive(Debug, Clone, PartialEq)]
pub struct LilyPondFile {
    pub version: Option<Version>,
    pub items: Vec<ToplevelExpression>,
}

/// `\version "2.24.0"` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Version {
    pub version: String,
}

// ---------------------------------------------------------------------------
// Top-level expressions
// ---------------------------------------------------------------------------

/// A single top-level item in a `.ly` file.
///
/// Mirrors the `toplevel_expression` production in the grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum ToplevelExpression {
    Score(ScoreBlock),
    Book(BookBlock),
    BookPart(BookPartBlock),
    Header(HeaderBlock),
    /// Top-level assignment: `name = expr`.
    Assignment(Assignment),
    /// Standalone music at the top level (e.g. `\relative { c d e f }`).
    Music(Music),
}

// ---------------------------------------------------------------------------
// Score
// ---------------------------------------------------------------------------

/// `\score { ... }` block.
///
/// Contains a body of score items (music, header, layout, midi, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreBlock {
    pub items: Vec<ScoreItem>,
}

/// An item inside a `\score { ... }` body.
///
/// Mirrors `score_items` from the grammar: the body collects music, output
/// definitions (layout, midi), and an optional header.
#[derive(Debug, Clone, PartialEq)]
pub enum ScoreItem {
    Music(Music),
    Header(HeaderBlock),
    Layout(LayoutBlock),
    Midi(MidiBlock),
}

// ---------------------------------------------------------------------------
// Book / BookPart
// ---------------------------------------------------------------------------

/// `\book { ... }` block.
#[derive(Debug, Clone, PartialEq)]
pub struct BookBlock {
    pub items: Vec<BookItem>,
}

/// An item inside a `\book { ... }` body.
#[derive(Debug, Clone, PartialEq)]
pub enum BookItem {
    Score(ScoreBlock),
    BookPart(BookPartBlock),
    Header(HeaderBlock),
    Paper(PaperBlock),
    Music(Music),
    Assignment(Assignment),
}

/// `\bookpart { ... }` block.
#[derive(Debug, Clone, PartialEq)]
pub struct BookPartBlock {
    pub items: Vec<BookPartItem>,
}

/// An item inside a `\bookpart { ... }` body.
#[derive(Debug, Clone, PartialEq)]
pub enum BookPartItem {
    Score(ScoreBlock),
    Header(HeaderBlock),
    Paper(PaperBlock),
    Music(Music),
    Assignment(Assignment),
}

// ---------------------------------------------------------------------------
// Header
// ---------------------------------------------------------------------------

/// `\header { ... }` block containing key–value assignments.
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderBlock {
    pub fields: Vec<Assignment>,
}

// ---------------------------------------------------------------------------
// Output definitions (stubs — content parsed in later phases)
// ---------------------------------------------------------------------------

/// `\layout { ... }` block (body opaque for now).
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBlock {
    pub body: Vec<LayoutItem>,
}

/// Item inside `\layout { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutItem {
    Assignment(Assignment),
    ContextBlock(ContextModBlock),
}

/// `\midi { ... }` block (body opaque for now).
#[derive(Debug, Clone, PartialEq)]
pub struct MidiBlock {
    pub body: Vec<Assignment>,
}

/// `\paper { ... }` block (body opaque for now).
#[derive(Debug, Clone, PartialEq)]
pub struct PaperBlock {
    pub body: Vec<Assignment>,
}

/// `\context { ... }` block inside `\layout`.
#[derive(Debug, Clone, PartialEq)]
pub struct ContextModBlock {
    pub items: Vec<ContextModItem>,
}

/// Item inside a `\context { ... }` block.
#[derive(Debug, Clone, PartialEq)]
pub enum ContextModItem {
    /// `\ContextName` (e.g. `\Score`, `\Staff`).
    ContextRef(String),
    /// `\consists "Engraver_name"` or `\consists EngraverName`.
    Consists(String),
    /// `\remove "Engraver_name"` or `\remove EngraverName`.
    Remove(String),
    Assignment(Assignment),
}

// ---------------------------------------------------------------------------
// Assignments
// ---------------------------------------------------------------------------

/// `name = value` assignment.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: AssignmentValue,
}

/// Right-hand side of an assignment.
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentValue {
    String(String),
    Number(f64),
    Music(Box<Music>),
    /// A bare identifier reference (e.g. `\melody`).
    Identifier(String),
    /// A Scheme-like expression stored as raw text (for now).
    SchemeExpr(String),
    /// A markup expression stored as raw text (for now).
    Markup(String),
}

// ---------------------------------------------------------------------------
// Music
// ---------------------------------------------------------------------------

/// Repeat type for `\repeat TYPE count { ... }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatType {
    /// `\repeat volta` — folded repeat with volta brackets.
    Volta,
    /// `\repeat unfold` — expanded without repeat notation.
    Unfold,
    /// `\repeat percent` — percent/slash notation.
    Percent,
    /// `\repeat tremolo` — tremolo repeat.
    Tremolo,
    /// `\repeat segno` — segno-based repeat (LilyPond 2.24+).
    Segno,
}

impl RepeatType {
    /// Parse a repeat type from a string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "volta" => Some(Self::Volta),
            "unfold" => Some(Self::Unfold),
            "percent" => Some(Self::Percent),
            "tremolo" => Some(Self::Tremolo),
            "segno" => Some(Self::Segno),
            _ => None,
        }
    }

    /// Return the LilyPond name of this repeat type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Volta => "volta",
            Self::Unfold => "unfold",
            Self::Percent => "percent",
            Self::Tremolo => "tremolo",
            Self::Segno => "segno",
        }
    }
}

/// Whether `\new` or `\context` was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKeyword {
    /// `\new` — creates a fresh context instance.
    New,
    /// `\context` — references an existing (or creates a default) context.
    Context,
}

/// A music expression.
///
/// Variants are expanded as new phases implement more of the grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum Music {
    /// Sequential music: `{ ... }`.
    Sequential(Vec<Music>),
    /// Simultaneous music: `<< ... >>`.
    Simultaneous(Vec<Music>),
    /// `\relative [pitch] { ... }`.
    Relative {
        pitch: Option<Box<Music>>,
        body: Box<Music>,
    },
    /// `\fixed pitch { ... }`.
    Fixed { pitch: Box<Music>, body: Box<Music> },
    /// `\transpose from to { ... }`.
    Transpose {
        from: Box<Music>,
        to: Box<Music>,
        body: Box<Music>,
    },
    /// `\tuplet n/m [duration] { ... }` or `\times n/m { ... }`.
    Tuplet {
        numerator: u32,
        denominator: u32,
        /// Optional tuplet span duration (only for `\tuplet`).
        span_duration: Option<Duration>,
        body: Box<Music>,
    },
    /// `\new ContextType [= "name"] [\with { ... }] music` or
    /// `\context ContextType [= "name"] [\with { ... }] music`.
    ContextedMusic {
        keyword: ContextKeyword,
        context_type: String,
        name: Option<String>,
        with_block: Option<Vec<ContextModItem>>,
        music: Box<Music>,
    },
    /// `\change ContextType = "name"`.
    ContextChange { context_type: String, name: String },
    /// A note event with structured pitch and duration.
    Note(NoteEvent),
    /// A chord event: `< c e g >4`.
    Chord(ChordEvent),
    /// A rest event (`r`).
    Rest(RestEvent),
    /// A skip event (`s`).
    Skip(SkipEvent),
    /// A multi-measure rest (`R`).
    MultiMeasureRest(MultiMeasureRestEvent),
    /// `\clef "treble"` — set the clef.
    Clef(Clef),
    /// `\key pitch \mode` — set the key signature.
    KeySignature(KeySignature),
    /// `\time n/m` — set the time signature.
    TimeSignature(TimeSignature),
    /// `\autoBeamOn` — enable automatic beaming.
    AutoBeamOn,
    /// `\autoBeamOff` — disable automatic beaming.
    AutoBeamOff,
    /// `\grace { ... }` — standard grace notes.
    Grace { body: Box<Music> },
    /// `\acciaccatura { ... }` — acciaccatura grace notes.
    Acciaccatura { body: Box<Music> },
    /// `\appoggiatura { ... }` — appoggiatura grace notes.
    Appoggiatura { body: Box<Music> },
    /// `\afterGrace [fraction] main { grace }` — grace notes after main note.
    AfterGrace {
        /// Optional fraction (e.g. `3/4`), defaults to `3/4` in LilyPond.
        fraction: Option<(u32, u32)>,
        main: Box<Music>,
        grace: Box<Music>,
    },
    /// `\repeat type count { body } [\alternative { ... }]`.
    Repeat {
        repeat_type: RepeatType,
        count: u32,
        body: Box<Music>,
        alternatives: Option<Vec<Music>>,
    },
    /// A note/rest/chord event stored as raw text (for tokens not yet
    /// decomposed into structured types).
    Event(String),
    /// An identifier reference like `\melody`.
    Identifier(String),
    /// Unparsed content inside braces (to be refined in later phases).
    Unparsed(String),
}

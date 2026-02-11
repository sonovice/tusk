//! LilyPond AST types mirroring the grammar in `specs/lilypond/repo/lily/parser.yy`.
//!
//! Types are added incrementally per phase (score, music, note, pitch, duration, etc.).

pub mod duration;
pub mod markup;
pub mod note;
pub mod pitch;
pub mod property;
pub mod scheme;
pub mod signature;

pub use duration::Duration;
pub use markup::{Markup, MarkupList};
pub use note::{
    BassFigure, ChordEvent, ChordModeEvent, ChordModifier, ChordQualityItem, ChordRepetitionEvent,
    ChordStep, Direction, DrumChordEvent, DrumNoteEvent, FigureAlteration, FigureEvent,
    FiguredBassModification, KNOWN_DRUM_PITCHES, KNOWN_DYNAMICS, KNOWN_ORNAMENTS, LyricEvent,
    MultiMeasureRestEvent, NoteEvent, PostEvent, RestEvent, ScriptAbbreviation, SkipEvent,
    StepAlteration,
};
pub use pitch::Pitch;
pub use property::{PropertyPath, PropertyValue};
pub use scheme::SchemeExpr;
pub use signature::{
    Clef, KeySignature, Mark, MarkLabel, Mode, Tempo, TempoRange, TextMark, TimeSignature,
};

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
    /// Top-level `\paper { ... }` block.
    Paper(PaperBlock),
    /// Top-level `\layout { ... }` block.
    Layout(LayoutBlock),
    /// Top-level `\midi { ... }` block.
    Midi(MidiBlock),
    /// Top-level assignment: `name = expr`.
    Assignment(Assignment),
    /// Standalone music at the top level (e.g. `\relative { c d e f }`).
    Music(Music),
    /// Top-level `\markup { ... }`.
    Markup(Markup),
    /// Top-level `\markuplist { ... }`.
    MarkupList(MarkupList),
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

/// `\midi { ... }` block.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiBlock {
    pub body: Vec<MidiItem>,
}

/// Item inside `\midi { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub enum MidiItem {
    Assignment(Assignment),
    ContextBlock(ContextModBlock),
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

/// Item inside a `\context { ... }` or `\with { ... }` block.
#[derive(Debug, Clone, PartialEq)]
pub enum ContextModItem {
    /// `\ContextName` (e.g. `\Score`, `\Staff`).
    ContextRef(String),
    /// `\consists "Engraver_name"` or `\consists EngraverName`.
    Consists(String),
    /// `\remove "Engraver_name"` or `\remove EngraverName`.
    Remove(String),
    Assignment(Assignment),
    /// `\override path = value` inside a context block.
    Override {
        path: PropertyPath,
        value: PropertyValue,
    },
    /// `\revert path` inside a context block.
    Revert {
        path: PropertyPath,
    },
    /// `\set property = value` inside a context block.
    Set {
        path: PropertyPath,
        value: PropertyValue,
    },
    /// `\unset property` inside a context block.
    Unset {
        path: PropertyPath,
    },
    /// `\denies "ContextName"` — deny nested context type.
    Denies(String),
    /// `\accepts "ContextName"` — accept nested context type.
    Accepts(String),
    /// `\alias "ContextName"` — declare context alias.
    Alias(String),
    /// `\defaultchild "ContextName"` — set default child context.
    DefaultChild(String),
    /// `\description "text"` — context description string.
    Description(String),
    /// `\name "ContextName"` — set context name.
    Name(String),
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
    /// An arithmetic expression with optional units (e.g. `180\mm - 2\cm`).
    NumericExpression(NumericExpression),
    Music(Box<Music>),
    /// A bare identifier reference (e.g. `\melody`).
    Identifier(String),
    /// A Scheme expression (e.g. `#red`, `#42`, `##t`).
    SchemeExpr(SchemeExpr),
    /// A structured markup expression.
    Markup(Markup),
    /// A structured markup list expression.
    MarkupList(MarkupList),
}

// ---------------------------------------------------------------------------
// Numeric expressions
// ---------------------------------------------------------------------------

/// An arithmetic expression in output-def contexts (`\paper`, `\layout`).
///
/// Mirrors the `number_expression` / `number_term` / `number_factor` /
/// `bare_number` productions in the LilyPond grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum NumericExpression {
    /// A literal number: `42`, `3.5`.
    Literal(f64),
    /// A number with a unit suffix: `180\mm`, `2.5\cm`.
    WithUnit(f64, String),
    /// Unary negation: `-expr`.
    Negate(Box<NumericExpression>),
    /// Addition: `a + b`.
    Add(Box<NumericExpression>, Box<NumericExpression>),
    /// Subtraction: `a - b`.
    Sub(Box<NumericExpression>, Box<NumericExpression>),
    /// Multiplication: `a * b`.
    Mul(Box<NumericExpression>, Box<NumericExpression>),
    /// Division: `a / b`.
    Div(Box<NumericExpression>, Box<NumericExpression>),
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
    /// A chord repetition (`q`): repeats the pitches of the most recent chord.
    ChordRepetition(ChordRepetitionEvent),
    /// `\clef "treble"` — set the clef.
    Clef(Clef),
    /// `\key pitch \mode` — set the key signature.
    KeySignature(KeySignature),
    /// `\time n/m` — set the time signature.
    TimeSignature(TimeSignature),
    /// `\tempo` — tempo indication.
    Tempo(Tempo),
    /// `\mark` — rehearsal or ad-hoc mark.
    Mark(Mark),
    /// `\textMark` — text mark (LilyPond 2.24+).
    TextMark(TextMark),
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
    /// Bar check: `|` — a timing assertion at measure boundaries.
    BarCheck,
    /// Bar line command: `\bar "type"` — sets explicit bar line style.
    BarLine { bar_type: String },
    /// `\override path = value` — set a grob property.
    Override {
        path: PropertyPath,
        value: PropertyValue,
    },
    /// `\revert path` — revert a grob property to default.
    Revert { path: PropertyPath },
    /// `\set path = value` — set a context property.
    Set {
        path: PropertyPath,
        value: PropertyValue,
    },
    /// `\unset path` — unset a context property.
    Unset { path: PropertyPath },
    /// `\once music` — apply the following property operation once.
    Once { music: Box<Music> },
    /// `\chordmode { ... }` — chord mode music.
    ChordMode { body: Box<Music> },
    /// A chord-mode event: `root[:quality][/inversion][/+bass]`.
    ChordModeEntry(ChordModeEvent),
    /// `\lyricmode { ... }` — lyric mode music.
    LyricMode { body: Box<Music> },
    /// `\addlyrics { ... }` — attach lyrics to preceding music.
    ///
    /// Parsed as: `music \addlyrics lyrics1 [\addlyrics lyrics2 ...]`.
    /// The `music` field is the preceding music, `lyrics` are the lyric blocks.
    AddLyrics {
        music: Box<Music>,
        lyrics: Vec<Music>,
    },
    /// `\lyricsto "voice" { ... }` — attach lyrics to a named voice.
    LyricsTo {
        voice_id: String,
        lyrics: Box<Music>,
    },
    /// `\drummode { ... }` — drum mode music.
    DrumMode { body: Box<Music> },
    /// A drum note event inside drum mode: `drumtype [duration] [post_events]`.
    DrumNote(DrumNoteEvent),
    /// A drum chord event: `< drum1 drum2 ... > duration post_events`.
    DrumChord(DrumChordEvent),
    /// `\figuremode { ... }` — figured bass mode music.
    FigureMode { body: Box<Music> },
    /// A figure event: `\< figures \> duration` inside figure mode.
    Figure(FigureEvent),
    /// A lyric event: a syllable with optional duration and post-events.
    Lyric(LyricEvent),
    /// A `\markup { ... }` expression in a music context.
    Markup(Markup),
    /// A `\markuplist { ... }` expression in a music context.
    MarkupList(MarkupList),
    /// A generic music function call: `\functionName arg1 arg2 ...`.
    ///
    /// Used for user-defined or unrecognized music functions. Built-in
    /// functions like `\grace`, `\tuplet`, `\relative` etc. have their own
    /// dedicated variants above.
    MusicFunction {
        /// Function name (without leading backslash).
        name: String,
        /// Arguments passed to the function.
        args: Vec<FunctionArg>,
    },
    /// A partial function application terminated by `\etc`.
    ///
    /// `\functionName arg1 arg2 \etc` — supplies some arguments now;
    /// remaining arguments are expected later when the result is called.
    PartialFunction {
        /// Function name (without leading backslash).
        name: String,
        /// Arguments supplied so far.
        args: Vec<FunctionArg>,
    },
    /// A Scheme expression in music position (grammar: `music_embedded`).
    ///
    /// Represents `#expr` where the Scheme value is used as a music expression,
    /// e.g. `#(make-music 'NoteEvent ...)` or `#(ly:export ...)`.
    /// We cannot evaluate Scheme, so we store the expression opaquely.
    SchemeMusic(SchemeExpr),
    /// A note/rest/chord event stored as raw text (for tokens not yet
    /// decomposed into structured types).
    Event(String),
    /// An identifier reference like `\melody`.
    Identifier(String),
    /// Unparsed content inside braces (to be refined in later phases).
    Unparsed(String),
}

// ---------------------------------------------------------------------------
// Function arguments
// ---------------------------------------------------------------------------

/// An argument to a music function call.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionArg {
    /// A music expression argument.
    Music(Music),
    /// A string argument.
    String(String),
    /// A numeric argument.
    Number(f64),
    /// A Scheme expression argument (e.g. `#'print`, `#42`).
    SchemeExpr(SchemeExpr),
    /// A duration argument (e.g. `4.` in `\tuplet 3/2 4. { ... }`).
    Duration(Duration),
    /// An identifier reference (e.g. `\varName`).
    Identifier(String),
    /// `\default` — explicit placeholder for an optional argument.
    Default,
    /// A symbol list argument (e.g. `Staff.NoteHead.color`).
    ///
    /// Used by functions like `\keepWithTag`, `\removeWithTag`, etc.
    /// Mirrors the `symbol_list_arg` production in the grammar. Elements are
    /// dot-separated symbols, strings, or integers stored as strings.
    SymbolList(Vec<String>),
}

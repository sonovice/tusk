//! Token definitions for the LilyPond lexer.
//!
//! Token variants mirror the terminal symbols of the LilyPond grammar
//! (LilyPond's `lily/parser.yy`). The lexer produces a stream of
//! these tokens for the parser.

/// Byte offset span in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// A token produced by the lexer, together with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}

/// LilyPond token.
///
/// Variant names follow the LilyPond grammar where possible.
/// Comments and whitespace are skipped by the lexer and not represented here.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ── Literals ──────────────────────────────────────────────────────
    /// Quoted string, e.g. `"Allegro"`. Value has quotes stripped and
    /// escape sequences resolved.
    String(String),

    /// Unsigned integer, e.g. `4`, `120`.
    Unsigned(u64),

    /// Real number with decimal point, e.g. `2.5`. This is the
    /// STRICTREAL case (contains a `.`).
    Real(f64),

    // ── Keywords (backslash commands) ────────────────────────────────
    // Sorted alphabetically. Each maps to the grammar's %token.
    /// `\accepts`
    Accepts,
    /// `\addlyrics`
    AddLyrics,
    /// `\alias`
    Alias,
    /// `\alternative`
    Alternative,
    /// `\book`
    Book,
    /// `\bookpart`
    BookPart,
    /// `\change`
    Change,
    /// `\chordmode`
    ChordMode,
    /// `\chords`
    Chords,
    /// `\consists`
    Consists,
    /// `\context`
    Context,
    /// `\default`
    Default,
    /// `\defaultchild`
    DefaultChild,
    /// `\denies`
    Denies,
    /// `\description`
    Description,
    /// `\drummode`
    DrumMode,
    /// `\drums`
    Drums,
    /// `\etc`
    Etc,
    /// `\figuremode`
    FigureMode,
    /// `\figures`
    Figures,
    /// `\fixed`
    Fixed,
    /// `\header`
    Header,
    /// `\include`
    Include,
    /// `\language`
    Language,
    /// `\layout`
    Layout,
    /// `\lyricmode`
    LyricMode,
    /// `\lyrics`
    Lyrics,
    /// `\lyricsto`
    LyricsTo,
    /// `\markup`
    Markup,
    /// `\markuplist`
    MarkupList,
    /// `\midi`
    Midi,
    /// `\name`
    Name,
    /// `\new`
    New,
    /// `\notemode`
    NoteMode,
    /// `\once`
    Once,
    /// `\override`
    Override,
    /// `\paper`
    Paper,
    /// `\partial`
    Partial,
    /// `\relative`
    Relative,
    /// `\remove`
    Remove,
    /// `\repeat`
    Repeat,
    /// `\rest`
    Rest,
    /// `\revert`
    Revert,
    /// `\score`
    Score,
    /// `\sequential`
    Sequential,
    /// `\set`
    Set,
    /// `\simultaneous`
    Simultaneous,
    /// `\tempo`
    Tempo,
    /// `\time`
    Time,
    /// `\times`
    Times,
    /// `\transpose`
    Transpose,
    /// `\tuplet`
    Tuplet,
    /// `\tweak`
    Tweak,
    /// `\type`
    Type,
    /// `\unset`
    Unset,
    /// `\version`
    Version,
    /// `\with`
    With,

    /// An escaped word that is not a known keyword, e.g. `\major`,
    /// `\italic`, `\bold`, `\fine`, `\caesura`, `\break`.
    /// Stored without the leading backslash.
    EscapedWord(String),

    /// An escaped digit, e.g. `\1`, `\2` — used for string numbers.
    /// Stored as the numeric value (0–9).
    EscapedUnsigned(u64),

    // ── Note names ───────────────────────────────────────────────────
    /// A note name like `c`, `d`, `eis`, `bes`, `fisis`, etc.
    /// Stored as the raw name string (e.g. `"cis"`, `"bes"`).
    NoteName(String),

    // ── Identifiers & symbols ────────────────────────────────────────
    /// An unescaped word that is not a note name, e.g. `volta`, `Staff`.
    /// In some contexts these are variable references or context names.
    Symbol(String),

    // ── Operators & punctuation ──────────────────────────────────────
    /// `{`
    BraceOpen,
    /// `}`
    BraceClose,
    /// `<` — chord or angle bracket open
    AngleOpen,
    /// `>` — chord or angle bracket close
    AngleClose,
    /// `<<` — simultaneous music open
    DoubleAngleOpen,
    /// `>>` — simultaneous music close
    DoubleAngleClose,
    /// `[` — beam start
    BracketOpen,
    /// `]` — beam end
    BracketClose,
    /// `(` — slur start
    ParenOpen,
    /// `)` — slur end
    ParenClose,
    /// `~` — tie
    Tilde,
    /// `|` — bar check
    Pipe,
    /// `=` — assignment or octave check
    Equals,
    /// `.` — dot (duration dot or property access)
    Dot,
    /// `'` — octave up
    Quote,
    /// `,` — octave down
    Comma,
    /// `!` — force accidental (or \\! hairpin end)
    Exclamation,
    /// `?` — cautionary accidental
    Question,
    /// `-` — direction neutral / articulation prefix
    Dash,
    /// `^` — direction up
    Caret,
    /// `_` — direction down
    Underscore,
    /// `+` — augmentation or chord bass
    Plus,
    /// `*` — duration multiplier
    Star,
    /// `/` — chord inversion or fraction
    Slash,
    /// `:`  — chord quality separator or tremolo
    Colon,
    /// `\\` — phrasing slur / backslash in events
    DoubleBackslash,
    /// `#` — Scheme expression prefix
    Hash,

    // ── Compound operators ───────────────────────────────────────────
    /// `\(` — phrasing slur start
    EscapedParenOpen,
    /// `\)` — phrasing slur end
    EscapedParenClose,
    /// `\!` — hairpin end
    EscapedExclamation,
    /// `\+` — figured bass augmentation
    EscapedPlus,
    /// `\<` — figure open or crescendo
    EscapedAngleOpen,
    /// `\>` — figure close or decrescendo
    EscapedAngleClose,

    /// `--` — lyric hyphen
    LyricHyphen,
    /// `__` — lyric extender
    LyricExtender,

    // ── Special ──────────────────────────────────────────────────────
    /// End of file.
    Eof,
}

impl Token {
    /// Returns `true` if this token is a keyword (backslash command).
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::Accepts
                | Token::AddLyrics
                | Token::Alias
                | Token::Alternative
                | Token::Book
                | Token::BookPart
                | Token::Change
                | Token::ChordMode
                | Token::Chords
                | Token::Consists
                | Token::Context
                | Token::Default
                | Token::DefaultChild
                | Token::Denies
                | Token::Description
                | Token::DrumMode
                | Token::Drums
                | Token::Etc
                | Token::FigureMode
                | Token::Figures
                | Token::Fixed
                | Token::Header
                | Token::Include
                | Token::Language
                | Token::Layout
                | Token::LyricMode
                | Token::Lyrics
                | Token::LyricsTo
                | Token::Markup
                | Token::MarkupList
                | Token::Midi
                | Token::Name
                | Token::New
                | Token::NoteMode
                | Token::Once
                | Token::Override
                | Token::Paper
                | Token::Partial
                | Token::Relative
                | Token::Remove
                | Token::Repeat
                | Token::Rest
                | Token::Revert
                | Token::Score
                | Token::Sequential
                | Token::Set
                | Token::Simultaneous
                | Token::Tempo
                | Token::Time
                | Token::Times
                | Token::Transpose
                | Token::Tuplet
                | Token::Tweak
                | Token::Type
                | Token::Unset
                | Token::Version
                | Token::With
        )
    }
}

/// Map a bare word (without backslash) to a keyword token, if it matches.
pub fn keyword_from_str(word: &str) -> Option<Token> {
    match word {
        "accepts" => Some(Token::Accepts),
        "addlyrics" => Some(Token::AddLyrics),
        "alias" => Some(Token::Alias),
        "alternative" => Some(Token::Alternative),
        "book" => Some(Token::Book),
        "bookpart" => Some(Token::BookPart),
        "change" => Some(Token::Change),
        "chordmode" => Some(Token::ChordMode),
        "chords" => Some(Token::Chords),
        "consists" => Some(Token::Consists),
        "context" => Some(Token::Context),
        "default" => Some(Token::Default),
        "defaultchild" => Some(Token::DefaultChild),
        "denies" => Some(Token::Denies),
        "description" => Some(Token::Description),
        "drummode" => Some(Token::DrumMode),
        "drums" => Some(Token::Drums),
        "etc" => Some(Token::Etc),
        "figuremode" => Some(Token::FigureMode),
        "figures" => Some(Token::Figures),
        "fixed" => Some(Token::Fixed),
        "header" => Some(Token::Header),
        "include" => Some(Token::Include),
        "key" => Some(Token::EscapedWord("key".into())),
        "language" => Some(Token::Language),
        "layout" => Some(Token::Layout),
        "lyricmode" => Some(Token::LyricMode),
        "lyrics" => Some(Token::Lyrics),
        "lyricsto" => Some(Token::LyricsTo),
        "markup" => Some(Token::Markup),
        "markuplist" => Some(Token::MarkupList),
        "midi" => Some(Token::Midi),
        "name" => Some(Token::Name),
        "new" => Some(Token::New),
        "notemode" => Some(Token::NoteMode),
        "once" => Some(Token::Once),
        "override" => Some(Token::Override),
        "paper" => Some(Token::Paper),
        "partial" => Some(Token::Partial),
        "relative" => Some(Token::Relative),
        "remove" => Some(Token::Remove),
        "repeat" => Some(Token::Repeat),
        "rest" => Some(Token::Rest),
        "revert" => Some(Token::Revert),
        "score" => Some(Token::Score),
        "sequential" => Some(Token::Sequential),
        "set" => Some(Token::Set),
        "simultaneous" => Some(Token::Simultaneous),
        "tempo" => Some(Token::Tempo),
        "time" => Some(Token::Time),
        "times" => Some(Token::Times),
        "transpose" => Some(Token::Transpose),
        "tuplet" => Some(Token::Tuplet),
        "tweak" => Some(Token::Tweak),
        "type" => Some(Token::Type),
        "unset" => Some(Token::Unset),
        "version" => Some(Token::Version),
        "with" => Some(Token::With),
        _ => None,
    }
}

/// Standard LilyPond (Dutch/default) note names.
///
/// Returns `true` if the given bare word is a standard note name.
/// This covers the default (Dutch) naming convention used by LilyPond:
///   c d e f g a b
///   + optional accidental suffixes: is, es, isis, eses, ih, eh
///     (sharp = is, flat = es, double-sharp = isis, double-flat = eses,
///     quarter-sharp = ih, quarter-flat = eh)
pub fn is_note_name(word: &str) -> bool {
    // Must start with a–g
    let mut chars = word.chars();
    let first = match chars.next() {
        Some(c @ 'a'..='g') => c,
        _ => return false,
    };
    let rest: String = chars.collect();
    if rest.is_empty() {
        return true;
    }

    // Special cases: "as", "es" are valid (A-flat, E-flat in Dutch)
    // "b" alone is B-natural; note names are only a-g
    match (first, rest.as_str()) {
        // Sharp suffixes
        (_, "is") | (_, "isis") => true,
        // Flat suffixes
        (_, "es") | (_, "eses") => true,
        // Quarter-tone suffixes
        (_, "ih") | (_, "isih") | (_, "eh") | (_, "eseh") => true,
        // Special Dutch: "as" = A-flat, "es" already covered above
        ('a', "s") | ('e', "s") => true,
        ('a', "ses") | ('e', "ses") => true,
        _ => false,
    }
}

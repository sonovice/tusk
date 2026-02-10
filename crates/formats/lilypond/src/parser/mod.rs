//! Recursive-descent parser for LilyPond syntax.
//!
//! Builds an AST from token stream produced by the lexer. The parser follows
//! the productions in `specs/lilypond/repo/lily/parser.yy` but implemented as
//! a hand-rolled recursive-descent parser in Rust.

use thiserror::Error;

use crate::lexer::{LexError, Lexer, SpannedToken, Token};
use crate::model::*;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("lexer error: {0}")]
    Lex(#[from] LexError),

    #[error("unexpected token {found:?} at byte offset {offset}, expected {expected}")]
    Unexpected {
        found: Token,
        offset: usize,
        expected: String,
    },

    #[error("unexpected end of input, expected {expected}")]
    UnexpectedEof { expected: String },

    #[error("invalid note name '{name}' at byte offset {offset}")]
    InvalidNoteName { name: String, offset: usize },
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Recursive-descent parser.
///
/// Wraps a [`Lexer`] and produces a [`LilyPondFile`] AST.
pub struct Parser<'src> {
    lexer: Lexer<'src>,
    /// Source text for extracting raw spans.
    src: &'src str,
    /// One-token lookahead.
    current: SpannedToken,
}

impl<'src> Parser<'src> {
    /// Create a parser over the given LilyPond source.
    pub fn new(src: &'src str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(src);
        let current = lexer.next_token()?;
        Ok(Self {
            lexer,
            src,
            current,
        })
    }

    /// Parse the entire file and return the AST.
    pub fn parse(mut self) -> Result<LilyPondFile, ParseError> {
        self.parse_file()
    }

    // ──────────────────────────────────────────────────────────────────
    // Token helpers
    // ──────────────────────────────────────────────────────────────────

    fn peek(&self) -> &Token {
        &self.current.token
    }

    fn offset(&self) -> usize {
        self.current.span.start
    }

    fn advance(&mut self) -> Result<SpannedToken, ParseError> {
        let prev = self.current.clone();
        self.current = self.lexer.next_token()?;
        Ok(prev)
    }

    fn expect(&mut self, expected: &Token) -> Result<SpannedToken, ParseError> {
        if &self.current.token == expected {
            self.advance()
        } else {
            Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: format!("{expected:?}"),
            })
        }
    }

    fn expect_string(&mut self) -> Result<String, ParseError> {
        match &self.current.token {
            Token::String(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::String(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "string literal".into(),
            }),
        }
    }

    fn at_eof(&self) -> bool {
        self.current.token == Token::Eof
    }

    // ──────────────────────────────────────────────────────────────────
    // Top-level: lilypond → version? (toplevel_expression | assignment)*
    // ──────────────────────────────────────────────────────────────────

    fn parse_file(&mut self) -> Result<LilyPondFile, ParseError> {
        // Optional \version
        let version = if *self.peek() == Token::Version {
            Some(self.parse_version()?)
        } else {
            None
        };

        let mut items = Vec::new();
        while !self.at_eof() {
            items.push(self.parse_toplevel_expression()?);
        }

        Ok(LilyPondFile { version, items })
    }

    // ──────────────────────────────────────────────────────────────────
    // \version "..."
    // ──────────────────────────────────────────────────────────────────

    fn parse_version(&mut self) -> Result<Version, ParseError> {
        self.expect(&Token::Version)?;
        let version = self.expect_string()?;
        Ok(Version { version })
    }

    // ──────────────────────────────────────────────────────────────────
    // toplevel_expression
    // ──────────────────────────────────────────────────────────────────

    fn parse_toplevel_expression(&mut self) -> Result<ToplevelExpression, ParseError> {
        match self.peek() {
            Token::Score => Ok(ToplevelExpression::Score(self.parse_score_block()?)),
            Token::Book => Ok(ToplevelExpression::Book(self.parse_book_block()?)),
            Token::BookPart => Ok(ToplevelExpression::BookPart(self.parse_bookpart_block()?)),
            Token::Header => Ok(ToplevelExpression::Header(self.parse_header_block()?)),
            Token::Markup => {
                let m = self.parse_markup()?;
                Ok(ToplevelExpression::Markup(m))
            }
            Token::MarkupList => {
                let ml = self.parse_markuplist()?;
                Ok(ToplevelExpression::MarkupList(ml))
            }
            // Assignment: symbol = ...
            Token::Symbol(_) | Token::NoteName(_) => {
                // Peek ahead for `=` to distinguish assignment from music
                self.parse_assignment_or_music()
            }
            // Music expressions
            _ => {
                let music = self.parse_music()?;
                let music = self.try_wrap_addlyrics(music)?;
                Ok(ToplevelExpression::Music(music))
            }
        }
    }

    fn parse_assignment_or_music(&mut self) -> Result<ToplevelExpression, ParseError> {
        // Save position for backtracking
        let name = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::NoteName(s) => s.clone(),
            _ => return Ok(ToplevelExpression::Music(self.parse_music()?)),
        };

        // We need to look ahead: if next token after the name is `=`, it's an
        // assignment. Otherwise it's music.
        let saved_current = self.current.clone();
        self.advance()?;

        if *self.peek() == Token::Equals {
            // It's an assignment
            self.advance()?; // consume `=`
            let value = self.parse_assignment_value()?;
            Ok(ToplevelExpression::Assignment(Assignment { name, value }))
        } else {
            // Backtrack: restore state and parse as music
            // We can't truly backtrack the lexer, so rebuild from saved position
            self.lexer = Lexer::new(self.src);
            // Re-lex up to the saved position
            self.current = SpannedToken::new(Token::Eof, crate::lexer::Span::new(0, 0));
            let mut new_lexer = Lexer::new(self.src);
            // Fast-forward lexer to saved_current position
            loop {
                let tok = new_lexer.next_token()?;
                if tok.span.start >= saved_current.span.start {
                    self.current = tok;
                    break;
                }
            }
            self.lexer = new_lexer;
            Ok(ToplevelExpression::Music(self.parse_music()?))
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Assignment value
    // ──────────────────────────────────────────────────────────────────

    fn parse_assignment_value(&mut self) -> Result<AssignmentValue, ParseError> {
        match self.peek() {
            Token::String(_) => {
                let s = self.expect_string()?;
                Ok(AssignmentValue::String(s))
            }
            Token::Unsigned(n) => {
                let n = *n as f64;
                self.advance()?;
                Ok(AssignmentValue::Number(n))
            }
            Token::Real(n) => {
                let n = *n;
                self.advance()?;
                Ok(AssignmentValue::Number(n))
            }
            Token::BraceOpen | Token::DoubleAngleOpen => {
                let m = self.parse_music()?;
                Ok(AssignmentValue::Music(Box::new(m)))
            }
            Token::Relative
            | Token::Fixed
            | Token::Transpose
            | Token::Tuplet
            | Token::Times
            | Token::Repeat
            | Token::New
            | Token::Context
            | Token::Change
            | Token::LyricMode
            | Token::Lyrics
            | Token::LyricsTo => {
                let m = self.parse_music()?;
                Ok(AssignmentValue::Music(Box::new(m)))
            }
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(AssignmentValue::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            Token::Markup => {
                let m = self.parse_markup()?;
                Ok(AssignmentValue::Markup(m))
            }
            Token::MarkupList => {
                let ml = self.parse_markuplist()?;
                Ok(AssignmentValue::MarkupList(ml))
            }
            Token::Hash => {
                let raw = self.parse_scheme_raw()?;
                Ok(AssignmentValue::SchemeExpr(raw))
            }
            _ => {
                // Try to parse as music
                let m = self.parse_music()?;
                Ok(AssignmentValue::Music(Box::new(m)))
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \score { score_body }
    // ──────────────────────────────────────────────────────────────────

    fn parse_score_block(&mut self) -> Result<ScoreBlock, ParseError> {
        self.expect(&Token::Score)?;
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_score_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(ScoreBlock { items })
    }

    fn parse_score_item(&mut self) -> Result<ScoreItem, ParseError> {
        match self.peek() {
            Token::Header => Ok(ScoreItem::Header(self.parse_header_block()?)),
            Token::Layout => Ok(ScoreItem::Layout(self.parse_layout_block()?)),
            Token::Midi => Ok(ScoreItem::Midi(self.parse_midi_block()?)),
            _ => {
                let music = self.parse_music()?;
                let music = self.try_wrap_addlyrics(music)?;
                Ok(ScoreItem::Music(music))
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \book { book_body }
    // ──────────────────────────────────────────────────────────────────

    fn parse_book_block(&mut self) -> Result<BookBlock, ParseError> {
        self.expect(&Token::Book)?;
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_book_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(BookBlock { items })
    }

    fn parse_book_item(&mut self) -> Result<BookItem, ParseError> {
        match self.peek() {
            Token::Score => Ok(BookItem::Score(self.parse_score_block()?)),
            Token::BookPart => Ok(BookItem::BookPart(self.parse_bookpart_block()?)),
            Token::Header => Ok(BookItem::Header(self.parse_header_block()?)),
            Token::Paper => Ok(BookItem::Paper(self.parse_paper_block()?)),
            Token::Symbol(_) | Token::NoteName(_) => {
                // Could be assignment or music
                self.parse_book_assignment_or_music()
            }
            _ => Ok(BookItem::Music(self.parse_music()?)),
        }
    }

    fn parse_book_assignment_or_music(&mut self) -> Result<BookItem, ParseError> {
        let name = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::NoteName(s) => s.clone(),
            _ => return Ok(BookItem::Music(self.parse_music()?)),
        };

        let saved_current = self.current.clone();
        self.advance()?;

        if *self.peek() == Token::Equals {
            self.advance()?;
            let value = self.parse_assignment_value()?;
            Ok(BookItem::Assignment(Assignment { name, value }))
        } else {
            // Backtrack
            let mut new_lexer = Lexer::new(self.src);
            loop {
                let tok = new_lexer.next_token()?;
                if tok.span.start >= saved_current.span.start {
                    self.current = tok;
                    break;
                }
            }
            self.lexer = new_lexer;
            Ok(BookItem::Music(self.parse_music()?))
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \bookpart { bookpart_body }
    // ──────────────────────────────────────────────────────────────────

    fn parse_bookpart_block(&mut self) -> Result<BookPartBlock, ParseError> {
        self.expect(&Token::BookPart)?;
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_bookpart_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(BookPartBlock { items })
    }

    fn parse_bookpart_item(&mut self) -> Result<BookPartItem, ParseError> {
        match self.peek() {
            Token::Score => Ok(BookPartItem::Score(self.parse_score_block()?)),
            Token::Header => Ok(BookPartItem::Header(self.parse_header_block()?)),
            Token::Paper => Ok(BookPartItem::Paper(self.parse_paper_block()?)),
            Token::Symbol(_) | Token::NoteName(_) => self.parse_bookpart_assignment_or_music(),
            _ => Ok(BookPartItem::Music(self.parse_music()?)),
        }
    }

    fn parse_bookpart_assignment_or_music(&mut self) -> Result<BookPartItem, ParseError> {
        let name = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::NoteName(s) => s.clone(),
            _ => return Ok(BookPartItem::Music(self.parse_music()?)),
        };

        let saved_current = self.current.clone();
        self.advance()?;

        if *self.peek() == Token::Equals {
            self.advance()?;
            let value = self.parse_assignment_value()?;
            Ok(BookPartItem::Assignment(Assignment { name, value }))
        } else {
            let mut new_lexer = Lexer::new(self.src);
            loop {
                let tok = new_lexer.next_token()?;
                if tok.span.start >= saved_current.span.start {
                    self.current = tok;
                    break;
                }
            }
            self.lexer = new_lexer;
            Ok(BookPartItem::Music(self.parse_music()?))
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \header { ... }
    // ──────────────────────────────────────────────────────────────────

    fn parse_header_block(&mut self) -> Result<HeaderBlock, ParseError> {
        self.expect(&Token::Header)?;
        self.expect(&Token::BraceOpen)?;
        let mut fields = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            fields.push(self.parse_header_field()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(HeaderBlock { fields })
    }

    fn parse_header_field(&mut self) -> Result<Assignment, ParseError> {
        let name = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::NoteName(s) => s.clone(),
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "header field name".into(),
                });
            }
        };
        self.advance()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_header_value()?;
        Ok(Assignment { name, value })
    }

    fn parse_header_value(&mut self) -> Result<AssignmentValue, ParseError> {
        match self.peek() {
            Token::String(_) => {
                let s = self.expect_string()?;
                Ok(AssignmentValue::String(s))
            }
            Token::Unsigned(n) => {
                let n = *n as f64;
                self.advance()?;
                Ok(AssignmentValue::Number(n))
            }
            Token::Real(n) => {
                let n = *n;
                self.advance()?;
                Ok(AssignmentValue::Number(n))
            }
            Token::Markup => {
                let m = self.parse_markup()?;
                Ok(AssignmentValue::Markup(m))
            }
            Token::MarkupList => {
                let ml = self.parse_markuplist()?;
                Ok(AssignmentValue::MarkupList(ml))
            }
            Token::Hash => {
                let raw = self.parse_scheme_raw()?;
                Ok(AssignmentValue::SchemeExpr(raw))
            }
            _ => {
                // Anything else — try as string (some headers have unquoted values)
                let s = self.expect_string()?;
                Ok(AssignmentValue::String(s))
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \layout { ... }
    // ──────────────────────────────────────────────────────────────────

    fn parse_layout_block(&mut self) -> Result<LayoutBlock, ParseError> {
        self.expect(&Token::Layout)?;
        self.expect(&Token::BraceOpen)?;
        let mut body = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            body.push(self.parse_layout_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(LayoutBlock { body })
    }

    fn parse_layout_item(&mut self) -> Result<LayoutItem, ParseError> {
        match self.peek() {
            Token::Context => Ok(LayoutItem::ContextBlock(self.parse_context_mod_block()?)),
            _ => {
                let a = self.parse_output_def_assignment()?;
                Ok(LayoutItem::Assignment(a))
            }
        }
    }

    fn parse_context_mod_block(&mut self) -> Result<ContextModBlock, ParseError> {
        self.expect(&Token::Context)?;
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_context_mod_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(ContextModBlock { items })
    }

    fn parse_context_mod_item(&mut self) -> Result<ContextModItem, ParseError> {
        match self.peek() {
            Token::Consists => {
                self.advance()?;
                let name = self.parse_engraver_name()?;
                Ok(ContextModItem::Consists(name))
            }
            Token::Remove => {
                self.advance()?;
                let name = self.parse_engraver_name()?;
                Ok(ContextModItem::Remove(name))
            }
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(ContextModItem::ContextRef(s)),
                    _ => unreachable!(),
                }
            }
            Token::Symbol(_) | Token::NoteName(_) => {
                let a = self.parse_output_def_assignment()?;
                Ok(ContextModItem::Assignment(a))
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "context modifier (\\consists, \\remove, \\ContextName, or assignment)"
                    .into(),
            }),
        }
    }

    fn parse_engraver_name(&mut self) -> Result<String, ParseError> {
        match &self.current.token {
            Token::String(_) => self.expect_string(),
            Token::Symbol(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Symbol(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "engraver name (string or symbol)".into(),
            }),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \midi { ... }
    // ──────────────────────────────────────────────────────────────────

    fn parse_midi_block(&mut self) -> Result<MidiBlock, ParseError> {
        self.expect(&Token::Midi)?;
        self.expect(&Token::BraceOpen)?;
        let mut body = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            body.push(self.parse_output_def_assignment()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(MidiBlock { body })
    }

    // ──────────────────────────────────────────────────────────────────
    // \paper { ... }
    // ──────────────────────────────────────────────────────────────────

    fn parse_paper_block(&mut self) -> Result<PaperBlock, ParseError> {
        self.expect(&Token::Paper)?;
        self.expect(&Token::BraceOpen)?;
        let mut body = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            body.push(self.parse_output_def_assignment()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(PaperBlock { body })
    }

    // ──────────────────────────────────────────────────────────────────
    // Output-def assignment: name = value
    // ──────────────────────────────────────────────────────────────────

    fn parse_output_def_assignment(&mut self) -> Result<Assignment, ParseError> {
        let name = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::NoteName(s) => s.clone(),
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "assignment name".into(),
                });
            }
        };
        self.advance()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_assignment_value()?;
        Ok(Assignment { name, value })
    }

    // ──────────────────────────────────────────────────────────────────
    // Music expressions
    // ──────────────────────────────────────────────────────────────────

    fn parse_music(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            Token::BraceOpen => self.parse_sequential_music(),
            Token::DoubleAngleOpen => self.parse_simultaneous_music(),
            Token::AngleOpen => self.parse_chord(),
            Token::Sequential => self.parse_explicit_sequential(),
            Token::Simultaneous => self.parse_explicit_simultaneous(),
            Token::Relative => self.parse_relative(),
            Token::Fixed => self.parse_fixed(),
            Token::Transpose => self.parse_transpose(),
            Token::Tuplet => self.parse_tuplet(),
            Token::Times => self.parse_times(),
            Token::Repeat => self.parse_repeat(),
            Token::Alternative => self.parse_alternative_as_music(),
            Token::New | Token::Context => self.parse_context_music(),
            Token::Change => self.parse_context_change(),
            Token::Time => self.parse_time_signature(),
            Token::Tempo => self.parse_tempo(),
            Token::EscapedWord(s) if s == "clef" => self.parse_clef(),
            Token::EscapedWord(s) if s == "key" => self.parse_key_signature(),
            Token::EscapedWord(s) if s == "mark" => self.parse_mark(),
            Token::EscapedWord(s) if s == "textMark" => self.parse_text_mark(),
            Token::EscapedWord(s) if s == "autoBeamOn" => {
                self.advance()?;
                Ok(Music::AutoBeamOn)
            }
            Token::EscapedWord(s) if s == "autoBeamOff" => {
                self.advance()?;
                Ok(Music::AutoBeamOff)
            }
            Token::EscapedWord(s) if s == "grace" => self.parse_grace(),
            Token::EscapedWord(s) if s == "acciaccatura" => self.parse_acciaccatura(),
            Token::EscapedWord(s) if s == "appoggiatura" => self.parse_appoggiatura(),
            Token::EscapedWord(s) if s == "afterGrace" => self.parse_after_grace(),
            Token::Markup => {
                let m = self.parse_markup()?;
                Ok(Music::Markup(m))
            }
            Token::MarkupList => {
                let ml = self.parse_markuplist()?;
                Ok(Music::MarkupList(ml))
            }
            Token::LyricMode => self.parse_lyric_mode(),
            Token::Lyrics => self.parse_lyrics_shorthand(),
            Token::LyricsTo => self.parse_lyricsto(),
            Token::AddLyrics => {
                // Standalone \addlyrics without preceding music — parse error
                Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "music expression before \\addlyrics".into(),
                })
            }
            Token::EscapedWord(s) if s == "bar" => self.parse_bar_line(),
            Token::Pipe => {
                self.advance()?;
                Ok(Music::BarCheck)
            }
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            Token::NoteName(_) => self.parse_note_event(),
            Token::Symbol(s) if s == "r" || s == "s" || s == "R" => self.parse_rest_or_skip(),
            Token::Symbol(s) if s == "q" => self.parse_chord_repetition(),
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "music expression".into(),
            }),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Note event: pitch exclamations questions optional_duration [\rest]
    // ──────────────────────────────────────────────────────────────────

    fn parse_note_event(&mut self) -> Result<Music, ParseError> {
        let offset = self.offset();
        let tok = self.advance()?;
        let note_name = match tok.token {
            Token::NoteName(s) => s,
            _ => unreachable!(),
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        // Parse octave marks (quotes)
        let octave = self.parse_quotes();

        // Parse exclamations and questions (force/cautionary accidentals)
        let force_accidental = self.try_consume(&Token::Exclamation);
        let cautionary = self.try_consume(&Token::Question);

        // Parse octave check: `=` followed by optional octave marks
        let octave_check = if *self.peek() == Token::Equals {
            self.advance()?; // consume `=`
            Some(self.parse_quotes())
        } else {
            None
        };

        // Parse optional duration
        let duration = self.parse_optional_duration()?;

        // Parse optional tremolo `:N`
        let tremolo = self.parse_optional_tremolo();

        // Check for \rest (pitched rest)
        let pitched_rest = self.try_consume(&Token::Rest);

        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }

        Ok(Music::Note(NoteEvent {
            pitch: Pitch {
                step,
                alter,
                octave,
                force_accidental,
                cautionary,
                octave_check,
            },
            duration,
            pitched_rest,
            post_events,
        }))
    }

    // ──────────────────────────────────────────────────────────────────
    // Chord: < pitch1 pitch2 ... > duration post_events
    // ──────────────────────────────────────────────────────────────────

    fn parse_chord(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::AngleOpen)?;
        let mut pitches = Vec::new();
        while *self.peek() != Token::AngleClose && !self.at_eof() {
            pitches.push(self.parse_chord_body_pitch()?);
        }
        self.expect(&Token::AngleClose)?;
        let duration = self.parse_optional_duration()?;
        let tremolo = self.parse_optional_tremolo();
        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }
        Ok(Music::Chord(ChordEvent {
            pitches,
            duration,
            post_events,
        }))
    }

    /// Parse a single pitch element inside a chord body.
    ///
    /// Mirrors `chord_body_element`: pitch with octave marks, accidental
    /// markers (! ?), but no duration (duration is shared on the chord).
    fn parse_chord_body_pitch(&mut self) -> Result<Pitch, ParseError> {
        let offset = self.offset();
        let tok = self.advance()?;
        let note_name = match tok.token {
            Token::NoteName(s) => s,
            other => {
                return Err(ParseError::Unexpected {
                    found: other,
                    offset,
                    expected: "pitch in chord body".into(),
                });
            }
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        let octave = self.parse_quotes();
        let force_accidental = self.try_consume(&Token::Exclamation);
        let cautionary = self.try_consume(&Token::Question);

        // Octave check inside chord body
        let octave_check = if *self.peek() == Token::Equals {
            self.advance()?;
            Some(self.parse_quotes())
        } else {
            None
        };

        Ok(Pitch {
            step,
            alter,
            octave,
            force_accidental,
            cautionary,
            octave_check,
        })
    }

    // ──────────────────────────────────────────────────────────────────
    // Rest (r), skip (s), multi-measure rest (R)
    // ──────────────────────────────────────────────────────────────────

    fn parse_rest_or_skip(&mut self) -> Result<Music, ParseError> {
        let tok = self.advance()?;
        let kind = match tok.token {
            Token::Symbol(s) => s,
            _ => unreachable!(),
        };
        let duration = self.parse_optional_duration()?;
        let tremolo = self.parse_optional_tremolo();
        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }
        match kind.as_str() {
            "r" => Ok(Music::Rest(RestEvent {
                duration,
                post_events,
            })),
            "s" => Ok(Music::Skip(SkipEvent {
                duration,
                post_events,
            })),
            "R" => Ok(Music::MultiMeasureRest(MultiMeasureRestEvent {
                duration,
                post_events,
            })),
            _ => unreachable!(),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Chord repetition: q
    // ──────────────────────────────────────────────────────────────────

    fn parse_chord_repetition(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume `q`
        let duration = self.parse_optional_duration()?;
        let tremolo = self.parse_optional_tremolo();
        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }
        Ok(Music::ChordRepetition(ChordRepetitionEvent {
            duration,
            post_events,
        }))
    }

    // ──────────────────────────────────────────────────────────────────
    // Post-events: tie ~, slur ( ), phrasing slur \( \)
    // ──────────────────────────────────────────────────────────────────

    /// Parse optional tremolo: `:` followed by optional unsigned integer.
    ///
    /// Mirrors the `tremolo_type` production in the grammar. Returns `None`
    /// if no colon is present; returns `Tremolo(0)` for bare `:`.
    fn parse_optional_tremolo(&mut self) -> Option<PostEvent> {
        if *self.peek() != Token::Colon {
            return None;
        }
        let _ = self.advance(); // consume `:`
        if let Token::Unsigned(n) = self.peek() {
            let n = *n as u32;
            let _ = self.advance();
            Some(PostEvent::Tremolo(n))
        } else {
            Some(PostEvent::Tremolo(0))
        }
    }

    fn parse_post_events(&mut self) -> Vec<PostEvent> {
        let mut events = Vec::new();
        loop {
            match self.peek() {
                Token::Tilde => {
                    let _ = self.advance();
                    events.push(PostEvent::Tie);
                }
                Token::ParenOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::SlurStart);
                }
                Token::ParenClose => {
                    let _ = self.advance();
                    events.push(PostEvent::SlurEnd);
                }
                Token::EscapedParenOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::PhrasingSlurStart);
                }
                Token::EscapedParenClose => {
                    let _ = self.advance();
                    events.push(PostEvent::PhrasingSlurEnd);
                }
                Token::BracketOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::BeamStart);
                }
                Token::BracketClose => {
                    let _ = self.advance();
                    events.push(PostEvent::BeamEnd);
                }
                Token::EscapedAngleOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::Crescendo);
                }
                Token::EscapedAngleClose => {
                    let _ = self.advance();
                    events.push(PostEvent::Decrescendo);
                }
                Token::EscapedExclamation => {
                    let _ = self.advance();
                    events.push(PostEvent::HairpinEnd);
                }
                Token::EscapedWord(s) if note::is_dynamic_marking(s) => {
                    let s = s.clone();
                    let _ = self.advance();
                    events.push(PostEvent::Dynamic(s));
                }
                // Undirected ornaments/scripts: \trill, \mordent, \turn, etc.
                Token::EscapedWord(s) if note::is_ornament_or_script(s) => {
                    let s = s.clone();
                    let _ = self.advance();
                    events.push(PostEvent::NamedArticulation {
                        direction: note::Direction::Neutral,
                        name: s,
                    });
                }
                // Undirected string number: \1, \2, etc.
                Token::EscapedUnsigned(n) if *n <= 9 => {
                    let number = *n as u8;
                    let _ = self.advance();
                    events.push(PostEvent::StringNumber {
                        direction: note::Direction::Neutral,
                        number,
                    });
                }
                // Direction prefixes: -, ^, _ followed by script/fingering/articulation
                Token::Dash | Token::Caret | Token::Underscore => {
                    if let Some(ev) = self.try_parse_directed_post_event() {
                        events.push(ev);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        events
    }

    /// Try to parse a direction-prefixed post-event: `-X`, `^X`, `_X`.
    ///
    /// Returns `None` if the token after the direction prefix isn't a valid
    /// post-event (the direction token is NOT consumed in that case).
    fn try_parse_directed_post_event(&mut self) -> Option<PostEvent> {
        let direction = match self.peek() {
            Token::Dash => note::Direction::Neutral,
            Token::Caret => note::Direction::Up,
            Token::Underscore => note::Direction::Down,
            _ => return None,
        };

        // We need lookahead: peek past the direction to see what follows.
        // Save state for potential backtrack.
        let saved = self.current.clone();
        let _ = self.advance(); // consume direction token

        match self.peek() {
            // Script abbreviation: . - > ^ + ! _
            Token::Dot => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Dot,
                })
            }
            Token::Dash => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Dash,
                })
            }
            Token::AngleClose => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Accent,
                })
            }
            Token::Caret => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Marcato,
                })
            }
            Token::Plus => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Stopped,
                })
            }
            Token::Exclamation => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Staccatissimo,
                })
            }
            Token::Underscore => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Portato,
                })
            }
            // Fingering: digit 0-9
            Token::Unsigned(n) if *n <= 9 => {
                let digit = *n as u8;
                let _ = self.advance();
                Some(PostEvent::Fingering { direction, digit })
            }
            // Named articulation: \name (e.g. \staccato, \accent, \trill)
            Token::EscapedWord(name) => {
                let name = name.clone();
                let _ = self.advance();
                Some(PostEvent::NamedArticulation { direction, name })
            }
            // String number: \1, \2, etc.
            Token::EscapedUnsigned(n) if *n <= 9 => {
                let number = *n as u8;
                let _ = self.advance();
                Some(PostEvent::StringNumber { direction, number })
            }
            _ => {
                // Not a valid post-event after direction — backtrack
                self.current = saved;
                None
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Quotes: octave marks (' and ,)
    // ──────────────────────────────────────────────────────────────────

    fn parse_quotes(&mut self) -> i8 {
        let mut octave: i8 = 0;
        loop {
            match self.peek() {
                Token::Quote => {
                    octave = octave.saturating_add(1);
                    let _ = self.advance();
                }
                Token::Comma => {
                    octave = octave.saturating_sub(1);
                    let _ = self.advance();
                }
                _ => break,
            }
        }
        octave
    }

    // ──────────────────────────────────────────────────────────────────
    // Optional duration: UNSIGNED dots multipliers
    // ──────────────────────────────────────────────────────────────────

    fn parse_optional_duration(&mut self) -> Result<Option<Duration>, ParseError> {
        match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                let base = match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                };
                let dots = self.parse_dots();
                let multipliers = self.parse_multipliers()?;
                Ok(Some(Duration {
                    base,
                    dots,
                    multipliers,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Parse dots: zero or more `.` tokens.
    fn parse_dots(&mut self) -> u8 {
        let mut dots: u8 = 0;
        while *self.peek() == Token::Dot {
            dots = dots.saturating_add(1);
            let _ = self.advance();
        }
        dots
    }

    /// Parse multipliers: zero or more `*N` or `*N/M` sequences.
    fn parse_multipliers(&mut self) -> Result<Vec<(u32, u32)>, ParseError> {
        let mut multipliers = Vec::new();
        while *self.peek() == Token::Star {
            let _ = self.advance(); // consume `*`
            if let Token::Unsigned(n) = self.peek() {
                let n = *n as u32;
                let _ = self.advance();
                if *self.peek() == Token::Slash {
                    let _ = self.advance(); // consume `/`
                    if let Token::Unsigned(d) = self.peek() {
                        let d = *d as u32;
                        let _ = self.advance();
                        multipliers.push((n, d));
                    } else {
                        // `*N/` without denominator — treat as `*N/1`
                        multipliers.push((n, 1));
                    }
                } else {
                    multipliers.push((n, 1));
                }
            }
        }
        Ok(multipliers)
    }

    /// Try to consume a specific token, returning true if consumed.
    fn try_consume(&mut self, token: &Token) -> bool {
        if self.peek() == token {
            let _ = self.advance();
            true
        } else {
            false
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Bar line: \bar "type"
    // ──────────────────────────────────────────────────────────────────

    fn parse_bar_line(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \bar
        let bar_type = self.expect_string()?;
        Ok(Music::BarLine { bar_type })
    }

    // ──────────────────────────────────────────────────────────────────
    // Grace notes
    // ──────────────────────────────────────────────────────────────────

    fn parse_grace(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \grace
        let body = Box::new(self.parse_music()?);
        Ok(Music::Grace { body })
    }

    fn parse_acciaccatura(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \acciaccatura
        let body = Box::new(self.parse_music()?);
        Ok(Music::Acciaccatura { body })
    }

    fn parse_appoggiatura(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \appoggiatura
        let body = Box::new(self.parse_music()?);
        Ok(Music::Appoggiatura { body })
    }

    fn parse_after_grace(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \afterGrace

        // Optional fraction: N/M (both must be unsigned integers with `/`)
        let fraction = self.try_parse_fraction();

        let main = Box::new(self.parse_music()?);
        let grace = Box::new(self.parse_music()?);
        Ok(Music::AfterGrace {
            fraction,
            main,
            grace,
        })
    }

    /// Try to parse a fraction `N/M` for `\afterGrace`. Returns `None` if
    /// the current token isn't an unsigned integer. Since `Unsigned` can't
    /// start a music expression, its presence after `\afterGrace` unambiguously
    /// signals a fraction.
    fn try_parse_fraction(&mut self) -> Option<(u32, u32)> {
        if let Token::Unsigned(n) = self.peek() {
            let n = *n as u32;
            let _ = self.advance(); // consume numerator
            if *self.peek() == Token::Slash {
                let _ = self.advance(); // consume `/`
                if let Token::Unsigned(d) = self.peek() {
                    let d = *d as u32;
                    let _ = self.advance(); // consume denominator
                    return Some((n, d));
                }
                // N/ without denominator — treat as N/1
                return Some((n, 1));
            }
            // Bare N without / — treat as N/1
            return Some((n, 1));
        }
        None
    }

    fn parse_sequential_music(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            let m = self.parse_music()?;
            let m = self.try_wrap_addlyrics(m)?;
            items.push(m);
        }
        self.expect(&Token::BraceClose)?;
        Ok(Music::Sequential(items))
    }

    /// `\sequential { ... }` — explicit sequential keyword form.
    fn parse_explicit_sequential(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Sequential)?;
        self.parse_sequential_music()
    }

    fn parse_simultaneous_music(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::DoubleAngleOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::DoubleAngleClose && !self.at_eof() {
            // Skip `\\` voice separators — they act as delimiters between
            // voices but don't add semantic content at the AST level.
            if *self.peek() == Token::DoubleBackslash {
                self.advance()?;
                continue;
            }
            let m = self.parse_music()?;
            let m = self.try_wrap_addlyrics(m)?;
            items.push(m);
        }
        self.expect(&Token::DoubleAngleClose)?;
        Ok(Music::Simultaneous(items))
    }

    /// `\simultaneous { ... }` — explicit simultaneous keyword form.
    fn parse_explicit_simultaneous(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Simultaneous)?;
        // \simultaneous uses braces, not angle brackets
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_music()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(Music::Simultaneous(items))
    }

    fn parse_relative(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Relative)?;
        // Optional reference pitch before the braced body
        let pitch = if *self.peek() != Token::BraceOpen {
            // Parse a single pitch as Music::Note (or Event for now)
            if matches!(self.peek(), Token::NoteName(_)) {
                Some(Box::new(self.parse_note_event()?))
            } else {
                None
            }
        } else {
            None
        };
        let body = Box::new(self.parse_music()?);
        Ok(Music::Relative { pitch, body })
    }

    fn parse_fixed(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Fixed)?;
        // Reference pitch
        if !matches!(self.peek(), Token::NoteName(_)) {
            return Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "pitch after \\fixed".into(),
            });
        }
        let pitch = Box::new(self.parse_note_event()?);
        let body = Box::new(self.parse_music()?);
        Ok(Music::Fixed { pitch, body })
    }

    fn parse_transpose(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Transpose)?;
        // From pitch
        if !matches!(self.peek(), Token::NoteName(_)) {
            return Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "pitch after \\transpose".into(),
            });
        }
        let from = Box::new(self.parse_note_event()?);
        // To pitch
        if !matches!(self.peek(), Token::NoteName(_)) {
            return Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "second pitch after \\transpose".into(),
            });
        }
        let to = Box::new(self.parse_note_event()?);
        let body = Box::new(self.parse_music()?);
        Ok(Music::Transpose { from, to, body })
    }

    fn parse_context_music(&mut self) -> Result<Music, ParseError> {
        // \new or \context
        let keyword = match self.peek() {
            Token::New => ContextKeyword::New,
            Token::Context => ContextKeyword::Context,
            _ => unreachable!(),
        };
        self.advance()?;

        // Context type name (Symbol or String — grammar allows simple_string)
        let context_type = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::String(s) => s.clone(),
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "context type name (e.g. Staff, Voice)".into(),
                });
            }
        };
        self.advance()?;

        // Optional = "name"
        let name = if *self.peek() == Token::Equals {
            self.advance()?;
            Some(self.expect_simple_string()?)
        } else {
            None
        };

        // Optional \with { ... } (can repeat per grammar)
        let with_block = self.parse_optional_context_mods()?;

        // Music body
        let music = Box::new(self.parse_music()?);
        Ok(Music::ContextedMusic {
            keyword,
            context_type,
            name,
            with_block,
            music,
        })
    }

    fn parse_context_change(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Change)?;
        // Context type name
        let context_type = match &self.current.token {
            Token::Symbol(s) => s.clone(),
            Token::String(s) => s.clone(),
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "context type name after \\change".into(),
                });
            }
        };
        self.advance()?;
        self.expect(&Token::Equals)?;
        let name = self.expect_simple_string()?;
        Ok(Music::ContextChange { context_type, name })
    }

    /// Parse `optional_context_mods` — zero or more `\with { ... }` blocks.
    fn parse_optional_context_mods(&mut self) -> Result<Option<Vec<ContextModItem>>, ParseError> {
        let mut all_items = Vec::new();
        while *self.peek() == Token::With {
            self.advance()?;
            self.expect(&Token::BraceOpen)?;
            while *self.peek() != Token::BraceClose && !self.at_eof() {
                all_items.push(self.parse_context_mod_item()?);
            }
            self.expect(&Token::BraceClose)?;
        }
        if all_items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_items))
        }
    }

    /// Parse a `simple_string`: either a quoted string or a bare symbol.
    fn expect_simple_string(&mut self) -> Result<String, ParseError> {
        match &self.current.token {
            Token::String(_) => self.expect_string(),
            Token::Symbol(s) => {
                let s = s.clone();
                self.advance()?;
                Ok(s)
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "string or symbol".into(),
            }),
        }
    }
}
mod lyrics;
mod markup;
mod raw_blocks;
mod signatures;

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Parse a LilyPond source string into an AST.
pub fn parse(src: &str) -> Result<LilyPondFile, ParseError> {
    Parser::new(src)?.parse()
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_barcheck;
#[cfg(test)]
mod tests_chord_rep;
#[cfg(test)]
mod tests_grace;
#[cfg(test)]
mod tests_lyrics;
#[cfg(test)]
mod tests_markup;
#[cfg(test)]
mod tests_post_events;
#[cfg(test)]
mod tests_repeats;
#[cfg(test)]
mod tests_tempo_marks;

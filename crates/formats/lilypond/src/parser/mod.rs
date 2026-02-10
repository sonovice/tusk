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
            // Assignment: symbol = ...
            Token::Symbol(_) | Token::NoteName(_) => {
                // Peek ahead for `=` to distinguish assignment from music
                self.parse_assignment_or_music()
            }
            // Music expressions
            _ => Ok(ToplevelExpression::Music(self.parse_music()?)),
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
            | Token::New
            | Token::Context
            | Token::Change => {
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
                let raw = self.parse_markup_raw()?;
                Ok(AssignmentValue::Markup(raw))
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
            _ => Ok(ScoreItem::Music(self.parse_music()?)),
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
                let raw = self.parse_markup_raw()?;
                Ok(AssignmentValue::Markup(raw))
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
            Token::New | Token::Context => self.parse_context_music(),
            Token::Change => self.parse_context_change(),
            Token::Time => self.parse_time_signature(),
            Token::EscapedWord(s) if s == "clef" => self.parse_clef(),
            Token::EscapedWord(s) if s == "key" => self.parse_key_signature(),
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            Token::NoteName(_) => self.parse_note_event(),
            Token::Symbol(s) if s == "r" || s == "s" || s == "R" => self.parse_rest_or_skip(),
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

        // Check for \rest (pitched rest)
        let pitched_rest = self.try_consume(&Token::Rest);

        let post_events = self.parse_post_events();

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
        let post_events = self.parse_post_events();
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
        let post_events = self.parse_post_events();
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
    // Post-events: tie ~, slur ( ), phrasing slur \( \)
    // ──────────────────────────────────────────────────────────────────

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
                _ => break,
            }
        }
        events
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

    fn parse_sequential_music(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_music()?);
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
            items.push(self.parse_music()?);
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

    // ──────────────────────────────────────────────────────────────────
    // \clef "name"
    // ──────────────────────────────────────────────────────────────────

    fn parse_clef(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \clef (EscapedWord("clef"))
        // Clef name: quoted string or bare symbol
        let name = match &self.current.token {
            Token::String(_) => self.expect_string()?,
            Token::Symbol(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Symbol(s) => s,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "clef name (string or symbol)".into(),
                });
            }
        };
        Ok(Music::Clef(Clef { name }))
    }

    // ──────────────────────────────────────────────────────────────────
    // \key pitch \mode
    // ──────────────────────────────────────────────────────────────────

    fn parse_key_signature(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \key (EscapedWord("key"))

        // Parse tonic pitch (note name + optional octave marks)
        let offset = self.offset();
        let note_name = match &self.current.token {
            Token::NoteName(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::NoteName(s) => s,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "pitch after \\key".into(),
                });
            }
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        // Optional octave marks (rarely used in \key but valid)
        let octave = self.parse_quotes();

        let pitch = Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        };

        // Parse mode: \major, \minor, \dorian, etc.
        let mode = match &self.current.token {
            Token::EscapedWord(s) => {
                let mode = Mode::from_name(s).ok_or_else(|| ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "mode (\\major, \\minor, \\dorian, etc.)".into(),
                })?;
                self.advance()?;
                mode
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "mode (\\major, \\minor, \\dorian, etc.)".into(),
                });
            }
        };

        Ok(Music::KeySignature(KeySignature { pitch, mode }))
    }

    // ──────────────────────────────────────────────────────────────────
    // \time [n+m+...]/d
    // ──────────────────────────────────────────────────────────────────

    fn parse_time_signature(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Time)?;

        // Parse numerator(s): N or N+M+... (compound)
        let mut numerators = Vec::new();
        match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => numerators.push(n as u32),
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "time signature numerator".into(),
                });
            }
        }

        // Check for additive numerators: +N+M...
        while *self.peek() == Token::Plus {
            self.advance()?; // consume +
            match self.peek() {
                Token::Unsigned(_) => {
                    let tok = self.advance()?;
                    match tok.token {
                        Token::Unsigned(n) => numerators.push(n as u32),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    return Err(ParseError::Unexpected {
                        found: self.current.token.clone(),
                        offset: self.offset(),
                        expected: "number after + in time signature".into(),
                    });
                }
            }
        }

        // Expect /
        self.expect(&Token::Slash)?;

        // Parse denominator
        let denominator = match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "time signature denominator".into(),
                });
            }
        };

        Ok(Music::TimeSignature(TimeSignature {
            numerators,
            denominator,
        }))
    }

    // ──────────────────────────────────────────────────────────────────
    // Markup (raw, for now)
    // ──────────────────────────────────────────────────────────────────

    fn parse_markup_raw(&mut self) -> Result<String, ParseError> {
        let start = self.offset();
        self.expect(&Token::Markup)?;

        if *self.peek() == Token::BraceOpen {
            // \markup { ... } — balance braces
            self.expect(&Token::BraceOpen)?;
            let mut depth = 1u32;
            while depth > 0 && !self.at_eof() {
                match self.peek() {
                    Token::BraceOpen => {
                        depth += 1;
                        self.advance()?;
                    }
                    Token::BraceClose => {
                        depth -= 1;
                        if depth > 0 {
                            self.advance()?;
                        }
                    }
                    _ => {
                        self.advance()?;
                    }
                }
            }
            self.expect(&Token::BraceClose)?;
            // Extract raw text from source
            Ok(self.src[start..self.lexer.position()].to_string())
        } else {
            // \markup \command ... — single token
            let tok = self.advance()?;
            Ok(self.src[start..tok.span.end].to_string())
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Scheme (raw, for now)
    // ──────────────────────────────────────────────────────────────────

    fn parse_scheme_raw(&mut self) -> Result<String, ParseError> {
        let start = self.offset();
        self.expect(&Token::Hash)?;

        match self.peek() {
            Token::ParenOpen => {
                // #( ... ) — balance parens
                self.advance()?;
                let mut depth = 1u32;
                while depth > 0 && !self.at_eof() {
                    match self.peek() {
                        Token::ParenOpen => {
                            depth += 1;
                            self.advance()?;
                        }
                        Token::ParenClose => {
                            depth -= 1;
                            if depth > 0 {
                                self.advance()?;
                            }
                        }
                        _ => {
                            self.advance()?;
                        }
                    }
                }
                self.expect(&Token::ParenClose)?;
                Ok(self.src[start..self.lexer.position()].to_string())
            }
            _ => {
                // #value — single token
                let tok = self.advance()?;
                Ok(self.src[start..tok.span.end].to_string())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Parse a LilyPond source string into an AST.
pub fn parse(src: &str) -> Result<LilyPondFile, ParseError> {
    Parser::new(src)?.parse()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_only() {
        let ast = parse("\\version \"2.24.0\"").unwrap();
        assert_eq!(
            ast.version,
            Some(Version {
                version: "2.24.0".into()
            })
        );
        assert!(ast.items.is_empty());
    }

    #[test]
    fn parse_minimal_score() {
        let ast = parse("\\version \"2.24.0\"\n\\score {\n  { c4 }\n}").unwrap();
        assert!(ast.version.is_some());
        assert_eq!(ast.items.len(), 1);
        match &ast.items[0] {
            ToplevelExpression::Score(sb) => {
                assert_eq!(sb.items.len(), 1);
                match &sb.items[0] {
                    ScoreItem::Music(Music::Sequential(items)) => {
                        assert_eq!(items.len(), 1);
                        match &items[0] {
                            Music::Note(n) => {
                                assert_eq!(n.pitch.step, 'c');
                                assert_eq!(n.pitch.alter, 0.0);
                                assert_eq!(n.duration.as_ref().unwrap().base, 4);
                            }
                            other => panic!("expected Note, got {other:?}"),
                        }
                    }
                    other => panic!("expected sequential music, got {other:?}"),
                }
            }
            other => panic!("expected score, got {other:?}"),
        }
    }

    #[test]
    fn parse_score_with_layout_midi() {
        let input = r#"\version "2.24.0"
\score {
  { c4 }
  \layout { }
  \midi { }
}"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Score(sb) => {
                assert_eq!(sb.items.len(), 3);
                assert!(matches!(&sb.items[0], ScoreItem::Music(_)));
                assert!(matches!(&sb.items[1], ScoreItem::Layout(_)));
                assert!(matches!(&sb.items[2], ScoreItem::Midi(_)));
            }
            other => panic!("expected score, got {other:?}"),
        }
    }

    #[test]
    fn parse_header_block() {
        let input = r#"\header {
  title = "My Piece"
  composer = "JS Bach"
}"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Header(hb) => {
                assert_eq!(hb.fields.len(), 2);
                assert_eq!(hb.fields[0].name, "title");
                assert_eq!(
                    hb.fields[0].value,
                    AssignmentValue::String("My Piece".into())
                );
                assert_eq!(hb.fields[1].name, "composer");
            }
            other => panic!("expected header, got {other:?}"),
        }
    }

    #[test]
    fn parse_book_block() {
        let input = r#"\book {
  \header { title = "Sonata" }
  \score { { c4 } }
}"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Book(bb) => {
                assert_eq!(bb.items.len(), 2);
                assert!(matches!(&bb.items[0], BookItem::Header(_)));
                assert!(matches!(&bb.items[1], BookItem::Score(_)));
            }
            other => panic!("expected book, got {other:?}"),
        }
    }

    #[test]
    fn parse_bookpart_block() {
        let input = r#"\bookpart {
  \header { title = "Movement 1" }
  \score { { c4 } }
}"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::BookPart(bp) => {
                assert_eq!(bp.items.len(), 2);
                assert!(matches!(&bp.items[0], BookPartItem::Header(_)));
                assert!(matches!(&bp.items[1], BookPartItem::Score(_)));
            }
            other => panic!("expected bookpart, got {other:?}"),
        }
    }

    #[test]
    fn parse_toplevel_assignment() {
        let input = "melody = { c4 d4 e4 f4 }";
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Assignment(a) => {
                assert_eq!(a.name, "melody");
                assert!(matches!(&a.value, AssignmentValue::Music(_)));
            }
            other => panic!("expected assignment, got {other:?}"),
        }
    }

    #[test]
    fn parse_relative_music() {
        let input = "\\relative c' { c4 d e f }";
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Relative { pitch, body }) => {
                assert!(pitch.is_some());
                match pitch.as_deref() {
                    Some(Music::Note(n)) => {
                        assert_eq!(n.pitch.step, 'c');
                        assert_eq!(n.pitch.octave, 1);
                    }
                    other => panic!("expected Note, got {other:?}"),
                }
                assert!(matches!(body.as_ref(), Music::Sequential(_)));
            }
            other => panic!("expected relative music, got {other:?}"),
        }
    }

    #[test]
    fn parse_new_staff() {
        let input = "\\new Staff { c4 }";
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::ContextedMusic {
                keyword,
                context_type,
                name,
                ..
            }) => {
                assert_eq!(*keyword, ContextKeyword::New);
                assert_eq!(context_type, "Staff");
                assert!(name.is_none());
            }
            other => panic!("expected contexted music, got {other:?}"),
        }
    }

    #[test]
    fn parse_fragment_score_minimal() {
        let input = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/fragment_score_minimal.ly"
        ))
        .expect("fixture file");
        let ast = parse(&input).unwrap();
        assert!(ast.version.is_some());
        assert_eq!(ast.items.len(), 1);
        assert!(matches!(&ast.items[0], ToplevelExpression::Score(_)));
    }

    #[test]
    fn parse_simple_ly() {
        let input = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/simple.ly"
        ))
        .expect("fixture file");
        let ast = parse(&input).unwrap();
        assert_eq!(
            ast.version,
            Some(Version {
                version: "2.19.21".into()
            })
        );
        assert_eq!(ast.items.len(), 1);
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Relative { pitch, body }) => {
                assert!(pitch.is_none());
                match body.as_ref() {
                    Music::Sequential(items) => assert_eq!(items.len(), 8),
                    other => panic!("expected sequential, got {other:?}"),
                }
            }
            other => panic!("expected relative music, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_simple_ly() {
        let input = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/simple.ly"
        ))
        .expect("fixture file");
        let ast = parse(&input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    #[test]
    fn roundtrip_fragment_score_minimal() {
        let input = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/fragment_score_minimal.ly"
        ))
        .expect("fixture file");
        let ast = parse(&input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    // ── Phase 3 tests ───────────────────────────────────────────────

    #[test]
    fn parse_note_with_pitch() {
        let ast = parse("{ c }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'c');
                    assert_eq!(n.pitch.alter, 0.0);
                    assert_eq!(n.pitch.octave, 0);
                    assert!(!n.pitch.force_accidental);
                    assert!(!n.pitch.cautionary);
                    assert!(n.duration.is_none());
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_note_with_accidental_octave_duration() {
        let ast = parse("{ cis''4. }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'c');
                    assert_eq!(n.pitch.alter, 1.0);
                    assert_eq!(n.pitch.octave, 2);
                    let dur = n.duration.as_ref().unwrap();
                    assert_eq!(dur.base, 4);
                    assert_eq!(dur.dots, 1);
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_note_force_accidental() {
        let ast = parse("{ cis! }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert!(n.pitch.force_accidental);
                    assert!(!n.pitch.cautionary);
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_note_cautionary_accidental() {
        let ast = parse("{ bes? }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert!(!n.pitch.force_accidental);
                    assert!(n.pitch.cautionary);
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_rest() {
        let ast = parse("{ r4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Rest(r) => {
                    let dur = r.duration.as_ref().unwrap();
                    assert_eq!(dur.base, 4);
                    assert_eq!(dur.dots, 0);
                }
                other => panic!("expected Rest, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_skip() {
        let ast = parse("{ s2. }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Skip(s) => {
                    let dur = s.duration.as_ref().unwrap();
                    assert_eq!(dur.base, 2);
                    assert_eq!(dur.dots, 1);
                }
                other => panic!("expected Skip, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_multi_measure_rest() {
        let ast = parse("{ R1 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::MultiMeasureRest(r) => {
                    let dur = r.duration.as_ref().unwrap();
                    assert_eq!(dur.base, 1);
                }
                other => panic!("expected MultiMeasureRest, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_duration_multiplier() {
        let ast = parse("{ R1*4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::MultiMeasureRest(r) => {
                    let dur = r.duration.as_ref().unwrap();
                    assert_eq!(dur.base, 1);
                    assert_eq!(dur.multipliers, vec![(4, 1)]);
                }
                other => panic!("expected MultiMeasureRest, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_duration_fraction_multiplier() {
        let ast = parse("{ c4*2/3 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    let dur = n.duration.as_ref().unwrap();
                    assert_eq!(dur.base, 4);
                    assert_eq!(dur.multipliers, vec![(2, 3)]);
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_pitched_rest() {
        let ast = parse("{ c4\\rest }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert!(n.pitched_rest);
                    assert_eq!(n.pitch.step, 'c');
                }
                other => panic!("expected Note (pitched rest), got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_rest_no_duration() {
        let ast = parse("{ r }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Rest(r) => {
                    assert!(r.duration.is_none());
                }
                other => panic!("expected Rest, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_note_no_duration() {
        let ast = parse("{ c }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert!(n.duration.is_none());
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_multiple_notes() {
        let ast = parse("{ c4 d8 e16 f2 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 4);
                match &items[0] {
                    Music::Note(n) => {
                        assert_eq!(n.pitch.step, 'c');
                        assert_eq!(n.duration.as_ref().unwrap().base, 4);
                    }
                    other => panic!("expected Note, got {other:?}"),
                }
                match &items[1] {
                    Music::Note(n) => {
                        assert_eq!(n.pitch.step, 'd');
                        assert_eq!(n.duration.as_ref().unwrap().base, 8);
                    }
                    other => panic!("expected Note, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_octave_down() {
        let ast = parse("{ c,, }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.octave, -2);
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    // ── Phase 3 fixture roundtrip tests ──────────────────────────────

    fn roundtrip_fixture(name: &str) {
        let input = std::fs::read_to_string(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../tests/fixtures/lilypond/"
            )
            .to_string()
                + name,
        )
        .expect("fixture file");
        let ast = parse(&input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2, "AST mismatch after roundtrip of {name}");
    }

    #[test]
    fn roundtrip_fragment_pitches() {
        roundtrip_fixture("fragment_pitches.ly");
    }

    #[test]
    fn roundtrip_fragment_durations() {
        roundtrip_fixture("fragment_durations.ly");
    }

    #[test]
    fn roundtrip_fragment_rests() {
        roundtrip_fixture("fragment_rests.ly");
    }

    // ── Phase 4 tests ───────────────────────────────────────────────

    #[test]
    fn parse_nested_sequential_simultaneous() {
        let ast = parse("{ << { c4 d4 } { e4 f4 } >> }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(outer)) => {
                assert_eq!(outer.len(), 1);
                match &outer[0] {
                    Music::Simultaneous(voices) => {
                        assert_eq!(voices.len(), 2);
                        match &voices[0] {
                            Music::Sequential(items) => {
                                assert_eq!(items.len(), 2);
                                assert!(matches!(&items[0], Music::Note(n) if n.pitch.step == 'c'));
                                assert!(matches!(&items[1], Music::Note(n) if n.pitch.step == 'd'));
                            }
                            other => panic!("expected sequential, got {other:?}"),
                        }
                        match &voices[1] {
                            Music::Sequential(items) => {
                                assert_eq!(items.len(), 2);
                                assert!(matches!(&items[0], Music::Note(n) if n.pitch.step == 'e'));
                                assert!(matches!(&items[1], Music::Note(n) if n.pitch.step == 'f'));
                            }
                            other => panic!("expected sequential, got {other:?}"),
                        }
                    }
                    other => panic!("expected simultaneous, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_explicit_sequential_keyword() {
        let ast = parse("\\sequential { c4 d4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_explicit_simultaneous_keyword() {
        let ast = parse("\\simultaneous { { c4 } { d4 } }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Simultaneous(items)) => {
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected simultaneous, got {other:?}"),
        }
    }

    #[test]
    fn parse_voice_separator_backslash() {
        let ast = parse("<< { c4 d4 } \\\\ { e4 f4 } >>").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Simultaneous(voices)) => {
                assert_eq!(voices.len(), 2);
                assert!(matches!(&voices[0], Music::Sequential(_)));
                assert!(matches!(&voices[1], Music::Sequential(_)));
            }
            other => panic!("expected simultaneous, got {other:?}"),
        }
    }

    #[test]
    fn parse_deeply_nested_music() {
        let ast = parse("{ { { c4 } } }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(l1)) => {
                assert_eq!(l1.len(), 1);
                match &l1[0] {
                    Music::Sequential(l2) => {
                        assert_eq!(l2.len(), 1);
                        match &l2[0] {
                            Music::Sequential(l3) => {
                                assert_eq!(l3.len(), 1);
                                assert!(matches!(&l3[0], Music::Note(_)));
                            }
                            other => panic!("expected sequential, got {other:?}"),
                        }
                    }
                    other => panic!("expected sequential, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_empty_sequential() {
        let ast = parse("{ }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert!(items.is_empty());
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_empty_simultaneous() {
        let ast = parse("<< >>").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Simultaneous(items)) => {
                assert!(items.is_empty());
            }
            other => panic!("expected simultaneous, got {other:?}"),
        }
    }

    #[test]
    fn parse_simultaneous_with_notes() {
        // Notes directly inside << >> (no inner braces)
        let ast = parse("<< c4 d4 e4 >>").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Simultaneous(items)) => {
                assert_eq!(items.len(), 3);
            }
            other => panic!("expected simultaneous, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_fragment_sequential_simultaneous() {
        roundtrip_fixture("fragment_sequential_simultaneous.ly");
    }

    // ── Phase 5 tests ───────────────────────────────────────────────

    #[test]
    fn parse_new_staff_with_name() {
        let ast = parse("\\new Staff = \"violin\" { c4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::ContextedMusic {
                keyword,
                context_type,
                name,
                ..
            }) => {
                assert_eq!(*keyword, ContextKeyword::New);
                assert_eq!(context_type, "Staff");
                assert_eq!(name.as_deref(), Some("violin"));
            }
            other => panic!("expected contexted music, got {other:?}"),
        }
    }

    #[test]
    fn parse_context_staff() {
        let ast = parse("\\context Staff = \"main\" { c4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::ContextedMusic {
                keyword,
                context_type,
                name,
                ..
            }) => {
                assert_eq!(*keyword, ContextKeyword::Context);
                assert_eq!(context_type, "Staff");
                assert_eq!(name.as_deref(), Some("main"));
            }
            other => panic!("expected contexted music, got {other:?}"),
        }
    }

    #[test]
    fn parse_new_with_block() {
        let input = r#"\new Staff \with {
  \consists "Span_arpeggio_engraver"
  instrumentName = "Piano"
} { c4 }"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::ContextedMusic {
                keyword,
                context_type,
                with_block,
                ..
            }) => {
                assert_eq!(*keyword, ContextKeyword::New);
                assert_eq!(context_type, "Staff");
                let items = with_block.as_ref().unwrap();
                assert_eq!(items.len(), 2);
                assert!(
                    matches!(&items[0], ContextModItem::Consists(s) if s == "Span_arpeggio_engraver")
                );
                assert!(
                    matches!(&items[1], ContextModItem::Assignment(a) if a.name == "instrumentName")
                );
            }
            other => panic!("expected contexted music, got {other:?}"),
        }
    }

    #[test]
    fn parse_context_change() {
        let ast = parse("{ c4 \\change Staff = \"other\" d4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 3);
                match &items[1] {
                    Music::ContextChange { context_type, name } => {
                        assert_eq!(context_type, "Staff");
                        assert_eq!(name, "other");
                    }
                    other => panic!("expected ContextChange, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_nested_new_staff() {
        let input = r#"\new StaffGroup << \new Staff { c4 } \new Staff { d4 } >>"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::ContextedMusic {
                keyword,
                context_type,
                music,
                ..
            }) => {
                assert_eq!(*keyword, ContextKeyword::New);
                assert_eq!(context_type, "StaffGroup");
                match music.as_ref() {
                    Music::Simultaneous(items) => {
                        assert_eq!(items.len(), 2);
                        assert!(
                            matches!(&items[0], Music::ContextedMusic { context_type, .. } if context_type == "Staff")
                        );
                        assert!(
                            matches!(&items[1], Music::ContextedMusic { context_type, .. } if context_type == "Staff")
                        );
                    }
                    other => panic!("expected simultaneous, got {other:?}"),
                }
            }
            other => panic!("expected contexted music, got {other:?}"),
        }
    }

    #[test]
    fn parse_piano_staff() {
        let input = r#"\new PianoStaff <<
  \new Staff = "right" { c'4 }
  \new Staff = "left" { c4 }
>>"#;
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::ContextedMusic {
                keyword,
                context_type,
                ..
            }) => {
                assert_eq!(*keyword, ContextKeyword::New);
                assert_eq!(context_type, "PianoStaff");
            }
            other => panic!("expected PianoStaff, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_context_fixture() {
        roundtrip_fixture("fragment_contexts.ly");
    }

    // ── Phase 6 tests ───────────────────────────────────────────────

    #[test]
    fn parse_clef_string() {
        let ast = parse("{ \\clef \"treble\" }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Clef(c) => assert_eq!(c.name, "treble"),
                other => panic!("expected Clef, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_clef_bare_symbol() {
        let ast = parse("{ \\clef bass }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Clef(c) => assert_eq!(c.name, "bass"),
                other => panic!("expected Clef, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_clef_transposed() {
        let ast = parse("{ \\clef \"G_8\" }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Clef(c) => assert_eq!(c.name, "G_8"),
                other => panic!("expected Clef, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_key_signature() {
        let ast = parse("{ \\key d \\major }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::KeySignature(ks) => {
                    assert_eq!(ks.pitch.step, 'd');
                    assert_eq!(ks.pitch.alter, 0.0);
                    assert_eq!(ks.mode, Mode::Major);
                }
                other => panic!("expected KeySignature, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_key_signature_flat() {
        let ast = parse("{ \\key bes \\minor }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::KeySignature(ks) => {
                    assert_eq!(ks.pitch.step, 'b');
                    assert_eq!(ks.pitch.alter, -1.0);
                    assert_eq!(ks.mode, Mode::Minor);
                }
                other => panic!("expected KeySignature, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_key_signature_modes() {
        for (mode_str, mode) in [
            ("major", Mode::Major),
            ("minor", Mode::Minor),
            ("dorian", Mode::Dorian),
            ("phrygian", Mode::Phrygian),
            ("lydian", Mode::Lydian),
            ("mixolydian", Mode::Mixolydian),
            ("aeolian", Mode::Aeolian),
            ("locrian", Mode::Locrian),
            ("ionian", Mode::Ionian),
        ] {
            let input = format!("{{ \\key c \\{mode_str} }}");
            let ast = parse(&input).unwrap();
            match &ast.items[0] {
                ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                    Music::KeySignature(ks) => assert_eq!(ks.mode, mode, "mode: {mode_str}"),
                    other => panic!("expected KeySignature for {mode_str}, got {other:?}"),
                },
                other => panic!("expected sequential for {mode_str}, got {other:?}"),
            }
        }
    }

    #[test]
    fn parse_time_signature_simple() {
        let ast = parse("{ \\time 4/4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::TimeSignature(ts) => {
                    assert_eq!(ts.numerators, vec![4]);
                    assert_eq!(ts.denominator, 4);
                }
                other => panic!("expected TimeSignature, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_time_signature_compound() {
        let ast = parse("{ \\time 3/4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::TimeSignature(ts) => {
                    assert_eq!(ts.numerators, vec![3]);
                    assert_eq!(ts.denominator, 4);
                }
                other => panic!("expected TimeSignature, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_time_signature_additive() {
        let ast = parse("{ \\time 2+3/8 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::TimeSignature(ts) => {
                    assert_eq!(ts.numerators, vec![2, 3]);
                    assert_eq!(ts.denominator, 8);
                }
                other => panic!("expected TimeSignature, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_time_signature_triple_additive() {
        let ast = parse("{ \\time 3+3+2/8 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::TimeSignature(ts) => {
                    assert_eq!(ts.numerators, vec![3, 3, 2]);
                    assert_eq!(ts.denominator, 8);
                }
                other => panic!("expected TimeSignature, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_clef_key_time_sequence() {
        let input = "{ \\clef \"treble\" \\key d \\major \\time 4/4 c4 }";
        let ast = parse(input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 4);
                assert!(matches!(&items[0], Music::Clef(_)));
                assert!(matches!(&items[1], Music::KeySignature(_)));
                assert!(matches!(&items[2], Music::TimeSignature(_)));
                assert!(matches!(&items[3], Music::Note(_)));
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_fragment_clef_key_time() {
        roundtrip_fixture("fragment_clef_key_time.ly");
    }

    // ── Phase 7 tests ───────────────────────────────────────────────

    #[test]
    fn parse_transpose() {
        let ast = parse("\\transpose c d { c4 d e f }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Transpose { from, to, body }) => {
                match from.as_ref() {
                    Music::Note(n) => {
                        assert_eq!(n.pitch.step, 'c');
                        assert_eq!(n.pitch.octave, 0);
                    }
                    other => panic!("expected Note for from, got {other:?}"),
                }
                match to.as_ref() {
                    Music::Note(n) => {
                        assert_eq!(n.pitch.step, 'd');
                        assert_eq!(n.pitch.octave, 0);
                    }
                    other => panic!("expected Note for to, got {other:?}"),
                }
                assert!(matches!(body.as_ref(), Music::Sequential(_)));
            }
            other => panic!("expected Transpose, got {other:?}"),
        }
    }

    #[test]
    fn parse_transpose_with_octave() {
        let ast = parse("\\transpose c' d'' { c4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Transpose { from, to, .. }) => {
                match from.as_ref() {
                    Music::Note(n) => assert_eq!(n.pitch.octave, 1),
                    other => panic!("expected Note, got {other:?}"),
                }
                match to.as_ref() {
                    Music::Note(n) => assert_eq!(n.pitch.octave, 2),
                    other => panic!("expected Note, got {other:?}"),
                }
            }
            other => panic!("expected Transpose, got {other:?}"),
        }
    }

    #[test]
    fn parse_transpose_with_accidentals() {
        let ast = parse("\\transpose bes ees' { c4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Transpose { from, to, .. }) => {
                match from.as_ref() {
                    Music::Note(n) => {
                        assert_eq!(n.pitch.step, 'b');
                        assert_eq!(n.pitch.alter, -1.0);
                    }
                    other => panic!("expected Note, got {other:?}"),
                }
                match to.as_ref() {
                    Music::Note(n) => {
                        assert_eq!(n.pitch.step, 'e');
                        assert_eq!(n.pitch.alter, -1.0);
                        assert_eq!(n.pitch.octave, 1);
                    }
                    other => panic!("expected Note, got {other:?}"),
                }
            }
            other => panic!("expected Transpose, got {other:?}"),
        }
    }

    #[test]
    fn parse_octave_check() {
        let ast = parse("{ c='4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'c');
                    assert_eq!(n.pitch.octave_check, Some(1));
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_octave_check_down() {
        let ast = parse("{ f=,,4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'f');
                    assert_eq!(n.pitch.octave_check, Some(-2));
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_octave_check_no_marks() {
        // `=` with no following octave marks means octave 0
        let ast = parse("{ c=4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.octave_check, Some(0));
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_no_octave_check() {
        let ast = parse("{ c'4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert!(n.pitch.octave_check.is_none());
                }
                other => panic!("expected Note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_relative_no_pitch() {
        let ast = parse("\\relative { c'4 d e f }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Relative { pitch, body }) => {
                assert!(pitch.is_none());
                assert!(matches!(body.as_ref(), Music::Sequential(_)));
            }
            other => panic!("expected relative, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_fragment_relative_transpose() {
        roundtrip_fixture("fragment_relative_transpose.ly");
    }

    // ── Phase 8 chord parser tests ────────────────────────────────────

    #[test]
    fn parse_chord_basic() {
        let ast = parse("{ <c e g>4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 1);
                match &items[0] {
                    Music::Chord(chord) => {
                        assert_eq!(chord.pitches.len(), 3);
                        assert_eq!(chord.pitches[0].step, 'c');
                        assert_eq!(chord.pitches[1].step, 'e');
                        assert_eq!(chord.pitches[2].step, 'g');
                        assert_eq!(chord.duration.as_ref().unwrap().base, 4);
                    }
                    other => panic!("expected Chord, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_accidentals() {
        let ast = parse("{ <c es g>2. }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Chord(chord) => {
                    assert_eq!(chord.pitches.len(), 3);
                    assert_eq!(chord.pitches[1].step, 'e');
                    assert_eq!(chord.pitches[1].alter, -1.0); // es = E-flat
                    assert_eq!(chord.duration.as_ref().unwrap().base, 2);
                    assert_eq!(chord.duration.as_ref().unwrap().dots, 1);
                }
                other => panic!("expected Chord, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_octave_marks() {
        let ast = parse("{ <d' fis' a'>8 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Chord(chord) => {
                    assert_eq!(chord.pitches.len(), 3);
                    assert_eq!(chord.pitches[0].step, 'd');
                    assert_eq!(chord.pitches[0].octave, 1);
                    assert_eq!(chord.pitches[1].step, 'f');
                    assert_eq!(chord.pitches[1].alter, 1.0); // fis = F-sharp
                    assert_eq!(chord.pitches[1].octave, 1);
                    assert_eq!(chord.pitches[2].step, 'a');
                    assert_eq!(chord.pitches[2].octave, 1);
                    assert_eq!(chord.duration.as_ref().unwrap().base, 8);
                }
                other => panic!("expected Chord, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_force_cautionary() {
        let ast = parse("{ <cis''! e''?>4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Chord(chord) => {
                    assert_eq!(chord.pitches.len(), 2);
                    assert!(chord.pitches[0].force_accidental);
                    assert!(!chord.pitches[0].cautionary);
                    assert!(!chord.pitches[1].force_accidental);
                    assert!(chord.pitches[1].cautionary);
                }
                other => panic!("expected Chord, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_no_duration() {
        let ast = parse("{ <c e g> }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Chord(chord) => {
                    assert_eq!(chord.pitches.len(), 3);
                    assert!(chord.duration.is_none());
                }
                other => panic!("expected Chord, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_single_pitch() {
        let ast = parse("{ <c>4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Chord(chord) => {
                    assert_eq!(chord.pitches.len(), 1);
                    assert_eq!(chord.pitches[0].step, 'c');
                }
                other => panic!("expected Chord, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_mixed_with_notes() {
        let ast = parse("{ c4 <c e g>4 d4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 3);
                assert!(matches!(&items[0], Music::Note(_)));
                assert!(matches!(&items[1], Music::Chord(_)));
                assert!(matches!(&items[2], Music::Note(_)));
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_chord_basic() {
        let input = "{ <c e g>4 }";
        let ast = parse(input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    #[test]
    fn roundtrip_chord_complex() {
        let input = "{ <c es g>2. <d' fis' a'>8 <bes, d f>1 }";
        let ast = parse(input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    #[test]
    fn roundtrip_fragment_chords() {
        roundtrip_fixture("fragment_chords.ly");
    }

    // ── Phase 9 parser tests ──────────────────────────────────────

    #[test]
    fn parse_tie() {
        let ast = parse("{ c4~ c4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 2);
                match &items[0] {
                    Music::Note(n) => {
                        assert_eq!(n.post_events, vec![PostEvent::Tie]);
                    }
                    other => panic!("expected note, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_slur() {
        let ast = parse("{ c4( d4 e4) }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 3);
                match &items[0] {
                    Music::Note(n) => {
                        assert_eq!(n.post_events, vec![PostEvent::SlurStart]);
                    }
                    other => panic!("expected note, got {other:?}"),
                }
                match &items[2] {
                    Music::Note(n) => {
                        assert_eq!(n.post_events, vec![PostEvent::SlurEnd]);
                    }
                    other => panic!("expected note, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_phrasing_slur() {
        let ast = parse("{ c4\\( d4 e4\\) }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => {
                assert_eq!(items.len(), 3);
                match &items[0] {
                    Music::Note(n) => {
                        assert_eq!(n.post_events, vec![PostEvent::PhrasingSlurStart]);
                    }
                    other => panic!("expected note, got {other:?}"),
                }
                match &items[2] {
                    Music::Note(n) => {
                        assert_eq!(n.post_events, vec![PostEvent::PhrasingSlurEnd]);
                    }
                    other => panic!("expected note, got {other:?}"),
                }
            }
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_multiple_post_events() {
        let ast = parse("{ c4~( d4) }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::Tie, PostEvent::SlurStart]);
                }
                other => panic!("expected note, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_chord_with_tie() {
        let ast = parse("{ <c e g>4~ <c e g>4 }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Chord(c) => {
                    assert_eq!(c.post_events, vec![PostEvent::Tie]);
                }
                other => panic!("expected chord, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn parse_rest_with_slur() {
        // Unusual but valid in LilyPond grammar
        let ast = parse("{ r4( c4) }").unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Rest(r) => {
                    assert_eq!(r.post_events, vec![PostEvent::SlurStart]);
                }
                other => panic!("expected rest, got {other:?}"),
            },
            other => panic!("expected sequential, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_tie() {
        let input = "{ c4~ c4 }";
        let ast = parse(input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    #[test]
    fn roundtrip_slur() {
        let input = "{ c4( d4 e4) }";
        let ast = parse(input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    #[test]
    fn roundtrip_phrasing_slur() {
        let input = "{ c4\\( d4 e4\\) }";
        let ast = parse(input).unwrap();
        let output = crate::serializer::serialize(&ast);
        let ast2 = parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    #[test]
    fn roundtrip_fragment_ties_slurs() {
        roundtrip_fixture("fragment_ties_slurs.ly");
    }
}

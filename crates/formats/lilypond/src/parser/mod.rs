//! Recursive-descent parser for LilyPond syntax.
//!
//! Builds an AST from token stream produced by the lexer. The parser follows
//! the productions in `specs/lilypond/repo/lily/parser.yy` but implemented as
//! a hand-rolled recursive-descent parser in Rust.

use std::fmt;

use thiserror::Error;

use crate::lexer::{LexError, Lexer, SpannedToken, Token};
use crate::model::*;

// ---------------------------------------------------------------------------
// Errors & Warnings
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

/// Non-fatal diagnostic emitted during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseWarning {
    /// Mixed `'` and `,` in a single octave mark sequence (e.g. `c',`).
    MixedOctaveMarks { offset: usize },
    /// Octave marks found after the duration (e.g. `c4''`).
    OctaveAfterDuration { offset: usize, note_offset: usize },
    /// A parse error occurred inside a `{ }` or `<< >>` block and the
    /// parser skipped tokens to recover.
    RecoveredError { offset: usize, message: String },
}

impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MixedOctaveMarks { offset } => {
                write!(f, "mixed ' and , in octave marks at byte offset {offset}")
            }
            Self::OctaveAfterDuration {
                offset,
                note_offset,
            } => {
                write!(
                    f,
                    "octave marks at byte offset {offset} should precede duration \
                     (note at byte offset {note_offset})"
                )
            }
            Self::RecoveredError { offset, message } => {
                write!(f, "recovered from error at byte offset {offset}: {message}")
            }
        }
    }
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
    /// Optional second lookahead token (for two-token peek).
    lookahead: Option<SpannedToken>,
    /// Non-fatal warnings collected during parsing.
    warnings: Vec<ParseWarning>,
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
            lookahead: None,
            warnings: Vec::new(),
        })
    }

    /// Parse the entire file and return the AST.
    pub fn parse(mut self) -> Result<LilyPondFile, ParseError> {
        self.parse_file()
    }

    /// Parse the entire file, returning the AST and any warnings.
    pub fn parse_with_warnings(mut self) -> Result<(LilyPondFile, Vec<ParseWarning>), ParseError> {
        let file = self.parse_file()?;
        Ok((file, self.warnings))
    }

    /// Record a non-fatal warning.
    pub(super) fn warn(&mut self, warning: ParseWarning) {
        self.warnings.push(warning);
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
        self.current = if let Some(la) = self.lookahead.take() {
            la
        } else {
            self.lexer.next_token()?
        };
        Ok(prev)
    }

    /// Peek at the second token (two-token lookahead).
    fn peek2(&mut self) -> Result<&Token, ParseError> {
        if self.lookahead.is_none() {
            self.lookahead = Some(self.lexer.next_token()?);
        }
        Ok(&self.lookahead.as_ref().unwrap().token)
    }

    fn expect(&mut self, expected: &Token) -> Result<SpannedToken, ParseError> {
        if &self.current.token == expected {
            self.advance()
        } else if self.at_eof() {
            Err(ParseError::UnexpectedEof {
                expected: Self::describe_token(expected),
            })
        } else {
            Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: Self::describe_token(expected),
            })
        }
    }

    /// Human-readable description of a token for error messages.
    fn describe_token(token: &Token) -> String {
        match token {
            Token::BraceOpen => "opening brace '{'".into(),
            Token::BraceClose => "closing brace '}'".into(),
            Token::AngleOpen => "'<'".into(),
            Token::AngleClose => "'>'".into(),
            Token::DoubleAngleOpen => "'<<'".into(),
            Token::DoubleAngleClose => "'>>'".into(),
            Token::Equals => "'='".into(),
            other => format!("{other:?}"),
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

    /// Force the lexer past any characters it can't tokenize.
    ///
    /// Skips bytes one at a time until the lexer produces a valid token
    /// (or hits EOF), updating `self.current` with the result.
    fn force_past_lex_errors(&mut self) {
        loop {
            self.lexer.skip_byte();
            match self.lexer.next_token() {
                Ok(tok) => {
                    self.current = tok;
                    return;
                }
                Err(_) => {
                    if self.lexer.position() >= self.src.len() {
                        self.current = SpannedToken::new(
                            Token::Eof,
                            crate::lexer::Span::new(self.src.len(), self.src.len()),
                        );
                        return;
                    }
                }
            }
        }
    }

    /// Skip tokens until a recovery point is reached.
    ///
    /// Recovery points are: `}`, `|`, `>>`, EOF, or any token that can
    /// start a new music expression. If a `|` is found it is consumed
    /// (it's a bar check); closers `}` / `>>` are left in place for the
    /// enclosing parse loop.
    fn skip_to_recovery_point(&mut self) {
        loop {
            match self.peek() {
                Token::BraceClose | Token::DoubleAngleClose | Token::Eof => break,
                Token::Pipe => {
                    let _ = self.advance(); // consume bar check
                    break;
                }
                // Known music-start tokens — stop before them
                Token::NoteName(_)
                | Token::BraceOpen
                | Token::AngleOpen
                | Token::DoubleAngleOpen => break,
                Token::Symbol(s) if s == "r" || s == "s" || s == "R" || s == "q" => break,
                _ => {
                    if self.advance().is_err() {
                        self.force_past_lex_errors();
                    }
                }
            }
        }
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
            Token::Paper => Ok(ToplevelExpression::Paper(self.parse_paper_block()?)),
            Token::Layout => Ok(ToplevelExpression::Layout(self.parse_layout_block()?)),
            Token::Midi => Ok(ToplevelExpression::Midi(self.parse_midi_block()?)),
            Token::Markup => {
                let m = self.parse_markup()?;
                Ok(ToplevelExpression::Markup(m))
            }
            Token::MarkupList => {
                let ml = self.parse_markuplist()?;
                Ok(ToplevelExpression::MarkupList(ml))
            }
            // Assignment: symbol/string = ...
            Token::Symbol(_) | Token::NoteName(_) => {
                // Peek ahead for `=` to distinguish assignment from music
                self.parse_assignment_or_music()
            }
            // String as assignment LHS: "name" = value (grammar: assignment_id)
            Token::String(_) => self.parse_string_assignment(),
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

    /// Parse `"string" = value` assignment (grammar: assignment_id STRING).
    /// No backtracking needed — a bare string at statement level can only be
    /// an assignment LHS.
    fn parse_string_assignment(&mut self) -> Result<ToplevelExpression, ParseError> {
        let name = self.expect_string()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_assignment_value()?;
        Ok(ToplevelExpression::Assignment(Assignment { name, value }))
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
            Token::Unsigned(_) | Token::Real(_) => self.parse_assignment_number(),
            // Unary minus starting a numeric expression: `-5`, `-180\mm`
            Token::Dash => {
                let expr = self.parse_number_expression()?;
                Ok(AssignmentValue::NumericExpression(expr))
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
            | Token::ChordMode
            | Token::Chords
            | Token::DrumMode
            | Token::Drums
            | Token::FigureMode
            | Token::Figures
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
                let expr = self.parse_scheme_expr()?;
                Ok(AssignmentValue::SchemeExpr(expr))
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
            Token::String(_) => {
                let name = self.expect_string()?;
                self.expect(&Token::Equals)?;
                let value = self.parse_assignment_value()?;
                Ok(BookItem::Assignment(Assignment { name, value }))
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
            Token::String(_) => {
                let name = self.expect_string()?;
                self.expect(&Token::Equals)?;
                let value = self.parse_assignment_value()?;
                Ok(BookPartItem::Assignment(Assignment { name, value }))
            }
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
            Token::String(s) => s.clone(),
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
                let expr = self.parse_scheme_expr()?;
                Ok(AssignmentValue::SchemeExpr(expr))
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

    // ──────────────────────────────────────────────────────────────────
    // \midi { ... }
    // ──────────────────────────────────────────────────────────────────

    fn parse_midi_block(&mut self) -> Result<MidiBlock, ParseError> {
        self.expect(&Token::Midi)?;
        self.expect(&Token::BraceOpen)?;
        let mut body = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            body.push(self.parse_midi_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(MidiBlock { body })
    }

    fn parse_midi_item(&mut self) -> Result<MidiItem, ParseError> {
        match self.peek() {
            Token::Context => Ok(MidiItem::ContextBlock(self.parse_context_mod_block()?)),
            _ => {
                let a = self.parse_output_def_assignment()?;
                Ok(MidiItem::Assignment(a))
            }
        }
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
            Token::String(s) => s.clone(),
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
            Token::ChordMode => self.parse_chord_mode(),
            Token::Chords => self.parse_chords_shorthand(),
            Token::DrumMode => self.parse_drum_mode(),
            Token::Drums => self.parse_drums_shorthand(),
            Token::FigureMode => self.parse_figure_mode(),
            Token::Figures => self.parse_figures_shorthand(),
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
            Token::Override => self.parse_override(),
            Token::Revert => self.parse_revert(),
            Token::Set => self.parse_set(),
            Token::Unset => self.parse_unset(),
            Token::Once => self.parse_once(),
            Token::EscapedWord(s) if s == "bar" => self.parse_bar_line(),
            Token::Pipe => {
                self.advance()?;
                Ok(Music::BarCheck)
            }
            Token::Hash => {
                // #expr in music position (grammar: music_embedded / embedded_scm_active).
                // We can't evaluate Scheme, so wrap as Music::SchemeMusic.
                let expr = self.parse_scheme_expr()?;
                Ok(Music::SchemeMusic(expr))
            }
            Token::EscapedWord(_) => self.parse_identifier_or_function_call(),
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
            let before = self.offset();
            match self.parse_music() {
                Ok(m) => {
                    let m = self.try_wrap_addlyrics(m)?;
                    items.push(m);
                }
                Err(e) => {
                    // Recovery: skip to next `|`, `}`, or known music-start token
                    self.warn(ParseWarning::RecoveredError {
                        offset: before,
                        message: e.to_string(),
                    });
                    self.skip_to_recovery_point();
                    // If no progress (e.g. lex error left current unchanged),
                    // force past the problematic characters.
                    if self.offset() == before && !self.at_eof() {
                        self.force_past_lex_errors();
                    }
                }
            }
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
            let before = self.offset();
            match self.parse_music() {
                Ok(m) => {
                    let m = self.try_wrap_addlyrics(m)?;
                    items.push(m);
                }
                Err(e) => {
                    self.warn(ParseWarning::RecoveredError {
                        offset: before,
                        message: e.to_string(),
                    });
                    self.skip_to_recovery_point();
                    if self.offset() == before && !self.at_eof() {
                        self.force_past_lex_errors();
                    }
                }
            }
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

    /// Parse `\new`/`\context` followed by context type, optional name,
    /// optional `\with` block, and a `contextable_music` body.
    ///
    /// Grammar: `\new CONTEXT_TYPE [= STRING] [optional_context_mods]
    ///           contextable_music`
    ///
    /// `contextable_music` accepts `basic_music | pitch_as_music | event_chord`
    /// — a subset of full music that excludes standalone `\addlyrics` and other
    /// constructs requiring wrapping. In practice, `parse_music()` already
    /// rejects standalone `\addlyrics`, so this uses `parse_pitch_or_music()`
    /// which additionally handles bare pitches as music events.
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

        // contextable_music body (pitch_or_music handles bare pitches too)
        let music = Box::new(self.parse_pitch_or_music()?);
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
mod chords;
mod drums;
mod figures;
mod functions;
mod lyrics;
mod markup;
mod note_events;
mod numeric;
mod properties;
mod raw_blocks;
mod signatures;

/// Parse a LilyPond source string into an AST.
pub fn parse(src: &str) -> Result<LilyPondFile, ParseError> {
    Parser::new(src)?.parse()
}

/// Parse a LilyPond source string, returning the AST and any warnings.
pub fn parse_with_warnings(src: &str) -> Result<(LilyPondFile, Vec<ParseWarning>), ParseError> {
    Parser::new(src)?.parse_with_warnings()
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_barcheck;
#[cfg(test)]
mod tests_chord_rep;
#[cfg(test)]
mod tests_chords;
#[cfg(test)]
mod tests_drums;
#[cfg(test)]
mod tests_error_recovery;
#[cfg(test)]
mod tests_figures;
#[cfg(test)]
mod tests_grace;
#[cfg(test)]
mod tests_lyrics;
#[cfg(test)]
mod tests_markup;
#[cfg(test)]
mod tests_music_functions;
#[cfg(test)]
mod tests_output_defs;
#[cfg(test)]
mod tests_post_events;
#[cfg(test)]
mod tests_properties;
#[cfg(test)]
mod tests_repeats;
#[cfg(test)]
mod tests_scheme;
#[cfg(test)]
mod tests_tempo_marks;
#[cfg(test)]
mod tests_variables;

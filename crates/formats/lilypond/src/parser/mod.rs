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
            Token::Relative | Token::Fixed | Token::New | Token::Context => {
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
    // Music expressions (simplified for Phase 2)
    // ──────────────────────────────────────────────────────────────────

    fn parse_music(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            Token::BraceOpen => self.parse_sequential_music(),
            Token::DoubleAngleOpen => self.parse_simultaneous_music(),
            Token::Relative => self.parse_relative(),
            Token::Fixed => self.parse_fixed(),
            Token::New | Token::Context => self.parse_context_music(),
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            Token::NoteName(_) => {
                let tok = self.advance()?;
                let mut text = match tok.token {
                    Token::NoteName(s) => s,
                    _ => unreachable!(),
                };
                // Consume trailing octave marks, accidental modifiers, duration, dots
                self.consume_note_suffix(&mut text);
                Ok(Music::Event(text))
            }
            Token::Symbol(s) if s == "r" || s == "s" || s == "R" => {
                let tok = self.advance()?;
                let mut text = match tok.token {
                    Token::Symbol(s) => s,
                    _ => unreachable!(),
                };
                self.consume_note_suffix(&mut text);
                Ok(Music::Event(text))
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "music expression".into(),
            }),
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

    fn parse_simultaneous_music(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::DoubleAngleOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::DoubleAngleClose && !self.at_eof() {
            items.push(self.parse_music()?);
        }
        self.expect(&Token::DoubleAngleClose)?;
        Ok(Music::Simultaneous(items))
    }

    fn parse_relative(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Relative)?;
        // Optional reference pitch before the braced body
        let pitch = if *self.peek() != Token::BraceOpen {
            // Parse a single pitch as Music::Event
            if matches!(self.peek(), Token::NoteName(_)) {
                let tok = self.advance()?;
                let mut text = match tok.token {
                    Token::NoteName(s) => s,
                    _ => unreachable!(),
                };
                self.consume_note_suffix(&mut text);
                Some(Box::new(Music::Event(text)))
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
        let tok = self.advance()?;
        let mut text = match tok.token {
            Token::NoteName(s) => s,
            _ => {
                return Err(ParseError::Unexpected {
                    found: tok.token,
                    offset: tok.span.start,
                    expected: "pitch after \\fixed".into(),
                });
            }
        };
        self.consume_note_suffix(&mut text);
        let pitch = Box::new(Music::Event(text));
        let body = Box::new(self.parse_music()?);
        Ok(Music::Fixed { pitch, body })
    }

    fn parse_context_music(&mut self) -> Result<Music, ParseError> {
        // \new or \context
        self.advance()?;

        // Context type name
        let context_type = match &self.current.token {
            Token::Symbol(s) => s.clone(),
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
            Some(self.expect_string()?)
        } else {
            None
        };

        // Optional \with { ... }
        let with_block = if *self.peek() == Token::With {
            self.advance()?;
            self.expect(&Token::BraceOpen)?;
            let mut items = Vec::new();
            while *self.peek() != Token::BraceClose && !self.at_eof() {
                items.push(self.parse_context_mod_item()?);
            }
            self.expect(&Token::BraceClose)?;
            Some(items)
        } else {
            None
        };

        // Music body
        let music = Box::new(self.parse_music()?);
        Ok(Music::ContextedMusic {
            context_type,
            name,
            with_block,
            music,
        })
    }

    /// Consume trailing octave marks (`'`, `,`), force/cautionary (`!`, `?`),
    /// duration digits, and dots after a note name or rest.
    fn consume_note_suffix(&mut self, text: &mut String) {
        loop {
            match self.peek() {
                Token::Quote => {
                    text.push('\'');
                    let _ = self.advance();
                }
                Token::Comma => {
                    text.push(',');
                    let _ = self.advance();
                }
                Token::Exclamation => {
                    text.push('!');
                    let _ = self.advance();
                }
                Token::Question => {
                    text.push('?');
                    let _ = self.advance();
                }
                Token::Unsigned(n) => {
                    text.push_str(&n.to_string());
                    let _ = self.advance();
                }
                Token::Dot => {
                    text.push('.');
                    let _ = self.advance();
                }
                Token::Star => {
                    // Duration multiplier: *N or *N/M
                    text.push('*');
                    let _ = self.advance();
                    if let Token::Unsigned(n) = self.peek() {
                        text.push_str(&n.to_string());
                        let _ = self.advance();
                        if *self.peek() == Token::Slash {
                            text.push('/');
                            let _ = self.advance();
                            if let Token::Unsigned(d) = self.peek() {
                                text.push_str(&d.to_string());
                                let _ = self.advance();
                            }
                        }
                    }
                }
                _ => break,
            }
        }
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
                            Music::Event(s) => assert_eq!(s, "c4"),
                            other => panic!("expected Event, got {other:?}"),
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
                    Some(Music::Event(s)) => assert_eq!(s, "c'"),
                    other => panic!("expected pitch Event, got {other:?}"),
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
                context_type, name, ..
            }) => {
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
}

//! Lexer and tokenizer for LilyPond input.
//!
//! Handles mode switching (note, lyric, chord, drum, figured bass, markup) and
//! produces a stream of tokens for the parser.
//!
//! The lexer is a hand-rolled scanner that consumes UTF-8 input one byte at a
//! time and produces [`SpannedToken`]s. Whitespace and comments are silently
//! skipped; they do not appear in the token stream.

pub mod tokens;

use thiserror::Error;
pub use tokens::{Span, SpannedToken, Token};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character '{ch}' at byte offset {offset}")]
    UnexpectedChar { ch: char, offset: usize },

    #[error("unterminated string literal starting at byte offset {offset}")]
    UnterminatedString { offset: usize },

    #[error("unterminated block comment starting at byte offset {offset}")]
    UnterminatedBlockComment { offset: usize },

    #[error("invalid escape sequence '\\{ch}' in string at byte offset {offset}")]
    InvalidEscape { ch: char, offset: usize },
}

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

/// Lexer state machine for LilyPond source.
///
/// Create with [`Lexer::new`], then call [`Lexer::next_token`] repeatedly
/// until [`Token::Eof`] is returned.
pub struct Lexer<'src> {
    /// Full source text (UTF-8).
    src: &'src str,
    /// Source as byte slice for fast indexing.
    bytes: &'src [u8],
    /// Current byte position.
    pos: usize,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer over the given source string.
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    /// Current byte position in the source.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Produce the next token, skipping whitespace and comments.
    pub fn next_token(&mut self) -> Result<SpannedToken, LexError> {
        self.skip_whitespace_and_comments()?;

        if self.pos >= self.bytes.len() {
            return Ok(SpannedToken::new(Token::Eof, Span::new(self.pos, self.pos)));
        }

        let start = self.pos;
        let b = self.bytes[self.pos];

        match b {
            // ── String literal ───────────────────────────────────────
            b'"' => self.lex_string(),

            // ── Backslash commands & escaped operators ────────────────
            b'\\' => self.lex_backslash(start),

            // ── Digits → Unsigned or Real ────────────────────────────
            b'0'..=b'9' => self.lex_number(start),

            // ── Letters → note name, symbol, or special words ────────
            b'a'..=b'z' | b'A'..=b'Z' => self.lex_word(start),

            // ── Brackets & braces ────────────────────────────────────
            b'{' => self.single_char(Token::BraceOpen, start),
            b'}' => self.single_char(Token::BraceClose, start),
            b'[' => self.single_char(Token::BracketOpen, start),
            b']' => self.single_char(Token::BracketClose, start),
            b'(' => self.single_char(Token::ParenOpen, start),
            b')' => self.single_char(Token::ParenClose, start),

            // ── Angle brackets (single or double) ────────────────────
            b'<' => {
                if self.peek_at(1) == Some(b'<') {
                    self.pos += 2;
                    Ok(SpannedToken::new(
                        Token::DoubleAngleOpen,
                        Span::new(start, self.pos),
                    ))
                } else {
                    self.single_char(Token::AngleOpen, start)
                }
            }
            b'>' => {
                if self.peek_at(1) == Some(b'>') {
                    self.pos += 2;
                    Ok(SpannedToken::new(
                        Token::DoubleAngleClose,
                        Span::new(start, self.pos),
                    ))
                } else {
                    self.single_char(Token::AngleClose, start)
                }
            }

            // ── Simple single-character operators ────────────────────
            b'~' => self.single_char(Token::Tilde, start),
            b'|' => self.single_char(Token::Pipe, start),
            b'=' => self.single_char(Token::Equals, start),
            b'.' => self.single_char(Token::Dot, start),
            b'\'' => self.single_char(Token::Quote, start),
            b',' => self.single_char(Token::Comma, start),
            b'!' => self.single_char(Token::Exclamation, start),
            b'?' => self.single_char(Token::Question, start),
            b'+' => self.single_char(Token::Plus, start),
            b'*' => self.single_char(Token::Star, start),
            b'/' => self.single_char(Token::Slash, start),
            b':' => self.single_char(Token::Colon, start),
            b'#' => self.single_char(Token::Hash, start),

            // ── Direction / articulation / other prefix chars ─────────
            // Note: `--` and `__` are lyric hyphen/extender in lyric mode,
            // but in note mode they are two separate tokens. Since we don't
            // have lexer mode switching yet, always produce individual tokens.
            // Lyric mode (Phase 20) will handle compound `--`/`__` at the
            // parser level or via mode-aware lexing.
            b'-' => self.single_char(Token::Dash, start),
            b'^' => self.single_char(Token::Caret, start),
            b'_' => self.single_char(Token::Underscore, start),

            _ => {
                // Try to decode a UTF-8 char for the error message.
                let ch = self.src[self.pos..].chars().next().unwrap_or('?');
                Err(LexError::UnexpectedChar {
                    ch,
                    offset: self.pos,
                })
            }
        }
    }

    /// Collect all remaining tokens into a Vec (convenience for tests).
    pub fn tokenize_all(&mut self) -> Result<Vec<SpannedToken>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok.token == Token::Eof {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }

    // ──────────────────────────────────────────────────────────────────
    // Internal helpers
    // ──────────────────────────────────────────────────────────────────

    /// Peek at the byte `offset` positions ahead of `self.pos`.
    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    /// Advance by one byte and return a single-character token.
    fn single_char(&mut self, token: Token, start: usize) -> Result<SpannedToken, LexError> {
        self.pos += 1;
        Ok(SpannedToken::new(token, Span::new(start, self.pos)))
    }

    // ── Whitespace & comments ────────────────────────────────────────

    /// Skip whitespace and comments (`%` line and `%{ ... %}` block).
    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexError> {
        loop {
            // Skip whitespace
            while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos >= self.bytes.len() {
                return Ok(());
            }
            if self.bytes[self.pos] == b'%' {
                if self.peek_at(1) == Some(b'{') {
                    // Block comment %{ ... %}
                    self.skip_block_comment()?;
                } else {
                    // Line comment % ... \n
                    self.skip_line_comment();
                }
            } else {
                return Ok(());
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
            self.pos += 1;
        }
        // Skip the newline itself
        if self.pos < self.bytes.len() {
            self.pos += 1;
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), LexError> {
        let start = self.pos;
        self.pos += 2; // skip %{
        let mut depth = 1u32;
        while self.pos < self.bytes.len() && depth > 0 {
            if self.bytes[self.pos] == b'%' && self.peek_at(1) == Some(b'{') {
                depth += 1;
                self.pos += 2;
            } else if self.bytes[self.pos] == b'%' && self.peek_at(1) == Some(b'}') {
                depth -= 1;
                self.pos += 2;
            } else {
                self.pos += 1;
            }
        }
        if depth > 0 {
            return Err(LexError::UnterminatedBlockComment { offset: start });
        }
        Ok(())
    }

    // ── String literals ──────────────────────────────────────────────

    fn lex_string(&mut self) -> Result<SpannedToken, LexError> {
        let start = self.pos;
        self.pos += 1; // skip opening quote
        let mut value = String::new();
        loop {
            if self.pos >= self.bytes.len() {
                return Err(LexError::UnterminatedString { offset: start });
            }
            let b = self.bytes[self.pos];
            match b {
                b'"' => {
                    self.pos += 1;
                    return Ok(SpannedToken::new(
                        Token::String(value),
                        Span::new(start, self.pos),
                    ));
                }
                b'\\' => {
                    self.pos += 1;
                    if self.pos >= self.bytes.len() {
                        return Err(LexError::UnterminatedString { offset: start });
                    }
                    let esc = self.bytes[self.pos];
                    match esc {
                        b'n' => value.push('\n'),
                        b't' => value.push('\t'),
                        b'\\' => value.push('\\'),
                        b'"' => value.push('"'),
                        b'\'' => value.push('\''),
                        _ => {
                            let ch = self.src[self.pos..].chars().next().unwrap_or('?');
                            return Err(LexError::InvalidEscape {
                                ch,
                                offset: self.pos,
                            });
                        }
                    }
                    self.pos += 1;
                }
                _ => {
                    // Regular UTF-8 character — advance by char, not byte
                    let ch = self.src[self.pos..].chars().next().unwrap_or('?');
                    value.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
    }

    // ── Backslash commands ───────────────────────────────────────────

    fn lex_backslash(&mut self, start: usize) -> Result<SpannedToken, LexError> {
        self.pos += 1; // skip '\'
        if self.pos >= self.bytes.len() {
            // Lone backslash at end → DoubleBackslash (or error)
            return Ok(SpannedToken::new(
                Token::DoubleBackslash,
                Span::new(start, self.pos),
            ));
        }
        let b = self.bytes[self.pos];
        match b {
            // Escaped operators
            b'(' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::EscapedParenOpen,
                    Span::new(start, self.pos),
                ))
            }
            b')' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::EscapedParenClose,
                    Span::new(start, self.pos),
                ))
            }
            b'!' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::EscapedExclamation,
                    Span::new(start, self.pos),
                ))
            }
            b'+' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::EscapedPlus,
                    Span::new(start, self.pos),
                ))
            }
            b'<' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::EscapedAngleOpen,
                    Span::new(start, self.pos),
                ))
            }
            b'>' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::EscapedAngleClose,
                    Span::new(start, self.pos),
                ))
            }
            b'\\' => {
                self.pos += 1;
                Ok(SpannedToken::new(
                    Token::DoubleBackslash,
                    Span::new(start, self.pos),
                ))
            }
            // Escaped word (keyword or identifier)
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let word_start = self.pos;
                self.advance_word();
                let word = &self.src[word_start..self.pos];
                let token = tokens::keyword_from_str(word)
                    .unwrap_or_else(|| Token::EscapedWord(word.to_owned()));
                Ok(SpannedToken::new(token, Span::new(start, self.pos)))
            }
            _ => {
                // Unknown escape — return as escaped word with empty name?
                // Better: return error for truly unexpected chars.
                let ch = self.src[self.pos..].chars().next().unwrap_or('?');
                Err(LexError::UnexpectedChar { ch, offset: start })
            }
        }
    }

    // ── Numbers ──────────────────────────────────────────────────────

    fn lex_number(&mut self, start: usize) -> Result<SpannedToken, LexError> {
        // Consume digits
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        // Check for decimal point → Real
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
            // Only treat as real if followed by a digit (to avoid `4.` being
            // consumed as a real when `.` is a duration dot).
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1].is_ascii_digit() {
                self.pos += 1; // skip '.'
                while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
                let text = &self.src[start..self.pos];
                let val: f64 = text.parse().unwrap_or(0.0);
                return Ok(SpannedToken::new(
                    Token::Real(val),
                    Span::new(start, self.pos),
                ));
            }
        }
        let text = &self.src[start..self.pos];
        let val: u64 = text.parse().unwrap_or(0);
        Ok(SpannedToken::new(
            Token::Unsigned(val),
            Span::new(start, self.pos),
        ))
    }

    // ── Words (note names, symbols) ──────────────────────────────────

    fn lex_word(&mut self, start: usize) -> Result<SpannedToken, LexError> {
        self.advance_word();
        let word = &self.src[start..self.pos];

        // Check if it's a note name
        if tokens::is_note_name(word) {
            return Ok(SpannedToken::new(
                Token::NoteName(word.to_owned()),
                Span::new(start, self.pos),
            ));
        }

        // Otherwise it's a symbol/identifier
        Ok(SpannedToken::new(
            Token::Symbol(word.to_owned()),
            Span::new(start, self.pos),
        ))
    }

    /// Advance `self.pos` past a contiguous word.
    ///
    /// LilyPond symbols are `[a-zA-Z]([_-][a-zA-Z]|[a-zA-Z])*` — purely
    /// alphabetic with optional underscores/hyphens between letters. Digits
    /// are NOT part of identifiers or note names; they are separate tokens
    /// (durations, numbers).
    fn advance_word(&mut self) {
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            match b {
                b'a'..=b'z' | b'A'..=b'Z' => self.pos += 1,
                b'_' | b'-' => {
                    // Underscore/hyphen is part of word only if followed by a letter
                    if self.pos + 1 < self.bytes.len()
                        && self.bytes[self.pos + 1].is_ascii_alphabetic()
                    {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: lex input and collect just the token variants.
    fn tokens(input: &str) -> Result<Vec<Token>, LexError> {
        let mut lexer = Lexer::new(input);
        let spanned = lexer.tokenize_all()?;
        Ok(spanned.into_iter().map(|s| s.token).collect())
    }

    // ── Whitespace & comments ────────────────────────────────────────

    #[test]
    fn empty_input() {
        assert_eq!(tokens("").unwrap(), vec![Token::Eof]);
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(tokens("   \n\t  ").unwrap(), vec![Token::Eof]);
    }

    #[test]
    fn line_comment() {
        let toks = tokens("% this is a comment\n").unwrap();
        assert_eq!(toks, vec![Token::Eof]);
    }

    #[test]
    fn line_comment_before_token() {
        let toks = tokens("% comment\n{").unwrap();
        assert_eq!(toks, vec![Token::BraceOpen, Token::Eof]);
    }

    #[test]
    fn double_percent_comment() {
        // %% is also a line comment in LilyPond
        let toks = tokens("%% double comment\n}").unwrap();
        assert_eq!(toks, vec![Token::BraceClose, Token::Eof]);
    }

    #[test]
    fn block_comment() {
        let toks = tokens("%{ block comment %} {").unwrap();
        assert_eq!(toks, vec![Token::BraceOpen, Token::Eof]);
    }

    #[test]
    fn nested_block_comment() {
        let toks = tokens("%{ outer %{ inner %} still comment %} {").unwrap();
        assert_eq!(toks, vec![Token::BraceOpen, Token::Eof]);
    }

    #[test]
    fn unterminated_block_comment() {
        let err = tokens("%{ no close").unwrap_err();
        assert!(matches!(err, LexError::UnterminatedBlockComment { .. }));
    }

    // ── Strings ──────────────────────────────────────────────────────

    #[test]
    fn simple_string() {
        let toks = tokens(r#""hello""#).unwrap();
        assert_eq!(toks, vec![Token::String("hello".into()), Token::Eof]);
    }

    #[test]
    fn string_with_escapes() {
        let toks = tokens(r#""line\none""#).unwrap();
        assert_eq!(toks, vec![Token::String("line\none".into()), Token::Eof]);
    }

    #[test]
    fn string_with_escaped_quote() {
        let toks = tokens(r#""say \"hi\"""#).unwrap();
        assert_eq!(toks, vec![Token::String("say \"hi\"".into()), Token::Eof]);
    }

    #[test]
    fn unterminated_string() {
        let err = tokens(r#""no close"#).unwrap_err();
        assert!(matches!(err, LexError::UnterminatedString { .. }));
    }

    // ── Numbers ──────────────────────────────────────────────────────

    #[test]
    fn unsigned_integer() {
        let toks = tokens("42").unwrap();
        assert_eq!(toks, vec![Token::Unsigned(42), Token::Eof]);
    }

    #[test]
    fn real_number() {
        let toks = tokens("2.5").unwrap();
        assert_eq!(toks, vec![Token::Real(2.5), Token::Eof]);
    }

    #[test]
    fn integer_then_dot() {
        // `4.` should be integer 4 + dot (duration dot), not a real
        let toks = tokens("4.").unwrap();
        assert_eq!(toks, vec![Token::Unsigned(4), Token::Dot, Token::Eof]);
    }

    // ── Keywords ─────────────────────────────────────────────────────

    #[test]
    fn keyword_version() {
        let toks = tokens("\\version").unwrap();
        assert_eq!(toks, vec![Token::Version, Token::Eof]);
    }

    #[test]
    fn keyword_score() {
        let toks = tokens("\\score").unwrap();
        assert_eq!(toks, vec![Token::Score, Token::Eof]);
    }

    #[test]
    fn keyword_relative() {
        let toks = tokens("\\relative").unwrap();
        assert_eq!(toks, vec![Token::Relative, Token::Eof]);
    }

    #[test]
    fn keyword_new() {
        let toks = tokens("\\new").unwrap();
        assert_eq!(toks, vec![Token::New, Token::Eof]);
    }

    #[test]
    fn escaped_word_not_keyword() {
        let toks = tokens("\\major").unwrap();
        assert_eq!(toks, vec![Token::EscapedWord("major".into()), Token::Eof]);
    }

    // ── Note names ───────────────────────────────────────────────────

    #[test]
    fn note_names_basic() {
        let toks = tokens("c d e f g a b").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::NoteName("c".into()),
                Token::NoteName("d".into()),
                Token::NoteName("e".into()),
                Token::NoteName("f".into()),
                Token::NoteName("g".into()),
                Token::NoteName("a".into()),
                Token::NoteName("b".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn note_names_accidentals() {
        let toks = tokens("cis des eis fisis geses").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::NoteName("cis".into()),
                Token::NoteName("des".into()),
                Token::NoteName("eis".into()),
                Token::NoteName("fisis".into()),
                Token::NoteName("geses".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn note_name_dutch_special() {
        // "as" = A-flat, "es" = E-flat in Dutch
        let toks = tokens("as es").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::NoteName("as".into()),
                Token::NoteName("es".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn symbol_not_note() {
        // "Staff" is a symbol, not a note name
        let toks = tokens("Staff").unwrap();
        assert_eq!(toks, vec![Token::Symbol("Staff".into()), Token::Eof]);
    }

    // ── Operators & punctuation ──────────────────────────────────────

    #[test]
    fn braces() {
        let toks = tokens("{ }").unwrap();
        assert_eq!(toks, vec![Token::BraceOpen, Token::BraceClose, Token::Eof]);
    }

    #[test]
    fn angle_brackets_single() {
        let toks = tokens("< >").unwrap();
        assert_eq!(toks, vec![Token::AngleOpen, Token::AngleClose, Token::Eof]);
    }

    #[test]
    fn angle_brackets_double() {
        let toks = tokens("<< >>").unwrap();
        assert_eq!(
            toks,
            vec![Token::DoubleAngleOpen, Token::DoubleAngleClose, Token::Eof]
        );
    }

    #[test]
    fn tie_and_pipe() {
        let toks = tokens("~ |").unwrap();
        assert_eq!(toks, vec![Token::Tilde, Token::Pipe, Token::Eof]);
    }

    #[test]
    fn octave_marks() {
        let toks = tokens("c''").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::NoteName("c".into()),
                Token::Quote,
                Token::Quote,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn octave_marks_down() {
        let toks = tokens("c,,").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::NoteName("c".into()),
                Token::Comma,
                Token::Comma,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn force_accidental() {
        let toks = tokens("cis!").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::NoteName("cis".into()),
                Token::Exclamation,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn cautionary_accidental() {
        let toks = tokens("bes?").unwrap();
        assert_eq!(
            toks,
            vec![Token::NoteName("bes".into()), Token::Question, Token::Eof,]
        );
    }

    #[test]
    fn duration_multiplier() {
        let toks = tokens("1*4").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Unsigned(1),
                Token::Star,
                Token::Unsigned(4),
                Token::Eof
            ]
        );
    }

    #[test]
    fn double_dash() {
        // In note mode, `--` is two Dash tokens (direction + tenuto abbreviation).
        // In lyric mode (Phase 20) this will become LyricHyphen.
        let toks = tokens("--").unwrap();
        assert_eq!(toks, vec![Token::Dash, Token::Dash, Token::Eof]);
    }

    #[test]
    fn double_underscore() {
        // In note mode, `__` is two Underscore tokens.
        // In lyric mode (Phase 20) this will become LyricExtender.
        let toks = tokens("__").unwrap();
        assert_eq!(toks, vec![Token::Underscore, Token::Underscore, Token::Eof]);
    }

    // ── Escaped operators ────────────────────────────────────────────

    #[test]
    fn escaped_operators() {
        let toks = tokens("\\( \\) \\! \\+ \\< \\>").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::EscapedParenOpen,
                Token::EscapedParenClose,
                Token::EscapedExclamation,
                Token::EscapedPlus,
                Token::EscapedAngleOpen,
                Token::EscapedAngleClose,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn double_backslash() {
        let toks = tokens("\\\\").unwrap();
        assert_eq!(toks, vec![Token::DoubleBackslash, Token::Eof]);
    }

    // ── Compound sequences ───────────────────────────────────────────

    #[test]
    fn version_string() {
        let toks = tokens("\\version \"2.24.0\"").unwrap();
        assert_eq!(
            toks,
            vec![Token::Version, Token::String("2.24.0".into()), Token::Eof,]
        );
    }

    #[test]
    fn score_with_note() {
        let toks = tokens("\\score { { c4 } }").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Score,
                Token::BraceOpen,
                Token::BraceOpen,
                Token::NoteName("c".into()),
                Token::Unsigned(4),
                Token::BraceClose,
                Token::BraceClose,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn relative_scale() {
        let toks = tokens("\\relative { c' d e f }").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Relative,
                Token::BraceOpen,
                Token::NoteName("c".into()),
                Token::Quote,
                Token::NoteName("d".into()),
                Token::NoteName("e".into()),
                Token::NoteName("f".into()),
                Token::BraceClose,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn chord_notation() {
        let toks = tokens("< c e g >4").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::AngleOpen,
                Token::NoteName("c".into()),
                Token::NoteName("e".into()),
                Token::NoteName("g".into()),
                Token::AngleClose,
                Token::Unsigned(4),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn key_time() {
        // \key is a music function (not a grammar keyword) → EscapedWord
        // \time is a grammar keyword → Token::Time
        let toks = tokens("\\key f \\major \\time 2/2").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::EscapedWord("key".into()),
                Token::NoteName("f".into()),
                Token::EscapedWord("major".into()),
                Token::Time,
                Token::Unsigned(2),
                Token::Slash,
                Token::Unsigned(2),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn identifier_with_underscore() {
        let toks = tokens("Bar_number_engraver").unwrap();
        assert_eq!(
            toks,
            vec![Token::Symbol("Bar_number_engraver".into()), Token::Eof]
        );
    }

    #[test]
    fn identifier_with_hyphen() {
        let toks = tokens("system-count").unwrap();
        assert_eq!(toks, vec![Token::Symbol("system-count".into()), Token::Eof]);
    }

    // ── Span tracking ────────────────────────────────────────────────

    #[test]
    fn spans_correct() {
        let mut lexer = Lexer::new("\\version \"2.24\"");
        let t1 = lexer.next_token().unwrap();
        assert_eq!(t1.span, Span::new(0, 8)); // \version
        let t2 = lexer.next_token().unwrap();
        assert_eq!(t2.span, Span::new(9, 15)); // "2.24"
    }

    // ── Full fixture: minimal score ──────────────────────────────────

    #[test]
    fn fixture_minimal_score() {
        let input = "\\version \"2.24.0\"\n\\score {\n  { c4 }\n}\n";
        let toks = tokens(input).unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Version,
                Token::String("2.24.0".into()),
                Token::Score,
                Token::BraceOpen,
                Token::BraceOpen,
                Token::NoteName("c".into()),
                Token::Unsigned(4),
                Token::BraceClose,
                Token::BraceClose,
                Token::Eof,
            ]
        );
    }

    // ── is_note_name ─────────────────────────────────────────────────

    #[test]
    fn note_name_recognition() {
        // Valid note names
        for name in &[
            "c", "d", "e", "f", "g", "a", "b", "cis", "ces", "dis", "des", "eis", "fis", "ges",
            "gis", "ais", "bes", "cisis", "deses", "as", "es", "aes", "ees",
        ] {
            assert!(tokens::is_note_name(name), "{name} should be a note name");
        }
        // Not note names
        for name in &[
            "h", "x", "Staff", "major", "relative", "rest", "cd", "ab", "bc", "ce",
        ] {
            assert!(
                !tokens::is_note_name(name),
                "{name} should NOT be a note name"
            );
        }
    }
}

//! Numeric expression parsing.
//!
//! Implements the `number_expression`, `number_term`, `number_factor`, and
//! `bare_number` productions from the LilyPond grammar.
//!
//! ```text
//! number_expression: number_term
//!                  | number_expression '+' number_term
//!                  | number_expression '-' number_term
//!
//! number_term: number_factor
//!            | number_factor '*' number_factor
//!            | number_factor '/' number_factor
//!
//! number_factor: '-' number_factor
//!              | bare_number
//!
//! bare_number: UNSIGNED | REAL
//!            | UNSIGNED NUMBER_IDENTIFIER   (e.g. 180\mm)
//!            | REAL NUMBER_IDENTIFIER       (e.g. 2.5\cm)
//! ```

use crate::lexer::Token;
use crate::model::{AssignmentValue, NumericExpression};

use super::{ParseError, Parser};

/// Known LilyPond unit suffixes (dimension keywords).
const UNITS: &[&str] = &["mm", "cm", "pt", "in", "bp", "dd", "cc", "sp"];

/// Returns `true` if the given name is a LilyPond dimension unit.
pub fn is_unit(name: &str) -> bool {
    UNITS.contains(&name)
}

impl<'src> Parser<'src> {
    /// Parse a `number_expression` (additive level).
    ///
    /// ```text
    /// number_expression: number_term (('+' | '-') number_term)*
    /// ```
    pub(super) fn parse_number_expression(&mut self) -> Result<NumericExpression, ParseError> {
        let mut lhs = self.parse_number_term()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance()?;
                    let rhs = self.parse_number_term()?;
                    lhs = NumericExpression::Add(Box::new(lhs), Box::new(rhs));
                }
                Token::Dash => {
                    self.advance()?;
                    let rhs = self.parse_number_term()?;
                    lhs = NumericExpression::Sub(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    /// Parse a `number_term` (multiplicative level).
    ///
    /// ```text
    /// number_term: number_factor (('*' | '/') number_factor)*
    /// ```
    pub(super) fn parse_number_term(&mut self) -> Result<NumericExpression, ParseError> {
        let mut lhs = self.parse_number_factor()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance()?;
                    let rhs = self.parse_number_factor()?;
                    lhs = NumericExpression::Mul(Box::new(lhs), Box::new(rhs));
                }
                Token::Slash => {
                    self.advance()?;
                    let rhs = self.parse_number_factor()?;
                    lhs = NumericExpression::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    /// Parse a `number_factor`.
    ///
    /// ```text
    /// number_factor: '-' number_factor | bare_number
    /// ```
    pub(super) fn parse_number_factor(&mut self) -> Result<NumericExpression, ParseError> {
        if *self.peek() == Token::Dash {
            self.advance()?;
            let inner = self.parse_number_factor()?;
            return Ok(NumericExpression::Negate(Box::new(inner)));
        }
        self.parse_bare_number()
    }

    /// Parse a `bare_number`: literal with optional unit suffix.
    ///
    /// ```text
    /// bare_number: UNSIGNED [UNIT] | REAL [UNIT]
    /// ```
    pub(super) fn parse_bare_number(&mut self) -> Result<NumericExpression, ParseError> {
        let n = match self.peek() {
            Token::Unsigned(v) => {
                let v = *v as f64;
                self.advance()?;
                v
            }
            Token::Real(v) => {
                let v = *v;
                self.advance()?;
                v
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "number in numeric expression".into(),
                });
            }
        };
        // Check for unit suffix: \mm, \cm, \pt, etc.
        if let Token::EscapedWord(word) = self.peek()
            && is_unit(word)
        {
            let unit = word.clone();
            self.advance()?;
            return Ok(NumericExpression::WithUnit(n, unit));
        }
        Ok(NumericExpression::Literal(n))
    }

    /// Check whether the current position looks like the start of a numeric
    /// expression that goes beyond a simple literal number.
    ///
    /// Called after consuming a number token. Returns `true` if the next token
    /// is a unit suffix or an arithmetic operator.
    pub(super) fn at_numeric_expression_continuation(&self) -> bool {
        match self.peek() {
            Token::Plus | Token::Star | Token::Dash | Token::Slash => true,
            Token::EscapedWord(word) => is_unit(word),
            _ => false,
        }
    }

    /// Parse a number in assignment context — plain `Number` or `NumericExpression`.
    ///
    /// Peeks after the first number to decide: if followed by a unit suffix
    /// (`\mm`, etc.) or arithmetic operator (`+`, `-`, `*`, `/`), delegates to
    /// full expression parsing. Otherwise returns a simple `AssignmentValue::Number`.
    pub(super) fn parse_assignment_number(&mut self) -> Result<AssignmentValue, ParseError> {
        // Consume the leading number
        let n = match self.peek() {
            Token::Unsigned(v) => {
                let v = *v as f64;
                self.advance()?;
                v
            }
            Token::Real(v) => {
                let v = *v;
                self.advance()?;
                v
            }
            _ => unreachable!(),
        };

        // Check for continuation (unit or operator)
        if self.at_numeric_expression_continuation() {
            // Build the first factor (number ± unit) and feed into expression parsing
            let first = if let Token::EscapedWord(word) = self.peek()
                && is_unit(word)
            {
                let unit = word.clone();
                self.advance()?;
                NumericExpression::WithUnit(n, unit)
            } else {
                NumericExpression::Literal(n)
            };

            // Continue with operator-level parsing
            let expr = self.continue_number_expression(first)?;
            Ok(AssignmentValue::NumericExpression(expr))
        } else {
            Ok(AssignmentValue::Number(n))
        }
    }

    /// Continue parsing a `number_expression` given an already-parsed left operand.
    ///
    /// Handles the additive and multiplicative operators that follow the first term.
    fn continue_number_expression(
        &mut self,
        first: NumericExpression,
    ) -> Result<NumericExpression, ParseError> {
        // First, handle any multiplicative operators on the first factor
        let mut lhs = self.continue_number_term(first)?;

        // Then handle additive operators
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance()?;
                    let rhs = self.parse_number_term()?;
                    lhs = NumericExpression::Add(Box::new(lhs), Box::new(rhs));
                }
                Token::Dash => {
                    self.advance()?;
                    let rhs = self.parse_number_term()?;
                    lhs = NumericExpression::Sub(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    /// Continue parsing a `number_term` given an already-parsed left factor.
    fn continue_number_term(
        &mut self,
        first: NumericExpression,
    ) -> Result<NumericExpression, ParseError> {
        let mut lhs = first;
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance()?;
                    let rhs = self.parse_number_factor()?;
                    lhs = NumericExpression::Mul(Box::new(lhs), Box::new(rhs));
                }
                Token::Slash => {
                    self.advance()?;
                    let rhs = self.parse_number_factor()?;
                    lhs = NumericExpression::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
}

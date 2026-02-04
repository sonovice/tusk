//! Notation data types.
//!
//! Types for representing musical notation elements like beams, tremolos, trills,
//! state indicators (start/stop), and level/number types.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::ParseError;

// ============================================================================
// State Types (start/stop/continue)
// ============================================================================

/// Indicates start or stop of an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartStop {
    Start,
    Stop,
}

impl fmt::Display for StartStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartStop::Start => write!(f, "start"),
            StartStop::Stop => write!(f, "stop"),
        }
    }
}

impl FromStr for StartStop {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(StartStop::Start),
            "stop" => Ok(StartStop::Stop),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates start, stop, or continue of an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartStopContinue {
    Start,
    Stop,
    Continue,
}

impl fmt::Display for StartStopContinue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartStopContinue::Start => write!(f, "start"),
            StartStopContinue::Stop => write!(f, "stop"),
            StartStopContinue::Continue => write!(f, "continue"),
        }
    }
}

impl FromStr for StartStopContinue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(StartStopContinue::Start),
            "stop" => Ok(StartStopContinue::Stop),
            "continue" => Ok(StartStopContinue::Continue),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates start, stop, single, or continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartStopSingle {
    Start,
    Stop,
    Single,
}

impl fmt::Display for StartStopSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartStopSingle::Start => write!(f, "start"),
            StartStopSingle::Stop => write!(f, "stop"),
            StartStopSingle::Single => write!(f, "single"),
        }
    }
}

impl FromStr for StartStopSingle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(StartStopSingle::Start),
            "stop" => Ok(StartStopSingle::Stop),
            "single" => Ok(StartStopSingle::Single),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Start, stop, discontinue for barlines and endings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartStopDiscontinue {
    Start,
    Stop,
    Discontinue,
}

impl fmt::Display for StartStopDiscontinue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartStopDiscontinue::Start => write!(f, "start"),
            StartStopDiscontinue::Stop => write!(f, "stop"),
            StartStopDiscontinue::Discontinue => write!(f, "discontinue"),
        }
    }
}

impl FromStr for StartStopDiscontinue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(StartStopDiscontinue::Start),
            "stop" => Ok(StartStopDiscontinue::Stop),
            "discontinue" => Ok(StartStopDiscontinue::Discontinue),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Start, stop, change, continue for pedals and other direction types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartStopChangeContinue {
    Start,
    Stop,
    Change,
    Continue,
}

impl fmt::Display for StartStopChangeContinue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartStopChangeContinue::Start => write!(f, "start"),
            StartStopChangeContinue::Stop => write!(f, "stop"),
            StartStopChangeContinue::Change => write!(f, "change"),
            StartStopChangeContinue::Continue => write!(f, "continue"),
        }
    }
}

impl FromStr for StartStopChangeContinue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(StartStopChangeContinue::Start),
            "stop" => Ok(StartStopChangeContinue::Stop),
            "change" => Ok(StartStopChangeContinue::Change),
            "continue" => Ok(StartStopChangeContinue::Continue),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

// ============================================================================
// Level/Number Types
// ============================================================================

/// Beam level (1-8) identifying concurrent beams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BeamLevel(pub u8);

impl BeamLevel {
    pub fn new(value: u8) -> Option<Self> {
        if (1..=8).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for BeamLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BeamLevel {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Number level (1-6) for distinguishing overlapping elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NumberLevel(pub u8);

impl NumberLevel {
    pub fn new(value: u8) -> Option<Self> {
        if (1..=6).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for NumberLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NumberLevel {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Number of staff lines (0-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NumberOfLines(pub u8);

impl NumberOfLines {
    pub fn new(value: u8) -> Option<Self> {
        if value <= 3 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for NumberOfLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NumberOfLines {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Staff line number (positive integer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StaffLine(pub u32);

impl StaffLine {
    pub fn new(value: u32) -> Option<Self> {
        if value > 0 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for StaffLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StaffLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Staff number (positive integer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StaffNumber(pub u32);

impl StaffNumber {
    pub fn new(value: u32) -> Option<Self> {
        if value > 0 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for StaffNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StaffNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// String number (positive integer for string instruments).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringNumber(pub u32);

impl StringNumber {
    pub fn new(value: u32) -> Option<Self> {
        if value > 0 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for StringNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StringNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

// ============================================================================
// Notation-Specific Types
// ============================================================================

/// Tremolo marks (0-8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TremoloMarks(pub u8);

impl TremoloMarks {
    pub fn new(value: u8) -> Option<Self> {
        if value <= 8 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for TremoloMarks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TremoloMarks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Trill beats (minimum 2).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrillBeats(pub f64);

impl TrillBeats {
    pub fn new(value: f64) -> Option<Self> {
        if value >= 2.0 {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for TrillBeats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TrillBeats {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_stop_parse() {
        assert_eq!(StartStop::from_str("start").unwrap(), StartStop::Start);
        assert_eq!(StartStop::from_str("stop").unwrap(), StartStop::Stop);
    }

    #[test]
    fn test_start_stop_continue_parse() {
        assert_eq!(
            StartStopContinue::from_str("start").unwrap(),
            StartStopContinue::Start
        );
        assert_eq!(
            StartStopContinue::from_str("stop").unwrap(),
            StartStopContinue::Stop
        );
        assert_eq!(
            StartStopContinue::from_str("continue").unwrap(),
            StartStopContinue::Continue
        );
    }

    #[test]
    fn test_beam_level() {
        assert!(BeamLevel::new(1).is_some());
        assert!(BeamLevel::new(8).is_some());
        assert!(BeamLevel::new(0).is_none());
        assert!(BeamLevel::new(9).is_none());
    }

    #[test]
    fn test_number_level() {
        assert!(NumberLevel::new(1).is_some());
        assert!(NumberLevel::new(6).is_some());
        assert!(NumberLevel::new(0).is_none());
        assert!(NumberLevel::new(7).is_none());
    }

    #[test]
    fn test_staff_number() {
        assert!(StaffNumber::new(1).is_some());
        assert!(StaffNumber::new(0).is_none());
    }

    #[test]
    fn test_tremolo_marks() {
        assert!(TremoloMarks::new(0).is_some());
        assert!(TremoloMarks::new(8).is_some());
        assert!(TremoloMarks::new(9).is_none());
    }

    #[test]
    fn test_trill_beats() {
        assert!(TrillBeats::new(2.0).is_some());
        assert!(TrillBeats::new(1.9).is_none());
    }
}

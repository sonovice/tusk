//! Pitch and duration data types.
//!
//! Types for representing pitch, octave, duration, and related musical values.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::ParseError;

// ============================================================================
// Numeric Types (Duration-related)
// ============================================================================

/// Musical divisions per quarter note for duration calculations.
/// Represented as a decimal but should preferably be an integer.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Divisions(pub f64);

impl Divisions {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Divisions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display without trailing zeros for integer values
        if self.0.fract() == 0.0 {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl FromStr for Divisions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>()
            .map(Divisions)
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))
    }
}

/// Positive divisions value (must be > 0).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PositiveDivisions(pub f64);

impl PositiveDivisions {
    pub fn new(value: f64) -> Option<Self> {
        if value > 0.0 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for PositiveDivisions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.fract() == 0.0 {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl FromStr for PositiveDivisions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

// ============================================================================
// Music-Specific Types (Pitch-related)
// ============================================================================

/// Octave values (0-9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Octave(pub u8);

impl Octave {
    pub fn new(value: u8) -> Option<Self> {
        if value <= 9 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Octave {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Semitones for chromatic alteration (can be fractional for microtones).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Semitones(pub f64);

impl Semitones {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Semitones {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.fract() == 0.0 {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl FromStr for Semitones {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>()
            .map(Semitones)
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))
    }
}

/// Step in the diatonic scale (C, D, E, F, G, A, B).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::A => write!(f, "A"),
            Step::B => write!(f, "B"),
            Step::C => write!(f, "C"),
            Step::D => write!(f, "D"),
            Step::E => write!(f, "E"),
            Step::F => write!(f, "F"),
            Step::G => write!(f, "G"),
        }
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Step::A),
            "B" => Ok(Step::B),
            "C" => Ok(Step::C),
            "D" => Ok(Step::D),
            "E" => Ok(Step::E),
            "F" => Ok(Step::F),
            "G" => Ok(Step::G),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Fifths on the circle of fifths for key signatures (-7 to 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Fifths(pub i8);

impl Fifths {
    pub fn new(value: i8) -> Self {
        Self(value)
    }
}

impl fmt::Display for Fifths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Fifths {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i8>()
            .map(Fifths)
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))
    }
}

// ============================================================================
// MIDI Types
// ============================================================================

/// MIDI 1.0 values ranging from 1 to 16 (channels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Midi16(pub u8);

impl Midi16 {
    pub fn new(value: u8) -> Option<Self> {
        if (1..=16).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for Midi16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Midi16 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// MIDI 1.0 values ranging from 1 to 128 (programs, notes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Midi128(pub u8);

impl Midi128 {
    pub fn new(value: u8) -> Option<Self> {
        if (1..=128).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for Midi128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Midi128 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// MIDI 1.0 values ranging from 1 to 16384 (banks).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Midi16384(pub u16);

impl Midi16384 {
    pub fn new(value: u16) -> Option<Self> {
        if (1..=16384).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for Midi16384 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Midi16384 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<u16>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisions_parse() {
        assert_eq!(Divisions::from_str("4").unwrap().0, 4.0);
        assert_eq!(Divisions::from_str("1.5").unwrap().0, 1.5);
    }

    #[test]
    fn test_divisions_display() {
        assert_eq!(Divisions(4.0).to_string(), "4");
        assert_eq!(Divisions(1.5).to_string(), "1.5");
    }

    #[test]
    fn test_positive_divisions() {
        assert!(PositiveDivisions::new(1.0).is_some());
        assert!(PositiveDivisions::new(0.0).is_none());
        assert!(PositiveDivisions::new(-1.0).is_none());
    }

    #[test]
    fn test_octave() {
        assert!(Octave::new(0).is_some());
        assert!(Octave::new(9).is_some());
        assert!(Octave::new(10).is_none());
    }

    #[test]
    fn test_step_parse() {
        assert_eq!(Step::from_str("C").unwrap(), Step::C);
        assert_eq!(Step::from_str("D").unwrap(), Step::D);
        assert_eq!(Step::from_str("E").unwrap(), Step::E);
        assert_eq!(Step::from_str("F").unwrap(), Step::F);
        assert_eq!(Step::from_str("G").unwrap(), Step::G);
        assert_eq!(Step::from_str("A").unwrap(), Step::A);
        assert_eq!(Step::from_str("B").unwrap(), Step::B);
        assert!(Step::from_str("H").is_err());
    }

    #[test]
    fn test_step_display() {
        assert_eq!(Step::C.to_string(), "C");
        assert_eq!(Step::G.to_string(), "G");
    }

    #[test]
    fn test_fifths() {
        assert_eq!(Fifths::from_str("-7").unwrap().0, -7);
        assert_eq!(Fifths::from_str("0").unwrap().0, 0);
        assert_eq!(Fifths::from_str("7").unwrap().0, 7);
    }

    #[test]
    fn test_semitones() {
        assert_eq!(Semitones::from_str("-1").unwrap().0, -1.0);
        assert_eq!(Semitones::from_str("0.5").unwrap().0, 0.5);
    }

    #[test]
    fn test_midi16() {
        assert!(Midi16::new(1).is_some());
        assert!(Midi16::new(16).is_some());
        assert!(Midi16::new(0).is_none());
        assert!(Midi16::new(17).is_none());
    }

    #[test]
    fn test_midi128() {
        assert!(Midi128::new(1).is_some());
        assert!(Midi128::new(128).is_some());
        assert!(Midi128::new(0).is_none());
        assert!(Midi128::new(129).is_none());
    }

    #[test]
    fn test_midi16384() {
        assert!(Midi16384::new(1).is_some());
        assert!(Midi16384::new(16384).is_some());
        assert!(Midi16384::new(0).is_none());
        assert!(Midi16384::new(16385).is_none());
    }
}

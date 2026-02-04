//! MusicXML 4.0 data types (simple types from the XSD schema).
//!
//! These types represent the fundamental data types used throughout MusicXML.
//! They map to xs:simpleType definitions in the MusicXML XSD.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

// ============================================================================
// Positional/Directional Types
// ============================================================================

/// Indicates whether an element appears above or below another element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AboveBelow {
    Above,
    Below,
}

impl fmt::Display for AboveBelow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AboveBelow::Above => write!(f, "above"),
            AboveBelow::Below => write!(f, "below"),
        }
    }
}

impl FromStr for AboveBelow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "above" => Ok(AboveBelow::Above),
            "below" => Ok(AboveBelow::Below),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates whether an element appears to the left or right of another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LeftRight {
    Left,
    Right,
}

impl fmt::Display for LeftRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeftRight::Left => write!(f, "left"),
            LeftRight::Right => write!(f, "right"),
        }
    }
}

impl FromStr for LeftRight {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(LeftRight::Left),
            "right" => Ok(LeftRight::Right),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Defines horizontal alignment and text justification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LeftCenterRight {
    Left,
    Center,
    Right,
}

impl fmt::Display for LeftCenterRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeftCenterRight::Left => write!(f, "left"),
            LeftCenterRight::Center => write!(f, "center"),
            LeftCenterRight::Right => write!(f, "right"),
        }
    }
}

impl FromStr for LeftCenterRight {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(LeftCenterRight::Left),
            "center" => Ok(LeftCenterRight::Center),
            "right" => Ok(LeftCenterRight::Right),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates whether something is above, below, or in the middle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OverUnder {
    Over,
    Under,
}

impl fmt::Display for OverUnder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OverUnder::Over => write!(f, "over"),
            OverUnder::Under => write!(f, "under"),
        }
    }
}

impl FromStr for OverUnder {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "over" => Ok(OverUnder::Over),
            "under" => Ok(OverUnder::Under),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates whether something is at the top or bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TopBottom {
    Top,
    Bottom,
}

impl fmt::Display for TopBottom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopBottom::Top => write!(f, "top"),
            TopBottom::Bottom => write!(f, "bottom"),
        }
    }
}

impl FromStr for TopBottom {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(TopBottom::Top),
            "bottom" => Ok(TopBottom::Bottom),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates upward or downward direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpDown {
    Up,
    Down,
}

impl fmt::Display for UpDown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpDown::Up => write!(f, "up"),
            UpDown::Down => write!(f, "down"),
        }
    }
}

impl FromStr for UpDown {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(UpDown::Up),
            "down" => Ok(UpDown::Down),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Indicates upright or inverted orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UprightInverted {
    Upright,
    Inverted,
}

impl fmt::Display for UprightInverted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UprightInverted::Upright => write!(f, "upright"),
            UprightInverted::Inverted => write!(f, "inverted"),
        }
    }
}

impl FromStr for UprightInverted {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "upright" => Ok(UprightInverted::Upright),
            "inverted" => Ok(UprightInverted::Inverted),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Vertical alignment values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Valign {
    Top,
    Middle,
    Bottom,
    Baseline,
}

impl fmt::Display for Valign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Valign::Top => write!(f, "top"),
            Valign::Middle => write!(f, "middle"),
            Valign::Bottom => write!(f, "bottom"),
            Valign::Baseline => write!(f, "baseline"),
        }
    }
}

impl FromStr for Valign {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(Valign::Top),
            "middle" => Ok(Valign::Middle),
            "bottom" => Ok(Valign::Bottom),
            "baseline" => Ok(Valign::Baseline),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Vertical alignment values for images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValignImage {
    Top,
    Middle,
    Bottom,
}

impl fmt::Display for ValignImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValignImage::Top => write!(f, "top"),
            ValignImage::Middle => write!(f, "middle"),
            ValignImage::Bottom => write!(f, "bottom"),
        }
    }
}

impl FromStr for ValignImage {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(ValignImage::Top),
            "middle" => Ok(ValignImage::Middle),
            "bottom" => Ok(ValignImage::Bottom),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

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
// Yes/No Types
// ============================================================================

/// Boolean-like type using "yes" and "no" strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
    Yes,
    No,
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YesNo::Yes => write!(f, "yes"),
            YesNo::No => write!(f, "no"),
        }
    }
}

impl FromStr for YesNo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(YesNo::Yes),
            "no" => Ok(YesNo::No),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

impl From<bool> for YesNo {
    fn from(b: bool) -> Self {
        if b { YesNo::Yes } else { YesNo::No }
    }
}

impl From<YesNo> for bool {
    fn from(yn: YesNo) -> Self {
        matches!(yn, YesNo::Yes)
    }
}

// ============================================================================
// Numeric Types
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

/// Non-negative decimal value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonNegativeDecimal(pub f64);

impl NonNegativeDecimal {
    pub fn new(value: f64) -> Option<Self> {
        if value >= 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for NonNegativeDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NonNegativeDecimal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Positive decimal value (must be > 0).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PositiveDecimal(pub f64);

impl PositiveDecimal {
    pub fn new(value: f64) -> Option<Self> {
        if value > 0.0 { Some(Self(value)) } else { None }
    }
}

impl fmt::Display for PositiveDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PositiveDecimal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Percentage value (0 to 100).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Percent(pub f64);

impl Percent {
    pub fn new(value: f64) -> Option<Self> {
        if (0.0..=100.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Percent {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Tenths of interline staff space for positioning.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Tenths(pub f64);

impl Tenths {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Tenths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Tenths {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>()
            .map(Tenths)
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))
    }
}

/// Millimeters for page layout.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Millimeters(pub f64);

impl Millimeters {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Millimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Millimeters {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>()
            .map(Millimeters)
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))
    }
}

/// Milliseconds for timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Milliseconds(pub u32);

impl Milliseconds {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Milliseconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Milliseconds {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>()
            .map(Milliseconds)
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))
    }
}

/// Rotation in degrees (-180 to 180).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RotationDegrees(pub f64);

impl RotationDegrees {
    pub fn new(value: f64) -> Option<Self> {
        if (-180.0..=180.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl fmt::Display for RotationDegrees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RotationDegrees {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
        Self::new(value).ok_or(ParseError::InvalidValue(s.to_string()))
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
// Music-Specific Types
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

// ============================================================================
// Visual/Style Types
// ============================================================================

/// Color in hexadecimal RGB or ARGB format (e.g., "#800080" or "#40800080").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Color(pub String);

impl Color {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let s = value.into();
        // Validate format: # followed by 6 or 8 hex digits
        if s.starts_with('#') && (s.len() == 7 || s.len() == 9) {
            let hex_part = &s[1..];
            if hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(Self(s));
            }
        }
        None
    }

    /// Creates a color from RGB values (0-255).
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(format!("#{:02X}{:02X}{:02X}", r, g, b))
    }

    /// Creates a color from ARGB values (0-255).
    pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s).ok_or(ParseError::InvalidValue(s.to_string()))
    }
}

/// Font style (normal or italic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    Normal,
    Italic,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontStyle::Normal => write!(f, "normal"),
            FontStyle::Italic => write!(f, "italic"),
        }
    }
}

impl FromStr for FontStyle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(FontStyle::Normal),
            "italic" => Ok(FontStyle::Italic),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Font weight (normal or bold).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
    Normal,
    Bold,
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontWeight::Normal => write!(f, "normal"),
            FontWeight::Bold => write!(f, "bold"),
        }
    }
}

impl FromStr for FontWeight {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(FontWeight::Normal),
            "bold" => Ok(FontWeight::Bold),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// CSS font sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
}

impl fmt::Display for CssFontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CssFontSize::XxSmall => write!(f, "xx-small"),
            CssFontSize::XSmall => write!(f, "x-small"),
            CssFontSize::Small => write!(f, "small"),
            CssFontSize::Medium => write!(f, "medium"),
            CssFontSize::Large => write!(f, "large"),
            CssFontSize::XLarge => write!(f, "x-large"),
            CssFontSize::XxLarge => write!(f, "xx-large"),
        }
    }
}

impl FromStr for CssFontSize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xx-small" => Ok(CssFontSize::XxSmall),
            "x-small" => Ok(CssFontSize::XSmall),
            "small" => Ok(CssFontSize::Small),
            "medium" => Ok(CssFontSize::Medium),
            "large" => Ok(CssFontSize::Large),
            "x-large" => Ok(CssFontSize::XLarge),
            "xx-large" => Ok(CssFontSize::XxLarge),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Font size - either CSS size or numeric point size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FontSize {
    Css(CssFontSize),
    Points(f64),
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSize::Css(size) => write!(f, "{}", size),
            FontSize::Points(pts) => write!(f, "{}", pts),
        }
    }
}

impl FromStr for FontSize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try CSS size first
        if let Ok(css) = CssFontSize::from_str(s) {
            return Ok(FontSize::Css(css));
        }
        // Then try numeric
        s.parse::<f64>()
            .map(FontSize::Points)
            .map_err(|_| ParseError::InvalidValue(s.to_string()))
    }
}

/// Line shape (straight or curved).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineShape {
    Straight,
    Curved,
}

impl fmt::Display for LineShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineShape::Straight => write!(f, "straight"),
            LineShape::Curved => write!(f, "curved"),
        }
    }
}

impl FromStr for LineShape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "straight" => Ok(LineShape::Straight),
            "curved" => Ok(LineShape::Curved),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Line type (solid, dashed, dotted, wavy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineType {
    Solid,
    Dashed,
    Dotted,
    Wavy,
}

impl fmt::Display for LineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineType::Solid => write!(f, "solid"),
            LineType::Dashed => write!(f, "dashed"),
            LineType::Dotted => write!(f, "dotted"),
            LineType::Wavy => write!(f, "wavy"),
        }
    }
}

impl FromStr for LineType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "solid" => Ok(LineType::Solid),
            "dashed" => Ok(LineType::Dashed),
            "dotted" => Ok(LineType::Dotted),
            "wavy" => Ok(LineType::Wavy),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Line length (short, medium, long).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineLength {
    Short,
    Medium,
    Long,
}

impl fmt::Display for LineLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineLength::Short => write!(f, "short"),
            LineLength::Medium => write!(f, "medium"),
            LineLength::Long => write!(f, "long"),
        }
    }
}

impl FromStr for LineLength {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "short" => Ok(LineLength::Short),
            "medium" => Ok(LineLength::Medium),
            "long" => Ok(LineLength::Long),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Symbol size (cue, grace-cue, large, full).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolSize {
    /// Full-size symbol (default for regular notes).
    Full,
    /// Cue-size symbol.
    Cue,
    /// Grace-cue-size symbol.
    GraceCue,
    /// Large symbol.
    Large,
}

impl fmt::Display for SymbolSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolSize::Full => write!(f, "full"),
            SymbolSize::Cue => write!(f, "cue"),
            SymbolSize::GraceCue => write!(f, "grace-cue"),
            SymbolSize::Large => write!(f, "large"),
        }
    }
}

impl FromStr for SymbolSize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(SymbolSize::Full),
            "cue" => Ok(SymbolSize::Cue),
            "grace-cue" => Ok(SymbolSize::GraceCue),
            "large" => Ok(SymbolSize::Large),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Text direction (ltr or rtl).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextDirection {
    Ltr,
    Rtl,
}

impl fmt::Display for TextDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextDirection::Ltr => write!(f, "ltr"),
            TextDirection::Rtl => write!(f, "rtl"),
        }
    }
}

impl FromStr for TextDirection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ltr" => Ok(TextDirection::Ltr),
            "rtl" => Ok(TextDirection::Rtl),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

/// Enclosure shape around text or symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EnclosureShape {
    Rectangle,
    Square,
    Oval,
    Circle,
    Bracket,
    InvertedBracket,
    Triangle,
    Diamond,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Nonagon,
    Decagon,
    None,
}

impl fmt::Display for EnclosureShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnclosureShape::Rectangle => write!(f, "rectangle"),
            EnclosureShape::Square => write!(f, "square"),
            EnclosureShape::Oval => write!(f, "oval"),
            EnclosureShape::Circle => write!(f, "circle"),
            EnclosureShape::Bracket => write!(f, "bracket"),
            EnclosureShape::InvertedBracket => write!(f, "inverted-bracket"),
            EnclosureShape::Triangle => write!(f, "triangle"),
            EnclosureShape::Diamond => write!(f, "diamond"),
            EnclosureShape::Pentagon => write!(f, "pentagon"),
            EnclosureShape::Hexagon => write!(f, "hexagon"),
            EnclosureShape::Heptagon => write!(f, "heptagon"),
            EnclosureShape::Octagon => write!(f, "octagon"),
            EnclosureShape::Nonagon => write!(f, "nonagon"),
            EnclosureShape::Decagon => write!(f, "decagon"),
            EnclosureShape::None => write!(f, "none"),
        }
    }
}

impl FromStr for EnclosureShape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rectangle" => Ok(EnclosureShape::Rectangle),
            "square" => Ok(EnclosureShape::Square),
            "oval" => Ok(EnclosureShape::Oval),
            "circle" => Ok(EnclosureShape::Circle),
            "bracket" => Ok(EnclosureShape::Bracket),
            "inverted-bracket" => Ok(EnclosureShape::InvertedBracket),
            "triangle" => Ok(EnclosureShape::Triangle),
            "diamond" => Ok(EnclosureShape::Diamond),
            "pentagon" => Ok(EnclosureShape::Pentagon),
            "hexagon" => Ok(EnclosureShape::Hexagon),
            "heptagon" => Ok(EnclosureShape::Heptagon),
            "octagon" => Ok(EnclosureShape::Octagon),
            "nonagon" => Ok(EnclosureShape::Nonagon),
            "decagon" => Ok(EnclosureShape::Decagon),
            "none" => Ok(EnclosureShape::None),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

// ============================================================================
// Error Type
// ============================================================================

/// Error type for parsing MusicXML data types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The value string couldn't be parsed.
    InvalidValue(String),
    /// A numeric value couldn't be parsed.
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidValue(s) => write!(f, "invalid value: {}", s),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Positional/Directional Types Tests
    // ========================================================================

    #[test]
    fn test_above_below_parse() {
        assert_eq!(AboveBelow::from_str("above").unwrap(), AboveBelow::Above);
        assert_eq!(AboveBelow::from_str("below").unwrap(), AboveBelow::Below);
        assert!(AboveBelow::from_str("invalid").is_err());
    }

    #[test]
    fn test_above_below_display() {
        assert_eq!(AboveBelow::Above.to_string(), "above");
        assert_eq!(AboveBelow::Below.to_string(), "below");
    }

    #[test]
    fn test_left_right_parse() {
        assert_eq!(LeftRight::from_str("left").unwrap(), LeftRight::Left);
        assert_eq!(LeftRight::from_str("right").unwrap(), LeftRight::Right);
        assert!(LeftRight::from_str("center").is_err());
    }

    #[test]
    fn test_left_center_right_parse() {
        assert_eq!(
            LeftCenterRight::from_str("left").unwrap(),
            LeftCenterRight::Left
        );
        assert_eq!(
            LeftCenterRight::from_str("center").unwrap(),
            LeftCenterRight::Center
        );
        assert_eq!(
            LeftCenterRight::from_str("right").unwrap(),
            LeftCenterRight::Right
        );
    }

    #[test]
    fn test_up_down_parse() {
        assert_eq!(UpDown::from_str("up").unwrap(), UpDown::Up);
        assert_eq!(UpDown::from_str("down").unwrap(), UpDown::Down);
    }

    #[test]
    fn test_valign_parse() {
        assert_eq!(Valign::from_str("top").unwrap(), Valign::Top);
        assert_eq!(Valign::from_str("middle").unwrap(), Valign::Middle);
        assert_eq!(Valign::from_str("bottom").unwrap(), Valign::Bottom);
        assert_eq!(Valign::from_str("baseline").unwrap(), Valign::Baseline);
    }

    // ========================================================================
    // State Types Tests
    // ========================================================================

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

    // ========================================================================
    // Yes/No Tests
    // ========================================================================

    #[test]
    fn test_yes_no_parse() {
        assert_eq!(YesNo::from_str("yes").unwrap(), YesNo::Yes);
        assert_eq!(YesNo::from_str("no").unwrap(), YesNo::No);
    }

    #[test]
    fn test_yes_no_bool_conversion() {
        assert!(bool::from(YesNo::Yes));
        assert!(!bool::from(YesNo::No));
        assert_eq!(YesNo::from(true), YesNo::Yes);
        assert_eq!(YesNo::from(false), YesNo::No);
    }

    // ========================================================================
    // Numeric Types Tests
    // ========================================================================

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
    fn test_percent() {
        assert!(Percent::new(50.0).is_some());
        assert!(Percent::new(0.0).is_some());
        assert!(Percent::new(100.0).is_some());
        assert!(Percent::new(-1.0).is_none());
        assert!(Percent::new(101.0).is_none());
    }

    #[test]
    fn test_rotation_degrees() {
        assert!(RotationDegrees::new(0.0).is_some());
        assert!(RotationDegrees::new(180.0).is_some());
        assert!(RotationDegrees::new(-180.0).is_some());
        assert!(RotationDegrees::new(181.0).is_none());
    }

    // ========================================================================
    // MIDI Types Tests
    // ========================================================================

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

    // ========================================================================
    // Level/Number Types Tests
    // ========================================================================

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

    // ========================================================================
    // Music-Specific Types Tests
    // ========================================================================

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

    // ========================================================================
    // Visual/Style Types Tests
    // ========================================================================

    #[test]
    fn test_color() {
        assert!(Color::new("#FF0000").is_some());
        assert!(Color::new("#80FF00FF").is_some());
        assert!(Color::new("#GG0000").is_none());
        assert!(Color::new("FF0000").is_none());
        assert!(Color::new("#FF00").is_none());
    }

    #[test]
    fn test_color_from_rgb() {
        let color = Color::from_rgb(255, 0, 128);
        assert_eq!(color.to_string(), "#FF0080");
    }

    #[test]
    fn test_color_from_argb() {
        let color = Color::from_argb(128, 255, 0, 128);
        assert_eq!(color.to_string(), "#80FF0080");
    }

    #[test]
    fn test_font_style() {
        assert_eq!(FontStyle::from_str("normal").unwrap(), FontStyle::Normal);
        assert_eq!(FontStyle::from_str("italic").unwrap(), FontStyle::Italic);
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::from_str("normal").unwrap(), FontWeight::Normal);
        assert_eq!(FontWeight::from_str("bold").unwrap(), FontWeight::Bold);
    }

    #[test]
    fn test_css_font_size() {
        assert_eq!(
            CssFontSize::from_str("xx-small").unwrap(),
            CssFontSize::XxSmall
        );
        assert_eq!(
            CssFontSize::from_str("medium").unwrap(),
            CssFontSize::Medium
        );
        assert_eq!(
            CssFontSize::from_str("xx-large").unwrap(),
            CssFontSize::XxLarge
        );
    }

    #[test]
    fn test_font_size() {
        assert!(matches!(
            FontSize::from_str("medium").unwrap(),
            FontSize::Css(CssFontSize::Medium)
        ));
        assert!(matches!(
            FontSize::from_str("12").unwrap(),
            FontSize::Points(12.0)
        ));
        assert!(matches!(
            FontSize::from_str("12.5").unwrap(),
            FontSize::Points(12.5)
        ));
    }

    #[test]
    fn test_line_shape() {
        assert_eq!(
            LineShape::from_str("straight").unwrap(),
            LineShape::Straight
        );
        assert_eq!(LineShape::from_str("curved").unwrap(), LineShape::Curved);
    }

    #[test]
    fn test_line_type() {
        assert_eq!(LineType::from_str("solid").unwrap(), LineType::Solid);
        assert_eq!(LineType::from_str("dashed").unwrap(), LineType::Dashed);
        assert_eq!(LineType::from_str("dotted").unwrap(), LineType::Dotted);
        assert_eq!(LineType::from_str("wavy").unwrap(), LineType::Wavy);
    }

    #[test]
    fn test_symbol_size() {
        assert_eq!(SymbolSize::from_str("full").unwrap(), SymbolSize::Full);
        assert_eq!(SymbolSize::from_str("cue").unwrap(), SymbolSize::Cue);
        assert_eq!(
            SymbolSize::from_str("grace-cue").unwrap(),
            SymbolSize::GraceCue
        );
        assert_eq!(SymbolSize::from_str("large").unwrap(), SymbolSize::Large);
    }

    #[test]
    fn test_enclosure_shape() {
        assert_eq!(
            EnclosureShape::from_str("rectangle").unwrap(),
            EnclosureShape::Rectangle
        );
        assert_eq!(
            EnclosureShape::from_str("circle").unwrap(),
            EnclosureShape::Circle
        );
        assert_eq!(
            EnclosureShape::from_str("inverted-bracket").unwrap(),
            EnclosureShape::InvertedBracket
        );
        assert_eq!(
            EnclosureShape::from_str("none").unwrap(),
            EnclosureShape::None
        );
    }
}

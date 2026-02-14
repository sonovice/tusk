//! Formatting data types.
//!
//! Types for visual formatting, positioning, alignment, colors, fonts,
//! and other appearance-related values.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::ParseError;

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
// Numeric Measurement Types
// ============================================================================

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

/// Text direction (ltr, rtl, lro, rlo).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextDirection {
    Ltr,
    Rtl,
    Lro,
    Rlo,
}

impl fmt::Display for TextDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextDirection::Ltr => write!(f, "ltr"),
            TextDirection::Rtl => write!(f, "rtl"),
            TextDirection::Lro => write!(f, "lro"),
            TextDirection::Rlo => write!(f, "rlo"),
        }
    }
}

impl FromStr for TextDirection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ltr" => Ok(TextDirection::Ltr),
            "rtl" => Ok(TextDirection::Rtl),
            "lro" => Ok(TextDirection::Lro),
            "rlo" => Ok(TextDirection::Rlo),
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

#[cfg(test)]
mod tests {
    use super::*;

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

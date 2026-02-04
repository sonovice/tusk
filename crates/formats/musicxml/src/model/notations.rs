//! MusicXML 4.0 notations element types.
//!
//! Contains slurs, tied elements, articulations, and other notation markings
//! that appear within a note's <notations> element.

use serde::{Deserialize, Serialize};

use super::data::{AboveBelow, OverUnder, StartStopContinue, UpDown};

/// Container for notation elements on a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Notations {
    /// Slur elements (curved lines connecting notes).
    #[serde(rename = "slur", default, skip_serializing_if = "Vec::is_empty")]
    pub slurs: Vec<Slur>,

    /// Tied elements (graphic representation of ties).
    #[serde(rename = "tied", default, skip_serializing_if = "Vec::is_empty")]
    pub tied: Vec<Tied>,

    /// Articulation markings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articulations: Option<Articulations>,
}

/// Slur notation element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slur {
    /// Start, stop, or continue.
    #[serde(rename = "@type")]
    pub slur_type: StartStopContinue,

    /// Slur number (1-6) for distinguishing concurrent slurs.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Placement above or below the staff.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Orientation (over/under) for the curve.
    #[serde(rename = "@orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<OverUnder>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Bezier X offset.
    #[serde(rename = "@bezier-x", skip_serializing_if = "Option::is_none")]
    pub bezier_x: Option<f64>,

    /// Bezier Y offset.
    #[serde(rename = "@bezier-y", skip_serializing_if = "Option::is_none")]
    pub bezier_y: Option<f64>,

    /// Bezier X2 offset.
    #[serde(rename = "@bezier-x2", skip_serializing_if = "Option::is_none")]
    pub bezier_x2: Option<f64>,

    /// Bezier Y2 offset.
    #[serde(rename = "@bezier-y2", skip_serializing_if = "Option::is_none")]
    pub bezier_y2: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Slur {
    /// Create a new slur with the given type.
    pub fn new(slur_type: StartStopContinue) -> Self {
        Self {
            slur_type,
            number: None,
            placement: None,
            orientation: None,
            default_x: None,
            default_y: None,
            bezier_x: None,
            bezier_y: None,
            bezier_x2: None,
            bezier_y2: None,
            color: None,
            id: None,
        }
    }
}

/// Tied type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TiedType {
    Start,
    Stop,
    Continue,
    #[serde(rename = "let-ring")]
    LetRing,
}

/// Tied notation element (graphic representation of ties).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tied {
    /// Start, stop, continue, or let-ring.
    #[serde(rename = "@type")]
    pub tied_type: TiedType,

    /// Tie number (1-6) for distinguishing concurrent ties.
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Orientation (over/under) for the curve.
    #[serde(rename = "@orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<OverUnder>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Bezier X offset.
    #[serde(rename = "@bezier-x", skip_serializing_if = "Option::is_none")]
    pub bezier_x: Option<f64>,

    /// Bezier Y offset.
    #[serde(rename = "@bezier-y", skip_serializing_if = "Option::is_none")]
    pub bezier_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional ID.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Tied {
    /// Create a new tied with the given type.
    pub fn new(tied_type: TiedType) -> Self {
        Self {
            tied_type,
            number: None,
            orientation: None,
            default_x: None,
            default_y: None,
            bezier_x: None,
            bezier_y: None,
            color: None,
            id: None,
        }
    }
}

/// Empty placement type for articulation marks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyPlacement {
    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Color.
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Strong accent with type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StrongAccent {
    /// Type (up/down).
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub accent_type: Option<UpDown>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Default X position.
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,
}

/// Breath mark values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BreathMarkValue {
    /// No breath mark value (empty element).
    #[serde(rename = "")]
    Empty,
    Comma,
    Tick,
    Upbow,
    Salzedo,
}

impl Default for BreathMarkValue {
    fn default() -> Self {
        Self::Empty
    }
}

/// Breath mark element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BreathMark {
    /// Breath mark type.
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<BreathMarkValue>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,
}

/// Caesura values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaesuraValue {
    /// Normal caesura.
    #[serde(rename = "")]
    Normal,
    Short,
    Thick,
    Curved,
}

impl Default for CaesuraValue {
    fn default() -> Self {
        Self::Normal
    }
}

/// Caesura element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Caesura {
    /// Caesura type.
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CaesuraValue>,

    /// Placement above or below.
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,
}

/// Articulations container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Articulations {
    /// Accent (>).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<EmptyPlacement>,

    /// Strong accent (marcato, ^).
    #[serde(rename = "strong-accent", skip_serializing_if = "Option::is_none")]
    pub strong_accent: Option<StrongAccent>,

    /// Staccato (.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staccato: Option<EmptyPlacement>,

    /// Tenuto (-).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenuto: Option<EmptyPlacement>,

    /// Detached legato (tenuto + staccato).
    #[serde(rename = "detached-legato", skip_serializing_if = "Option::is_none")]
    pub detached_legato: Option<EmptyPlacement>,

    /// Staccatissimo (wedge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staccatissimo: Option<EmptyPlacement>,

    /// Spiccato.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spiccato: Option<EmptyPlacement>,

    /// Scoop (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoop: Option<EmptyPlacement>,

    /// Plop (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plop: Option<EmptyPlacement>,

    /// Doit (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doit: Option<EmptyPlacement>,

    /// Falloff (jazz articulation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falloff: Option<EmptyPlacement>,

    /// Breath mark.
    #[serde(rename = "breath-mark", skip_serializing_if = "Option::is_none")]
    pub breath_mark: Option<BreathMark>,

    /// Caesura.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caesura: Option<Caesura>,

    /// Stress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress: Option<EmptyPlacement>,

    /// Unstress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstress: Option<EmptyPlacement>,

    /// Soft accent.
    #[serde(rename = "soft-accent", skip_serializing_if = "Option::is_none")]
    pub soft_accent: Option<EmptyPlacement>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slur_creation() {
        let slur = Slur::new(StartStopContinue::Start);
        assert_eq!(slur.slur_type, StartStopContinue::Start);
        assert_eq!(slur.number, None);
    }

    #[test]
    fn test_tied_creation() {
        let tied = Tied::new(TiedType::Start);
        assert_eq!(tied.tied_type, TiedType::Start);
    }

    #[test]
    fn test_articulations_default() {
        let artics = Articulations::default();
        assert!(artics.accent.is_none());
        assert!(artics.staccato.is_none());
    }
}

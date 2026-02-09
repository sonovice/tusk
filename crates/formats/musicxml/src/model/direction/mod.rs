//! MusicXML 4.0 direction types.
//!
//! This module contains types for the `<direction>` element and its children,
//! including dynamics, tempo, pedals, wedges, and other musical directions.

mod dynamics;
mod metronome;
mod misc;
mod sound;
mod wedge;

pub use dynamics::*;
pub use metronome::*;
pub use misc::*;
pub use sound::*;
pub use wedge::*;

use serde::{Deserialize, Serialize};

use super::data::*;

// ============================================================================
// Direction Element
// ============================================================================

/// A musical direction that is not necessarily attached to a specific note.
///
/// Directions include dynamics, tempo markings, pedal markings, wedges (crescendo/diminuendo),
/// and other performance instructions. Multiple direction-type elements can be combined
/// (e.g., words followed by the start of a dashed line).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Direction {
    /// One or more direction types (required)
    #[serde(rename = "direction-type")]
    pub direction_types: Vec<DirectionType>,

    /// Offset from current position in divisions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Offset>,

    /// Staff number if different from default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<u32>,

    /// Sound/playback information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,

    /// Placement above or below the staff
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Whether this is a directive (performance instruction)
    #[serde(rename = "@directive", skip_serializing_if = "Option::is_none")]
    pub directive: Option<YesNo>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Direction {
    /// Create a new direction with the given direction types.
    pub fn new(direction_types: Vec<DirectionType>) -> Self {
        Self {
            direction_types,
            offset: None,
            staff: None,
            sound: None,
            placement: None,
            directive: None,
            id: None,
        }
    }

    /// Create a direction with a single wedge.
    pub fn wedge(wedge: Wedge) -> Self {
        Self::new(vec![DirectionType {
            content: DirectionTypeContent::Wedge(wedge),
            id: None,
        }])
    }

    /// Create a direction with dynamics.
    pub fn dynamics(dynamics: Vec<DynamicsValue>) -> Self {
        Self::new(vec![DirectionType {
            content: DirectionTypeContent::Dynamics(Dynamics {
                values: dynamics,
                placement: None,
            }),
            id: None,
        }])
    }
}

// ============================================================================
// Direction Type Container
// ============================================================================

/// Container for the type of direction.
///
/// A direction can contain one or more of these types. The direction-type
/// element groups together elements that represent different kinds of directions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionType {
    /// The content of this direction type
    #[serde(rename = "$value")]
    pub content: DirectionTypeContent,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// The actual content of a direction type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirectionTypeContent {
    /// Rehearsal marks (letters, numbers, section names)
    Rehearsal(Vec<Rehearsal>),
    /// Segno sign
    Segno(Vec<Segno>),
    /// Coda sign
    Coda(Vec<Coda>),
    /// Text direction (words)
    Words(Vec<Words>),
    /// Musical symbol using SMuFL glyph
    Symbol(Vec<Symbol>),
    /// Crescendo/diminuendo wedge
    Wedge(Wedge),
    /// Dynamic markings
    Dynamics(Dynamics),
    /// Dashes (for cresc./dim. text)
    Dashes(Dashes),
    /// Bracket line
    Bracket(Bracket),
    /// Piano pedal marks
    Pedal(Pedal),
    /// Metronome/tempo marking
    Metronome(Metronome),
    /// Octave shift (8va, 8vb, 15ma, etc.)
    OctaveShift(OctaveShift),
    /// Harp pedal diagram
    HarpPedals(HarpPedals),
    /// Harp damping mark
    Damp(Damp),
    /// Damp all strings
    DampAll(DampAll),
    /// Eyeglasses symbol (commercial music)
    Eyeglasses(Eyeglasses),
    /// String mute on/off
    StringMute(StringMute),
    /// Scordatura (string tuning changes)
    Scordatura(Scordatura),
    /// Embedded image
    Image(DirectionImage),
    /// Principal voice marking
    PrincipalVoice(PrincipalVoice),
    /// Percussion pictogram
    Percussion(Vec<Percussion>),
    /// Accordion registration diagram
    AccordionRegistration(AccordionRegistration),
    /// Staff division symbol
    StaffDivide(StaffDivide),
    /// Other direction not covered
    OtherDirection(OtherDirection),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_with_wedge() {
        let direction = Direction::wedge(Wedge::crescendo());
        assert_eq!(direction.direction_types.len(), 1);
    }

    #[test]
    fn test_direction_with_dynamics() {
        let direction = Direction::dynamics(vec![DynamicsValue::F]);
        assert_eq!(direction.direction_types.len(), 1);
    }

    #[test]
    fn test_direction_with_placement() {
        let mut direction = Direction::dynamics(vec![DynamicsValue::P]);
        direction.placement = Some(AboveBelow::Below);
        assert_eq!(direction.placement, Some(AboveBelow::Below));
    }
}

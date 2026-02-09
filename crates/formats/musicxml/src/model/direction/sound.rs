//! MusicXML 4.0 sound element and child types.
//!
//! The `<sound>` element contains general playback parameters. It can stand alone
//! within a part/measure, or be a component element within a direction.

use serde::{Deserialize, Serialize};

use super::super::data::YesNo;
use super::super::elements::score::{MidiDevice, MidiInstrument};
use super::Offset;

// ============================================================================
// Sound
// ============================================================================

/// Sound/playback information.
///
/// Contains general playback parameters including tempo, dynamics, navigation
/// (da capo, segno, coda), pedals, and MIDI instrument changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Sound {
    // -- Attributes --
    /// Tempo in quarter notes per minute
    #[serde(rename = "@tempo", skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f64>,

    /// Dynamics (percentage of default forte = 90)
    #[serde(rename = "@dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f64>,

    /// Da capo
    #[serde(rename = "@dacapo", skip_serializing_if = "Option::is_none")]
    pub dacapo: Option<YesNo>,

    /// Segno target
    #[serde(rename = "@segno", skip_serializing_if = "Option::is_none")]
    pub segno: Option<String>,

    /// Dal segno target
    #[serde(rename = "@dalsegno", skip_serializing_if = "Option::is_none")]
    pub dalsegno: Option<String>,

    /// Coda target
    #[serde(rename = "@coda", skip_serializing_if = "Option::is_none")]
    pub coda: Option<String>,

    /// To coda target
    #[serde(rename = "@tocoda", skip_serializing_if = "Option::is_none")]
    pub tocoda: Option<String>,

    /// Divisions per quarter note (used with segno/coda jumps)
    #[serde(rename = "@divisions", skip_serializing_if = "Option::is_none")]
    pub divisions: Option<f64>,

    /// Forward repeat implied but not displayed
    #[serde(rename = "@forward-repeat", skip_serializing_if = "Option::is_none")]
    pub forward_repeat: Option<YesNo>,

    /// Fine marking
    #[serde(rename = "@fine", skip_serializing_if = "Option::is_none")]
    pub fine: Option<String>,

    /// Apply only on specific times through a repeat
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,

    /// Pizzicato (yes=pizz, no=arco)
    #[serde(rename = "@pizzicato", skip_serializing_if = "Option::is_none")]
    pub pizzicato: Option<YesNo>,

    /// Pan position in degrees (-180 to 180). Deprecated in favor of midi-instrument pan.
    #[serde(rename = "@pan", skip_serializing_if = "Option::is_none")]
    pub pan: Option<f64>,

    /// Elevation in degrees (-180 to 180). Deprecated in favor of midi-instrument elevation.
    #[serde(rename = "@elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    /// Damper pedal (yes/no or percentage 0-100)
    #[serde(rename = "@damper-pedal", skip_serializing_if = "Option::is_none")]
    pub damper_pedal: Option<String>,

    /// Soft pedal (yes/no or percentage 0-100)
    #[serde(rename = "@soft-pedal", skip_serializing_if = "Option::is_none")]
    pub soft_pedal: Option<String>,

    /// Sostenuto pedal (yes/no or percentage 0-100)
    #[serde(rename = "@sostenuto-pedal", skip_serializing_if = "Option::is_none")]
    pub sostenuto_pedal: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // -- Children --
    /// Groups of instrument-change, midi-device, midi-instrument, play
    /// (can repeat as a group, ordered by instrument id)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub midi_instrument_changes: Vec<SoundMidiGroup>,

    /// Swing playback parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing: Option<Swing>,

    /// Timing offset from current position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Offset>,
}

impl Sound {
    /// Create a sound element with tempo.
    pub fn with_tempo(tempo: f64) -> Self {
        Self {
            tempo: Some(tempo),
            ..Default::default()
        }
    }

    /// Create a sound element with dynamics.
    pub fn with_dynamics(dynamics: f64) -> Self {
        Self {
            dynamics: Some(dynamics),
            ..Default::default()
        }
    }
}

// ============================================================================
// Sound MIDI Group
// ============================================================================

/// A group of instrument-change, midi-device, midi-instrument, and play elements
/// within a sound element. Per XSD, these repeat as a group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundMidiGroup {
    /// Instrument change (new sound for a score-instrument)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_change: Option<InstrumentChange>,

    /// MIDI device assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub midi_device: Option<MidiDevice>,

    /// MIDI instrument parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub midi_instrument: Option<MidiInstrument>,

    /// Playback techniques
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<Play>,
}

// ============================================================================
// Instrument Change
// ============================================================================

/// Represents a change to the virtual instrument sound for a score-instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentChange {
    /// Reference to the score-instrument being changed
    #[serde(rename = "@id")]
    pub id: String,

    /// New instrument sound (e.g. "wind.flutes.flute")
    #[serde(rename = "instrument-sound", skip_serializing_if = "Option::is_none")]
    pub instrument_sound: Option<String>,

    /// Solo performance (true = `<solo/>` present)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solo: Option<bool>,

    /// Ensemble with optional size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensemble: Option<String>,

    /// Virtual instrument library
    #[serde(rename = "virtual-library", skip_serializing_if = "Option::is_none")]
    pub virtual_library: Option<String>,

    /// Virtual instrument name
    #[serde(rename = "virtual-name", skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

// ============================================================================
// Play
// ============================================================================

/// Playback techniques for an instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Play {
    /// Instrument ID reference
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Playback technique entries (can repeat)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<PlayEntry>,
}

/// A single playback technique entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayEntry {
    /// IPA pronunciation for vocal music
    Ipa(String),
    /// Mute type
    Mute(String),
    /// Semi-pitched percussion category
    SemiPitched(String),
    /// Other playback technique
    OtherPlay(OtherPlay),
}

/// Other playback type not covered by standard entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherPlay {
    /// Type of playback
    #[serde(rename = "@type")]
    pub play_type: String,

    /// Text content
    #[serde(rename = "$value", default)]
    pub value: String,
}

// ============================================================================
// Swing
// ============================================================================

/// Swing playback parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Swing {
    /// Swing content: either straight or ratio-based
    pub content: SwingContent,

    /// Optional swing style description
    #[serde(rename = "swing-style", skip_serializing_if = "Option::is_none")]
    pub swing_style: Option<String>,
}

/// The content of a swing element: either straight or a ratio.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SwingContent {
    /// No swing — consecutive notes have equal durations
    Straight,
    /// Ratio-based swing
    Ratio(SwingRatio),
}

/// Ratio-based swing parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwingRatio {
    /// First duration in the ratio (e.g. 2 for 2:1)
    pub first: u32,
    /// Second duration in the ratio (e.g. 1 for 2:1)
    pub second: u32,
    /// Note type the swing applies to (eighth or 16th). Default: eighth.
    #[serde(rename = "swing-type", skip_serializing_if = "Option::is_none")]
    pub swing_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_with_tempo() {
        let sound = Sound::with_tempo(120.0);
        assert_eq!(sound.tempo, Some(120.0));
        assert_eq!(sound.dynamics, None);
    }

    #[test]
    fn test_sound_with_dynamics() {
        let sound = Sound::with_dynamics(80.0);
        assert_eq!(sound.dynamics, Some(80.0));
        assert_eq!(sound.tempo, None);
    }

    #[test]
    fn test_swing_straight() {
        let swing = Swing {
            content: SwingContent::Straight,
            swing_style: None,
        };
        assert_eq!(swing.content, SwingContent::Straight);
    }

    #[test]
    fn test_swing_ratio() {
        let swing = Swing {
            content: SwingContent::Ratio(SwingRatio {
                first: 2,
                second: 1,
                swing_type: Some("eighth".to_string()),
            }),
            swing_style: Some("Swing".to_string()),
        };
        assert_eq!(swing.swing_style, Some("Swing".to_string()));
    }
}

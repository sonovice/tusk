//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTempoGes {
    /**Captures the number of *quarter notes* per minute. In MIDI, a beat is always defined
          as a quarter note, *not the numerator of the time signature or the metronomic
          indication*.*/
    #[serde(rename = "@midi.bpm", skip_serializing_if = "Option::is_none")]
    pub midi_bpm: Option<crate::generated::data::DataMidibpm>,
    /**Records the number of microseconds per *quarter note*. In MIDI, a beat is always
          defined as a quarter note, *not the numerator of the time signature or the metronomic
          indication*. At 120 quarter notes per minute, each quarter note will last 500,000
          microseconds.*/
    #[serde(rename = "@midi.mspb", skip_serializing_if = "Option::is_none")]
    pub midi_mspb: Option<crate::generated::data::DataMidimspb>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record tempo in terms of beats per minute.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMmTempo {
    /**Used to describe tempo in terms of beats (often the meter signature denominator) per
          minute, ala M.M. (Maelzel’s Metronome). Do not confuse this attribute with midi.bpm or
          midi.mspb. In MIDI, a beat is always defined as a quarter note, *not the numerator of the
          time signature or the metronomic indication*.*/
    #[serde(rename = "@mm", skip_serializing_if = "Option::is_none")]
    pub mm: Option<crate::generated::data::DataTempovalue>,
    ///Captures the metronomic unit.
    #[serde(rename = "@mm.unit", skip_serializing_if = "Option::is_none")]
    pub mm_unit: Option<crate::generated::data::DataDuration>,
    ///Records the number of augmentation dots required by a dotted metronome unit.
    #[serde(rename = "@mm.dots", skip_serializing_if = "Option::is_none")]
    pub mm_dots: Option<crate::generated::data::DataAugmentdot>,
}

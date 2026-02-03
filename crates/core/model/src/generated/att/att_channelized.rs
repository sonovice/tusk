//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record MIDI channel information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttChannelized {
    ///Records a MIDI channel value.
    #[serde(rename = "@midi.channel", skip_serializing_if = "Option::is_none")]
    pub midi_channel: Option<crate::generated::data::DataMidichannel>,
    ///Specifies the 'on' part of the duty cycle as a percentage of a note’s duration.
    #[serde(rename = "@midi.duty", skip_serializing_if = "Option::is_none")]
    pub midi_duty: Option<crate::generated::data::DataPercentLimited>,
    ///Sets the MIDI port value.
    #[serde(rename = "@midi.port", skip_serializing_if = "Option::is_none")]
    pub midi_port: Option<crate::generated::data::DataMidivalueName>,
    ///Sets the MIDI track.
    #[serde(rename = "@midi.track", skip_serializing_if = "Option::is_none")]
    pub midi_track: Option<u64>,
}

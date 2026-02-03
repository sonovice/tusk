//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///MIDI attributes pertaining to key velocity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMidiVelocity {
    ///MIDI Note-on/off velocity.
    #[serde(rename = "@vel", skip_serializing_if = "Option::is_none")]
    pub vel: Option<crate::generated::data::DataMidivalue>,
}

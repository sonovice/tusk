//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record MIDI numbers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMidiNumber {
    ///MIDI number in the range set by data.MIDIVALUE.
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<crate::generated::data::DataMidivalue>,
}

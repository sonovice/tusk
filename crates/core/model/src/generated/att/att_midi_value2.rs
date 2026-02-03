//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record terminal MIDI values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMidiValue2 {
    ///MIDI number.
    #[serde(rename = "@val2", skip_serializing_if = "Option::is_none")]
    pub val2: Option<crate::generated::data::DataMidivalue>,
}

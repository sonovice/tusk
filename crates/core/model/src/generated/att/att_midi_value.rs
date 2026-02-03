//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record MIDI values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMidiValue {
    ///MIDI number.
    #[serde(rename = "@val", skip_serializing_if = "Option::is_none")]
    pub val: Option<crate::generated::data::DataMidivalue>,
}

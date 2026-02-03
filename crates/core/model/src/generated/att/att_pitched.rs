//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record written pitch name and octave number.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPitched {
    ///Contains a written pitch name.
    #[serde(rename = "@pname", skip_serializing_if = "Option::is_none")]
    pub pname: Option<crate::generated::data::DataPitchname>,
    ///Captures written octave information.
    #[serde(rename = "@oct", skip_serializing_if = "Option::is_none")]
    pub oct: Option<crate::generated::data::DataOctave>,
}

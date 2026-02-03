//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that identify location on a staff in terms of pitch and octave.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffLocPitched {
    ///Captures staff location in terms of written pitch name.
    #[serde(rename = "@ploc", skip_serializing_if = "Option::is_none")]
    pub ploc: Option<crate::generated::data::DataPitchname>,
    ///Records staff location in terms of written octave.
    #[serde(rename = "@oloc", skip_serializing_if = "Option::is_none")]
    pub oloc: Option<crate::generated::data::DataOctave>,
}

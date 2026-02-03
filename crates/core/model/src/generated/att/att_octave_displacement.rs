//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes describing the amount and direction of octave displacement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOctaveDisplacement {
    ///Records the amount of octave displacement.
    #[serde(rename = "@dis", skip_serializing_if = "Option::is_none")]
    pub dis: Option<crate::generated::data::DataOctaveDis>,
    ///Records the direction of octave displacement.
    #[serde(rename = "@dis.place", skip_serializing_if = "Option::is_none")]
    pub dis_place: Option<crate::generated::data::DataStaffrelBasic>,
}

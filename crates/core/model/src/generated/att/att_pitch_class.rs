//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe pitch class.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPitchClass {
    ///Holds pitch class information.
    #[serde(rename = "@pclass", skip_serializing_if = "Option::is_none")]
    pub pclass: Option<crate::generated::data::DataPitchclass>,
}

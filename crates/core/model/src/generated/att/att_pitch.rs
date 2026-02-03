//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record written pitch name.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPitch {
    ///Contains a written pitch name.
    #[serde(rename = "@pname", skip_serializing_if = "Option::is_none")]
    pub pname: Option<crate::generated::data::DataPitchname>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttKeySigAnl {
    ///Captures a written accidental.
    #[serde(rename = "@accid", skip_serializing_if = "Option::is_none")]
    pub accid: Option<crate::generated::data::DataAccidentalWritten>,
    ///Indicates major, minor, or other tonality.
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::generated::data::DataMode>,
    ///Contains a written pitch name.
    #[serde(rename = "@pname", skip_serializing_if = "Option::is_none")]
    pub pname: Option<crate::generated::data::DataPitchname>,
}

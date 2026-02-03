//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for describing key mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttKeyMode {
    ///Indicates major, minor, or other tonality.
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::generated::data::DataMode>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttKeySigLog {
    ///Written key signature.
    #[serde(rename = "@sig", default, skip_serializing_if = "Vec::is_empty")]
    pub sig: Vec<crate::generated::data::DataKeyfifths>,
}

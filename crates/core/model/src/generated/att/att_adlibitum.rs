//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe a performance resource as ad libitum (optional).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAdlibitum {
    ///Marks a performance resource as ad libitum (optional).
    #[serde(rename = "@adlib", skip_serializing_if = "Option::is_none")]
    pub adlib: Option<crate::generated::data::DataBoolean>,
}

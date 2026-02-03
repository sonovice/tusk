//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that define the characteristics and components of the performance resource or a performance resource list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPerfResBasic {
    ///Marks a performance resource as ad libitum (optional).
    #[serde(rename = "@adlib", skip_serializing_if = "Option::is_none")]
    pub adlib: Option<crate::generated::data::DataBoolean>,
    ///Indicates the number of performers.
    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

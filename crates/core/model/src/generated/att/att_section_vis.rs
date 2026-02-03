//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSectionVis {
    ///Indicates that staves begin again with this section.
    #[serde(rename = "@restart", skip_serializing_if = "Option::is_none")]
    pub restart: Option<crate::generated::data::DataBoolean>,
}

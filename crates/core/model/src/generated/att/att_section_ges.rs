//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSectionGes {
    /**Indicates that the performance of the next musical division should begin immediately
    following this one.*/
    #[serde(rename = "@attacca", skip_serializing_if = "Option::is_none")]
    pub attacca: Option<crate::generated::data::DataBoolean>,
}

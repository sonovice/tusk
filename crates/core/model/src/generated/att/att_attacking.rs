//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes whether an element is performed "attacca".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAttacking {
    /**Indicates that the performance of the next musical division should begin immediately
          following this one.*/
    #[serde(rename = "@attacca", skip_serializing_if = "Option::is_none")]
    pub attacca: Option<crate::generated::data::DataBoolean>,
}

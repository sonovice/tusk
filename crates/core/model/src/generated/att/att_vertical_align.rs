//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record vertical alignment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVerticalAlign {
    ///Records vertical alignment.
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<crate::generated::data::DataVerticalalignment>,
}

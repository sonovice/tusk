//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record horizontal alignment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHorizontalAlign {
    ///Records horizontal alignment.
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<crate::generated::data::DataHorizontalalignment>,
}

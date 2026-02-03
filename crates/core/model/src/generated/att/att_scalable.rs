//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe relative size.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScalable {
    ///Scale factor to be applied to the feature to make it the desired display size.
    #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<crate::generated::data::DataPercent>,
}

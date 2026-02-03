//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttRegularMethodMethod {
    ///Corrections and normalizations made silently.
    #[serde(rename = "silent")]
    Silent,
    ///Corrections and normalizations represented using markup.
    #[serde(rename = "markup")]
    Markup,
}
///Attributes that describe correction and normalization methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRegularMethod {
    ///Indicates the method employed to mark corrections and normalizations.
    #[serde(rename = "@method", skip_serializing_if = "Option::is_none")]
    pub method: Option<AttRegularMethodMethod>,
}

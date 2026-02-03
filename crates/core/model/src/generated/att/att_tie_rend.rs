//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe the rendition of ties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTieRend {
    ///
    #[serde(rename = "@tie.lform", skip_serializing_if = "Option::is_none")]
    pub tie_lform: Option<crate::generated::data::DataLineform>,
    ///
    #[serde(rename = "@tie.lwidth", skip_serializing_if = "Option::is_none")]
    pub tie_lwidth: Option<crate::generated::data::DataLinewidth>,
}

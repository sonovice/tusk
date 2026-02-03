//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe the rendition of slurs.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSlurRend {
    ///
    #[serde(rename = "@slur.lform", skip_serializing_if = "Option::is_none")]
    pub slur_lform: Option<crate::generated::data::DataLineform>,
    ///
    #[serde(rename = "@slur.lwidth", skip_serializing_if = "Option::is_none")]
    pub slur_lwidth: Option<crate::generated::data::DataLinewidth>,
}

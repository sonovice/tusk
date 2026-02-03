//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLigatureVis {
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
    as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    ///Provides an indication of the function of the ligature.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<crate::generated::data::DataLigatureform>,
}

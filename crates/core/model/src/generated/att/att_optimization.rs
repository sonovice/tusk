//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes pertaining to layout optimization.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOptimization {
    /**Indicates whether staves without notes, rests, etc. should be displayed. When the
    value is 'true', empty staves are not displayed.*/
    #[serde(rename = "@optimize", skip_serializing_if = "Option::is_none")]
    pub optimize: Option<crate::generated::data::DataBoolean>,
}

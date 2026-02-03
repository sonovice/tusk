//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes pertaining to measure numbers
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeasureNumbers {
    ///Indicates whether measure numbers should be displayed.
    #[serde(rename = "@mnum.visible", skip_serializing_if = "Option::is_none")]
    pub mnum_visible: Option<crate::generated::data::DataBoolean>,
}

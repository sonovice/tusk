//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record the placement and visibility of numbers that accompany a bowed notational feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNumberPlacement {
    ///States where the number will be placed in relation to the notational feature.
    #[serde(rename = "@num.place", skip_serializing_if = "Option::is_none")]
    pub num_place: Option<crate::generated::data::DataStaffrelBasic>,
    ///Determines if the number is visible.
    #[serde(rename = "@num.visible", skip_serializing_if = "Option::is_none")]
    pub num_visible: Option<crate::generated::data::DataBoolean>,
}

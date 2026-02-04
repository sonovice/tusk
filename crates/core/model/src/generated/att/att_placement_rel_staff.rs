//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes capturing placement information with respect to the staff.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPlacementRelStaff {
    /**Captures the placement of the item with respect to the staff with which it is
    associated.*/
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataStaffrel>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes capturing placement on a staff.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPlacementOnStaff {
    ///Indicates the placement of the item within the staff. A value oftruemeans on the staff, andfalseoff the staff.
    #[serde(rename = "@onstaff", skip_serializing_if = "Option::is_none")]
    pub onstaff: Option<crate::generated::data::DataBoolean>,
}

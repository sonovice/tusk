//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes capturing placement information with respect to an event.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPlacementRelEvent {
    /**Captures the placement of the item with respect to the event with which it is
             associated.*/
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataStaffrel>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe items printed near (above, below, or between) staves
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffItems {
    /**Describes vertical order of items printed above a staff, from closest to farthest away
          from the staff.*/
    #[serde(rename = "@aboveorder", default, skip_serializing_if = "Vec::is_empty")]
    pub aboveorder: Vec<crate::generated::data::DataStaffitem>,
    /**Describes vertical order of items printed below a staff, from closest to farthest away
          from the staff.*/
    #[serde(rename = "@beloworder", default, skip_serializing_if = "Vec::is_empty")]
    pub beloworder: Vec<crate::generated::data::DataStaffitem>,
    ///Describes vertical order of items printed between staves, from top to bottom.
    #[serde(rename = "@betweenorder", default, skip_serializing_if = "Vec::is_empty")]
    pub betweenorder: Vec<crate::generated::data::DataStaffitem>,
}

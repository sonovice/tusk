//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that identify location on a staff in terms of lines and spaces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffLoc {
    ///Holds the staff location of the feature.
    #[serde(rename = "@loc", skip_serializing_if = "Option::is_none")]
    pub loc: Option<crate::generated::data::DataStaffloc>,
}

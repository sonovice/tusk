//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for identifying the part in which the current feature appears.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPartIdent {
    /**Indicates the part in which the current feature should appear. Use '%all' when the
    feature should occur in every part.*/
    #[serde(rename = "@part", default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<String>,
    /**Signifies the part staff on which a notated feature occurs. Use '%all' when the
    feature should occur on every staff.*/
    #[serde(rename = "@partstaff", default, skip_serializing_if = "Vec::is_empty")]
    pub partstaff: Vec<String>,
}

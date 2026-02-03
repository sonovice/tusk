//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for marking the presence of a slur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSlurPresent {
    /**Indicates that this element participates in a slur. If visual information about the
    slur needs to be recorded, then aslurelement should be
    employed.*/
    #[serde(rename = "@slur", default, skip_serializing_if = "Vec::is_empty")]
    pub slur: Vec<crate::generated::data::DataSlur>,
}

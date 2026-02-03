//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteVisCmn {
    /**Presence of this attribute indicates that the secondary beam should be broken
    following this note/chord. The value of the attribute records the number of beams which
    should remain unbroken.*/
    #[serde(rename = "@breaksec", skip_serializing_if = "Option::is_none")]
    pub breaksec: Option<u64>,
}

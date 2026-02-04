//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for marking the presence of an ornament.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOrnamPresent {
    /**Indicates that this element has an attached ornament. If visual information about the
    ornament is needed, then one of the elements that represents an ornament (mordent, trill,
    or turn) should be employed.*/
    #[serde(rename = "@ornam", default, skip_serializing_if = "Vec::is_empty")]
    pub ornam: Vec<crate::generated::data::DataOrnamCmn>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for indicating the presence of a tuplet.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTupletPresent {
    /**Indicates that this feature participates in a tuplet. If visual information about the
    tuplet needs to be recorded, then atupletelement should be
    employed.*/
    #[serde(rename = "@tuplet", default, skip_serializing_if = "Vec::is_empty")]
    pub tuplet: Vec<crate::generated::data::DataTuplet>,
}

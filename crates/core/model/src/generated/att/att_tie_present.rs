//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that indicate the presence of a tie.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTiePresent {
    /**Indicates that this element participates in a tie. If visual information about the tie
          needs to be recorded, then atieelement should be employed.*/
    #[serde(rename = "@tie", default, skip_serializing_if = "Vec::is_empty")]
    pub tie: Vec<crate::generated::data::DataTie>,
}

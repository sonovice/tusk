//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Indication of coloration.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttColoration {
    /**Indicates this feature is 'colored'; that is, it is a participant in a change in
    rhythmic values. In mensural notation, coloration is indicated by colored notes (red,
    black, etc.) where void notes would otherwise occur. In CMN, coloration is indicated by an
    inverse color; that is, the note head is void when it would otherwise be filled and vice
    versa.*/
    #[serde(rename = "@colored", skip_serializing_if = "Option::is_none")]
    pub colored: Option<crate::generated::data::DataBoolean>,
}

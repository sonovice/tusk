//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that hold associated sung text syllables.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSylText {
    ///Holds an associated sung text syllable.
    #[serde(rename = "@syl", skip_serializing_if = "Option::is_none")]
    pub syl: Option<String>,
}

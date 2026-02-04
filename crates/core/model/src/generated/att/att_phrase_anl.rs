//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPhraseAnl {
    /**Used for linking visually separate entities that form a single logical entity, for
          example, multiple slurs broken across a system break that form a single musical phrase.
          Also used to indicate a measure which metrically completes the current one. Record the
          identifiers of the separately encoded components, excluding the one carrying the
          attribute.*/
    #[serde(rename = "@join", default, skip_serializing_if = "Vec::is_empty")]
    pub join: Vec<crate::generated::data::DataUri>,
}

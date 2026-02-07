//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that deal with string filing characteristics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFiling {
    /**Holds the number of initial characters (such as those constituting an article or
          preposition) that should not be used for sorting a title or name.*/
    #[serde(rename = "@nonfiling", skip_serializing_if = "Option::is_none")]
    pub nonfiling: Option<u64>,
}

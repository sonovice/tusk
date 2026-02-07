//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLabelled {
    /**Captures text to be used to generate a label for the element to which it’s attached, a
          "tool tip" or prefatory text, for example. Should not be used to record document
          content.*/
    #[serde(rename = "@label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

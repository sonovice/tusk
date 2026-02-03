//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for selecting data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDataSelecting {
    ///XPath used to select data to which an element or a property applies.
    #[serde(rename = "@select", skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
}

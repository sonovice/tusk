//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes describing a writing medium, such as pencil or ink.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMedium {
    ///Describes the writing medium.
    #[serde(rename = "@medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
}

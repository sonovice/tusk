//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for the identification of a causative agent.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAgentIdent {
    /**Signifies the causative agent of damage, illegibility, or other loss of original
    text.*/
    #[serde(rename = "@agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
}

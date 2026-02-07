//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that identify the reason why an editorial feature is used.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttReasonIdent {
    /**Holds a short phrase describing the reason for missing textual material (gap), why
          material is supplied (supplied), or why transcription is difficult (unclear).*/
    #[serde(rename = "@reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

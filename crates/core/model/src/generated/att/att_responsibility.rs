//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes capturing information regarding responsibility for some aspect of the text's
creation, transcription, editing, or encoding.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttResponsibility {
    /**Indicates the agent(s) responsible for some aspect of the text’s transcription,
    editing, or encoding. Its value must point to one or more identifiers declared in the
    document header.*/
    #[serde(rename = "@resp", default, skip_serializing_if = "Vec::is_empty")]
    pub resp: Vec<crate::generated::data::DataUri>,
}

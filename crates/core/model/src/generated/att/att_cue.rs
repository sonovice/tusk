//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe "cue-ness".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCue {
    ///
    #[serde(rename = "@cue", skip_serializing_if = "Option::is_none")]
    pub cue: Option<crate::generated::data::DataBoolean>,
}

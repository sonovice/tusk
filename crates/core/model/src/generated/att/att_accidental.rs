//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for capturing momentary pitch inflection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAccidental {
    ///Captures a written accidental.
    #[serde(rename = "@accid", skip_serializing_if = "Option::is_none")]
    pub accid: Option<crate::generated::data::DataAccidentalWritten>,
}

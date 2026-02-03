//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record written octave.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOctave {
    ///Captures written octave information.
    #[serde(rename = "@oct", skip_serializing_if = "Option::is_none")]
    pub oct: Option<crate::generated::data::DataOctave>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCustosLog {
    ///Captures a written accidental.
    #[serde(rename = "@accid", skip_serializing_if = "Option::is_none")]
    pub accid: Option<crate::generated::data::DataAccidentalWritten>,
    ///Contains a written pitch name.
    #[serde(rename = "@pname", skip_serializing_if = "Option::is_none")]
    pub pname: Option<crate::generated::data::DataPitchname>,
    ///Captures written octave information.
    #[serde(rename = "@oct", skip_serializing_if = "Option::is_none")]
    pub oct: Option<crate::generated::data::DataOctave>,
    /**Encodes the target note when its pitch differs from the pitch at which the custos
          appears.*/
    #[serde(rename = "@target", skip_serializing_if = "Option::is_none")]
    pub target: Option<crate::generated::data::DataUri>,
}

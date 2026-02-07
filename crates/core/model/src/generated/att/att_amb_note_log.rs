//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAmbNoteLog {
    ///Captures a written accidental.
    #[serde(rename = "@accid", skip_serializing_if = "Option::is_none")]
    pub accid: Option<crate::generated::data::DataAccidentalWritten>,
    /**Indicates this feature is 'colored'; that is, it is a participant in a change in
          rhythmic values. In mensural notation, coloration is indicated by colored notes (red,
          black, etc.) where void notes would otherwise occur. In CMN, coloration is indicated by an
          inverse color; that is, the note head is void when it would otherwise be filled and vice
          versa.*/
    #[serde(rename = "@colored", skip_serializing_if = "Option::is_none")]
    pub colored: Option<crate::generated::data::DataBoolean>,
    /**Records the duration of a feature using the relative durational values provided by the
          data.DURATION datatype.*/
    #[serde(rename = "@dur", skip_serializing_if = "Option::is_none")]
    pub dur: Option<crate::generated::data::DataDuration>,
    ///Contains a written pitch name.
    #[serde(rename = "@pname", skip_serializing_if = "Option::is_none")]
    pub pname: Option<crate::generated::data::DataPitchname>,
    ///Captures written octave information.
    #[serde(rename = "@oct", skip_serializing_if = "Option::is_none")]
    pub oct: Option<crate::generated::data::DataOctave>,
}

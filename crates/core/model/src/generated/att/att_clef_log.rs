//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttClefLog {
    ///Describes a clef’s shape.
    #[serde(rename = "@shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<crate::generated::data::DataClefshape>,
    /**Indicates the line upon which a feature stands. The value must be in the range between
          1 and the number of lines on the staff. The numbering of lines starts with the lowest line
          of the staff.*/
    #[serde(rename = "@line", skip_serializing_if = "Option::is_none")]
    pub line: Option<crate::generated::data::DataClefline>,
    ///Captures written octave information.
    #[serde(rename = "@oct", skip_serializing_if = "Option::is_none")]
    pub oct: Option<crate::generated::data::DataOctave>,
    ///Records the amount of octave displacement.
    #[serde(rename = "@dis", skip_serializing_if = "Option::is_none")]
    pub dis: Option<crate::generated::data::DataOctaveDis>,
    ///Records the direction of octave displacement.
    #[serde(rename = "@dis.place", skip_serializing_if = "Option::is_none")]
    pub dis_place: Option<crate::generated::data::DataStaffrelBasic>,
    /**Records the function of the clef. A "cautionary" clef does not change the following
          pitches.*/
    #[serde(rename = "@cautionary", skip_serializing_if = "Option::is_none")]
    pub cautionary: Option<crate::generated::data::DataBoolean>,
}

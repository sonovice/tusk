//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the logical
domain related to clefs.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCleffingLog {
    ///Encodes a value for the clef symbol.
    #[serde(rename = "@clef.shape", skip_serializing_if = "Option::is_none")]
    pub clef_shape: Option<crate::generated::data::DataClefshape>,
    /**Contains a default value for the position of the clef. The value must be in the range
    between 1 and the number of lines on the staff. The numbering of lines starts with the
    lowest line of the staff.*/
    #[serde(rename = "@clef.line", skip_serializing_if = "Option::is_none")]
    pub clef_line: Option<crate::generated::data::DataClefline>,
    ///Records the amount of octave displacement to be applied to the clef.
    #[serde(rename = "@clef.dis", skip_serializing_if = "Option::is_none")]
    pub clef_dis: Option<crate::generated::data::DataOctaveDis>,
    ///Records the direction of octave displacement to be applied to the clef.
    #[serde(rename = "@clef.dis.place", skip_serializing_if = "Option::is_none")]
    pub clef_dis_place: Option<crate::generated::data::DataStaffrelBasic>,
}

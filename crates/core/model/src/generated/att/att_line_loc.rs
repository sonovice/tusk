//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for identifying the staff line with which a feature is associated.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLineLoc {
    /**Indicates the line upon which a feature stands. The value must be in the range between
          1 and the number of lines on the staff. The numbering of lines starts with the lowest line
          of the staff.*/
    #[serde(rename = "@line", skip_serializing_if = "Option::is_none")]
    pub line: Option<crate::generated::data::DataClefline>,
}

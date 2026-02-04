//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that capture characters used to enclose symbols having a cautionary or
      editorial function.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttEnclosingChars {
    /**Records the characters often used to mark accidentals, articulations, and sometimes
          notes as having a cautionary or editorial function. For an example of cautionary
          accidentals enclosed in parentheses, see Read, p. 131, ex. 9-14.*/
    #[serde(rename = "@enclose", skip_serializing_if = "Option::is_none")]
    pub enclose: Option<crate::generated::data::DataEnclosure>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterSigGrpVis {
    /**Records the characters often used to mark accidentals, articulations, and sometimes
    notes as having a cautionary or editorial function. For an example of cautionary
    accidentals enclosed in parentheses, see Read, p. 131, ex. 9-14.*/
    #[serde(rename = "@enclose", skip_serializing_if = "Option::is_none")]
    pub enclose: Option<crate::generated::data::DataEnclosure>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
}

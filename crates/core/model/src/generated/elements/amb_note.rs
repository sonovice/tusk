//!Element: `<ambNote>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Highest or lowest pitch in a score, staff, or layer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ambNote")]
pub struct AmbNote {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub amb_note_anl: crate::generated::att::AttAmbNoteAnl,
    #[serde(flatten)]
    pub amb_note_ges: crate::generated::att::AttAmbNoteGes,
    #[serde(flatten)]
    pub amb_note_log: crate::generated::att::AttAmbNoteLog,
    #[serde(flatten)]
    pub amb_note_vis: crate::generated::att::AttAmbNoteVis,
}
impl Validate for AmbNote {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

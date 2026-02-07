//!Element: `<accid>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///accidental - Records a temporary alteration to the pitch of a note.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "accid")]
pub struct Accid {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub accid_anl: crate::generated::att::AttAccidAnl,
    #[serde(flatten)]
    pub accid_ges: crate::generated::att::AttAccidGes,
    #[serde(flatten)]
    pub accid_log: crate::generated::att::AttAccidLog,
    #[serde(flatten)]
    pub accid_vis: crate::generated::att::AttAccidVis,
}
impl crate::generated::model::ModelNoteModifierLike for Accid {}
impl crate::generated::model::ModelSyllablePart for Accid {}
impl Validate for Accid {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

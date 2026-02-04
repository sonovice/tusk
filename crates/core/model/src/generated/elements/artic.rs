//!Element: `<artic>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///articulation - An indication of how to play a note or chord.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "artic")]
pub struct Artic {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub artic_anl: crate::generated::att::AttArticAnl,
    #[serde(flatten)]
    pub artic_ges: crate::generated::att::AttArticGes,
    #[serde(flatten)]
    pub artic_log: crate::generated::att::AttArticLog,
    #[serde(flatten)]
    pub artic_vis: crate::generated::att::AttArticVis,
}
impl crate::generated::model::ModelChordPart for Artic {}
impl crate::generated::model::ModelNoteModifierLike for Artic {}
impl Validate for Artic {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

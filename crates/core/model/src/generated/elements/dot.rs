//!Element: `<dot>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Dot of augmentation or division.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "dot")]
pub struct Dot {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub dot_anl: crate::generated::att::AttDotAnl,
    #[serde(flatten)]
    pub dot_ges: crate::generated::att::AttDotGes,
    #[serde(flatten)]
    pub dot_log: crate::generated::att::AttDotLog,
    #[serde(flatten)]
    pub dot_vis: crate::generated::att::AttDotVis,
}
impl crate::generated::model::ModelNoteModifierLike for Dot {}
impl crate::generated::model::ModelEventLikeMensural for Dot {}
impl Validate for Dot {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

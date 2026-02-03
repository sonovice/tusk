//!Element: `<bend>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**A variation in pitch (often micro-tonal) upwards or downwards during the course of a
note.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bend")]
pub struct Bend {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub bend_log: crate::generated::att::AttBendLog,
    #[serde(flatten)]
    pub bend_vis: crate::generated::att::AttBendVis,
    #[serde(flatten)]
    pub bend_ges: crate::generated::att::AttBendGes,
    #[serde(flatten)]
    pub bend_anl: crate::generated::att::AttBendAnl,
}
impl crate::generated::model::ModelControlEventLike for Bend {}
impl Validate for Bend {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

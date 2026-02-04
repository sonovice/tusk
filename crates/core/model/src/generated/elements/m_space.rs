//!Element: `<mSpace>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///measure space - A measure containing only empty space in any meter.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mSpace")]
pub struct MSpace {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub m_space_log: crate::generated::att::AttMSpaceLog,
    #[serde(flatten)]
    pub m_space_vis: crate::generated::att::AttMSpaceVis,
    #[serde(flatten)]
    pub m_space_ges: crate::generated::att::AttMSpaceGes,
    #[serde(flatten)]
    pub m_space_anl: crate::generated::att::AttMSpaceAnl,
}
impl crate::generated::model::ModelEventLikeMeasureFilling for MSpace {}
impl Validate for MSpace {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<mRpt2>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**2-measure repeat - An indication that the previous two measures should be
repeated.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mRpt2")]
pub struct MRpt2 {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub m_rpt2_log: crate::generated::att::AttMRpt2Log,
    #[serde(flatten)]
    pub m_rpt2_vis: crate::generated::att::AttMRpt2Vis,
    #[serde(flatten)]
    pub m_rpt2_ges: crate::generated::att::AttMRpt2Ges,
    #[serde(flatten)]
    pub m_rpt2_anl: crate::generated::att::AttMRpt2Anl,
}
impl crate::generated::model::ModelEventLikeMeasureFilling for MRpt2 {}
impl Validate for MRpt2 {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

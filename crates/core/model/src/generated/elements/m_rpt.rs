//!Element: `<mRpt>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///measure repeat - An indication that the previous measure should be repeated.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mRpt")]
pub struct MRpt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub m_rpt_log: crate::generated::att::AttMRptLog,
    #[serde(flatten)]
    pub m_rpt_vis: crate::generated::att::AttMRptVis,
    #[serde(flatten)]
    pub m_rpt_ges: crate::generated::att::AttMRptGes,
    #[serde(flatten)]
    pub m_rpt_anl: crate::generated::att::AttMRptAnl,
}
impl crate::generated::model::ModelEventLikeMeasureFilling for MRpt {}
impl Validate for MRpt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<halfmRpt>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///half-measure repeat - A half-measure repeat in any meter.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "halfmRpt")]
pub struct HalfmRpt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub halfm_rpt_log: crate::generated::att::AttHalfmRptLog,
    #[serde(flatten)]
    pub halfm_rpt_vis: crate::generated::att::AttHalfmRptVis,
    #[serde(flatten)]
    pub halfm_rpt_ges: crate::generated::att::AttHalfmRptGes,
    #[serde(flatten)]
    pub halfm_rpt_anl: crate::generated::att::AttHalfmRptAnl,
}
impl crate::generated::model::ModelEventLikeCmn for HalfmRpt {}
impl Validate for HalfmRpt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<beatRpt>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///beat repeat - An indication that material on a preceding beat should be repeated.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "beatRpt")]
pub struct BeatRpt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub beat_rpt_log: crate::generated::att::AttBeatRptLog,
    #[serde(flatten)]
    pub beat_rpt_vis: crate::generated::att::AttBeatRptVis,
    #[serde(flatten)]
    pub beat_rpt_ges: crate::generated::att::AttBeatRptGes,
    #[serde(flatten)]
    pub beat_rpt_anl: crate::generated::att::AttBeatRptAnl,
    #[serde(flatten)]
    pub plist: crate::generated::att::AttPlist,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
}
impl crate::generated::model::ModelEventLikeCmn for BeatRpt {}
impl Validate for BeatRpt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

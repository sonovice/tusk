//!Element: `<multiRpt>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///multiple repeat - Multiple repeated measures.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "multiRpt")]
pub struct MultiRpt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub multi_rpt_log: crate::generated::att::AttMultiRptLog,
    #[serde(flatten)]
    pub multi_rpt_vis: crate::generated::att::AttMultiRptVis,
    #[serde(flatten)]
    pub multi_rpt_ges: crate::generated::att::AttMultiRptGes,
    #[serde(flatten)]
    pub multi_rpt_anl: crate::generated::att::AttMultiRptAnl,
}
impl crate::generated::model::ModelEventLikeMeasureFilling for MultiRpt {}
impl Validate for MultiRpt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

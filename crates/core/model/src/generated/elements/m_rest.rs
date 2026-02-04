//!Element: `<mRest>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///measure rest - Complete measure rest in any meter.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mRest")]
pub struct MRest {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub m_rest_log: crate::generated::att::AttMRestLog,
    #[serde(flatten)]
    pub m_rest_vis: crate::generated::att::AttMRestVis,
    #[serde(flatten)]
    pub m_rest_ges: crate::generated::att::AttMRestGes,
    #[serde(flatten)]
    pub m_rest_anl: crate::generated::att::AttMRestAnl,
}
impl crate::generated::model::ModelEventLikeMeasureFilling for MRest {}
impl Validate for MRest {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

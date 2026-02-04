//!Element: `<multiRest>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**multimeasure rest - Multiple full measure rests compressed into a single bar,
frequently found in performer parts.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "multiRest")]
pub struct MultiRest {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub multi_rest_log: crate::generated::att::AttMultiRestLog,
    #[serde(flatten)]
    pub multi_rest_vis: crate::generated::att::AttMultiRestVis,
    #[serde(flatten)]
    pub multi_rest_ges: crate::generated::att::AttMultiRestGes,
    #[serde(flatten)]
    pub multi_rest_anl: crate::generated::att::AttMultiRestAnl,
}
impl crate::generated::model::ModelEventLikeMeasureFilling for MultiRest {}
impl Validate for MultiRest {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<barLine>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Vertical line drawn through one or more staves that divides musical notation into metrical
units.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "barLine")]
pub struct BarLine {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    #[serde(flatten)]
    pub bar_line_anl: crate::generated::att::AttBarLineAnl,
    #[serde(flatten)]
    pub bar_line_ges: crate::generated::att::AttBarLineGes,
    #[serde(flatten)]
    pub bar_line_log: crate::generated::att::AttBarLineLog,
    #[serde(flatten)]
    pub bar_line_vis: crate::generated::att::AttBarLineVis,
}
impl crate::generated::model::ModelEventLike for BarLine {}
impl Validate for BarLine {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

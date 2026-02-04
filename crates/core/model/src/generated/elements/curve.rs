//!Element: `<curve>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**A curved line that cannot be represented by a more specific element, such as a
slur.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "curve")]
pub struct Curve {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub curve_anl: crate::generated::att::AttCurveAnl,
    #[serde(flatten)]
    pub curve_ges: crate::generated::att::AttCurveGes,
    #[serde(flatten)]
    pub curve_log: crate::generated::att::AttCurveLog,
    #[serde(flatten)]
    pub curve_vis: crate::generated::att::AttCurveVis,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
}
impl crate::generated::model::ModelGraphicPrimitiveLike for Curve {}
impl Validate for Curve {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<tupletSpan>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**tuplet span - Alternative element for encoding tuplets, especially useful for tuplets
that extend across bar lines.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tupletSpan")]
pub struct TupletSpan {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub tuplet_span_log: crate::generated::att::AttTupletSpanLog,
    #[serde(flatten)]
    pub tuplet_span_vis: crate::generated::att::AttTupletSpanVis,
    #[serde(flatten)]
    pub tuplet_span_ges: crate::generated::att::AttTupletSpanGes,
    #[serde(flatten)]
    pub tuplet_span_anl: crate::generated::att::AttTupletSpanAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for TupletSpan {}
impl Validate for TupletSpan {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

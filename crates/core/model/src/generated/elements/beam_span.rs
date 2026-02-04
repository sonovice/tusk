//!Element: `<beamSpan>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**beam span - Alternative element for explicitly encoding beams, particularly those which
extend across bar lines.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "beamSpan")]
pub struct BeamSpan {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub beam_span_log: crate::generated::att::AttBeamSpanLog,
    #[serde(flatten)]
    pub beam_span_vis: crate::generated::att::AttBeamSpanVis,
    #[serde(flatten)]
    pub beam_span_ges: crate::generated::att::AttBeamSpanGes,
    #[serde(flatten)]
    pub beam_span_anl: crate::generated::att::AttBeamSpanAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for BeamSpan {}
impl Validate for BeamSpan {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

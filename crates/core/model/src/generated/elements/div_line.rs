//!Element: `<divLine>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**Represents a division (divisio) in neume notation. Divisions indicate short, medium, or long pauses
    similar to breath marks in modern notation.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "divLine")]
pub struct DivLine {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub classed: crate::generated::att::AttClassed,
    #[serde(flatten)]
    pub color: crate::generated::att::AttColor,
    #[serde(flatten)]
    pub div_line_log: crate::generated::att::AttDivLineLog,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub labelled: crate::generated::att::AttLabelled,
    #[serde(flatten)]
    pub linking: crate::generated::att::AttLinking,
    #[serde(flatten)]
    pub n_number_like: crate::generated::att::AttNNumberLike,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    #[serde(flatten)]
    pub ext_sym: crate::generated::att::AttExtSym,
    #[serde(flatten)]
    pub staff_loc: crate::generated::att::AttStaffLoc,
    #[serde(flatten)]
    pub visibility: crate::generated::att::AttVisibility,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    #[serde(flatten)]
    pub visual_offset_ho: crate::generated::att::AttVisualOffsetHo,
}
impl crate::generated::model::ModelEventLikeNeumes for DivLine {}
impl Validate for DivLine {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
    }
}

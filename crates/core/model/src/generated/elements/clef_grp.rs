//!Element: `<clefGrp>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<clefGrp>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClefGrpChild {
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
}
impl ClefGrpChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ClefGrpChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///clef group - A set of simultaneously-occurring clefs.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "clefGrp")]
pub struct ClefGrp {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub event: crate::generated::att::AttEvent,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub clef_grp_anl: crate::generated::att::AttClefGrpAnl,
    #[serde(flatten)]
    pub clef_grp_ges: crate::generated::att::AttClefGrpGes,
    #[serde(flatten)]
    pub clef_grp_log: crate::generated::att::AttClefGrpLog,
    #[serde(flatten)]
    pub clef_grp_vis: crate::generated::att::AttClefGrpVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ClefGrpChild>,
}
impl crate::generated::model::ModelEventLike for ClefGrp {}
impl crate::generated::model::ModelStaffDefPart for ClefGrp {}
impl Validate for ClefGrp {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<grpSym>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<grpSym>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GrpSymChild {
    #[serde(rename = "labelAbbr")]
    LabelAbbr(Box<crate::generated::elements::LabelAbbr>),
    #[serde(rename = "label")]
    Label(Box<crate::generated::elements::Label>),
}
impl GrpSymChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            GrpSymChild::LabelAbbr(elem) => {
                ctx.enter("labelAbbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            GrpSymChild::Label(elem) => {
                ctx.enter("label", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**group symbol - A brace or bracket used to group two or more staves of a score or
      part.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "grpSym")]
pub struct GrpSym {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub grp_sym_anl: crate::generated::att::AttGrpSymAnl,
    #[serde(flatten)]
    pub grp_sym_ges: crate::generated::att::AttGrpSymGes,
    #[serde(flatten)]
    pub grp_sym_log: crate::generated::att::AttGrpSymLog,
    #[serde(flatten)]
    pub grp_sym_vis: crate::generated::att::AttGrpSymVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<GrpSymChild>,
}
impl Validate for GrpSym {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

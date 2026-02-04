//!Element: `<lv>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<lv>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LvChild {
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
}
impl LvChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LvChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///laissez vibrer - A "tie-like" indication that a note should ring beyond its written duration.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lv")]
pub struct Lv {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lv_log: crate::generated::att::AttLvLog,
    #[serde(flatten)]
    pub lv_vis: crate::generated::att::AttLvVis,
    #[serde(flatten)]
    pub lv_ges: crate::generated::att::AttLvGes,
    #[serde(flatten)]
    pub lv_anl: crate::generated::att::AttLvAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LvChild>,
}
impl crate::generated::model::ModelControlEventLikeCmn for Lv {}
impl Validate for Lv {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

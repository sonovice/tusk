//!Element: `<slur>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<slur>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SlurChild {
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
}
impl SlurChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SlurChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Indication of 1) a "unified melodic idea" or 2) performance technique.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "slur")]
pub struct Slur {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub slur_log: crate::generated::att::AttSlurLog,
    #[serde(flatten)]
    pub slur_vis: crate::generated::att::AttSlurVis,
    #[serde(flatten)]
    pub slur_ges: crate::generated::att::AttSlurGes,
    #[serde(flatten)]
    pub slur_anl: crate::generated::att::AttSlurAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SlurChild>,
}
impl crate::generated::model::ModelControlEventLikeCmn for Slur {}
impl Validate for Slur {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

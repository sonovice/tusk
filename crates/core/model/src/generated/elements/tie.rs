//!Element: `<tie>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<tie>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TieChild {
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
}
impl TieChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TieChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**An indication that two notes of the same pitch form a single note with their combined
      rhythmic values.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tie")]
pub struct Tie {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub tie_log: crate::generated::att::AttTieLog,
    #[serde(flatten)]
    pub tie_vis: crate::generated::att::AttTieVis,
    #[serde(flatten)]
    pub tie_ges: crate::generated::att::AttTieGes,
    #[serde(flatten)]
    pub tie_anl: crate::generated::att::AttTieAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TieChild>,
}
impl crate::generated::model::ModelControlEventLikeCmn for Tie {}
impl Validate for Tie {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

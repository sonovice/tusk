//!Element: `<bTrem>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<bTrem>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BTremChild {
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
}
impl BTremChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            BTremChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BTremChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///bowed tremolo - A rapid alternation on a single pitch or chord.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bTrem")]
pub struct BTrem {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub b_trem_log: crate::generated::att::AttBTremLog,
    #[serde(flatten)]
    pub b_trem_vis: crate::generated::att::AttBTremVis,
    #[serde(flatten)]
    pub b_trem_ges: crate::generated::att::AttBTremGes,
    #[serde(flatten)]
    pub b_trem_anl: crate::generated::att::AttBTremAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<BTremChild>,
}
impl crate::generated::model::ModelEventLikeCmn for BTrem {}
impl Validate for BTrem {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

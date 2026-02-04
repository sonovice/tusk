//!Element: `<phrase>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<phrase>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhraseChild {
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
}
impl PhraseChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PhraseChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Indication of 1) a "unified melodic idea" or 2) performance technique.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "phrase")]
pub struct Phrase {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub phrase_anl: crate::generated::att::AttPhraseAnl,
    #[serde(flatten)]
    pub phrase_ges: crate::generated::att::AttPhraseGes,
    #[serde(flatten)]
    pub phrase_log: crate::generated::att::AttPhraseLog,
    #[serde(flatten)]
    pub phrase_vis: crate::generated::att::AttPhraseVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PhraseChild>,
}
impl crate::generated::model::ModelControlEventLike for Phrase {}
impl Validate for Phrase {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

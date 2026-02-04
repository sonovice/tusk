//!Element: `<mei>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<mei>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeiChild {
    #[serde(rename = "meiHead")]
    MeiHead(Box<crate::generated::elements::MeiHead>),
    #[serde(rename = "music")]
    Music(Box<crate::generated::elements::Music>),
}
impl MeiChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MeiChild::MeiHead(elem) => {
                ctx.enter("meiHead", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiChild::Music(elem) => {
                ctx.enter("music", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Contains a single MEI-conformant document, consisting of an MEI header and a musical text,
either in isolation or as part of an meiCorpus element.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mei")]
pub struct Mei {
    #[serde(flatten)]
    pub id: crate::generated::att::AttId,
    #[serde(flatten)]
    pub mei_version: crate::generated::att::AttMeiVersion,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MeiChild>,
}
impl Validate for Mei {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

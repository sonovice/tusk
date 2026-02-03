//!Element: `<meiCorpus>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<meiCorpus>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeiCorpusChild {
    #[serde(rename = "meiHead")]
    MeiHead(Box<crate::generated::elements::MeiHead>),
    #[serde(rename = "mei")]
    Mei(Box<crate::generated::elements::Mei>),
}
impl MeiCorpusChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MeiCorpusChild::MeiHead(elem) => {
                ctx.enter("meiHead", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiCorpusChild::Mei(elem) => {
                ctx.enter("mei", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**MEI corpus - A group of related MEI documents, consisting of a header for the group, and
one or moremeielements, each with its own complete header.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "meiCorpus")]
pub struct MeiCorpus {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub mei_version: crate::generated::att::AttMeiVersion,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MeiCorpusChild>,
}
impl crate::generated::model::ModelStartLikeCorpus for MeiCorpus {}
impl Validate for MeiCorpus {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

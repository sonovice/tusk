//!Element: `<sourceDesc>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<sourceDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceDescChild {
    #[serde(rename = "source")]
    Source(Box<crate::generated::elements::Source>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl SourceDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SourceDescChild::Source(elem) => {
                ctx.enter("source", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SourceDescChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**source description - A container for the descriptions of the source(s) used in the
      creation of the electronic file.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sourceDesc")]
pub struct SourceDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SourceDescChild>,
}
impl Validate for SourceDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<tagsDecl>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<tagsDecl>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TagsDeclChild {
    #[serde(rename = "desc")]
    Desc(Box<crate::generated::elements::Desc>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "namespace")]
    Namespace(Box<crate::generated::elements::Namespace>),
}
impl TagsDeclChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TagsDeclChild::Desc(elem) => {
                ctx.enter("desc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TagsDeclChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TagsDeclChild::Namespace(elem) => {
                ctx.enter("namespace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**tagging declaration - Provides detailed information about the tagging applied to a
document.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tagsDecl")]
pub struct TagsDecl {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TagsDeclChild>,
}
impl Validate for TagsDecl {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

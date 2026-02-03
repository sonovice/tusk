//!Element: `<namespace>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<namespace>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NamespaceChild {
    #[serde(rename = "tagUsage")]
    TagUsage(Box<crate::generated::elements::TagUsage>),
    #[serde(rename = "desc")]
    Desc(Box<crate::generated::elements::Desc>),
    #[serde(rename = "attUsage")]
    AttUsage(Box<crate::generated::elements::AttUsage>),
}
impl NamespaceChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            NamespaceChild::TagUsage(elem) => {
                ctx.enter("tagUsage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            NamespaceChild::Desc(elem) => {
                ctx.enter("desc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            NamespaceChild::AttUsage(elem) => {
                ctx.enter("attUsage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Prefix associated with the formal identifier.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "namespace")]
pub struct Namespace {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<NamespaceChild>,
}
impl Validate for Namespace {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<langUsage>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<langUsage>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LangUsageChild {
    #[serde(rename = "language")]
    Language(Box<crate::generated::elements::Language>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl LangUsageChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LangUsageChild::Language(elem) => {
                ctx.enter("language", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LangUsageChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**language usage - Groups elements describing the languages, sub-languages, dialects,
      etc., represented within the encoded resource.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "langUsage")]
pub struct LangUsage {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LangUsageChild>,
}
impl Validate for LangUsage {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

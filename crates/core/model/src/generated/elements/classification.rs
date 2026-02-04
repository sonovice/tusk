//!Element: `<classification>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<classification>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClassificationChild {
    #[serde(rename = "termList")]
    TermList(Box<crate::generated::elements::TermList>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl ClassificationChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ClassificationChild::TermList(elem) => {
                ctx.enter("termList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ClassificationChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Groups information which describes the nature or topic of an entity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "classification")]
pub struct Classification {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ClassificationChild>,
}
impl Validate for Classification {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

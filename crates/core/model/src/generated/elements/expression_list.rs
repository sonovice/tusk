//!Element: `<expressionList>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<expressionList>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpressionListChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "expression")]
    Expression(Box<crate::generated::elements::Expression>),
}
impl ExpressionListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ExpressionListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionListChild::Expression(elem) => {
                ctx.enter("expression", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Gathers bibliographic expression entities.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "expressionList")]
pub struct ExpressionList {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ExpressionListChild>,
}
impl Validate for ExpressionList {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

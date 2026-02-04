//!Element: `<symProp>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<symProp>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymPropChild {
    #[serde(rename = "propValue")]
    PropValue(Box<crate::generated::elements::PropValue>),
    #[serde(rename = "propName")]
    PropName(Box<crate::generated::elements::PropName>),
}
impl SymPropChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SymPropChild::PropValue(elem) => {
                ctx.enter("propValue", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymPropChild::PropName(elem) => {
                ctx.enter("propName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**symbol property - Provides a name and value for some property of the parent
      symbol.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "symProp")]
pub struct SymProp {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SymPropChild>,
}
impl Validate for SymProp {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

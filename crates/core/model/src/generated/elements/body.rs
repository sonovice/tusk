//!Element: `<body>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<body>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BodyChild {
    #[serde(rename = "div")]
    Div(Box<crate::generated::elements::Div>),
    #[serde(rename = "mdiv")]
    Mdiv(Box<crate::generated::elements::Mdiv>),
}
impl BodyChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            BodyChild::Div(elem) => {
                ctx.enter("div", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BodyChild::Mdiv(elem) => {
                ctx.enter("mdiv", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Contains the whole of a single musical text, excluding any front or back matter.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "body")]
pub struct Body {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<BodyChild>,
}
impl Validate for Body {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

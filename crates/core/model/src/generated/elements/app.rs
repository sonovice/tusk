//!Element: `<app>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<app>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppChild {
    #[serde(rename = "lem")]
    Lem(Box<crate::generated::elements::Lem>),
    #[serde(rename = "rdg")]
    Rdg(Box<crate::generated::elements::Rdg>),
}
impl AppChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AppChild::Lem(elem) => {
                ctx.enter("lem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AppChild::Rdg(elem) => {
                ctx.enter("rdg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///apparatus - Contains one or more alternative encodings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "app")]
pub struct App {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AppChild>,
}
impl crate::generated::model::ModelAppLike for App {}
impl Validate for App {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

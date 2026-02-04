//!Element: `<componentList>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<componentList>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentListChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "item")]
    Item(Box<crate::generated::elements::Item>),
    #[serde(rename = "work")]
    Work(Box<crate::generated::elements::Work>),
    #[serde(rename = "manifestation")]
    Manifestation(Box<crate::generated::elements::Manifestation>),
    #[serde(rename = "expression")]
    Expression(Box<crate::generated::elements::Expression>),
}
impl ComponentListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ComponentListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ComponentListChild::Item(elem) => {
                ctx.enter("item", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ComponentListChild::Work(elem) => {
                ctx.enter("work", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ComponentListChild::Manifestation(elem) => {
                ctx.enter("manifestation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ComponentListChild::Expression(elem) => {
                ctx.enter("expression", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Container for intellectual or physical component parts of a bibliographic entity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "componentList")]
pub struct ComponentList {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ComponentListChild>,
}
impl Validate for ComponentList {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

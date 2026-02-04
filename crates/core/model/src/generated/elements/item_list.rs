//!Element: `<itemList>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<itemList>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemListChild {
    #[serde(rename = "item")]
    Item(Box<crate::generated::elements::Item>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl ItemListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ItemListChild::Item(elem) => {
                ctx.enter("item", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Gathers bibliographic item entities.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "itemList")]
pub struct ItemList {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ItemListChild>,
}
impl Validate for ItemList {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

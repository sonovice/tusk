//!Element: `<category>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<category>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CategoryChild {
    #[serde(rename = "altId")]
    AltId(Box<crate::generated::elements::AltId>),
    #[serde(rename = "desc")]
    Desc(Box<crate::generated::elements::Desc>),
    #[serde(rename = "category")]
    Category(Box<crate::generated::elements::Category>),
    #[serde(rename = "label")]
    Label(Box<crate::generated::elements::Label>),
    #[serde(rename = "catRel")]
    CatRel(Box<crate::generated::elements::CatRel>),
}
impl CategoryChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CategoryChild::AltId(elem) => {
                ctx.enter("altId", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CategoryChild::Desc(elem) => {
                ctx.enter("desc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CategoryChild::Category(elem) => {
                ctx.enter("category", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CategoryChild::Label(elem) => {
                ctx.enter("label", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CategoryChild::CatRel(elem) => {
                ctx.enter("catRel", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Contains an individual descriptive category in a user-defined taxonomy, possibly nested
within a superordinate category.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "category")]
pub struct Category {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CategoryChild>,
}
impl Validate for Category {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

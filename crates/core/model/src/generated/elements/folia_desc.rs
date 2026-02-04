//!Element: `<foliaDesc>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<foliaDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FoliaDescChild {
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "folium")]
    Folium(Box<crate::generated::elements::Folium>),
    #[serde(rename = "bifolium")]
    Bifolium(Box<crate::generated::elements::Bifolium>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
}
impl FoliaDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            FoliaDescChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliaDescChild::Folium(elem) => {
                ctx.enter("folium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliaDescChild::Bifolium(elem) => {
                ctx.enter("bifolium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliaDescChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliaDescChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliaDescChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliaDescChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Describes the order of folia and bifolia making up the text block of a manuscript or
      print.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "foliaDesc")]
pub struct FoliaDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<FoliaDescChild>,
}
impl crate::generated::model::ModelPhysDescPart for FoliaDesc {}
impl Validate for FoliaDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

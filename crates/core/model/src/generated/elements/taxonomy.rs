//!Element: `<taxonomy>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<taxonomy>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaxonomyChild {
    #[serde(rename = "category")]
    Category(Box<crate::generated::elements::Category>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "taxonomy")]
    Taxonomy(Box<crate::generated::elements::Taxonomy>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "desc")]
    Desc(Box<crate::generated::elements::Desc>),
}
impl TaxonomyChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TaxonomyChild::Category(elem) => {
                ctx.enter("category", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TaxonomyChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TaxonomyChild::Taxonomy(elem) => {
                ctx.enter("taxonomy", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TaxonomyChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TaxonomyChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TaxonomyChild::Desc(elem) => {
                ctx.enter("desc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Defines a typology either implicitly, by means of a bibliographic citation, or explicitly
by a structured taxonomy.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "taxonomy")]
pub struct Taxonomy {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TaxonomyChild>,
}
impl Validate for Taxonomy {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

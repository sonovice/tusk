//!Element: `<biblStruct>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<biblStruct>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BiblStructChild {
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "relatedItem")]
    RelatedItem(Box<crate::generated::elements::RelatedItem>),
    #[serde(rename = "series")]
    Series(Box<crate::generated::elements::Series>),
    #[serde(rename = "analytic")]
    Analytic(Box<crate::generated::elements::Analytic>),
    #[serde(rename = "monogr")]
    Monogr(Box<crate::generated::elements::Monogr>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
}
impl BiblStructChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            BiblStructChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblStructChild::RelatedItem(elem) => {
                ctx.enter("relatedItem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblStructChild::Series(elem) => {
                ctx.enter("series", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblStructChild::Analytic(elem) => {
                ctx.enter("analytic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblStructChild::Monogr(elem) => {
                ctx.enter("monogr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblStructChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**structured bibliographic citation - Contains a bibliographic citation in which
bibliographic sub-elements must appear in a specified order.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "biblStruct")]
pub struct BiblStruct {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub record_type: crate::generated::att::AttRecordType,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<BiblStructChild>,
}
impl crate::generated::model::ModelBiblLike for BiblStruct {}
impl Validate for BiblStruct {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<editorialDecl>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<editorialDecl>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditorialDeclChild {
    #[serde(rename = "normalization")]
    Normalization(Box<crate::generated::elements::Normalization>),
    #[serde(rename = "interpretation")]
    Interpretation(Box<crate::generated::elements::Interpretation>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "stdVals")]
    StdVals(Box<crate::generated::elements::StdVals>),
    #[serde(rename = "correction")]
    Correction(Box<crate::generated::elements::Correction>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "segmentation")]
    Segmentation(Box<crate::generated::elements::Segmentation>),
}
impl EditorialDeclChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            EditorialDeclChild::Normalization(elem) => {
                ctx.enter("normalization", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditorialDeclChild::Interpretation(elem) => {
                ctx.enter("interpretation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditorialDeclChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditorialDeclChild::StdVals(elem) => {
                ctx.enter("stdVals", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditorialDeclChild::Correction(elem) => {
                ctx.enter("correction", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditorialDeclChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditorialDeclChild::Segmentation(elem) => {
                ctx.enter("segmentation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**editorial declaration - Used to provide details of editorial principles and practices
applied during the encoding of musical text.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "editorialDecl")]
pub struct EditorialDecl {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<EditorialDeclChild>,
}
impl crate::generated::model::ModelEncodingPart for EditorialDecl {}
impl Validate for EditorialDecl {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

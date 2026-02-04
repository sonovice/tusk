//!Element: `<correction>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<correction>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CorrectionChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
}
impl CorrectionChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CorrectionChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CorrectionChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///States how and under what circumstances corrections have been made in the text.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "correction")]
pub struct Correction {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub regular_method: crate::generated::att::AttRegularMethod,
    ///Indicates the degree of correction applied to the text.
    #[serde(rename = "@corrlevel", skip_serializing_if = "Option::is_none")]
    pub corrlevel: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CorrectionChild>,
}
impl crate::generated::model::ModelEditorialDeclPart for Correction {}
impl Validate for Correction {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

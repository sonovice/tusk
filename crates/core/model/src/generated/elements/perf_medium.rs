//!Element: `<perfMedium>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<perfMedium>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PerfMediumChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "perfResList")]
    PerfResList(Box<crate::generated::elements::PerfResList>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
}
impl PerfMediumChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PerfMediumChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PerfMediumChild::PerfResList(elem) => {
                ctx.enter("perfResList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PerfMediumChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PerfMediumChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**performance medium - Indicates the number and character of the performing forces used in
a musical composition.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "perfMedium")]
pub struct PerfMedium {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PerfMediumChild>,
}
impl crate::generated::model::ModelTitlePagePart for PerfMedium {}
impl Validate for PerfMedium {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

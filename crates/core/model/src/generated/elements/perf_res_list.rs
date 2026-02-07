//!Element: `<perfResList>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<perfResList>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PerfResListChild {
    #[serde(rename = "perfRes")]
    PerfRes(Box<crate::generated::elements::PerfRes>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "perfResList")]
    PerfResList(Box<crate::generated::elements::PerfResList>),
}
impl PerfResListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PerfResListChild::PerfRes(elem) => {
                ctx.enter("perfRes", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PerfResListChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PerfResListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PerfResListChild::PerfResList(elem) => {
                ctx.enter("perfResList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///performance resources list - Several instrumental or vocal resources treated as a group.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "perfResList")]
pub struct PerfResList {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub edit: crate::generated::att::AttEdit,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub perf_res_basic: crate::generated::att::AttPerfResBasic,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PerfResListChild>,
}
impl Validate for PerfResList {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

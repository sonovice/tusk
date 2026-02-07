//!Element: `<custos>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<custos>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustosChild {
    #[serde(rename = "accid")]
    Accid(Box<crate::generated::elements::Accid>),
}
impl CustosChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CustosChild::Accid(elem) => {
                ctx.enter("accid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Symbol placed at the end of a line of music to indicate the first note of the next line.
      Sometimes called a "direct".*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "custos")]
pub struct Custos {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    #[serde(flatten)]
    pub custos_anl: crate::generated::att::AttCustosAnl,
    #[serde(flatten)]
    pub custos_ges: crate::generated::att::AttCustosGes,
    #[serde(flatten)]
    pub custos_log: crate::generated::att::AttCustosLog,
    #[serde(flatten)]
    pub custos_vis: crate::generated::att::AttCustosVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CustosChild>,
}
impl crate::generated::model::ModelEventLike for Custos {}
impl Validate for Custos {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

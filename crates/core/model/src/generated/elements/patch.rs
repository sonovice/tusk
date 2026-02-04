//!Element: `<patch>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<patch>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PatchChild {
    #[serde(rename = "folium")]
    Folium(Box<crate::generated::elements::Folium>),
    #[serde(rename = "bifolium")]
    Bifolium(Box<crate::generated::elements::Bifolium>),
}
impl PatchChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PatchChild::Folium(elem) => {
                ctx.enter("folium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PatchChild::Bifolium(elem) => {
                ctx.enter("bifolium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///patch is attached on surface beneath using a staple.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "patch")]
pub struct Patch {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub evidence: crate::generated::att::AttEvidence,
    #[serde(flatten)]
    pub measurement: crate::generated::att::AttMeasurement,
    #[serde(flatten)]
    pub trans: crate::generated::att::AttTrans,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PatchChild>,
}
impl crate::generated::model::ModelPaperModLike for Patch {}
impl Validate for Patch {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

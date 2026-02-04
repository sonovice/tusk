//!Element: `<patch>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<patch>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PatchChild {
    #[serde(rename = "bifolium")]
    Bifolium(Box<crate::generated::elements::Bifolium>),
    #[serde(rename = "folium")]
    Folium(Box<crate::generated::elements::Folium>),
}
impl PatchChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PatchChild::Bifolium(elem) => {
                ctx.enter("bifolium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PatchChild::Folium(elem) => {
                ctx.enter("folium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Describes a physical writing surface attached to the original document.
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
    ///Describes the position of the patch on the parent folium / bifolium.
    #[serde(rename = "@attached.to", skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    ///Describes the method of attachment of the patch.
    #[serde(rename = "@attached.by", skip_serializing_if = "Option::is_none")]
    pub attached_by: Option<String>,
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

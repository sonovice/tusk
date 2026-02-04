//!Element: `<cutout>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<cutout>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CutoutChild {
    #[serde(rename = "folium")]
    Folium(Box<crate::generated::elements::Folium>),
    #[serde(rename = "bifolium")]
    Bifolium(Box<crate::generated::elements::Bifolium>),
}
impl CutoutChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CutoutChild::Folium(elem) => {
                ctx.enter("folium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CutoutChild::Bifolium(elem) => {
                ctx.enter("bifolium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///A cutout is a section of a document sheet that has been removed and is now missing.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "cutout")]
pub struct Cutout {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub dimensions: crate::generated::att::AttDimensions,
    #[serde(flatten)]
    pub evidence: crate::generated::att::AttEvidence,
    #[serde(flatten)]
    pub measurement: crate::generated::att::AttMeasurement,
    #[serde(flatten)]
    pub trans: crate::generated::att::AttTrans,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    ///Describes the position of the cutout on the parent folium / bifolium.
    #[serde(rename = "@removed.from", skip_serializing_if = "Option::is_none")]
    pub removed_from: Option<String>,
    ///Describes the method of removing the cutout.
    #[serde(rename = "@removed.by", skip_serializing_if = "Option::is_none")]
    pub removed_by: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CutoutChild>,
}
impl crate::generated::model::ModelPaperModLike for Cutout {}
impl Validate for Cutout {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

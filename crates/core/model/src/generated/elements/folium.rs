//!Element: `<folium>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<folium>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FoliumChild {
    #[serde(rename = "cutout")]
    Cutout(Box<crate::generated::elements::Cutout>),
    #[serde(rename = "patch")]
    Patch(Box<crate::generated::elements::Patch>),
}
impl FoliumChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            FoliumChild::Cutout(elem) => {
                ctx.enter("cutout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FoliumChild::Patch(elem) => {
                ctx.enter("patch", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Describes a single leaf of paper.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "folium")]
pub struct Folium {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub dimensions: crate::generated::att::AttDimensions,
    #[serde(flatten)]
    pub measurement: crate::generated::att::AttMeasurement,
    #[serde(flatten)]
    pub folium_surfaces: crate::generated::att::AttFoliumSurfaces,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<FoliumChild>,
}
impl crate::generated::model::ModelFoliumLike for Folium {}
impl Validate for Folium {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<graphic>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<graphic>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GraphicChild {
    #[serde(rename = "zone")]
    Zone(Box<crate::generated::elements::Zone>),
}
impl GraphicChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            GraphicChild::Zone(elem) => {
                ctx.enter("zone", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Indicates the location of an inline graphic.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "graphic")]
pub struct Graphic {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub coordinated_ul: crate::generated::att::AttCoordinatedUl,
    #[serde(flatten)]
    pub dimensions: crate::generated::att::AttDimensions,
    #[serde(flatten)]
    pub internet_media: crate::generated::att::AttInternetMedia,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub start_id: crate::generated::att::AttStartId,
    #[serde(flatten)]
    pub visual_offset: crate::generated::att::AttVisualOffset,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<GraphicChild>,
}
impl crate::generated::model::ModelGraphicLike for Graphic {}
impl Validate for Graphic {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

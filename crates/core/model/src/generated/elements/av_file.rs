//!Element: `<avFile>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<avFile>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AvFileChild {
    #[serde(rename = "clip")]
    Clip(Box<crate::generated::elements::Clip>),
}
impl AvFileChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AvFileChild::Clip(elem) => {
                ctx.enter("clip", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///audio/video file - References an external digital audio or video file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "avFile")]
pub struct AvFile {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub internet_media: crate::generated::att::AttInternetMedia,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AvFileChild>,
}
impl Validate for AvFile {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

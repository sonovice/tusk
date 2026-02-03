//!Element: `<clip>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<clip>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClipChild {
    #[serde(rename = "avFile")]
    AvFile(Box<crate::generated::elements::AvFile>),
    #[serde(rename = "when")]
    When(Box<crate::generated::elements::When>),
}
impl ClipChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ClipChild::AvFile(elem) => {
                ctx.enter("avFile", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ClipChild::When(elem) => {
                ctx.enter("when", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Defines a time segment of interest within a recording or within a digital audio or video
file.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "clip")]
pub struct Clip {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub media_bounds: crate::generated::att::AttMediaBounds,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub start_id: crate::generated::att::AttStartId,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ClipChild>,
}
impl Validate for Clip {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

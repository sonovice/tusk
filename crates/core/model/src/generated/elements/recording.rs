//!Element: `<recording>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<recording>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordingChild {
    #[serde(rename = "clip")]
    Clip(Box<crate::generated::elements::Clip>),
    #[serde(rename = "avFile")]
    AvFile(Box<crate::generated::elements::AvFile>),
    #[serde(rename = "when")]
    When(Box<crate::generated::elements::When>),
}
impl RecordingChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RecordingChild::Clip(elem) => {
                ctx.enter("clip", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RecordingChild::AvFile(elem) => {
                ctx.enter("avFile", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RecordingChild::When(elem) => {
                ctx.enter("when", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///A recorded performance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "recording")]
pub struct Recording {
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
    pub children: Vec<RecordingChild>,
}
impl Validate for Recording {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

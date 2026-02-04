//!Element: `<performance>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<performance>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceChild {
    #[serde(rename = "recording")]
    Recording(Box<crate::generated::elements::Recording>),
}
impl PerformanceChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PerformanceChild::Recording(elem) => {
                ctx.enter("recording", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///A presentation of one or more musical works.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "performance")]
pub struct Performance {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PerformanceChild>,
}
impl crate::generated::model::ModelResourceLike for Performance {}
impl Validate for Performance {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

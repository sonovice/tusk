//!Element: `<parts>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<parts>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartsChild {
    #[serde(rename = "part")]
    Part(Box<crate::generated::elements::Part>),
}
impl PartsChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PartsChild::Part(elem) => {
                ctx.enter("part", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Provides a container for performers' parts.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "parts")]
pub struct Parts {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub parts_anl: crate::generated::att::AttPartsAnl,
    #[serde(flatten)]
    pub parts_ges: crate::generated::att::AttPartsGes,
    #[serde(flatten)]
    pub parts_log: crate::generated::att::AttPartsLog,
    #[serde(flatten)]
    pub parts_vis: crate::generated::att::AttPartsVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PartsChild>,
}
impl crate::generated::model::ModelPartsLike for Parts {}
impl Validate for Parts {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

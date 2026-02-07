//!Element: `<extMeta>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<extMeta>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtMetaChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl ExtMetaChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ExtMetaChild::Text(_) => {}
        }
    }
}
///extended metadata - Provides a container element for non-MEI metadata formats.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "extMeta")]
pub struct ExtMeta {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub whitespace: crate::generated::att::AttWhitespace,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ExtMetaChild>,
}
impl Validate for ExtMeta {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

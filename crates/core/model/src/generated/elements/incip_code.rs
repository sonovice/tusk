//!Element: `<incipCode>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<incipCode>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IncipCodeChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl IncipCodeChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            IncipCodeChild::Text(_) => {}
        }
    }
}
///Parsons code.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "incipCode")]
pub struct IncipCode {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub internet_media: crate::generated::att::AttInternetMedia,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub whitespace: crate::generated::att::AttWhitespace,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<IncipCodeChild>,
}
impl Validate for IncipCode {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

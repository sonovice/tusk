//!Element: `<propValue>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<propValue>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropValueChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl PropValueChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PropValueChild::Text(_) => {}
        }
    }
}
///property value - A single property value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "propValue")]
pub struct PropValue {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PropValueChild>,
}
impl Validate for PropValue {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

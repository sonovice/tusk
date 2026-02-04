//!Element: `<propName>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<propName>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropNameChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl PropNameChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PropNameChild::Text(_) => {}
        }
    }
}
///property name - Name of a property of the symbol.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "propName")]
pub struct PropName {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub labelled: crate::generated::att::AttLabelled,
    #[serde(flatten)]
    pub linking: crate::generated::att::AttLinking,
    #[serde(flatten)]
    pub n_number_like: crate::generated::att::AttNNumberLike,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    ///Characterizes the property name.
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PropNameChild>,
}
impl Validate for PropName {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

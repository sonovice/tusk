//!Element: `<incipCode>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
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
///Incipit coded in a non-XML, plain text format, such as Plaine & Easie Code.
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
    ///Form of the encoded incipit.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
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

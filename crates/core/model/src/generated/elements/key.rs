//!Element: `<key>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<key>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl KeyChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            KeyChild::Text(_) => {}
        }
    }
}
///Key captures information about tonal center and mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "key")]
pub struct Key {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub accidental: crate::generated::att::AttAccidental,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub key_mode: crate::generated::att::AttKeyMode,
    #[serde(flatten)]
    pub pitch: crate::generated::att::AttPitch,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<KeyChild>,
}
impl crate::generated::model::ModelWorkIdent for Key {}
impl Validate for Key {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

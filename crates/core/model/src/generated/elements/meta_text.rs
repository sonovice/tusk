//!Element: `<metaText>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<metaText>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetaTextChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl MetaTextChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MetaTextChild::Text(_) => {}
        }
    }
}
///MIDI text meta-event.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "metaText")]
pub struct MetaText {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MetaTextChild>,
}
impl Validate for MetaText {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<trkName>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<trkName>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrkNameChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl TrkNameChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TrkNameChild::Text(_) => {}
        }
    }
}
///track name - MIDI track/sequence name.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "trkName")]
pub struct TrkName {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TrkNameChild>,
}
impl Validate for TrkName {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

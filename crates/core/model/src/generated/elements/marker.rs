//!Element: `<marker>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<marker>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkerChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl MarkerChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MarkerChild::Text(_) => {}
        }
    }
}
///MIDI marker meta-event.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "marker")]
pub struct Marker {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MarkerChild>,
}
impl Validate for Marker {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<meter>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<meter>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeterChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl MeterChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MeterChild::Text(_) => {}
        }
    }
}
///Captures information about the time signature within bibliographic descriptions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "meter")]
pub struct Meter {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub meter_sig_log: crate::generated::att::AttMeterSigLog,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MeterChild>,
}
impl crate::generated::model::ModelWorkIdent for Meter {}
impl Validate for Meter {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

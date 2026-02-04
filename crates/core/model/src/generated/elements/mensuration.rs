//!Element: `<mensuration>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<mensuration>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MensurationChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
}
impl MensurationChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MensurationChild::Text(_) => {}
        }
    }
}
///Captures information about mensuration within bibliographic descriptions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mensuration")]
pub struct Mensuration {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub mensur_log: crate::generated::att::AttMensurLog,
    #[serde(flatten)]
    pub mensur_vis: crate::generated::att::AttMensurVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MensurationChild>,
}
impl crate::generated::model::ModelWorkIdent for Mensuration {}
impl Validate for Mensuration {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

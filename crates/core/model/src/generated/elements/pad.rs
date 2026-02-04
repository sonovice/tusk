//!Element: `<pad>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///padding - An indication of extra visual space between notational elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pad")]
pub struct Pad {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub pad_anl: crate::generated::att::AttPadAnl,
    #[serde(flatten)]
    pub pad_ges: crate::generated::att::AttPadGes,
    #[serde(flatten)]
    pub pad_log: crate::generated::att::AttPadLog,
    #[serde(flatten)]
    pub pad_vis: crate::generated::att::AttPadVis,
}
impl crate::generated::model::ModelEventLike for Pad {}
impl Validate for Pad {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

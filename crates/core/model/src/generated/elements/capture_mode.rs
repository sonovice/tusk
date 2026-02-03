//!Element: `<captureMode>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**capture mode - The means used to record notation, sound, or images in the production of
a source/manifestation (e.g., analogue, acoustic, electric, digital, optical etc.).*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "captureMode")]
pub struct CaptureMode {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelPhysDescPart for CaptureMode {}
impl Validate for CaptureMode {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<carrierForm>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**carrier form - The specific class of material to which the physical carrier of the
source/manifestation belongs (e.g., sound cassette, videodisc, microfilm cartridge,
transparency, etc.). The carrier for a manifestation comprising multiple physical components
may include more than one form (e.g., a filmstrip with an accompanying booklet, a separate
sound disc carrying the sound track for a film, etc.).*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "carrierForm")]
pub struct CarrierForm {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelPhysDescPart for CarrierForm {}
impl Validate for CarrierForm {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

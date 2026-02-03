//!Element: `<dimensions>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Information about the physical size of an entity; usually includes numerical data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "dimensions")]
pub struct Dimensions {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub evidence: crate::generated::att::AttEvidence,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub measurement: crate::generated::att::AttMeasurement,
}
impl crate::generated::model::ModelPhysDescPart for Dimensions {}
impl crate::generated::model::ModelTextPhraseLikeLimited for Dimensions {}
impl Validate for Dimensions {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

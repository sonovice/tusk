//!Element: `<perfDuration>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///performance duration - Holds a W3C duration value,e.g., "PT2H34M45.67S".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "perfDuration")]
pub struct PerfDuration {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelBiblPart for PerfDuration {}
impl crate::generated::model::ModelPhysDescPart for PerfDuration {}
impl crate::generated::model::ModelTitlePagePart for PerfDuration {}
impl Validate for PerfDuration {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

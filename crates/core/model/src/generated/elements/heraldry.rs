//!Element: `<heraldry>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Contains a heraldic formula or phrase, typically found as part of a blazon, coat of arms,
etc.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "heraldry")]
pub struct Heraldry {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelMsInline for Heraldry {}
impl crate::generated::model::ModelPhysDescPart for Heraldry {}
impl Validate for Heraldry {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

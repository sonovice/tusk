//!Element: `<plateNum>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**plate number - Designation assigned to a resource by a music publisher, usually printed
at the bottom of each page, and sometimes appearing also on the title page.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "plateNum")]
pub struct PlateNum {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelPhysDescPart for PlateNum {}
impl crate::generated::model::ModelTitlePagePart for PlateNum {}
impl Validate for PlateNum {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

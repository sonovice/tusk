//!Element: `<accMat>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Holds a description of any additional material bound with an item, such as
non-contemporaneous documents or fragments.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "accMat")]
pub struct AccMat {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelPhysDescPart for AccMat {}
impl Validate for AccMat {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

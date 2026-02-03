//!Element: `<catchwords>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Describes the system used to ensure correct ordering of the quires making up an item,
typically by means of annotations at the foot of the page.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "catchwords")]
pub struct Catchwords {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelMsInline for Catchwords {}
impl crate::generated::model::ModelPhysDescPart for Catchwords {}
impl Validate for Catchwords {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

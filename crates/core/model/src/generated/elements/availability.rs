//!Element: `<availability>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Groups elements that describe the availability of and access to a bibliographic item,
including an MEI-encoded document.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "availability")]
pub struct Availability {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
}
impl crate::generated::model::ModelPubStmtPart for Availability {}
impl crate::generated::model::ModelImprintPart for Availability {}
impl Validate for Availability {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

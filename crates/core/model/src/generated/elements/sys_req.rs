//!Element: `<sysReq>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///system requirements - System requirements for using the electronic item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sysReq")]
pub struct SysReq {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelTitlePagePart for SysReq {}
impl Validate for SysReq {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

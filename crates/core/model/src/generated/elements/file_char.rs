//!Element: `<fileChar>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**file characteristics - Standards or schemes used to encode the file (e.g., ASCII, SGML,
etc.), physical characteristics of the file (e.g., recording density, parity, blocking, etc.),
and other characteristics that have a bearing on how the file can be processed.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "fileChar")]
pub struct FileChar {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl crate::generated::model::ModelPhysDescPart for FileChar {}
impl Validate for FileChar {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

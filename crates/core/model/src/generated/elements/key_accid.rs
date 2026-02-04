//!Element: `<keyAccid>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///key accidental - Accidental in a key signature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "keyAccid")]
pub struct KeyAccid {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub key_accid_anl: crate::generated::att::AttKeyAccidAnl,
    #[serde(flatten)]
    pub key_accid_ges: crate::generated::att::AttKeyAccidGes,
    #[serde(flatten)]
    pub key_accid_log: crate::generated::att::AttKeyAccidLog,
    #[serde(flatten)]
    pub key_accid_vis: crate::generated::att::AttKeyAccidVis,
    /**Specifies whether enharmonic (written) values or implicit ("perform-able") values are
    allowed.*/
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
}
impl crate::generated::model::ModelKeyAccidLike for KeyAccid {}
impl Validate for KeyAccid {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

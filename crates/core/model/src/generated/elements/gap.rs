//!Element: `<gap>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Indicates a point where material has been omitted in a transcription, whether as part of
sampling practice or for editorial reasons described in the MEI header.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "gap")]
pub struct Gap {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub edit: crate::generated::att::AttEdit,
    #[serde(flatten)]
    pub extent: crate::generated::att::AttExtent,
    #[serde(flatten)]
    pub hand_ident: crate::generated::att::AttHandIdent,
    #[serde(flatten)]
    pub reason_ident: crate::generated::att::AttReasonIdent,
}
impl crate::generated::model::ModelTranscriptionLike for Gap {}
impl Validate for Gap {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

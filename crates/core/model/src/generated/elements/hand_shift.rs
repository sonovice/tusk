//!Element: `<handShift>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**Marks the beginning of a passage written in a new hand, or of a change in the scribe,
      writing style, ink or character of the document hand.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "handShift")]
pub struct HandShift {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub edit: crate::generated::att::AttEdit,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub medium: crate::generated::att::AttMedium,
    ///Describes the character of the new hand.
    #[serde(rename = "@character", skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    /**Identifies the new hand. The value must contain the ID of a hand element given
          elsewhere in the document.*/
    #[serde(rename = "@new", skip_serializing_if = "Option::is_none")]
    pub new: Option<crate::generated::data::DataUri>,
    /**Identifies the old hand. The value must contain the ID of a hand element given
          elsewhere in the document.*/
    #[serde(rename = "@old", skip_serializing_if = "Option::is_none")]
    pub old: Option<crate::generated::data::DataUri>,
}
impl crate::generated::model::ModelTranscriptionLike for HandShift {}
impl Validate for HandShift {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

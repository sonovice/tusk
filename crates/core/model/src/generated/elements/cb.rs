//!Element: `<cb>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///column beginning - Records the column number.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "cb")]
pub struct Cb {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub labelled: crate::generated::att::AttLabelled,
    #[serde(flatten)]
    pub linking: crate::generated::att::AttLinking,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    #[serde(flatten)]
    pub typed: crate::generated::att::AttTyped,
}
impl crate::generated::model::ModelMilestoneLikeMusic for Cb {}
impl crate::generated::model::ModelMilestoneLikeText for Cb {}
impl Validate for Cb {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
    }
}

//!Element: `<fermata>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**An indication placed over a note or rest to indicate that it should be held longer than
its written value. May also occur over a bar line to indicate the end of a phrase or section.
Sometimes called a 'hold' or 'pause'.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "fermata")]
pub struct Fermata {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub fermata_log: crate::generated::att::AttFermataLog,
    #[serde(flatten)]
    pub fermata_vis: crate::generated::att::AttFermataVis,
    #[serde(flatten)]
    pub fermata_ges: crate::generated::att::AttFermataGes,
    #[serde(flatten)]
    pub fermata_anl: crate::generated::att::AttFermataAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for Fermata {}
impl Validate for Fermata {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

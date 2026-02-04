//!Element: `<turn>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**An ornament consisting of four notes — the upper neighbor of the written note, the written
note, the lower neighbor, and the written note.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "turn")]
pub struct Turn {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub turn_anl: crate::generated::att::AttTurnAnl,
    #[serde(flatten)]
    pub turn_ges: crate::generated::att::AttTurnGes,
    #[serde(flatten)]
    pub turn_log: crate::generated::att::AttTurnLog,
    #[serde(flatten)]
    pub turn_vis: crate::generated::att::AttTurnVis,
}
impl crate::generated::model::ModelOrnamentLikeCmn for Turn {}
impl Validate for Turn {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

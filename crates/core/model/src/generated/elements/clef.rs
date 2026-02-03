//!Element: `<clef>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Indication of the exact location of a particular note on the staff and, therefore, the
other notes as well.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "clef")]
pub struct Clef {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub event: crate::generated::att::AttEvent,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub clef_anl: crate::generated::att::AttClefAnl,
    #[serde(flatten)]
    pub clef_ges: crate::generated::att::AttClefGes,
    #[serde(flatten)]
    pub clef_log: crate::generated::att::AttClefLog,
    #[serde(flatten)]
    pub clef_vis: crate::generated::att::AttClefVis,
}
impl crate::generated::model::ModelEventLike for Clef {}
impl crate::generated::model::ModelStaffDefPart for Clef {}
impl crate::generated::model::ModelSyllablePart for Clef {}
impl Validate for Clef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

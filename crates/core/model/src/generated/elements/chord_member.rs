//!Element: `<chordMember>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///An individual pitch in a chord defined by achordDefelement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chordMember")]
pub struct ChordMember {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub chord_member_anl: crate::generated::att::AttChordMemberAnl,
    #[serde(flatten)]
    pub chord_member_ges: crate::generated::att::AttChordMemberGes,
    #[serde(flatten)]
    pub chord_member_log: crate::generated::att::AttChordMemberLog,
    #[serde(flatten)]
    pub chord_member_vis: crate::generated::att::AttChordMemberVis,
}
impl Validate for ChordMember {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

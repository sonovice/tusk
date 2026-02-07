//!Element: `<barre>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///A barre in a chord tablature grid.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "barre")]
pub struct Barre {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub start_end_id: crate::generated::att::AttStartEndId,
    ///This attribute is deprecated in favor oftab.fret, and will be removed in a future version. Records the location at which the strings should be stopped against a fret in a fretboard diagram. This may or may not be the same as the actual location on the fretboard of the instrument in performance.
    #[serde(rename = "@fret", skip_serializing_if = "Option::is_none")]
    pub fret: Option<u64>,
}
impl Validate for Barre {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

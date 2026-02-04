//!Element: `<arpeg>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**arpeggiation - Indicates that the notes of a chord are to be performed successively
      rather than simultaneously, usually from lowest to highest. Sometimes called a "roll".*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "arpeg")]
pub struct Arpeg {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub arpeg_log: crate::generated::att::AttArpegLog,
    #[serde(flatten)]
    pub arpeg_vis: crate::generated::att::AttArpegVis,
    #[serde(flatten)]
    pub arpeg_ges: crate::generated::att::AttArpegGes,
    #[serde(flatten)]
    pub arpeg_anl: crate::generated::att::AttArpegAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for Arpeg {}
impl Validate for Arpeg {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

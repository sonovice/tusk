//!Element: `<meterSig>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///meter signature - Written meter signature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "meterSig")]
pub struct MeterSig {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub meter_sig_anl: crate::generated::att::AttMeterSigAnl,
    #[serde(flatten)]
    pub meter_sig_ges: crate::generated::att::AttMeterSigGes,
    #[serde(flatten)]
    pub meter_sig_log: crate::generated::att::AttMeterSigLog,
    #[serde(flatten)]
    pub meter_sig_vis: crate::generated::att::AttMeterSigVis,
}
impl crate::generated::model::ModelMeterSigLike for MeterSig {}
impl Validate for MeterSig {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

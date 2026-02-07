//!Element: `<meterSigGrp>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<meterSigGrp>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeterSigGrpChild {
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
}
impl MeterSigGrpChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MeterSigGrpChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeterSigGrpChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///meter signature group - Used to capture alternating, interchanging, mixed or other non-standard meter signatures.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "meterSigGrp")]
pub struct MeterSigGrp {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub meter_sig_grp_anl: crate::generated::att::AttMeterSigGrpAnl,
    #[serde(flatten)]
    pub meter_sig_grp_ges: crate::generated::att::AttMeterSigGrpGes,
    #[serde(flatten)]
    pub meter_sig_grp_log: crate::generated::att::AttMeterSigGrpLog,
    #[serde(flatten)]
    pub meter_sig_grp_vis: crate::generated::att::AttMeterSigGrpVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MeterSigGrpChild>,
}
impl crate::generated::model::ModelMeterSigLike for MeterSigGrp {}
impl Validate for MeterSigGrp {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

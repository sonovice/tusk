//!Element: `<staffDef>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<staffDef>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaffDefChild {
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "layerDef")]
    LayerDef(Box<crate::generated::elements::LayerDef>),
    #[serde(rename = "tuning")]
    Tuning(Box<crate::generated::elements::Tuning>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "ambitus")]
    Ambitus(Box<crate::generated::elements::Ambitus>),
    #[serde(rename = "label")]
    Label(Box<crate::generated::elements::Label>),
    #[serde(rename = "labelAbbr")]
    LabelAbbr(Box<crate::generated::elements::LabelAbbr>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "instrDef")]
    InstrDef(Box<crate::generated::elements::InstrDef>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
}
impl StaffDefChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            StaffDefChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::LayerDef(elem) => {
                ctx.enter("layerDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::Tuning(elem) => {
                ctx.enter("tuning", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::Ambitus(elem) => {
                ctx.enter("ambitus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::Label(elem) => {
                ctx.enter("label", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::LabelAbbr(elem) => {
                ctx.enter("labelAbbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::InstrDef(elem) => {
                ctx.enter("instrDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffDefChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///staff definition - Container for staff meta-information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "staffDef")]
pub struct StaffDef {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub labelled: crate::generated::att::AttLabelled,
    #[serde(flatten)]
    pub linking: crate::generated::att::AttLinking,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub n_integer: crate::generated::att::AttNInteger,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    #[serde(flatten)]
    pub typed: crate::generated::att::AttTyped,
    #[serde(flatten)]
    pub staff_def_anl: crate::generated::att::AttStaffDefAnl,
    #[serde(flatten)]
    pub staff_def_ges: crate::generated::att::AttStaffDefGes,
    #[serde(flatten)]
    pub staff_def_log: crate::generated::att::AttStaffDefLog,
    #[serde(flatten)]
    pub staff_def_vis: crate::generated::att::AttStaffDefVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<StaffDefChild>,
}
impl crate::generated::model::ModelStaffDefLike for StaffDef {}
impl Validate for StaffDef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

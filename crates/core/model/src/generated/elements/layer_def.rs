//!Element: `<layerDef>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<layerDef>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerDefChild {
    #[serde(rename = "labelAbbr")]
    LabelAbbr(Box<crate::generated::elements::LabelAbbr>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "instrDef")]
    InstrDef(Box<crate::generated::elements::InstrDef>),
    #[serde(rename = "ambitus")]
    Ambitus(Box<crate::generated::elements::Ambitus>),
    #[serde(rename = "label")]
    Label(Box<crate::generated::elements::Label>),
}
impl LayerDefChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LayerDefChild::LabelAbbr(elem) => {
                ctx.enter("labelAbbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerDefChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerDefChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerDefChild::InstrDef(elem) => {
                ctx.enter("instrDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerDefChild::Ambitus(elem) => {
                ctx.enter("ambitus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerDefChild::Label(elem) => {
                ctx.enter("label", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///layer definition - Container for layer meta-information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "layerDef")]
pub struct LayerDef {
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
    pub layer_def_anl: crate::generated::att::AttLayerDefAnl,
    #[serde(flatten)]
    pub layer_def_ges: crate::generated::att::AttLayerDefGes,
    #[serde(flatten)]
    pub layer_def_log: crate::generated::att::AttLayerDefLog,
    #[serde(flatten)]
    pub layer_def_vis: crate::generated::att::AttLayerDefVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LayerDefChild>,
}
impl crate::generated::model::ModelLayerDefLike for LayerDef {}
impl Validate for LayerDef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<layer>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<layer>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerChild {
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "tabDurSym")]
    TabDurSym(Box<crate::generated::elements::TabDurSym>),
    #[serde(rename = "mSpace")]
    MSpace(Box<crate::generated::elements::MSpace>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "neume")]
    Neume(Box<crate::generated::elements::Neume>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
    #[serde(rename = "divLine")]
    DivLine(Box<crate::generated::elements::DivLine>),
    #[serde(rename = "mRpt")]
    MRpt(Box<crate::generated::elements::MRpt>),
    #[serde(rename = "sb")]
    Sb(Box<crate::generated::elements::Sb>),
    #[serde(rename = "mRest")]
    MRest(Box<crate::generated::elements::MRest>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "halfmRpt")]
    HalfmRpt(Box<crate::generated::elements::HalfmRpt>),
    #[serde(rename = "midi")]
    Midi(Box<crate::generated::elements::Midi>),
    #[serde(rename = "multiRest")]
    MultiRest(Box<crate::generated::elements::MultiRest>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "app")]
    App(Box<crate::generated::elements::App>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "barLine")]
    BarLine(Box<crate::generated::elements::BarLine>),
    #[serde(rename = "pad")]
    Pad(Box<crate::generated::elements::Pad>),
    #[serde(rename = "beatRpt")]
    BeatRpt(Box<crate::generated::elements::BeatRpt>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "artic")]
    Artic(Box<crate::generated::elements::Artic>),
    #[serde(rename = "beam")]
    Beam(Box<crate::generated::elements::Beam>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "dot")]
    Dot(Box<crate::generated::elements::Dot>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "syllable")]
    Syllable(Box<crate::generated::elements::Syllable>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "rest")]
    Rest(Box<crate::generated::elements::Rest>),
    #[serde(rename = "fTrem")]
    FTrem(Box<crate::generated::elements::FTrem>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "ligature")]
    Ligature(Box<crate::generated::elements::Ligature>),
    #[serde(rename = "custos")]
    Custos(Box<crate::generated::elements::Custos>),
    #[serde(rename = "tuplet")]
    Tuplet(Box<crate::generated::elements::Tuplet>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "tabGrp")]
    TabGrp(Box<crate::generated::elements::TabGrp>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "multiRpt")]
    MultiRpt(Box<crate::generated::elements::MultiRpt>),
    #[serde(rename = "graceGrp")]
    GraceGrp(Box<crate::generated::elements::GraceGrp>),
    #[serde(rename = "accid")]
    Accid(Box<crate::generated::elements::Accid>),
    #[serde(rename = "bTrem")]
    BTrem(Box<crate::generated::elements::BTrem>),
    #[serde(rename = "mRpt2")]
    MRpt2(Box<crate::generated::elements::MRpt2>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
}
impl LayerChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LayerChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::TabDurSym(elem) => {
                ctx.enter("tabDurSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MSpace(elem) => {
                ctx.enter("mSpace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Neume(elem) => {
                ctx.enter("neume", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::DivLine(elem) => {
                ctx.enter("divLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MRpt(elem) => {
                ctx.enter("mRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Sb(elem) => {
                ctx.enter("sb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MRest(elem) => {
                ctx.enter("mRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::HalfmRpt(elem) => {
                ctx.enter("halfmRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Midi(elem) => {
                ctx.enter("midi", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MultiRest(elem) => {
                ctx.enter("multiRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::App(elem) => {
                ctx.enter("app", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::BarLine(elem) => {
                ctx.enter("barLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Pad(elem) => {
                ctx.enter("pad", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::BeatRpt(elem) => {
                ctx.enter("beatRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Artic(elem) => {
                ctx.enter("artic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Beam(elem) => {
                ctx.enter("beam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Dot(elem) => {
                ctx.enter("dot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Syllable(elem) => {
                ctx.enter("syllable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Rest(elem) => {
                ctx.enter("rest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::FTrem(elem) => {
                ctx.enter("fTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Ligature(elem) => {
                ctx.enter("ligature", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Custos(elem) => {
                ctx.enter("custos", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Tuplet(elem) => {
                ctx.enter("tuplet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::TabGrp(elem) => {
                ctx.enter("tabGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MultiRpt(elem) => {
                ctx.enter("multiRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::GraceGrp(elem) => {
                ctx.enter("graceGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Accid(elem) => {
                ctx.enter("accid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::BTrem(elem) => {
                ctx.enter("bTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::MRpt2(elem) => {
                ctx.enter("mRpt2", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LayerChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///An independent stream of events on a staff.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "layer")]
pub struct Layer {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
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
    pub layer_anl: crate::generated::att::AttLayerAnl,
    #[serde(flatten)]
    pub layer_ges: crate::generated::att::AttLayerGes,
    #[serde(flatten)]
    pub layer_log: crate::generated::att::AttLayerLog,
    #[serde(flatten)]
    pub layer_vis: crate::generated::att::AttLayerVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LayerChild>,
}
impl crate::generated::model::ModelLayerLike for Layer {}
impl Validate for Layer {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

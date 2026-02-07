//!Element: `<ligature>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<ligature>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LigatureChild {
    #[serde(rename = "custos")]
    Custos(Box<crate::generated::elements::Custos>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "app")]
    App(Box<crate::generated::elements::App>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
    #[serde(rename = "tabGrp")]
    TabGrp(Box<crate::generated::elements::TabGrp>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "rest")]
    Rest(Box<crate::generated::elements::Rest>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "ligature")]
    Ligature(Box<crate::generated::elements::Ligature>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "pad")]
    Pad(Box<crate::generated::elements::Pad>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "dot")]
    Dot(Box<crate::generated::elements::Dot>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "tabDurSym")]
    TabDurSym(Box<crate::generated::elements::TabDurSym>),
    #[serde(rename = "neume")]
    Neume(Box<crate::generated::elements::Neume>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "barLine")]
    BarLine(Box<crate::generated::elements::BarLine>),
    #[serde(rename = "divLine")]
    DivLine(Box<crate::generated::elements::DivLine>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
}
impl LigatureChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LigatureChild::Custos(elem) => {
                ctx.enter("custos", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::App(elem) => {
                ctx.enter("app", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::TabGrp(elem) => {
                ctx.enter("tabGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Rest(elem) => {
                ctx.enter("rest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Ligature(elem) => {
                ctx.enter("ligature", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Pad(elem) => {
                ctx.enter("pad", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Dot(elem) => {
                ctx.enter("dot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::TabDurSym(elem) => {
                ctx.enter("tabDurSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Neume(elem) => {
                ctx.enter("neume", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::BarLine(elem) => {
                ctx.enter("barLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::DivLine(elem) => {
                ctx.enter("divLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LigatureChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///A mensural notation symbol that combines two or more notes into a single sign.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ligature")]
pub struct Ligature {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub ligature_log: crate::generated::att::AttLigatureLog,
    #[serde(flatten)]
    pub ligature_vis: crate::generated::att::AttLigatureVis,
    #[serde(flatten)]
    pub ligature_ges: crate::generated::att::AttLigatureGes,
    #[serde(flatten)]
    pub ligature_anl: crate::generated::att::AttLigatureAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LigatureChild>,
}
impl crate::generated::model::ModelEventLikeMensural for Ligature {}
impl Validate for Ligature {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

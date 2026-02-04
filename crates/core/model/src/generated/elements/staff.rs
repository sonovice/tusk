//!Element: `<staff>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<staff>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaffChild {
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "sb")]
    Sb(Box<crate::generated::elements::Sb>),
    #[serde(rename = "metaMark")]
    MetaMark(Box<crate::generated::elements::MetaMark>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "fing")]
    Fing(Box<crate::generated::elements::Fing>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "dynam")]
    Dynam(Box<crate::generated::elements::Dynam>),
    #[serde(rename = "app")]
    App(Box<crate::generated::elements::App>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "gliss")]
    Gliss(Box<crate::generated::elements::Gliss>),
    #[serde(rename = "cpMark")]
    CpMark(Box<crate::generated::elements::CpMark>),
    #[serde(rename = "ornam")]
    Ornam(Box<crate::generated::elements::Ornam>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "repeatMark")]
    RepeatMark(Box<crate::generated::elements::RepeatMark>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "staffDef")]
    StaffDef(Box<crate::generated::elements::StaffDef>),
    #[serde(rename = "harm")]
    Harm(Box<crate::generated::elements::Harm>),
    #[serde(rename = "phrase")]
    Phrase(Box<crate::generated::elements::Phrase>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "bend")]
    Bend(Box<crate::generated::elements::Bend>),
    #[serde(rename = "caesura")]
    Caesura(Box<crate::generated::elements::Caesura>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "dir")]
    Dir(Box<crate::generated::elements::Dir>),
    #[serde(rename = "sp")]
    Sp(Box<crate::generated::elements::Sp>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "stageDir")]
    StageDir(Box<crate::generated::elements::StageDir>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "layer")]
    Layer(Box<crate::generated::elements::Layer>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "ossia")]
    Ossia(Box<crate::generated::elements::Ossia>),
    #[serde(rename = "fingGrp")]
    FingGrp(Box<crate::generated::elements::FingGrp>),
}
impl StaffChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            StaffChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Sb(elem) => {
                ctx.enter("sb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::MetaMark(elem) => {
                ctx.enter("metaMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Fing(elem) => {
                ctx.enter("fing", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Dynam(elem) => {
                ctx.enter("dynam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::App(elem) => {
                ctx.enter("app", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Gliss(elem) => {
                ctx.enter("gliss", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::CpMark(elem) => {
                ctx.enter("cpMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Ornam(elem) => {
                ctx.enter("ornam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::RepeatMark(elem) => {
                ctx.enter("repeatMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::StaffDef(elem) => {
                ctx.enter("staffDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Harm(elem) => {
                ctx.enter("harm", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Phrase(elem) => {
                ctx.enter("phrase", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Bend(elem) => {
                ctx.enter("bend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Caesura(elem) => {
                ctx.enter("caesura", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Dir(elem) => {
                ctx.enter("dir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Sp(elem) => {
                ctx.enter("sp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::StageDir(elem) => {
                ctx.enter("stageDir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Layer(elem) => {
                ctx.enter("layer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::Ossia(elem) => {
                ctx.enter("ossia", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffChild::FingGrp(elem) => {
                ctx.enter("fingGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**A group of equidistant horizontal lines on which notes are placed in order to represent
pitch or a grouping element for individual 'strands' of notes, rests, etc. that may or may not
actually be rendered on staff lines; that is, both diastematic and non-diastematic
signs.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "staff")]
pub struct Staff {
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
    pub staff_log: crate::generated::att::AttStaffLog,
    #[serde(flatten)]
    pub staff_vis: crate::generated::att::AttStaffVis,
    #[serde(flatten)]
    pub staff_ges: crate::generated::att::AttStaffGes,
    #[serde(flatten)]
    pub staff_anl: crate::generated::att::AttStaffAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<StaffChild>,
}
impl crate::generated::model::ModelStaffLike for Staff {}
impl Validate for Staff {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

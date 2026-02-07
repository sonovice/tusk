//!Element: `<scoreDef>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<scoreDef>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScoreDefChild {
    #[serde(rename = "instrGrp")]
    InstrGrp(Box<crate::generated::elements::InstrGrp>),
    #[serde(rename = "chordTable")]
    ChordTable(Box<crate::generated::elements::ChordTable>),
    #[serde(rename = "symbolTable")]
    SymbolTable(Box<crate::generated::elements::SymbolTable>),
    #[serde(rename = "pgFoot")]
    PgFoot(Box<crate::generated::elements::PgFoot>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "staffGrp")]
    StaffGrp(Box<crate::generated::elements::StaffGrp>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "grpSym")]
    GrpSym(Box<crate::generated::elements::GrpSym>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "ambitus")]
    Ambitus(Box<crate::generated::elements::Ambitus>),
    #[serde(rename = "pgHead")]
    PgHead(Box<crate::generated::elements::PgHead>),
}
impl ScoreDefChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ScoreDefChild::InstrGrp(elem) => {
                ctx.enter("instrGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::ChordTable(elem) => {
                ctx.enter("chordTable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::SymbolTable(elem) => {
                ctx.enter("symbolTable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::PgFoot(elem) => {
                ctx.enter("pgFoot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::StaffGrp(elem) => {
                ctx.enter("staffGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::GrpSym(elem) => {
                ctx.enter("grpSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::Ambitus(elem) => {
                ctx.enter("ambitus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ScoreDefChild::PgHead(elem) => {
                ctx.enter("pgHead", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///score definition - Container for score meta-information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "scoreDef")]
pub struct ScoreDef {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub score_def_anl: crate::generated::att::AttScoreDefAnl,
    #[serde(flatten)]
    pub score_def_ges: crate::generated::att::AttScoreDefGes,
    #[serde(flatten)]
    pub score_def_log: crate::generated::att::AttScoreDefLog,
    #[serde(flatten)]
    pub score_def_vis: crate::generated::att::AttScoreDefVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ScoreDefChild>,
}
impl crate::generated::model::ModelScoreDefLike for ScoreDef {}
impl Validate for ScoreDef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

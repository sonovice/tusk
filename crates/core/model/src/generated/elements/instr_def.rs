//!Element: `<instrDef>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///instrument definition - MIDI instrument declaration.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "instrDef")]
pub struct InstrDef {
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
    pub instr_def_anl: crate::generated::att::AttInstrDefAnl,
    #[serde(flatten)]
    pub instr_def_ges: crate::generated::att::AttInstrDefGes,
    #[serde(flatten)]
    pub instr_def_log: crate::generated::att::AttInstrDefLog,
    #[serde(flatten)]
    pub instr_def_vis: crate::generated::att::AttInstrDefVis,
}
impl crate::generated::model::ModelInstrDefLike for InstrDef {}
impl Validate for InstrDef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
    }
}

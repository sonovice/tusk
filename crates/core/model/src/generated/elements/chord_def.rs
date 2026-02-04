//!Element: `<chordDef>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<chordDef>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChordDefChild {
    #[serde(rename = "barre")]
    Barre(Box<crate::generated::elements::Barre>),
    #[serde(rename = "chordMember")]
    ChordMember(Box<crate::generated::elements::ChordMember>),
}
impl ChordDefChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ChordDefChild::Barre(elem) => {
                ctx.enter("barre", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ChordDefChild::ChordMember(elem) => {
                ctx.enter("chordMember", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///chord definition - Chord tablature definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chordDef")]
pub struct ChordDef {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub chord_def_anl: crate::generated::att::AttChordDefAnl,
    #[serde(flatten)]
    pub chord_def_ges: crate::generated::att::AttChordDefGes,
    #[serde(flatten)]
    pub chord_def_log: crate::generated::att::AttChordDefLog,
    #[serde(flatten)]
    pub chord_def_vis: crate::generated::att::AttChordDefVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ChordDefChild>,
}
impl Validate for ChordDef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

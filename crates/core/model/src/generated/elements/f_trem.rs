//!Element: `<fTrem>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<fTrem>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FTremChild {
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
}
impl FTremChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            FTremChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FTremChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FTremChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**fingered tremolo - A rapid alternation between a pair of notes (or chords or perhaps
between a note and a chord) that are (usually) farther apart than a major second.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "fTrem")]
pub struct FTrem {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub f_trem_log: crate::generated::att::AttFTremLog,
    #[serde(flatten)]
    pub f_trem_vis: crate::generated::att::AttFTremVis,
    #[serde(flatten)]
    pub f_trem_ges: crate::generated::att::AttFTremGes,
    #[serde(flatten)]
    pub f_trem_anl: crate::generated::att::AttFTremAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<FTremChild>,
}
impl crate::generated::model::ModelEventLikeCmn for FTrem {}
impl Validate for FTrem {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

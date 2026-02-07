//!Element: `<chordTable>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<chordTable>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChordTableChild {
    #[serde(rename = "chordDef")]
    ChordDef(Box<crate::generated::elements::ChordDef>),
}
impl ChordTableChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ChordTableChild::ChordDef(elem) => {
                ctx.enter("chordDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Chord/tablature look-up table.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chordTable")]
pub struct ChordTable {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ChordTableChild>,
}
impl crate::generated::model::ModelChordTableLike for ChordTable {}
impl Validate for ChordTable {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

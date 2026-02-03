//!Element: `<instrGrp>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<instrGrp>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrGrpChild {
    #[serde(rename = "instrDef")]
    InstrDef(Box<crate::generated::elements::InstrDef>),
}
impl InstrGrpChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            InstrGrpChild::InstrDef(elem) => {
                ctx.enter("instrDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///instrument group - Collects MIDI instrument definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "instrGrp")]
pub struct InstrGrp {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<InstrGrpChild>,
}
impl Validate for InstrGrp {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

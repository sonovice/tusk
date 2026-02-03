//!Element: `<tuning>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<tuning>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TuningChild {
    #[serde(rename = "course")]
    Course(Box<crate::generated::elements::Course>),
}
impl TuningChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TuningChild::Course(elem) => {
                ctx.enter("course", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Describes the tuning of an instrument.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tuning")]
pub struct Tuning {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub tuning_log: crate::generated::att::AttTuningLog,
    #[serde(flatten)]
    pub tuning_vis: crate::generated::att::AttTuningVis,
    #[serde(flatten)]
    pub tuning_ges: crate::generated::att::AttTuningGes,
    #[serde(flatten)]
    pub tuning_anl: crate::generated::att::AttTuningAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TuningChild>,
}
impl crate::generated::model::ModelStaffDefPart for Tuning {}
impl Validate for Tuning {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

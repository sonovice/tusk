//!Element: `<reh>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<reh>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RehChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
}
impl RehChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RehChild::Text(_) => {}
            RehChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RehChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RehChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**rehearsal mark - In an orchestral score and its corresponding parts, a mark indicating a
convenient point from which to resume rehearsal after a break.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "reh")]
pub struct Reh {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub reh_log: crate::generated::att::AttRehLog,
    #[serde(flatten)]
    pub reh_vis: crate::generated::att::AttRehVis,
    #[serde(flatten)]
    pub reh_ges: crate::generated::att::AttRehGes,
    #[serde(flatten)]
    pub reh_anl: crate::generated::att::AttRehAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<RehChild>,
}
impl crate::generated::model::ModelControlEventLikeCmn for Reh {}
impl Validate for Reh {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

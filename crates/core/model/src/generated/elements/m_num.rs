//!Element: `<mNum>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<mNum>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MNumChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
}
impl MNumChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MNumChild::Text(_) => {}
            MNumChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MNumChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MNumChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**measure number - Designation, name, or label for a measure, often but not always
      consisting of digits. Use this element when thenattribute onmeasuredoes not adequately capture the appearance or placement of the measure
      number/label.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mNum")]
pub struct MNum {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub m_num_log: crate::generated::att::AttMNumLog,
    #[serde(flatten)]
    pub m_num_vis: crate::generated::att::AttMNumVis,
    #[serde(flatten)]
    pub m_num_ges: crate::generated::att::AttMNumGes,
    #[serde(flatten)]
    pub m_num_anl: crate::generated::att::AttMNumAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MNumChild>,
}
impl Validate for MNum {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

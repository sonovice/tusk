//!Element: `<when>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<when>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WhenChild {
    #[serde(rename = "extData")]
    ExtData(Box<crate::generated::elements::ExtData>),
}
impl WhenChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            WhenChild::ExtData(elem) => {
                ctx.enter("extData", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Identifies the reference point for determining the time of the current when element,
which is obtained by adding the interval to the time of the reference point. The value
should be the ID of another when element within the same parent element. If the since
attribute is omitted and the absolute attribute is not specified, then the reference point
is understood to be the immediately preceding when element.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "when")]
pub struct When {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<WhenChild>,
}
impl Validate for When {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<attUsage>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<attUsage>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttUsageChild {
    #[serde(rename = "desc")]
    Desc(Box<crate::generated::elements::Desc>),
}
impl AttUsageChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AttUsageChild::Desc(elem) => {
                ctx.enter("desc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Documents the usage of a specific attribute of the element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "attUsage")]
pub struct AttUsage {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    ///Name of the attribute.
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::generated::data::DataNmtoken>,
    ///Circumstances in which the attribute appears, an XPath expression.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AttUsageChild>,
}
impl Validate for AttUsage {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

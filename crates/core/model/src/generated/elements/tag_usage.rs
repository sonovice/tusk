//!Element: `<tagUsage>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<tagUsage>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TagUsageChild {
    #[serde(rename = "attUsage")]
    AttUsage(Box<crate::generated::elements::AttUsage>),
    #[serde(rename = "desc")]
    Desc(Box<crate::generated::elements::Desc>),
}
impl TagUsageChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TagUsageChild::AttUsage(elem) => {
                ctx.enter("attUsage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TagUsageChild::Desc(elem) => {
                ctx.enter("desc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Documents the usage of a specific element within the document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tagUsage")]
pub struct TagUsage {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    ///Name of the element.
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::generated::data::DataNmtoken>,
    ///Circumstances in which the element appears, an XPath expression.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    ///Number of occurrences in the defined context.
    #[serde(rename = "@occurs", skip_serializing_if = "Option::is_none")]
    pub occurs: Option<u64>,
    ///Number of occurrences in the defined context that have anxml:idattribute.
    #[serde(rename = "@withid", skip_serializing_if = "Option::is_none")]
    pub withid: Option<u64>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TagUsageChild>,
}
impl Validate for TagUsage {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

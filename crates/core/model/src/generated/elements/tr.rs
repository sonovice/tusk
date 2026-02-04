//!Element: `<tr>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<tr>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrChild {
    #[serde(rename = "td")]
    Td(Box<crate::generated::elements::Td>),
    #[serde(rename = "th")]
    Th(Box<crate::generated::elements::Th>),
}
impl TrChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TrChild::Td(elem) => {
                ctx.enter("td", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TrChild::Th(elem) => {
                ctx.enter("th", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**table row - A formatting element that contains one or more cells (intersection of a row
and a column) in atable.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tr")]
pub struct Tr {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TrChild>,
}
impl Validate for Tr {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

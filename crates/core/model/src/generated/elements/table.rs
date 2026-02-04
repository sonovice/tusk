//!Element: `<table>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<table>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TableChild {
    #[serde(rename = "caption")]
    Caption(Box<crate::generated::elements::Caption>),
    #[serde(rename = "tr")]
    Tr(Box<crate::generated::elements::Tr>),
}
impl TableChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TableChild::Caption(elem) => {
                ctx.enter("caption", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TableChild::Tr(elem) => {
                ctx.enter("tr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Contains text displayed in tabular form.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "table")]
pub struct Table {
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
    pub children: Vec<TableChild>,
}
impl crate::generated::model::ModelTableLike for Table {}
impl Validate for Table {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

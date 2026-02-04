//!Element: `<change>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<change>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeChild {
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "changeDesc")]
    ChangeDesc(Box<crate::generated::elements::ChangeDesc>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
}
impl ChangeChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ChangeChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ChangeChild::ChangeDesc(elem) => {
                ctx.enter("changeDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ChangeChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Individual change within the revision description.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "change")]
pub struct Change {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub datable: crate::generated::att::AttDatable,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ChangeChild>,
}
impl Validate for Change {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

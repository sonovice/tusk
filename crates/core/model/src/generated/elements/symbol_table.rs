//!Element: `<symbolTable>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<symbolTable>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolTableChild {
    #[serde(rename = "symbolDef")]
    SymbolDef(Box<crate::generated::elements::SymbolDef>),
}
impl SymbolTableChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SymbolTableChild::SymbolDef(elem) => {
                ctx.enter("symbolDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Contains a set of user-defined symbols.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "symbolTable")]
pub struct SymbolTable {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SymbolTableChild>,
}
impl crate::generated::model::ModelSymbolTableLike for SymbolTable {}
impl Validate for SymbolTable {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

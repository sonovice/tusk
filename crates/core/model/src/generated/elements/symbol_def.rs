//!Element: `<symbolDef>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<symbolDef>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolDefChild {
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "mapping")]
    Mapping(Box<crate::generated::elements::Mapping>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "symProp")]
    SymProp(Box<crate::generated::elements::SymProp>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "graphic")]
    Graphic(Box<crate::generated::elements::Graphic>),
    #[serde(rename = "symName")]
    SymName(Box<crate::generated::elements::SymName>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
}
impl SymbolDefChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SymbolDefChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::Mapping(elem) => {
                ctx.enter("mapping", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::SymProp(elem) => {
                ctx.enter("symProp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::Graphic(elem) => {
                ctx.enter("graphic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::SymName(elem) => {
                ctx.enter("symName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SymbolDefChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///symbol definition - Declaration of an individual symbol in a symbolTable.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "symbolDef")]
pub struct SymbolDef {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub coordinated: crate::generated::att::AttCoordinated,
    #[serde(flatten)]
    pub data_selecting: crate::generated::att::AttDataSelecting,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SymbolDefChild>,
}
impl Validate for SymbolDef {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

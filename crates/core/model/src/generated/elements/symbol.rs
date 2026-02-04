//!Element: `<symbol>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///A reference to a previously defined symbol.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "symbol")]
pub struct Symbol {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub symbol_anl: crate::generated::att::AttSymbolAnl,
    #[serde(flatten)]
    pub symbol_ges: crate::generated::att::AttSymbolGes,
    #[serde(flatten)]
    pub symbol_log: crate::generated::att::AttSymbolLog,
    #[serde(flatten)]
    pub symbol_vis: crate::generated::att::AttSymbolVis,
}
impl crate::generated::model::ModelTextPhraseLikeLimited for Symbol {}
impl Validate for Symbol {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

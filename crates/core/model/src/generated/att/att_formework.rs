//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record the function (i.e., placement) of forme work elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFormework {
    ///Records the function (i.e., placement) of a page header or footer.
    #[serde(rename = "@func", skip_serializing_if = "Option::is_none")]
    pub func: Option<crate::generated::data::DataPgfunc>,
}

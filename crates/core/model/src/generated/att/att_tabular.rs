//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes shared by table cells.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTabular {
    ///The number of columns spanned by this cell.
    #[serde(rename = "@colspan", skip_serializing_if = "Option::is_none")]
    pub colspan: Option<u64>,
    ///The number of rows spanned by this cell.
    #[serde(rename = "@rowspan", skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<u64>,
}

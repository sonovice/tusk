//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttGuitarGridVis {
    ///Determines whether to display guitar chord grids.
    #[serde(rename = "@grid.show", skip_serializing_if = "Option::is_none")]
    pub grid_show: Option<crate::generated::data::DataBoolean>,
}

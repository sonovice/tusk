//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///String tablature position information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStringtabPosition {
    ///Records fret position.
    #[serde(rename = "@tab.pos", skip_serializing_if = "Option::is_none")]
    pub tab_pos: Option<u64>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttEndingsEndingRend {
    ///Ending rendered only above top staff.
    #[serde(rename = "top")]
    Top,
    ///Ending rendered above staves that have bar lines drawn across them.
    #[serde(rename = "barred")]
    Barred,
    ///Endings rendered above staff groups.
    #[serde(rename = "grouped")]
    Grouped,
}
///Attributes that record ending style information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttEndings {
    ///Describes where ending marks should be displayed.
    #[serde(rename = "@ending.rend", skip_serializing_if = "Option::is_none")]
    pub ending_rend: Option<AttEndingsEndingRend>,
}

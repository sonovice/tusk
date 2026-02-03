//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttPbVisFolium {
    ///The back of a manuscript page.
    #[serde(rename = "verso")]
    Verso,
    ///The front of a manuscript page.
    #[serde(rename = "recto")]
    Recto,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPbVis {
    ///States the side of a leaf (as in a manuscript) on which the content following thepbelement occurs.
    #[serde(rename = "@folium", skip_serializing_if = "Option::is_none")]
    pub folium: Option<AttPbVisFolium>,
}

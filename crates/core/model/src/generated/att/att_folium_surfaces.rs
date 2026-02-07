//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that link a folium element with asurfaceelement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFoliumSurfaces {
    /**A reference to asurfaceelement positioned on the recto side of
          the sheet.*/
    #[serde(rename = "@recto", skip_serializing_if = "Option::is_none")]
    pub recto: Option<crate::generated::data::DataUri>,
    /**A reference to asurfaceelement positioned on the verso side of
          the sheet.*/
    #[serde(rename = "@verso", skip_serializing_if = "Option::is_none")]
    pub verso: Option<crate::generated::data::DataUri>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that link a bifolium element with asurfaceelement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBifoliumSurfaces {
    /**A reference to asurfaceelement positioned on the outer recto
    side of a (folded) sheet.*/
    #[serde(rename = "@outer.recto", skip_serializing_if = "Option::is_none")]
    pub outer_recto: Option<crate::generated::data::DataUri>,
    /**A reference to asurfaceelement positioned on the inner verso
    side of a (folded) sheet.*/
    #[serde(rename = "@inner.verso", skip_serializing_if = "Option::is_none")]
    pub inner_verso: Option<crate::generated::data::DataUri>,
    /**A reference to asurfaceelement positioned on the inner recto
    side of a (folded) sheet.*/
    #[serde(rename = "@inner.recto", skip_serializing_if = "Option::is_none")]
    pub inner_recto: Option<crate::generated::data::DataUri>,
    /**A reference to asurfaceelement positioned on the outer verso
    side of a (folded) sheet.*/
    #[serde(rename = "@outer.verso", skip_serializing_if = "Option::is_none")]
    pub outer_verso: Option<crate::generated::data::DataUri>,
}

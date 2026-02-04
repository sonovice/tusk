//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes listing the active participants in a user-defined collection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPlist {
    /**When the target attribute is present, plist identifies the active participants; that
    is, those entities pointed "from", in a relationship with the specified target(s). When
    the target attribute is not present, it identifies participants in a mutual
    relationship.*/
    #[serde(rename = "@plist", default, skip_serializing_if = "Vec::is_empty")]
    pub plist: Vec<crate::generated::data::DataUri>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Provides attributes for elements which may be associated with particular contextual
elements within the header.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMetadataPointing {
    /**Identifies one or more metadata elements (other than classification terms) within the
    header, which are understood to apply to the element bearing this attribute and its
    content.*/
    #[serde(rename = "@decls", default, skip_serializing_if = "Vec::is_empty")]
    pub decls: Vec<crate::generated::data::DataUri>,
}

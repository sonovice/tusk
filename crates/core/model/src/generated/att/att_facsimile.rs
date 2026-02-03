//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that associate a feature corresponding with all or part of an image.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFacsimile {
    ///Points to one or more images, portions of an image, or surfaces which correspond to the current element.
    #[serde(rename = "@facs", default, skip_serializing_if = "Vec::is_empty")]
    pub facs: Vec<crate::generated::data::DataUri>,
}

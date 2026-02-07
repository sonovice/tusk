//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes common to elements that may refer to a source.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSource {
    /**Contains a list of one or more pointers indicating the sources which attest to a given
          reading. Each value should correspond to the ID of asourceormanifestationelement located in the document header.*/
    #[serde(rename = "@source", default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<crate::generated::data::DataUri>,
}

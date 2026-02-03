//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that identify the layer associated with a distant feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOriginLayerIdent {
    ///identifies the layer on which referenced notation occurs.
    #[serde(rename = "@origin.layer", skip_serializing_if = "Option::is_none")]
    pub origin_layer: Option<crate::generated::SpaceSeparated<u64>>,
}

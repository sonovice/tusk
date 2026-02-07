//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttLayerLogMetcon {
    ///Complete;i.e., conformant with the prevailing meter.
    #[serde(rename = "c")]
    C,
    ///Incomplete;i.e., not enough beats.
    #[serde(rename = "i")]
    I,
    ///Overfull;i.e., too many beats.
    #[serde(rename = "o")]
    O,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLayerLog {
    ///
    #[serde(rename = "@cue", skip_serializing_if = "Option::is_none")]
    pub cue: Option<crate::generated::data::DataBoolean>,
    /**Indicates the relationship between the content of a staff or layer and the prevailing
          meter.*/
    #[serde(rename = "@metcon", skip_serializing_if = "Option::is_none")]
    pub metcon: Option<AttLayerLogMetcon>,
    ///Provides a mechanism for linking the layer to a layerDef element.
    #[serde(rename = "@def", skip_serializing_if = "Option::is_none")]
    pub def: Option<crate::generated::data::DataUri>,
}

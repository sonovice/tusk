//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the visual
      domain related to key signatures.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttKeySigDefaultVis {
    ///Determines where cautionary accidentals should be displayed at a key change.
    #[serde(rename = "@keysig.cancelaccid", skip_serializing_if = "Option::is_none")]
    pub keysig_cancelaccid: Option<crate::generated::data::DataCancelaccid>,
    ///Determines whether the key signature is to be displayed.
    #[serde(rename = "@keysig.visible", skip_serializing_if = "Option::is_none")]
    pub keysig_visible: Option<crate::generated::data::DataBoolean>,
}

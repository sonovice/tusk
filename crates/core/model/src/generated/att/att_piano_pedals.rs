//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by scoreDef and staffDef to provide default description of piano pedal
      rendition.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPianoPedals {
    ///Determines whether piano pedal marks should be rendered as lines or as terms.
    #[serde(rename = "@pedal.style", skip_serializing_if = "Option::is_none")]
    pub pedal_style: Option<crate::generated::data::DataPedalstyle>,
}

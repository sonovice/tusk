//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the visual
      domain related to clefs.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCleffingVis {
    ///Describes the color of the clef.
    #[serde(rename = "@clef.color", skip_serializing_if = "Option::is_none")]
    pub clef_color: Option<crate::generated::data::DataColor>,
    ///Determines whether the clef is to be displayed.
    #[serde(rename = "@clef.visible", skip_serializing_if = "Option::is_none")]
    pub clef_visible: Option<crate::generated::data::DataBoolean>,
}

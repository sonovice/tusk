//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttSpaceVisCutout {
    ///The staff lines should not be drawn.
    #[serde(rename = "cutout")]
    Cutout,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSpaceVis {
    ///"Cut-out" style.
    #[serde(rename = "@cutout", skip_serializing_if = "Option::is_none")]
    pub cutout: Option<AttSpaceVisCutout>,
    /**Indicates whether a space is 'compressible',i.e., if it may be removed at the
    discretion of processing software.*/
    #[serde(rename = "@compressable", skip_serializing_if = "Option::is_none")]
    pub compressable: Option<crate::generated::data::DataBoolean>,
}

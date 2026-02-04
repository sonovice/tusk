//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Horizontal offset attributes specified in terms of time.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffsetTo {
    /**Records a timestamp adjustment of a feature’s programmatically-determined location in
    terms of musical time; that is, beats.*/
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::generated::data::DataTstampoffset>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that indicate whether an event participates in a glissando.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttGlissPresent {
    /**Indicates that this element participates in a glissando. If visual information about
    the glissando needs to be recorded, then aglisselement should be
    employed instead.*/
    #[serde(rename = "@gliss", skip_serializing_if = "Option::is_none")]
    pub gliss: Option<crate::generated::data::DataGlissando>,
}

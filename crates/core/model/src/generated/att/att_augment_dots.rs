//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record the number of dots of augmentation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAugmentDots {
    ///Records the number of augmentation dots required by a written dotted duration.
    #[serde(rename = "@dots", skip_serializing_if = "Option::is_none")]
    pub dots: Option<crate::generated::data::DataAugmentdot>,
}

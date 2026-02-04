//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes used to supply a number-like designation for an element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNNumberLike {
    /**Provides a number-like designation that indicates an element’s position in a sequence
          of similar elements. May not contain space characters.*/
    #[serde(rename = "@n", skip_serializing_if = "Option::is_none")]
    pub n: Option<crate::generated::data::DataWord>,
}

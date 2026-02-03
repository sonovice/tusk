//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes used to supply an integer number designation for an element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNInteger {
    /**Provides a numeric designation that indicates an element’s position in a sequence of
    similar elements. Its value must be a non-negative integer.*/
    #[serde(rename = "@n", skip_serializing_if = "Option::is_none")]
    pub n: Option<u64>,
}

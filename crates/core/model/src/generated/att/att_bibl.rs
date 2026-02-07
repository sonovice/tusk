//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Bibliographic attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBibl {
    /**Contains a reference to a field or element in another descriptive encoding system to
          which this MEI element is comparable.*/
    #[serde(rename = "@analog", skip_serializing_if = "Option::is_none")]
    pub analog: Option<String>,
}

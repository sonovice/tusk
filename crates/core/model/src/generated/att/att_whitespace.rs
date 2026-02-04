//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttWhitespaceXmlSpace {
    /**Allows the application to handle white space as necessary. Not including an
    xml:space attribute produces the same result as using the default value.*/
    #[serde(rename = "default")]
    Default,
    /**Instructs the application to maintain white space "as-is", suggesting that it
    might have meaning.*/
    #[serde(rename = "preserve")]
    Preserve,
}
///Attributes that address whitespace processing.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttWhitespace {
    /**Allows one to signal to an application whether an element’s white space is
    "significant". The behavior of xml:space cascades to all descendant elements, but it can
    be turned off locally by setting the xml:space attribute to the valuedefault.*/
    #[serde(rename = "xml:space", skip_serializing_if = "Option::is_none")]
    pub xml_space: Option<AttWhitespaceXmlSpace>,
}

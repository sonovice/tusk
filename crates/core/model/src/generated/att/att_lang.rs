//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Language attributes common to text elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLang {
    /**Identifies the language of the element’s content. The values for this attribute are
    language 'tags' as defined in BCP 47. All language tags that make use of private use
    sub-tags must be documented in a corresponding language element in the MEI header whose id
    attribute is the same as the language tag’s value.*/
    #[serde(rename = "xml:lang", skip_serializing_if = "Option::is_none")]
    pub xml_lang: Option<String>,
    ///Specifies the transliteration technique used.
    #[serde(rename = "@translit", skip_serializing_if = "Option::is_none")]
    pub translit: Option<String>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes which record the type of an electronic resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttInternetMedia {
    /**Specifies the applicable MIME (multimedia internet mail extension) type. The value
    should be a valid MIME media type defined by the Internet Engineering Task Force in RFC
    2046.*/
    #[serde(rename = "@mimetype", skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

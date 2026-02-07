//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that establish the boundaries of a media object.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMediaBounds {
    /**Specifies a point where the relevant content begins. A numerical value must be less
          and a time value must be earlier than that given by the end attribute.*/
    #[serde(rename = "@begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /**Specifies a point where the relevant content ends. If not specified, the end of the
          content is assumed to be the end point. A numerical value must be greater and a time value
          must be later than that given by the begin attribute.*/
    #[serde(rename = "@end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /**Type of values used in the begin/end attributes. The begin and end attributes can only
          be interpreted meaningfully in conjunction with this attribute.*/
    #[serde(rename = "@betype", skip_serializing_if = "Option::is_none")]
    pub betype: Option<crate::generated::data::DataBetype>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that can be used to associate a representation such as a name or title with
canonical information about the object being named or referenced.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCanonical {
    /**A value that represents or identifies other data. Often, it is a primary key in the
    database or a unique value in the coded list identified by theauthorauth.uriattributes.*/
    #[serde(rename = "@codedval", default, skip_serializing_if = "Vec::is_empty")]
    pub codedval: Vec<String>,
}

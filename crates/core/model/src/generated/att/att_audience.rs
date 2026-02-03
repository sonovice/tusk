//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttAudienceAudience {
    ///Internal use only.
    #[serde(rename = "private")]
    Private,
    ///Available to all audiences.
    #[serde(rename = "public")]
    Public,
}
///Attributes that describe the intended audience.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAudience {
    ///The intended audience.
    #[serde(rename = "@audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<AttAudienceAudience>,
}

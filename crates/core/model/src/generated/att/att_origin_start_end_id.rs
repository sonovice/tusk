//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes recording the identifiers of the first and last elements of a sequence of
      distant elements.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOriginStartEndId {
    ///indicates the first element in a sequence of events.
    #[serde(rename = "@origin.startid", skip_serializing_if = "Option::is_none")]
    pub origin_startid: Option<crate::generated::data::DataUri>,
    ///indicates the final element in a sequence of events.
    #[serde(rename = "@origin.endid", skip_serializing_if = "Option::is_none")]
    pub origin_endid: Option<crate::generated::data::DataUri>,
}

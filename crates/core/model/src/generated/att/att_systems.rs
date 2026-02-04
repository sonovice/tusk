//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that capture system layout information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSystems {
    /**Indicates whether the system starts with a continuous line connecting all staves,
    including single-staff systems. Do not confuse this with the heavy vertical line used as a grouping
    symbol.*/
    #[serde(rename = "@system.leftline", skip_serializing_if = "Option::is_none")]
    pub system_leftline: Option<crate::generated::data::DataBoolean>,
    /**Describes the amount of whitespace at the left system margin relative to
    page.leftmar.*/
    #[serde(rename = "@system.leftmar", skip_serializing_if = "Option::is_none")]
    pub system_leftmar: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Describes the amount of whitespace at the right system margin relative to
    page.rightmar.*/
    #[serde(rename = "@system.rightmar", skip_serializing_if = "Option::is_none")]
    pub system_rightmar: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Describes the distance from page’s top edge to the first system; used for first page
    only.*/
    #[serde(rename = "@system.topmar", skip_serializing_if = "Option::is_none")]
    pub system_topmar: Option<crate::generated::data::DataMeasurementunsigned>,
}

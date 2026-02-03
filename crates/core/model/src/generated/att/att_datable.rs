//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes common to dates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDatable {
    ///Contains the end point of a date range in standard ISO form.
    #[serde(rename = "@enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<crate::generated::data::DataIsodate>,
    ///Provides the value of a textual date in standard ISO form.
    #[serde(rename = "@isodate", skip_serializing_if = "Option::is_none")]
    pub isodate: Option<crate::generated::data::DataIsodate>,
    ///Contains an upper boundary for an uncertain date in standard ISO form.
    #[serde(rename = "@notafter", skip_serializing_if = "Option::is_none")]
    pub notafter: Option<crate::generated::data::DataIsodate>,
    ///Contains a lower boundary, in standard ISO form, for an uncertain date.
    #[serde(rename = "@notbefore", skip_serializing_if = "Option::is_none")]
    pub notbefore: Option<crate::generated::data::DataIsodate>,
    ///Contains the starting point of a date range in standard ISO form.
    #[serde(rename = "@startdate", skip_serializing_if = "Option::is_none")]
    pub startdate: Option<crate::generated::data::DataIsodate>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Shared attributes in the mensural repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMensuralShared {
    ///Describes the maxima-long relationship.
    #[serde(rename = "@modusmaior", skip_serializing_if = "Option::is_none")]
    pub modusmaior: Option<crate::generated::data::DataModusmaior>,
    ///Describes the long-breve relationship.
    #[serde(rename = "@modusminor", skip_serializing_if = "Option::is_none")]
    pub modusminor: Option<crate::generated::data::DataModusminor>,
    ///Describes the semibreve-minim relationship.
    #[serde(rename = "@prolatio", skip_serializing_if = "Option::is_none")]
    pub prolatio: Option<crate::generated::data::DataProlatio>,
    ///Describes the breve-semibreve relationship.
    #[serde(rename = "@tempus", skip_serializing_if = "Option::is_none")]
    pub tempus: Option<crate::generated::data::DataTempus>,
    ///Describes the divisions of the breve in use in 14th-century Italy.
    #[serde(rename = "@divisio", skip_serializing_if = "Option::is_none")]
    pub divisio: Option<crate::generated::data::DataDivisio>,
}

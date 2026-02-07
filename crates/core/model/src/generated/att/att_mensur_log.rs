//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMensurLog {
    /**Along with numbase, describes duration as a ratio. num is the first value in the
          ratio, while numbase is the second.*/
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<u64>,
    /**Along with num, describes duration as a ratio. num is the first value in the ratio,
          while numbase is the second.*/
    #[serde(rename = "@numbase", skip_serializing_if = "Option::is_none")]
    pub numbase: Option<u64>,
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
    ///Level of duration at which the proportion given by the @num and @numbase ratio applies.
    #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
    pub level: Option<crate::generated::data::DataDurationMensural>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Logical domain attributes for a score in the mensural repertoire. The values set in these
attributes act as score-wide defaults for attributes that are not set in descendant
elements.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefLogMensural {
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
    ///Together, proport.num and proport.numbase specify a proportional change as a ratio,e.g., 1:3. Proport.num is for the first value in the ratio.
    #[serde(rename = "@proport.num", skip_serializing_if = "Option::is_none")]
    pub proport_num: Option<u64>,
    ///Together, proport.num and proport.numbase specify a proportional change as a ratio,e.g., 1:3. Proport.numbase is for the second value in the ratio.
    #[serde(rename = "@proport.numbase", skip_serializing_if = "Option::is_none")]
    pub proport_numbase: Option<u64>,
}

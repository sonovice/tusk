//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the logical
      domain related to mensuration. The tempus, prolatio, modusmaior, and modusminor attributes
      (from the att.mensural.shared class) specify the relationship between the four principle
      levels of note value,i.e., the long, breve, semibreve and minim, in mensural notation.
      Modusminor describes the long-breve relationship, while tempus describes the breve-semibreve,
      and prolatio the semibreve-minim relationship, respectively. Modusmaior is for the maxima-long
      relationship. The proport.* attributes describe augmentation or diminution of the normal value
      of the notes in mensural notation.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMensuralLog {
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

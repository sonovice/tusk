//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes common to all elements representing variant readings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCrit {
    ///Signifies the hand responsible for an action. The value must be the ID of ahandelement declared in the header.
    #[serde(rename = "@hand", skip_serializing_if = "Option::is_none")]
    pub hand: Option<crate::generated::data::DataUri>,
    /**Used to assign a sequence number related to the order in which the encoded features
          carrying this attribute are believed to have occurred.*/
    #[serde(rename = "@seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    /**Contains a list of one or more pointers indicating the sources which attest to a given
          reading. Each value should correspond to the ID of asourceormanifestationelement located in the document header.*/
    #[serde(rename = "@source", default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<crate::generated::data::DataUri>,
    /**Classifies the cause for the variant reading, according to any appropriate typology of
          possible origins.*/
    #[serde(rename = "@cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the analytical
domain that are related to key signatures.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttKeySigDefaultAnl {
    ///Contains an accidental for the tonic key, if one is required,e.g., ifkey.pnameequalscandkey.accidequalss, then a tonic of C# is indicated.
    #[serde(rename = "@key.accid", skip_serializing_if = "Option::is_none")]
    pub key_accid: Option<crate::generated::data::DataAccidentalGestural>,
    ///Indicates major, minor, or other tonality.
    #[serde(rename = "@key.mode", skip_serializing_if = "Option::is_none")]
    pub key_mode: Option<crate::generated::data::DataMode>,
    ///Holds the pitch name of the tonic key,e.g.,cfor the key of C.
    #[serde(rename = "@key.pname", skip_serializing_if = "Option::is_none")]
    pub key_pname: Option<crate::generated::data::DataPitchname>,
}

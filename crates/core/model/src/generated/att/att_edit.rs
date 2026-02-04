//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttEditEvidence {
    ///There is evidence within the document to support the intervention.
    #[serde(rename = "internal")]
    Internal,
    ///There is evidence outside the document to support the intervention.
    #[serde(rename = "external")]
    External,
    /**The assertion has been made by the editor, cataloguer, or scholar on the basis of
              their expertise.*/
    #[serde(rename = "conjecture")]
    Conjecture,
}
/**Attributes describing the nature of an encoded scholarly intervention or
      interpretation.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttEdit {
    /**Contains a list of one or more pointers indicating the sources which attest to a given
          reading. Each value should correspond to the ID of asourceormanifestationelement located in the document header.*/
    #[serde(rename = "@source", default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<crate::generated::data::DataUri>,
    ///Signifies the degree of certainty or precision associated with a feature.
    #[serde(rename = "@cert", skip_serializing_if = "Option::is_none")]
    pub cert: Option<crate::generated::data::DataCertainty>,
    /**Indicates the nature of the evidence supporting the reliability or accuracy of the
          intervention or interpretation.*/
    #[serde(rename = "@evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<AttEditEvidence>,
}

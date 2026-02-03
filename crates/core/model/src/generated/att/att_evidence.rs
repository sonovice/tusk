//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttEvidenceEvidence {
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
///Attributes describing the support for and the certainty of an assertion.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttEvidence {
    ///Signifies the degree of certainty or precision associated with a feature.
    #[serde(rename = "@cert", skip_serializing_if = "Option::is_none")]
    pub cert: Option<crate::generated::data::DataCertainty>,
    /**Indicates the nature of the evidence supporting the reliability or accuracy of the
    intervention or interpretation.*/
    #[serde(rename = "@evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<AttEvidenceEvidence>,
}

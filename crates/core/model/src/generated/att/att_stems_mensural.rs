//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe the properties of stemmed features specific to mensural repertoires.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStemsMensural {
    ///Records the form of the stem.
    #[serde(rename = "@stem.form", skip_serializing_if = "Option::is_none")]
    pub stem_form: Option<crate::generated::data::DataStemformMensural>,
}

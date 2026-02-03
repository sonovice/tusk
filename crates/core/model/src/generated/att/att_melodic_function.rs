//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes describing melodic function.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMelodicFunction {
    ///Describes melodic function usingHumdrum **embel syntax.
    #[serde(rename = "@mfunc", skip_serializing_if = "Option::is_none")]
    pub mfunc: Option<crate::generated::data::DataMelodicfunction>,
}

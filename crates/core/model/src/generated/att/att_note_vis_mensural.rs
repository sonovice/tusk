//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes in the Mensural repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteVisMensural {
    ///Indicates this element’s participation in a ligature.
    #[serde(rename = "@lig", skip_serializing_if = "Option::is_none")]
    pub lig: Option<crate::generated::data::DataLigatureform>,
}

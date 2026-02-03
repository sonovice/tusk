//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes in the Mensural repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteLogMensural {}

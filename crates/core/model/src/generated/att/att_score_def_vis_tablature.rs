//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes for scoreDef in the tablature repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefVisTablature {}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes that describe the properties of a stem in the mensural repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStemGes {}

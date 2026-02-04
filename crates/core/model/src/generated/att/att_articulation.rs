//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for capturing the written signs that describe the method of performance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttArticulation {
    /**Encodes the written articulation(s). Articulations are normally encoded in order from
          the note head outward; that is, away from the stem. See additional notes at att.vis.note.
          Only articulations should be encoded in the artic attribute; for example, fingerings
          should be encoded using thefingelement.*/
    #[serde(rename = "@artic", default, skip_serializing_if = "Vec::is_empty")]
    pub artic: Vec<crate::generated::data::DataArticulation>,
}

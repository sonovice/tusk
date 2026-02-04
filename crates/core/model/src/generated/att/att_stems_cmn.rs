//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that describe the properties of stemmed features; that is, chords and
      notes.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStemsCmn {
    /**Contains an indication of which staff a note or chord that logically belongs to the
          current staff should be visually placed on; that is, the one above or the one
          below.*/
    #[serde(rename = "@stem.with", skip_serializing_if = "Option::is_none")]
    pub stem_with: Option<crate::generated::data::DataNeighboringlayer>,
}

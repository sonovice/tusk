//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Visual domain attributes for chord. The slur, slur.dir, slur.rend, tie, tie.dir, and
      tie.rend attributes here are "syntactic sugar" for these attributes on each of the chord's
      individual notes. The values here apply to all the notes in the chord. If some notes are
      slurred or tied while others aren't, then the individual note attributes must be used.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttChordVisCmn {
    /**Presence of this attribute indicates that the secondary beam should be broken
          following this note/chord. The value of the attribute records the number of beams which
          should remain unbroken.*/
    #[serde(rename = "@breaksec", skip_serializing_if = "Option::is_none")]
    pub breaksec: Option<u64>,
}

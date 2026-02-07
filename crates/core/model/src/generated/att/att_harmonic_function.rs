//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes describing the harmonic function of a single pitch.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHarmonicFunction {
    /**Captures scale degree information usingHumdrum **deg syntax-- an optional indicator
          of melodic approach (^ = ascending approach, v = descending approach), a scale degree
          value (1 = tonic ... 7 = leading tone), and an optional indication of chromatic
          alteration,1,v7,^1, orv5+, for example.
          The amount of chromatic alternation is not indicated.*/
    #[serde(rename = "@deg", skip_serializing_if = "Option::is_none")]
    pub deg: Option<crate::generated::data::DataScaledegree>,
}

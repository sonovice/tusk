//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttBeamSpanVisForm {
    /**means that the secondary beams become progressively more distant
              toward the end of the beam.*/
    #[serde(rename = "acc")]
    Acc,
    ///for beams that are "feathered" in both directions.
    #[serde(rename = "mixed")]
    Mixed,
    /**indicates that the secondary beams get progressively closer together
            toward the end of the beam.*/
    #[serde(rename = "rit")]
    Rit,
    /**indicates that the secondary beams are equidistant along the course of
              the beam.*/
    #[serde(rename = "norm")]
    Norm,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBeamSpanVis {
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
          as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    ///
    #[serde(rename = "@cue", skip_serializing_if = "Option::is_none")]
    pub cue: Option<crate::generated::data::DataBoolean>,
    ///Captures whether a beam is "feathered" and in which direction.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<AttBeamSpanVisForm>,
    ///Records the placement of the beam relative to the events it affects.
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataBeamplace>,
    ///Indicates presence of slash through the beam.
    #[serde(rename = "@slash", skip_serializing_if = "Option::is_none")]
    pub slash: Option<crate::generated::data::DataBoolean>,
    ///Records the slope of the beam.
    #[serde(rename = "@slope", skip_serializing_if = "Option::is_none")]
    pub slope: Option<f64>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
          or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
}

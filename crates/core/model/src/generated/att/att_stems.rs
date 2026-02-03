//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that describe the properties of stemmed features; that is, chords and
notes.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStems {
    /**Contains an indication of which staff a note or chord that logically belongs to the
    current staff should be visually placed on; that is, the one above or the one
    below.*/
    #[serde(rename = "@stem.with", skip_serializing_if = "Option::is_none")]
    pub stem_with: Option<crate::generated::data::DataNeighboringlayer>,
    ///Records the form of the stem.
    #[serde(rename = "@stem.form", skip_serializing_if = "Option::is_none")]
    pub stem_form: Option<crate::generated::data::DataStemformMensural>,
    ///Describes the direction of a stem.
    #[serde(rename = "@stem.dir", skip_serializing_if = "Option::is_none")]
    pub stem_dir: Option<crate::generated::data::DataStemdirection>,
    ///Encodes the stem length.
    #[serde(rename = "@stem.len", skip_serializing_if = "Option::is_none")]
    pub stem_len: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Encodes any stem "modifiers"; that is, symbols rendered on the stem, such as tremolo
    or Sprechstimme indicators.*/
    #[serde(rename = "@stem.mod", skip_serializing_if = "Option::is_none")]
    pub stem_mod: Option<crate::generated::data::DataStemmodifier>,
    ///Records the position of the stem in relation to the note head(s).
    #[serde(rename = "@stem.pos", skip_serializing_if = "Option::is_none")]
    pub stem_pos: Option<crate::generated::data::DataStemposition>,
    /**Points to a note element in a different layer whose stem is shared.
    The linked notes should be rendered like a chord though they are part of different layers.*/
    #[serde(rename = "@stem.sameas", skip_serializing_if = "Option::is_none")]
    pub stem_sameas: Option<crate::generated::data::DataUri>,
    ///Determines whether a stem should be displayed.
    #[serde(rename = "@stem.visible", skip_serializing_if = "Option::is_none")]
    pub stem_visible: Option<crate::generated::data::DataBoolean>,
    ///Records the output x coordinate of the stem’s attachment point.
    #[serde(rename = "@stem.x", skip_serializing_if = "Option::is_none")]
    pub stem_x: Option<f64>,
    ///Records the output y coordinate of the stem’s attachment point.
    #[serde(rename = "@stem.y", skip_serializing_if = "Option::is_none")]
    pub stem_y: Option<f64>,
}

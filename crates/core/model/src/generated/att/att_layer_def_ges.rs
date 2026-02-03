//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLayerDefGes {
    /**Provides a way of pointing to a MIDI instrument definition. It must contain the ID of
    aninstrDefelement elsewhere in the document.*/
    #[serde(rename = "@instr", skip_serializing_if = "Option::is_none")]
    pub instr: Option<crate::generated::data::DataUri>,
    ///Holds a value for cycles per second,i.e., Hertz, for a tuning reference pitch.
    #[serde(rename = "@tune.Hz", skip_serializing_if = "Option::is_none")]
    pub tune_hz: Option<f64>,
    ///Holds the pitch name of a tuning reference pitch,i.e., the central tone of a tuning system.
    #[serde(rename = "@tune.pname", skip_serializing_if = "Option::is_none")]
    pub tune_pname: Option<crate::generated::data::DataPitchname>,
    ///Provides an indication of the tuning system,just, for example.
    #[serde(rename = "@tune.temper", skip_serializing_if = "Option::is_none")]
    pub tune_temper: Option<crate::generated::data::DataTemperament>,
}

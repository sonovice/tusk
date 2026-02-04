//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes for staffDef in the CMN repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffDefGes {
    /**Provides a way of pointing to a MIDI instrument definition. It must contain the ID of
          aninstrDefelement elsewhere in the document.*/
    #[serde(rename = "@instr", skip_serializing_if = "Option::is_none")]
    pub instr: Option<crate::generated::data::DataUri>,
    /**This attribute is deprecated in favor of the newtuningelement and will be removed in a future version. Provides a *written* pitch and octave for each open string or course of
          strings.*/
    #[serde(rename = "@tab.strings", skip_serializing_if = "Option::is_none")]
    pub tab_strings: Option<crate::generated::SpaceSeparated<String>>,
    ///This attribute is deprecated in favor of the newtuningelement and will be removed in a future version. Provides a *written* pitch and octave for each open string or course of strings.
    #[serde(rename = "@tab.courses", skip_serializing_if = "Option::is_none")]
    pub tab_courses: Option<crate::generated::SpaceSeparated<String>>,
    /**Indicates the number of pulses (sometimes referred to as ticks or divisions) per
          quarter note. Unlike MIDI, MEI permits different values for a score and individual
          staves.*/
    #[serde(rename = "@ppq", skip_serializing_if = "Option::is_none")]
    pub ppq: Option<u64>,
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

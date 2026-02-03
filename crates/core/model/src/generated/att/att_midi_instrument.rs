//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record MIDI instrument information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMidiInstrument {
    ///Captures the General MIDI instrument number. Use an integer for a 0-based value.
    #[serde(rename = "@midi.instrnum", skip_serializing_if = "Option::is_none")]
    pub midi_instrnum: Option<crate::generated::data::DataMidivalue>,
    ///Provides a General MIDI label for the MIDI instrument.
    #[serde(rename = "@midi.instrname", skip_serializing_if = "Option::is_none")]
    pub midi_instrname: Option<crate::generated::data::DataMidinames>,
    /**Sets the instrument’s position in a stereo field. MIDI values of 0 and 1 both pan
    left, 127 or 128 pans right, and 63 or 64 pans to the center. Positve percentage values
    pan to the right, negative ones to the left. 0% is centered.*/
    #[serde(rename = "@midi.pan", skip_serializing_if = "Option::is_none")]
    pub midi_pan: Option<crate::generated::data::DataMidivaluePan>,
    ///Records a non-General MIDI patch/instrument name.
    #[serde(rename = "@midi.patchname", skip_serializing_if = "Option::is_none")]
    pub midi_patchname: Option<String>,
    ///Records a non-General MIDI patch/instrument number.
    #[serde(rename = "@midi.patchnum", skip_serializing_if = "Option::is_none")]
    pub midi_patchnum: Option<crate::generated::data::DataMidivalue>,
    ///Sets the instrument’s volume.
    #[serde(rename = "@midi.volume", skip_serializing_if = "Option::is_none")]
    pub midi_volume: Option<crate::generated::data::DataMidivaluePercent>,
}

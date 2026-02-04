//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Gestural domain attributes for scoreDef. The values set in these attributes act as
score-wide defaults for attributes that are not set in descendant elements. For example, the
grace attribute value here applies to all the grace attribute values in the score (or, more
accurately, until the nextscoreDefelement) without having to
individually set each note’s grace attribute value. The midi.* attributes function as default
values when creating sounding output. The tune.* attributes provide the capability of
recording a tuning reference pitch.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefGes {
    ///Records a MIDI channel value.
    #[serde(rename = "@midi.channel", skip_serializing_if = "Option::is_none")]
    pub midi_channel: Option<crate::generated::data::DataMidichannel>,
    ///Specifies the 'on' part of the duty cycle as a percentage of a note’s duration.
    #[serde(rename = "@midi.duty", skip_serializing_if = "Option::is_none")]
    pub midi_duty: Option<crate::generated::data::DataPercentLimited>,
    ///Sets the MIDI port value.
    #[serde(rename = "@midi.port", skip_serializing_if = "Option::is_none")]
    pub midi_port: Option<crate::generated::data::DataMidivalueName>,
    ///Sets the MIDI track.
    #[serde(rename = "@midi.track", skip_serializing_if = "Option::is_none")]
    pub midi_track: Option<u64>,
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
    /**Captures the number of *quarter notes* per minute. In MIDI, a beat is always defined
    as a quarter note, *not the numerator of the time signature or the metronomic
    indication*.*/
    #[serde(rename = "@midi.bpm", skip_serializing_if = "Option::is_none")]
    pub midi_bpm: Option<crate::generated::data::DataMidibpm>,
    /**Records the number of microseconds per *quarter note*. In MIDI, a beat is always
    defined as a quarter note, *not the numerator of the time signature or the metronomic
    indication*. At 120 quarter notes per minute, each quarter note will last 500,000
    microseconds.*/
    #[serde(rename = "@midi.mspb", skip_serializing_if = "Option::is_none")]
    pub midi_mspb: Option<crate::generated::data::DataMidimspb>,
    /**Used to describe tempo in terms of beats (often the meter signature denominator) per
    minute, ala M.M. (Maelzel’s Metronome). Do not confuse this attribute with midi.bpm or
    midi.mspb. In MIDI, a beat is always defined as a quarter note, *not the numerator of the
    time signature or the metronomic indication*.*/
    #[serde(rename = "@mm", skip_serializing_if = "Option::is_none")]
    pub mm: Option<crate::generated::data::DataTempovalue>,
    ///Captures the metronomic unit.
    #[serde(rename = "@mm.unit", skip_serializing_if = "Option::is_none")]
    pub mm_unit: Option<crate::generated::data::DataDuration>,
    ///Records the number of augmentation dots required by a dotted metronome unit.
    #[serde(rename = "@mm.dots", skip_serializing_if = "Option::is_none")]
    pub mm_dots: Option<crate::generated::data::DataAugmentdot>,
}

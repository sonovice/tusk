//! Barline extras conversion from MusicXML to ExtensionStore.
//!
//! Barlines with extra children (repeat, ending, fermata, segno, coda,
//! wavy-line) store typed data in ExtensionStore for lossless roundtrip.
//! The basic bar-style and repeats are set on MEI measure @left/@right
//! via bar_style_to_mei_barrendition in structure.rs.

use crate::model::elements::Barline;
use tusk_model::musicxml_ext::{BarlineData, EndingData, RepeatData};

/// Build typed BarlineData from a MusicXML Barline for ExtensionStore.
pub fn build_barline_data(b: &Barline) -> BarlineData {
    use crate::model::data::YesNo;

    BarlineData {
        location: b.location.as_ref().map(|l| l.to_musicxml_str().to_string()),
        bar_style: b
            .bar_style
            .as_ref()
            .map(|s| s.to_musicxml_str().to_string()),
        repeat: b.repeat.as_ref().map(|r| RepeatData {
            direction: match r.direction {
                crate::model::elements::BackwardForward::Forward => "forward".to_string(),
                crate::model::elements::BackwardForward::Backward => "backward".to_string(),
            },
            times: r.times,
            after_jump: r.after_jump.map(|v| matches!(v, YesNo::Yes)),
            winged: r
                .winged
                .as_ref()
                .and_then(|w| serde_json::to_value(w).ok())
                .and_then(|v| v.as_str().map(|s| s.to_string())),
        }),
        ending: b.ending.as_ref().map(|e| EndingData {
            number: e.number.clone(),
            ending_type: e.ending_type.to_string(),
            text: e.text.clone(),
            visual: None,
        }),
        fermatas: b.fermatas.clone(),
        segno: b.segno.clone(),
        coda: b.coda.clone(),
        wavy_line: b.wavy_line.clone(),
        segno_attr: b.segno_attr.clone(),
        coda_attr: b.coda_attr.clone(),
        divisions: b.divisions,
    }
}

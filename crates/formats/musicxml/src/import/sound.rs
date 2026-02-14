//! Sound element conversion from MusicXML to MEI.
//!
//! Converts standalone MusicXML `<sound>` elements to MEI `<dir>` control events
//! with ExtensionStore data for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::direction::Sound;
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::SoundData;

/// Convert a standalone MusicXML `<sound>` element to an MEI `<dir>` measure child.
///
/// Data is stored in ExtensionStore for lossless roundtrip.
/// A human-readable summary is stored as the text child.
pub fn convert_sound(sound: &Sound, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("sound"));

    // Store typed SoundData in ExtensionStore per-concept map
    if let Some(ref id) = dir.common.xml_id {
        ctx.ext_store_mut()
            .insert_sound(id.clone(), build_sound_data(sound));
    }

    // Human-readable summary
    let summary = sound_summary(sound);
    if !summary.is_empty() {
        dir.children.push(DirChild::Text(summary));
    }

    // Set tstamp to current beat position (1-based MEI convention)
    let beat_position = ctx.beat_position_in_beats();
    // Apply sound offset if present
    let offset_beats = sound
        .offset
        .as_ref()
        .map(|o| ctx.divisions_to_beats(o.value))
        .unwrap_or(0.0);
    let tstamp = beat_position + offset_beats + 1.0;
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(tstamp));

    // Staff
    let staff = ctx.current_staff();
    dir.dir_log.staff = Some(staff.to_string());

    MeasureChild::Dir(Box::new(dir))
}

/// Generate a human-readable summary of a Sound element.
fn sound_summary(sound: &Sound) -> String {
    let mut parts = Vec::new();

    if let Some(tempo) = sound.tempo {
        parts.push(format!("tempo={tempo}"));
    }
    if let Some(dynamics) = sound.dynamics {
        parts.push(format!("dynamics={dynamics}"));
    }
    if sound.dacapo.is_some() {
        parts.push("dacapo".to_string());
    }
    if let Some(ref segno) = sound.segno {
        parts.push(format!("segno={segno}"));
    }
    if let Some(ref dalsegno) = sound.dalsegno {
        parts.push(format!("dalsegno={dalsegno}"));
    }
    if let Some(ref coda) = sound.coda {
        parts.push(format!("coda={coda}"));
    }
    if let Some(ref tocoda) = sound.tocoda {
        parts.push(format!("tocoda={tocoda}"));
    }
    if let Some(ref fine) = sound.fine {
        parts.push(format!("fine={fine}"));
    }
    if sound.swing.is_some() {
        parts.push("swing".to_string());
    }
    if !sound.midi_instrument_changes.is_empty() {
        parts.push("midi".to_string());
    }

    parts.join("; ")
}

pub(crate) fn build_sound_data(s: &Sound) -> SoundData {
    use crate::model::data::YesNo;
    use crate::model::direction::SwingContent;
    use tusk_model::musicxml_ext::{
        InstrumentChangeData, MidiDeviceData, MidiInstrumentDataInner, OffsetData, PlayData,
        SoundMidiGroupData, SwingData, VirtualInstrumentData,
    };

    SoundData {
        tempo: s.tempo,
        dynamics: s.dynamics,
        dacapo: s.dacapo.map(|v| matches!(v, YesNo::Yes)),
        segno: s.segno.clone(),
        dalsegno: s.dalsegno.clone(),
        coda: s.coda.clone(),
        tocoda: s.tocoda.clone(),
        divisions: s.divisions,
        forward_repeat: s.forward_repeat.map(|v| matches!(v, YesNo::Yes)),
        fine: s.fine.clone(),
        time_only: s.time_only.clone(),
        pizzicato: s.pizzicato.map(|v| matches!(v, YesNo::Yes)),
        pan: s.pan,
        elevation: s.elevation,
        damper_pedal: s.damper_pedal.clone(),
        soft_pedal: s.soft_pedal.clone(),
        sostenuto_pedal: s.sostenuto_pedal.clone(),
        midi_groups: s
            .midi_instrument_changes
            .iter()
            .map(|g| SoundMidiGroupData {
                instrument_change: g.instrument_change.as_ref().map(|ic| InstrumentChangeData {
                    id: ic.id.clone(),
                    instrument_sound: ic.instrument_sound.clone(),
                    solo: ic.solo,
                    ensemble: ic.ensemble.as_ref().map(|e| {
                        if e.is_empty() {
                            None
                        } else {
                            e.parse::<u32>().ok()
                        }
                    }),
                    virtual_instrument: if ic.virtual_library.is_some() || ic.virtual_name.is_some()
                    {
                        Some(VirtualInstrumentData {
                            library: ic.virtual_library.clone(),
                            name: ic.virtual_name.clone(),
                        })
                    } else {
                        None
                    },
                }),
                midi_device: g.midi_device.as_ref().map(|md| MidiDeviceData {
                    value: md.value.clone(),
                    port: md.port,
                    id: md.id.clone(),
                }),
                midi_instrument: g
                    .midi_instrument
                    .as_ref()
                    .map(|mi| MidiInstrumentDataInner {
                        id: mi.id.clone(),
                        channel: mi.midi_channel,
                        name: mi.midi_name.clone(),
                        bank: mi.midi_bank,
                        program: mi.midi_program,
                        unpitched: mi.midi_unpitched,
                        volume: mi.volume,
                        pan: mi.pan,
                        elevation: mi.elevation,
                    }),
                play: g.play.as_ref().map(|p| PlayData {
                    id: p.id.clone(),
                    entries: p
                        .entries
                        .iter()
                        .filter_map(|e| serde_json::to_value(e).ok())
                        .collect(),
                }),
            })
            .collect(),
        swing: s.swing.as_ref().map(|sw| {
            let (content_type, first, second, swing_type) = match &sw.content {
                SwingContent::Straight => ("straight".to_string(), None, None, None),
                SwingContent::Ratio(r) => (
                    "ratio".to_string(),
                    Some(r.first),
                    Some(r.second),
                    r.swing_type.clone(),
                ),
            };
            SwingData {
                content_type,
                first,
                second,
                swing_type,
                style: sw.swing_style.clone(),
            }
        }),
        offset: s.offset.as_ref().map(|o| OffsetData {
            value: o.value,
            sound: o.sound.map(|v| matches!(v, YesNo::Yes)),
        }),
        id: s.id.clone(),
    }
}

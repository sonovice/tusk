//! Sound element export from MEI to MusicXML.
//!
//! Converts MEI `<dir>` elements carrying standalone sound data back to
//! MusicXML `<sound>` measure content elements. Reconstructs the full Sound
//! struct from typed SoundData in ExtensionStore.

use crate::context::ConversionContext;
use crate::model::data::YesNo;
use crate::model::direction::{
    InstrumentChange, Offset, Play, Sound, SoundMidiGroup, Swing, SwingContent, SwingRatio,
};
use crate::model::elements::MeasureContent;
use crate::model::elements::score::{MidiDevice, MidiInstrument};
use tusk_model::elements::Dir;
use tusk_model::musicxml_ext::{SoundData, SoundMidiGroupData, SwingData};

/// Convert an MEI `<dir>` with standalone sound data to a MusicXML `<sound>` element.
///
/// Reads SoundData from ExtensionStore and reconstructs the full Sound struct.
pub fn convert_mei_sound_dir(dir: &Dir, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    let id = dir.common.xml_id.as_ref()?;
    let data = ctx.ext_store().sound(id)?;
    let sound = build_sound_from_data(data);
    Some(MeasureContent::Sound(Box::new(sound)))
}

/// Build a MusicXML `Sound` from typed `SoundData`.
pub(crate) fn build_sound_from_data(data: &SoundData) -> Sound {
    let bool_to_yesno = |b| if b { YesNo::Yes } else { YesNo::No };

    Sound {
        tempo: data.tempo,
        dynamics: data.dynamics,
        dacapo: data.dacapo.map(bool_to_yesno),
        segno: data.segno.clone(),
        dalsegno: data.dalsegno.clone(),
        coda: data.coda.clone(),
        tocoda: data.tocoda.clone(),
        divisions: data.divisions,
        forward_repeat: data.forward_repeat.map(bool_to_yesno),
        fine: data.fine.clone(),
        time_only: data.time_only.clone(),
        pizzicato: data.pizzicato.map(bool_to_yesno),
        pan: data.pan,
        elevation: data.elevation,
        damper_pedal: data.damper_pedal.clone(),
        soft_pedal: data.soft_pedal.clone(),
        sostenuto_pedal: data.sostenuto_pedal.clone(),
        midi_instrument_changes: data.midi_groups.iter().map(build_midi_group).collect(),
        swing: data.swing.as_ref().map(build_swing),
        offset: data.offset.as_ref().map(|o| Offset {
            value: o.value,
            sound: o.sound.map(bool_to_yesno),
        }),
        id: data.id.clone(),
    }
}

fn build_midi_group(g: &SoundMidiGroupData) -> SoundMidiGroup {
    SoundMidiGroup {
        instrument_change: g.instrument_change.as_ref().map(|ic| InstrumentChange {
            id: ic.id.clone(),
            instrument_sound: ic.instrument_sound.clone(),
            solo: ic.solo,
            ensemble: ic.ensemble.as_ref().map(|e| match e {
                Some(n) => n.to_string(),
                None => String::new(),
            }),
            virtual_library: ic
                .virtual_instrument
                .as_ref()
                .and_then(|vi| vi.library.clone()),
            virtual_name: ic
                .virtual_instrument
                .as_ref()
                .and_then(|vi| vi.name.clone()),
        }),
        midi_device: g.midi_device.as_ref().map(|md| MidiDevice {
            value: md.value.clone(),
            port: md.port,
            id: md.id.clone(),
        }),
        midi_instrument: g.midi_instrument.as_ref().map(|mi| MidiInstrument {
            id: mi.id.clone(),
            midi_channel: mi.channel,
            midi_name: mi.name.clone(),
            midi_bank: mi.bank,
            midi_program: mi.program,
            midi_unpitched: mi.unpitched,
            volume: mi.volume,
            pan: mi.pan,
            elevation: mi.elevation,
        }),
        play: g.play.as_ref().map(|p| Play {
            id: p.id.clone(),
            entries: p.entries.clone(),
        }),
    }
}

fn build_swing(sw: &SwingData) -> Swing {
    let content = if sw.content_type == "straight" {
        SwingContent::Straight
    } else {
        SwingContent::Ratio(SwingRatio {
            first: sw.first.unwrap_or(2),
            second: sw.second.unwrap_or(1),
            swing_type: sw.swing_type.clone(),
        })
    };
    Swing {
        content,
        swing_style: sw.style.clone(),
    }
}

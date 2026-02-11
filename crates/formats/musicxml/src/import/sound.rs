//! Sound element conversion from MusicXML to MEI.
//!
//! Converts standalone MusicXML `<sound>` elements to MEI `<dir>` control events
//! with ExtensionStore data for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::direction::Sound;
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::SoundData;

/// Label marker for MEI dir elements carrying standalone sound data (via ExtensionStore).
pub const SOUND_LABEL_PREFIX: &str = "musicxml:sound";

/// Convert a standalone MusicXML `<sound>` element to an MEI `<dir>` measure child.
///
/// Data is stored in ExtensionStore for lossless roundtrip.
/// A human-readable summary is stored as the text child.
pub fn convert_sound(sound: &Sound, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("sound"));
    dir.common.label = Some(SOUND_LABEL_PREFIX.to_string());

    // Store typed SoundData + raw MusicXML JSON in ExtensionStore
    if let Some(ref id) = dir.common.xml_id {
        let entry = ctx.ext_store_mut().entry(id.clone());
        entry.sound = Some(build_sound_data(sound));
        entry.mxml_json = serde_json::to_value(sound).ok();
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

fn build_sound_data(s: &Sound) -> SoundData {
    use crate::model::data::YesNo;
    use tusk_model::musicxml_ext::OffsetData;

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
            .filter_map(|g| serde_json::to_value(g).ok())
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect(),
        swing: s.swing.as_ref().and_then(|sw| {
            serde_json::to_value(sw)
                .ok()
                .and_then(|v| serde_json::from_value(v).ok())
        }),
        offset: s.offset.as_ref().map(|o| OffsetData {
            value: o.value,
            sound: o.sound.map(|v| matches!(v, YesNo::Yes)),
        }),
        id: s.id.clone(),
    }
}

/// Deserialize a Sound from a legacy JSON roundtrip label.
pub fn sound_from_label(label: &str) -> Option<Sound> {
    if label == SOUND_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:sound,")?;
    serde_json::from_str(json).ok()
}

//! Sound element conversion from MusicXML to MEI.
//!
//! Converts standalone MusicXML `<sound>` elements to MEI `<dir>` control events
//! with JSON-in-label for lossless roundtrip. Uses the same pattern as
//! harmony, figured-bass, and print conversions.

use crate::context::ConversionContext;
use crate::model::direction::Sound;
use tusk_model::elements::{Dir, DirChild, MeasureChild};

/// Label prefix for MEI dir elements carrying standalone sound JSON data.
pub const SOUND_LABEL_PREFIX: &str = "musicxml:sound,";

/// Convert a standalone MusicXML `<sound>` element to an MEI `<dir>` measure child.
///
/// The full Sound struct is serialized as JSON in the dir's `@label` attribute
/// for lossless roundtrip. A human-readable summary is stored as the text child.
pub fn convert_sound(sound: &Sound, ctx: &mut ConversionContext) -> MeasureChild {
    let json_label = serde_json::to_string(sound)
        .ok()
        .map(|json| format!("{}{}", SOUND_LABEL_PREFIX, json));

    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("sound"));
    if let Some(label) = json_label {
        dir.common.label = Some(label);
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

/// Deserialize a Sound from a roundtrip label string (standalone sound).
pub fn sound_from_label(label: &str) -> Option<Sound> {
    let json = label.strip_prefix(SOUND_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

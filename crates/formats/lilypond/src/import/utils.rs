//! Utility functions for LilyPond import.

use tusk_model::elements::{Layer, LayerChild};

use crate::model::{Music, ToplevelExpression};

use super::context_analysis::{is_staff_context, is_staff_group_context};

/// Parse a serialized `\tempo ...` string back into a Tempo AST node.
pub(super) fn parse_tempo_from_serialized(s: &str) -> Option<crate::model::signature::Tempo> {
    use crate::parser::Parser;
    // Wrap in a parseable form: the serialized string is the full \tempo expression
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                if let Music::Tempo(t) = m {
                    return Some(t.clone());
                }
            }
        }
        if let ToplevelExpression::Music(Music::Tempo(t)) = item {
            return Some(t.clone());
        }
    }
    None
}

/// Extract voice streams from LilyPond music.
///
/// If the top-level music is `Simultaneous` and each child is a distinct
/// voice (Sequential block or single event), each child becomes a separate
/// voice (MEI layer). Otherwise, all music goes into a single voice.
///
/// Also unwraps `\relative`, `\transpose`, `\fixed` wrappers to find
/// inner `Simultaneous` music. When unwrapping, only bare music items
/// (no ContextedMusic) are accepted, since ContextedMusic contexts like
/// `\new Voice` are handled by `analyze_staves`.
pub(super) fn extract_voices(music: &Music) -> Vec<Vec<&Music>> {
    // Try splitting the direct music (allows ContextedMusic like \new Voice)
    if let Some(voices) = try_split_simultaneous(music, true) {
        return voices;
    }
    // Try unwrapping pitch context wrappers, then split with strict mode
    let inner = unwrap_pitch_context(music);
    if !std::ptr::eq(inner, music) {
        if let Some(voices) = try_split_simultaneous(inner, false) {
            return voices;
        }
    }
    vec![vec![music]]
}

/// Unwrap `\relative` / `\transpose` / `\fixed` / single-item Sequential
/// to find the structurally significant inner music.
fn unwrap_pitch_context(music: &Music) -> &Music {
    match music {
        Music::Relative { body, .. } | Music::Fixed { body, .. } => unwrap_pitch_context(body),
        Music::Transpose { body, .. } => unwrap_pitch_context(body),
        Music::Sequential(items) if items.len() == 1 => unwrap_pitch_context(&items[0]),
        _ => music,
    }
}

/// Try to split Simultaneous music into separate voice streams.
/// With `allow_context=true`, ContextedMusic (Voice etc.) is accepted.
/// With `allow_context=false`, only bare music items are accepted.
fn try_split_simultaneous(music: &Music, allow_context: bool) -> Option<Vec<Vec<&Music>>> {
    if let Music::Simultaneous(items) = music {
        if items.len() > 1 {
            let all_voice_like = items.iter().all(|item| {
                matches!(
                    item,
                    Music::Sequential(_)
                        | Music::Note(_)
                        | Music::Chord(_)
                        | Music::ChordRepetition(_)
                        | Music::Rest(_)
                        | Music::MultiMeasureRest(_)
                        | Music::Relative { .. }
                        | Music::Fixed { .. }
                        | Music::Transpose { .. }
                ) || (allow_context && matches!(
                    item,
                    Music::ContextedMusic { context_type, .. } if !is_staff_context(context_type) && !is_staff_group_context(context_type)
                ))
            });
            if all_voice_like {
                return Some(items.iter().map(|item| vec![item]).collect());
            }
        }
    }
    None
}

/// Check if a voice's music items are "bare" (no pitch context wrappers).
///
/// Returns true if none of the voice items have `\relative`/`\fixed`/`\transpose`.
/// When true and the staff has `original_music` with pitch context, the import
/// should pre-initialize `PitchContext` so bare voice items get correct pitch resolution.
pub(super) fn voice_needs_pitch_context(voice: &[&Music]) -> bool {
    voice.iter().all(|m| {
        !matches!(
            m,
            Music::Relative { .. } | Music::Fixed { .. } | Music::Transpose { .. }
        )
    })
}


/// Check if a `\tweak` path targets an element ID property.
///
/// Matches paths like `id`, `NoteHead.id`, etc. — the last segment is `id`.
pub(super) fn is_id_tweak(path: &crate::model::PropertyPath) -> bool {
    path.segments
        .last()
        .is_some_and(|s| matches!(s, crate::model::PathSegment::Named(n) if n == "id"))
}

/// Extract a string value from a `PropertyValue`, if it is a `String` or `SchemeExpr::String`.
pub(super) fn extract_tweak_string_value(value: &crate::model::PropertyValue) -> Option<String> {
    match value {
        crate::model::PropertyValue::String(s) => Some(s.clone()),
        crate::model::PropertyValue::SchemeExpr(crate::model::SchemeExpr::String(s)) => {
            Some(s.clone())
        }
        _ => None,
    }
}

/// Set `xml:id` on the last note/rest/chord in a layer.
pub(super) fn set_xml_id_on_last_layer_child(layer: &mut Layer, id: &str) {
    let last = layer.children.last_mut();
    let xml_id = match last {
        Some(LayerChild::Note(n)) => &mut n.common.xml_id,
        Some(LayerChild::Rest(r)) => &mut r.common.xml_id,
        Some(LayerChild::Chord(c)) => &mut c.common.xml_id,
        _ => return,
    };
    *xml_id = Some(id.to_string());
}

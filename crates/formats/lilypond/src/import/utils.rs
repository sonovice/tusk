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
pub(super) fn extract_voices(music: &Music) -> Vec<Vec<&Music>> {
    match music {
        Music::Simultaneous(items) if items.len() > 1 => {
            // Check if children look like separate voice streams
            // (each is a Sequential block or a single event, NOT \new Staff)
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
                ) || matches!(
                    item,
                    Music::ContextedMusic { context_type, .. } if !is_staff_context(context_type) && !is_staff_group_context(context_type)
                )
            });
            if all_voice_like {
                items.iter().map(|item| vec![item]).collect()
            } else {
                vec![vec![music]]
            }
        }
        _ => vec![vec![music]],
    }
}

/// Escape pipe characters in JSON so they don't break `|`-delimited label segments.
pub(super) fn escape_json_pipe(json: &str) -> String {
    json.replace('|', "\\u007c")
}

/// Append a label segment to the last note/rest/chord in a layer.
pub(super) fn append_label_to_last_layer_child(layer: &mut Layer, segment: &str) {
    let last = layer.children.last_mut();
    let label = match last {
        Some(LayerChild::Note(n)) => &mut n.common.label,
        Some(LayerChild::Rest(r)) => &mut r.common.label,
        Some(LayerChild::Chord(c)) => &mut c.common.label,
        _ => return,
    };
    match label {
        Some(existing) => {
            existing.push('|');
            existing.push_str(segment);
        }
        None => *label = Some(segment.to_string()),
    }
}

/// Check if a `\tweak` path targets an element ID property.
///
/// Matches paths like `id`, `NoteHead.id`, etc. — the last segment is `id`.
pub(super) fn is_id_tweak(path: &crate::model::PropertyPath) -> bool {
    path.segments.last().is_some_and(|s| s == "id")
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

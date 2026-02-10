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

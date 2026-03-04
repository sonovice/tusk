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
        // Try splitting Sequential with embedded Simultaneous blocks
        if let Some(voices) = try_split_sequential_with_simultaneous(inner) {
            return voices;
        }
    }
    // Try on original music too (may be a Sequential without wrappers)
    if let Some(voices) = try_split_sequential_with_simultaneous(music) {
        return voices;
    }
    vec![vec![music]]
}

/// Unwrap structural wrappers to find inner `Simultaneous` music for voice splitting.
///
/// Traverses `\relative`, `\transpose`, `\fixed`, single-item Sequential,
/// ContextedMusic (Voice etc.), and Sequential-with-prefix-items to find
/// the inner `<< { } { } >>` that represents multiple voices.
fn unwrap_pitch_context(music: &Music) -> &Music {
    match music {
        Music::Relative { body, .. } | Music::Fixed { body, .. } => unwrap_pitch_context(body),
        Music::Transpose { body, .. } | Music::DrumMode { body } => unwrap_pitch_context(body),
        Music::Sequential(items) if items.len() == 1 => unwrap_pitch_context(&items[0]),
        // Sequential with a Simultaneous as last item preceded by non-note prefix
        // items (e.g. `{ \set ... << { } { } >> }`) — unwrap to the Simultaneous
        Music::Sequential(items) if items.len() > 1 => {
            if let Some(last) = items.last()
                && matches!(last, Music::Simultaneous(_))
                && items[..items.len() - 1].iter().all(|m| is_prefix_item(m))
            {
                last
            }
            // Single Simultaneous + only boundary markers (BarLine/BarCheck):
            // e.g. `{ << { v1 } { v2 } >> \bar "|." }` — unwrap to the Sim
            else if items.iter().filter(|m| matches!(m, Music::Simultaneous(_))).count() == 1
                && items.iter().all(|m| {
                    matches!(
                        m,
                        Music::Simultaneous(_) | Music::BarLine { .. } | Music::BarCheck
                    )
                })
            {
                items
                    .iter()
                    .find(|m| matches!(m, Music::Simultaneous(_)))
                    .unwrap()
            } else {
                music
            }
        }
        // Unwrap ContextedMusic (e.g. `\context Voice = "name" { ... }`)
        // to find inner Simultaneous or Sequential-with-Sims for voice splitting.
        // Pitch context (\relative etc.) is preserved via original_music in the caller.
        Music::ContextedMusic { music: inner, .. } => {
            let unwrapped = unwrap_pitch_context(inner);
            match unwrapped {
                Music::Simultaneous(_) => unwrapped,
                // Return Sequential only if it contains Simultaneous blocks
                // (the per-measure voice pattern from export)
                Music::Sequential(items)
                    if items.iter().any(|m| matches!(m, Music::Simultaneous(_))) =>
                {
                    unwrapped
                }
                _ => music,
            }
        }
        _ => music,
    }
}

/// Check if a music item is a non-note prefix (settings, overrides, etc.)
/// that can be hoisted before voice splitting.
fn is_prefix_item(music: &Music) -> bool {
    matches!(
        music,
        Music::Set { .. }
            | Music::Unset { .. }
            | Music::Override { .. }
            | Music::Revert { .. }
            | Music::Identifier(_)
            | Music::MusicFunction { .. }
            | Music::BarCheck
            | Music::LineComment(_)
    )
}

/// Try to split Simultaneous music into separate voice streams.
/// With `allow_context=true`, ContextedMusic (Voice etc.) is accepted.
/// With `allow_context=false`, only bare music items are accepted.
fn try_split_simultaneous(music: &Music, allow_context: bool) -> Option<Vec<Vec<&Music>>> {
    if let Music::Simultaneous(items) = music
        && items.len() > 1 {
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
    None
}

/// Split a Sequential that contains embedded Simultaneous blocks into voices.
///
/// Handles patterns like `{ << { v1 } { v2 } >> | items | << { v1 } { v2 } >> }`
/// where the export produced per-measure `<<>>` blocks separated by bar checks,
/// with single-voice measures as flat items between them.
///
/// Validates per-segment (between BarChecks): each segment must be either
/// a single Simultaneous (multi-voice measure) or all non-Simultaneous items
/// (single-voice measure). Mixed segments (notes + `<<>>` in same measure)
/// are rejected to avoid incorrectly splitting `{ notes <<>> }` patterns.
fn try_split_sequential_with_simultaneous(music: &Music) -> Option<Vec<Vec<&Music>>> {
    let items = match music {
        Music::Sequential(items) if items.len() > 1 => items,
        _ => return None,
    };

    // Segment items at BarCheck boundaries
    let mut segments: Vec<Vec<&Music>> = vec![Vec::new()];
    for item in items {
        if matches!(item, Music::BarCheck) {
            segments.push(Vec::new());
        } else {
            segments.last_mut().unwrap().push(item);
        }
    }
    // Remove trailing empty segment
    if segments.last().is_some_and(|s| s.is_empty()) {
        segments.pop();
    }

    // Find the maximum voice count from Simultaneous blocks.
    // Segments with Sim blocks may have varying voice counts (some voices
    // may be empty in some measures), so we use the maximum and pad shorter
    // blocks with their existing children.
    let mut max_voice_count = 0usize;
    let mut has_any_sim = false;
    for seg in &segments {
        let sim = seg.iter().find_map(|item| {
            if let Music::Simultaneous(sim_items) = item {
                Some(sim_items)
            } else {
                None
            }
        });
        if let Some(sim_items) = sim {
            // All non-Sim items in this segment must be non-note items
            let non_sim_ok = seg
                .iter()
                .all(|item| matches!(item, Music::Simultaneous(_)) || is_non_note_item(item));
            if non_sim_ok && sim_items.len() > 1 && sim_items.iter().all(|si| is_voice_like(si)) {
                has_any_sim = true;
                if sim_items.len() > max_voice_count {
                    max_voice_count = sim_items.len();
                }
            }
        }
    }
    if !has_any_sim || max_voice_count < 2 {
        return None;
    }
    let voice_count = max_voice_count;

    // Validate each segment: a Simultaneous segment may only contain
    // Simultaneous blocks plus optional non-note items (no notes/chords mixed in).
    // A non-Simultaneous segment must contain no Simultaneous items.
    // Sim blocks may have fewer voices than voice_count (empty voices dropped
    // by the export); those will produce fewer split items for that measure.
    for seg in &segments {
        let has_sim = seg.iter().any(|item| matches!(item, Music::Simultaneous(_)));
        if has_sim {
            // Check that non-Sim items are only non-note items
            let has_content = seg.iter().any(|item| {
                !matches!(item, Music::Simultaneous(_)) && !is_non_note_item(item)
            });
            if has_content {
                return None; // Notes/chords mixed with <<>> in same segment
            }
            // Validate all Sim children are voice-like
            for item in seg {
                if let Music::Simultaneous(sim_items) = item {
                    if sim_items.len() > voice_count
                        || !sim_items.iter().all(|si| is_voice_like(si))
                    {
                        return None;
                    }
                }
            }
        }
    }

    // Build voices: Simultaneous children → respective voices,
    // BarChecks → all voices (so measure boundaries align),
    // other items → first voice only
    let mut voices: Vec<Vec<&Music>> = (0..voice_count).map(|_| Vec::new()).collect();
    for item in items {
        if let Music::Simultaneous(sim_items) = item {
            for (i, sim_item) in sim_items.iter().enumerate() {
                voices[i].push(sim_item);
            }
        } else if matches!(item, Music::BarCheck) {
            for voice in &mut voices {
                voice.push(item);
            }
        } else {
            voices[0].push(item);
        }
    }
    Some(voices)
}

/// Check if a music item is a non-note item that can appear alongside a
/// Simultaneous in a segment without invalidating voice splitting.
/// These are boundary markers, comments, and control items the export places
/// adjacent to `<<>>` blocks.
fn is_non_note_item(music: &Music) -> bool {
    matches!(
        music,
        Music::BarLine { .. }
            | Music::LineComment(_)
            | Music::Set { .. }
            | Music::Unset { .. }
            | Music::Override { .. }
            | Music::Revert { .. }
            | Music::MusicFunction { .. }
    )
}

/// Check if a music item is voice-like (can be a child of Simultaneous for
/// voice splitting).
fn is_voice_like(music: &Music) -> bool {
    matches!(
        music,
        Music::Sequential(_)
            | Music::Note(_)
            | Music::Chord(_)
            | Music::ChordRepetition(_)
            | Music::Rest(_)
            | Music::MultiMeasureRest(_)
            | Music::Relative { .. }
            | Music::Fixed { .. }
            | Music::Transpose { .. }
            | Music::ContextedMusic { .. }
    )
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

//! Variable and assignment resolution for LilyPond import.
//!
//! Collects top-level assignments, builds a variable→Music map,
//! resolves `Music::Identifier` references, and builds label segments
//! for roundtrip storage.

use std::collections::HashMap;

use crate::model::{Assignment, AssignmentValue, Music, ToplevelExpression};
use crate::serializer;

use super::signatures;

/// Collect all top-level assignments from the LilyPond file.
pub(super) fn collect_assignments(file: &crate::model::LilyPondFile) -> Vec<Assignment> {
    file.items
        .iter()
        .filter_map(|item| {
            if let ToplevelExpression::Assignment(a) = item {
                Some(a.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Build a name→Music map from assignments whose values are music expressions.
pub(super) fn build_variable_map(assignments: &[Assignment]) -> HashMap<String, Music> {
    let mut map = HashMap::new();
    for a in assignments {
        if let AssignmentValue::Music(m) = &a.value {
            map.insert(a.name.clone(), *m.clone());
        } else if let AssignmentValue::Identifier(ref_name) = &a.value {
            // Transitive: if "soprano = \melody", resolve \melody from map
            if let Some(resolved) = map.get(ref_name) {
                map.insert(a.name.clone(), resolved.clone());
            }
        }
    }
    map
}

/// Resolve `Music::Identifier` references in a music tree using the variable map.
///
/// Returns a new tree with identifiers replaced by their assigned music values.
pub(super) fn resolve_identifiers(music: &Music, var_map: &HashMap<String, Music>) -> Music {
    match music {
        Music::Identifier(name) => {
            if let Some(resolved) = var_map.get(name) {
                resolved.clone()
            } else {
                music.clone()
            }
        }
        Music::Sequential(items) => Music::Sequential(
            items
                .iter()
                .map(|m| resolve_identifiers(m, var_map))
                .collect(),
        ),
        Music::Simultaneous(items) => Music::Simultaneous(
            items
                .iter()
                .map(|m| resolve_identifiers(m, var_map))
                .collect(),
        ),
        Music::Relative { pitch, body } => Music::Relative {
            pitch: pitch.clone(),
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::Fixed { pitch, body } => Music::Fixed {
            pitch: pitch.clone(),
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::Transpose { from, to, body } => Music::Transpose {
            from: from.clone(),
            to: to.clone(),
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::Tuplet {
            numerator,
            denominator,
            span_duration,
            body,
        } => Music::Tuplet {
            numerator: *numerator,
            denominator: *denominator,
            span_duration: span_duration.clone(),
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::ContextedMusic {
            keyword,
            context_type,
            name,
            with_block,
            music: inner,
        } => Music::ContextedMusic {
            keyword: *keyword,
            context_type: context_type.clone(),
            name: name.clone(),
            with_block: with_block.clone(),
            music: Box::new(resolve_identifiers(inner, var_map)),
        },
        Music::Grace { body } => Music::Grace {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::Acciaccatura { body } => Music::Acciaccatura {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::Appoggiatura { body } => Music::Appoggiatura {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::AfterGrace {
            fraction,
            main,
            grace,
        } => Music::AfterGrace {
            fraction: *fraction,
            main: Box::new(resolve_identifiers(main, var_map)),
            grace: Box::new(resolve_identifiers(grace, var_map)),
        },
        Music::Repeat {
            repeat_type,
            count,
            body,
            alternatives,
        } => Music::Repeat {
            repeat_type: *repeat_type,
            count: *count,
            body: Box::new(resolve_identifiers(body, var_map)),
            alternatives: alternatives.as_ref().map(|alts| {
                alts.iter()
                    .map(|a| resolve_identifiers(a, var_map))
                    .collect()
            }),
        },
        Music::AddLyrics {
            music: inner,
            lyrics,
        } => Music::AddLyrics {
            music: Box::new(resolve_identifiers(inner, var_map)),
            lyrics: lyrics
                .iter()
                .map(|l| resolve_identifiers(l, var_map))
                .collect(),
        },
        Music::ChordMode { body } => Music::ChordMode {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::DrumMode { body } => Music::DrumMode {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::FigureMode { body } => Music::FigureMode {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::LyricMode { body } => Music::LyricMode {
            body: Box::new(resolve_identifiers(body, var_map)),
        },
        Music::Once { music: inner } => Music::Once {
            music: Box::new(resolve_identifiers(inner, var_map)),
        },
        // Leaf nodes: no recursion needed
        _ => music.clone(),
    }
}

/// Build a label segment for top-level assignments.
///
/// Format: `lilypond:vars,{escaped_serialized_assignments}`
/// Each assignment is serialized to LilyPond text, joined by `\n`, then escaped.
pub(super) fn build_assignments_label(assignments: &[Assignment]) -> String {
    if assignments.is_empty() {
        return String::new();
    }
    let serialized: Vec<String> = assignments
        .iter()
        .map(serializer::serialize_assignment)
        .collect();
    let joined = serialized.join("\n");
    let escaped = signatures::escape_label_value_pub(&joined);
    format!("lilypond:vars,{escaped}")
}

/// Parse a `lilypond:vars,` label segment back into assignments.
///
/// Returns `None` if the label doesn't start with the expected prefix.
pub(crate) fn parse_assignments_label(label: &str) -> Option<Vec<Assignment>> {
    let escaped = label.strip_prefix("lilypond:vars,")?;
    let serialized = signatures::unescape_label_value(escaped);

    // Parse each line as an assignment
    use crate::parser::Parser;

    let file = Parser::new(&serialized).ok()?.parse().ok()?;
    let mut assignments = Vec::new();
    for item in file.items {
        if let ToplevelExpression::Assignment(a) = item {
            assignments.push(a);
        }
    }
    if assignments.is_empty() {
        None
    } else {
        Some(assignments)
    }
}

//! Variable and assignment resolution for LilyPond import.
//!
//! Collects top-level assignments, builds a variable→Music map,
//! resolves `Music::Identifier` references, and builds label segments
//! for roundtrip storage.

use std::collections::HashMap;

use tusk_model::{ExtAssignment, ExtValue, VariableAssignments};

use crate::model::{Assignment, AssignmentValue, FunctionArg, Music, ToplevelExpression};
use crate::serializer;

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
        Music::MusicFunction { name, args } => Music::MusicFunction {
            name: name.clone(),
            args: resolve_function_args(args, var_map),
        },
        Music::PartialFunction { name, args } => Music::PartialFunction {
            name: name.clone(),
            args: resolve_function_args(args, var_map),
        },
        // Leaf nodes: no recursion needed
        _ => music.clone(),
    }
}

fn resolve_function_args(
    args: &[FunctionArg],
    var_map: &HashMap<String, Music>,
) -> Vec<FunctionArg> {
    args.iter()
        .map(|arg| match arg {
            FunctionArg::Music(m) => FunctionArg::Music(resolve_identifiers(m, var_map)),
            other => other.clone(),
        })
        .collect()
}

/// Build a typed `VariableAssignments` extension from assignments.
///
/// Each assignment is serialized to LilyPond text and stored as a
/// name → ExtValue pair.
pub(super) fn build_assignments_ext(assignments: &[Assignment]) -> VariableAssignments {
    let exts = assignments
        .iter()
        .map(|a| {
            let value = match &a.value {
                AssignmentValue::String(s) => ExtValue::String(s.clone()),
                AssignmentValue::Number(n) => ExtValue::Number(*n),
                _ => {
                    // Music, Identifier, SchemeExpr, Markup, MarkupList
                    // Serialize the whole assignment, extract value part
                    let serialized = serializer::serialize_assignment(a);
                    let val_str = serialized
                        .find(" = ")
                        .map(|i| serialized[i + 3..].to_string())
                        .unwrap_or(serialized);
                    ExtValue::Music(val_str)
                }
            };
            ExtAssignment {
                name: a.name.clone(),
                value,
            }
        })
        .collect();
    VariableAssignments { assignments: exts }
}

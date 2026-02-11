//! Grace note wrapping for LilyPond export.

use tusk_model::elements::LayerChild;

use crate::model::Music;

/// Grace type info extracted from an MEI element's @grace and label.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum ExportGraceType {
    Grace,
    Acciaccatura,
    Appoggiatura,
    AfterGrace { fraction: Option<(u32, u32)> },
}

/// Check if a LayerChild is a grace note and extract the grace type.
fn layer_child_grace_type(child: &LayerChild) -> Option<ExportGraceType> {
    match child {
        LayerChild::Note(note) => {
            note.note_log.grace.as_ref()?;
            Some(parse_grace_label_from_note_label(
                note.common.label.as_deref(),
            ))
        }
        LayerChild::Chord(chord) => {
            chord.chord_log.grace.as_ref()?;
            Some(parse_grace_label_from_note_label(
                chord.common.label.as_deref(),
            ))
        }
        _ => None,
    }
}

/// Check if a BeamChild is a grace note.
fn beam_child_grace_type(child: &tusk_model::elements::BeamChild) -> Option<ExportGraceType> {
    use tusk_model::elements::BeamChild;
    match child {
        BeamChild::Note(note) => {
            note.note_log.grace.as_ref()?;
            Some(parse_grace_label_from_note_label(
                note.common.label.as_deref(),
            ))
        }
        BeamChild::Chord(chord) => {
            chord.chord_log.grace.as_ref()?;
            Some(parse_grace_label_from_note_label(
                chord.common.label.as_deref(),
            ))
        }
        _ => None,
    }
}

/// Parse grace type from a note/chord's label.
///
/// Label segment: `tusk:grace,{json}` using typed `GraceInfo`.
fn parse_grace_label_from_note_label(label: Option<&str>) -> ExportGraceType {
    let label = match label {
        Some(l) => l,
        None => return ExportGraceType::Grace,
    };
    // Find the `tusk:grace,...` segment (may be pipe-separated)
    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix("tusk:grace,")
            && let Ok(info) = serde_json::from_str::<tusk_model::GraceInfo>(json)
        {
            return grace_info_to_export(&info);
        }
    }
    ExportGraceType::Grace
}

/// Convert a typed `GraceInfo` to the export-side `ExportGraceType`.
fn grace_info_to_export(info: &tusk_model::GraceInfo) -> ExportGraceType {
    match info {
        tusk_model::GraceInfo::Grace => ExportGraceType::Grace,
        tusk_model::GraceInfo::Acciaccatura => ExportGraceType::Acciaccatura,
        tusk_model::GraceInfo::Appoggiatura => ExportGraceType::Appoggiatura,
        tusk_model::GraceInfo::AfterGrace { fraction } => ExportGraceType::AfterGrace {
            fraction: *fraction,
        },
    }
}

/// Collect grace type info from layer children, producing a parallel array.
///
/// Each entry corresponds to one Music item in the output. For Beam children,
/// each inner child produces one entry.
pub(super) fn collect_grace_types(layer_children: &[LayerChild]) -> Vec<Option<ExportGraceType>> {
    let mut types = Vec::new();
    for child in layer_children {
        match child {
            LayerChild::Beam(beam) => {
                for bc in &beam.children {
                    types.push(beam_child_grace_type(bc));
                }
            }
            _ => {
                types.push(layer_child_grace_type(child));
            }
        }
    }
    types
}

/// Wrap consecutive grace notes in Music::Grace/Acciaccatura/Appoggiatura/AfterGrace.
///
/// For `\afterGrace`, the grace notes follow the main note. The main note is the
/// non-grace note immediately before the grace group.
pub(super) fn apply_grace_wrapping(
    items: &mut Vec<Music>,
    grace_types: &[Option<ExportGraceType>],
) {
    if items.is_empty() {
        return;
    }

    // Build a list of grace groups: (start_idx, end_idx, grace_type)
    let mut groups: Vec<(usize, usize, ExportGraceType)> = Vec::new();
    let mut i = 0;
    while i < items.len() && i < grace_types.len() {
        if let Some(ref gt) = grace_types[i] {
            let group_type = gt.clone();
            let start = i;
            // Find end of consecutive grace notes with same type
            while i < items.len()
                && i < grace_types.len()
                && grace_types[i].as_ref() == Some(&group_type)
            {
                i += 1;
            }
            groups.push((start, i - 1, group_type));
        } else {
            i += 1;
        }
    }

    // Process groups in reverse order to avoid index shifting
    for (start, end, gt) in groups.into_iter().rev() {
        let grace_items: Vec<Music> = items.drain(start..=end).collect();
        let grace_body = if grace_items.len() == 1 {
            grace_items.into_iter().next().unwrap()
        } else {
            Music::Sequential(grace_items)
        };

        match gt {
            ExportGraceType::Grace => {
                items.insert(
                    start,
                    Music::Grace {
                        body: Box::new(grace_body),
                    },
                );
            }
            ExportGraceType::Acciaccatura => {
                items.insert(
                    start,
                    Music::Acciaccatura {
                        body: Box::new(grace_body),
                    },
                );
            }
            ExportGraceType::Appoggiatura => {
                items.insert(
                    start,
                    Music::Appoggiatura {
                        body: Box::new(grace_body),
                    },
                );
            }
            ExportGraceType::AfterGrace { fraction } => {
                // AfterGrace: the main note is immediately before the grace group
                if start > 0 {
                    let main = items.remove(start - 1);
                    items.insert(
                        start - 1,
                        Music::AfterGrace {
                            fraction,
                            main: Box::new(main),
                            grace: Box::new(grace_body),
                        },
                    );
                } else {
                    // No preceding main note — fall back to regular grace
                    items.insert(
                        start,
                        Music::Grace {
                            body: Box::new(grace_body),
                        },
                    );
                }
            }
        }
    }
}

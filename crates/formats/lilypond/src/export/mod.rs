//! Conversion from MEI to LilyPond AST.

mod book;
mod conversion;
pub(crate) mod lyrics;
mod output_defs;
mod pitch_context;
mod repeats;
mod signatures;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_book;
#[cfg(test)]
mod tests_chords;
#[cfg(test)]
mod tests_drums;
#[cfg(test)]
mod tests_export_completion;
#[cfg(test)]
mod tests_figures;
#[cfg(test)]
mod tests_functions;
#[cfg(test)]
mod tests_markup;
#[cfg(test)]
mod tests_output_defs;
#[cfg(test)]
mod tests_properties;
#[cfg(test)]
mod tests_scheme;
#[cfg(test)]
mod tests_tempo_marks;
#[cfg(test)]
mod tests_variables;

use std::collections::HashMap;
use thiserror::Error;
use tusk_model::elements::{
    LayerChild, MeasureChild, Mei, MeiChild, ScoreChild, ScoreDefChild, SectionChild, StaffGrpChild,
};
use tusk_model::{StaffContext, VariableAssignments};

use crate::model::Duration;
use crate::model::note::{Direction, PostEvent, ScriptAbbreviation};
use crate::model::{
    ContextKeyword, LilyPondFile, Music, ScoreBlock, ScoreItem, ToplevelExpression, Version,
};

use conversion::{convert_mei_chord, convert_mei_mrest, convert_mei_note, convert_mei_rest};
use pitch_context::{apply_pitch_contexts, extract_pitch_contexts};
use signatures::{extract_event_sequences, inject_signature_events};

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("LilyPond export is not yet implemented")]
    NotImplemented,
    #[error("no music found in MEI document")]
    NoMusic,
    #[error("export error: {0}")]
    Other(String),
}

/// Convert an MEI document to a LilyPond AST.
pub fn export(mei: &Mei) -> Result<LilyPondFile, ExportError> {
    // Check for book-structured multi-mdiv layout
    if let Some(entries) = book::find_book_entries(mei) {
        return export_book(mei, &entries);
    }

    // Single-score path (backward compatible)
    let score = find_score(mei).ok_or(ExportError::NoMusic)?;
    let score_block = export_single_score(score);

    // Build top-level items: assignments, header, paper, layout, midi, then score
    let mut items = Vec::new();

    let assignments = extract_assignments(score);
    for a in assignments {
        items.push(ToplevelExpression::Assignment(a));
    }

    let (top_header, top_paper, top_layout, top_midi) = output_defs::extract_toplevel_blocks(mei);
    if let Some(hb) = top_header {
        items.push(ToplevelExpression::Header(hb));
    }
    if let Some(pb) = top_paper {
        items.push(ToplevelExpression::Paper(pb));
    }
    if let Some(lb) = top_layout {
        items.push(ToplevelExpression::Layout(lb));
    }
    if let Some(mb) = top_midi {
        items.push(ToplevelExpression::Midi(mb));
    }

    items.push(ToplevelExpression::Score(score_block));

    Ok(LilyPondFile {
        version: Some(Version {
            version: "2.24.0".to_string(),
        }),
        items,
    })
}

/// Export a book-structured MEI (multiple mdivs with BookStructure labels).
fn export_book(mei: &Mei, entries: &[book::MdivEntry<'_>]) -> Result<LilyPondFile, ExportError> {
    let mut items = Vec::new();

    // Extract top-level assignments from first score's scoreDef
    if let Some(first) = entries.first() {
        let assignments = extract_assignments(first.score);
        for a in assignments {
            items.push(ToplevelExpression::Assignment(a));
        }
    }

    // Extract top-level blocks from MeiHead ExtMeta
    let (top_header, top_paper, top_layout, top_midi) = output_defs::extract_toplevel_blocks(mei);
    if let Some(hb) = top_header {
        items.push(ToplevelExpression::Header(hb));
    }
    if let Some(pb) = top_paper {
        items.push(ToplevelExpression::Paper(pb));
    }
    if let Some(lb) = top_layout {
        items.push(ToplevelExpression::Layout(lb));
    }
    if let Some(mb) = top_midi {
        items.push(ToplevelExpression::Midi(mb));
    }

    // Reconstruct book/bookpart hierarchy
    let book_items = book::reconstruct_books(entries, &export_single_score);
    items.extend(book_items);

    Ok(LilyPondFile {
        version: Some(Version {
            version: "2.24.0".to_string(),
        }),
        items,
    })
}

/// Export a single MEI Score to a LilyPond ScoreBlock.
fn export_single_score(score: &tusk_model::elements::Score) -> ScoreBlock {
    let group_meta = extract_group_meta(score);
    let staff_metas = extract_staff_metas(score);
    let event_sequences = extract_event_sequences(score);
    let pitch_contexts = extract_pitch_contexts(score);
    let lyrics_infos = extract_lyrics_infos(score);

    let mut staff_music: Vec<Vec<Vec<Music>>> = Vec::new();
    let mut staff_layer_children: Vec<Vec<&[LayerChild]>> = Vec::new();

    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for section_child in &section.children {
                if let SectionChild::Measure(measure) = section_child {
                    let mut post_event_map = collect_slur_post_events(&measure.children);
                    collect_dynam_post_events(&measure.children, &mut post_event_map);
                    collect_hairpin_post_events(&measure.children, &mut post_event_map);
                    collect_artic_post_events(&measure.children, &mut post_event_map);
                    collect_ornament_post_events(&measure.children, &mut post_event_map);

                    let property_ops = collect_property_ops(&measure.children);
                    let function_ops = collect_function_ops(&measure.children);
                    let tuplet_spans = collect_tuplet_spans(&measure.children);
                    let repeat_spans = collect_repeat_spans(&measure.children);
                    let ending_spans = collect_ending_spans(&measure.children);

                    let mut staff_idx = 0usize;
                    for mc in &measure.children {
                        if let MeasureChild::Staff(staff) = mc {
                            let mut layers: Vec<Vec<Music>> = Vec::new();
                            let mut raw_layers: Vec<&[LayerChild]> = Vec::new();
                            for sc in &staff.children {
                                let tusk_model::elements::StaffChild::Layer(layer) = sc;
                                raw_layers.push(&layer.children);
                                let grace_types = collect_grace_types(&layer.children);
                                let mut items = Vec::new();
                                let mut item_ids = Vec::new();
                                for lc in &layer.children {
                                    let start = items.len();
                                    convert_layer_child_to_items(lc, &post_event_map, &mut items);
                                    collect_layer_child_ids(lc, &mut item_ids, items.len() - start);
                                }
                                inject_property_ops(&mut items, &item_ids, &property_ops);
                                inject_function_ops(&mut items, &item_ids, &function_ops);
                                apply_tuplet_wrapping(&mut items, &item_ids, &tuplet_spans);
                                apply_grace_wrapping(&mut items, &grace_types);
                                apply_repeat_wrapping(
                                    &mut items,
                                    &item_ids,
                                    &repeat_spans,
                                    &ending_spans,
                                );
                                layers.push(items);
                            }

                            if let Some(seq) = event_sequences.get(staff_idx) {
                                inject_signature_events(&mut layers, seq);
                            }

                            staff_music.push(layers);
                            staff_layer_children.push(raw_layers);
                            staff_idx += 1;
                        }
                    }
                }
            }
        }
    }

    let chord_mode_events = collect_chord_mode_harms(score);
    let chord_names_meta = extract_chord_names_meta(score);
    let figure_mode_events = collect_figure_mode_fbs(score);
    let figured_bass_meta = extract_figured_bass_meta(score);

    apply_pitch_contexts(&mut staff_music, &pitch_contexts);

    let music = build_music_with_contexts(
        staff_music,
        &group_meta,
        &staff_metas,
        &lyrics_infos,
        &staff_layer_children,
        &chord_mode_events,
        &chord_names_meta,
        &figure_mode_events,
        &figured_bass_meta,
    );

    let mut score_items = vec![ScoreItem::Music(music)];
    let score_blocks = output_defs::extract_score_blocks(score);
    score_items.extend(score_blocks);

    ScoreBlock { items: score_items }
}

/// Extract stored variable assignments from scoreDef label.
///
/// Reads typed `tusk:vars,{json}` segments from the scoreDef's `@label` attribute,
/// then re-parses the serialized assignment values through the LilyPond parser.
fn extract_assignments(score: &tusk_model::elements::Score) -> Vec<crate::model::Assignment> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child
            && let Some(label) = &score_def.common.label
        {
            for segment in label.split('|') {
                if let Some(json) = segment.strip_prefix("tusk:vars,")
                    && let Ok(vars) = serde_json::from_str::<VariableAssignments>(json)
                {
                    return vars
                        .assignments
                        .into_iter()
                        .filter_map(ext_assignment_to_model)
                        .collect();
                }
            }
        }
    }
    Vec::new()
}

/// Convert a typed ExtAssignment back to a model Assignment.
fn ext_assignment_to_model(ea: tusk_model::ExtAssignment) -> Option<crate::model::Assignment> {
    use crate::model::{Assignment, AssignmentValue};
    use crate::parser::Parser;

    let value = match ea.value {
        tusk_model::ExtValue::String(s) => AssignmentValue::String(s),
        tusk_model::ExtValue::Number(n) => AssignmentValue::Number(n),
        tusk_model::ExtValue::Music(src) => {
            // Re-parse: wrap as `name = VALUE`
            let full = format!("{} = {src}", ea.name);
            if let Ok(file) = Parser::new(&full).and_then(|p| p.parse()) {
                for item in file.items {
                    if let ToplevelExpression::Assignment(a) = item {
                        return Some(a);
                    }
                }
            }
            return None;
        }
        tusk_model::ExtValue::Scheme(src) => {
            let full = format!("{} = {src}", ea.name);
            if let Ok(file) = Parser::new(&full).and_then(|p| p.parse()) {
                for item in file.items {
                    if let ToplevelExpression::Assignment(a) = item {
                        return Some(a);
                    }
                }
            }
            return None;
        }
        tusk_model::ExtValue::Markup(src) => {
            let full = format!("{} = {src}", ea.name);
            if let Ok(file) = Parser::new(&full).and_then(|p| p.parse()) {
                for item in file.items {
                    if let ToplevelExpression::Assignment(a) = item {
                        return Some(a);
                    }
                }
            }
            return None;
        }
        _ => {
            // Identifier, MarkupList, Bool — serialize and re-parse
            let src_str = match &ea.value {
                tusk_model::ExtValue::Identifier(s) => s.clone(),
                tusk_model::ExtValue::MarkupList(s) => s.clone(),
                tusk_model::ExtValue::Bool(b) => format!("#{}", if *b { "#t" } else { "#f" }),
                _ => return None,
            };
            let full = format!("{} = {src_str}", ea.name);
            if let Ok(file) = Parser::new(&full).and_then(|p| p.parse()) {
                for item in file.items {
                    if let ToplevelExpression::Assignment(a) = item {
                        return Some(a);
                    }
                }
            }
            return None;
        }
    };
    Some(Assignment {
        name: ea.name,
        value,
    })
}

/// Extract lyrics export info from all staffDef labels.
fn extract_lyrics_infos(
    score: &tusk_model::elements::Score,
) -> Vec<Option<lyrics::LyricsExportInfo>> {
    let mut infos = Vec::new();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            let info = sdef.labelled.label.as_deref().and_then(|label| {
                                label.split('|').find_map(lyrics::parse_lyrics_info_json)
                            });
                            infos.push(info);
                        }
                    }
                }
            }
        }
    }
    infos
}

// ---------------------------------------------------------------------------
// Context metadata extraction from scoreDef
// ---------------------------------------------------------------------------

/// Metadata for a staff group, extracted from staffGrp label/symbol.
struct GroupMeta {
    context_type: String,
    name: Option<String>,
    with_block_str: Option<String>,
}

/// Metadata for a single staff, extracted from staffDef label.
struct StaffMeta {
    context_type: String,
    name: Option<String>,
    with_block_str: Option<String>,
    /// True when the staff was created with an explicit `\new` or `\context` keyword.
    has_explicit_context: bool,
}

/// Extract group metadata from scoreDef's staffGrp.
fn extract_group_meta(score: &tusk_model::elements::Score) -> Option<GroupMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    // Check label for typed JSON group context
                    if let Some(label) = &grp.common.label {
                        for segment in label.split('|') {
                            if let Some(json) = segment.strip_prefix("tusk:group-context,")
                                && let Ok(ctx) = serde_json::from_str::<StaffContext>(json)
                            {
                                return Some(GroupMeta {
                                    context_type: ctx.context_type,
                                    name: ctx.name,
                                    with_block_str: ctx.with_block,
                                });
                            }
                        }
                    }
                    // Fallback: infer from symbol
                    if let Some(symbol) = &grp.staff_grp_vis.symbol {
                        let context_type = match symbol.as_str() {
                            "brace" => "PianoStaff",
                            "bracket" => "StaffGroup",
                            _ => "StaffGroup",
                        };
                        return Some(GroupMeta {
                            context_type: context_type.to_string(),
                            name: None,
                            with_block_str: None,
                        });
                    }
                }
            }
        }
    }
    None
}

/// Extract staff metadata from scoreDef's staffDef labels.
fn extract_staff_metas(score: &tusk_model::elements::Score) -> Vec<StaffMeta> {
    let mut metas = Vec::new();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            let meta = extract_staff_meta_from_label(sdef);
                            metas.push(meta);
                        }
                    }
                }
            }
        }
    }
    metas
}

/// Extract a single staff's metadata from its label.
fn extract_staff_meta_from_label(sdef: &tusk_model::elements::StaffDef) -> StaffMeta {
    if let Some(label) = &sdef.labelled.label {
        for segment in label.split('|') {
            if let Some(json) = segment.strip_prefix("tusk:staff-context,")
                && let Ok(ctx) = serde_json::from_str::<StaffContext>(json)
            {
                return StaffMeta {
                    context_type: ctx.context_type,
                    name: ctx.name,
                    with_block_str: ctx.with_block,
                    has_explicit_context: ctx.keyword.is_some(),
                };
            }
        }
    }
    StaffMeta {
        context_type: "Staff".to_string(),
        name: None,
        with_block_str: None,
        has_explicit_context: false,
    }
}

/// Parse a stored \with block string back into ContextModItems.
///
/// Re-parses the serialized content by wrapping it in a parseable form.
fn parse_with_block_str(with_str: &str) -> Option<Vec<crate::model::ContextModItem>> {
    use crate::parser::Parser;

    // Wrap in a form the parser can handle:
    // \new X \with { <content> } { }
    let src = format!("\\new X \\with {{\n{with_str}\n}} {{ }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Music(Music::ContextedMusic { with_block, .. }) = item {
            return with_block.clone();
        }
    }
    None
}

/// Build a Music expression from staff/layer structure, wrapping in context.
#[allow(clippy::too_many_arguments)]
fn build_music_with_contexts(
    staff_music: Vec<Vec<Vec<Music>>>,
    group_meta: &Option<GroupMeta>,
    staff_metas: &[StaffMeta],
    lyrics_infos: &[Option<lyrics::LyricsExportInfo>],
    staff_layer_children: &[Vec<&[LayerChild]>],
    chord_mode_events: &[Music],
    chord_names_meta: &Option<ChordNamesMeta>,
    figure_mode_events: &[Music],
    figured_bass_meta: &Option<FiguredBassMeta>,
) -> Music {
    let num_staves = staff_music.len();
    let has_chords = !chord_mode_events.is_empty();
    let has_figures = !figure_mode_events.is_empty();

    // Single staff, no group, no explicit staff context, no chord/figure names -> flat output
    if num_staves <= 1
        && group_meta.is_none()
        && !has_chords
        && !has_figures
        && (staff_metas.is_empty()
            || (staff_metas.len() == 1
                && staff_metas[0].name.is_none()
                && staff_metas[0].with_block_str.is_none()
                && staff_metas[0].context_type == "Staff"
                && !staff_metas[0].has_explicit_context))
    {
        let mut music = build_flat_music(staff_music);
        // Apply lyrics wrapping for single-staff case
        if let Some(Some(info)) = lyrics_infos.first()
            && let Some(raw) = staff_layer_children.first().and_then(|v| v.first())
        {
            music = lyrics::wrap_music_with_lyrics(music, raw, info);
        }
        return music;
    }

    // Standalone figured bass (no staves, no chords)
    if num_staves == 0 && !has_figures {
        return Music::Sequential(Vec::new());
    }
    if num_staves == 0 && has_figures {
        let figure_body = Music::FigureMode {
            body: Box::new(Music::Sequential(figure_mode_events.to_vec())),
        };
        let fb_with = figured_bass_meta
            .as_ref()
            .and_then(|m| m.with_block_str.as_deref())
            .and_then(parse_with_block_str);
        let fb_name = figured_bass_meta.as_ref().and_then(|m| m.name.clone());
        return Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "FiguredBass".to_string(),
            name: fb_name,
            with_block: fb_with,
            music: Box::new(figure_body),
        };
    }

    // Build per-staff music with \new Staff wrappers
    let mut staff_exprs: Vec<Music> = Vec::new();

    // Add ChordNames context if chord-mode events exist
    if has_chords {
        let chord_body = Music::ChordMode {
            body: Box::new(Music::Sequential(chord_mode_events.to_vec())),
        };
        let cn_with = chord_names_meta
            .as_ref()
            .and_then(|m| m.with_block_str.as_deref())
            .and_then(parse_with_block_str);
        let cn_name = chord_names_meta.as_ref().and_then(|m| m.name.clone());
        let chord_names_expr = Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "ChordNames".to_string(),
            name: cn_name,
            with_block: cn_with,
            music: Box::new(chord_body),
        };
        staff_exprs.push(chord_names_expr);
    }

    // Add FiguredBass context if figure events exist
    if has_figures {
        let figure_body = Music::FigureMode {
            body: Box::new(Music::Sequential(figure_mode_events.to_vec())),
        };
        let fb_with = figured_bass_meta
            .as_ref()
            .and_then(|m| m.with_block_str.as_deref())
            .and_then(parse_with_block_str);
        let fb_name = figured_bass_meta.as_ref().and_then(|m| m.name.clone());
        let fb_expr = Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "FiguredBass".to_string(),
            name: fb_name,
            with_block: fb_with,
            music: Box::new(figure_body),
        };
        staff_exprs.push(fb_expr);
    }

    for (i, layers) in staff_music.into_iter().enumerate() {
        let mut inner = build_layers_music(layers);
        // Apply lyrics wrapping per-staff
        if let Some(Some(info)) = lyrics_infos.get(i)
            && let Some(raw) = staff_layer_children.get(i).and_then(|v| v.first())
        {
            inner = lyrics::wrap_music_with_lyrics(inner, raw, info);
        }
        let meta = staff_metas.get(i);

        let with_block = meta
            .and_then(|m| m.with_block_str.as_deref())
            .and_then(parse_with_block_str);

        let ctx_type = meta
            .map(|m| m.context_type.clone())
            .unwrap_or_else(|| "Staff".to_string());

        // DrumStaff: wrap music in \drummode
        if ctx_type == "DrumStaff" {
            inner = Music::DrumMode {
                body: Box::new(inner),
            };
        }

        let staff_music_expr = Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: ctx_type,
            name: meta.and_then(|m| m.name.clone()),
            with_block,
            music: Box::new(inner),
        };
        staff_exprs.push(staff_music_expr);
    }

    // Wrap in simultaneous if multiple staves
    let inner = if staff_exprs.len() == 1 {
        staff_exprs.into_iter().next().unwrap()
    } else {
        Music::Simultaneous(staff_exprs)
    };

    // Wrap in group context if present
    if let Some(group) = group_meta {
        let with_block = group
            .with_block_str
            .as_deref()
            .and_then(parse_with_block_str);

        Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: group.context_type.clone(),
            name: group.name.clone(),
            with_block,
            music: Box::new(inner),
        }
    } else {
        inner
    }
}

/// Build flat music (no context wrappers) from staff/layer structure.
fn build_flat_music(staff_music: Vec<Vec<Vec<Music>>>) -> Music {
    let mut all_layers: Vec<Vec<Music>> = Vec::new();
    for layers in staff_music {
        all_layers.extend(layers);
    }
    build_layers_music(all_layers)
}

/// Build music from a set of layers (voices).
fn build_layers_music(layers: Vec<Vec<Music>>) -> Music {
    let non_empty: Vec<Vec<Music>> = layers.into_iter().filter(|l| !l.is_empty()).collect();

    match non_empty.len() {
        0 => Music::Sequential(Vec::new()),
        1 => Music::Sequential(non_empty.into_iter().next().unwrap()),
        _ => {
            let voices: Vec<Music> = non_empty.into_iter().map(Music::Sequential).collect();
            Music::Simultaneous(voices)
        }
    }
}

// ---------------------------------------------------------------------------
// Tuplet span handling
// ---------------------------------------------------------------------------

/// Collected tuplet span info from measure control events.
struct TupletSpanInfo {
    start_id: String,
    end_id: String,
    numerator: u32,
    denominator: u32,
    span_duration: Option<Duration>,
}

/// Collect TupletSpan control events from measure children.
fn collect_tuplet_spans(measure_children: &[MeasureChild]) -> Vec<TupletSpanInfo> {
    let mut spans = Vec::new();
    for mc in measure_children {
        if let MeasureChild::TupletSpan(ts) = mc {
            let start_id = ts
                .tuplet_span_log
                .startid
                .as_ref()
                .map(|u| u.0.trim_start_matches('#').to_string())
                .unwrap_or_default();
            let end_id = ts
                .tuplet_span_log
                .endid
                .as_ref()
                .map(|u| u.0.trim_start_matches('#').to_string())
                .unwrap_or_default();
            let numerator: u32 = ts
                .tuplet_span_log
                .num
                .as_deref()
                .and_then(|n| n.parse().ok())
                .unwrap_or(3);
            let denominator: u32 = ts
                .tuplet_span_log
                .numbase
                .as_deref()
                .and_then(|n| n.parse().ok())
                .unwrap_or(2);

            // Parse span_duration from label if present
            let span_duration = ts
                .common
                .label
                .as_deref()
                .and_then(parse_tuplet_span_duration);

            spans.push(TupletSpanInfo {
                start_id,
                end_id,
                numerator,
                denominator,
                span_duration,
            });
        }
    }
    spans
}

/// Parse span duration from a tuplet label.
///
/// Label format: `tusk:tuplet,{JSON}` with TupletInfo struct.
fn parse_tuplet_span_duration(label: &str) -> Option<Duration> {
    let json = label.strip_prefix("tusk:tuplet,")?;
    let info: tusk_model::TupletInfo = serde_json::from_str(json).ok()?;
    let dur_info = info.span_duration?;
    Some(Duration {
        base: dur_info.base,
        dots: dur_info.dots,
        multipliers: dur_info.multipliers,
    })
}

/// Collect xml:ids from a LayerChild in the same order as convert_layer_child_to_items
/// produces items.
fn collect_layer_child_ids(child: &LayerChild, ids: &mut Vec<Option<String>>, count: usize) {
    match child {
        LayerChild::Beam(beam) => {
            for bc in &beam.children {
                ids.push(beam_child_xml_id(bc).map(String::from));
            }
        }
        _ => {
            if count > 0 {
                ids.push(layer_child_xml_id(child).map(String::from));
            }
        }
    }
}

/// Apply tuplet wrapping to a list of Music items using tuplet span info.
///
/// For each tuplet span, finds the start and end indices in the items list
/// by matching xml:ids, then replaces that range with a Music::Tuplet wrapper.
/// Processes tuplets from innermost to outermost (sorted by range size, ascending).
fn apply_tuplet_wrapping(
    items: &mut Vec<Music>,
    item_ids: &[Option<String>],
    tuplet_spans: &[TupletSpanInfo],
) {
    if tuplet_spans.is_empty() || items.is_empty() {
        return;
    }

    // Build (start_idx, end_idx, span_info_idx) for each tuplet span
    // Indices are into the original item_ids list.
    let mut ranges: Vec<(usize, usize, usize)> = Vec::new();
    for (si, span) in tuplet_spans.iter().enumerate() {
        let start_idx = item_ids
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == span.start_id));
        let end_idx = item_ids
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == span.end_id));
        if let (Some(si_idx), Some(ei_idx)) = (start_idx, end_idx)
            && si_idx <= ei_idx
        {
            ranges.push((si_idx, ei_idx, si));
        }
    }

    // Sort by range size (smallest first = innermost first) for correct nesting.
    // For equal sizes, process later positions first to avoid index shifting issues.
    ranges.sort_by(|a, b| {
        let size_a = a.1 - a.0;
        let size_b = b.1 - b.0;
        size_a.cmp(&size_b).then(b.0.cmp(&a.0))
    });

    // Maintain a mutable id list that gets updated as items are wrapped.
    // When a range [s..=e] is wrapped, the ids collapse to a single entry
    // retaining the start_id (so outer tuplets referencing the same note can find it).
    let mut ids: Vec<Option<String>> = item_ids.to_vec();

    for &(_orig_start, _orig_end, span_idx) in &ranges {
        let span = &tuplet_spans[span_idx];

        // Find current positions in the (possibly modified) ids list
        let cur_start = ids
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == span.start_id));
        let cur_end = ids
            .iter()
            .rposition(|id| id.as_deref().is_some_and(|i| i == span.end_id));

        if let (Some(cs), Some(ce)) = (cur_start, cur_end)
            && cs <= ce
            && ce < items.len()
        {
            let start_id = ids[cs].clone();

            // Extract items in range and wrap in Tuplet
            let body_items: Vec<Music> = items.drain(cs..=ce).collect();
            let tuplet = Music::Tuplet {
                numerator: span.numerator,
                denominator: span.denominator,
                span_duration: span.span_duration.clone(),
                body: Box::new(Music::Sequential(body_items)),
            };
            items.insert(cs, tuplet);

            // Replace range of ids with single entry preserving start_id
            ids.drain(cs..=ce);
            ids.insert(cs, start_id);
        }
    }
}

mod grace;
use grace::{apply_grace_wrapping, collect_grace_types};

use repeats::{apply_repeat_wrapping, collect_ending_spans, collect_repeat_spans};

mod chord_names;
mod figured_bass;
use chord_names::{ChordNamesMeta, collect_chord_mode_harms, extract_chord_names_meta};
use figured_bass::{FiguredBassMeta, collect_figure_mode_fbs, extract_figured_bass_meta};

mod operations;
use operations::{
    collect_function_ops, collect_property_ops, inject_function_ops, inject_property_ops,
};

/// Find the Score element in the MEI hierarchy.
fn find_score(mei: &Mei) -> Option<&tusk_model::elements::Score> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child
            && let Some(tusk_model::elements::MusicChild::Body(body)) = music.children.first()
            && let Some(tusk_model::elements::BodyChild::Mdiv(mdiv)) = body.children.first()
            && let Some(tusk_model::elements::MdivChild::Score(score)) = mdiv.children.first()
        {
            return Some(score);
        }
    }
    None
}

/// Convert a LayerChild to Music items, handling Beam containers by flattening
/// their children and adding BeamStart/BeamEnd post-events.
fn convert_layer_child_to_items(
    child: &LayerChild,
    slur_map: &HashMap<String, Vec<PostEvent>>,
    items: &mut Vec<Music>,
) {
    match child {
        LayerChild::Beam(beam) => {
            let count = beam.children.len();
            for (i, bc) in beam.children.iter().enumerate() {
                if let Some(mut m) = convert_beam_child(bc) {
                    // Apply slur post-events by xml:id
                    if let Some(id) = beam_child_xml_id(bc)
                        && let Some(events) = slur_map.get(id)
                    {
                        append_post_events(&mut m, events);
                    }
                    // First child gets BeamStart, last gets BeamEnd
                    if i == 0 {
                        append_post_events(&mut m, &[PostEvent::BeamStart]);
                    }
                    if i == count - 1 {
                        append_post_events(&mut m, &[PostEvent::BeamEnd]);
                    }
                    items.push(m);
                }
            }
        }
        _ => {
            // Inject \change before notes with tusk:context-change label
            if let Some(change) = extract_context_change_from_label(child) {
                items.push(change);
            }
            if let Some(mut m) = convert_layer_child(child) {
                if let Some(id) = layer_child_xml_id(child)
                    && let Some(events) = slur_map.get(id)
                {
                    append_post_events(&mut m, events);
                }
                items.push(m);
            }
        }
    }
}

/// Extract a `\change` context change from a LayerChild's label.
///
/// Looks for `tusk:context-change,{json}` label segment and returns a
/// `Music::ContextChange` if found.
fn extract_context_change_from_label(child: &LayerChild) -> Option<Music> {
    let label = match child {
        LayerChild::Note(n) => n.common.label.as_deref()?,
        LayerChild::Rest(r) => r.common.label.as_deref()?,
        LayerChild::Chord(c) => c.common.label.as_deref()?,
        _ => return None,
    };
    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix("tusk:context-change,")
            && let Ok(cc) = serde_json::from_str::<tusk_model::ContextChange>(json)
        {
            return Some(Music::ContextChange {
                context_type: cc.context_type,
                name: cc.name,
            });
        }
    }
    None
}

/// Convert a single MEI LayerChild to a LilyPond Music expression.
fn convert_layer_child(child: &LayerChild) -> Option<Music> {
    match child {
        LayerChild::Note(note) => Some(convert_mei_note(note)),
        LayerChild::Rest(rest) => Some(convert_mei_rest(rest)),
        LayerChild::MRest(mrest) => Some(convert_mei_mrest(mrest)),
        LayerChild::Chord(chord) => Some(convert_mei_chord(chord)),
        LayerChild::BTrem(btrem) => Some(convert_mei_btrem(btrem)),
        _ => None,
    }
}

/// Convert a BeamChild to a LilyPond Music expression.
fn convert_beam_child(child: &tusk_model::elements::BeamChild) -> Option<Music> {
    use tusk_model::elements::BeamChild;
    match child {
        BeamChild::Note(note) => Some(convert_mei_note(note)),
        BeamChild::Rest(rest) => Some(convert_mei_rest(rest)),
        BeamChild::Chord(chord) => Some(convert_mei_chord(chord)),
        BeamChild::Beam(beam) => {
            // Nested beams: flatten recursively (nested beams just continue the beam)
            // This shouldn't produce beam markers for the inner beam since
            // LilyPond uses a flat [ ... ] without nesting
            let mut nested = Vec::new();
            for bc in &beam.children {
                if let Some(m) = convert_beam_child(bc) {
                    nested.push(m);
                }
            }
            // Return first item if single, otherwise none (shouldn't occur in practice)
            if nested.len() == 1 {
                nested.into_iter().next()
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get xml:id from a LayerChild.
fn layer_child_xml_id(child: &LayerChild) -> Option<&str> {
    match child {
        LayerChild::Note(note) => note.common.xml_id.as_deref(),
        LayerChild::Rest(rest) => rest.common.xml_id.as_deref(),
        LayerChild::MRest(mrest) => mrest.common.xml_id.as_deref(),
        LayerChild::Chord(chord) => chord.common.xml_id.as_deref(),
        LayerChild::BTrem(btrem) => btrem_inner_xml_id(btrem),
        _ => None,
    }
}

/// Get xml:id of the inner note/chord inside a BTrem.
fn btrem_inner_xml_id(btrem: &tusk_model::elements::BTrem) -> Option<&str> {
    btrem.children.first().and_then(|child| match child {
        tusk_model::elements::BTremChild::Note(n) => n.common.xml_id.as_deref(),
        tusk_model::elements::BTremChild::Chord(c) => c.common.xml_id.as_deref(),
    })
}

/// Get xml:id from a BeamChild.
fn beam_child_xml_id(child: &tusk_model::elements::BeamChild) -> Option<&str> {
    use tusk_model::elements::BeamChild;
    match child {
        BeamChild::Note(note) => note.common.xml_id.as_deref(),
        BeamChild::Rest(rest) => rest.common.xml_id.as_deref(),
        BeamChild::Chord(chord) => chord.common.xml_id.as_deref(),
        BeamChild::Beam(beam) => beam.common.xml_id.as_deref(),
        _ => None,
    }
}

/// Collect slur/phrase control events from measure children into a map of
/// note xml:id → PostEvent list.
fn collect_slur_post_events(measure_children: &[MeasureChild]) -> HashMap<String, Vec<PostEvent>> {
    let mut map: HashMap<String, Vec<PostEvent>> = HashMap::new();

    for mc in measure_children {
        if let MeasureChild::Slur(slur) = mc {
            let is_phrase = slur
                .common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("tusk:phrase,"));

            if let Some(ref startid) = slur.slur_log.startid {
                let id = startid.0.trim_start_matches('#').to_string();
                let event = if is_phrase {
                    PostEvent::PhrasingSlurStart
                } else {
                    PostEvent::SlurStart
                };
                map.entry(id).or_default().push(event);
            }

            if let Some(ref endid) = slur.slur_log.endid {
                let id = endid.0.trim_start_matches('#').to_string();
                let event = if is_phrase {
                    PostEvent::PhrasingSlurEnd
                } else {
                    PostEvent::SlurEnd
                };
                map.entry(id).or_default().push(event);
            }
        }
    }

    map
}

/// Collect dynamic control events from measure children into the post-event map.
///
/// Each `<dynam>` with a `@startid` is mapped to a `PostEvent::Dynamic(text)` on
/// the referenced note.
fn collect_dynam_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
) {
    for mc in measure_children {
        if let MeasureChild::Dynam(dynam) = mc {
            // Get the text content
            let text = dynam
                .children
                .iter()
                .map(|c| {
                    let tusk_model::elements::DynamChild::Text(t) = c;
                    t.clone()
                })
                .next()
                .unwrap_or_default();

            if let Some(ref startid) = dynam.dynam_log.startid {
                let id = startid.0.trim_start_matches('#').to_string();
                map.entry(id).or_default().push(PostEvent::Dynamic(text));
            }
        }
    }
}

/// Collect hairpin control events from measure children into the post-event map.
///
/// Each `<hairpin>` with `@startid`/`@endid` is mapped to `Crescendo`/`Decrescendo`
/// on the start note and `HairpinEnd` on the end note.
fn collect_hairpin_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
) {
    for mc in measure_children {
        if let MeasureChild::Hairpin(hairpin) = mc {
            let form = hairpin.hairpin_log.form.as_deref().unwrap_or("");

            if let Some(ref startid) = hairpin.hairpin_log.startid {
                let id = startid.0.trim_start_matches('#').to_string();
                let event = if form == "dim" {
                    PostEvent::Decrescendo
                } else {
                    PostEvent::Crescendo
                };
                map.entry(id).or_default().push(event);
            }

            if let Some(ref endid) = hairpin.hairpin_log.endid {
                let id = endid.0.trim_start_matches('#').to_string();
                map.entry(id).or_default().push(PostEvent::HairpinEnd);
            }
        }
    }
}

/// Collect articulation/fingering/string-number control events from `<dir>` elements
/// with `tusk:artic,{JSON}` labels.
fn collect_artic_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
) {
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let label = match dir.common.label.as_deref() {
                Some(l) => l,
                None => continue,
            };
            let startid = match dir.dir_log.startid.as_ref() {
                Some(s) => s.0.trim_start_matches('#').to_string(),
                None => continue,
            };

            if let Some(json) = label.strip_prefix("tusk:artic,")
                && let Ok(info) = serde_json::from_str::<tusk_model::ArticulationInfo>(json)
                && let Some(pe) = artic_info_to_post_event(&info)
            {
                map.entry(startid).or_default().push(pe);
            }
        }
    }
}

/// Convert an ArticulationInfo to the appropriate PostEvent.
fn artic_info_to_post_event(info: &tusk_model::ArticulationInfo) -> Option<PostEvent> {
    let dir = direction_ext_to_ly(info.direction);
    match info.kind {
        tusk_model::ArticulationKind::Articulation => {
            if let Some(script) = name_to_script_abbreviation(&info.value) {
                Some(PostEvent::Articulation {
                    direction: dir,
                    script,
                })
            } else {
                Some(PostEvent::NamedArticulation {
                    direction: dir,
                    name: info.value.clone(),
                })
            }
        }
        tusk_model::ArticulationKind::Fingering => {
            let digit: u8 = info.value.parse().ok()?;
            Some(PostEvent::Fingering {
                direction: dir,
                digit,
            })
        }
        tusk_model::ArticulationKind::StringNumber => {
            let number: u8 = info.value.parse().ok()?;
            Some(PostEvent::StringNumber {
                direction: dir,
                number,
            })
        }
    }
}

/// Collect ornament control events (trill, mordent, turn, fermata, ornam) from
/// measure children into the post-event map.
fn collect_ornament_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
) {
    for mc in measure_children {
        match mc {
            MeasureChild::Trill(trill) => {
                if let Some(ref startid) = trill.trill_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let (name, direction) =
                        parse_ornament_label(trill.common.label.as_deref(), "trill");
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            MeasureChild::Mordent(mordent) => {
                if let Some(ref startid) = mordent.mordent_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let fallback = match mordent.mordent_log.form.as_deref() {
                        Some("upper") => "prall",
                        _ => "mordent",
                    };
                    let (name, direction) =
                        parse_ornament_label(mordent.common.label.as_deref(), fallback);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            MeasureChild::Turn(turn) => {
                if let Some(ref startid) = turn.turn_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let fallback = match turn.turn_log.form.as_deref() {
                        Some("lower") => "reverseturn",
                        _ => "turn",
                    };
                    let (name, direction) =
                        parse_ornament_label(turn.common.label.as_deref(), fallback);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            MeasureChild::Fermata(fermata) => {
                if let Some(ref startid) = fermata.fermata_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let (name, direction) =
                        parse_ornament_label(fermata.common.label.as_deref(), "fermata");
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            MeasureChild::Ornam(ornam) => {
                if let Some(ref startid) = ornam.ornam_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let fallback_name = ornam
                        .children
                        .first()
                        .map(|c| {
                            let tusk_model::elements::OrnamChild::Text(t) = c;
                            t.clone()
                        })
                        .unwrap_or_else(|| "ornam".to_string());
                    let (name, direction) =
                        parse_ornament_label(ornam.common.label.as_deref(), &fallback_name);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            _ => {}
        }
    }
}

/// Parse name and direction from a typed ornament label.
fn parse_ornament_label(label: Option<&str>, fallback_name: &str) -> (String, Direction) {
    if let Some(json) = label.and_then(|l| l.strip_prefix("tusk:ornament,"))
        && let Ok(info) = serde_json::from_str::<tusk_model::OrnamentInfo>(json)
    {
        let dir = direction_ext_to_ly(info.direction);
        return (info.name, dir);
    }
    (fallback_name.to_string(), Direction::Neutral)
}

/// Convert an extension DirectionExt to a LilyPond Direction.
fn direction_ext_to_ly(dir: Option<tusk_model::DirectionExt>) -> Direction {
    match dir {
        Some(tusk_model::DirectionExt::Up) => Direction::Up,
        Some(tusk_model::DirectionExt::Down) => Direction::Down,
        None => Direction::Neutral,
    }
}

/// Convert an MEI BTrem (bowed tremolo) to a LilyPond Music expression.
///
/// Extracts the inner note/chord and adds a `PostEvent::Tremolo` with the
/// subdivision value restored from the label.
fn convert_mei_btrem(btrem: &tusk_model::elements::BTrem) -> Music {
    // Restore subdivision value from typed JSON label
    let value = btrem
        .common
        .label
        .as_deref()
        .and_then(|l| l.strip_prefix("tusk:tremolo,"))
        .and_then(|json| serde_json::from_str::<tusk_model::TremoloInfo>(json).ok())
        .map(|info| info.value)
        .unwrap_or_else(|| {
            // Fallback: compute from @num (slash count)
            let num: u32 = btrem
                .b_trem_log
                .num
                .as_deref()
                .and_then(|n| n.parse().ok())
                .unwrap_or(0);
            if num == 0 {
                0
            } else {
                // Reverse of trailing_zeros - 2: value = 2^(num+2)
                1 << (num + 2)
            }
        });

    let mut music = btrem
        .children
        .first()
        .map(|child| match child {
            tusk_model::elements::BTremChild::Note(n) => convert_mei_note(n),
            tusk_model::elements::BTremChild::Chord(c) => convert_mei_chord(c),
        })
        .unwrap_or_else(|| {
            Music::Rest(crate::model::RestEvent {
                duration: None,
                post_events: vec![],
            })
        });

    // Append tremolo post-event
    append_post_events(&mut music, &[PostEvent::Tremolo(value)]);
    music
}

/// Map a LilyPond articulation name to its ScriptAbbreviation, if one exists.
fn name_to_script_abbreviation(name: &str) -> Option<ScriptAbbreviation> {
    match name {
        "staccato" => Some(ScriptAbbreviation::Dot),
        "tenuto" => Some(ScriptAbbreviation::Dash),
        "accent" => Some(ScriptAbbreviation::Accent),
        "marcato" => Some(ScriptAbbreviation::Marcato),
        "stopped" => Some(ScriptAbbreviation::Stopped),
        "staccatissimo" => Some(ScriptAbbreviation::Staccatissimo),
        "portato" => Some(ScriptAbbreviation::Portato),
        _ => None,
    }
}

/// Append post-events to a Music item's post_events list.
fn append_post_events(music: &mut Music, events: &[PostEvent]) {
    match music {
        Music::Note(note) => note.post_events.extend(events.iter().cloned()),
        Music::Rest(rest) => rest.post_events.extend(events.iter().cloned()),
        Music::Chord(chord) => chord.post_events.extend(events.iter().cloned()),
        Music::ChordRepetition(cr) => cr.post_events.extend(events.iter().cloned()),
        Music::MultiMeasureRest(mrest) => mrest.post_events.extend(events.iter().cloned()),
        _ => {}
    }
}

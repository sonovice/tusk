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
mod tests_skip;
#[cfg(test)]
mod tests_tempo_marks;
#[cfg(test)]
mod tests_toplevel_markup;
#[cfg(test)]
mod tests_variables;

use std::collections::HashMap;
use thiserror::Error;
use tusk_model::elements::{
    LayerChild, MeasureChild, Mei, MeiChild, ScoreChild, ScoreDefChild, SectionChild, StaffGrpChild,
};
use tusk_model::extensions::ExtensionStore;
use tusk_model::{ToplevelMarkup, ToplevelMarkupKind};

use crate::model::Duration;
use crate::model::note::{Direction, MultiMeasureRestEvent, PostEvent, ScriptAbbreviation};
use crate::model::property::{PropertyPath, PropertyValue};
use crate::model::scheme::SchemeExpr;
use crate::model::{
    ContextKeyword, LilyPondFile, Music, ScoreBlock, ScoreItem, ToplevelExpression, Version,
};

use conversion::{
    convert_mei_chord, convert_mei_mrest, convert_mei_note, convert_mei_rest, convert_mei_space,
};
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
pub fn export(mei: &Mei, ext_store: &ExtensionStore) -> Result<LilyPondFile, ExportError> {
    // Check for book-structured multi-mdiv layout
    if let Some(entries) = book::find_book_entries(mei, ext_store) {
        return export_book(mei, &entries, ext_store);
    }

    // Single-score path (backward compatible)
    let score = find_score(mei).ok_or(ExportError::NoMusic)?;
    let score_block = export_single_score(score, ext_store);

    // Build non-markup top-level items in natural order
    let mut non_markup_items = Vec::new();

    let assignments = extract_assignments(score, ext_store);
    for a in assignments {
        non_markup_items.push(ToplevelExpression::Assignment(a));
    }

    let (top_header, top_paper, top_layout, top_midi) = output_defs::extract_toplevel_blocks(mei, ext_store);
    if let Some(hb) = top_header {
        non_markup_items.push(ToplevelExpression::Header(hb));
    }
    if let Some(pb) = top_paper {
        non_markup_items.push(ToplevelExpression::Paper(pb));
    }
    if let Some(lb) = top_layout {
        non_markup_items.push(ToplevelExpression::Layout(lb));
    }
    if let Some(mb) = top_midi {
        non_markup_items.push(ToplevelExpression::Midi(mb));
    }

    non_markup_items.push(ToplevelExpression::Score(score_block));

    // Merge with top-level markups at their original positions
    let markup_items = extract_toplevel_markups(score, ext_store);
    let items = merge_items_with_markups(non_markup_items, markup_items);

    Ok(LilyPondFile {
        version: Some(Version {
            version: "2.24.0".to_string(),
        }),
        items,
    })
}

/// Export a book-structured MEI (multiple mdivs with BookStructure labels).
fn export_book(mei: &Mei, entries: &[book::MdivEntry<'_>], ext_store: &ExtensionStore) -> Result<LilyPondFile, ExportError> {
    let mut non_markup_items = Vec::new();

    // Extract top-level assignments from first score's scoreDef
    let markup_items = if let Some(first) = entries.first() {
        let assignments = extract_assignments(first.score, ext_store);
        for a in assignments {
            non_markup_items.push(ToplevelExpression::Assignment(a));
        }
        extract_toplevel_markups(first.score, ext_store)
    } else {
        Vec::new()
    };

    // Extract top-level blocks from MeiHead ExtMeta
    let (top_header, top_paper, top_layout, top_midi) = output_defs::extract_toplevel_blocks(mei, ext_store);
    if let Some(hb) = top_header {
        non_markup_items.push(ToplevelExpression::Header(hb));
    }
    if let Some(pb) = top_paper {
        non_markup_items.push(ToplevelExpression::Paper(pb));
    }
    if let Some(lb) = top_layout {
        non_markup_items.push(ToplevelExpression::Layout(lb));
    }
    if let Some(mb) = top_midi {
        non_markup_items.push(ToplevelExpression::Midi(mb));
    }

    // Reconstruct book/bookpart hierarchy
    let book_items = book::reconstruct_books(entries, &|score| export_single_score(score, ext_store));
    non_markup_items.extend(book_items);

    // Merge with top-level markups at their original positions
    let items = merge_items_with_markups(non_markup_items, markup_items);

    Ok(LilyPondFile {
        version: Some(Version {
            version: "2.24.0".to_string(),
        }),
        items,
    })
}

/// Export a single MEI Score to a LilyPond ScoreBlock.
fn export_single_score(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> ScoreBlock {
    let group_meta = extract_group_meta(score, ext_store);
    let staff_metas = extract_staff_metas(score, ext_store);
    let event_sequences = extract_event_sequences(score, ext_store);
    let pitch_contexts = extract_pitch_contexts(score, ext_store);
    let lyrics_infos = extract_lyrics_infos(score, ext_store);

    let mei_defaults = extract_mei_defaults(score);

    // Inject \voiceOne/\voiceTwo in multi-voice measures only for
    // MusicXML-originated content (always has part_details in ext_store).
    // LilyPond-originated content preserves original voice structure.
    let inject_voice_cmds = !ext_store.part_details_map.is_empty();

    // Accumulate music per staff: each staff gets a single flat Vec<Music>
    // containing all measures' content sequentially. Multi-voice measures
    // are wrapped in << >> inline so voice count can vary per measure.
    let mut staff_music: Vec<Vec<Vec<Music>>> = Vec::new();
    let mut staff_layer_children: Vec<Vec<&[LayerChild]>> = Vec::new();
    let mut measure_numbers: Vec<String> = Vec::new();
    let mut staves_initialized = false;

    // Track current time signature for measure-duration spacers.
    // Extract initial time sig from first staff's event sequence.
    let mut current_time_num: Vec<u32> = vec![4];
    let mut current_time_den: u32 = 4;
    if let Some(seq) = event_sequences.first() {
        for ev in seq {
            if let Music::TimeSignature(ts) = &ev.music {
                current_time_num = ts.numerators.clone();
                current_time_den = ts.denominator;
                break;
            }
        }
    }

    // Pre-collect slur/phrase post events across ALL measures. MEI slur control
    // events live in the end measure but may reference start notes in earlier
    // measures. A global map ensures cross-measure slur starts get applied.
    let global_slur_map = {
        let mut map: HashMap<String, Vec<PostEvent>> = HashMap::new();
        for child in &score.children {
            if let ScoreChild::Section(section) = child {
                for sc in &section.children {
                    if let SectionChild::Measure(m) = sc {
                        let m_map = collect_slur_post_events(&m.children, ext_store);
                        for (k, v) in m_map {
                            map.entry(k).or_default().extend(v);
                        }
                    }
                }
            }
        }
        map
    };

    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for section_child in &section.children {
                if let SectionChild::Measure(measure) = section_child {
                    let measure_n = measure
                        .common
                        .n
                        .as_ref()
                        .map(|n| n.0.clone())
                        .unwrap_or_else(|| (measure_numbers.len() + 1).to_string());
                    measure_numbers.push(measure_n.clone());

                    let mut post_event_map = global_slur_map.clone();
                    collect_dynam_post_events(&measure.children, &mut post_event_map);
                    collect_hairpin_post_events(&measure.children, &mut post_event_map);
                    collect_artic_post_events(&measure.children, &mut post_event_map, ext_store);
                    collect_ornament_post_events(&measure.children, &mut post_event_map, ext_store);
                    collect_text_script_post_events(&measure.children, &mut post_event_map, ext_store);

                    let property_ops = collect_property_ops(&measure.children, ext_store);
                    let function_ops = collect_function_ops(&measure.children, ext_store);
                    let scheme_music_ops = collect_scheme_music_ops(&measure.children, ext_store);
                    let tuplet_spans = collect_tuplet_spans(&measure.children, ext_store);
                    let repeat_spans = collect_repeat_spans(&measure.children, ext_store);
                    let ending_spans = collect_ending_spans(&measure.children, ext_store);

                    // Build all staves' layers for this measure
                    let mut all_staves: Vec<(Vec<Vec<Music>>, Vec<&[LayerChild]>)> = Vec::new();
                    for mc in &measure.children {
                        if let MeasureChild::Staff(staff) = mc {
                            let mut layers: Vec<Vec<Music>> = Vec::new();
                            let mut raw_layers: Vec<&[LayerChild]> = Vec::new();
                            for sc in &staff.children {
                                let tusk_model::elements::StaffChild::Layer(layer) = sc;
                                raw_layers.push(&layer.children);
                                let grace_types = collect_grace_types(&layer.children, ext_store);
                                let mut items = Vec::new();
                                let mut item_ids = Vec::new();
                                for lc in &layer.children {
                                    let start = items.len();
                                    convert_layer_child_to_items(lc, &post_event_map, &mut items, ext_store, &mei_defaults);
                                    collect_layer_child_ids(lc, &mut item_ids, items.len() - start);
                                }
                                let log1 = inject_property_ops(&mut items, &mut item_ids, &property_ops);
                                let log2 = inject_function_ops(&mut items, &mut item_ids, &function_ops);
                                let log3 = inject_scheme_music_ops(&mut items, &mut item_ids, &scheme_music_ops);
                                let mut grace_types = grace_types;
                                apply_insertion_log(&mut grace_types, &log1);
                                apply_insertion_log(&mut grace_types, &log2);
                                apply_insertion_log(&mut grace_types, &log3);
                                apply_tuplet_wrapping(&mut items, &mut item_ids, &tuplet_spans, &mut grace_types);
                                apply_grace_wrapping(&mut items, &grace_types);
                                apply_repeat_wrapping(
                                    &mut items,
                                    &item_ids,
                                    &repeat_spans,
                                    &ending_spans,
                                );
                                layers.push(items);
                            }
                            all_staves.push((layers, raw_layers));
                        }
                    }

                    // Detect time signature changes from first staff's first layer
                    if let Some((first_layers, _)) = all_staves.first() {
                        if let Some(first_layer) = first_layers.first() {
                            if let Some((num, den)) = extract_time_from_items(first_layer) {
                                current_time_num = num;
                                current_time_den = den;
                            }
                        }
                    }

                    // Build spacer for MusicXML content to enforce measure duration
                    let spacer = if inject_voice_cmds {
                        Some(make_measure_spacer(&current_time_num, current_time_den))
                    } else {
                        None
                    };

                    // Accumulate into staff_music
                    let num_staves = all_staves.len();
                    let measure_comment = Music::LineComment(format!("m.{}", measure_n));
                    for (staff_idx, (layers, raw_layers)) in all_staves.into_iter().enumerate() {
                        if !staves_initialized {
                            if let Some(seq) = event_sequences.get(staff_idx) {
                                let mut first_layers = layers;
                                if let Some(first_layer) = first_layers.first_mut() {
                                    inject_signature_events(first_layer, seq);
                                }
                                let mut measure_items = flatten_measure_layers(first_layers, inject_voice_cmds, spacer.clone());
                                measure_items.insert(0, measure_comment.clone());
                                staff_music.push(vec![measure_items]);
                            } else {
                                let mut measure_items = flatten_measure_layers(layers, inject_voice_cmds, spacer.clone());
                                measure_items.insert(0, measure_comment.clone());
                                staff_music.push(vec![measure_items]);
                            }
                            // Only first layer — \addlyrics aligns with first voice in << >>
                            staff_layer_children.push(raw_layers.into_iter().take(1).collect());
                        } else {
                            let mut measure_items = flatten_measure_layers(layers, inject_voice_cmds, spacer.clone());
                            // Reset measure position before \time changes to
                            // prevent "mid-measure time signature" warnings
                            // from tuplet timing rounding errors.
                            if inject_voice_cmds {
                                insert_timing_reset_before_time_change(&mut measure_items);
                            }
                            let stream = &mut staff_music[staff_idx][0];
                            if inject_voice_cmds {
                                // Explicit barline resets timing counter —
                                // prevents cascading barline misplacement
                                // when source has irregular measure durations.
                                stream.push(Music::BarLine {
                                    bar_type: "|".to_string(),
                                });
                            } else {
                                stream.push(Music::BarCheck);
                            }
                            stream.push(measure_comment.clone());
                            stream.extend(measure_items);
                            // Only first layer — \addlyrics aligns with first voice
                            staff_layer_children[staff_idx].extend(raw_layers.into_iter().take(1));
                        }
                    }
                    if !staves_initialized && num_staves > 0 {
                        staves_initialized = true;
                    }
                }
            }
        }
    }

    let chord_mode_events = collect_chord_mode_harms(score, ext_store);
    let chord_names_meta = extract_chord_names_meta(score, ext_store);
    let figure_mode_events = collect_figure_mode_fbs(score, ext_store);
    let figured_bass_meta = extract_figured_bass_meta(score, ext_store);

    // For MusicXML-originated content, lyrics_infos may be all-None because
    // ext_store doesn't have lyrics metadata. Detect lyrics from layer children
    // and assign to the staff with the most lyric content (typically the melody staff).
    let mut lyrics_infos = lyrics_infos;
    {
        let mut best_idx = None;
        let mut best_count = 0usize;
        let mut best_ids = Vec::new();
        for (i, info) in lyrics_infos.iter().enumerate() {
            if info.is_none() {
                if let Some(layer_children) = staff_layer_children.get(i) {
                    let verse_ids = detect_verse_ids(layer_children);
                    if !verse_ids.is_empty() {
                        let count = count_lyric_notes(layer_children, &verse_ids[0]);
                        if count > best_count {
                            best_count = count;
                            best_idx = Some(i);
                            best_ids = verse_ids;
                        }
                    }
                }
            }
        }
        if let Some(idx) = best_idx {
            lyrics_infos[idx] = Some(lyrics::LyricsExportInfo {
                style: lyrics::LyricsExportStyle::AddLyrics { verse_ids: best_ids },
            });
        }
    }

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
        ext_store,
        &measure_numbers,
    );

    let mut score_items = vec![ScoreItem::Music(music)];
    let score_blocks = output_defs::extract_score_blocks(score, ext_store);
    score_items.extend(score_blocks);

    ScoreBlock { items: score_items }
}

/// Extract stored variable assignments from scoreDef.
///
/// Looks up typed `VariableAssignments` in the extension store by the
/// scoreDef's xml:id, then re-parses the serialized assignment values
/// through the LilyPond parser.
fn extract_assignments(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Vec<crate::model::Assignment> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child
            && let Some(id) = score_def.common.xml_id.as_deref()
                && let Some(vars) = ext_store.variable_assignments(id) {
                    return vars
                        .assignments
                        .iter()
                        .cloned()
                        .filter_map(ext_assignment_to_model)
                        .collect();
                }
    }
    Vec::new()
}

/// Extract stored top-level markup/markuplist entries from scoreDef.
///
/// Looks up typed `Vec<ToplevelMarkup>` in the extension store by the
/// scoreDef's xml:id, then re-parses each serialized form through the
/// LilyPond parser.
fn extract_toplevel_markups(
    score: &tusk_model::elements::Score,
    ext_store: &ExtensionStore,
) -> Vec<(usize, ToplevelExpression)> {
    let markups = extract_raw_toplevel_markups(score, ext_store);
    let mut result = Vec::new();
    for m in markups {
        if let Some(expr) = toplevel_markup_to_expr(&m) {
            result.push((m.position, expr));
        }
    }
    result
}

/// Read raw ToplevelMarkup vec from ext_store via scoreDef xml:id.
fn extract_raw_toplevel_markups(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Vec<ToplevelMarkup> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child
            && let Some(id) = score_def.common.xml_id.as_deref()
                && let Some(markups) = ext_store.toplevel_markups(id) {
                    return markups.clone();
                }
    }
    Vec::new()
}

/// Convert a ToplevelMarkup entry to a ToplevelExpression by re-parsing.
fn toplevel_markup_to_expr(m: &ToplevelMarkup) -> Option<ToplevelExpression> {
    use crate::parser::Parser;
    match &m.kind {
        ToplevelMarkupKind::Markup(serialized) => {
            let src = format!("\\markup {serialized}");
            let file = Parser::new(&src).ok()?.parse().ok()?;
            file.items.into_iter().find_map(|item| {
                if let ToplevelExpression::Markup(mk) = item {
                    Some(ToplevelExpression::Markup(mk))
                } else {
                    None
                }
            })
        }
        ToplevelMarkupKind::MarkupList(serialized) => {
            let src = format!("\\markuplist {serialized}");
            let file = Parser::new(&src).ok()?.parse().ok()?;
            file.items.into_iter().find_map(|item| {
                if let ToplevelExpression::MarkupList(ml) = item {
                    Some(ToplevelExpression::MarkupList(ml))
                } else {
                    None
                }
            })
        }
    }
}

/// Merge non-markup items with positioned markup items.
///
/// Markups carry their original 0-based position among all top-level items.
/// Non-markup items fill the remaining slots in their natural order.
fn merge_items_with_markups(
    non_markup: Vec<ToplevelExpression>,
    markups: Vec<(usize, ToplevelExpression)>,
) -> Vec<ToplevelExpression> {
    if markups.is_empty() {
        return non_markup;
    }

    let total = non_markup.len() + markups.len();
    let mut result = Vec::with_capacity(total);

    // Build a set of positions occupied by markups
    let mut markup_map: Vec<(usize, ToplevelExpression)> = markups;
    markup_map.sort_by_key(|(pos, _)| *pos);

    let mut nm_iter = non_markup.into_iter();
    let mut mk_iter = markup_map.into_iter().peekable();

    for pos in 0..total {
        if mk_iter.peek().is_some_and(|(p, _)| *p == pos) {
            let (_, expr) = mk_iter.next().unwrap();
            result.push(expr);
        } else if let Some(item) = nm_iter.next() {
            result.push(item);
        }
    }
    // Append any remaining items (defensive)
    result.extend(nm_iter);
    for (_, expr) in mk_iter {
        result.push(expr);
    }

    result
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

/// Extract lyrics export info from all staffDefs via ext_store.
fn extract_lyrics_infos(
    score: &tusk_model::elements::Score,
    ext_store: &ExtensionStore,
) -> Vec<Option<lyrics::LyricsExportInfo>> {
    let mut infos = Vec::new();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for sdef in collect_staff_defs_from_grp(grp) {
                        let info = sdef.basic.xml_id.as_deref().and_then(|id| {
                            let ext = ext_store.lyrics_info(id)?;
                            lyrics::ext_lyrics_info_to_export(ext)
                        });
                        infos.push(info);
                    }
                }
            }
        }
    }
    infos
}

// ---------------------------------------------------------------------------
// StaffGrp traversal helpers
// ---------------------------------------------------------------------------

/// Detect the maximum verse number across all layer children.
///
/// Scans notes/chords for `<verse>` children and returns the highest verse
/// number found. Used as fallback when ext_store has no lyrics metadata
/// (MusicXML-originated content).
fn detect_verse_ids(layer_children: &[&[LayerChild]]) -> Vec<String> {
    use std::collections::BTreeSet;
    use tusk_model::elements::NoteChild;
    let mut ids = BTreeSet::new();
    for children in layer_children {
        for child in *children {
            let verses = match child {
                LayerChild::Note(n) => &n.children,
                LayerChild::Chord(c) => {
                    if let Some(tusk_model::elements::ChordChild::Note(n)) = c.children.first() {
                        &n.children
                    } else {
                        continue;
                    }
                }
                LayerChild::Beam(beam) => {
                    for bc in &beam.children {
                        let note_children = match bc {
                            tusk_model::elements::BeamChild::Note(n) => &n.children,
                            tusk_model::elements::BeamChild::Chord(c) => {
                                if let Some(tusk_model::elements::ChordChild::Note(n)) = c.children.first() {
                                    &n.children
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        };
                        for nc in note_children {
                            if let NoteChild::Verse(v) = nc {
                                if let Some(n_attr) = v.common.n.as_ref() {
                                    ids.insert(n_attr.0.clone());
                                }
                            }
                        }
                    }
                    continue;
                }
                _ => continue,
            };
            for nc in verses {
                if let NoteChild::Verse(v) = nc {
                    if let Some(n_attr) = v.common.n.as_ref() {
                        ids.insert(n_attr.0.clone());
                    }
                }
            }
        }
    }
    ids.into_iter().collect()
}

/// Count how many notes have a verse with the given ID.
fn count_lyric_notes(layer_children: &[&[LayerChild]], verse_id: &str) -> usize {
    use tusk_model::elements::NoteChild;
    let mut count = 0;
    for children in layer_children {
        for child in *children {
            let has_verse = |note_children: &[NoteChild]| -> bool {
                note_children.iter().any(|nc| {
                    matches!(nc, NoteChild::Verse(v) if v.common.n.as_ref().is_some_and(|n| n.0 == verse_id))
                })
            };
            match child {
                LayerChild::Note(n) => {
                    if has_verse(&n.children) { count += 1; }
                }
                LayerChild::Chord(c) => {
                    if let Some(tusk_model::elements::ChordChild::Note(n)) = c.children.first() {
                        if has_verse(&n.children) { count += 1; }
                    }
                }
                LayerChild::Beam(beam) => {
                    for bc in &beam.children {
                        match bc {
                            tusk_model::elements::BeamChild::Note(n) => {
                                if has_verse(&n.children) { count += 1; }
                            }
                            tusk_model::elements::BeamChild::Chord(c) => {
                                if let Some(tusk_model::elements::ChordChild::Note(n)) = c.children.first() {
                                    if has_verse(&n.children) { count += 1; }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    count
}

/// Recursively collect all staffDefs from a staffGrp, walking into nested staffGrps.
fn collect_staff_defs_from_grp<'a>(grp: &'a tusk_model::elements::StaffGrp) -> Vec<&'a tusk_model::elements::StaffDef> {
    let mut defs = Vec::new();
    for child in &grp.children {
        match child {
            StaffGrpChild::StaffDef(sdef) => defs.push(sdef.as_ref()),
            StaffGrpChild::StaffGrp(sub_grp) => defs.extend(collect_staff_defs_from_grp(sub_grp)),
            _ => {}
        }
    }
    defs
}

/// Find the innermost staffGrp that has a symbol attribute (brace/bracket).
/// This is the group that should become the LilyPond context wrapper (PianoStaff, StaffGroup).
fn find_group_staff_grp(grp: &tusk_model::elements::StaffGrp) -> Option<&tusk_model::elements::StaffGrp> {
    // Check nested staffGrps first (prefer innermost)
    for child in &grp.children {
        if let StaffGrpChild::StaffGrp(sub_grp) = child {
            if let Some(found) = find_group_staff_grp(sub_grp) {
                return Some(found);
            }
        }
    }
    // Then check self
    if grp.staff_grp_vis.symbol.is_some() || grp.common.xml_id.is_some() {
        return Some(grp);
    }
    None
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

/// Extract group metadata from scoreDef's staffGrp via ext_store.
fn extract_group_meta(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Option<GroupMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    let target = find_group_staff_grp(grp).unwrap_or(grp);
                    // Check ext_store for typed group context
                    if let Some(id) = target.common.xml_id.as_deref()
                        && let Some(ctx) = ext_store.staff_context(id) {
                            return Some(GroupMeta {
                                context_type: ctx.context_type.clone(),
                                name: ctx.name.clone(),
                                with_block_str: ctx.with_block.clone(),
                            });
                        }
                    // Fallback: infer from symbol
                    if let Some(symbol) = &target.staff_grp_vis.symbol {
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

/// Extract staff metadata from scoreDef's staffDefs via ext_store.
fn extract_staff_metas(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Vec<StaffMeta> {
    let mut metas = Vec::new();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for sdef in collect_staff_defs_from_grp(grp) {
                        let meta = extract_staff_meta_from_ext(sdef, ext_store);
                        metas.push(meta);
                    }
                }
            }
        }
    }
    metas
}

/// Extract a single staff's metadata from ext_store.
fn extract_staff_meta_from_ext(sdef: &tusk_model::elements::StaffDef, ext_store: &ExtensionStore) -> StaffMeta {
    if let Some(id) = sdef.basic.xml_id.as_deref()
        && let Some(ctx) = ext_store.staff_context(id) {
            return StaffMeta {
                context_type: ctx.context_type.clone(),
                name: ctx.name.clone(),
                with_block_str: ctx.with_block.clone(),
                has_explicit_context: ctx.keyword.is_some(),
            };
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

/// Build a default `\with` block for ChordNames that uses conventional
/// jazz/pop chord name rendering instead of LilyPond's default symbols.
///
/// - `majorSevenSymbol` → "maj7" (instead of △)
/// - Augmented chords → "+" prefix (instead of ♯5)
fn jazz_chord_name_with_block() -> Vec<crate::model::ContextModItem> {
    use crate::model::{
        markup::Markup, Assignment, AssignmentValue, ContextModItem, SchemeExpr,
    };

    // majorSevenSymbol = \markup "maj7"
    let major_seven = ContextModItem::Assignment(Assignment {
        name: "majorSevenSymbol".to_string(),
        value: AssignmentValue::Markup(Markup::String("maj7".to_string())),
    });

    // chordNameExceptions = #(append (sequential-music-to-chord-exceptions #{ ... #} #t) ignatzekExceptions)
    let exceptions = ContextModItem::Assignment(Assignment {
        name: "chordNameExceptions".to_string(),
        value: AssignmentValue::SchemeExpr(SchemeExpr::Raw(
            concat!(
                "(append\n",
                "      (sequential-music-to-chord-exceptions\n",
                "       #{\n",
                "        <c e gis>-\\markup { \"+\" }\n",
                "        <c e gis b>-\\markup { \"+maj7\" }\n",
                "        <c e gis bes>-\\markup { \"+7\" }\n",
                "       #} #t)\n",
                "      ignatzekExceptions)",
            )
            .to_string(),
        )),
    });

    vec![major_seven, exceptions]
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
    ext_store: &ExtensionStore,
    measure_numbers: &[String],
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
            && let Some(raw) = staff_layer_children.first()
        {
            music = lyrics::wrap_music_with_lyrics(music, raw, info, ext_store, &measure_numbers);
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
            .and_then(parse_with_block_str)
            .or_else(|| Some(jazz_chord_name_with_block()));
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

    // Multi-staff (PianoStaff etc.): \addlyrics inside << >> misaligns with
    // multi-voice bars. Use \context Voice + \new Lyrics \lyricsto instead.
    // ChordNames/FiguredBass don't cause this — \addlyrics binds to the
    // preceding music expr, so it works fine in << >> with just chords.
    let is_multi_staff = num_staves > 1;

    for (i, layers) in staff_music.into_iter().enumerate() {
        let mut inner = build_layers_music(layers);
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

        // Check if this staff has lyrics
        let has_lyrics = lyrics_infos.get(i).is_some_and(|opt| opt.is_some());

        // Multi-staff with lyrics: wrap inner music in \context Voice = "name"
        let voice_name = if has_lyrics && is_multi_staff {
            let name = format!("lyrics-v{}", i + 1);
            inner = Music::ContextedMusic {
                keyword: ContextKeyword::Context,
                context_type: "Voice".to_string(),
                name: Some(name.clone()),
                with_block: None,
                music: Box::new(inner),
            };
            Some(name)
        } else {
            None
        };

        let staff_music_expr = Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: ctx_type,
            name: meta.and_then(|m| m.name.clone()),
            with_block,
            music: Box::new(inner),
        };

        if let Some(voice_name) = voice_name {
            // Multi-staff: use \new Lyrics \lyricsto instead of \addlyrics
            if let Some(Some(info)) = lyrics_infos.get(i)
                && let Some(raw) = staff_layer_children.get(i)
            {
                let (staff_expr, lyrics_exprs) =
                    lyrics::wrap_music_with_lyricsto(staff_music_expr, raw, info, ext_store, &voice_name, &measure_numbers);
                staff_exprs.push(staff_expr);
                staff_exprs.extend(lyrics_exprs);
            } else {
                staff_exprs.push(staff_music_expr);
            }
        } else {
            // Single-staff or no lyrics: use \addlyrics as before
            let mut expr = staff_music_expr;
            if let Some(Some(info)) = lyrics_infos.get(i)
                && let Some(raw) = staff_layer_children.get(i)
            {
                expr = lyrics::wrap_music_with_lyrics(expr, raw, info, ext_store, &measure_numbers);
            }
            staff_exprs.push(expr);
        }
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

/// Flatten a single measure's layers into a list of Music items.
///
/// If only one layer, returns its items directly. If multiple layers,
/// wraps in `<< >>`. When `inject_voice_cmds` is true, prepends
/// `\voiceOne`/`\voiceTwo` etc. and appends `\oneVoice` to restore
/// default stem direction.
///
/// When `spacer` is provided, adds an invisible skip voice to the `<< >>`
/// block to enforce a minimum measure duration. This prevents duration
/// mismatches between staves from accumulating and causing orphan systems.
fn flatten_measure_layers(
    layers: Vec<Vec<Music>>,
    inject_voice_cmds: bool,
    spacer: Option<Music>,
) -> Vec<Music> {
    let non_empty: Vec<Vec<Music>> = layers.into_iter().filter(|l| !l.is_empty()).collect();
    match non_empty.len() {
        0 => {
            if let Some(s) = spacer {
                vec![s]
            } else {
                Vec::new()
            }
        }
        1 if spacer.is_none() => non_empty.into_iter().next().unwrap(),
        1 => {
            let content = non_empty.into_iter().next().unwrap();
            // If the layer has no sounding notes (only rests + attribute changes),
            // skip the polyphonic wrapper. Promote a lone rest to R (multi-measure
            // rest) so LilyPond centers it in the bar instead of left-aligning.
            if is_rest_only_layer(&content) {
                promote_lone_rest_to_mmrest(content)
            } else {
                let mut voices = vec![Music::Sequential(content)];
                if let Some(s) = spacer {
                    voices.push(Music::Sequential(vec![s]));
                }
                vec![Music::Simultaneous(voices)]
            }
        }
        _ if inject_voice_cmds => {
            let voice_names = ["voiceOne", "voiceTwo", "voiceThree", "voiceFour"];
            let mut voices: Vec<Music> = non_empty
                .into_iter()
                .enumerate()
                .map(|(i, mut items)| {
                    if let Some(&name) = voice_names.get(i) {
                        items.insert(
                            0,
                            Music::MusicFunction {
                                name: name.to_string(),
                                args: vec![],
                            },
                        );
                    }
                    Music::Sequential(items)
                })
                .collect();
            if let Some(s) = spacer {
                voices.push(Music::Sequential(vec![s]));
            }
            vec![
                Music::Simultaneous(voices),
                Music::MusicFunction {
                    name: "oneVoice".to_string(),
                    args: vec![],
                },
            ]
        }
        _ => {
            let mut voices: Vec<Music> = non_empty.into_iter().map(Music::Sequential).collect();
            if let Some(s) = spacer {
                voices.push(Music::Sequential(vec![s]));
            }
            vec![Music::Simultaneous(voices)]
        }
    }
}

/// Create a spacer skip for a given time signature to enforce measure duration.
///
/// Produces e.g. `s4*4` for 4/4, `s4*5` for 5/4, `s4*2` for 2/4.
fn make_measure_spacer(numerators: &[u32], denominator: u32) -> Music {
    let total_num: u32 = numerators.iter().sum();
    use crate::model::note::SkipEvent;
    Music::Skip(SkipEvent {
        duration: Some(Duration {
            base: denominator,
            dots: 0,
            multipliers: vec![(total_num, 1)],
        }),
        post_events: vec![],
    })
}

/// Check if a layer contains only rests and non-sounding items (clef, time, key changes, etc.).
/// Such layers don't need a spacer-enforced polyphonic wrapper — removing it lets LilyPond
/// center whole-bar rests instead of left-aligning them.
fn is_rest_only_layer(items: &[Music]) -> bool {
    items.iter().all(|m| {
        matches!(
            m,
            Music::Rest(_)
                | Music::MultiMeasureRest(_)
                | Music::Skip(_)
                | Music::Clef(_)
                | Music::KeySignature(_)
                | Music::TimeSignature(_)
                | Music::BarLine { .. }
                | Music::BarCheck
                | Music::Mark(_)
                | Music::TextMark(_)
                | Music::Tempo(_)
                | Music::Override { .. }
                | Music::Revert { .. }
                | Music::Set { .. }
                | Music::Unset { .. }
                | Music::Once { .. }
                | Music::LineComment(_)
        )
    })
}

/// If the layer has exactly one `Music::Rest`, promote it to `Music::MultiMeasureRest`
/// (`R` instead of `r`) so LilyPond centers it in the bar.
fn promote_lone_rest_to_mmrest(mut items: Vec<Music>) -> Vec<Music> {
    let rest_count = items.iter().filter(|m| matches!(m, Music::Rest(_))).count();
    if rest_count == 1 {
        for item in &mut items {
            if let Music::Rest(r) = item {
                *item = Music::MultiMeasureRest(MultiMeasureRestEvent {
                    duration: r.duration.take(),
                    post_events: std::mem::take(&mut r.post_events),
                });
                break;
            }
        }
    }
    items
}

/// Scan a layer's items for a `\time` change and return the new numerators/denominator.
fn extract_time_from_items(items: &[Music]) -> Option<(Vec<u32>, u32)> {
    for item in items {
        if let Music::TimeSignature(ts) = item {
            return Some((ts.numerators.clone(), ts.denominator));
        }
    }
    None
}

/// Insert `\set Timing.measurePosition = #(ly:make-moment 0)` before any
/// `\time` change in the items. This prevents LilyPond's "mid-measure time
/// signature without \partial" warning caused by tuplet floating-point
/// timing rounding errors that accumulate across measures.
///
/// Handles both flat items and nested `<< { \time ... } { spacer } >>`.
fn insert_timing_reset_before_time_change(items: &mut Vec<Music>) {
    let reset = || Music::Set {
        path: PropertyPath::new(vec!["Timing".to_string(), "measurePosition".to_string()]),
        value: PropertyValue::SchemeExpr(SchemeExpr::List("(ly:make-moment 0)".to_string())),
    };
    // Check top-level items first
    if let Some(pos) = items.iter().position(|m| matches!(m, Music::TimeSignature(_))) {
        items.insert(pos, reset());
        return;
    }
    // Check inside << { items... } { spacer } >> wrappers
    for item in items.iter_mut() {
        if let Music::Simultaneous(voices) = item {
            if let Some(Music::Sequential(inner)) = voices.first_mut() {
                if let Some(pos) = inner.iter().position(|m| matches!(m, Music::TimeSignature(_))) {
                    inner.insert(pos, reset());
                    return;
                }
            }
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
fn collect_tuplet_spans(measure_children: &[MeasureChild], ext_store: &ExtensionStore) -> Vec<TupletSpanInfo> {
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

            // Look up span_duration from ext_store
            let span_duration = ts.common.xml_id.as_deref().and_then(|id| {
                let info = ext_store.tuplet_info(id)?;
                let dur_info = info.span_duration.as_ref()?;
                Some(Duration {
                    base: dur_info.base,
                    dots: dur_info.dots,
                    multipliers: dur_info.multipliers.clone(),
                })
            });

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
            // Push None for any extra items (e.g. context changes) injected
            // before the note by convert_layer_child_to_items.
            for _ in 1..count {
                ids.push(None);
            }
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
    item_ids: &mut Vec<Option<String>>,
    tuplet_spans: &[TupletSpanInfo],
    grace_types: &mut Vec<Option<grace::ExportGraceType>>,
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

    // Update item_ids in place so downstream wrapping steps (grace, repeat)
    // see positions that match the modified items array.
    for &(_orig_start, _orig_end, span_idx) in &ranges {
        let span = &tuplet_spans[span_idx];

        // Find current positions in the (possibly modified) ids list
        let cur_start = item_ids
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == span.start_id));
        let cur_end = item_ids
            .iter()
            .rposition(|id| id.as_deref().is_some_and(|i| i == span.end_id));

        if let (Some(cs), Some(ce)) = (cur_start, cur_end)
            && cs <= ce
            && ce < items.len()
        {
            let start_id = item_ids[cs].clone();

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
            item_ids.drain(cs..=ce);
            item_ids.insert(cs, start_id);

            // Keep grace_types in sync: tuplet itself is not grace
            let gt_end = ce.min(grace_types.len().saturating_sub(1));
            if cs <= gt_end && cs < grace_types.len() {
                grace_types.drain(cs..=gt_end);
                grace_types.insert(cs, None);
            }
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
    InsertionLog, collect_function_ops, collect_property_ops, collect_scheme_music_ops,
    inject_function_ops, inject_property_ops, inject_scheme_music_ops,
};

/// Apply an insertion log to a parallel array, inserting `None` at the logged positions.
///
/// The log contains `(position, count)` pairs in descending position order,
/// matching the back-to-front insertion order used by `inject_ops_by_startid`.
fn apply_insertion_log<T>(vec: &mut Vec<Option<T>>, log: &InsertionLog) {
    for &(pos, count) in log {
        for j in 0..count {
            if pos + j <= vec.len() {
                vec.insert(pos + j, None);
            }
        }
    }
}

/// Extract dur.default and oct.default from MEI scoreDef/staffDef hierarchy.
fn extract_mei_defaults(score: &tusk_model::elements::Score) -> conversion::MeiDefaults {
    let mut defaults = conversion::MeiDefaults::default();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            // Score-level defaults
            if let Some(ref dur) = score_def.score_def_log.dur_default {
                defaults.dur = conversion::mei_data_dur_to_ly(dur);
            }
            if let Some(ref oct) = score_def.score_def_log.oct_default {
                defaults.oct = Some(oct.0);
            }
            // StaffDef-level overrides (use first staffDef as representative)
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    if let Some(sdef) = collect_staff_defs_from_grp(grp).first() {
                        if let Some(ref dur) = sdef.staff_def_log.dur_default {
                            defaults.dur = conversion::mei_data_dur_to_ly(dur);
                        }
                        if let Some(ref oct) = sdef.staff_def_log.oct_default {
                            defaults.oct = Some(oct.0);
                        }
                        return defaults;
                    }
                }
            }
            return defaults;
        }
    }
    defaults
}

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
    ext_store: &ExtensionStore,
    defaults: &conversion::MeiDefaults,
) {
    match child {
        LayerChild::Beam(beam) => {
            let count = beam.children.len();
            for (i, bc) in beam.children.iter().enumerate() {
                if let Some(mut m) = convert_beam_child(bc, ext_store, defaults) {
                    // Apply slur post-events by xml:id (including chord child notes)
                    let events = collect_post_events_for_beam_child(bc, slur_map);
                    if !events.is_empty() {
                        append_post_events(&mut m, &events);
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
            // Inject \change before notes with context-change ext
            if let Some(change) = extract_context_change(child, ext_store) {
                items.push(change);
            }
            if let Some(mut m) = convert_layer_child(child, ext_store, defaults) {
                let events = collect_post_events_for_layer_child(child, slur_map);
                if !events.is_empty() {
                    append_post_events(&mut m, &events);
                }
                items.push(m);
            }
        }
    }
}

/// Extract a `\change` context change from a LayerChild via ext_store.
fn extract_context_change(child: &LayerChild, ext_store: &ExtensionStore) -> Option<Music> {
    let id = match child {
        LayerChild::Note(n) => n.common.xml_id.as_deref()?,
        LayerChild::Rest(r) => r.common.xml_id.as_deref()?,
        LayerChild::Chord(c) => c.common.xml_id.as_deref()?,
        _ => return None,
    };
    let cc = ext_store.context_change(id)?;
    Some(Music::ContextChange {
        context_type: cc.context_type.clone(),
        name: cc.name.clone(),
    })
}

/// Convert a single MEI LayerChild to a LilyPond Music expression.
fn convert_layer_child(child: &LayerChild, ext_store: &ExtensionStore, defaults: &conversion::MeiDefaults) -> Option<Music> {
    match child {
        LayerChild::Note(note) => Some(convert_mei_note(note, ext_store, defaults)),
        LayerChild::Rest(rest) => Some(convert_mei_rest(rest, ext_store, defaults)),
        LayerChild::MRest(mrest) => Some(convert_mei_mrest(mrest, ext_store)),
        LayerChild::Chord(chord) => Some(convert_mei_chord(chord, ext_store, defaults)),
        LayerChild::BTrem(btrem) => Some(convert_mei_btrem(btrem, ext_store, defaults)),
        LayerChild::Space(space) => Some(convert_mei_space(space)),
        LayerChild::MeterSig(msig) => convert_mei_meter_sig(msig),
        LayerChild::Clef(clef) => convert_mei_inline_clef(clef),
        _ => None,
    }
}

/// Convert an inline MEI `<meterSig>` to `\time`.
fn convert_mei_meter_sig(msig: &tusk_model::elements::MeterSig) -> Option<Music> {
    let count_str = msig.meter_sig_log.count.as_deref()?;
    let numerators: Vec<u32> = count_str
        .split('+')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    if numerators.is_empty() {
        return None;
    }
    let denominator: u32 = msig
        .meter_sig_log
        .unit
        .as_deref()
        .and_then(|u| u.parse().ok())
        .unwrap_or(4);
    Some(Music::TimeSignature(
        crate::model::signature::TimeSignature {
            numerators,
            denominator,
        },
    ))
}

/// Convert an inline MEI `<clef>` to `\clef`.
fn convert_mei_inline_clef(clef: &tusk_model::elements::Clef) -> Option<Music> {
    let shape = clef.clef_log.shape.as_ref()?;
    let line = clef.clef_log.line.as_ref().map(|l| l.0).unwrap_or(2);
    let dis = clef.clef_log.dis.as_ref().map(|d| d.0);
    let dis_place = clef.clef_log.dis_place.as_ref();
    let name = crate::import::mei_clef_to_name(shape, line, dis, dis_place);
    Some(Music::Clef(crate::model::signature::Clef { name }))
}

/// Convert a BeamChild to a LilyPond Music expression.
fn convert_beam_child(child: &tusk_model::elements::BeamChild, ext_store: &ExtensionStore, defaults: &conversion::MeiDefaults) -> Option<Music> {
    use tusk_model::elements::BeamChild;
    match child {
        BeamChild::Note(note) => Some(convert_mei_note(note, ext_store, defaults)),
        BeamChild::Rest(rest) => Some(convert_mei_rest(rest, ext_store, defaults)),
        BeamChild::Chord(chord) => Some(convert_mei_chord(chord, ext_store, defaults)),
        BeamChild::Beam(beam) => {
            // Nested beams: flatten recursively (nested beams just continue the beam)
            // This shouldn't produce beam markers for the inner beam since
            // LilyPond uses a flat [ ... ] without nesting
            let mut nested = Vec::new();
            for bc in &beam.children {
                if let Some(m) = convert_beam_child(bc, ext_store, defaults) {
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
        LayerChild::Space(space) => space.common.xml_id.as_deref(),
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

/// Collect post-events for a layer child from the slur map.
/// For chords, also checks child note IDs since MEI slurs reference
/// individual notes, but LilyPond attaches slur marks to the chord.
fn collect_post_events_for_layer_child(
    child: &LayerChild,
    slur_map: &HashMap<String, Vec<PostEvent>>,
) -> Vec<PostEvent> {
    let mut events = Vec::new();
    if let Some(id) = layer_child_xml_id(child) {
        if let Some(evts) = slur_map.get(id) {
            events.extend(evts.iter().cloned());
        }
    }
    // For chords, also check child note IDs
    if let LayerChild::Chord(chord) = child {
        for cc in &chord.children {
            let tusk_model::elements::ChordChild::Note(note) = cc;
            if let Some(id) = note.common.xml_id.as_deref() {
                if let Some(evts) = slur_map.get(id) {
                    events.extend(evts.iter().cloned());
                }
            }
        }
    }
    events
}

/// Collect post-events for a beam child from the slur map.
/// For chords, also checks child note IDs.
fn collect_post_events_for_beam_child(
    child: &tusk_model::elements::BeamChild,
    slur_map: &HashMap<String, Vec<PostEvent>>,
) -> Vec<PostEvent> {
    let mut events = Vec::new();
    if let Some(id) = beam_child_xml_id(child) {
        if let Some(evts) = slur_map.get(id) {
            events.extend(evts.iter().cloned());
        }
    }
    if let tusk_model::elements::BeamChild::Chord(chord) = child {
        for cc in &chord.children {
            let tusk_model::elements::ChordChild::Note(note) = cc;
            if let Some(id) = note.common.xml_id.as_deref() {
                if let Some(evts) = slur_map.get(id) {
                    events.extend(evts.iter().cloned());
                }
            }
        }
    }
    events
}

/// Collect slur/phrase control events from measure children into a map of
/// note xml:id -> PostEvent list.
fn collect_slur_post_events(measure_children: &[MeasureChild], ext_store: &ExtensionStore) -> HashMap<String, Vec<PostEvent>> {
    let mut map: HashMap<String, Vec<PostEvent>> = HashMap::new();

    for mc in measure_children {
        if let MeasureChild::Slur(slur) = mc {
            let is_phrase = slur
                .common
                .xml_id
                .as_deref()
                .is_some_and(|id| ext_store.phrasing_slur(id).is_some());

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
/// via ext_store.
fn collect_artic_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
    ext_store: &ExtensionStore,
) {
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let dir_id = match dir.common.xml_id.as_deref() {
                Some(id) => id,
                None => continue,
            };
            let startid = match dir.dir_log.startid.as_ref() {
                Some(s) => s.0.trim_start_matches('#').to_string(),
                None => continue,
            };

            if let Some(info) = ext_store.articulation_info(dir_id)
                && let Some(pe) = artic_info_to_post_event(info)
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

/// Collect text script post-events from `<dir>` elements via ext_store.
fn collect_text_script_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
    ext_store: &ExtensionStore,
) {
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let dir_id = match dir.common.xml_id.as_deref() {
                Some(id) => id,
                None => continue,
            };
            let startid = match dir.dir_log.startid.as_ref() {
                Some(s) => s.0.trim_start_matches('#').to_string(),
                None => continue,
            };

            if let Some(info) = ext_store.text_script_info(dir_id)
                && let Some(pe) = text_script_info_to_post_event(info)
            {
                map.entry(startid).or_default().push(pe);
            }
        }
    }
}

/// Convert a TextScriptInfo to a PostEvent::TextScript.
fn text_script_info_to_post_event(info: &tusk_model::TextScriptInfo) -> Option<PostEvent> {
    let direction = direction_ext_to_ly(info.direction);
    let text = parse_text_script_text(&info.serialized)?;
    Some(PostEvent::TextScript { direction, text })
}

/// Re-parse a serialized text script text into a Markup AST.
///
/// Handles both `"string"` (quoted string) and `\markup ...` forms.
fn parse_text_script_text(s: &str) -> Option<crate::model::markup::Markup> {
    let trimmed = s.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        // Quoted string: extract the content between quotes
        let inner = &trimmed[1..trimmed.len() - 1];
        Some(crate::model::markup::Markup::String(inner.to_string()))
    } else if trimmed.starts_with("\\markup") {
        // Markup expression: re-parse through the LilyPond parser
        use crate::parser::Parser;
        let file = Parser::new(trimmed).ok()?.parse().ok()?;
        for item in &file.items {
            if let crate::model::ToplevelExpression::Markup(m) = item {
                return Some(m.clone());
            }
        }
        None
    } else {
        None
    }
}

/// Collect ornament control events (trill, mordent, turn, fermata, ornam) from
/// measure children into the post-event map via ext_store.
fn collect_ornament_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<PostEvent>>,
    ext_store: &ExtensionStore,
) {
    for mc in measure_children {
        match mc {
            MeasureChild::Trill(trill) => {
                if let Some(ref startid) = trill.trill_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let (name, direction) =
                        parse_ornament_from_ext(trill.common.xml_id.as_deref(), "trill", ext_store);
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
                        parse_ornament_from_ext(mordent.common.xml_id.as_deref(), fallback, ext_store);
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
                        parse_ornament_from_ext(turn.common.xml_id.as_deref(), fallback, ext_store);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            MeasureChild::Fermata(fermata) => {
                if let Some(ref startid) = fermata.fermata_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let (name, direction) =
                        parse_ornament_from_ext(fermata.common.xml_id.as_deref(), "fermata", ext_store);
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
                        parse_ornament_from_ext(ornam.common.xml_id.as_deref(), &fallback_name, ext_store);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            _ => {}
        }
    }
}

/// Parse name and direction from ext_store ornament info.
fn parse_ornament_from_ext(xml_id: Option<&str>, fallback_name: &str, ext_store: &ExtensionStore) -> (String, Direction) {
    if let Some(id) = xml_id
        && let Some(info) = ext_store.ornament_info(id) {
            let dir = direction_ext_to_ly(info.direction);
            return (info.name.clone(), dir);
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
/// subdivision value restored from ext_store.
fn convert_mei_btrem(btrem: &tusk_model::elements::BTrem, ext_store: &ExtensionStore, defaults: &conversion::MeiDefaults) -> Music {
    // Restore subdivision value from ext_store
    let value = btrem
        .common
        .xml_id
        .as_deref()
        .and_then(|id| ext_store.tremolo_info(id))
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
            tusk_model::elements::BTremChild::Note(n) => convert_mei_note(n, ext_store, defaults),
            tusk_model::elements::BTremChild::Chord(c) => convert_mei_chord(c, ext_store, defaults),
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

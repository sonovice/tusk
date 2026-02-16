//! Conversion from LilyPond AST to MEI.

mod beams;
mod context_analysis;
mod control_events;
mod conversion;
mod events;
pub(crate) mod lyrics;
pub(crate) mod output_def_conv;
mod output_defs;
pub(crate) mod signatures;
pub(crate) mod variables;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_book;
#[cfg(test)]
mod tests_chords;
#[cfg(test)]
mod tests_completion;
#[cfg(test)]
mod tests_control;
#[cfg(test)]
mod tests_drums;
#[cfg(test)]
mod tests_figures;
#[cfg(test)]
mod tests_functions;
#[cfg(test)]
mod tests_output_defs;
#[cfg(test)]
mod tests_properties;
#[cfg(test)]
mod tests_tempo_marks;
#[cfg(test)]
mod tests_toplevel_markup;
#[cfg(test)]
mod tests_validation;
#[cfg(test)]
mod tests_variables;

use thiserror::Error;
use tusk_model::elements::{
    Body, BodyChild, Layer, LayerChild, Mdiv, MdivChild, Measure, MeasureChild, Mei, MeiChild,
    Score, ScoreChild, Section, SectionChild, Staff, StaffChild,
};
use tusk_model::ExtensionStore;
// Re-exported for test modules that use `use super::*`
#[cfg(test)]
#[allow(unused_imports)]
use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffDef, StaffGrp, StaffGrpChild};
use tusk_model::generated::data::{DataTie, DataWord};

use crate::model::{self, Assignment, Music, PostEvent, ScoreItem, ToplevelExpression};
use tusk_model::{ToplevelMarkup, ToplevelMarkupKind};

use context_analysis::{StaffLayout, analyze_staves, build_score_def_from_staves};
use events::{
    GraceType, LyEvent, PitchContext, apply_grace_to_chord, apply_grace_to_note, collect_events,
    extract_pitch_from_music,
};
pub use signatures::{fifths_to_key, mei_clef_to_name};
use utils::voice_needs_pitch_context;

use conversion::{
    convert_chord, convert_drum_chord, convert_drum_note, convert_mrest, convert_note,
    convert_pitched_rest, convert_rest, convert_skip,
};

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("LilyPond import is not yet implemented")]
    NotImplemented,
    #[error("no music found in LilyPond file")]
    NoMusic,
    #[error("validation errors:\n{}", .0.iter().map(|e| format!("  - {e}")).collect::<Vec<_>>().join("\n"))]
    Validation(Vec<crate::validator::ValidationError>),
    #[error("import error: {0}")]
    Other(String),
}

/// Convert a parsed LilyPond AST to an MEI document.
///
/// Runs structural validation before conversion; returns
/// [`ImportError::Validation`] if the AST has problems.
pub fn import(file: &model::LilyPondFile) -> Result<(Mei, ExtensionStore), ImportError> {
    // Validate structure before import
    if let Err(errors) = crate::validator::validate(file) {
        return Err(ImportError::Validation(errors));
    }

    let mut ext_store = ExtensionStore::default();

    // Collect top-level assignments for variable resolution
    let assignments = collect_assignments(file);
    let var_map = build_variable_map(&assignments);

    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some("6.0-dev".to_string());

    // Build meiHead with metadata from \header and output-def blocks
    let mei_head = output_defs::build_mei_head_from_file(file, &mut ext_store);
    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

    // Collect top-level markups
    let toplevel_markups = collect_toplevel_markups(file);

    // Collect all score entries (handling book/bookpart hierarchy)
    let entries = collect_score_entries(file);

    // Use book-structured path only when \book wrappers exist
    let has_book = entries.iter().any(|e| e.book_structure.is_some());

    let mei_music = if has_book && !entries.is_empty() {
        // Book-structured: one mdiv per score entry
        build_music_multi(&entries, &assignments, &var_map, &toplevel_markups, &mut ext_store)?
    } else {
        // Non-book: use find_music for backward-compatible behavior
        let raw_music = find_music(file).ok_or(ImportError::NoMusic)?;
        let music = resolve_identifiers(raw_music, &var_map);
        let score_block = entries.first().map(|e| e.score_block);
        build_music_single(music, score_block, &assignments, &toplevel_markups, &mut ext_store)?
    };
    mei.children.push(MeiChild::Music(Box::new(mei_music)));

    Ok((mei, ext_store))
}

/// A score entry extracted from a book/bookpart/top-level structure.
struct ScoreEntry<'a> {
    music: &'a Music,
    score_block: &'a model::ScoreBlock,
    book_structure: Option<tusk_model::BookStructure>,
}

/// Collect all score entries from the file, walking into \book and \bookpart.
fn collect_score_entries<'a>(file: &'a model::LilyPondFile) -> Vec<ScoreEntry<'a>> {
    let mut entries = Vec::new();
    let mut book_idx = 0usize;

    for item in &file.items {
        match item {
            ToplevelExpression::Score(score) => {
                if let Some(m) = find_music_in_score(score) {
                    entries.push(ScoreEntry {
                        music: m,
                        score_block: score,
                        book_structure: None,
                    });
                }
            }
            ToplevelExpression::Book(book) => {
                let book_defs = collect_output_defs_from_book(book);
                let mut bookpart_idx = 0usize;
                let mut score_idx = 0usize;

                for bi in &book.items {
                    match bi {
                        model::BookItem::Score(score) => {
                            if let Some(m) = find_music_in_score(score) {
                                entries.push(ScoreEntry {
                                    music: m,
                                    score_block: score,
                                    book_structure: Some(tusk_model::BookStructure {
                                        book_index: Some(book_idx),
                                        bookpart_index: None,
                                        score_index: Some(score_idx),
                                        book_output_defs: book_defs.clone(),
                                        bookpart_output_defs: vec![],
                                    }),
                                });
                                score_idx += 1;
                            }
                        }
                        model::BookItem::BookPart(bp) => {
                            let bp_defs = collect_output_defs_from_bookpart(bp);
                            let mut bp_score_idx = 0usize;

                            for bpi in &bp.items {
                                if let model::BookPartItem::Score(score) = bpi
                                    && let Some(m) = find_music_in_score(score)
                                {
                                    entries.push(ScoreEntry {
                                        music: m,
                                        score_block: score,
                                        book_structure: Some(tusk_model::BookStructure {
                                            book_index: Some(book_idx),
                                            bookpart_index: Some(bookpart_idx),
                                            score_index: Some(bp_score_idx),
                                            book_output_defs: book_defs.clone(),
                                            bookpart_output_defs: bp_defs.clone(),
                                        }),
                                    });
                                    bp_score_idx += 1;
                                }
                            }
                            bookpart_idx += 1;
                        }
                        _ => {}
                    }
                }
                book_idx += 1;
            }
            _ => {}
        }
    }

    entries
}

/// Collect top-level `\markup` and `\markuplist` expressions with their positions.
fn collect_toplevel_markups(file: &model::LilyPondFile) -> Vec<ToplevelMarkup> {
    let mut markups = Vec::new();
    for (idx, item) in file.items.iter().enumerate() {
        match item {
            ToplevelExpression::Markup(m) => {
                let serialized = crate::serializer::serialize_markup(m);
                markups.push(ToplevelMarkup {
                    position: idx,
                    kind: ToplevelMarkupKind::Markup(serialized),
                });
            }
            ToplevelExpression::MarkupList(ml) => {
                let serialized = crate::serializer::serialize_markuplist(ml);
                markups.push(ToplevelMarkup {
                    position: idx,
                    kind: ToplevelMarkupKind::MarkupList(serialized),
                });
            }
            _ => {}
        }
    }
    markups
}

/// Find music expression inside a score block.
fn find_music_in_score(score: &model::ScoreBlock) -> Option<&Music> {
    score.items.iter().find_map(|si| {
        if let ScoreItem::Music(m) = si {
            Some(m)
        } else {
            None
        }
    })
}

/// Collect output defs (header/paper) at book level.
fn collect_output_defs_from_book(book: &model::BookBlock) -> Vec<tusk_model::OutputDef> {
    let mut defs = Vec::new();
    for item in &book.items {
        match item {
            model::BookItem::Header(hb) => {
                defs.push(output_def_conv::header_to_output_def(hb));
            }
            model::BookItem::Paper(pb) => {
                defs.push(output_def_conv::paper_to_output_def(pb));
            }
            _ => {}
        }
    }
    defs
}

/// Collect output defs (header/paper) at bookpart level.
fn collect_output_defs_from_bookpart(bp: &model::BookPartBlock) -> Vec<tusk_model::OutputDef> {
    let mut defs = Vec::new();
    for item in &bp.items {
        match item {
            model::BookPartItem::Header(hb) => {
                defs.push(output_def_conv::header_to_output_def(hb));
            }
            model::BookPartItem::Paper(pb) => {
                defs.push(output_def_conv::paper_to_output_def(pb));
            }
            _ => {}
        }
    }
    defs
}

/// Find the first music expression in the LilyPond file (including inside books).
fn find_music(file: &model::LilyPondFile) -> Option<&Music> {
    for item in &file.items {
        match item {
            ToplevelExpression::Score(score) => {
                if let Some(m) = find_music_in_score(score) {
                    return Some(m);
                }
            }
            ToplevelExpression::Book(book) => {
                for bi in &book.items {
                    match bi {
                        model::BookItem::Score(score) => {
                            if let Some(m) = find_music_in_score(score) {
                                return Some(m);
                            }
                        }
                        model::BookItem::BookPart(bp) => {
                            for bpi in &bp.items {
                                if let model::BookPartItem::Score(score) = bpi
                                    && let Some(m) = find_music_in_score(score)
                                {
                                    return Some(m);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            ToplevelExpression::Music(m) => return Some(m),
            _ => {}
        }
    }
    None
}

use variables::{build_variable_map, collect_assignments, resolve_identifiers};

/// Build MEI Music with multiple mdivs for book-structured files.
fn build_music_multi(
    entries: &[ScoreEntry<'_>],
    assignments: &[Assignment],
    var_map: &std::collections::HashMap<String, Music>,
    toplevel_markups: &[ToplevelMarkup],
    ext_store: &mut ExtensionStore,
) -> Result<tusk_model::elements::Music, ImportError> {
    let mut body = Body::default();

    for (i, entry) in entries.iter().enumerate() {
        let resolved = resolve_identifiers(entry.music, var_map);
        // Only store toplevel markups on the first score
        let markups = if i == 0 { toplevel_markups } else { &[] };
        let mei_score =
            build_score_from_music(resolved, Some(entry.score_block), assignments, markups, ext_store)?;

        let mut mdiv = Mdiv::default();
        mdiv.common.n = Some(DataWord((i + 1).to_string()));

        if let Some(ref bs) = entry.book_structure {
            let mdiv_id = format!("ly-mdiv-{i}");
            mdiv.common.xml_id = Some(mdiv_id.clone());
            ext_store.insert_book_structure(mdiv_id, bs.clone());
        }

        mdiv.children.push(MdivChild::Score(Box::new(mei_score)));
        body.children.push(BodyChild::Mdiv(Box::new(mdiv)));
    }

    let mut music = tusk_model::elements::Music::default();
    music
        .children
        .push(tusk_model::elements::MusicChild::Body(Box::new(body)));
    Ok(music)
}

/// Build MEI Music from a single LilyPond music tree.
fn build_music_single(
    ly_music: Music,
    score_block: Option<&model::ScoreBlock>,
    assignments: &[Assignment],
    toplevel_markups: &[ToplevelMarkup],
    ext_store: &mut ExtensionStore,
) -> Result<tusk_model::elements::Music, ImportError> {
    let score = build_score_from_music(ly_music, score_block, assignments, toplevel_markups, ext_store)?;

    let mut mdiv = Mdiv::default();
    mdiv.children.push(MdivChild::Score(Box::new(score)));

    let mut body = Body::default();
    body.children.push(BodyChild::Mdiv(Box::new(mdiv)));

    let mut music = tusk_model::elements::Music::default();
    music
        .children
        .push(tusk_model::elements::MusicChild::Body(Box::new(body)));

    Ok(music)
}

/// Build an MEI Score element from a LilyPond music expression.
///
/// When `score_block` is provided, score-level output defs are extracted from it.
/// Otherwise, falls back to scanning the file for the first `\score` block.
fn build_score_from_music(
    ly_music: Music,
    score_block: Option<&model::ScoreBlock>,
    assignments: &[Assignment],
    toplevel_markups: &[ToplevelMarkup],
    ext_store: &mut ExtensionStore,
) -> Result<Score, ImportError> {
    let mut score = Score::default();

    // Analyze context structure to determine staves
    let staff_infos = analyze_staves(&ly_music);

    // Build ScoreDef with staffDef(s)
    let mut score_def = build_score_def_from_staves(&staff_infos, assignments, ext_store);

    // Store score-level \header/\layout/\midi in ScoreDef via ext_store
    if let Some(sb) = score_block {
        let score_output_defs = output_defs::collect_score_block_output_defs(sb);
        if !score_output_defs.is_empty() {
            let sd_id = score_def.common.xml_id.get_or_insert_with(|| "ly-scoredef-0".to_string()).clone();
            ext_store.insert_output_defs(sd_id, score_output_defs);
        }
    }

    // Store top-level markups via ext_store
    if !toplevel_markups.is_empty() {
        let sd_id = score_def.common.xml_id.get_or_insert_with(|| "ly-scoredef-0".to_string()).clone();
        ext_store.insert_toplevel_markups(sd_id, toplevel_markups.to_vec());
    }

    score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));

    // Section with measure(s) containing the notes
    let section = build_section_from_staves(&staff_infos, ext_store)?;
    score.children.push(ScoreChild::Section(Box::new(section)));

    Ok(score)
}

/// Build a PitchContext from a music tree that has pitch context wrappers.
///
/// Walks through `\relative`/`\fixed`/`\transpose` wrappers (and single-item
/// Sequential) to extract the pitch context, same as `collect_events` would.
/// Used to pre-initialize voice contexts when voices were split from inside
/// a pitch context wrapper.
fn build_pitch_context_from_music(music: &Music) -> Option<PitchContext> {
    match music {
        Music::Relative { pitch, .. } => {
            let (ref_step, ref_oct) = if let Some(ref_pitch_music) = pitch {
                extract_pitch_from_music(ref_pitch_music)
                    .map(|p| (p.step, p.octave))
                    .unwrap_or(('f', 0))
            } else {
                ('f', 0)
            };
            let mut ctx = PitchContext::new();
            ctx.relative = Some((ref_step, ref_oct));
            Some(ctx)
        }
        Music::Fixed { pitch, .. } => {
            let ref_oct = extract_pitch_from_music(pitch)
                .map(|p| p.octave)
                .unwrap_or(1);
            let mut ctx = PitchContext::new();
            ctx.fixed = Some(ref_oct);
            Some(ctx)
        }
        Music::Sequential(items) if items.len() == 1 => build_pitch_context_from_music(&items[0]),
        _ => None,
    }
}

/// Get the xml:id of the last note/rest/chord in a layer.
fn get_last_layer_child_id(layer: &Layer) -> Option<String> {
    match layer.children.last() {
        Some(LayerChild::Note(n)) => n.common.xml_id.clone(),
        Some(LayerChild::Rest(r)) => r.common.xml_id.clone(),
        Some(LayerChild::Chord(c)) => c.common.xml_id.clone(),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Section building from staff layout
// ---------------------------------------------------------------------------

/// A pending slur or phrasing slur waiting for its end note.
struct PendingSpanner {
    start_id: String,
    is_phrase: bool,
    staff_n: u32,
}

/// A pending hairpin (crescendo/decrescendo) waiting for its end note.
struct PendingHairpin {
    start_id: String,
    /// "cres" for crescendo, "dim" for diminuendo.
    form: String,
    staff_n: u32,
}

/// A pending repeat structure waiting for its body end note.
struct PendingRepeat {
    /// xml:id of the first note in the repeat body.
    start_id: String,
    repeat_type: model::RepeatType,
    count: u32,
    num_alternatives: u32,
    staff_n: u32,
}

/// A pending alternative ending waiting for its end note.
struct PendingAlternative {
    /// xml:id of the first note in the alternative.
    start_id: String,
    /// 0-based index of this alternative.
    index: u32,
    staff_n: u32,
}

/// A pending tuplet waiting for its end note.
struct PendingTuplet {
    /// xml:id of the first note in the tuplet.
    start_id: String,
    numerator: u32,
    denominator: u32,
    span_duration: Option<crate::model::Duration>,
    staff_n: u32,
}

/// A pending tempo/mark/textMark waiting for next note's startid.
enum PendingTempoMark {
    Tempo(crate::model::signature::Tempo),
    Mark(String),
    TextMark(String),
}

/// Build a Section from analyzed staff layout.
fn build_section_from_staves(layout: &StaffLayout<'_>, ext_store: &mut ExtensionStore) -> Result<Section, ImportError> {
    let mut section = Section::default();
    let mut id_counter = 0u32;
    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    let mut slur_counter = 0u32;
    let mut beam_counter = 0u32;
    let mut dynam_counter = 0u32;
    let mut hairpin_counter = 0u32;
    let mut artic_counter = 0u32;
    let mut ornam_counter = 0u32;
    let mut tuplet_counter = 0u32;
    let mut repeat_counter = 0u32;
    let mut tempo_mark_counter = 0u32;
    let mut harm_counter = 0u32;
    let mut fb_counter = 0u32;
    let mut func_counter = 0u32;
    let mut scm_counter = 0u32;
    let mut text_script_counter = 0u32;

    for staff_info in &layout.staves {
        let mut staff = Staff::default();
        staff.n_integer.n = Some(staff_info.n.to_string());

        // When voices were split from inside a pitch context wrapper (\relative/\fixed),
        // pre-initialize PitchContext so bare voice items get correct resolution.
        let split_pitch_ctx = if staff_info.voices.len() > 1
            && staff_info.voices.iter().all(|v| voice_needs_pitch_context(v))
        {
            staff_info
                .original_music
                .and_then(build_pitch_context_from_music)
        } else {
            None
        };

        for (voice_idx, voice_music) in staff_info.voices.iter().enumerate() {
            let mut layer = Layer::default();
            layer.n_integer.n = Some((voice_idx + 1).to_string());

            let mut events = Vec::new();
            let mut voice_ctx = split_pitch_ctx.clone().unwrap_or_else(PitchContext::new);
            for m in voice_music {
                collect_events(m, &mut events, &mut voice_ctx);
            }

            // Track beam start/end positions (index in layer.children)
            let mut beam_starts: Vec<usize> = Vec::new();

            // Track IDs of notes for tie/slur resolution
            let mut pending_slurs: Vec<PendingSpanner> = Vec::new();
            let mut pending_hairpins: Vec<PendingHairpin> = Vec::new();
            let mut pending_tuplets: Vec<PendingTuplet> = Vec::new();
            let mut pending_repeats: Vec<PendingRepeat> = Vec::new();
            let mut pending_alternatives: Vec<PendingAlternative> = Vec::new();
            let mut tie_pending = false;
            // Track the last note/chord/rest xml:id for tuplet boundary resolution
            let mut last_note_id: Option<String> = None;
            // Track current grace context for setting @grace on notes
            let mut current_grace: Option<GraceType> = None;
            // Pending tempo/mark/textMark waiting for next note's startid
            let mut pending_tempo_marks: Vec<PendingTempoMark> = Vec::new();
            // Pending inline chord names waiting for first note
            let mut pending_chord_names: Vec<(crate::model::note::ChordModeEvent, u32)> =
                Vec::new();
            // Pending property operations waiting for next note's startid
            let mut pending_property_ops: Vec<String> = Vec::new();
            // Pending music function calls waiting for next note's startid
            let mut pending_function_ops: Vec<tusk_model::FunctionCall> = Vec::new();
            // Pending Scheme music expressions waiting for next note's startid
            let mut pending_scheme_music: Vec<String> = Vec::new();
            // Cross-staff override from \change Staff = "name"
            let mut cross_staff_override: Option<tusk_model::ContextChange> = None;

            for event in &events {
                let (post_events, current_id) = match event {
                    LyEvent::Note(note) => {
                        id_counter += 1;
                        let mut mei_note = convert_note(note, id_counter);
                        if tie_pending {
                            mei_note.note_anl.tie = Some(DataTie::from("t".to_string()));
                            tie_pending = false;
                        }
                        let id_str = format!("ly-note-{}", id_counter);
                        let pe = note.post_events.clone();
                        if pe.contains(&PostEvent::Tie) {
                            match &mei_note.note_anl.tie {
                                Some(t) if t.0 == "t" => {
                                    mei_note.note_anl.tie = Some(DataTie::from("m".to_string()));
                                }
                                _ => {
                                    mei_note.note_anl.tie = Some(DataTie::from("i".to_string()));
                                }
                            }
                            tie_pending = true;
                        }
                        if let Some(ref gt) = current_grace {
                            apply_grace_to_note(&mut mei_note, gt, ext_store);
                        }
                        layer.children.push(LayerChild::Note(Box::new(mei_note)));
                        (pe, id_str)
                    }
                    LyEvent::Rest(rest) => {
                        id_counter += 1;
                        let mei_rest = convert_rest(rest, id_counter);
                        let id_str = format!("ly-rest-{}", id_counter);
                        let pe = rest.post_events.clone();
                        layer.children.push(LayerChild::Rest(Box::new(mei_rest)));
                        (pe, id_str)
                    }
                    LyEvent::PitchedRest(note) => {
                        id_counter += 1;
                        let mei_rest = convert_pitched_rest(note, id_counter, ext_store);
                        let id_str = format!("ly-rest-{}", id_counter);
                        let pe = note.post_events.clone();
                        layer.children.push(LayerChild::Rest(Box::new(mei_rest)));
                        (pe, id_str)
                    }
                    LyEvent::Skip(skip) => {
                        id_counter += 1;
                        let mei_space = convert_skip(skip, id_counter);
                        let id_str = format!("ly-space-{}", id_counter);
                        let pe = skip.post_events.clone();
                        layer.children.push(LayerChild::Space(Box::new(mei_space)));
                        (pe, id_str)
                    }
                    LyEvent::Chord {
                        pitches,
                        duration,
                        post_events,
                        is_chord_repetition,
                    } => {
                        id_counter += 1;
                        let mut mei_chord =
                            convert_chord(pitches, duration.as_ref(), &mut id_counter);
                        if *is_chord_repetition {
                            let id = mei_chord.common.xml_id.clone().unwrap();
                            ext_store.insert_chord_repetition(id, tusk_model::ChordRepetition);
                        }
                        if tie_pending {
                            for child in &mut mei_chord.children {
                                let tusk_model::elements::ChordChild::Note(n) = child;
                                n.note_anl.tie = Some(DataTie::from("t".to_string()));
                            }
                            tie_pending = false;
                        }
                        let id_str = mei_chord
                            .common
                            .xml_id
                            .clone()
                            .unwrap_or_else(|| format!("ly-chord-{}", id_counter));
                        let pe = post_events.clone();
                        if pe.contains(&PostEvent::Tie) {
                            for child in &mut mei_chord.children {
                                let tusk_model::elements::ChordChild::Note(n) = child;
                                match &n.note_anl.tie {
                                    Some(t) if t.0 == "t" => {
                                        n.note_anl.tie = Some(DataTie::from("m".to_string()));
                                    }
                                    _ => {
                                        n.note_anl.tie = Some(DataTie::from("i".to_string()));
                                    }
                                }
                            }
                            tie_pending = true;
                        }
                        if let Some(ref gt) = current_grace {
                            apply_grace_to_chord(&mut mei_chord, gt, ext_store);
                        }
                        layer.children.push(LayerChild::Chord(Box::new(mei_chord)));
                        (pe, id_str)
                    }
                    LyEvent::MeasureRest(rest) => {
                        id_counter += 1;
                        let mei_mrest = convert_mrest(rest, id_counter, ext_store);
                        layer.children.push(LayerChild::MRest(Box::new(mei_mrest)));
                        continue;
                    }
                    LyEvent::TupletStart {
                        numerator,
                        denominator,
                        span_duration,
                    } => {
                        pending_tuplets.push(PendingTuplet {
                            start_id: String::new(), // filled on next note
                            numerator: *numerator,
                            denominator: *denominator,
                            span_duration: span_duration.clone(),
                            staff_n: staff_info.n,
                        });
                        continue;
                    }
                    LyEvent::TupletEnd => {
                        if let Some(pending) = pending_tuplets.pop()
                            && let Some(end_id) = &last_note_id
                            && !pending.start_id.is_empty()
                        {
                            tuplet_counter += 1;
                            let ts = make_tuplet_span(
                                &pending.start_id,
                                end_id,
                                pending.staff_n,
                                pending.numerator,
                                pending.denominator,
                                pending.span_duration.as_ref(),
                                tuplet_counter,
                                ext_store,
                            );
                            measure
                                .children
                                .push(MeasureChild::TupletSpan(Box::new(ts)));
                        }
                        continue;
                    }
                    LyEvent::RepeatStart {
                        repeat_type,
                        count,
                        num_alternatives,
                    } => {
                        pending_repeats.push(PendingRepeat {
                            start_id: String::new(),
                            repeat_type: *repeat_type,
                            count: *count,
                            num_alternatives: *num_alternatives,
                            staff_n: staff_info.n,
                        });
                        continue;
                    }
                    LyEvent::RepeatEnd => {
                        if let Some(pending) = pending_repeats.pop()
                            && let Some(end_id) = &last_note_id
                            && !pending.start_id.is_empty()
                        {
                            repeat_counter += 1;
                            let dir = make_repeat_dir(
                                &pending.start_id,
                                end_id,
                                pending.staff_n,
                                pending.repeat_type,
                                pending.count,
                                pending.num_alternatives,
                                repeat_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        continue;
                    }
                    LyEvent::AlternativeStart { index } => {
                        pending_alternatives.push(PendingAlternative {
                            start_id: String::new(),
                            index: *index,
                            staff_n: staff_info.n,
                        });
                        continue;
                    }
                    LyEvent::AlternativeEnd => {
                        if let Some(pending) = pending_alternatives.pop()
                            && let Some(end_id) = &last_note_id
                            && !pending.start_id.is_empty()
                        {
                            repeat_counter += 1;
                            let dir = make_ending_dir(
                                &pending.start_id,
                                end_id,
                                pending.staff_n,
                                pending.index,
                                repeat_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        continue;
                    }
                    LyEvent::GraceStart(gt) => {
                        current_grace = Some(gt.clone());
                        continue;
                    }
                    LyEvent::GraceEnd => {
                        current_grace = None;
                        continue;
                    }
                    LyEvent::Tempo(serialized) => {
                        if let Some(tempo) = parse_tempo_from_serialized(serialized) {
                            pending_tempo_marks.push(PendingTempoMark::Tempo(tempo));
                        }
                        continue;
                    }
                    LyEvent::Mark(serialized) => {
                        pending_tempo_marks.push(PendingTempoMark::Mark(serialized.clone()));
                        continue;
                    }
                    LyEvent::TextMark(serialized) => {
                        pending_tempo_marks.push(PendingTempoMark::TextMark(serialized.clone()));
                        continue;
                    }
                    LyEvent::ChordName(ce) => {
                        harm_counter += 1;
                        // Inline chord name: create Harm with startid if a note
                        // has already been seen, otherwise queue for later
                        if let Some(ref note_id) = last_note_id {
                            let harm = make_harm(ce, note_id, staff_info.n, harm_counter, ext_store);
                            measure.children.push(MeasureChild::Harm(Box::new(harm)));
                        } else {
                            pending_chord_names.push((ce.clone(), staff_info.n));
                        }
                        continue;
                    }
                    LyEvent::FigureEvent(fe) => {
                        fb_counter += 1;
                        let fb = make_fb(fe, staff_info.n, fb_counter, ext_store);
                        measure.children.push(MeasureChild::Fb(Box::new(fb)));
                        continue;
                    }
                    LyEvent::DrumEvent(dn) => {
                        id_counter += 1;
                        let n = convert_drum_note(dn, id_counter, ext_store);
                        let id = format!("ly-note-{}", id_counter);
                        let pe = dn.post_events.clone();
                        layer.children.push(LayerChild::Note(Box::new(n)));
                        (pe, id)
                    }
                    LyEvent::DrumChordEvent(dc) => {
                        id_counter += 1;
                        let n = convert_drum_chord(dc, id_counter, ext_store);
                        let id = format!("ly-note-{}", id_counter);
                        let pe = dc.post_events.clone();
                        layer.children.push(LayerChild::Note(Box::new(n)));
                        (pe, id)
                    }
                    LyEvent::PropertyOp(serialized) => {
                        pending_property_ops.push(serialized.clone());
                        continue;
                    }
                    LyEvent::MusicFunction(fc) => {
                        pending_function_ops.push(fc.clone());
                        continue;
                    }
                    LyEvent::SchemeMusic(serialized) => {
                        pending_scheme_music.push(serialized.clone());
                        continue;
                    }
                    LyEvent::ContextChange { context_type, name } => {
                        // Track cross-staff override; store in ext_store for roundtrip
                        let cc = tusk_model::ContextChange {
                            context_type: context_type.clone(),
                            name: name.clone(),
                        };
                        cross_staff_override = Some(cc);
                        continue;
                    }
                    LyEvent::Clef(_)
                    | LyEvent::KeySig(_)
                    | LyEvent::TimeSig(_)
                    | LyEvent::AutoBeamOn
                    | LyEvent::AutoBeamOff
                    | LyEvent::BarCheck
                    | LyEvent::BarLine(_)
                    | LyEvent::Markup(_)
                    | LyEvent::MarkupList(_) => continue,
                };

                // Set start_id on any pending tuplets/repeats/alternatives
                for pt in &mut pending_tuplets {
                    if pt.start_id.is_empty() {
                        pt.start_id = current_id.clone();
                    }
                }
                for pr in &mut pending_repeats {
                    if pr.start_id.is_empty() {
                        pr.start_id = current_id.clone();
                    }
                }
                for pa in &mut pending_alternatives {
                    if pa.start_id.is_empty() {
                        pa.start_id = current_id.clone();
                    }
                }
                last_note_id = Some(current_id.clone());

                // Apply cross-staff context change if active
                if let Some(ref cc) = cross_staff_override
                    && let Some(id) = get_last_layer_child_id(&layer) {
                        ext_store.insert_context_change(id, cc.clone());
                    }

                // Flush pending inline chord names
                for (ce, staff_n) in pending_chord_names.drain(..) {
                    harm_counter += 1;
                    let harm = make_harm(&ce, &current_id, staff_n, harm_counter, ext_store);
                    measure.children.push(MeasureChild::Harm(Box::new(harm)));
                }

                // Flush pending tempo/mark/textMark events
                for ptm in pending_tempo_marks.drain(..) {
                    tempo_mark_counter += 1;
                    match ptm {
                        PendingTempoMark::Tempo(t) => {
                            let mei_tempo =
                                make_tempo(&t, &current_id, staff_info.n, tempo_mark_counter, ext_store);
                            measure
                                .children
                                .push(MeasureChild::Tempo(Box::new(mei_tempo)));
                        }
                        PendingTempoMark::Mark(s) => {
                            let dir =
                                make_mark_dir(&s, &current_id, staff_info.n, tempo_mark_counter, ext_store);
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        PendingTempoMark::TextMark(s) => {
                            let dir = make_textmark_dir(
                                &s,
                                &current_id,
                                staff_info.n,
                                tempo_mark_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                    }
                }

                // Flush pending property operations
                for prop_serialized in pending_property_ops.drain(..) {
                    artic_counter += 1;
                    let dir = make_property_dir(
                        &prop_serialized,
                        &current_id,
                        staff_info.n,
                        artic_counter,
                        ext_store,
                    );
                    measure.children.push(MeasureChild::Dir(Box::new(dir)));
                }

                // Flush pending music function calls
                for fc in pending_function_ops.drain(..) {
                    func_counter += 1;
                    let dir = make_function_dir(&fc, &current_id, staff_info.n, func_counter, ext_store);
                    measure.children.push(MeasureChild::Dir(Box::new(dir)));
                }

                // Flush pending Scheme music expressions
                for serialized in pending_scheme_music.drain(..) {
                    scm_counter += 1;
                    let dir =
                        make_scheme_music_dir(&serialized, &current_id, staff_info.n, scm_counter, ext_store);
                    measure.children.push(MeasureChild::Dir(Box::new(dir)));
                }

                // Process post-events
                for pe in &post_events {
                    match pe {
                        PostEvent::SlurStart => {
                            pending_slurs.push(PendingSpanner {
                                start_id: current_id.clone(),
                                is_phrase: false,
                                staff_n: staff_info.n,
                            });
                        }
                        PostEvent::SlurEnd => {
                            if let Some(pos) = pending_slurs.iter().rposition(|s| !s.is_phrase) {
                                let pending = pending_slurs.remove(pos);
                                slur_counter += 1;
                                let slur = make_slur(
                                    &pending.start_id,
                                    &current_id,
                                    pending.staff_n,
                                    slur_counter,
                                    false,
                                    ext_store,
                                );
                                measure.children.push(MeasureChild::Slur(Box::new(slur)));
                            }
                        }
                        PostEvent::PhrasingSlurStart => {
                            pending_slurs.push(PendingSpanner {
                                start_id: current_id.clone(),
                                is_phrase: true,
                                staff_n: staff_info.n,
                            });
                        }
                        PostEvent::PhrasingSlurEnd => {
                            if let Some(pos) = pending_slurs.iter().rposition(|s| s.is_phrase) {
                                let pending = pending_slurs.remove(pos);
                                slur_counter += 1;
                                let slur = make_slur(
                                    &pending.start_id,
                                    &current_id,
                                    pending.staff_n,
                                    slur_counter,
                                    true,
                                    ext_store,
                                );
                                measure.children.push(MeasureChild::Slur(Box::new(slur)));
                            }
                        }
                        PostEvent::Dynamic(name) => {
                            dynam_counter += 1;
                            let dynam = make_dynam(name, &current_id, staff_info.n, dynam_counter);
                            measure.children.push(MeasureChild::Dynam(Box::new(dynam)));
                        }
                        PostEvent::Crescendo => {
                            pending_hairpins.push(PendingHairpin {
                                start_id: current_id.clone(),
                                form: "cres".to_string(),
                                staff_n: staff_info.n,
                            });
                        }
                        PostEvent::Decrescendo => {
                            pending_hairpins.push(PendingHairpin {
                                start_id: current_id.clone(),
                                form: "dim".to_string(),
                                staff_n: staff_info.n,
                            });
                        }
                        PostEvent::HairpinEnd => {
                            if let Some(pending) = pending_hairpins.pop() {
                                hairpin_counter += 1;
                                let hairpin = make_hairpin(
                                    &pending.start_id,
                                    &current_id,
                                    pending.staff_n,
                                    &pending.form,
                                    hairpin_counter,
                                );
                                measure
                                    .children
                                    .push(MeasureChild::Hairpin(Box::new(hairpin)));
                            }
                        }
                        PostEvent::Tie => {}
                        PostEvent::Articulation {
                            direction, script, ..
                        } => {
                            artic_counter += 1;
                            let dir = make_artic_dir(
                                script.articulation_name(),
                                *direction,
                                &current_id,
                                staff_info.n,
                                artic_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        PostEvent::NamedArticulation {
                            direction, name, ..
                        } => {
                            if let Some(mc) = make_ornament_control_event(
                                name,
                                *direction,
                                &current_id,
                                staff_info.n,
                                &mut ornam_counter,
                                ext_store,
                            ) {
                                measure.children.push(mc);
                            } else {
                                artic_counter += 1;
                                let dir = make_artic_dir(
                                    name,
                                    *direction,
                                    &current_id,
                                    staff_info.n,
                                    artic_counter,
                                    ext_store,
                                );
                                measure.children.push(MeasureChild::Dir(Box::new(dir)));
                            }
                        }
                        PostEvent::Fingering {
                            direction, digit, ..
                        } => {
                            artic_counter += 1;
                            let dir = make_fing_dir(
                                *digit,
                                *direction,
                                &current_id,
                                staff_info.n,
                                artic_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        PostEvent::StringNumber {
                            direction, number, ..
                        } => {
                            artic_counter += 1;
                            let dir = make_string_dir(
                                *number,
                                *direction,
                                &current_id,
                                staff_info.n,
                                artic_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        PostEvent::BeamStart => {
                            // Record position of this note in the layer
                            beam_starts.push(layer.children.len() - 1);
                        }
                        PostEvent::BeamEnd => {
                            // Match with most recent beam start
                            if let Some(start_pos) = beam_starts.pop() {
                                let end_pos = layer.children.len() - 1;
                                beam_counter += 1;
                                group_beamed_notes(&mut layer, start_pos, end_pos, beam_counter);
                                // Adjust any remaining beam_starts indices
                                // (grouping replaced N items with 1 Beam item)
                                let removed = end_pos - start_pos; // items collapsed
                                for bs in &mut beam_starts {
                                    if *bs > start_pos {
                                        *bs -= removed;
                                    }
                                }
                            }
                        }
                        PostEvent::Tremolo(value) => {
                            wrap_last_in_btrem(&mut layer, *value, &mut ornam_counter, ext_store);
                        }
                        PostEvent::Tweak { path, value } => {
                            // Check for \tweak id #"value" — set xml:id
                            if is_id_tweak(path)
                                && let Some(id_val) = extract_tweak_string_value(value)
                            {
                                set_xml_id_on_last_layer_child(&mut layer, &id_val);
                            }
                            let serialized = crate::serializer::serialize_tweak(path, value);
                            let tweak_info = tusk_model::TweakInfo {
                                path: serialized.clone(),
                                value: tusk_model::ExtValue::String(String::new()),
                            };
                            if let Some(id) = get_last_layer_child_id(&layer) {
                                let mut tweaks = ext_store.tweak_infos(&id).cloned().unwrap_or_default();
                                tweaks.push(tweak_info);
                                ext_store.insert_tweak_infos(id, tweaks);
                            }
                        }
                        PostEvent::TextScript {
                            direction, text, ..
                        } => {
                            text_script_counter += 1;
                            let dir = make_text_script_dir(
                                text,
                                *direction,
                                &current_id,
                                staff_info.n,
                                text_script_counter,
                                ext_store,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        PostEvent::LyricHyphen | PostEvent::LyricExtender => {
                            // Lyric post-events handled in Phase 20.2
                        }
                    }
                }
            }

            // Flush remaining pending chord names (no notes followed them)
            if !pending_chord_names.is_empty() {
                let mut beat = 1.0f64;
                for (ce, staff_n) in pending_chord_names.drain(..) {
                    harm_counter += 1;
                    let mut harm = make_harm(&ce, "", staff_n, harm_counter, ext_store);
                    harm.harm_log.startid = None;
                    harm.harm_log.tstamp = Some(tusk_model::generated::data::DataBeat(beat));
                    measure.children.push(MeasureChild::Harm(Box::new(harm)));
                    if let Some(dur) = &ce.duration {
                        beat += duration_to_beats(dur);
                    }
                }
            }

            // Attach lyrics to notes in this layer
            for (verse_idx, lyric_info) in staff_info.lyrics.iter().enumerate() {
                let verse_n = (verse_idx + 1) as u32;
                lyrics::attach_lyrics_to_layer(&mut layer.children, &lyric_info.syllables, verse_n, ext_store);
                lyrics::refine_wordpos(&mut layer.children, verse_n);
            }

            staff.children.push(StaffChild::Layer(Box::new(layer)));
        }

        measure.children.push(MeasureChild::Staff(Box::new(staff)));
    }

    // Process dedicated ChordNames contexts → Harm control events
    for cn_info in &layout.chord_names {
        let mut cn_events = Vec::new();
        let mut cn_ctx = PitchContext::new();
        collect_events(cn_info.music, &mut cn_events, &mut cn_ctx);
        // Use @tstamp for timing since chord names have no notes to attach to
        let mut beat = 1.0f64; // beat 1 of the measure
        for ev in &cn_events {
            if let LyEvent::ChordName(ce) = ev {
                harm_counter += 1;
                let mut harm = make_harm(ce, "", 1, harm_counter, ext_store);
                // Override: use @tstamp instead of @startid
                harm.harm_log.startid = None;
                harm.harm_log.tstamp = Some(tusk_model::generated::data::DataBeat(beat));
                measure.children.push(MeasureChild::Harm(Box::new(harm)));
                // Advance beat position based on chord duration
                if let Some(dur) = &ce.duration {
                    beat += duration_to_beats(dur);
                }
            }
        }
    }

    // Process dedicated FiguredBass contexts → Fb control events
    for fb_info in &layout.figured_bass {
        let mut fb_events = Vec::new();
        let mut fb_ctx = PitchContext::new();
        collect_events(fb_info.music, &mut fb_events, &mut fb_ctx);
        for ev in &fb_events {
            if let LyEvent::FigureEvent(fe) = ev {
                fb_counter += 1;
                let fb = make_fb(fe, 1, fb_counter, ext_store);
                measure.children.push(MeasureChild::Fb(Box::new(fb)));
            }
        }
    }

    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    Ok(section)
}

use beams::{duration_to_beats, group_beamed_notes};

use control_events::{
    make_artic_dir, make_dynam, make_ending_dir, make_fb, make_fing_dir, make_function_dir,
    make_hairpin, make_harm, make_mark_dir, make_ornament_control_event, make_property_dir,
    make_repeat_dir, make_scheme_music_dir, make_slur, make_string_dir, make_tempo,
    make_text_script_dir, make_textmark_dir, make_tuplet_span, wrap_last_in_btrem,
};

mod utils;
use utils::{
    extract_tweak_string_value, is_id_tweak,
    parse_tempo_from_serialized, set_xml_id_on_last_layer_child,
};

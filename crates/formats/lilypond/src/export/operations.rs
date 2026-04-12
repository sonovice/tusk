//! Property operation and music function call roundtrip for LilyPond export.
//!
//! Collects `<dir>` control events with property op / function call info from ExtensionStore,
//! parses them back into LilyPond AST nodes, and injects them
//! into the items list before their referenced notes.

use std::collections::HashMap;

use tusk_model::elements::MeasureChild;
use tusk_model::extensions::ExtensionStore;

use crate::model::Music;

// ---------------------------------------------------------------------------
// Property operation roundtrip (export)
// ---------------------------------------------------------------------------

/// Collected property operation info: startid → list of Music property ops.
pub(super) struct PropertyOpInfo {
    pub(super) start_id: String,
    pub(super) music: Music,
}

/// Collect property operations from measure `<dir>` elements via ext_store.
pub(super) fn collect_property_ops(measure_children: &[MeasureChild], ext_store: &ExtensionStore) -> Vec<PropertyOpInfo> {
    let mut ops = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let dir_id = match dir.common.xml_id.as_deref() {
                Some(id) => id,
                None => continue,
            };
            if let Some(info) = ext_store.property_op_info(dir_id)
                && let Some(music) = parse_property_op_str(&info.serialized)
            {
                let start_id = dir
                    .dir_log
                    .startid
                    .as_ref()
                    .map(|u| u.0.trim_start_matches('#').to_string())
                    .unwrap_or_default();
                ops.push(PropertyOpInfo { start_id, music });
            }
        }
    }
    ops
}

/// Parse a serialized property operation string back into a Music variant.
fn parse_property_op_str(s: &str) -> Option<Music> {
    use crate::parser::Parser;
    // Wrap in a sequence with a note so the parser can handle it
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                match m {
                    Music::Override { .. }
                    | Music::Revert { .. }
                    | Music::Set { .. }
                    | Music::Unset { .. }
                    | Music::Once { .. } => return Some(m.clone()),
                    _ => {}
                }
            }
        }
        // Bare property op (no sequential wrapper)
        if let crate::model::ToplevelExpression::Music(
            m @ (Music::Override { .. }
            | Music::Revert { .. }
            | Music::Set { .. }
            | Music::Unset { .. }
            | Music::Once { .. }),
        ) = item
        {
            return Some(m.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Music function call roundtrip (export)
// ---------------------------------------------------------------------------

/// Collected music function call info: startid → Music.
#[derive(Clone)]
pub(super) struct FunctionOpInfo {
    pub(super) start_id: String,
    pub(super) music: Music,
}

/// Collect music function calls from measure `<dir>` elements via ext_store.
pub(super) fn collect_function_ops(measure_children: &[MeasureChild], ext_store: &ExtensionStore) -> Vec<FunctionOpInfo> {
    let mut ops = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let dir_id = match dir.common.xml_id.as_deref() {
                Some(id) => id,
                None => continue,
            };
            if let Some(fc) = ext_store.function_call(dir_id) {
                let start_id = dir
                    .dir_log
                    .startid
                    .as_ref()
                    .map(|u| u.0.trim_start_matches('#').to_string())
                    .unwrap_or_default();
                let music = function_call_to_music(fc);
                ops.push(FunctionOpInfo { start_id, music });
            }
        }
    }
    ops
}

/// Collect semantic LilyPond function ops from native MEI control events.
///
/// Uses a tstamp→note_id map as fallback when control events lack @startid
/// (common for MusicXML-originated octave-shift directions).
/// Collect semantic function ops across all measures for cross-measure resolution.
///
/// Ottava spans use `@tstamp2` (e.g. `2m+1.0`) which references a note in a
/// future measure. This function builds tstamp maps for all measures so both
/// start and stop can be resolved.
pub(super) fn collect_semantic_function_ops_global(
    all_measures: &[&tusk_model::elements::Measure],
) -> Vec<FunctionOpInfo> {
    let tstamp_maps: Vec<TstampIdMap> = all_measures
        .iter()
        .map(|m| build_tstamp_id_map(&m.children))
        .collect();
    let mut ops = Vec::new();
    for (measure_idx, measure) in all_measures.iter().enumerate() {
        for mc in &measure.children {
            if let MeasureChild::Octave(octave) = mc {
                ops.extend(octave_to_function_ops(octave, &tstamp_maps, measure_idx));
            }
        }
    }
    ops
}

/// Collect pedal control events into the post-event map.
///
/// Pedal commands (`\sustainOn`, `\sustainOff`, etc.) are post-events in LilyPond
/// (attached after the note), not pre-note functions. Uses tstamp→note_id
/// resolution as fallback when @startid is missing (MusicXML origin).
pub(super) fn collect_pedal_post_events(
    measure_children: &[MeasureChild],
    map: &mut HashMap<String, Vec<crate::model::note::PostEvent>>,
) {
    let tstamp_map = build_tstamp_id_map(measure_children);
    for mc in measure_children {
        if let MeasureChild::Pedal(pedal) = mc {
            if let Some((id, name)) = resolve_pedal_target(pedal, &tstamp_map) {
                map.entry(id)
                    .or_default()
                    .push(crate::model::note::PostEvent::NamedArticulation {
                        direction: crate::model::note::Direction::Neutral,
                        name,
                    });
            }
        }
    }
}

/// Resolve pedal event to (note_id, lilypond_command_name).
fn resolve_pedal_target(
    pedal: &tusk_model::elements::Pedal,
    tstamp_map: &TstampIdMap,
) -> Option<(String, String)> {
    // For pedal stop with tstamp past the last note: attach to the last note
    // in the measure.  The post-processing consolidation pass will then move
    // it to the next \sustainOn note for seamless bracket retakes.
    if pedal.pedal_log.dir.as_deref() == Some("up") {
        if let Some(ts) = pedal.pedal_log.tstamp.as_ref() {
            let staff_n = pedal.pedal_log.staff.as_deref().unwrap_or("1");
            if let Some(entries) = tstamp_map.get(staff_n) {
                if let Some((last_beat, last_id)) = entries.last() {
                    if ts.0 > *last_beat + 0.01 {
                        // Attach to last note instead of skipping
                        return Some((last_id.clone(), "sustainOff".to_string()));
                    }
                }
            }
        }
    }

    let start_id = pedal
        .pedal_log
        .startid
        .as_ref()
        .map(|u| u.0.trim_start_matches('#').to_string())
        .unwrap_or_default();
    let start_id = if start_id.is_empty() {
        resolve_tstamp(
            pedal.pedal_log.tstamp.as_ref(),
            pedal.pedal_log.staff.as_deref(),
            tstamp_map,
        )?
    } else {
        start_id
    };

    let name = match (pedal.pedal_log.func.as_deref(), pedal.pedal_log.dir.as_deref()) {
        (Some("sostenuto"), Some("down")) => "sostenutoOn",
        (Some("sostenuto"), Some("up")) => "sostenutoOff",
        (Some("soft"), Some("down")) => "unaCorda",
        (Some("soft"), Some("up")) => "treCorde",
        (_, Some("down")) => "sustainOn",
        (_, Some("up")) => "sustainOff",
        _ => return None,
    };

    Some((start_id, name.to_string()))
}

fn octave_to_function_ops(
    octave: &tusk_model::elements::Octave,
    tstamp_maps: &[TstampIdMap],
    measure_idx: usize,
) -> Vec<FunctionOpInfo> {
    let start_id = octave
        .octave_log
        .startid
        .as_ref()
        .map(|u| u.0.trim_start_matches('#').to_string())
        .unwrap_or_default();
    // Fallback: resolve tstamp + staff → note ID in current measure
    let start_id = if start_id.is_empty() {
        match resolve_tstamp(
            octave.octave_log.tstamp.as_ref(),
            octave.octave_log.staff.as_deref(),
            tstamp_maps.get(measure_idx).unwrap_or(&HashMap::new()),
        ) {
            Some(id) => id,
            None => return Vec::new(),
        }
    } else {
        start_id
    };

    let mut ops = Vec::new();
    ops.push(FunctionOpInfo {
        start_id,
        music: Music::MusicFunction {
            name: "ottava".to_string(),
            args: vec![crate::model::FunctionArg::Number(dis_to_ottava_number(octave) as f64)],
        },
    });

    // Resolve end: try endid first, then tstamp2 (cross-measure)
    let end_id = octave
        .octave_log
        .endid
        .as_ref()
        .map(|u| u.0.trim_start_matches('#').to_string())
        .filter(|s| !s.is_empty());
    let end_id = end_id.or_else(|| {
        // Parse tstamp2: "Nm+beat" → measure offset N, beat position
        let ts2 = octave.octave_log.tstamp2.as_ref().map(|t| t.0.as_str())?;
        let (offset_str, beat_str) = ts2.split_once("m+")?;
        let offset: usize = offset_str.parse().ok()?;
        let beat: f64 = beat_str.parse().ok()?;
        let target_measure = measure_idx + offset;
        let target_map = tstamp_maps.get(target_measure)?;
        let staff = octave.octave_log.staff.as_deref();
        resolve_tstamp(
            Some(&tusk_model::data::DataBeat::from(beat)),
            staff,
            target_map,
        )
    });

    if let Some(eid) = end_id {
        ops.push(FunctionOpInfo {
            start_id: eid,
            music: Music::MusicFunction {
                name: "ottava".to_string(),
                args: vec![crate::model::FunctionArg::Number(0.0)],
            },
        });
    }

    ops
}

fn dis_to_ottava_number(octave: &tusk_model::elements::Octave) -> i32 {
    let steps = octave.octave_log.dis.as_ref().map(|d| d.0).unwrap_or(8);
    let magnitude = match steps {
        8 => 1,
        15 => 2,
        22 => 3,
        n => ((n as i64 - 1) / 7).max(1) as i32,
    };
    match octave.octave_log.dis_place {
        Some(tusk_model::data::DataStaffrelBasic::Below) => -magnitude,
        _ => magnitude,
    }
}


/// Convert a typed `FunctionCall` back into a LilyPond `Music` variant.
fn function_call_to_music(fc: &tusk_model::FunctionCall) -> Music {
    let args: Vec<crate::model::FunctionArg> =
        fc.args.iter().map(ext_value_to_function_arg).collect();
    if fc.is_partial {
        Music::PartialFunction {
            name: fc.name.clone(),
            args,
        }
    } else {
        Music::MusicFunction {
            name: fc.name.clone(),
            args,
        }
    }
}

/// Convert an `ExtValue` back into a LilyPond `FunctionArg`.
fn ext_value_to_function_arg(val: &tusk_model::ExtValue) -> crate::model::FunctionArg {
    use crate::model::FunctionArg;
    match val {
        tusk_model::ExtValue::Music(s) => {
            // Parse serialized music back into AST.
            // Strip slur events — the global slur map handles slur attachment
            // separately, so keeping them here would duplicate slurs.
            if let Some(mut m) = parse_music_str(s) {
                strip_slur_events(&mut m);
                FunctionArg::Music(m)
            } else {
                FunctionArg::Music(Music::Unparsed(s.clone()))
            }
        }
        tusk_model::ExtValue::String(s) => FunctionArg::String(s.clone()),
        tusk_model::ExtValue::Number(n) => FunctionArg::Number(*n),
        tusk_model::ExtValue::Scheme(s) => {
            if let Some(expr) = parse_scheme_str(s) {
                FunctionArg::SchemeExpr(expr)
            } else {
                FunctionArg::SchemeExpr(crate::model::SchemeExpr::Raw(s.clone()))
            }
        }
        tusk_model::ExtValue::Duration(base, dots) => {
            FunctionArg::Duration(crate::model::Duration {
                base: *base,
                dots: *dots,
                multipliers: vec![],
            })
        }
        tusk_model::ExtValue::Identifier(name) => FunctionArg::Identifier(name.clone()),
        tusk_model::ExtValue::Default => FunctionArg::Default,
        tusk_model::ExtValue::SymbolList(segments) => FunctionArg::SymbolList(segments.clone()),
        tusk_model::ExtValue::Bool(b) => {
            FunctionArg::SchemeExpr(crate::model::SchemeExpr::Bool(*b))
        }
        tusk_model::ExtValue::Markup(s) => {
            // Store as scheme-like form for roundtrip
            FunctionArg::Music(Music::Unparsed(s.clone()))
        }
        tusk_model::ExtValue::MarkupList(s) => FunctionArg::Music(Music::Unparsed(s.clone())),
    }
}

/// Remove SlurStart/SlurEnd/PhrasingSlurStart/PhrasingSlurEnd from post_events
/// in a Music tree. These are handled by the global slur map during export;
/// keeping them in re-parsed function args would duplicate slurs.
fn strip_slur_events(m: &mut Music) {
    use crate::model::PostEvent;
    fn strip_pe(events: &mut Vec<PostEvent>) {
        events.retain(|pe| !matches!(pe,
            PostEvent::SlurStart | PostEvent::SlurEnd |
            PostEvent::DirectedSlurStart(_) | PostEvent::DirectedSlurEnd(_) |
            PostEvent::PhrasingSlurStart | PostEvent::PhrasingSlurEnd |
            PostEvent::DirectedPhrasingSlurStart(_) | PostEvent::DirectedPhrasingSlurEnd(_)
        ));
    }
    match m {
        Music::Note(n) => strip_pe(&mut n.post_events),
        Music::Chord(c) => strip_pe(&mut c.post_events),
        Music::Rest(r) => strip_pe(&mut r.post_events),
        Music::Skip(s) => strip_pe(&mut s.post_events),
        Music::MultiMeasureRest(r) => strip_pe(&mut r.post_events),
        Music::ChordRepetition(cr) => strip_pe(&mut cr.post_events),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items { strip_slur_events(item); }
        }
        Music::Relative { body, .. } | Music::Fixed { body, .. }
        | Music::Grace { body } | Music::Acciaccatura { body }
        | Music::Appoggiatura { body } | Music::Tuplet { body, .. }
        | Music::Transpose { body, .. } | Music::Once { music: body }
        | Music::ContextedMusic { music: body, .. } => strip_slur_events(body),
        Music::Repeat { body, alternatives, .. } => {
            strip_slur_events(body);
            if let Some(alts) = alternatives {
                for alt in alts { strip_slur_events(alt); }
            }
        }
        Music::AfterGrace { main, grace, .. } => {
            strip_slur_events(main);
            strip_slur_events(grace);
        }
        _ => {}
    }
}

/// Parse a serialized music string back into a Music variant.
fn parse_music_str(s: &str) -> Option<Music> {
    use crate::parser::Parser;
    let file = Parser::new(s).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(m) = item {
            return Some(m.clone());
        }
    }
    None
}

/// Parse a serialized Scheme expression string back into a SchemeExpr.
fn parse_scheme_str(s: &str) -> Option<crate::model::SchemeExpr> {
    use crate::parser::Parser;
    // Wrap as assignment value so the parser can handle it
    let src = format!("x = {s}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Assignment(a) = item
            && let crate::model::AssignmentValue::SchemeExpr(expr) = &a.value
        {
            return Some(expr.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Shared injection logic
// ---------------------------------------------------------------------------

/// Insertion log entry: (position, count) in descending position order.
///
/// Used to synchronize parallel arrays (like grace_types) that weren't
/// directly updated during injection.
pub(super) type InsertionLog = Vec<(usize, usize)>;

/// Inject items from a startid→Music map into the items list before their referenced notes.
///
/// Also updates `item_ids` in parallel (inserting `None` for each injected item)
/// so that subsequent operations (tuplet/grace/repeat wrapping) see correct positions.
///
/// Returns an insertion log: `(position, count)` pairs in descending position order,
/// which callers can use to update other parallel arrays.
fn inject_ops_by_startid(
    items: &mut Vec<Music>,
    item_ids: &mut Vec<Option<String>>,
    ops: &[(String, Music)],
) -> InsertionLog {
    if ops.is_empty() {
        return Vec::new();
    }
    // Build a map of id → list of items to inject before that id
    let mut inject_map: HashMap<String, Vec<Music>> = HashMap::new();
    for (start_id, music) in ops {
        if !start_id.is_empty() {
            inject_map
                .entry(start_id.clone())
                .or_default()
                .push(music.clone());
        }
    }

    // Walk items in reverse to avoid index shifting
    let mut insertions: Vec<(usize, Vec<Music>)> = Vec::new();
    for (i, id) in item_ids.iter().enumerate() {
        if let Some(id_str) = id
            && let Some(items_to_inject) = inject_map.remove(id_str.as_str())
        {
            insertions.push((i, items_to_inject));
        }
    }
    // Sort by position descending so we insert from back to front
    insertions.sort_by(|a, b| b.0.cmp(&a.0));
    let mut log = Vec::with_capacity(insertions.len());
    for (pos, inject_items) in insertions {
        if pos <= items.len() {
            let count = inject_items.len();
            for (j, op) in inject_items.into_iter().enumerate() {
                items.insert(pos + j, op);
                item_ids.insert(pos + j, None);
            }
            log.push((pos, count));
        }
    }
    log
}

/// Inject property operations into the items list before their referenced notes.
pub(super) fn inject_property_ops(
    items: &mut Vec<Music>,
    item_ids: &mut Vec<Option<String>>,
    ops: &[PropertyOpInfo],
) -> InsertionLog {
    let pairs: Vec<(String, Music)> = ops
        .iter()
        .map(|op| (op.start_id.clone(), op.music.clone()))
        .collect();
    inject_ops_by_startid(items, item_ids, &pairs)
}

/// Inject music function calls into the items list before their referenced notes.
pub(super) fn inject_function_ops(
    items: &mut Vec<Music>,
    item_ids: &mut Vec<Option<String>>,
    ops: &[FunctionOpInfo],
) -> InsertionLog {
    let pairs: Vec<(String, Music)> = ops
        .iter()
        .map(|op| (op.start_id.clone(), op.music.clone()))
        .collect();
    inject_ops_by_startid(items, item_ids, &pairs)
}

// ---------------------------------------------------------------------------
// Scheme music expressions
// ---------------------------------------------------------------------------

/// Collected Scheme music op: startid → Music.
pub(super) struct SchemeMusicOp {
    pub(super) start_id: String,
    pub(super) music: Music,
}

/// Collect Scheme music expressions from measure `<dir>` elements via ext_store.
pub(super) fn collect_scheme_music_ops(measure_children: &[MeasureChild], ext_store: &ExtensionStore) -> Vec<SchemeMusicOp> {
    let mut ops = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let dir_id = match dir.common.xml_id.as_deref() {
                Some(id) => id,
                None => continue,
            };
            if let Some(info) = ext_store.scheme_music_info(dir_id) {
                let start_id = dir
                    .dir_log
                    .startid
                    .as_ref()
                    .map(|u| u.0.trim_start_matches('#').to_string())
                    .unwrap_or_default();
                // Parse the serialized Scheme expression back into a SchemeExpr
                let expr = if let Some(e) = parse_scheme_str(&info.serialized) {
                    e
                } else {
                    crate::model::SchemeExpr::Raw(info.serialized.clone())
                };
                let music = Music::SchemeMusic(expr);
                ops.push(SchemeMusicOp { start_id, music });
            }
        }
    }
    ops
}

/// Inject Scheme music expressions into the items list before their referenced notes.
pub(super) fn inject_scheme_music_ops(
    items: &mut Vec<Music>,
    item_ids: &mut Vec<Option<String>>,
    ops: &[SchemeMusicOp],
) -> InsertionLog {
    let pairs: Vec<(String, Music)> = ops
        .iter()
        .map(|op| (op.start_id.clone(), op.music.clone()))
        .collect();
    inject_ops_by_startid(items, item_ids, &pairs)
}

// ---------------------------------------------------------------------------
// Tstamp → note ID resolution
// ---------------------------------------------------------------------------

/// Map from staff number string to sorted list of (tstamp, note_xml_id).
type TstampIdMap = HashMap<String, Vec<(f64, String)>>;

/// Build a tstamp→note_id map from a measure's staff/layer structure.
///
/// Walks through each staff's first layer, computing cumulative beat positions
/// (1-indexed, in quarter-note units) for each note/chord. Returns a map keyed
/// by MEI staff number.
fn build_tstamp_id_map(measure_children: &[MeasureChild]) -> TstampIdMap {
    use tusk_model::elements::{LayerChild, MeasureChild as MC, StaffChild};
    let mut map = TstampIdMap::new();
    for mc in measure_children {
        let MC::Staff(staff) = mc else { continue };
        let staff_n = staff.n_integer.n.clone().unwrap_or_default();
        // Use first layer (primary voice) for tstamp resolution
        let Some(StaffChild::Layer(layer)) = staff.children.first() else { continue };
        let mut beat = 1.0_f64; // MEI tstamp is 1-based
        let mut entries = Vec::new();
        for lc in &layer.children {
            if let Some((id, dur_quarters)) = layer_child_beat_info(lc) {
                entries.push((beat, id));
                beat += dur_quarters;
            } else if let LayerChild::Beam(beam) = lc {
                for bc in &beam.children {
                    if let Some((id, dur_q)) = beam_child_beat_info(bc) {
                        entries.push((beat, id));
                        beat += dur_q;
                    }
                }
            }
        }
        map.insert(staff_n, entries);
    }
    map
}

/// Extract (xml_id, duration_in_quarters) from a LayerChild.
/// Grace notes are skipped — they don't consume beats.
fn layer_child_beat_info(lc: &tusk_model::elements::LayerChild) -> Option<(String, f64)> {
    use tusk_model::elements::LayerChild;
    match lc {
        LayerChild::Note(n) => {
            if n.note_log.grace.is_some() {
                return None; // grace notes don't advance beat position
            }
            let id = n.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters(n.note_log.dur.as_ref()?, n.note_log.dots.as_ref());
            Some((id, q))
        }
        LayerChild::Chord(c) => {
            if c.chord_log.grace.is_some() {
                return None;
            }
            let id = c.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters(c.chord_log.dur.as_ref()?, c.chord_log.dots.as_ref());
            Some((id, q))
        }
        LayerChild::Rest(r) => {
            let id = r.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters_rest(r.rest_log.dur.as_ref()?, r.rest_log.dots.as_ref());
            Some((id, q))
        }
        LayerChild::Space(s) => {
            let id = s.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters(s.space_log.dur.as_ref()?, s.space_log.dots.as_ref());
            Some((id, q))
        }
        _ => None,
    }
}

/// Extract (xml_id, duration_in_quarters) from a BeamChild.
fn beam_child_beat_info(bc: &tusk_model::elements::BeamChild) -> Option<(String, f64)> {
    use tusk_model::elements::BeamChild;
    match bc {
        BeamChild::Note(n) => {
            if n.note_log.grace.is_some() { return None; }
            let id = n.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters(n.note_log.dur.as_ref()?, n.note_log.dots.as_ref());
            Some((id, q))
        }
        BeamChild::Chord(c) => {
            if c.chord_log.grace.is_some() { return None; }
            let id = c.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters(c.chord_log.dur.as_ref()?, c.chord_log.dots.as_ref());
            Some((id, q))
        }
        BeamChild::Rest(r) => {
            let id = r.common.xml_id.as_ref()?.clone();
            let q = dur_to_quarters_rest(r.rest_log.dur.as_ref()?, r.rest_log.dots.as_ref());
            Some((id, q))
        }
        _ => None,
    }
}

/// Convert MEI DataDuration + dots to quarter-note duration.
fn dur_to_quarters(
    dur: &tusk_model::generated::data::DataDuration,
    dots: Option<&tusk_model::generated::data::DataAugmentdot>,
) -> f64 {
    use tusk_model::generated::data::{DataDuration, DataDurationCmn};
    let base = match dur {
        DataDuration::MeiDataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => 16.0,
            DataDurationCmn::Breve => 8.0,
            DataDurationCmn::N1 => 4.0,
            DataDurationCmn::N2 => 2.0,
            DataDurationCmn::N4 => 1.0,
            DataDurationCmn::N8 => 0.5,
            DataDurationCmn::N16 => 0.25,
            DataDurationCmn::N32 => 0.125,
            DataDurationCmn::N64 => 0.0625,
            _ => 1.0,
        },
        _ => 1.0,
    };
    apply_dots(base, dots)
}

/// Convert MEI DataDurationrests + dots to quarter-note duration.
fn dur_to_quarters_rest(
    dur: &tusk_model::generated::data::DataDurationrests,
    dots: Option<&tusk_model::generated::data::DataAugmentdot>,
) -> f64 {
    use tusk_model::generated::data::{DataDurationCmn, DataDurationrests};
    let base = match dur {
        DataDurationrests::MeiDataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => 16.0,
            DataDurationCmn::Breve => 8.0,
            DataDurationCmn::N1 => 4.0,
            DataDurationCmn::N2 => 2.0,
            DataDurationCmn::N4 => 1.0,
            DataDurationCmn::N8 => 0.5,
            DataDurationCmn::N16 => 0.25,
            DataDurationCmn::N32 => 0.125,
            DataDurationCmn::N64 => 0.0625,
            _ => 1.0,
        },
        _ => 1.0,
    };
    apply_dots(base, dots)
}

fn apply_dots(base: f64, dots: Option<&tusk_model::generated::data::DataAugmentdot>) -> f64 {
    let dot_count = dots.map(|d| d.0).unwrap_or(0);
    let mut total = base;
    let mut addition = base / 2.0;
    for _ in 0..dot_count {
        total += addition;
        addition /= 2.0;
    }
    total
}

/// Resolve a tstamp + staff to a note xml:id using the tstamp map.
///
/// Finds the last note whose beat position is ≤ the given tstamp
/// (the note sounding at that time). Falls back to nearest note
/// if none is at or before the tstamp.
fn resolve_tstamp(
    tstamp: Option<&tusk_model::data::DataBeat>,
    staff: Option<&str>,
    map: &TstampIdMap,
) -> Option<String> {
    let ts = tstamp?.0;
    let staff_n = staff.unwrap_or("1");
    let entries = map.get(staff_n)?;
    // Find last note at or before the tstamp (the note sounding at that time)
    entries
        .iter()
        .filter(|(beat, _)| *beat <= ts + 0.01)
        .last()
        .or_else(|| entries.first()) // fallback: first note if tstamp precedes all
        .map(|(_, id)| id.clone())
}

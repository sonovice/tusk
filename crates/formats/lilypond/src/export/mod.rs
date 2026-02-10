//! Conversion from MEI to LilyPond AST.

mod conversion;
mod pitch_context;
mod signatures;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use thiserror::Error;
use tusk_model::elements::{
    LayerChild, MeasureChild, Mei, MeiChild, ScoreChild, ScoreDefChild, SectionChild, StaffGrpChild,
};

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
    // Find the Music -> Body -> Mdiv -> Score path
    let score = find_score(mei).ok_or(ExportError::NoMusic)?;

    // Extract staffGrp metadata for context reconstruction
    let group_meta = extract_group_meta(score);
    let staff_metas = extract_staff_metas(score);

    // Extract event sequences from staffDef labels (for clef/key/time roundtrip)
    let event_sequences = extract_event_sequences(score);

    // Extract pitch context labels (relative/transpose) from staffDefs
    let pitch_contexts = extract_pitch_contexts(score);

    // Walk section -> measures -> staves -> layers -> notes/rests
    let mut staff_music: Vec<Vec<Vec<Music>>> = Vec::new(); // staff -> layer -> items

    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for section_child in &section.children {
                if let SectionChild::Measure(measure) = section_child {
                    // Collect control events → note-id post-event map
                    let mut post_event_map = collect_slur_post_events(&measure.children);
                    collect_dynam_post_events(&measure.children, &mut post_event_map);
                    collect_hairpin_post_events(&measure.children, &mut post_event_map);
                    collect_artic_post_events(&measure.children, &mut post_event_map);
                    collect_ornament_post_events(&measure.children, &mut post_event_map);

                    // Collect tuplet spans for wrapping
                    let tuplet_spans = collect_tuplet_spans(&measure.children);

                    let mut staff_idx = 0usize;
                    for mc in &measure.children {
                        if let MeasureChild::Staff(staff) = mc {
                            let mut layers: Vec<Vec<Music>> = Vec::new();
                            for sc in &staff.children {
                                let tusk_model::elements::StaffChild::Layer(layer) = sc;
                                let mut items = Vec::new();
                                let mut item_ids = Vec::new();
                                for lc in &layer.children {
                                    let start = items.len();
                                    convert_layer_child_to_items(lc, &post_event_map, &mut items);
                                    // Track xml:ids for newly added items
                                    collect_layer_child_ids(lc, &mut item_ids, items.len() - start);
                                }
                                // Wrap tuplet ranges in Music::Tuplet
                                apply_tuplet_wrapping(&mut items, &item_ids, &tuplet_spans);
                                layers.push(items);
                            }

                            // Inject clef/key/time events from the event sequence
                            if let Some(seq) = event_sequences.get(staff_idx) {
                                inject_signature_events(&mut layers, seq);
                            }

                            staff_music.push(layers);
                            staff_idx += 1;
                        }
                    }
                }
            }
        }
    }

    // Apply pitch context wrappers (relative/transpose) to each staff's music
    apply_pitch_contexts(&mut staff_music, &pitch_contexts);

    // Build music expression from collected layers, wrapping in contexts
    let music = build_music_with_contexts(staff_music, &group_meta, &staff_metas);

    let score_block = ScoreBlock {
        items: vec![ScoreItem::Music(music)],
    };

    Ok(LilyPondFile {
        version: Some(Version {
            version: "2.24.0".to_string(),
        }),
        items: vec![ToplevelExpression::Score(score_block)],
    })
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
}

/// Extract group metadata from scoreDef's staffGrp.
fn extract_group_meta(score: &tusk_model::elements::Score) -> Option<GroupMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    // Check label for group info
                    if let Some(rest) = grp
                        .common
                        .label
                        .as_deref()
                        .and_then(|l| l.strip_prefix("lilypond:group,"))
                    {
                        return Some(parse_context_label(rest));
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
                            let meta = if let Some(label) = &sdef.labelled.label {
                                if let Some(rest) = label.strip_prefix("lilypond:staff,") {
                                    let gm = parse_context_label(rest);
                                    StaffMeta {
                                        context_type: gm.context_type,
                                        name: gm.name,
                                        with_block_str: gm.with_block_str,
                                    }
                                } else {
                                    StaffMeta {
                                        context_type: "Staff".to_string(),
                                        name: None,
                                        with_block_str: None,
                                    }
                                }
                            } else {
                                StaffMeta {
                                    context_type: "Staff".to_string(),
                                    name: None,
                                    with_block_str: None,
                                }
                            };
                            metas.push(meta);
                        }
                    }
                }
            }
        }
    }
    metas
}

/// Parse a context label string into metadata.
///
/// Format: `ContextType[,name=Name][,with=...]`
fn parse_context_label(s: &str) -> GroupMeta {
    let mut context_type = String::new();
    let mut name = None;
    let mut with_block_str = None;

    // Split carefully -- the "with=" part may contain commas in its content
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_with = false;

    for c in s.chars() {
        if in_with {
            current.push(c);
        } else if c == ',' {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(c);
            if current == "with=" {
                in_with = true;
            }
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            context_type = part.clone();
        } else if let Some(n) = part.strip_prefix("name=") {
            name = Some(n.to_string());
        } else if let Some(w) = part.strip_prefix("with=") {
            with_block_str = Some(w.to_string());
        }
    }

    GroupMeta {
        context_type,
        name,
        with_block_str,
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
fn build_music_with_contexts(
    staff_music: Vec<Vec<Vec<Music>>>,
    group_meta: &Option<GroupMeta>,
    staff_metas: &[StaffMeta],
) -> Music {
    let num_staves = staff_music.len();

    // Single staff, no group, no explicit staff context -> flat output
    if num_staves <= 1
        && group_meta.is_none()
        && (staff_metas.is_empty()
            || (staff_metas.len() == 1
                && staff_metas[0].name.is_none()
                && staff_metas[0].with_block_str.is_none()
                && staff_metas[0].context_type == "Staff"))
    {
        return build_flat_music(staff_music);
    }

    // Build per-staff music with \new Staff wrappers
    let mut staff_exprs: Vec<Music> = Vec::new();
    for (i, layers) in staff_music.into_iter().enumerate() {
        let inner = build_layers_music(layers);
        let meta = staff_metas.get(i);

        let with_block = meta
            .and_then(|m| m.with_block_str.as_deref())
            .and_then(parse_with_block_str);

        let staff_music_expr = Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: meta
                .map(|m| m.context_type.clone())
                .unwrap_or_else(|| "Staff".to_string()),
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
/// Label format: `lilypond:tuplet,NUM/DENOM[,span=DUR]`
/// Duration format: `BASE[.DOTS][*N][*N/M]`
fn parse_tuplet_span_duration(label: &str) -> Option<Duration> {
    let rest = label.strip_prefix("lilypond:tuplet,")?;
    // Find span= part
    let span_str = rest.split(',').find_map(|p| p.strip_prefix("span="))?;
    parse_duration_from_label(span_str)
}

/// Parse a compact duration string from a label.
///
/// Format: `BASE[.DOTS][*N][*N/M]`
fn parse_duration_from_label(s: &str) -> Option<Duration> {
    let mut chars = s.chars().peekable();

    // Parse base (digits before first dot or *)
    let mut base_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            base_str.push(c);
            chars.next();
        } else {
            break;
        }
    }
    let base: u32 = base_str.parse().ok()?;

    // Parse dots
    let mut dots = 0u8;
    while let Some(&'.') = chars.peek() {
        dots += 1;
        chars.next();
    }

    // Parse multipliers
    let mut multipliers = Vec::new();
    while let Some(&'*') = chars.peek() {
        chars.next();
        let mut num_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                chars.next();
            } else {
                break;
            }
        }
        let num: u32 = num_str.parse().ok()?;

        if let Some(&'/') = chars.peek() {
            chars.next();
            let mut den_str = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    den_str.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let den: u32 = den_str.parse().ok()?;
            multipliers.push((num, den));
        } else {
            multipliers.push((num, 1));
        }
    }

    Some(Duration {
        base,
        dots,
        multipliers,
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
                .is_some_and(|l| l == "lilypond:phrase");

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
/// with `lilypond:artic,`, `lilypond:fing,`, or `lilypond:string,` labels.
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

            if let Some(rest) = label.strip_prefix("lilypond:artic,")
                && let Some(pe) = parse_artic_label(rest)
            {
                map.entry(startid).or_default().push(pe);
            } else if let Some(rest) = label.strip_prefix("lilypond:fing,")
                && let Some(pe) = parse_fing_label(rest)
            {
                map.entry(startid).or_default().push(pe);
            } else if let Some(rest) = label.strip_prefix("lilypond:string,")
                && let Some(pe) = parse_string_label(rest)
            {
                map.entry(startid).or_default().push(pe);
            }
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
                    let direction = trill
                        .common
                        .label
                        .as_deref()
                        .map(parse_ornament_direction)
                        .unwrap_or(Direction::Neutral);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation {
                            direction,
                            name: "trill".to_string(),
                        });
                }
            }
            MeasureChild::Mordent(mordent) => {
                if let Some(ref startid) = mordent.mordent_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let direction = mordent
                        .common
                        .label
                        .as_deref()
                        .map(parse_ornament_direction)
                        .unwrap_or(Direction::Neutral);
                    let name = match mordent.mordent_log.form.as_deref() {
                        Some("upper") => "prall",
                        _ => "mordent",
                    };
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation {
                            direction,
                            name: name.to_string(),
                        });
                }
            }
            MeasureChild::Turn(turn) => {
                if let Some(ref startid) = turn.turn_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let direction = turn
                        .common
                        .label
                        .as_deref()
                        .map(parse_ornament_direction)
                        .unwrap_or(Direction::Neutral);
                    let name = match turn.turn_log.form.as_deref() {
                        Some("lower") => "reverseturn",
                        _ => "turn",
                    };
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation {
                            direction,
                            name: name.to_string(),
                        });
                }
            }
            MeasureChild::Fermata(fermata) => {
                if let Some(ref startid) = fermata.fermata_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let (name, direction) = parse_fermata_label(fermata);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation {
                            direction,
                            name: name.to_string(),
                        });
                }
            }
            MeasureChild::Ornam(ornam) => {
                if let Some(ref startid) = ornam.ornam_log.startid {
                    let id = startid.0.trim_start_matches('#').to_string();
                    let (name, direction) = parse_ornam_label(ornam);
                    map.entry(id)
                        .or_default()
                        .push(PostEvent::NamedArticulation { direction, name });
                }
            }
            _ => {}
        }
    }
}

/// Parse direction from an ornament label (e.g., "lilypond:trill,dir=up").
fn parse_ornament_direction(label: &str) -> Direction {
    if label.ends_with(",dir=up") {
        Direction::Up
    } else if label.ends_with(",dir=down") {
        Direction::Down
    } else {
        Direction::Neutral
    }
}

/// Parse fermata variant and direction from a Fermata element's label.
fn parse_fermata_label(fermata: &tusk_model::elements::Fermata) -> (&str, Direction) {
    if let Some(label) = fermata.common.label.as_deref() {
        if let Some(rest) = label.strip_prefix("lilypond:fermata,") {
            let (name, dir) = split_label_direction(rest);
            return (name, dir);
        }
        // Label exists but no variant prefix — check for direction only
        let dir = parse_ornament_direction(label);
        return ("fermata", dir);
    }
    ("fermata", Direction::Neutral)
}

/// Parse ornament name and direction from an Ornam element's label.
fn parse_ornam_label(ornam: &tusk_model::elements::Ornam) -> (String, Direction) {
    if let Some(label) = ornam.common.label.as_deref()
        && let Some(rest) = label.strip_prefix("lilypond:ornam,")
    {
        let (name, dir) = split_label_direction(rest);
        return (name.to_string(), dir);
    }
    // Fallback: use text content
    let name = ornam
        .children
        .iter()
        .map(|c| {
            let tusk_model::elements::OrnamChild::Text(t) = c;
            t.clone()
        })
        .next()
        .unwrap_or_else(|| "ornam".to_string());
    (name, Direction::Neutral)
}

/// Convert an MEI BTrem (bowed tremolo) to a LilyPond Music expression.
///
/// Extracts the inner note/chord and adds a `PostEvent::Tremolo` with the
/// subdivision value restored from the label.
fn convert_mei_btrem(btrem: &tusk_model::elements::BTrem) -> Music {
    // Restore subdivision value from label
    let value = btrem
        .common
        .label
        .as_deref()
        .and_then(|l| l.strip_prefix("lilypond:tremolo,"))
        .and_then(|v| v.parse::<u32>().ok())
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

/// Parse direction suffix from a label part.
fn parse_direction(s: &str) -> Direction {
    match s {
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => Direction::Neutral,
    }
}

/// Parse an `artic` label: `NAME[,dir=up|down]` → PostEvent.
fn parse_artic_label(label: &str) -> Option<PostEvent> {
    let (name, dir) = split_label_direction(label);
    // Try to map to script abbreviation first
    if let Some(script) = name_to_script_abbreviation(name) {
        Some(PostEvent::Articulation {
            direction: dir,
            script,
        })
    } else {
        Some(PostEvent::NamedArticulation {
            direction: dir,
            name: name.to_string(),
        })
    }
}

/// Parse a `fing` label: `DIGIT[,dir=up|down]` → PostEvent::Fingering.
fn parse_fing_label(label: &str) -> Option<PostEvent> {
    let (digit_str, dir) = split_label_direction(label);
    let digit: u8 = digit_str.parse().ok()?;
    Some(PostEvent::Fingering {
        direction: dir,
        digit,
    })
}

/// Parse a `string` label: `NUMBER[,dir=up|down]` → PostEvent::StringNumber.
fn parse_string_label(label: &str) -> Option<PostEvent> {
    let (num_str, dir) = split_label_direction(label);
    let number: u8 = num_str.parse().ok()?;
    Some(PostEvent::StringNumber {
        direction: dir,
        number,
    })
}

/// Split a label value into (main_value, Direction).
///
/// Input: `"staccato,dir=up"` → `("staccato", Direction::Up)`
/// Input: `"staccato"` → `("staccato", Direction::Neutral)`
fn split_label_direction(label: &str) -> (&str, Direction) {
    if let Some((main, rest)) = label.split_once(",dir=") {
        (main, parse_direction(rest))
    } else {
        (label, Direction::Neutral)
    }
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
        Music::MultiMeasureRest(mrest) => mrest.post_events.extend(events.iter().cloned()),
        _ => {}
    }
}

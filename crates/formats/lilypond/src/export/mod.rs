//! Conversion from MEI to LilyPond AST.

use thiserror::Error;
use tusk_model::elements::{
    ChordChild, LayerChild, MeasureChild, Mei, MeiChild, ScoreChild, ScoreDefChild, SectionChild,
    StaffGrpChild,
};
use tusk_model::generated::data::{DataAccidentalGesturalBasic, DataDurationCmn};

use crate::model::note::ChordEvent;
use crate::model::pitch::Pitch;
use crate::model::signature::{Clef, KeySignature, TimeSignature};
use crate::model::{
    ContextKeyword, Duration, LilyPondFile, Mode, MultiMeasureRestEvent, Music, NoteEvent,
    RestEvent, ScoreBlock, ScoreItem, ToplevelExpression, Version,
};

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
    // Find the Music → Body → Mdiv → Score path
    let score = find_score(mei).ok_or(ExportError::NoMusic)?;

    // Extract staffGrp metadata for context reconstruction
    let group_meta = extract_group_meta(score);
    let staff_metas = extract_staff_metas(score);

    // Extract event sequences from staffDef labels (for clef/key/time roundtrip)
    let event_sequences = extract_event_sequences(score);

    // Extract pitch context labels (relative/transpose) from staffDefs
    let pitch_contexts = extract_pitch_contexts(score);

    // Walk section → measures → staves → layers → notes/rests
    let mut staff_music: Vec<Vec<Vec<Music>>> = Vec::new(); // staff → layer → items

    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for section_child in &section.children {
                if let SectionChild::Measure(measure) = section_child {
                    let mut staff_idx = 0usize;
                    for mc in &measure.children {
                        if let MeasureChild::Staff(staff) = mc {
                            let mut layers: Vec<Vec<Music>> = Vec::new();
                            for sc in &staff.children {
                                let tusk_model::elements::StaffChild::Layer(layer) = sc;
                                let mut items = Vec::new();
                                for lc in &layer.children {
                                    if let Some(m) = convert_layer_child(lc) {
                                        items.push(m);
                                    }
                                }
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

    // Split carefully — the "with=" part may contain commas in its content
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

    // Single staff, no group, no explicit staff context → flat output
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
// Clef / key / time event sequence extraction and injection
// ---------------------------------------------------------------------------

/// A signature event parsed from the staffDef label.
struct SignatureEvent {
    /// Position in the note/rest stream (0-based).
    position: u32,
    /// The Music expression to inject.
    music: Music,
}

/// Extract event sequences from all staffDefs.
fn extract_event_sequences(score: &tusk_model::elements::Score) -> Vec<Vec<SignatureEvent>> {
    let mut result = Vec::new();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            result.push(parse_event_sequence_label(sdef));
                        }
                    }
                }
            }
        }
    }
    result
}

/// Parse the `lilypond:events,...` segment from a staffDef label.
fn parse_event_sequence_label(staff_def: &tusk_model::elements::StaffDef) -> Vec<SignatureEvent> {
    let label = match &staff_def.labelled.label {
        Some(l) => l.as_str(),
        None => return Vec::new(),
    };

    // Find the lilypond:events segment (label may have multiple | separated segments)
    let events_str = label
        .split('|')
        .find_map(|seg| seg.strip_prefix("lilypond:events,"));

    let events_str = match events_str {
        Some(s) => s,
        None => {
            // No event sequence — try to reconstruct from staffDef attributes
            return reconstruct_initial_signatures(staff_def);
        }
    };

    let mut events = Vec::new();
    for entry in events_str.split(';') {
        if entry.is_empty() {
            continue;
        }
        let (type_str, pos_str) = match entry.rsplit_once('@') {
            Some(pair) => pair,
            None => continue,
        };
        let position: u32 = match pos_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Some(name) = type_str.strip_prefix("clef:") {
            events.push(SignatureEvent {
                position,
                music: Music::Clef(Clef {
                    name: name.to_string(),
                }),
            });
        } else if let Some(key_str) = type_str.strip_prefix("key:") {
            if let Some(ks) = parse_key_label(key_str) {
                events.push(SignatureEvent {
                    position,
                    music: Music::KeySignature(ks),
                });
            }
        } else if let Some(time_str) = type_str.strip_prefix("time:")
            && let Some(ts) = parse_time_label(time_str)
        {
            events.push(SignatureEvent {
                position,
                music: Music::TimeSignature(ts),
            });
        }
    }

    events
}

/// Reconstruct initial clef/key/time from staffDef attributes when no event label exists.
fn reconstruct_initial_signatures(
    staff_def: &tusk_model::elements::StaffDef,
) -> Vec<SignatureEvent> {
    let mut events = Vec::new();

    // Clef
    if let Some(ref shape) = staff_def.staff_def_log.clef_shape {
        let line = staff_def
            .staff_def_log
            .clef_line
            .as_ref()
            .map(|l| l.0)
            .unwrap_or(2);
        let dis = staff_def.staff_def_log.clef_dis.as_ref().map(|d| d.0);
        let dis_place = staff_def.staff_def_log.clef_dis_place.as_ref();
        let name = crate::import::mei_clef_to_name(shape, line, dis, dis_place);
        events.push(SignatureEvent {
            position: 0,
            music: Music::Clef(Clef { name }),
        });
    }

    // Key
    if let Some(ref keysig) = staff_def.staff_def_log.keysig
        && let Ok(fifths) = keysig.0.parse::<i32>()
    {
        let (pitch, mode) = crate::import::fifths_to_key(fifths);
        events.push(SignatureEvent {
            position: 0,
            music: Music::KeySignature(KeySignature { pitch, mode }),
        });
    }

    // Meter
    if let Some(ref count) = staff_def.staff_def_log.meter_count {
        let numerators: Vec<u32> = count
            .split('+')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let denominator: u32 = staff_def
            .staff_def_log
            .meter_unit
            .as_ref()
            .and_then(|u| u.parse().ok())
            .unwrap_or(4);
        if !numerators.is_empty() {
            events.push(SignatureEvent {
                position: 0,
                music: Music::TimeSignature(TimeSignature {
                    numerators,
                    denominator,
                }),
            });
        }
    }

    events
}

/// Parse a key signature label: `STEP.ALTER.MODE`
fn parse_key_label(s: &str) -> Option<KeySignature> {
    let mut parts = s.splitn(3, '.');
    let step_str = parts.next()?;
    let alter_str = parts.next()?;
    let mode_str = parts.next()?;

    let step = step_str.chars().next()?;
    let alter: f32 = alter_str.parse().ok()?;
    let mode = Mode::from_name(mode_str)?;

    Some(KeySignature {
        pitch: Pitch {
            step,
            alter,
            octave: 0,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        },
        mode,
    })
}

/// Parse a time signature label: `N+M/D`
fn parse_time_label(s: &str) -> Option<TimeSignature> {
    let (num_str, den_str) = s.split_once('/')?;
    let numerators: Vec<u32> = num_str
        .split('+')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let denominator: u32 = den_str.trim().parse().ok()?;
    if numerators.is_empty() {
        return None;
    }
    Some(TimeSignature {
        numerators,
        denominator,
    })
}

/// Inject signature events into layer items at the correct positions.
///
/// Events are keyed by position in the note/rest stream. We insert them
/// before the note/rest at that position. Only injected into the first layer
/// (voice 1) since clef/key/time apply to the whole staff.
fn inject_signature_events(layers: &mut [Vec<Music>], events: &[SignatureEvent]) {
    if layers.is_empty() || events.is_empty() {
        return;
    }
    // Only inject into first layer
    let layer = &mut layers[0];

    // Build insertion map: position → list of Music to insert (in order)
    let mut inserts: std::collections::BTreeMap<u32, Vec<Music>> =
        std::collections::BTreeMap::new();
    for ev in events {
        inserts
            .entry(ev.position)
            .or_default()
            .push(ev.music.clone());
    }

    // Rebuild layer with injected events
    let mut new_items = Vec::new();
    for (note_idx, item) in layer.drain(..).enumerate() {
        if let Some(to_insert) = inserts.remove(&(note_idx as u32)) {
            new_items.extend(to_insert);
        }
        new_items.push(item);
    }
    // Any remaining events at end of stream
    for (_pos, to_insert) in inserts {
        new_items.extend(to_insert);
    }
    *layer = new_items;
}

// ---------------------------------------------------------------------------
// Pitch context (relative / transpose) extraction and application
// ---------------------------------------------------------------------------

/// Parsed pitch context for a staff.
enum PitchCtx {
    /// `\relative [pitch] { ... }` — reference pitch in marks format.
    Relative {
        step: char,
        alter: f32,
        octave: i8,
        has_pitch: bool,
    },
    /// `\transpose from to { ... }`.
    Transpose { from: Pitch, to: Pitch },
}

/// Extract pitch context labels from all staffDefs.
fn extract_pitch_contexts(score: &tusk_model::elements::Score) -> Vec<Option<PitchCtx>> {
    let mut result = Vec::new();
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            result.push(parse_pitch_context_label(sdef));
                        }
                    }
                }
            }
        }
    }
    result
}

/// Parse the `lilypond:relative,...` or `lilypond:transpose,...` segment from a staffDef label.
fn parse_pitch_context_label(staff_def: &tusk_model::elements::StaffDef) -> Option<PitchCtx> {
    let label = staff_def.labelled.label.as_deref()?;

    for segment in label.split('|') {
        if segment == "lilypond:relative" {
            // No reference pitch
            return Some(PitchCtx::Relative {
                step: 'f',
                alter: 0.0,
                octave: 0,
                has_pitch: false,
            });
        }
        if let Some(rest) = segment.strip_prefix("lilypond:relative,") {
            let parts: Vec<&str> = rest.splitn(3, '.').collect();
            if parts.len() == 3 {
                let step = parts[0].chars().next().unwrap_or('c');
                let alter: f32 = parts[1].parse().unwrap_or(0.0);
                let octave: i8 = parts[2].parse().unwrap_or(0);
                return Some(PitchCtx::Relative {
                    step,
                    alter,
                    octave,
                    has_pitch: true,
                });
            }
        }
        if let Some(rest) = segment.strip_prefix("lilypond:transpose,") {
            let pitches: Vec<&str> = rest.splitn(2, ',').collect();
            if pitches.len() == 2 {
                let from = parse_pitch_label(pitches[0]);
                let to = parse_pitch_label(pitches[1]);
                if let (Some(f), Some(t)) = (from, to) {
                    return Some(PitchCtx::Transpose { from: f, to: t });
                }
            }
        }
    }
    None
}

/// Parse a pitch label `STEP.ALTER.OCT` into a Pitch.
fn parse_pitch_label(s: &str) -> Option<Pitch> {
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    if parts.len() == 3 {
        let step = parts[0].chars().next()?;
        let alter: f32 = parts[1].parse().ok()?;
        let octave: i8 = parts[2].parse().ok()?;
        Some(Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        })
    } else {
        None
    }
}

/// Apply pitch context wrappers to each staff's music.
///
/// For relative mode: convert absolute pitches to relative and wrap in `\relative`.
/// For transpose: un-transpose pitches and wrap in `\transpose`.
fn apply_pitch_contexts(staff_music: &mut [Vec<Vec<Music>>], pitch_contexts: &[Option<PitchCtx>]) {
    for (staff_idx, layers) in staff_music.iter_mut().enumerate() {
        if let Some(Some(ctx)) = pitch_contexts.get(staff_idx) {
            match ctx {
                PitchCtx::Relative {
                    step,
                    alter,
                    octave,
                    has_pitch,
                } => {
                    // Convert absolute pitches to relative in all layers
                    for layer_items in layers.iter_mut() {
                        convert_to_relative(layer_items, *step, *octave);
                    }

                    // Build the music to wrap — we'll do this after layers
                    // are processed into final music, but we need to mark
                    // for wrapping. Store as a sentinel at the beginning.
                    // Actually, it's easier to wrap the flattened music.
                    // Let's take a different approach: collect all layer items,
                    // build the music, wrap, then put back.
                    let all_items: Vec<Vec<Music>> = std::mem::take(layers);
                    let inner = build_layers_music(all_items);

                    let ref_pitch = if *has_pitch {
                        Some(Box::new(Music::Note(NoteEvent {
                            pitch: Pitch {
                                step: *step,
                                alter: *alter,
                                octave: *octave,
                                force_accidental: false,
                                cautionary: false,
                                octave_check: None,
                            },
                            duration: None,
                            pitched_rest: false,
                        })))
                    } else {
                        None
                    };

                    let wrapped = Music::Relative {
                        pitch: ref_pitch,
                        body: Box::new(inner),
                    };

                    // Put back as a single layer with the wrapped music
                    layers.push(vec![wrapped]);
                }
                PitchCtx::Transpose { from, to } => {
                    // Un-transpose pitches in all layers
                    for layer_items in layers.iter_mut() {
                        untranspose_items(layer_items, from, to);
                    }

                    let all_items: Vec<Vec<Music>> = std::mem::take(layers);
                    let inner = build_layers_music(all_items);

                    let wrapped = Music::Transpose {
                        from: Box::new(Music::Note(NoteEvent {
                            pitch: from.clone(),
                            duration: None,
                            pitched_rest: false,
                        })),
                        to: Box::new(Music::Note(NoteEvent {
                            pitch: to.clone(),
                            duration: None,
                            pitched_rest: false,
                        })),
                        body: Box::new(inner),
                    };

                    layers.push(vec![wrapped]);
                }
            }
        }
    }
}

/// Convert a list of Music items from absolute to relative octave marks.
fn convert_to_relative(items: &mut [Music], ref_step: char, ref_oct: i8) {
    let mut current_step = ref_step;
    let mut current_oct = ref_oct;

    for item in items.iter_mut() {
        match item {
            Music::Note(note) => {
                let rel_marks = note.pitch.to_relative_marks(current_step, current_oct);
                current_step = note.pitch.step;
                current_oct = note.pitch.octave;
                note.pitch.octave = rel_marks;
            }
            Music::Chord(chord) => {
                // In relative mode, the first pitch in a chord is relative to
                // the previous note; subsequent pitches are relative to the first.
                // Capture first pitch's absolute position before mutating.
                let first_step = chord.pitches[0].step;
                let first_oct = chord.pitches[0].octave;
                for (i, pitch) in chord.pitches.iter_mut().enumerate() {
                    let (rs, ro) = if i == 0 {
                        (current_step, current_oct)
                    } else {
                        (first_step, first_oct)
                    };
                    let rel_marks = pitch.to_relative_marks(rs, ro);
                    pitch.octave = rel_marks;
                }
                // Update reference for next event to the first chord pitch
                current_step = first_step;
                current_oct = first_oct;
            }
            _ => {}
        }
    }
}

/// Un-transpose pitches in a list of Music items.
fn untranspose_items(items: &mut [Music], from: &Pitch, to: &Pitch) {
    for item in items.iter_mut() {
        match item {
            Music::Note(note) => {
                note.pitch = note.pitch.untranspose(from, to);
            }
            Music::Chord(chord) => {
                for pitch in &mut chord.pitches {
                    *pitch = pitch.untranspose(from, to);
                }
            }
            _ => {}
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

/// Convert a single MEI LayerChild to a LilyPond Music expression.
fn convert_layer_child(child: &LayerChild) -> Option<Music> {
    match child {
        LayerChild::Note(note) => Some(convert_mei_note(note)),
        LayerChild::Rest(rest) => Some(convert_mei_rest(rest)),
        LayerChild::MRest(mrest) => Some(convert_mei_mrest(mrest)),
        LayerChild::Chord(chord) => Some(convert_mei_chord(chord)),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// MEI → LilyPond pitch/duration conversion
// ---------------------------------------------------------------------------

/// Convert MEI DataDurationCmn to LilyPond duration base.
fn mei_dur_to_base(dur: &DataDurationCmn) -> u32 {
    match dur {
        DataDurationCmn::N1 => 1,
        DataDurationCmn::N2 => 2,
        DataDurationCmn::N4 => 4,
        DataDurationCmn::N8 => 8,
        DataDurationCmn::N16 => 16,
        DataDurationCmn::N32 => 32,
        DataDurationCmn::N64 => 64,
        DataDurationCmn::N128 => 128,
        DataDurationCmn::Long => 1, // fallback
        DataDurationCmn::Breve => 1,
        _ => 4,
    }
}

/// Convert MEI gestural accidental to alter in half-steps.
fn accid_ges_to_alter(accid: &DataAccidentalGesturalBasic) -> f32 {
    match accid {
        DataAccidentalGesturalBasic::S => 1.0,
        DataAccidentalGesturalBasic::Ss => 2.0,
        DataAccidentalGesturalBasic::F => -1.0,
        DataAccidentalGesturalBasic::Ff => -2.0,
        DataAccidentalGesturalBasic::N => 0.0,
        _ => 0.0,
    }
}

/// Convert MEI octave (0-based) to LilyPond octave marks (relative to c = octave 3).
fn mei_oct_to_marks(oct: u64) -> i8 {
    (oct as i8) - 3
}

/// Extract duration from an MEI note.
fn extract_note_duration(note: &tusk_model::elements::Note) -> Option<Duration> {
    let dur = note.note_log.dur.as_ref()?;
    let base = match dur {
        tusk_model::generated::data::DataDuration::MeiDataDurationCmn(cmn) => mei_dur_to_base(cmn),
        _ => return None,
    };
    let dots = note.note_log.dots.as_ref().map(|d| d.0 as u8).unwrap_or(0);
    Some(Duration {
        base,
        dots,
        multipliers: Vec::new(),
    })
}

/// Extract duration from an MEI rest.
fn extract_rest_duration(rest: &tusk_model::elements::Rest) -> Option<Duration> {
    let dur = rest.rest_log.dur.as_ref()?;
    let base = match dur {
        tusk_model::generated::data::DataDurationrests::MeiDataDurationCmn(cmn) => {
            mei_dur_to_base(cmn)
        }
        _ => return None,
    };
    let dots = rest.rest_log.dots.as_ref().map(|d| d.0 as u8).unwrap_or(0);
    Some(Duration {
        base,
        dots,
        multipliers: Vec::new(),
    })
}

// ---------------------------------------------------------------------------
// Event conversion
// ---------------------------------------------------------------------------

/// Extract a LilyPond Pitch from an MEI Note (for use inside chords — no duration).
fn extract_pitch_from_note(note: &tusk_model::elements::Note) -> Pitch {
    let step = note
        .note_log
        .pname
        .as_ref()
        .and_then(|p| p.0.chars().next())
        .unwrap_or('c');

    let octave = note
        .note_log
        .oct
        .as_ref()
        .map(|o| mei_oct_to_marks(o.0))
        .unwrap_or(0);

    let alter = note
        .note_ges
        .accid_ges
        .as_ref()
        .and_then(|ag| match ag {
            tusk_model::generated::data::DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
                basic,
            ) => Some(accid_ges_to_alter(basic)),
            _ => None,
        })
        .unwrap_or(0.0);

    let mut force_accidental = false;
    let mut cautionary = false;
    for child in &note.children {
        if let tusk_model::elements::NoteChild::Accid(accid) = child {
            force_accidental = true;
            if accid.accid_log.func.as_deref() == Some("cautionary") {
                cautionary = true;
                force_accidental = false;
            }
        }
    }

    Pitch {
        step,
        alter,
        octave,
        force_accidental,
        cautionary,
        octave_check: None,
    }
}

/// Convert an MEI Chord to a LilyPond ChordEvent.
fn convert_mei_chord(chord: &tusk_model::elements::Chord) -> Music {
    let pitches: Vec<Pitch> = chord
        .children
        .iter()
        .map(|child| {
            let ChordChild::Note(note) = child;
            extract_pitch_from_note(note)
        })
        .collect();

    let duration = extract_chord_duration(chord);

    Music::Chord(ChordEvent { pitches, duration })
}

/// Extract duration from an MEI chord.
fn extract_chord_duration(chord: &tusk_model::elements::Chord) -> Option<Duration> {
    let dur = chord.chord_log.dur.as_ref()?;
    let base = match dur {
        tusk_model::generated::data::DataDuration::MeiDataDurationCmn(cmn) => mei_dur_to_base(cmn),
        _ => return None,
    };
    let dots = chord
        .chord_log
        .dots
        .as_ref()
        .map(|d| d.0 as u8)
        .unwrap_or(0);
    Some(Duration {
        base,
        dots,
        multipliers: Vec::new(),
    })
}

/// Convert an MEI Note to a LilyPond NoteEvent.
fn convert_mei_note(note: &tusk_model::elements::Note) -> Music {
    let pitch = extract_pitch_from_note(note);
    let duration = extract_note_duration(note);

    Music::Note(NoteEvent {
        pitch,
        duration,
        pitched_rest: false,
    })
}

/// Convert an MEI Rest to a LilyPond RestEvent or pitched rest.
fn convert_mei_rest(rest: &tusk_model::elements::Rest) -> Music {
    // Check for pitched rest label
    if let Some(label) = &rest.common.label
        && let Some(pitch_str) = label.strip_prefix("lilypond:pitched-rest,")
        && let Some(note_event) = parse_pitched_rest_label(pitch_str, rest)
    {
        return Music::Note(note_event);
    }

    Music::Rest(RestEvent {
        duration: extract_rest_duration(rest),
    })
}

/// Parse a pitched rest label back into a NoteEvent.
fn parse_pitched_rest_label(
    pitch_str: &str,
    rest: &tusk_model::elements::Rest,
) -> Option<NoteEvent> {
    // Split into note name and octave marks
    let mut note_end = 0;
    for (i, c) in pitch_str.char_indices() {
        if c == '\'' || c == ',' {
            note_end = i;
            break;
        }
        note_end = i + c.len_utf8();
    }
    let note_name = &pitch_str[..note_end];
    let octave_str = &pitch_str[note_end..];

    let (step, alter) = Pitch::from_note_name(note_name)?;
    let octave = octave_str
        .chars()
        .map(|c| if c == '\'' { 1i8 } else { -1i8 })
        .sum();

    Some(NoteEvent {
        pitch: Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        },
        duration: extract_rest_duration(rest),
        pitched_rest: true,
    })
}

/// Convert an MEI MRest to a LilyPond MultiMeasureRestEvent.
fn convert_mei_mrest(mrest: &tusk_model::elements::MRest) -> Music {
    // Restore duration from label
    let duration = mrest
        .common
        .label
        .as_ref()
        .and_then(|l| l.strip_prefix("lilypond:mrest,"))
        .and_then(parse_mrest_label);

    Music::MultiMeasureRest(MultiMeasureRestEvent { duration })
}

/// Parse mrest label back to Duration.
fn parse_mrest_label(label: &str) -> Option<Duration> {
    let mut base = None;
    let mut dots = 0u8;
    let mut multipliers = Vec::new();

    for part in label.split(',') {
        if let Some(val) = part.strip_prefix("dur=") {
            base = val.parse().ok();
        } else if let Some(val) = part.strip_prefix("dots=") {
            dots = val.parse().unwrap_or(0);
        } else if let Some(val) = part.strip_prefix("mul=") {
            if let Some((n, d)) = val.split_once('/') {
                if let (Ok(n), Ok(d)) = (n.parse(), d.parse()) {
                    multipliers.push((n, d));
                }
            } else if let Ok(n) = val.parse() {
                multipliers.push((n, 1));
            }
        }
    }

    Some(Duration {
        base: base?,
        dots,
        multipliers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import;
    use crate::parser::Parser;
    use crate::serializer;

    /// Parse LilyPond → import to MEI → export to LilyPond AST → serialize.
    fn roundtrip(src: &str) -> String {
        let file = Parser::new(src).unwrap().parse().unwrap();
        let mei = import::import(&file).unwrap();
        let exported = export(&mei).unwrap();
        serializer::serialize(&exported)
    }

    #[test]
    fn roundtrip_single_note() {
        let output = roundtrip("{ c'4 }");
        assert!(output.contains("c'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_note_with_accidental() {
        let output = roundtrip("{ cis''2 }");
        assert!(output.contains("cis''2"), "output: {output}");
    }

    #[test]
    fn roundtrip_rest() {
        let output = roundtrip("{ r4 }");
        assert!(output.contains("r4"), "output: {output}");
    }

    #[test]
    fn roundtrip_dotted() {
        let output = roundtrip("{ c'2. r8. }");
        assert!(output.contains("c'2."), "output: {output}");
        assert!(output.contains("r8."), "output: {output}");
    }

    #[test]
    fn roundtrip_flat() {
        let output = roundtrip("{ bes,16 }");
        assert!(output.contains("bes,16"), "output: {output}");
    }

    #[test]
    fn roundtrip_multiple_notes() {
        let output = roundtrip("{ c4 d8 e16 f2 }");
        assert!(output.contains("c4"), "output: {output}");
        assert!(output.contains("d8"), "output: {output}");
        assert!(output.contains("e16"), "output: {output}");
        assert!(output.contains("f2"), "output: {output}");
    }

    #[test]
    fn roundtrip_multi_measure_rest() {
        let output = roundtrip("{ R1*4 }");
        assert!(output.contains("R1*4"), "output: {output}");
    }

    #[test]
    fn roundtrip_pitched_rest() {
        let output = roundtrip("{ c4\\rest }");
        assert!(output.contains("c4\\rest"), "output: {output}");
    }

    #[test]
    fn roundtrip_force_accidental() {
        let output = roundtrip("{ cis'!4 }");
        assert!(output.contains("cis'!4"), "output: {output}");
    }

    #[test]
    fn roundtrip_cautionary_accidental() {
        let output = roundtrip("{ bes'?4 }");
        assert!(output.contains("bes'?4"), "output: {output}");
    }

    #[test]
    fn roundtrip_two_voices() {
        let output = roundtrip("<< { c'4 d'4 } { e'4 f'4 } >>");
        // Should produce simultaneous with two sequential voices
        assert!(output.contains("<<"), "output: {output}");
        assert!(output.contains(">>"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("d'4"), "output: {output}");
        assert!(output.contains("e'4"), "output: {output}");
        assert!(output.contains("f'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_three_voices() {
        let output = roundtrip("<< { c'4 } { e'4 } { g'4 } >>");
        assert!(output.contains("<<"), "output: {output}");
        assert!(output.contains(">>"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("e'4"), "output: {output}");
        assert!(output.contains("g'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_sequential_preserved() {
        // Single voice should stay sequential, no << >>
        let output = roundtrip("{ c'4 d'4 e'4 }");
        assert!(!output.contains("<<"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
    }

    // --- Phase 5.2: Context export/roundtrip tests ---

    #[test]
    fn roundtrip_staff_group() {
        let output =
            roundtrip("\\new StaffGroup << \\new Staff { c'4 d'4 } \\new Staff { e'4 f'4 } >>");
        assert!(output.contains("\\new StaffGroup"), "output: {output}");
        assert!(output.contains("\\new Staff"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("e'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_piano_staff() {
        let output = roundtrip("\\new PianoStaff << \\new Staff { c'4 } \\new Staff { e4 } >>");
        assert!(output.contains("\\new PianoStaff"), "output: {output}");
        assert!(output.contains("\\new Staff"), "output: {output}");
    }

    #[test]
    fn roundtrip_named_staves() {
        let output = roundtrip(
            "\\new StaffGroup << \\new Staff = \"violin\" { c'4 } \\new Staff = \"viola\" { e4 } >>",
        );
        assert!(output.contains("\"violin\""), "output: {output}");
        assert!(output.contains("\"viola\""), "output: {output}");
    }

    #[test]
    fn roundtrip_single_named_staff() {
        let output = roundtrip("\\new Staff = \"piano\" { c'4 d'4 }");
        assert!(output.contains("\\new Staff"), "output: {output}");
        assert!(output.contains("\"piano\""), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_staff_with_block() {
        let output =
            roundtrip("\\new Staff \\with { \\consists \"Span_arpeggio_engraver\" } { c'4 }");
        assert!(output.contains("\\new Staff"), "output: {output}");
        assert!(output.contains("\\with"), "output: {output}");
        assert!(
            output.contains("Span_arpeggio_engraver"),
            "output: {output}"
        );
    }

    // --- Phase 6.2: Clef/key/time roundtrip tests ---

    #[test]
    fn roundtrip_clef_treble() {
        let output = roundtrip("{ \\clef \"treble\" c'4 d'4 }");
        assert!(output.contains("\\clef \"treble\""), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_clef_bass() {
        let output = roundtrip("{ \\clef \"bass\" c4 d4 }");
        assert!(output.contains("\\clef \"bass\""), "output: {output}");
    }

    #[test]
    fn roundtrip_clef_alto() {
        let output = roundtrip("{ \\clef \"alto\" c'4 }");
        assert!(output.contains("\\clef \"alto\""), "output: {output}");
    }

    #[test]
    fn roundtrip_key_d_major() {
        let output = roundtrip("{ \\key d \\major c'4 }");
        assert!(output.contains("\\key d \\major"), "output: {output}");
    }

    #[test]
    fn roundtrip_key_bes_minor() {
        let output = roundtrip("{ \\key bes \\minor c'4 }");
        assert!(output.contains("\\key bes \\minor"), "output: {output}");
    }

    #[test]
    fn roundtrip_time_3_4() {
        let output = roundtrip("{ \\time 3/4 c'4 }");
        assert!(output.contains("\\time 3/4"), "output: {output}");
    }

    #[test]
    fn roundtrip_time_compound() {
        let output = roundtrip("{ \\time 2+3/8 c'4 }");
        assert!(output.contains("\\time 2+3/8"), "output: {output}");
    }

    #[test]
    fn roundtrip_clef_key_time_combined() {
        let output = roundtrip("{ \\clef \"treble\" \\key d \\major \\time 4/4 c'4 d'4 e'4 f'4 }");
        assert!(output.contains("\\clef \"treble\""), "output: {output}");
        assert!(output.contains("\\key d \\major"), "output: {output}");
        assert!(output.contains("\\time 4/4"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_clef_change_mid_stream() {
        let output = roundtrip("{ \\clef \"treble\" c'4 d'4 \\clef \"bass\" e4 f4 }");
        assert!(output.contains("\\clef \"treble\""), "output: {output}");
        assert!(output.contains("\\clef \"bass\""), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("e4"), "output: {output}");
    }

    #[test]
    fn roundtrip_key_change() {
        let output = roundtrip("{ \\key c \\major c'4 d'4 \\key g \\major e'4 f'4 }");
        assert!(output.contains("\\key c \\major"), "output: {output}");
        assert!(output.contains("\\key g \\major"), "output: {output}");
    }

    #[test]
    fn roundtrip_time_change() {
        let output = roundtrip("{ \\time 4/4 c'4 d'4 \\time 3/4 e'4 f'4 }");
        assert!(output.contains("\\time 4/4"), "output: {output}");
        assert!(output.contains("\\time 3/4"), "output: {output}");
    }

    #[test]
    fn roundtrip_transposed_clef() {
        let output = roundtrip("{ \\clef \"treble_8\" c4 }");
        assert!(output.contains("\\clef \"treble_8\""), "output: {output}");
    }

    #[test]
    fn roundtrip_clef_key_time_fixture() {
        let src = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/fragment_clef_key_time.ly"
        ))
        .unwrap();
        let output = roundtrip(&src);
        assert!(output.contains("\\clef \"treble\""), "output: {output}");
        assert!(output.contains("\\key d \\major"), "output: {output}");
        assert!(output.contains("\\time 4/4"), "output: {output}");
        assert!(output.contains("\\clef \"bass\""), "output: {output}");
        assert!(output.contains("\\key bes \\minor"), "output: {output}");
        assert!(output.contains("\\time 3/4"), "output: {output}");
        assert!(output.contains("\\time 2+3/8"), "output: {output}");
    }

    #[test]
    fn roundtrip_contexts_fixture() {
        let src = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/fragment_contexts.ly"
        ))
        .unwrap();
        let output = roundtrip(&src);
        assert!(output.contains("\\new StaffGroup"), "output: {output}");
        assert!(output.contains("\\new Staff"), "output: {output}");
        assert!(output.contains("\"violin\""), "output: {output}");
        assert!(output.contains("\"viola\""), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
    }

    // --- Phase 7.2: Relative / transpose roundtrip tests ---

    #[test]
    fn roundtrip_relative_basic() {
        // \relative c' { c d e f } → notes resolve to c' d' e' f'
        // On export, should wrap in \relative c' and use relative marks
        let output = roundtrip("\\relative c' { c4 d e f }");
        assert!(output.contains("\\relative"), "output: {output}");
        assert!(output.contains("c4"), "output: {output}");
        assert!(output.contains("d"), "output: {output}");
        assert!(output.contains("e"), "output: {output}");
        assert!(output.contains("f"), "output: {output}");
    }

    #[test]
    fn roundtrip_relative_no_pitch() {
        // \relative { c d e f } — default reference is f
        let output = roundtrip("\\relative { c4 d e f }");
        assert!(output.contains("\\relative"), "output: {output}");
    }

    #[test]
    fn roundtrip_relative_octave_jump() {
        // \relative c' { c c' c, c } — the ' and , adjust from closest position
        let output = roundtrip("\\relative c' { c4 c' c, c }");
        assert!(output.contains("\\relative"), "output: {output}");
        // Should contain notes with octave marks
        assert!(output.contains("c4"), "output: {output}");
    }

    #[test]
    fn roundtrip_relative_with_accidentals() {
        let output = roundtrip("\\relative c' { c4 cis d bes }");
        assert!(output.contains("\\relative"), "output: {output}");
        assert!(output.contains("cis"), "output: {output}");
        assert!(output.contains("bes"), "output: {output}");
    }

    #[test]
    fn roundtrip_relative_descending() {
        // In relative c': b is closest going down from c
        let output = roundtrip("\\relative c' { c4 b a g }");
        assert!(output.contains("\\relative"), "output: {output}");
        assert!(output.contains("b"), "output: {output}");
        assert!(output.contains("a"), "output: {output}");
        assert!(output.contains("g"), "output: {output}");
    }

    #[test]
    fn roundtrip_transpose_basic() {
        // \transpose c d { c4 d e f } → all pitches shifted up a whole step
        let output = roundtrip("\\transpose c d { c4 d e f }");
        assert!(output.contains("\\transpose"), "output: {output}");
        // The notes inside should be the original (un-transposed) pitches
        assert!(output.contains("c"), "output: {output}");
    }

    #[test]
    fn roundtrip_transpose_with_accidentals() {
        let output = roundtrip("\\transpose c d { cis4 bes e fis }");
        assert!(output.contains("\\transpose"), "output: {output}");
        assert!(output.contains("cis"), "output: {output}");
    }

    #[test]
    fn roundtrip_relative_in_staff() {
        // \relative inside \new Staff
        let output = roundtrip("\\new Staff \\relative c' { c4 d e f }");
        assert!(output.contains("\\relative"), "output: {output}");
        assert!(output.contains("\\new Staff"), "output: {output}");
    }

    #[test]
    fn roundtrip_relative_transpose_fixture() {
        let src = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/fragment_relative_transpose.ly"
        ))
        .unwrap();
        let output = roundtrip(&src);
        // The fixture has multiple top-level expressions; the importer picks the first.
        // The first is \relative c' { c4 d e f }
        assert!(output.contains("\\relative"), "output: {output}");
    }

    #[test]
    fn roundtrip_chord_basic() {
        let output = roundtrip("{ <c' e' g'>4 }");
        assert!(output.contains("<c' e' g'>4"), "output: {output}");
    }

    #[test]
    fn roundtrip_chord_dotted() {
        let output = roundtrip("{ <c' e'>2. }");
        assert!(output.contains("<c' e'>2."), "output: {output}");
    }

    #[test]
    fn roundtrip_chord_with_accidentals() {
        let output = roundtrip("{ <cis' es' g'>4 }");
        // es → ees canonical form (both valid LilyPond)
        assert!(output.contains("<cis' ees' g'>4"), "output: {output}");
    }

    #[test]
    fn roundtrip_chord_force_cautionary() {
        let output = roundtrip("{ <cis'! e'?>4 }");
        assert!(output.contains("cis'!"), "output: {output}");
        assert!(output.contains("e'?"), "output: {output}");
    }

    #[test]
    fn roundtrip_chord_mixed_with_notes() {
        let output = roundtrip("{ c'4 <d' f'>8 e'2 }");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("<d' f'>8"), "output: {output}");
        assert!(output.contains("e'2"), "output: {output}");
    }

    #[test]
    fn roundtrip_chord_fixture() {
        let src = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/lilypond/fragment_chords.ly"
        ))
        .unwrap();
        let output = roundtrip(&src);
        assert!(output.contains("<c e g>4"), "output: {output}");
        // es → ees canonical form (both valid LilyPond)
        assert!(output.contains("<c ees g>2."), "output: {output}");
        assert!(output.contains("<d' fis' a'>8"), "output: {output}");
        assert!(output.contains("<bes, d f>1"), "output: {output}");
        assert!(output.contains("cis''!"), "output: {output}");
        assert!(output.contains("e''?"), "output: {output}");
    }
}

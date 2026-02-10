//! Conversion from LilyPond AST to MEI.

mod conversion;
mod signatures;

#[cfg(test)]
mod tests;

use thiserror::Error;
use tusk_model::elements::{
    Body, BodyChild, FileDesc, FileDescChild, Layer, LayerChild, Mdiv, MdivChild, Measure,
    MeasureChild, Mei, MeiChild, MeiHead, MeiHeadChild, Score, ScoreChild, ScoreDef, ScoreDefChild,
    Section, SectionChild, Slur, Staff, StaffChild, StaffDef, StaffGrp, StaffGrpChild, TitleStmt,
};
use tusk_model::generated::data::{DataTie, DataUri, DataWord};

use crate::model::{
    self, ContextKeyword, ContextModItem, Music, NoteEvent, PostEvent, RestEvent, ScoreItem,
    ToplevelExpression,
};
use crate::serializer;

use signatures::apply_signatures_to_staff_def;
pub use signatures::{fifths_to_key, mei_clef_to_name};

use conversion::{convert_chord, convert_mrest, convert_note, convert_pitched_rest, convert_rest};

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("LilyPond import is not yet implemented")]
    NotImplemented,
    #[error("no music found in LilyPond file")]
    NoMusic,
    #[error("import error: {0}")]
    Other(String),
}

/// Convert a parsed LilyPond AST to an MEI document.
pub fn import(file: &model::LilyPondFile) -> Result<Mei, ImportError> {
    let music = find_music(file).ok_or(ImportError::NoMusic)?;

    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some("6.0-dev".to_string());

    // Minimal meiHead with empty fileDesc/titleStmt
    let mei_head = build_mei_head();
    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

    // Music -> Body -> Mdiv -> Score
    let mei_music = build_music(music)?;
    mei.children.push(MeiChild::Music(Box::new(mei_music)));

    Ok(mei)
}

/// Find the first music expression in the LilyPond file.
fn find_music(file: &model::LilyPondFile) -> Option<&Music> {
    for item in &file.items {
        match item {
            ToplevelExpression::Score(score) => {
                for si in &score.items {
                    if let ScoreItem::Music(m) = si {
                        return Some(m);
                    }
                }
            }
            ToplevelExpression::Music(m) => return Some(m),
            _ => {}
        }
    }
    None
}

/// Build a minimal MeiHead.
fn build_mei_head() -> MeiHead {
    let title_stmt = TitleStmt::default();
    let mut file_desc = FileDesc::default();
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
    let mut head = MeiHead::default();
    head.children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
    head
}

/// Build MEI Music -> Body -> Mdiv -> Score from LilyPond music.
fn build_music(ly_music: &Music) -> Result<tusk_model::elements::Music, ImportError> {
    let mut score = Score::default();

    // Analyze context structure to determine staves
    let staff_infos = analyze_staves(ly_music);

    // Build ScoreDef with staffDef(s)
    let score_def = build_score_def_from_staves(&staff_infos);
    score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));

    // Section with measure(s) containing the notes
    let section = build_section_from_staves(&staff_infos)?;
    score.children.push(ScoreChild::Section(Box::new(section)));

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

// ---------------------------------------------------------------------------
// Context analysis -- extract staff structure from LilyPond AST
// ---------------------------------------------------------------------------

/// Information about a single staff extracted from the LilyPond AST.
struct StaffInfo<'a> {
    /// Staff number (1-based).
    n: u32,
    /// Context name (e.g. "violin") if `\new Staff = "violin"`.
    name: Option<String>,
    /// Context type (e.g. "Staff").
    context_type: String,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
    /// The music content for this staff (one or more voice streams).
    voices: Vec<Vec<&'a Music>>,
}

/// Information about a staff group wrapping multiple staves.
struct GroupInfo {
    /// Context type (e.g. "StaffGroup", "PianoStaff").
    context_type: String,
    /// Context name, if any.
    name: Option<String>,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
}

/// Result of analyzing the context hierarchy.
struct StaffLayout<'a> {
    group: Option<GroupInfo>,
    staves: Vec<StaffInfo<'a>>,
}

/// Analyze the LilyPond music tree to extract staff structure.
///
/// Detects patterns like:
/// - `\new StaffGroup << \new Staff { } \new Staff { } >>`
/// - `\new PianoStaff << \new Staff { } \new Staff { } >>`
/// - `\new Staff { ... }` (single staff)
/// - `{ ... }` (bare music, single staff)
fn analyze_staves(music: &Music) -> StaffLayout<'_> {
    // Unwrap score-level context (e.g. \new StaffGroup << ... >>)
    if let Music::ContextedMusic {
        keyword: _,
        context_type,
        name,
        with_block,
        music: inner,
    } = music
    {
        // Check if this is a group context wrapping staves
        if is_staff_group_context(context_type) {
            let group = GroupInfo {
                context_type: context_type.clone(),
                name: name.clone(),
                with_block: with_block.clone(),
            };
            let staves = extract_staves_from_group(inner);
            if !staves.is_empty() {
                return StaffLayout {
                    group: Some(group),
                    staves,
                };
            }
        }

        // Single contexted staff (e.g. \new Staff { ... })
        if is_staff_context(context_type) {
            let voices = extract_voices(inner);
            return StaffLayout {
                group: None,
                staves: vec![StaffInfo {
                    n: 1,
                    name: name.clone(),
                    context_type: context_type.clone(),
                    with_block: with_block.clone(),
                    voices,
                }],
            };
        }

        // Unknown context type -- treat inner music as bare
        return analyze_staves(inner);
    }

    // Check if simultaneous music contains \new Staff children
    if let Music::Simultaneous(items) = music {
        let staves = extract_staves_from_simultaneous(items);
        if !staves.is_empty() {
            return StaffLayout {
                group: None,
                staves,
            };
        }
    }

    // Bare music -- single staff, possibly multiple voices
    let voices = extract_voices(music);
    StaffLayout {
        group: None,
        staves: vec![StaffInfo {
            n: 1,
            name: None,
            context_type: "Staff".to_string(),
            with_block: None,
            voices,
        }],
    }
}

/// Check if a context type is a staff group (StaffGroup, PianoStaff, etc.)
fn is_staff_group_context(ctx: &str) -> bool {
    matches!(
        ctx,
        "StaffGroup"
            | "PianoStaff"
            | "GrandStaff"
            | "ChoirStaff"
            | "InnerStaffGroup"
            | "InnerChoirStaff"
    )
}

/// Check if a context type is a staff-level context.
fn is_staff_context(ctx: &str) -> bool {
    matches!(
        ctx,
        "Staff"
            | "RhythmicStaff"
            | "TabStaff"
            | "DrumStaff"
            | "GregorianTranscriptionStaff"
            | "MensuralStaff"
            | "PetrucciStaff"
            | "VaticanaStaff"
    )
}

/// Extract staff infos from the inner music of a group context.
fn extract_staves_from_group(music: &Music) -> Vec<StaffInfo<'_>> {
    match music {
        Music::Simultaneous(items) => extract_staves_from_simultaneous(items),
        _ => Vec::new(),
    }
}

/// Extract staff infos from a simultaneous music list that contains \new Staff children.
fn extract_staves_from_simultaneous<'a>(items: &'a [Music]) -> Vec<StaffInfo<'a>> {
    let mut staves = Vec::new();
    let mut n = 1u32;

    for item in items {
        if let Music::ContextedMusic {
            context_type,
            name,
            with_block,
            music: inner,
            ..
        } = item
            && is_staff_context(context_type)
        {
            let voices = extract_voices(inner);
            staves.push(StaffInfo {
                n,
                name: name.clone(),
                context_type: context_type.clone(),
                with_block: with_block.clone(),
                voices,
            });
            n += 1;
        }
    }

    staves
}

// ---------------------------------------------------------------------------
// ScoreDef building from staff layout
// ---------------------------------------------------------------------------

/// Map LilyPond group context type to MEI staffGrp @symbol.
fn group_context_to_symbol(context_type: &str) -> Option<&'static str> {
    match context_type {
        "StaffGroup" => Some("bracket"),
        "PianoStaff" | "GrandStaff" => Some("brace"),
        "ChoirStaff" => Some("bracket"),
        _ => None,
    }
}

/// Build a ScoreDef from analyzed staff structure, setting initial clef/key/time.
fn build_score_def_from_staves(layout: &StaffLayout<'_>) -> ScoreDef {
    let mut staff_grp = StaffGrp::default();

    // Set group symbol if present
    if let Some(group) = &layout.group {
        staff_grp.staff_grp_vis.symbol =
            group_context_to_symbol(&group.context_type).map(String::from);

        // Store group context metadata in label for roundtrip
        let label = build_group_label(group);
        if !label.is_empty() {
            staff_grp.common.label = Some(label);
        }
    }

    for staff_info in &layout.staves {
        let mut staff_def = StaffDef::default();
        staff_def.n_integer.n = Some(staff_info.n.to_string());

        // Collect events from all voices to find initial clef/key/time
        let mut events = Vec::new();
        let mut sig_ctx = PitchContext::new();
        for voice_music in &staff_info.voices {
            for m in voice_music {
                collect_events(m, &mut events, &mut sig_ctx);
            }
        }

        // Set initial clef/key/time on staffDef and collect event sequence for label
        let event_sequence = apply_signatures_to_staff_def(&events, &mut staff_def);

        // Detect relative/transpose context from the music tree
        let pitch_context_label = build_pitch_context_label(&staff_info.voices);

        // Build label: start with context metadata, append event sequence and pitch context
        let mut label = build_staff_label(staff_info);
        if !event_sequence.is_empty() {
            if !label.is_empty() {
                label.push('|');
            }
            label.push_str(&event_sequence);
        }
        if !pitch_context_label.is_empty() {
            if !label.is_empty() {
                label.push('|');
            }
            label.push_str(&pitch_context_label);
        }
        if !label.is_empty() {
            staff_def.labelled.label = Some(label);
        }

        staff_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
    }

    let mut score_def = ScoreDef::default();
    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));
    score_def
}

/// Build a label string for group context metadata.
///
/// Format: `lilypond:group,ContextType[,name=Name][,with={serialized}]`
fn build_group_label(group: &GroupInfo) -> String {
    let mut parts = vec![format!("lilypond:group,{}", group.context_type)];
    if let Some(name) = &group.name {
        parts.push(format!("name={name}"));
    }
    if let Some(with_items) = &group.with_block
        && !with_items.is_empty()
    {
        let with_str = serialize_with_block(with_items);
        parts.push(format!("with={with_str}"));
    }
    parts.join(",")
}

/// Build a label string for staff context metadata.
///
/// Format: `lilypond:staff,ContextType[,name=Name][,with={serialized}]`
fn build_staff_label(staff: &StaffInfo<'_>) -> String {
    let mut parts = vec![format!("lilypond:staff,{}", staff.context_type)];
    if let Some(name) = &staff.name {
        parts.push(format!("name={name}"));
    }
    if let Some(with_items) = &staff.with_block
        && !with_items.is_empty()
    {
        let with_str = serialize_with_block(with_items);
        parts.push(format!("with={with_str}"));
    }
    parts.join(",")
}

/// Serialize a \with { ... } block to a compact string for label storage.
///
/// Uses the LilyPond serializer to produce the block content.
fn serialize_with_block(items: &[ContextModItem]) -> String {
    // Create a minimal AST with a ContextedMusic to serialize the with block
    let file = model::LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "X".to_string(),
            name: None,
            with_block: Some(items.to_vec()),
            music: Box::new(Music::Sequential(Vec::new())),
        })],
    };
    let serialized = serializer::serialize(&file);
    // Extract just the \with block content from the serialized output
    // Format: "\new X \with {\n  ...\n} {\n}\n"
    if let Some(start) = serialized.find("\\with {") {
        let with_part = &serialized[start + 7..]; // skip "\with {"
        if let Some(end) = find_matching_brace(with_part) {
            return with_part[..end].trim().to_string();
        }
    }
    String::new()
}

/// Build a label segment encoding the outermost relative/transpose context.
///
/// Detects the first `\relative` or `\transpose` wrapper in the music tree for
/// a staff's voices and encodes it as:
/// - `lilypond:relative,STEP.ALTER.OCT` (with reference pitch) or `lilypond:relative` (no pitch)
/// - `lilypond:transpose,FROM_STEP.FROM_ALTER.FROM_OCT,TO_STEP.TO_ALTER.TO_OCT`
fn build_pitch_context_label(voices: &[Vec<&Music>]) -> String {
    // Look at each voice's music to find the outermost relative/transpose
    for voice in voices {
        for m in voice {
            if let Some(label) = detect_pitch_context(m) {
                return label;
            }
        }
    }
    String::new()
}

/// Detect the outermost relative/transpose wrapper in a music tree.
fn detect_pitch_context(music: &Music) -> Option<String> {
    match music {
        Music::Relative { pitch, .. } => {
            if let Some(ref_pitch_music) = pitch
                && let Some(p) = extract_pitch_from_music(ref_pitch_music)
            {
                Some(format!(
                    "lilypond:relative,{}.{}.{}",
                    p.step, p.alter, p.octave
                ))
            } else {
                Some("lilypond:relative".to_string())
            }
        }
        Music::Transpose { from, to, .. } => {
            let fp = extract_pitch_from_music(from)?;
            let tp = extract_pitch_from_music(to)?;
            Some(format!(
                "lilypond:transpose,{}.{}.{},{}.{}.{}",
                fp.step, fp.alter, fp.octave, tp.step, tp.alter, tp.octave
            ))
        }
        // Unwrap transparent wrappers to find nested relative/transpose
        Music::ContextedMusic { music, .. } => detect_pitch_context(music),
        _ => None,
    }
}

/// Find the position of the matching closing brace, handling nesting.
fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
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

/// Build a Section from analyzed staff layout.
fn build_section_from_staves(layout: &StaffLayout<'_>) -> Result<Section, ImportError> {
    let mut section = Section::default();
    let mut id_counter = 0u32;
    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    let mut slur_counter = 0u32;

    for staff_info in &layout.staves {
        let mut staff = Staff::default();
        staff.n_integer.n = Some(staff_info.n.to_string());

        for (voice_idx, voice_music) in staff_info.voices.iter().enumerate() {
            let mut layer = Layer::default();
            layer.n_integer.n = Some((voice_idx + 1).to_string());

            let mut events = Vec::new();
            let mut voice_ctx = PitchContext::new();
            for m in voice_music {
                collect_events(m, &mut events, &mut voice_ctx);
            }

            // Track IDs of notes for tie/slur resolution
            let mut pending_slurs: Vec<PendingSpanner> = Vec::new();
            let mut tie_pending = false;

            for event in &events {
                let (post_events, current_id) = match event {
                    LyEvent::Note(note) => {
                        id_counter += 1;
                        let mut mei_note = convert_note(note, id_counter);
                        // If a tie was pending from the previous note, mark this as tie end
                        if tie_pending {
                            mei_note.note_anl.tie = Some(DataTie::from("t".to_string()));
                            tie_pending = false;
                        }
                        let id_str = format!("ly-note-{}", id_counter);
                        let pe = note.post_events.clone();
                        // Check if this note starts a tie
                        if pe.contains(&PostEvent::Tie) {
                            match &mei_note.note_anl.tie {
                                Some(t) if t.0 == "t" => {
                                    // Already has terminal tie → medial
                                    mei_note.note_anl.tie = Some(DataTie::from("m".to_string()));
                                }
                                _ => {
                                    mei_note.note_anl.tie = Some(DataTie::from("i".to_string()));
                                }
                            }
                            tie_pending = true;
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
                        let mei_rest = convert_pitched_rest(note, id_counter);
                        let id_str = format!("ly-rest-{}", id_counter);
                        let pe = note.post_events.clone();
                        layer.children.push(LayerChild::Rest(Box::new(mei_rest)));
                        (pe, id_str)
                    }
                    LyEvent::Chord {
                        pitches,
                        duration,
                        post_events,
                    } => {
                        id_counter += 1;
                        let mut mei_chord =
                            convert_chord(pitches, duration.as_ref(), &mut id_counter);
                        // If a tie was pending, mark all chord notes as tie end
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
                        // Chord ties: set @tie on all child notes
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
                        layer.children.push(LayerChild::Chord(Box::new(mei_chord)));
                        (pe, id_str)
                    }
                    LyEvent::MeasureRest(rest) => {
                        id_counter += 1;
                        let mei_mrest = convert_mrest(rest, id_counter);
                        layer.children.push(LayerChild::MRest(Box::new(mei_mrest)));
                        continue;
                    }
                    LyEvent::Skip(_)
                    | LyEvent::Clef(_)
                    | LyEvent::KeySig(_)
                    | LyEvent::TimeSig(_) => continue,
                };

                // Process slur/phrasing slur post-events
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
                            // Find most recent pending regular slur
                            if let Some(pos) = pending_slurs.iter().rposition(|s| !s.is_phrase) {
                                let pending = pending_slurs.remove(pos);
                                slur_counter += 1;
                                let slur = make_slur(
                                    &pending.start_id,
                                    &current_id,
                                    pending.staff_n,
                                    slur_counter,
                                    false,
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
                            // Find most recent pending phrasing slur
                            if let Some(pos) = pending_slurs.iter().rposition(|s| s.is_phrase) {
                                let pending = pending_slurs.remove(pos);
                                slur_counter += 1;
                                let slur = make_slur(
                                    &pending.start_id,
                                    &current_id,
                                    pending.staff_n,
                                    slur_counter,
                                    true,
                                );
                                measure.children.push(MeasureChild::Slur(Box::new(slur)));
                            }
                        }
                        PostEvent::Tie => {} // handled above
                    }
                }
            }

            staff.children.push(StaffChild::Layer(Box::new(layer)));
        }

        measure.children.push(MeasureChild::Staff(Box::new(staff)));
    }

    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    Ok(section)
}

/// Create an MEI Slur control event.
fn make_slur(start_id: &str, end_id: &str, staff_n: u32, slur_id: u32, is_phrase: bool) -> Slur {
    let mut slur = Slur::default();
    slur.common.xml_id = Some(format!("ly-slur-{slur_id}"));
    slur.slur_log.startid = Some(DataUri(format!("#{start_id}")));
    slur.slur_log.endid = Some(DataUri(format!("#{end_id}")));
    slur.slur_log.staff = Some(staff_n.to_string());
    if is_phrase {
        slur.common.label = Some("lilypond:phrase".to_string());
    }
    slur
}

/// Extract voice streams from LilyPond music.
///
/// If the top-level music is `Simultaneous` and each child is a distinct
/// voice (Sequential block or single event), each child becomes a separate
/// voice (MEI layer). Otherwise, all music goes into a single voice.
fn extract_voices(music: &Music) -> Vec<Vec<&Music>> {
    match music {
        Music::Simultaneous(items) if items.len() > 1 => {
            // Check if children look like separate voice streams
            // (each is a Sequential block or a single event, NOT \new Staff)
            let all_voice_like = items.iter().all(|item| {
                matches!(
                    item,
                    Music::Sequential(_)
                        | Music::Note(_)
                        | Music::Chord(_)
                        | Music::Rest(_)
                        | Music::MultiMeasureRest(_)
                        | Music::Relative { .. }
                        | Music::Fixed { .. }
                        | Music::Transpose { .. }
                ) || matches!(
                    item,
                    Music::ContextedMusic { context_type, .. } if !is_staff_context(context_type) && !is_staff_group_context(context_type)
                )
            });
            if all_voice_like {
                items.iter().map(|item| vec![item]).collect()
            } else {
                vec![vec![music]]
            }
        }
        _ => vec![vec![music]],
    }
}

/// Internal event representation for collecting from the AST.
///
/// Events own resolved copies of notes (not references) because relative/transpose
/// resolution produces new Pitch values.
enum LyEvent {
    Note(NoteEvent),
    Chord {
        pitches: Vec<crate::model::Pitch>,
        duration: Option<crate::model::Duration>,
        post_events: Vec<PostEvent>,
    },
    Rest(RestEvent),
    PitchedRest(NoteEvent),
    MeasureRest(model::MultiMeasureRestEvent),
    Skip(()),
    Clef(model::Clef),
    KeySig(model::KeySignature),
    TimeSig(model::TimeSignature),
}

/// Pitch context tracking for relative mode and transposition.
#[derive(Clone)]
struct PitchContext {
    /// If in relative mode, (ref_step, ref_oct in marks format).
    relative: Option<(char, i8)>,
    /// Stack of transpositions to apply: (from, to) pairs.
    transpositions: Vec<(crate::model::Pitch, crate::model::Pitch)>,
}

impl PitchContext {
    fn new() -> Self {
        PitchContext {
            relative: None,
            transpositions: Vec::new(),
        }
    }

    /// Resolve a pitch through the current context (relative -> absolute, then transpose).
    fn resolve(&mut self, pitch: &crate::model::Pitch) -> crate::model::Pitch {
        let mut resolved = if let Some((ref_step, ref_oct)) = self.relative {
            let abs = pitch.resolve_relative(ref_step, ref_oct);
            // Update reference for next note
            self.relative = Some((abs.step, abs.octave));
            abs
        } else {
            pitch.clone()
        };

        // Apply transpositions (innermost first)
        for (from, to) in &self.transpositions {
            resolved = resolved.transpose(from, to);
        }

        resolved
    }
}

/// Recursively collect note/rest/skip events from LilyPond music,
/// resolving relative pitches and transpositions to absolute.
fn collect_events(music: &Music, events: &mut Vec<LyEvent>, ctx: &mut PitchContext) {
    match music {
        Music::Note(note) => {
            let mut resolved = note.clone();
            resolved.pitch = ctx.resolve(&note.pitch);
            if note.pitched_rest {
                events.push(LyEvent::PitchedRest(resolved));
            } else {
                events.push(LyEvent::Note(resolved));
            }
        }
        Music::Chord(chord) => {
            let resolved_pitches: Vec<_> = chord.pitches.iter().map(|p| ctx.resolve(p)).collect();
            events.push(LyEvent::Chord {
                pitches: resolved_pitches,
                duration: chord.duration.clone(),
                post_events: chord.post_events.clone(),
            });
        }
        Music::Rest(rest) => events.push(LyEvent::Rest(rest.clone())),
        Music::Skip(_) => events.push(LyEvent::Skip(())),
        Music::MultiMeasureRest(mrest) => events.push(LyEvent::MeasureRest(mrest.clone())),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                collect_events(item, events, ctx);
            }
        }
        Music::Relative { pitch, body } => {
            let mut inner_ctx = ctx.clone();
            // Set the reference pitch for relative mode
            let (ref_step, ref_oct) = if let Some(ref_pitch_music) = pitch {
                extract_pitch_from_music(ref_pitch_music)
                    .map(|p| (p.step, p.octave))
                    .unwrap_or(('f', 0)) // default: f (middle of keyboard)
            } else {
                ('f', 0) // LilyPond default: f (below middle C)
            };
            inner_ctx.relative = Some((ref_step, ref_oct));
            collect_events(body, events, &mut inner_ctx);
        }
        Music::Fixed { pitch: _, body } => {
            // Fixed mode: pitches are already absolute relative to the given pitch.
            // The pitch argument is the "origin" -- notes are absolute in that octave.
            // For now, just collect from body (pitches are written absolute).
            collect_events(body, events, ctx);
        }
        Music::Transpose { from, to, body } => {
            let from_pitch = extract_pitch_from_music(from);
            let to_pitch = extract_pitch_from_music(to);
            if let (Some(fp), Some(tp)) = (from_pitch, to_pitch) {
                let mut inner_ctx = ctx.clone();
                inner_ctx.transpositions.push((fp, tp));
                collect_events(body, events, &mut inner_ctx);
            } else {
                // Can't extract pitches -- collect without transposing
                collect_events(body, events, ctx);
            }
        }
        Music::ContextedMusic { music, .. } => {
            collect_events(music, events, ctx);
        }
        Music::ContextChange { .. } => {
            // Context changes don't produce note events
        }
        Music::Clef(c) => events.push(LyEvent::Clef(c.clone())),
        Music::KeySignature(ks) => events.push(LyEvent::KeySig(ks.clone())),
        Music::TimeSignature(ts) => events.push(LyEvent::TimeSig(ts.clone())),
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) => {}
    }
}

/// Extract a Pitch from a Music node (for \relative and \transpose arguments).
fn extract_pitch_from_music(music: &Music) -> Option<crate::model::Pitch> {
    match music {
        Music::Note(n) => Some(n.pitch.clone()),
        _ => None,
    }
}

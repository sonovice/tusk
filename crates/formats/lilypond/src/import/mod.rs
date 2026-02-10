//! Conversion from LilyPond AST to MEI.

mod beams;
mod control_events;
mod conversion;
mod events;
pub(crate) mod lyrics;
pub(crate) mod signatures;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_chords;
#[cfg(test)]
mod tests_control;
#[cfg(test)]
mod tests_drums;
#[cfg(test)]
mod tests_figures;
#[cfg(test)]
mod tests_tempo_marks;

use thiserror::Error;
use tusk_model::elements::{
    Body, BodyChild, FileDesc, FileDescChild, Layer, LayerChild, Mdiv, MdivChild, Measure,
    MeasureChild, Mei, MeiChild, MeiHead, MeiHeadChild, Score, ScoreChild, ScoreDef, ScoreDefChild,
    Section, SectionChild, Staff, StaffChild, StaffDef, StaffGrp, StaffGrpChild, TitleStmt,
};
use tusk_model::generated::data::{DataTie, DataWord};

use crate::model::{
    self, ContextKeyword, ContextModItem, Music, PostEvent, ScoreItem, ToplevelExpression,
};
use crate::serializer;

use events::{
    GraceType, LyEvent, PitchContext, apply_grace_to_chord, apply_grace_to_note, collect_events,
    extract_pitch_from_music,
};
use signatures::apply_signatures_to_staff_def;
pub use signatures::{fifths_to_key, mei_clef_to_name};

use conversion::{
    convert_chord, convert_drum_chord, convert_drum_note, convert_mrest, convert_note,
    convert_pitched_rest, convert_rest,
};

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
    /// Lyrics attached to this staff (from \addlyrics, \lyricsto, etc.).
    lyrics: Vec<lyrics::LyricsInfo>,
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

/// Information about a ChordNames context found alongside staves.
struct ChordNamesInfo<'a> {
    /// Context name, if any.
    name: Option<String>,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
    /// The music content (chord-mode entries).
    music: &'a Music,
}

/// Information about a FiguredBass context found alongside staves.
struct FiguredBassInfo<'a> {
    /// Context name, if any.
    name: Option<String>,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
    /// The music content (figure-mode entries).
    music: &'a Music,
}

/// Result of analyzing the context hierarchy.
struct StaffLayout<'a> {
    group: Option<GroupInfo>,
    staves: Vec<StaffInfo<'a>>,
    /// ChordNames contexts found at the same level as staves.
    chord_names: Vec<ChordNamesInfo<'a>>,
    /// FiguredBass contexts found at the same level as staves.
    figured_bass: Vec<FiguredBassInfo<'a>>,
}

/// Analyze the LilyPond music tree to extract staff structure.
///
/// Detects patterns like:
/// - `\new StaffGroup << \new Staff { } \new Staff { } >>`
/// - `\new PianoStaff << \new Staff { } \new Staff { } >>`
/// - `\new Staff { ... }` (single staff)
/// - `{ ... }` (bare music, single staff)
fn analyze_staves(music: &Music) -> StaffLayout<'_> {
    // Check for \addlyrics wrapping (music \addlyrics { ... })
    if let Some((inner_music, lyric_infos)) = lyrics::extract_addlyrics(music) {
        let mut layout = analyze_staves(inner_music);
        // Attach lyrics to the first staff
        if let Some(staff) = layout.staves.first_mut() {
            staff.lyrics = lyric_infos;
        }
        return layout;
    }

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
            let (staves, chord_names, figured_bass) =
                extract_staves_chords_figures_from_group(inner);
            if !staves.is_empty() {
                return StaffLayout {
                    group: Some(group),
                    staves,
                    chord_names,
                    figured_bass,
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
                    lyrics: Vec::new(),
                }],
                chord_names: Vec::new(),
                figured_bass: Vec::new(),
            };
        }

        // FiguredBass context at top level (e.g. \figures { ... })
        if context_type == "FiguredBass" {
            return StaffLayout {
                group: None,
                staves: Vec::new(),
                chord_names: Vec::new(),
                figured_bass: vec![FiguredBassInfo {
                    name: name.clone(),
                    with_block: with_block.clone(),
                    music: inner,
                }],
            };
        }

        // Unknown context type -- treat inner music as bare
        return analyze_staves(inner);
    }

    // Check if simultaneous music contains \new Staff / \new Lyrics children
    if let Music::Simultaneous(items) = music {
        let (staves, chord_names, figured_bass) =
            extract_staves_chords_figures_from_simultaneous(items);
        if !staves.is_empty() {
            // Check for \lyricsto targeting named voices
            let mut layout = StaffLayout {
                group: None,
                staves,
                chord_names,
                figured_bass,
            };
            attach_lyricsto_from_simultaneous(items, &mut layout.staves);
            return layout;
        }
    }

    // Bare \drummode { ... } → treat as DrumStaff
    if let Music::DrumMode { body } = music {
        let voices = extract_voices(body);
        return StaffLayout {
            group: None,
            staves: vec![StaffInfo {
                n: 1,
                name: None,
                context_type: "DrumStaff".to_string(),
                with_block: None,
                voices,
                lyrics: Vec::new(),
            }],
            chord_names: Vec::new(),
            figured_bass: Vec::new(),
        };
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
            lyrics: Vec::new(),
        }],
        chord_names: Vec::new(),
        figured_bass: Vec::new(),
    }
}

/// Scan simultaneous items for `\lyricsto` constructs and attach them to
/// the named staff.
fn attach_lyricsto_from_simultaneous(items: &[Music], staves: &mut [StaffInfo<'_>]) {
    for item in items {
        // Direct \lyricsto
        if let Some(info) = lyrics::extract_lyricsto(item) {
            attach_lyricsto_info(info, staves);
            continue;
        }
        // \new Lyrics \lyricsto "..." { ... }
        if let Music::ContextedMusic {
            context_type,
            music: inner,
            ..
        } = item
            && context_type == "Lyrics"
            && let Some(info) = lyrics::extract_lyricsto(inner)
        {
            attach_lyricsto_info(info, staves);
        }
    }
}

/// Attach a LyricsTo info to the staff whose name matches the voice_id.
fn attach_lyricsto_info(info: lyrics::LyricsInfo, staves: &mut [StaffInfo<'_>]) {
    if let lyrics::LyricsStyle::LyricsTo { ref voice_id } = info.style {
        // Find the staff with this name
        for staff in staves.iter_mut() {
            if staff.name.as_deref() == Some(voice_id) {
                staff.lyrics.push(info);
                return;
            }
        }
        // If no named match found, attach to first staff
        if let Some(staff) = staves.first_mut() {
            staff.lyrics.push(info);
        }
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

/// Extract staff, chord-names, and figured-bass infos from a group context.
fn extract_staves_chords_figures_from_group(
    music: &Music,
) -> (
    Vec<StaffInfo<'_>>,
    Vec<ChordNamesInfo<'_>>,
    Vec<FiguredBassInfo<'_>>,
) {
    match music {
        Music::Simultaneous(items) => extract_staves_chords_figures_from_simultaneous(items),
        _ => (Vec::new(), Vec::new(), Vec::new()),
    }
}

/// Extract staff, chord-names, and figured-bass infos from a simultaneous music list.
fn extract_staves_chords_figures_from_simultaneous<'a>(
    items: &'a [Music],
) -> (
    Vec<StaffInfo<'a>>,
    Vec<ChordNamesInfo<'a>>,
    Vec<FiguredBassInfo<'a>>,
) {
    let mut staves = Vec::new();
    let mut chord_names = Vec::new();
    let mut figured_bass = Vec::new();
    let mut n = 1u32;

    for item in items {
        if let Music::ContextedMusic {
            context_type,
            name,
            with_block,
            music: inner,
            ..
        } = item
        {
            if is_staff_context(context_type) || is_voice_context(context_type) {
                let voices = extract_voices(inner);
                staves.push(StaffInfo {
                    n,
                    name: name.clone(),
                    context_type: context_type.clone(),
                    with_block: with_block.clone(),
                    voices,
                    lyrics: Vec::new(),
                });
                n += 1;
            } else if context_type == "ChordNames" {
                chord_names.push(ChordNamesInfo {
                    name: name.clone(),
                    with_block: with_block.clone(),
                    music: inner,
                });
            } else if context_type == "FiguredBass" {
                figured_bass.push(FiguredBassInfo {
                    name: name.clone(),
                    with_block: with_block.clone(),
                    music: inner,
                });
            }
        }
    }

    (staves, chord_names, figured_bass)
}

/// Check if a context type is a voice-level context.
fn is_voice_context(ctx: &str) -> bool {
    matches!(ctx, "Voice" | "CueVoice" | "NullVoice")
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

        // Store lyrics style info in label for roundtrip
        let lyrics_label = lyrics::build_lyrics_label(&staff_info.lyrics);
        if !lyrics_label.is_empty() {
            if !label.is_empty() {
                label.push('|');
            }
            label.push_str(&lyrics_label);
        }

        if !label.is_empty() {
            staff_def.labelled.label = Some(label);
        }

        staff_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
    }

    // Store chord-names context info in staffGrp label for roundtrip
    if !layout.chord_names.is_empty() {
        let cn_label = build_chord_names_label(&layout.chord_names);
        if !cn_label.is_empty() {
            match &mut staff_grp.common.label {
                Some(existing) => {
                    existing.push('|');
                    existing.push_str(&cn_label);
                }
                None => staff_grp.common.label = Some(cn_label),
            }
        }
    }

    // Store figured-bass context info in staffGrp label for roundtrip
    if !layout.figured_bass.is_empty() {
        let fb_label = build_figured_bass_label(&layout.figured_bass);
        if !fb_label.is_empty() {
            match &mut staff_grp.common.label {
                Some(existing) => {
                    existing.push('|');
                    existing.push_str(&fb_label);
                }
                None => staff_grp.common.label = Some(fb_label),
            }
        }
    }

    let mut score_def = ScoreDef::default();
    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));
    score_def
}

/// Build a label segment for chord-names context info.
///
/// Format: `lilypond:chordnames[,name=Name][,with={serialized}]`
fn build_chord_names_label(chord_names: &[ChordNamesInfo<'_>]) -> String {
    // For now, support one ChordNames context
    if let Some(cn) = chord_names.first() {
        let mut parts = vec!["lilypond:chordnames".to_string()];
        if let Some(name) = &cn.name {
            parts.push(format!("name={name}"));
        }
        if let Some(with_items) = &cn.with_block
            && !with_items.is_empty()
        {
            let with_str = serialize_with_block(with_items);
            parts.push(format!("with={with_str}"));
        }
        parts.join(",")
    } else {
        String::new()
    }
}

/// Build a label segment for figured-bass context info.
///
/// Format: `lilypond:figuredbass[,name=Name][,with={serialized}]`
fn build_figured_bass_label(figured_bass: &[FiguredBassInfo<'_>]) -> String {
    // Support one FiguredBass context (same pattern as ChordNames)
    if let Some(fb) = figured_bass.first() {
        let mut parts = vec!["lilypond:figuredbass".to_string()];
        if let Some(name) = &fb.name {
            parts.push(format!("name={name}"));
        }
        if let Some(with_items) = &fb.with_block
            && !with_items.is_empty()
        {
            let with_str = serialize_with_block(with_items);
            parts.push(format!("with={with_str}"));
        }
        parts.join(",")
    } else {
        String::new()
    }
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
fn build_section_from_staves(layout: &StaffLayout<'_>) -> Result<Section, ImportError> {
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
                            apply_grace_to_note(&mut mei_note, gt);
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
                        is_chord_repetition,
                    } => {
                        id_counter += 1;
                        let mut mei_chord =
                            convert_chord(pitches, duration.as_ref(), &mut id_counter);
                        if *is_chord_repetition {
                            mei_chord.common.label = Some("lilypond:chord-rep".to_string());
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
                            apply_grace_to_chord(&mut mei_chord, gt);
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
                            let harm = make_harm(ce, note_id, staff_info.n, harm_counter);
                            measure.children.push(MeasureChild::Harm(Box::new(harm)));
                        } else {
                            pending_chord_names.push((ce.clone(), staff_info.n));
                        }
                        continue;
                    }
                    LyEvent::FigureEvent(fe) => {
                        fb_counter += 1;
                        let fb = make_fb(fe, staff_info.n, fb_counter);
                        measure.children.push(MeasureChild::Fb(Box::new(fb)));
                        continue;
                    }
                    LyEvent::DrumEvent(dn) => {
                        id_counter += 1;
                        let n = convert_drum_note(dn, id_counter);
                        let id = format!("ly-note-{}", id_counter);
                        let pe = dn.post_events.clone();
                        layer.children.push(LayerChild::Note(Box::new(n)));
                        (pe, id)
                    }
                    LyEvent::DrumChordEvent(dc) => {
                        id_counter += 1;
                        let n = convert_drum_chord(dc, id_counter);
                        let id = format!("ly-note-{}", id_counter);
                        let pe = dc.post_events.clone();
                        layer.children.push(LayerChild::Note(Box::new(n)));
                        (pe, id)
                    }
                    LyEvent::Skip(_)
                    | LyEvent::Clef(_)
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

                // Flush pending inline chord names
                for (ce, staff_n) in pending_chord_names.drain(..) {
                    harm_counter += 1;
                    let harm = make_harm(&ce, &current_id, staff_n, harm_counter);
                    measure.children.push(MeasureChild::Harm(Box::new(harm)));
                }

                // Flush pending tempo/mark/textMark events
                for ptm in pending_tempo_marks.drain(..) {
                    tempo_mark_counter += 1;
                    match ptm {
                        PendingTempoMark::Tempo(t) => {
                            let mei_tempo =
                                make_tempo(&t, &current_id, staff_info.n, tempo_mark_counter);
                            measure
                                .children
                                .push(MeasureChild::Tempo(Box::new(mei_tempo)));
                        }
                        PendingTempoMark::Mark(s) => {
                            let dir =
                                make_mark_dir(&s, &current_id, staff_info.n, tempo_mark_counter);
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                        PendingTempoMark::TextMark(s) => {
                            let dir = make_textmark_dir(
                                &s,
                                &current_id,
                                staff_info.n,
                                tempo_mark_counter,
                            );
                            measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                    }
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
                            wrap_last_in_btrem(&mut layer, *value, &mut ornam_counter);
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
                    let mut harm = make_harm(&ce, "", staff_n, harm_counter);
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
                lyrics::attach_lyrics_to_layer(&mut layer.children, &lyric_info.syllables, verse_n);
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
                let mut harm = make_harm(ce, "", 1, harm_counter);
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
                let fb = make_fb(fe, 1, fb_counter);
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
    make_artic_dir, make_dynam, make_ending_dir, make_fb, make_fing_dir, make_hairpin, make_harm,
    make_mark_dir, make_ornament_control_event, make_repeat_dir, make_slur, make_string_dir,
    make_tempo, make_textmark_dir, make_tuplet_span, wrap_last_in_btrem,
};

/// Parse a serialized `\tempo ...` string back into a Tempo AST node.
fn parse_tempo_from_serialized(s: &str) -> Option<crate::model::signature::Tempo> {
    use crate::parser::Parser;
    // Wrap in a parseable form: the serialized string is the full \tempo expression
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                if let Music::Tempo(t) = m {
                    return Some(t.clone());
                }
            }
        }
        if let ToplevelExpression::Music(Music::Tempo(t)) = item {
            return Some(t.clone());
        }
    }
    None
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
                        | Music::ChordRepetition(_)
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

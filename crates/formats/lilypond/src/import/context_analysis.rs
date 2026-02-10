//! Context analysis — extract staff structure from a LilyPond AST.
//!
//! Determines staves, voice streams, chord-names contexts, figured-bass
//! contexts, group wrappers, and builds the initial `ScoreDef` with label
//! metadata for lossless roundtrip.

use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffDef, StaffGrp, StaffGrpChild};

use crate::model::{self, ContextKeyword, ContextModItem, Music, ToplevelExpression};
use crate::serializer;

use super::events::{PitchContext, collect_events, extract_pitch_from_music};
use super::lyrics;
use super::signatures::apply_signatures_to_staff_def;
use super::utils::extract_voices;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// Information about a single staff extracted from the LilyPond AST.
pub(super) struct StaffInfo<'a> {
    /// Staff number (1-based).
    pub n: u32,
    /// Context name (e.g. "violin") if `\new Staff = "violin"`.
    pub name: Option<String>,
    /// Context type (e.g. "Staff").
    pub context_type: String,
    /// `\with { ... }` block items, if present.
    pub with_block: Option<Vec<ContextModItem>>,
    /// The music content for this staff (one or more voice streams).
    pub voices: Vec<Vec<&'a Music>>,
    /// Lyrics attached to this staff (from \addlyrics, \lyricsto, etc.).
    pub lyrics: Vec<lyrics::LyricsInfo>,
}

/// Information about a staff group wrapping multiple staves.
pub(super) struct GroupInfo {
    /// Context type (e.g. "StaffGroup", "PianoStaff").
    context_type: String,
    /// Context name, if any.
    name: Option<String>,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
}

/// Information about a ChordNames context found alongside staves.
pub(super) struct ChordNamesInfo<'a> {
    /// Context name, if any.
    name: Option<String>,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
    /// The music content (chord-mode entries).
    pub music: &'a Music,
}

/// Information about a FiguredBass context found alongside staves.
pub(super) struct FiguredBassInfo<'a> {
    /// Context name, if any.
    name: Option<String>,
    /// `\with { ... }` block items, if present.
    with_block: Option<Vec<ContextModItem>>,
    /// The music content (figure-mode entries).
    pub music: &'a Music,
}

/// Result of analyzing the context hierarchy.
pub(super) struct StaffLayout<'a> {
    pub group: Option<GroupInfo>,
    pub staves: Vec<StaffInfo<'a>>,
    /// ChordNames contexts found at the same level as staves.
    pub chord_names: Vec<ChordNamesInfo<'a>>,
    /// FiguredBass contexts found at the same level as staves.
    pub figured_bass: Vec<FiguredBassInfo<'a>>,
}

// ---------------------------------------------------------------------------
// Staff analysis
// ---------------------------------------------------------------------------

/// Analyze the LilyPond music tree to extract staff structure.
///
/// Detects patterns like:
/// - `\new StaffGroup << \new Staff { } \new Staff { } >>`
/// - `\new PianoStaff << \new Staff { } \new Staff { } >>`
/// - `\new Staff { ... }` (single staff)
/// - `{ ... }` (bare music, single staff)
pub(super) fn analyze_staves(music: &Music) -> StaffLayout<'_> {
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
pub(super) fn is_staff_group_context(ctx: &str) -> bool {
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
pub(super) fn is_staff_context(ctx: &str) -> bool {
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
pub(super) fn build_score_def_from_staves(layout: &StaffLayout<'_>) -> ScoreDef {
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

// ---------------------------------------------------------------------------
// Label builders
// ---------------------------------------------------------------------------

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

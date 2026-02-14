//! Direction/dynamics/tempo conversion from MusicXML to MEI.
//!
//! This module handles conversion of MusicXML direction elements to MEI control events:
//! - `<dynamics>` → `<dynam>`
//! - `<wedge>` → `<hairpin>`
//! - `<metronome>` → `<tempo>`
//! - `<words>` → `<dir>`
//! - Rehearsal, Segno, Coda, Pedal, OctaveShift, and other direction types → `<dir>` with
//!   @label set to "musicxml:<type>" for roundtrip (export maps label back to MusicXML).

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::import::utils::{
    beat_unit_string_to_duration, dynamics_value_to_string, format_metronome_text,
};
use crate::model::data::AboveBelow;
use crate::model::direction::{
    Direction, DirectionTypeContent, MetronomeContent, OctaveShiftType, PedalType, WedgeType,
};
use tusk_model::data::{
    DataAugmentdot, DataBeat, DataBoolean, DataStaffrel, DataStaffrelBasic, DataTempovalue,
};
use tusk_model::elements::{Dir, DirChild, Dynam, DynamChild, Hairpin, Tempo, TempoChild};

/// Label prefix for MEI dir elements that roundtrip to a specific MusicXML direction type.
#[allow(dead_code)]
pub const MXML_DIR_LABEL_PREFIX: &str = "musicxml:";

// ============================================================================
// Direction to Control Event Conversion
// ============================================================================

/// Result of converting a MusicXML direction to MEI control events.
///
/// A single MusicXML direction can produce multiple MEI control events,
/// for example when a direction contains both dynamics and a wedge.
pub enum DirectionConversionResult {
    /// Dynamic indication (f, p, mf, etc.)
    Dynam(Dynam),
    /// Hairpin/wedge (crescendo, diminuendo)
    Hairpin(Hairpin),
    /// Tempo indication
    Tempo(Tempo),
    /// General directive text
    Dir(Dir),
}

impl DirectionConversionResult {
    /// Get the xml:id of the MEI element wrapped by this result.
    pub fn element_id(&self) -> Option<&str> {
        match self {
            DirectionConversionResult::Dynam(d) => d.common.xml_id.as_deref(),
            DirectionConversionResult::Hairpin(h) => h.common.xml_id.as_deref(),
            DirectionConversionResult::Tempo(t) => t.common.xml_id.as_deref(),
            DirectionConversionResult::Dir(d) => d.common.xml_id.as_deref(),
        }
    }
}

/// Convert a MusicXML direction to MEI control events.
///
/// MusicXML `<direction>` elements can contain multiple direction types.
/// Each direction type is converted to the appropriate MEI control event:
/// - `<dynamics>` → `<dynam>`
/// - `<wedge>` → `<hairpin>`
/// - `<metronome>` → `<tempo>`
/// - `<words>` → `<dir>` (or `<tempo>` if it contains tempo-like text)
///
/// # Arguments
///
/// * `direction` - The MusicXML direction to convert
/// * `ctx` - The conversion context for tracking state
///
/// # Returns
///
/// A vector of MEI control events, one for each direction type in the input.
pub fn convert_direction(
    direction: &Direction,
    ctx: &mut ConversionContext,
) -> ConversionResult<Vec<DirectionConversionResult>> {
    let mut results = Vec::new();

    // Calculate timestamp for control events
    let tstamp = calculate_tstamp(direction, ctx);
    // Use global MEI staff number from context, not within-part MusicXML staff.
    // MusicXML <staff> is within-part (e.g., piano staff 1), but MEI @staff is global.
    let staff = ctx.current_staff();
    let place = convert_placement(direction.placement.as_ref());

    for direction_type in &direction.direction_types {
        match &direction_type.content {
            DirectionTypeContent::Dynamics(dynamics) => {
                let dynam = convert_dynamics(dynamics, tstamp.clone(), staff, place.clone(), ctx);
                results.push(DirectionConversionResult::Dynam(dynam));
            }
            DirectionTypeContent::Wedge(wedge) => {
                if let Some(hairpin) =
                    convert_wedge(wedge, tstamp.clone(), staff, place.clone(), ctx)
                {
                    results.push(DirectionConversionResult::Hairpin(hairpin));
                }
            }
            DirectionTypeContent::Metronome(metronome) => {
                let tempo = convert_metronome(metronome, tstamp.clone(), staff, place.clone(), ctx);
                results.push(DirectionConversionResult::Tempo(tempo));
            }
            DirectionTypeContent::Words(words) => {
                let dir = convert_words(words, tstamp.clone(), staff, place.clone(), ctx);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Rehearsal(rehearsals) => {
                let text = rehearsals
                    .iter()
                    .map(|r| r.value.as_str())
                    .collect::<Vec<_>>()
                    .join("");
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:rehearsal",
                    &text,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Segno(_) => {
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:segno",
                    "",
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Coda(_) => {
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:coda",
                    "",
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Symbol(symbols) => {
                let text = symbols
                    .iter()
                    .map(|s| s.value.as_str())
                    .collect::<Vec<_>>()
                    .join("");
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:symbol",
                    &text,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Dashes(dashes) => {
                let text = dash_bracket_type_to_str(dashes.dash_type);
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:dashes",
                    text,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Bracket(bracket) => {
                let text = format!(
                    "{}:{}",
                    dash_bracket_type_to_str(bracket.bracket_type),
                    line_end_to_str(bracket.line_end)
                );
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:bracket",
                    &text,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Pedal(pedal) => {
                let text = pedal_type_to_str(pedal.pedal_type);
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:pedal",
                    text,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::OctaveShift(shift) => {
                let text = format!(
                    "{}:{}",
                    octave_shift_type_to_str(shift.shift_type),
                    shift.size.unwrap_or(8)
                );
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:octave-shift",
                    &text,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::HarpPedals(hp) => {
                let json = serde_json::to_string(hp).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:harp-pedals",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Damp(_) => {
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:damp",
                    "",
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::DampAll(_) => {
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:damp-all",
                    "",
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Eyeglasses(_) => {
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:eyeglasses",
                    "",
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::StringMute(sm) => {
                let json = serde_json::to_string(sm).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:string-mute",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Scordatura(sc) => {
                let json = serde_json::to_string(sc).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:scordatura",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Image(img) => {
                let json = serde_json::to_string(img).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:image",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::PrincipalVoice(pv) => {
                let json = serde_json::to_string(pv).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:principal-voice",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Percussion(perc) => {
                let json = serde_json::to_string(perc).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:percussion",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::AccordionRegistration(ar) => {
                let json = serde_json::to_string(ar).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:accordion-registration",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::StaffDivide(sd) => {
                let json = serde_json::to_string(sd).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:staff-divide",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::OtherDirection(other) => {
                let json = serde_json::to_string(other).unwrap_or_default();
                let dir = dir_with_label(
                    tstamp.clone(),
                    staff,
                    place.clone(),
                    ctx,
                    "musicxml:other",
                    &json,
                );
                results.push(DirectionConversionResult::Dir(dir));
            }
        }
    }

    // Store direction-level sound data in ExtensionStore for roundtrip.
    // A MusicXML <direction> can have a <sound> child — store its JSON keyed by
    // each produced MEI element's ID so export can reconstruct direction.sound.
    if let Some(ref sound) = direction.sound {
        if let Ok(json) = serde_json::to_string(sound) {
            let escaped = json.replace('|', "\\u007c");
            for result in &results {
                if let Some(id) = result.element_id() {
                    ctx.ext_store_mut()
                        .insert_direction_sound_json(id.to_string(), escaped.clone());
                }
            }
        }
    }

    Ok(results)
}

/// Convert MusicXML placement (above/below) to MEI DataStaffrel.
pub(crate) fn convert_placement(placement: Option<&AboveBelow>) -> Option<DataStaffrel> {
    placement.map(|p| match p {
        AboveBelow::Above => DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above),
        AboveBelow::Below => DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below),
    })
}

/// Calculate the timestamp (beat position) for a direction.
///
/// Converts beat position to beats, applies offset, and shifts to 1-based MEI tstamp.
fn calculate_tstamp(direction: &Direction, ctx: &ConversionContext) -> DataBeat {
    let mut beat_position = ctx.beat_position_in_beats();

    // Apply offset if present (offset is in divisions)
    if let Some(ref offset) = direction.offset {
        beat_position += ctx.divisions_to_beats(offset.value);
    }

    // MEI tstamp is 1-based (beat 1 is the first beat)
    DataBeat::from(beat_position + 1.0)
}

/// Build a Dir with a label (for MusicXML direction-type roundtrip) and optional text.
fn dir_with_label(
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
    label: &str,
    text: &str,
) -> Dir {
    let mut dir = Dir::default();
    let dir_id = ctx.generate_id_with_suffix("dir");
    dir.common.xml_id = Some(dir_id);
    dir.common.label = Some(label.to_string());
    dir.dir_log.tstamp = Some(tstamp);
    dir.dir_log.staff = Some((staff as u64).to_string());
    dir.dir_vis.place = place;
    if !text.is_empty() {
        dir.children.push(DirChild::Text(text.to_string()));
    }
    dir
}

fn dash_bracket_type_to_str(t: crate::model::data::StartStopContinue) -> &'static str {
    use crate::model::data::StartStopContinue;
    match t {
        StartStopContinue::Start => "start",
        StartStopContinue::Stop => "stop",
        StartStopContinue::Continue => "continue",
    }
}

fn line_end_to_str(t: crate::model::direction::LineEnd) -> &'static str {
    use crate::model::direction::LineEnd;
    match t {
        LineEnd::Up => "up",
        LineEnd::Down => "down",
        LineEnd::Both => "both",
        LineEnd::Arrow => "arrow",
        LineEnd::None => "none",
    }
}

fn pedal_type_to_str(t: PedalType) -> &'static str {
    match t {
        PedalType::Start => "start",
        PedalType::Stop => "stop",
        PedalType::Sostenuto => "sostenuto",
        PedalType::Change => "change",
        PedalType::Continue => "continue",
        PedalType::Discontinue => "discontinue",
        PedalType::Resume => "resume",
    }
}

fn octave_shift_type_to_str(t: OctaveShiftType) -> &'static str {
    match t {
        OctaveShiftType::Up => "up",
        OctaveShiftType::Down => "down",
        OctaveShiftType::Stop => "stop",
        OctaveShiftType::Continue => "continue",
    }
}

/// Convert MusicXML dynamics to MEI dynam element.
///
/// Maps dynamic markings:
/// - ppp, pp, p, mp, mf, f, ff, fff → text content
/// - Combined dynamics (sfp, sfz, etc.) → text content
fn convert_dynamics(
    dynamics: &crate::model::direction::Dynamics,
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
) -> Dynam {
    let mut dynam = Dynam::default();

    // Generate and set xml:id
    let dynam_id = ctx.generate_id_with_suffix("dynam");
    dynam.common.xml_id = Some(dynam_id);

    // Set timestamp and staff
    dynam.dynam_log.tstamp = Some(tstamp);
    dynam.dynam_log.staff = Some((staff as u64).to_string());

    // Set placement
    dynam.dynam_vis.place = place;

    // Convert dynamics values to text content
    let text_content = dynamics
        .values
        .iter()
        .map(dynamics_value_to_string)
        .collect::<Vec<_>>()
        .join("");

    dynam.children.push(DynamChild::Text(text_content));

    dynam
}

/// Convert MusicXML wedge to MEI hairpin element.
///
/// Maps wedge types:
/// - crescendo/diminuendo → hairpin with form="cres"/"dim", registers as pending
/// - stop → resolves pending hairpin, adds CompletedHairpin with tstamp2
/// - continue → ignored (system-break continuation, no MEI equivalent)
///
/// Returns Some(Hairpin) for start wedges, None for stop/continue.
fn convert_wedge(
    wedge: &crate::model::direction::Wedge,
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
) -> Option<Hairpin> {
    use crate::context::PendingHairpin;
    use crate::model::data::YesNo;
    use tusk_model::data::{DataColor, DataColorvalues};

    match wedge.wedge_type {
        WedgeType::Crescendo | WedgeType::Diminuendo => {
            let mut hairpin = Hairpin::default();

            // Generate and set xml:id
            let hairpin_id = ctx.generate_id_with_suffix("hairpin");
            hairpin.common.xml_id = Some(hairpin_id.clone());

            // Map original ID if present
            if let Some(ref orig_id) = wedge.id {
                ctx.map_id(orig_id, hairpin_id.clone());
            }

            // Set form (cres or dim); MEI uses string values in @form
            hairpin.hairpin_log.form = Some(match wedge.wedge_type {
                WedgeType::Crescendo => "cres".to_string(),
                WedgeType::Diminuendo => "dim".to_string(),
                _ => unreachable!(),
            });

            // Set niente if present
            if let Some(YesNo::Yes) = wedge.niente {
                hairpin.hairpin_log.niente = Some(DataBoolean::True);
            }

            // Set timestamp and staff
            hairpin.hairpin_log.tstamp = Some(tstamp.clone());
            hairpin.hairpin_log.staff = Some((staff as u64).to_string());

            // Set placement
            hairpin.hairpin_vis.place = place;

            // Map color
            if let Some(ref color) = wedge.color {
                hairpin.hairpin_vis.color = Some(DataColor::MeiDataColorvalues(DataColorvalues(
                    color.clone(),
                )));
            }

            // Register as pending for later stop resolution
            let part_id = ctx.position().part_id.clone().unwrap_or_default();
            ctx.add_pending_hairpin(PendingHairpin {
                hairpin_id,
                part_id,
                number: wedge.number.unwrap_or(1),
                start_measure_idx: ctx.measure_idx(),
                start_tstamp: tstamp.0,
                mei_staff: staff,
                start_spread: wedge.spread,
            });

            Some(hairpin)
        }
        WedgeType::Stop => {
            // Resolve the pending hairpin and compute tstamp2
            let part_id = ctx.position().part_id.clone().unwrap_or_default();
            let number = wedge.number.unwrap_or(1);
            if let Some(pending) = ctx.resolve_hairpin(&part_id, number) {
                let stop_tstamp = tstamp.0; // 1-based beat at stop point
                let measure_offset = ctx.measure_idx() - pending.start_measure_idx;
                // MEI tstamp2 format: "Nm+B" where N = measures ahead, B = beat in target measure
                let tstamp2 = format!("{measure_offset}m+{stop_tstamp}");
                ctx.add_completed_hairpin(crate::context::CompletedHairpin {
                    hairpin_id: pending.hairpin_id,
                    tstamp2,
                    stop_spread: wedge.spread,
                });
            }
            None
        }
        WedgeType::Continue => {
            // Continue wedges indicate system-break continuation;
            // no separate MEI element needed
            None
        }
    }
}

/// Convert MusicXML metronome to MEI tempo element.
///
/// Maps metronome content:
/// - beat-unit + per-minute → tempo with mm, mm.unit attributes
fn convert_metronome(
    metronome: &crate::model::direction::Metronome,
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
) -> Tempo {
    let mut tempo = Tempo::default();

    // Generate and set xml:id
    let tempo_id = ctx.generate_id_with_suffix("tempo");
    tempo.common.xml_id = Some(tempo_id);

    // Set timestamp and staff
    tempo.tempo_log.tstamp = Some(tstamp);
    tempo.tempo_log.staff = Some((staff as u64).to_string());

    // Set placement
    tempo.tempo_vis.place = place;

    // Set function to instantaneous (static tempo)
    tempo.tempo_log.func = Some("instantaneous".to_string());

    // Convert metronome content
    match &metronome.content {
        MetronomeContent::BeatUnit {
            beat_unit,
            beat_unit_dots,
            per_minute,
            ..
        } => {
            // MEI @mm.unit
            if let Some(dur) = beat_unit_string_to_duration(beat_unit) {
                tempo.tempo_log.mm_unit = Some(dur);
            }

            // Set dots if present
            if !beat_unit_dots.is_empty() {
                tempo.tempo_log.mm_dots = Some(DataAugmentdot::from(beat_unit_dots.len() as u64));
            }

            // Parse per-minute value
            if let Ok(mm_value) = per_minute.parse::<f64>() {
                tempo.tempo_log.mm = Some(DataTempovalue::from(mm_value));
            }

            // Also add text content for display
            let text = format_metronome_text(beat_unit, beat_unit_dots.len(), per_minute);
            tempo.children.push(TempoChild::Text(text));
        }
        MetronomeContent::BeatUnitEquivalent(modulation) => {
            // Metric modulation: beat-unit = beat-unit
            // Set function to metricmod
            tempo.tempo_log.func = Some("metricmod".to_string());

            // Add text content for metric modulation
            let text = format!("{} = {}", modulation.beat_unit_1, modulation.beat_unit_2);
            tempo.children.push(TempoChild::Text(text));
        }
        MetronomeContent::MetronomeNotes(_) => {
            // Complex metric relationship (e.g., swing notation)
            // Set function to metricmod
            tempo.tempo_log.func = Some("metricmod".to_string());

            // Store text placeholder
            tempo.children.push(TempoChild::Text("metric".to_string()));
        }
    }

    // Store full metronome JSON in ExtensionStore for lossless roundtrip
    // of beat-unit-tied, metronome-note, and other details not captured in MEI
    if let Some(ref id) = tempo.common.xml_id {
        if let Ok(json) = serde_json::to_string(metronome) {
            let escaped = json.replace('|', "\\u007c");
            ctx.ext_store_mut()
                .insert_metronome_json(id.clone(), escaped);
        }
    }

    tempo
}

/// Convert MusicXML words to MEI dir element.
///
/// Words directions are converted to general directives.
fn convert_words(
    words: &[crate::model::direction::Words],
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
) -> Dir {
    use tusk_model::data::{DataColor, DataColorvalues};

    let mut dir = Dir::default();

    // Generate and set xml:id
    let dir_id = ctx.generate_id_with_suffix("dir");
    dir.common.xml_id = Some(dir_id);

    // Set timestamp and staff
    dir.dir_log.tstamp = Some(tstamp);
    dir.dir_log.staff = Some((staff as u64).to_string());

    // Set placement
    dir.dir_vis.place = place;

    // Map color from first words element to MEI @color
    if let Some(first) = words.first() {
        if let Some(ref color) = first.color {
            dir.dir_vis.color = Some(DataColor::MeiDataColorvalues(DataColorvalues(
                color.clone(),
            )));
        }
    }

    // Combine all words text into dir content
    for word in words {
        dir.children.push(DirChild::Text(word.value.clone()));
    }

    // Store full words visual attrs as JSON-in-label for lossless roundtrip
    let has_visual_attrs = words.iter().any(|w| {
        w.font_family.is_some()
            || w.font_style.is_some()
            || w.font_size.is_some()
            || w.font_weight.is_some()
            || w.color.is_some()
            || w.enclosure.is_some()
            || w.halign.is_some()
            || w.valign.is_some()
            || w.justify.is_some()
            || w.default_x.is_some()
            || w.default_y.is_some()
            || w.relative_x.is_some()
            || w.relative_y.is_some()
    });
    if has_visual_attrs {
        if let Ok(json) = serde_json::to_string(words) {
            // Escape pipe characters in JSON to avoid breaking label segment splitting
            let escaped = json.replace('|', "\\u007c");
            append_dir_label(&mut dir, &format!("musicxml:words-vis,{escaped}"));
        }
    }

    // Dual-path: store typed DirectionVisualData in ExtensionStore
    if has_visual_attrs {
        if let Some(ref id) = dir.common.xml_id {
            use tusk_model::musicxml_ext::{DirectionVisualData, VisualAttrs, WordsVisualData};
            let words_vis: Vec<WordsVisualData> = words
                .iter()
                .map(|w| WordsVisualData {
                    value: w.value.clone(),
                    visual: VisualAttrs {
                        font_family: w.font_family.clone(),
                        font_size: w
                            .font_size
                            .as_ref()
                            .and_then(|fs| serde_json::to_value(fs).ok().and_then(|v| v.as_f64())),
                        font_style: w.font_style.as_ref().map(|s| s.to_string()),
                        font_weight: w.font_weight.as_ref().map(|wt| wt.to_string()),
                        color: w.color.clone(),
                        default_x: w.default_x,
                        default_y: w.default_y,
                        relative_x: w.relative_x,
                        relative_y: w.relative_y,
                    },
                    enclosure: w.enclosure.as_ref().map(|e| e.to_string()),
                    halign: w.halign.as_ref().map(|h| h.to_string()),
                    valign: w.valign.as_ref().map(|v| v.to_string()),
                    justify: w.justify.as_ref().map(|j| j.to_string()),
                    id: w.id.clone(),
                })
                .collect();
            ctx.ext_store_mut().insert_direction_visual(
                id.clone(),
                DirectionVisualData {
                    words: words_vis,
                    wedge_color: None,
                    wedge_niente: None,
                },
            );
        }
    }

    dir
}

/// Append a label segment to a Dir element using '|' separator.
fn append_dir_label(dir: &mut Dir, segment: &str) {
    match &mut dir.common.label {
        Some(existing) => {
            existing.push('|');
            existing.push_str(segment);
        }
        None => {
            dir.common.label = Some(segment.to_string());
        }
    }
}

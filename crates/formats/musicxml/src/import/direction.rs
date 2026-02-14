//! Direction/dynamics/tempo conversion from MusicXML to MEI.
//!
//! This module handles conversion of MusicXML direction elements to MEI control events:
//! - `<dynamics>` → `<dynam>`
//! - `<wedge>` → `<hairpin>`
//! - `<metronome>` → `<tempo>`
//! - `<words>` → `<dir>`
//! - Rehearsal, Segno, Coda, Pedal, OctaveShift, and other direction types → `<dir>` with
//!   data stored in ExtensionStore for roundtrip.

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::import::utils::{
    beat_unit_string_to_duration, dynamics_value_to_string, format_metronome_text,
};
use crate::model::data::AboveBelow;
use crate::model::direction::{Direction, DirectionTypeContent, MetronomeContent, WedgeType};
use tusk_model::data::{
    DataAugmentdot, DataBeat, DataBoolean, DataStaffrel, DataStaffrelBasic, DataTempovalue,
};
use tusk_model::elements::{Dir, DirChild, Dynam, DynamChild, Hairpin, Tempo, TempoChild};
use tusk_model::musicxml_ext::DirectionContentData;
use tusk_model::musicxml_ext::{
    BeatUnitTiedData, MetricModulationData, MetronomeBeamData, MetronomeContentData, MetronomeData,
    MetronomeNoteData, MetronomeNotesData, MetronomeTupletData,
};

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
                let data = DirectionContentData::Rehearsal(rehearsals.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Segno(segnos) => {
                let data = DirectionContentData::Segno(segnos.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Coda(codas) => {
                let data = DirectionContentData::Coda(codas.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Symbol(symbols) => {
                let data = DirectionContentData::Symbol(symbols.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Dashes(dashes) => {
                let data = DirectionContentData::Dashes(dashes.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Bracket(bracket) => {
                let data = DirectionContentData::Bracket(bracket.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Pedal(pedal) => {
                let data = DirectionContentData::Pedal(pedal.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::OctaveShift(shift) => {
                let data = DirectionContentData::OctaveShift(shift.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::HarpPedals(hp) => {
                let data = DirectionContentData::HarpPedals(hp.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Damp(d) => {
                let data = DirectionContentData::Damp(d.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::DampAll(da) => {
                let data = DirectionContentData::DampAll(da.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Eyeglasses(eg) => {
                let data = DirectionContentData::Eyeglasses(eg.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::StringMute(sm) => {
                let data = DirectionContentData::StringMute(sm.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Scordatura(sc) => {
                let data = DirectionContentData::Scordatura(sc.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Image(img) => {
                let data = DirectionContentData::Image(img.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::PrincipalVoice(pv) => {
                let data = DirectionContentData::PrincipalVoice(pv.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::Percussion(perc) => {
                let data = DirectionContentData::Percussion(perc.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::AccordionRegistration(ar) => {
                let data = DirectionContentData::AccordionRegistration(ar.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::StaffDivide(sd) => {
                let data = DirectionContentData::StaffDivide(sd.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
            DirectionTypeContent::OtherDirection(other) => {
                let data = DirectionContentData::OtherDirection(other.clone());
                let dir = dir_with_ext(tstamp.clone(), staff, place.clone(), ctx, data);
                results.push(DirectionConversionResult::Dir(dir));
            }
        }
    }

    // Store direction-level sound data in ExtensionStore for roundtrip.
    // A MusicXML <direction> can have a <sound> child — store typed SoundData keyed
    // by each produced MEI element's ID so export can reconstruct direction.sound.
    if let Some(ref sound) = direction.sound {
        let data = crate::import::sound::build_sound_data(sound);
        for result in &results {
            if let Some(id) = result.element_id() {
                ctx.ext_store_mut()
                    .insert_direction_sound(id.to_string(), data.clone());
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

/// Build a Dir and store its direction content data in ExtensionStore for roundtrip.
fn dir_with_ext(
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
    data: DirectionContentData,
) -> Dir {
    let mut dir = Dir::default();
    let dir_id = ctx.generate_id_with_suffix("dir");
    dir.common.xml_id = Some(dir_id.clone());
    dir.dir_log.tstamp = Some(tstamp);
    dir.dir_log.staff = Some((staff as u64).to_string());
    dir.dir_vis.place = place;
    ctx.ext_store_mut().insert_direction_content(dir_id, data);
    dir
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

    // Store typed metronome data in ExtensionStore for lossless roundtrip
    // of beat-unit-tied, metronome-note, and other details not captured in MEI
    if let Some(ref id) = tempo.common.xml_id {
        let data = build_metronome_data(metronome);
        ctx.ext_store_mut().insert_metronome(id.clone(), data);
    }

    tempo
}

/// Build typed `MetronomeData` from a MusicXML `Metronome`.
fn build_metronome_data(m: &crate::model::direction::Metronome) -> MetronomeData {
    use crate::model::data::{LeftCenterRight, Valign, YesNo};
    use crate::model::direction::{BeatUnitTied, MetronomeNote};

    fn convert_tied(tied: &[BeatUnitTied]) -> Vec<BeatUnitTiedData> {
        tied.iter()
            .map(|t| BeatUnitTiedData {
                unit: t.beat_unit.clone(),
                dots: t.beat_unit_dots.len() as u32,
            })
            .collect()
    }

    fn convert_note(n: &MetronomeNote) -> MetronomeNoteData {
        MetronomeNoteData {
            note_type: n.note_type.clone(),
            dots: n.dots.len() as u32,
            beams: n
                .beams
                .iter()
                .map(|b| MetronomeBeamData {
                    number: b.number,
                    value: b.value.clone(),
                })
                .collect(),
            tied: n
                .tied
                .as_ref()
                .map(|t| format!("{:?}", t.tied_type).to_lowercase()),
            tuplet: n.tuplet.as_ref().map(|t| MetronomeTupletData {
                tuplet_type: format!("{:?}", t.tuplet_type).to_lowercase(),
                bracket: t.bracket.as_ref().map(|b| *b == YesNo::Yes),
                show_number: t.show_number.clone(),
                actual_notes: t.actual_notes,
                normal_notes: t.normal_notes,
                normal_type: t.normal_type.clone(),
                normal_dots: t.normal_dots.len() as u32,
            }),
        }
    }

    let content = match &m.content {
        MetronomeContent::BeatUnit {
            beat_unit,
            beat_unit_dots,
            beat_unit_tied,
            per_minute,
        } => MetronomeContentData::BeatUnit {
            unit: beat_unit.clone(),
            dots: beat_unit_dots.len() as u32,
            tied: convert_tied(beat_unit_tied),
            pm: per_minute.clone(),
        },
        MetronomeContent::BeatUnitEquivalent(mod_) => {
            MetronomeContentData::Modulation(MetricModulationData {
                unit1: mod_.beat_unit_1.clone(),
                dots1: mod_.beat_unit_dots_1.len() as u32,
                tied1: convert_tied(&mod_.beat_unit_tied_1),
                unit2: mod_.beat_unit_2.clone(),
                dots2: mod_.beat_unit_dots_2.len() as u32,
                tied2: convert_tied(&mod_.beat_unit_tied_2),
            })
        }
        MetronomeContent::MetronomeNotes(notes) => {
            MetronomeContentData::Notes(MetronomeNotesData {
                arrows: notes.arrows,
                notes1: notes.notes_1.iter().map(convert_note).collect(),
                relation: notes.relation.clone(),
                notes2: notes.notes_2.iter().map(convert_note).collect(),
            })
        }
    };

    fn lcr_str(v: &LeftCenterRight) -> String {
        match v {
            LeftCenterRight::Left => "left".to_string(),
            LeftCenterRight::Center => "center".to_string(),
            LeftCenterRight::Right => "right".to_string(),
        }
    }

    fn valign_str(v: &Valign) -> String {
        match v {
            Valign::Top => "top".to_string(),
            Valign::Middle => "middle".to_string(),
            Valign::Bottom => "bottom".to_string(),
            Valign::Baseline => "baseline".to_string(),
        }
    }

    MetronomeData {
        content,
        parentheses: m.parentheses.as_ref().map(|p| *p == YesNo::Yes),
        print_object: m.print_object.as_ref().map(|p| *p == YesNo::Yes),
        justify: m.justify.as_ref().map(lcr_str),
        default_x: m.default_x,
        default_y: m.default_y,
        halign: m.halign.as_ref().map(lcr_str),
        valign: m.valign.as_ref().map(valign_str),
        id: m.id.clone(),
    }
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

    // Store typed DirectionVisualData in ExtensionStore for lossless roundtrip
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

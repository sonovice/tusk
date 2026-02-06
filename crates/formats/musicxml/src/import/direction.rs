//! Direction/dynamics/tempo conversion from MusicXML to MEI.
//!
//! This module handles conversion of MusicXML direction elements to MEI control events:
//! - `<dynamics>` → `<dynam>`
//! - `<wedge>` → `<hairpin>`
//! - `<metronome>` → `<tempo>`
//! - `<words>` → `<dir>`

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::import::utils::{
    beat_unit_string_to_duration, dynamics_value_to_string, format_metronome_text,
};
use crate::model::data::AboveBelow;
use crate::model::direction::{Direction, DirectionTypeContent, MetronomeContent, WedgeType};
use tusk_model::att::{AttHairpinLogForm, AttTempoLogFunc};
use tusk_model::data::{
    DataAugmentdot, DataBeat, DataBoolean, DataStaffrel, DataStaffrelBasic, DataTempovalue,
};
use tusk_model::elements::{Dir, DirChild, Dynam, DynamChild, Hairpin, Tempo, TempoChild};

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
    let staff = direction.staff.unwrap_or(ctx.current_staff());
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
            // Other direction types can be added in future phases
            _ => {}
        }
    }

    Ok(results)
}

/// Convert MusicXML placement (above/below) to MEI DataStaffrel.
fn convert_placement(placement: Option<&AboveBelow>) -> Option<DataStaffrel> {
    placement.map(|p| match p {
        AboveBelow::Above => DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above),
        AboveBelow::Below => DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Below),
    })
}

/// Calculate the timestamp (beat position) for a direction.
///
/// beat_position is in divisions (raw MusicXML duration units).
/// Convert to beats by dividing by divisions, then add offset (also in divisions),
/// then shift to 1-based MEI tstamp.
fn calculate_tstamp(direction: &Direction, ctx: &ConversionContext) -> DataBeat {
    let divisions = ctx.divisions();
    // Convert beat_position from divisions to beats
    let mut beat_position = if divisions > 0.0 {
        ctx.beat_position() / divisions
    } else {
        ctx.beat_position()
    };

    // Apply offset if present (offset is in divisions)
    if let Some(ref offset) = direction.offset {
        let offset_beats = if divisions > 0.0 {
            offset.value / divisions
        } else {
            offset.value
        };
        beat_position += offset_beats;
    }

    // MEI tstamp is 1-based (beat 1 is the first beat)
    DataBeat::from(beat_position + 1.0)
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
    dynam.dynam_log.staff = vec![staff as u64];

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
/// - crescendo → hairpin with form="cres"
/// - diminuendo → hairpin with form="dim"
/// - stop → None (closes a previous hairpin via context)
///
/// Returns None for stop wedges since they don't create new elements,
/// but rather close existing ones.
fn convert_wedge(
    wedge: &crate::model::direction::Wedge,
    tstamp: DataBeat,
    staff: u32,
    place: Option<DataStaffrel>,
    ctx: &mut ConversionContext,
) -> Option<Hairpin> {
    use crate::model::data::YesNo;

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

            // Set form (cres or dim)
            hairpin.hairpin_log.form = Some(match wedge.wedge_type {
                WedgeType::Crescendo => AttHairpinLogForm::Cres,
                WedgeType::Diminuendo => AttHairpinLogForm::Dim,
                _ => unreachable!(),
            });

            // Set niente if present
            if let Some(YesNo::Yes) = wedge.niente {
                hairpin.hairpin_log.niente = Some(DataBoolean::True);
            }

            // Set timestamp and staff
            hairpin.hairpin_log.tstamp = Some(tstamp);
            hairpin.hairpin_log.staff = vec![staff as u64];

            // Set placement
            hairpin.hairpin_vis.place = place;

            Some(hairpin)
        }
        WedgeType::Stop | WedgeType::Continue => {
            // Stop and continue wedges don't create new elements
            // In a full implementation, we would update the corresponding
            // start hairpin with tstamp2 or endid
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
    tempo.tempo_log.staff = vec![staff as u64];

    // Set placement
    tempo.tempo_vis.place = place;

    // Set function to instantaneous (static tempo)
    tempo.tempo_log.func = Some(AttTempoLogFunc::Instantaneous);

    // Convert metronome content
    match &metronome.content {
        MetronomeContent::BeatUnit {
            beat_unit,
            beat_unit_dots,
            per_minute,
        } => {
            // Convert beat unit to MEI duration
            if let Some(mm_unit) = beat_unit_string_to_duration(beat_unit) {
                tempo.tempo_log.mm_unit = Some(mm_unit);
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
            tempo.tempo_log.func = Some(AttTempoLogFunc::Metricmod);

            // Add text content for metric modulation
            let text = format!("{} = {}", modulation.beat_unit_1, modulation.beat_unit_2);
            tempo.children.push(TempoChild::Text(text));
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
    let mut dir = Dir::default();

    // Generate and set xml:id
    let dir_id = ctx.generate_id_with_suffix("dir");
    dir.common.xml_id = Some(dir_id);

    // Set timestamp and staff
    dir.dir_log.tstamp = Some(tstamp);
    dir.dir_log.staff = vec![staff as u64];

    // Set placement
    dir.dir_vis.place = place;

    // Combine all words text into dir content
    for word in words {
        dir.children.push(DirChild::Text(word.value.clone()));
    }

    dir
}

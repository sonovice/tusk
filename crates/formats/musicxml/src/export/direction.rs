//! MEI control events to MusicXML direction conversion.
//!
//! This module handles conversion of MEI control events (dynam, hairpin, dir, tempo)
//! to MusicXML direction elements.

use super::utils::convert_mei_duration_to_beat_unit;
use crate::context::ConversionContext;
use crate::model::data::{AboveBelow, StartStopContinue};
use crate::model::direction::{
    Bracket, Coda, Damp, DampAll, Dashes, Direction, DirectionType, DirectionTypeContent,
    Eyeglasses, HarpPedals, LineEnd, OctaveShift, OctaveShiftType, Pedal, PedalType,
    PrincipalVoice, Rehearsal, Segno, StaffDivide, Symbol, Words,
};
use crate::model::elements::MeasureContent;

/// Convert an MEI dynam element to a MusicXML direction with dynamics.
///
/// Maps:
/// - MEI `<dynam>` text content -> MusicXML dynamics element
/// - Standard dynamics (p, f, mf, etc.) -> specific MusicXML dynamic marks
/// - Non-standard dynamics -> MusicXML other-dynamics
/// - MEI `@tstamp` -> direction position (via offset from measure start)
/// - MEI `@staff` -> MusicXML direction staff
///
/// # Arguments
///
/// * `dynam` - The MEI dynam element to convert
/// * `ctx` - Conversion context
///
/// # Returns
///
/// A MusicXML Direction element, or None if conversion fails.
pub fn convert_mei_dynam(
    dynam: &tusk_model::elements::Dynam,
    ctx: &mut ConversionContext,
) -> Option<crate::model::direction::Direction> {
    use crate::model::direction::{Direction, DirectionType, DirectionTypeContent, Dynamics};
    use tusk_model::elements::DynamChild;

    // Extract text content from dynam element
    let text_content: String = dynam
        .children
        .iter()
        .map(|child| {
            let DynamChild::Text(t) = child;
            t.as_str()
        })
        .collect::<Vec<_>>()
        .join("");

    if text_content.is_empty() {
        ctx.add_warning("dynam", "Empty dynam element - skipping");
        return None;
    }

    // Parse the dynamic marking
    let dynamics_value = parse_dynamics_text(&text_content);

    let dynamics = Dynamics {
        values: vec![dynamics_value],
        placement: None,
    };

    let direction_type = DirectionType {
        content: DirectionTypeContent::Dynamics(dynamics),
        id: None,
    };

    let mut direction = Direction::new(vec![direction_type]);

    // Set staff: MEI @staff is global, MusicXML <staff> is within-part.
    // With 1:1 part→staff mapping, within-part staff is always 1.
    if dynam
        .dynam_log
        .staff
        .as_ref()
        .is_some_and(|s| !s.is_empty())
    {
        direction.staff = Some(1);
    }

    // Set placement from MEI @place (no default — only emit if explicitly set)
    direction.placement = convert_place_to_placement(&dynam.dynam_vis.place);

    // Preserve ID if present
    if let Some(ref xml_id) = dynam.common.xml_id {
        direction.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert tstamp to offset for proper repositioning on reimport
    direction.offset = convert_tstamp_to_offset(&dynam.dynam_log.tstamp, ctx);

    Some(direction)
}

/// Parse dynamics text to a MusicXML DynamicsValue.
pub(crate) fn parse_dynamics_text(text: &str) -> crate::model::direction::DynamicsValue {
    use crate::model::direction::DynamicsValue;

    match text.trim() {
        "pppppp" => DynamicsValue::Pppppp,
        "ppppp" => DynamicsValue::Ppppp,
        "pppp" => DynamicsValue::Pppp,
        "ppp" => DynamicsValue::Ppp,
        "pp" => DynamicsValue::Pp,
        "p" => DynamicsValue::P,
        "mp" => DynamicsValue::Mp,
        "mf" => DynamicsValue::Mf,
        "f" => DynamicsValue::F,
        "ff" => DynamicsValue::Ff,
        "fff" => DynamicsValue::Fff,
        "ffff" => DynamicsValue::Ffff,
        "fffff" => DynamicsValue::Fffff,
        "ffffff" => DynamicsValue::Ffffff,
        "fp" => DynamicsValue::Fp,
        "pf" => DynamicsValue::Pf,
        "sf" => DynamicsValue::Sf,
        "sfz" => DynamicsValue::Sfz,
        "sfp" => DynamicsValue::Sfp,
        "sfpp" => DynamicsValue::Sfpp,
        "sffz" => DynamicsValue::Sffz,
        "sfzp" => DynamicsValue::Sfzp,
        "rf" => DynamicsValue::Rf,
        "rfz" => DynamicsValue::Rfz,
        "fz" => DynamicsValue::Fz,
        "n" => DynamicsValue::N,
        other => DynamicsValue::OtherDynamics(other.to_string()),
    }
}

/// Convert an MEI hairpin element to MusicXML directions.
///
/// Maps:
/// - MEI `<hairpin form="cres">` -> MusicXML wedge type="crescendo"
/// - MEI `<hairpin form="dim">` -> MusicXML wedge type="diminuendo"
/// - MEI `@niente` -> MusicXML wedge niente attribute
/// - MEI `@tstamp`, `@tstamp2` -> wedge start and stop positions
///
/// # Arguments
///
/// * `hairpin` - The MEI hairpin element to convert
/// * `ctx` - Conversion context
///
/// # Returns
///
/// A vector of MusicXML Direction elements (typically one for start, one for stop if tstamp2 is present).
pub fn convert_mei_hairpin(
    hairpin: &tusk_model::elements::Hairpin,
    ctx: &mut ConversionContext,
) -> Vec<crate::model::direction::Direction> {
    use crate::model::data::YesNo;
    use crate::model::direction::{
        Direction, DirectionType, DirectionTypeContent, Wedge, WedgeType,
    };
    let mut directions = Vec::new();

    // Determine wedge type from form (MEI @form is string: "cres" or "dim")
    let wedge_type = match hairpin.hairpin_log.form.as_deref() {
        Some("cres") => WedgeType::Crescendo,
        Some("dim") => WedgeType::Diminuendo,
        None | Some(_) => {
            ctx.add_warning(
                "hairpin",
                "Hairpin without form attribute - defaulting to crescendo",
            );
            WedgeType::Crescendo
        }
    };

    let mut wedge = Wedge::new(wedge_type);

    // Convert niente attribute
    if hairpin.hairpin_log.niente.as_ref() == Some(&tusk_model::data::DataBoolean::True) {
        wedge.niente = Some(YesNo::Yes);
    }

    // Restore color from MEI @color
    if let Some(ref color) = hairpin.hairpin_vis.color {
        wedge.color = Some(convert_mei_color_to_string(color));
    }

    // Preserve ID
    if let Some(ref xml_id) = hairpin.common.xml_id {
        wedge.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    let direction_type = DirectionType {
        content: DirectionTypeContent::Wedge(wedge),
        id: None,
    };

    let mut direction = Direction::new(vec![direction_type]);

    // Set staff: MEI @staff is global, MusicXML <staff> is within-part (always 1 for 1:1 mapping)
    if hairpin
        .hairpin_log
        .staff
        .as_ref()
        .is_some_and(|s| !s.is_empty())
    {
        direction.staff = Some(1);
    }

    // Set placement from MEI @place (no default — only emit if explicitly set)
    direction.placement = convert_place_to_placement(&hairpin.hairpin_vis.place);

    // Convert tstamp to offset for proper repositioning on reimport
    direction.offset = convert_tstamp_to_offset(&hairpin.hairpin_log.tstamp, ctx);

    let staff_n = hairpin
        .hairpin_log
        .staff
        .as_ref()
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(1usize);

    directions.push(direction);

    // If hairpin has tstamp2, create a stop wedge (same measure or deferred)
    if let Some(ref tstamp2) = hairpin.hairpin_log.tstamp2 {
        let (measures_ahead, stop_beat) = parse_tstamp2(&tstamp2.0);

        // Restore stop spread from extension store
        let stop_spread = hairpin
            .common
            .xml_id
            .as_ref()
            .and_then(|id| ctx.ext_store().get(id))
            .and_then(|ext| ext.wedge_stop_spread);

        if measures_ahead == 0 {
            // Same-measure stop: emit stop wedge immediately
            let stop_dir = make_hairpin_stop_direction(stop_beat, stop_spread, ctx);
            directions.push(stop_dir);
        } else {
            // Cross-measure: defer to future measure
            // measures_remaining is 0-based: 0 means emit next time resolve is called
            ctx.add_deferred_hairpin_stop(crate::context::DeferredHairpinStop {
                measures_remaining: measures_ahead - 1,
                beat: stop_beat,
                staff: staff_n,
                spread: stop_spread,
            });
        }
    }

    directions
}

/// Parse MEI tstamp2 format "Nm+B" into (measures_ahead, beat).
///
/// Examples: "0m+3" → (0, 3.0), "2m+1" → (2, 1.0), "1m+2.5" → (1, 2.5)
fn parse_tstamp2(s: &str) -> (usize, f64) {
    if let Some((m_part, b_part)) = s.split_once("m+") {
        let measures = m_part.parse().unwrap_or(0);
        let beat = b_part.parse().unwrap_or(1.0);
        (measures, beat)
    } else {
        // Fallback: treat as beat in same measure
        (0, s.parse().unwrap_or(1.0))
    }
}

/// Create a stop wedge direction at the given beat position.
fn make_hairpin_stop_direction(
    beat: f64,
    spread: Option<f64>,
    ctx: &ConversionContext,
) -> crate::model::direction::Direction {
    use crate::model::direction::{
        Direction, DirectionType, DirectionTypeContent, Wedge, WedgeType,
    };

    let mut wedge = Wedge::new(WedgeType::Stop);
    wedge.spread = spread;

    let direction_type = DirectionType {
        content: DirectionTypeContent::Wedge(wedge),
        id: None,
    };

    let mut direction = Direction::new(vec![direction_type]);
    direction.staff = Some(1);

    // Convert beat to offset
    let beat_position = beat - 1.0; // 1-based → 0-based
    let offset_divisions = beat_position * ctx.divisions();
    direction.offset = Some(crate::model::direction::Offset::new(offset_divisions));

    direction
}

/// Resolve deferred hairpin stops that target the current measure.
///
/// Call at the start of each measure during export. Decrements the counter on
/// each deferred stop; those reaching 0 are emitted as stop wedge directions.
pub fn resolve_deferred_hairpin_stops(
    staff_n: usize,
    mxml_measure: &mut crate::model::elements::Measure,
    ctx: &mut ConversionContext,
) {
    let deferred = ctx.drain_deferred_hairpin_stops();
    for mut stop in deferred {
        if stop.staff != staff_n {
            // Not for this staff — re-defer without decrementing
            ctx.add_deferred_hairpin_stop(stop);
        } else if stop.measures_remaining > 0 {
            stop.measures_remaining -= 1;
            ctx.add_deferred_hairpin_stop(stop);
        } else {
            // Emit stop wedge in this measure
            let stop_dir = make_hairpin_stop_direction(stop.beat, stop.spread, ctx);
            mxml_measure
                .content
                .push(MeasureContent::Direction(Box::new(stop_dir)));
        }
    }
}

/// Convert an MEI dir (directive) element to a MusicXML direction.
///
/// If MEI dir has @label starting with "musicxml:", the label is used to emit the
/// corresponding MusicXML direction type (rehearsal, segno, coda, pedal, octave-shift, etc.)
/// for roundtrip. Otherwise, dir text is emitted as MusicXML words.
pub fn convert_mei_dir(
    dir: &tusk_model::elements::Dir,
    ctx: &mut ConversionContext,
) -> Option<Direction> {
    use tusk_model::elements::DirChild;

    let text_content: String = dir
        .children
        .iter()
        .map(|child| {
            let DirChild::Text(t) = child;
            t.as_str()
        })
        .collect::<Vec<_>>()
        .join("");

    let direction_type_content = match dir.common.label.as_deref() {
        Some("musicxml:rehearsal") => {
            DirectionTypeContent::Rehearsal(vec![Rehearsal::new(&text_content)])
        }
        Some("musicxml:segno") => DirectionTypeContent::Segno(vec![Segno::default()]),
        Some("musicxml:coda") => DirectionTypeContent::Coda(vec![Coda::default()]),
        Some("musicxml:symbol") => DirectionTypeContent::Symbol(vec![Symbol::new(&text_content)]),
        Some("musicxml:dashes") => {
            let dash_type = parse_start_stop_continue(&text_content);
            DirectionTypeContent::Dashes(Dashes::new(dash_type))
        }
        Some("musicxml:bracket") => {
            let (bracket_type, line_end) = parse_bracket_payload(&text_content);
            DirectionTypeContent::Bracket(Bracket::new(bracket_type, line_end))
        }
        Some("musicxml:pedal") => {
            let pedal_type = parse_pedal_type(&text_content);
            DirectionTypeContent::Pedal(Pedal::new(pedal_type))
        }
        Some("musicxml:octave-shift") => {
            let (shift_type, size) = parse_octave_shift_payload(&text_content);
            let mut shift = OctaveShift::new(shift_type);
            shift.size = Some(size);
            DirectionTypeContent::OctaveShift(shift)
        }
        Some("musicxml:harp-pedals") => serde_json::from_str::<HarpPedals>(&text_content)
            .map(DirectionTypeContent::HarpPedals)
            .unwrap_or(DirectionTypeContent::Words(vec![])),
        Some("musicxml:damp") => DirectionTypeContent::Damp(Damp::default()),
        Some("musicxml:damp-all") => DirectionTypeContent::DampAll(DampAll::default()),
        Some("musicxml:eyeglasses") => DirectionTypeContent::Eyeglasses(Eyeglasses::default()),
        Some("musicxml:string-mute") => {
            serde_json::from_str::<crate::model::direction::StringMute>(&text_content)
                .map(DirectionTypeContent::StringMute)
                .unwrap_or(DirectionTypeContent::Words(vec![]))
        }
        Some("musicxml:scordatura") => {
            serde_json::from_str::<crate::model::direction::Scordatura>(&text_content)
                .map(DirectionTypeContent::Scordatura)
                .unwrap_or(DirectionTypeContent::Words(vec![]))
        }
        Some("musicxml:image") => {
            serde_json::from_str::<crate::model::direction::DirectionImage>(&text_content)
                .map(DirectionTypeContent::Image)
                .unwrap_or(DirectionTypeContent::Words(vec![]))
        }
        Some("musicxml:principal-voice") => serde_json::from_str::<PrincipalVoice>(&text_content)
            .map(DirectionTypeContent::PrincipalVoice)
            .unwrap_or(DirectionTypeContent::Words(vec![])),
        Some("musicxml:percussion") => {
            serde_json::from_str::<Vec<crate::model::direction::Percussion>>(&text_content)
                .map(DirectionTypeContent::Percussion)
                .unwrap_or(DirectionTypeContent::Words(vec![]))
        }
        Some("musicxml:accordion-registration") => {
            serde_json::from_str::<crate::model::direction::AccordionRegistration>(&text_content)
                .map(DirectionTypeContent::AccordionRegistration)
                .unwrap_or(DirectionTypeContent::AccordionRegistration(
                    Default::default(),
                ))
        }
        Some("musicxml:staff-divide") => serde_json::from_str::<StaffDivide>(&text_content)
            .map(DirectionTypeContent::StaffDivide)
            .unwrap_or(DirectionTypeContent::Words(vec![])),
        Some("musicxml:other") => {
            serde_json::from_str::<crate::model::direction::OtherDirection>(&text_content)
                .map(DirectionTypeContent::OtherDirection)
                .unwrap_or(DirectionTypeContent::Words(vec![]))
        }
        _ => {
            // Check for stored words visual attrs — prefer ExtensionStore
            let restored_words = dir
                .common
                .xml_id
                .as_ref()
                .and_then(|id| ctx.ext_store().get(id))
                .and_then(|ext| ext.direction_visual.as_ref())
                .and_then(|dv| {
                    if dv.words.is_empty() {
                        None
                    } else {
                        // Convert WordsVisualData back to Vec<Words>
                        let val = serde_json::to_value(&dv.words).ok()?;
                        serde_json::from_value::<Vec<Words>>(val).ok()
                    }
                })
                .or_else(|| {
                    // Fallback: parse from label
                    dir.common.label.as_deref().and_then(|label| {
                        label.split('|').find_map(|seg| {
                            seg.strip_prefix("musicxml:words-vis,")
                                .and_then(|json| serde_json::from_str::<Vec<Words>>(json).ok())
                        })
                    })
                });
            if let Some(words) = restored_words {
                DirectionTypeContent::Words(words)
            } else {
                DirectionTypeContent::Words(vec![Words::new(&text_content)])
            }
        }
    };

    let direction_type = DirectionType {
        content: direction_type_content,
        id: None,
    };

    let mut direction = Direction::new(vec![direction_type]);

    if dir.dir_log.staff.as_ref().is_some_and(|s| !s.is_empty()) {
        direction.staff = Some(1);
    }

    direction.placement = convert_place_to_placement(&dir.dir_vis.place);

    if let Some(ref xml_id) = dir.common.xml_id {
        direction.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    direction.offset = convert_tstamp_to_offset(&dir.dir_log.tstamp, ctx);

    Some(direction)
}

fn parse_start_stop_continue(s: &str) -> StartStopContinue {
    match s.trim().to_lowercase().as_str() {
        "stop" => StartStopContinue::Stop,
        "continue" => StartStopContinue::Continue,
        _ => StartStopContinue::Start,
    }
}

fn parse_bracket_payload(s: &str) -> (StartStopContinue, LineEnd) {
    let mut parts = s.splitn(2, ':');
    let first = parts.next().unwrap_or("").trim().to_lowercase();
    let second = parts.next().unwrap_or("up").trim().to_lowercase();
    let bracket_type = match first.as_str() {
        "stop" => StartStopContinue::Stop,
        "continue" => StartStopContinue::Continue,
        _ => StartStopContinue::Start,
    };
    let line_end = match second.as_str() {
        "down" => LineEnd::Down,
        "both" => LineEnd::Both,
        "arrow" => LineEnd::Arrow,
        "none" => LineEnd::None,
        _ => LineEnd::Up,
    };
    (bracket_type, line_end)
}

fn parse_pedal_type(s: &str) -> PedalType {
    match s.trim().to_lowercase().as_str() {
        "stop" => PedalType::Stop,
        "sostenuto" => PedalType::Sostenuto,
        "change" => PedalType::Change,
        "continue" => PedalType::Continue,
        "discontinue" => PedalType::Discontinue,
        "resume" => PedalType::Resume,
        _ => PedalType::Start,
    }
}

fn parse_octave_shift_payload(s: &str) -> (OctaveShiftType, u8) {
    let mut parts = s.splitn(2, ':');
    let kind = parts.next().unwrap_or("").trim().to_lowercase();
    let size_str = parts.next().unwrap_or("8").trim();
    let size = size_str.parse().unwrap_or(8);
    let shift_type = match kind.as_str() {
        "down" => OctaveShiftType::Down,
        "stop" => OctaveShiftType::Stop,
        "continue" => OctaveShiftType::Continue,
        _ => OctaveShiftType::Up,
    };
    (shift_type, size)
}

/// Convert an MEI tempo element to a MusicXML direction.
///
/// Maps:
/// - MEI `<tempo>` text content -> MusicXML words element
/// - MEI `@mm`, `@mm.unit`, `@mm.dots` -> MusicXML metronome element
/// - MEI `@tstamp` -> direction position
/// - MEI `@staff` -> direction staff
///
/// # Arguments
///
/// * `tempo` - The MEI tempo element to convert
/// * `ctx` - Conversion context
///
/// # Returns
///
/// A MusicXML Direction element, or None if conversion fails.
pub fn convert_mei_tempo(
    tempo: &tusk_model::elements::Tempo,
    ctx: &mut ConversionContext,
) -> Option<crate::model::direction::Direction> {
    use crate::model::direction::{
        Direction, DirectionType, DirectionTypeContent, Metronome, MetronomeContent, Sound, Words,
    };
    use tusk_model::elements::TempoChild;

    let mut direction_types = Vec::new();

    // Extract text content
    let text_content: String = tempo
        .children
        .iter()
        .map(|child| {
            let TempoChild::Text(t) = child;
            t.as_str()
        })
        .collect::<Vec<_>>()
        .join("");

    // Check for stored metronome JSON for lossless roundtrip
    let stored_metronome: Option<Metronome> = tempo
        .common
        .xml_id
        .as_ref()
        .and_then(|id| ctx.ext_store().get(id))
        .and_then(|ext| ext.metronome_json.as_ref())
        .and_then(|json| {
            let unescaped = json.replace("\\u007c", "|");
            serde_json::from_str(&unescaped).ok()
        });

    // Metronome is present if mm_unit is set or stored JSON exists
    let has_metronome = tempo.tempo_log.mm_unit.is_some() || stored_metronome.is_some();

    // Add text content as words ONLY if no metronome is present.
    // When both exist, the metronome is sufficient — the import reconstructs
    // display text from the metronome, so emitting both would create a spurious
    // Dir element on re-import.
    if !text_content.is_empty() && !has_metronome {
        let words = Words::new(&text_content);
        direction_types.push(DirectionType {
            content: DirectionTypeContent::Words(vec![words]),
            id: None,
        });
    }

    if let Some(metronome) = stored_metronome {
        direction_types.push(DirectionType {
            content: DirectionTypeContent::Metronome(metronome),
            id: None,
        });
    } else if let Some(mm_unit) = &tempo.tempo_log.mm_unit {
        let beat_unit = convert_mei_duration_to_beat_unit(mm_unit);
        // Use numeric mm if available, otherwise extract per-minute from text
        // content (handles non-numeric values like "132-144", "c. 108")
        let per_minute = tempo
            .tempo_log
            .mm
            .as_ref()
            .map(|m| format!("{}", m.0 as u32))
            .unwrap_or_else(|| {
                text_content
                    .split(" = ")
                    .nth(1)
                    .unwrap_or(&text_content)
                    .to_string()
            });
        let beat_unit_dots = tempo
            .tempo_log
            .mm_dots
            .as_ref()
            .map(|d| vec![(); d.0 as usize])
            .unwrap_or_default();

        let metronome = Metronome {
            content: MetronomeContent::BeatUnit {
                beat_unit,
                beat_unit_dots,
                beat_unit_tied: Vec::new(),
                per_minute,
            },
            parentheses: None,
            print_object: None,
            justify: None,
            default_x: None,
            default_y: None,
            halign: None,
            valign: None,
            id: None,
        };

        direction_types.push(DirectionType {
            content: DirectionTypeContent::Metronome(metronome),
            id: None,
        });
    }

    // If direction_types is empty but mm is present, we still create a direction
    // for playback purposes (with just a Sound element)
    let has_mm = tempo.tempo_log.mm.is_some();

    if direction_types.is_empty() && !has_mm {
        ctx.add_warning("tempo", "Empty tempo element - skipping");
        return None;
    }

    let mut direction = Direction::new(direction_types);

    // Set staff: MEI @staff is global, MusicXML <staff> is within-part (always 1 for 1:1 mapping)
    if tempo
        .tempo_log
        .staff
        .as_ref()
        .is_some_and(|s| !s.is_empty())
    {
        direction.staff = Some(1);
    }

    // Set placement from MEI @place (no default — only emit if explicitly set)
    direction.placement = convert_place_to_placement(&tempo.tempo_vis.place);

    // Add sound element with tempo if mm is present
    if let Some(mm) = &tempo.tempo_log.mm {
        direction.sound = Some(Sound::with_tempo(mm.0));
    }

    // Preserve ID
    if let Some(ref xml_id) = tempo.common.xml_id {
        direction.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert tstamp to offset for proper repositioning on reimport
    direction.offset = convert_tstamp_to_offset(&tempo.tempo_log.tstamp, ctx);

    Some(direction)
}

/// Convert MEI tstamp to MusicXML offset.
///
/// MEI @tstamp is Option<DataBeat> (1-based beat position). MusicXML offset is in divisions from
/// the current position.
fn convert_tstamp_to_offset(
    tstamp: &Option<tusk_model::data::DataBeat>,
    ctx: &ConversionContext,
) -> Option<crate::model::direction::Offset> {
    tstamp.as_ref().map(|b| {
        let beat_1based = b.0;
        let beat_position = beat_1based - 1.0; // Convert 1-based to 0-based
        let offset_divisions = beat_position * ctx.divisions();
        crate::model::direction::Offset::new(offset_divisions)
    })
}

/// Convert MEI placement (@place) to MusicXML AboveBelow.
pub(crate) fn convert_place_to_placement(
    place: &Option<tusk_model::data::DataStaffrel>,
) -> Option<AboveBelow> {
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
    place.as_ref().and_then(|p| match p {
        DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above) => Some(AboveBelow::Above),
        DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below) => Some(AboveBelow::Below),
        _ => None,
    })
}

/// Convert MEI DataColor to a MusicXML color string.
pub(crate) fn convert_mei_color_to_string(color: &tusk_model::data::DataColor) -> String {
    use tusk_model::data::DataColor;
    match color {
        DataColor::MeiDataColorvalues(v) => v.0.clone(),
        DataColor::MeiDataColornames(n) => format!("{n:?}").to_lowercase(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::context::ConversionDirection;

    #[test]
    fn test_convert_mei_dynam_basic() {
        use tusk_model::elements::{Dynam, DynamChild};

        let mut dynam = Dynam::default();
        dynam.children.push(DynamChild::Text("f".to_string()));
        dynam.dynam_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        dynam.dynam_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_dynam(&dynam, &mut ctx);

        assert!(direction.is_some());
        let dir = direction.unwrap();
        assert_eq!(dir.direction_types.len(), 1);
        // Check dynamics content
        if let crate::model::direction::DirectionTypeContent::Dynamics(dyn_content) =
            &dir.direction_types[0].content
        {
            assert_eq!(dyn_content.values.len(), 1);
        } else {
            panic!("Expected dynamics direction type");
        }
    }

    #[test]
    fn test_convert_mei_dynam_with_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let mut dynam = Dynam::default();
        dynam.children.push(DynamChild::Text("mp".to_string()));
        dynam.dynam_log.tstamp = Some(tusk_model::data::DataBeat::from(2.5));
        dynam.dynam_log.staff = Some("2".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_dynam(&dynam, &mut ctx);

        assert!(direction.is_some());
        let dir = direction.unwrap();
        assert_eq!(dir.staff, Some(1)); // within-part staff is always 1 for 1:1 mapping
    }

    #[test]
    fn test_convert_mei_dynam_recognizes_standard_dynamics() {
        use crate::model::direction::{DirectionTypeContent, DynamicsValue};
        use tusk_model::elements::{Dynam, DynamChild};

        for (text, expected) in [
            ("ppp", DynamicsValue::Ppp),
            ("pp", DynamicsValue::Pp),
            ("p", DynamicsValue::P),
            ("mp", DynamicsValue::Mp),
            ("mf", DynamicsValue::Mf),
            ("f", DynamicsValue::F),
            ("ff", DynamicsValue::Ff),
            ("fff", DynamicsValue::Fff),
            ("sfz", DynamicsValue::Sfz),
            ("fp", DynamicsValue::Fp),
        ] {
            let mut dynam = Dynam::default();
            dynam.children.push(DynamChild::Text(text.to_string()));
            dynam.dynam_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
            dynam.dynam_log.staff = Some("1".to_string());

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            let direction = convert_mei_dynam(&dynam, &mut ctx);

            assert!(direction.is_some(), "Failed for dynamic: {}", text);
            let dir = direction.unwrap();
            if let DirectionTypeContent::Dynamics(dyn_content) = &dir.direction_types[0].content {
                assert_eq!(
                    dyn_content.values[0], expected,
                    "Mismatch for dynamic: {}",
                    text
                );
            } else {
                panic!("Expected dynamics for: {}", text);
            }
        }
    }

    #[test]
    fn test_convert_mei_hairpin_crescendo() {
        use crate::model::direction::{DirectionTypeContent, WedgeType};
        use tusk_model::elements::Hairpin;

        let mut hairpin = Hairpin::default();
        hairpin.hairpin_log.form = Some("cres".to_string());
        hairpin.hairpin_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        hairpin.hairpin_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let directions = convert_mei_hairpin(&hairpin, &mut ctx);

        // Crescendo should produce a single direction with wedge start
        assert_eq!(directions.len(), 1);
        if let DirectionTypeContent::Wedge(wedge) = &directions[0].direction_types[0].content {
            assert_eq!(wedge.wedge_type, WedgeType::Crescendo);
        } else {
            panic!("Expected wedge direction type");
        }
    }

    #[test]
    fn test_convert_mei_hairpin_diminuendo() {
        use crate::model::direction::{DirectionTypeContent, WedgeType};
        use tusk_model::elements::Hairpin;

        let mut hairpin = Hairpin::default();
        hairpin.hairpin_log.form = Some("dim".to_string());
        hairpin.hairpin_log.tstamp = Some(tusk_model::data::DataBeat::from(3.0));
        hairpin.hairpin_log.staff = Some("2".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let directions = convert_mei_hairpin(&hairpin, &mut ctx);

        assert_eq!(directions.len(), 1);
        if let DirectionTypeContent::Wedge(wedge) = &directions[0].direction_types[0].content {
            assert_eq!(wedge.wedge_type, WedgeType::Diminuendo);
        } else {
            panic!("Expected wedge direction type");
        }
        assert_eq!(directions[0].staff, Some(1)); // within-part staff is always 1
    }

    #[test]
    fn test_convert_mei_hairpin_with_niente() {
        use crate::model::data::YesNo;
        use crate::model::direction::DirectionTypeContent;
        use tusk_model::elements::Hairpin;

        let mut hairpin = Hairpin::default();
        hairpin.hairpin_log.form = Some("cres".to_string());
        hairpin.hairpin_log.niente = Some(tusk_model::data::DataBoolean::True);
        hairpin.hairpin_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        hairpin.hairpin_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let directions = convert_mei_hairpin(&hairpin, &mut ctx);

        assert_eq!(directions.len(), 1);
        if let DirectionTypeContent::Wedge(wedge) = &directions[0].direction_types[0].content {
            assert_eq!(wedge.niente, Some(YesNo::Yes));
        } else {
            panic!("Expected wedge direction type");
        }
    }

    #[test]
    fn test_convert_mei_dir_basic() {
        use crate::model::direction::DirectionTypeContent;
        use tusk_model::elements::{Dir, DirChild};

        let mut dir = Dir::default();
        dir.children.push(DirChild::Text("dolce".to_string()));
        dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        dir.dir_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_dir(&dir, &mut ctx);

        assert!(direction.is_some());
        let d = direction.unwrap();
        if let DirectionTypeContent::Words(words) = &d.direction_types[0].content {
            assert_eq!(words[0].value, "dolce");
        } else {
            panic!("Expected words direction type");
        }
    }

    #[test]
    fn test_convert_mei_tempo_basic() {
        use crate::model::direction::DirectionTypeContent;
        use tusk_model::elements::{Tempo, TempoChild};

        let mut tempo = Tempo::default();
        tempo.children.push(TempoChild::Text("Allegro".to_string()));
        tempo.tempo_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        tempo.tempo_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_tempo(&tempo, &mut ctx);

        assert!(direction.is_some());
        let d = direction.unwrap();
        // Tempo text should be converted to words
        if let DirectionTypeContent::Words(words) = &d.direction_types[0].content {
            assert_eq!(words[0].value, "Allegro");
        } else {
            panic!("Expected words direction type for tempo text");
        }
    }

    #[test]
    fn test_convert_mei_tempo_with_metronome() {
        use crate::model::direction::{DirectionTypeContent, MetronomeContent};
        use tusk_model::data::{DataDuration, DataDurationCmn};
        use tusk_model::elements::Tempo;
        use tusk_model::generated::data::DataTempovalue;

        let mut tempo = Tempo::default();
        tempo.tempo_log.mm = Some(DataTempovalue::from(120.0));
        tempo.tempo_log.mm_unit = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        tempo.tempo_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        tempo.tempo_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_tempo(&tempo, &mut ctx);

        assert!(direction.is_some());
        let d = direction.unwrap();
        // Find metronome in direction types
        let has_metronome = d.direction_types.iter().any(|dt| {
            if let DirectionTypeContent::Metronome(met) = &dt.content {
                if let MetronomeContent::BeatUnit {
                    beat_unit,
                    per_minute,
                    ..
                } = &met.content
                {
                    return beat_unit == "quarter" && per_minute == "120";
                }
            }
            false
        });
        assert!(has_metronome, "Expected metronome marking");
    }

    #[test]
    fn test_convert_mei_tempo_with_bpm_sound() {
        use tusk_model::elements::Tempo;
        use tusk_model::generated::data::DataTempovalue;

        let mut tempo = Tempo::default();
        tempo.tempo_log.mm = Some(DataTempovalue::from(90.0));
        tempo.tempo_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
        tempo.tempo_log.staff = Some("1".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_tempo(&tempo, &mut ctx);

        assert!(direction.is_some());
        let d = direction.unwrap();
        // Should have sound element with tempo
        assert!(d.sound.is_some());
        assert_eq!(d.sound.as_ref().unwrap().tempo, Some(90.0));
    }
}

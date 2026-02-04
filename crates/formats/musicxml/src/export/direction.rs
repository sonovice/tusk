//! MEI control events to MusicXML direction conversion.
//!
//! This module handles conversion of MEI control events (dynam, hairpin, dir, tempo)
//! to MusicXML direction elements.

use super::utils::convert_mei_duration_to_beat_unit;
use crate::context::ConversionContext;

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
    use crate::model::data::AboveBelow;
    use crate::model::direction::{Direction, DirectionType, DirectionTypeContent, Dynamics};
    use tusk_model::elements::DynamChild;

    // Extract text content from dynam element
    let text_content: String = dynam
        .children
        .iter()
        .filter_map(|child| {
            if let DynamChild::Text(t) = child {
                Some(t.as_str())
            } else {
                None
            }
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
    };

    let direction_type = DirectionType {
        content: DirectionTypeContent::Dynamics(dynamics),
        id: None,
    };

    let mut direction = Direction::new(vec![direction_type]);

    // Set staff from MEI @staff attribute
    if let Some(&staff) = dynam.dynam_log.staff.first() {
        direction.staff = Some(staff as u32);
    }

    // Set placement (default below for dynamics)
    direction.placement = Some(AboveBelow::Below);

    // Preserve ID if present
    if let Some(ref xml_id) = dynam.common.xml_id {
        direction.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Note: MEI @tstamp is not directly convertible to MusicXML offset without knowing
    // divisions and beat position. The caller should handle positioning.

    Some(direction)
}

/// Parse dynamics text to a MusicXML DynamicsValue.
fn parse_dynamics_text(text: &str) -> crate::model::direction::DynamicsValue {
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
    use crate::model::data::{AboveBelow, YesNo};
    use crate::model::direction::{
        Direction, DirectionType, DirectionTypeContent, Wedge, WedgeType,
    };
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::DataBoolean;

    let mut directions = Vec::new();

    // Determine wedge type from form
    let wedge_type = match hairpin.hairpin_log.form {
        Some(AttHairpinLogForm::Cres) => WedgeType::Crescendo,
        Some(AttHairpinLogForm::Dim) => WedgeType::Diminuendo,
        None => {
            ctx.add_warning(
                "hairpin",
                "Hairpin without form attribute - defaulting to crescendo",
            );
            WedgeType::Crescendo
        }
    };

    let mut wedge = Wedge::new(wedge_type);

    // Convert niente attribute
    if let Some(DataBoolean::True) = hairpin.hairpin_log.niente {
        wedge.niente = Some(YesNo::Yes);
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

    // Set staff
    if let Some(&staff) = hairpin.hairpin_log.staff.first() {
        direction.staff = Some(staff as u32);
    }

    // Hairpins are typically below the staff
    direction.placement = Some(AboveBelow::Below);

    directions.push(direction);

    // If hairpin has tstamp2 (ending timestamp), we need to create a stop wedge
    // This is handled separately by the caller who has measure context
    // Note: MEI hairpins are span elements with start/end, MusicXML uses separate start/stop

    directions
}

/// Convert an MEI dir (directive) element to a MusicXML direction.
///
/// Maps:
/// - MEI `<dir>` text content -> MusicXML words element
/// - MEI `@tstamp` -> direction position
/// - MEI `@staff` -> direction staff
///
/// # Arguments
///
/// * `dir` - The MEI dir element to convert
/// * `ctx` - Conversion context
///
/// # Returns
///
/// A MusicXML Direction element, or None if conversion fails.
pub fn convert_mei_dir(
    dir: &tusk_model::elements::Dir,
    ctx: &mut ConversionContext,
) -> Option<crate::model::direction::Direction> {
    use crate::model::data::AboveBelow;
    use crate::model::direction::{Direction, DirectionType, DirectionTypeContent, Words};
    use tusk_model::elements::DirChild;

    // Extract text content
    let text_content: String = dir
        .children
        .iter()
        .filter_map(|child| {
            if let DirChild::Text(t) = child {
                Some(t.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("");

    if text_content.is_empty() {
        ctx.add_warning("dir", "Empty dir element - skipping");
        return None;
    }

    let words = Words::new(&text_content);

    let direction_type = DirectionType {
        content: DirectionTypeContent::Words(vec![words]),
        id: None,
    };

    let mut direction = Direction::new(vec![direction_type]);

    // Set staff
    if let Some(&staff) = dir.dir_log.staff.first() {
        direction.staff = Some(staff as u32);
    }

    // Directives are typically above the staff
    direction.placement = Some(AboveBelow::Above);

    // Preserve ID
    if let Some(ref xml_id) = dir.common.xml_id {
        direction.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    Some(direction)
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
    use crate::model::data::AboveBelow;
    use crate::model::direction::{
        Direction, DirectionType, DirectionTypeContent, Metronome, MetronomeContent, Sound, Words,
    };
    use tusk_model::elements::TempoChild;

    let mut direction_types = Vec::new();

    // Extract text content
    let text_content: String = tempo
        .children
        .iter()
        .filter_map(|child| {
            if let TempoChild::Text(t) = child {
                Some(t.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("");

    // Add text content as words if present
    if !text_content.is_empty() {
        let words = Words::new(&text_content);
        direction_types.push(DirectionType {
            content: DirectionTypeContent::Words(vec![words]),
            id: None,
        });
    }

    // Add metronome marking if mm and mm.unit are present
    if let (Some(mm), Some(mm_unit)) = (&tempo.tempo_log.mm, &tempo.tempo_log.mm_unit) {
        let beat_unit = convert_mei_duration_to_beat_unit(mm_unit);
        let per_minute = format!("{}", mm.0 as u32);
        let beat_unit_dots = if let Some(ref dots) = tempo.tempo_log.mm_dots {
            vec![(); dots.0 as usize]
        } else {
            Vec::new()
        };

        let metronome = Metronome {
            content: MetronomeContent::BeatUnit {
                beat_unit,
                beat_unit_dots,
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

    // Set staff
    if let Some(&staff) = tempo.tempo_log.staff.first() {
        direction.staff = Some(staff as u32);
    }

    // Tempo is typically above the staff
    direction.placement = Some(AboveBelow::Above);

    // Add sound element with tempo if mm is present
    if let Some(mm) = &tempo.tempo_log.mm {
        direction.sound = Some(Sound::with_tempo(mm.0));
    }

    // Preserve ID
    if let Some(ref xml_id) = tempo.common.xml_id {
        direction.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    Some(direction)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;

    #[test]
    fn test_convert_mei_dynam_basic() {
        use tusk_model::elements::{Dynam, DynamChild};
        use tusk_model::generated::data::DataBeat;

        let mut dynam = Dynam::default();
        dynam.children.push(DynamChild::Text("f".to_string()));
        dynam.dynam_log.tstamp = Some(DataBeat::from(1.0));
        dynam.dynam_log.staff = vec![1];

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
        use tusk_model::generated::data::DataBeat;

        let mut dynam = Dynam::default();
        dynam.children.push(DynamChild::Text("mp".to_string()));
        dynam.dynam_log.tstamp = Some(DataBeat::from(2.5));
        dynam.dynam_log.staff = vec![2];

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_dynam(&dynam, &mut ctx);

        assert!(direction.is_some());
        let dir = direction.unwrap();
        assert_eq!(dir.staff, Some(2));
    }

    #[test]
    fn test_convert_mei_dynam_recognizes_standard_dynamics() {
        use crate::model::direction::{DirectionTypeContent, DynamicsValue};
        use tusk_model::elements::{Dynam, DynamChild};
        use tusk_model::generated::data::DataBeat;

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
            dynam.dynam_log.tstamp = Some(DataBeat::from(1.0));
            dynam.dynam_log.staff = vec![1];

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
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;
        use tusk_model::generated::data::DataBeat;

        let mut hairpin = Hairpin::default();
        hairpin.hairpin_log.form = Some(AttHairpinLogForm::Cres);
        hairpin.hairpin_log.tstamp = Some(DataBeat::from(1.0));
        hairpin.hairpin_log.staff = vec![1];

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
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;
        use tusk_model::generated::data::DataBeat;

        let mut hairpin = Hairpin::default();
        hairpin.hairpin_log.form = Some(AttHairpinLogForm::Dim);
        hairpin.hairpin_log.tstamp = Some(DataBeat::from(3.0));
        hairpin.hairpin_log.staff = vec![2];

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let directions = convert_mei_hairpin(&hairpin, &mut ctx);

        assert_eq!(directions.len(), 1);
        if let DirectionTypeContent::Wedge(wedge) = &directions[0].direction_types[0].content {
            assert_eq!(wedge.wedge_type, WedgeType::Diminuendo);
        } else {
            panic!("Expected wedge direction type");
        }
        assert_eq!(directions[0].staff, Some(2));
    }

    #[test]
    fn test_convert_mei_hairpin_with_niente() {
        use crate::model::data::YesNo;
        use crate::model::direction::DirectionTypeContent;
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;
        use tusk_model::generated::data::DataBeat;

        let mut hairpin = Hairpin::default();
        hairpin.hairpin_log.form = Some(AttHairpinLogForm::Cres);
        hairpin.hairpin_log.niente = Some(DataBoolean::True);
        hairpin.hairpin_log.tstamp = Some(DataBeat::from(1.0));
        hairpin.hairpin_log.staff = vec![1];

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
        use tusk_model::generated::data::DataBeat;

        let mut dir = Dir::default();
        dir.children.push(DirChild::Text("dolce".to_string()));
        dir.dir_log.tstamp = Some(DataBeat::from(1.0));
        dir.dir_log.staff = vec![1];

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
        use tusk_model::generated::data::DataBeat;

        let mut tempo = Tempo::default();
        tempo.children.push(TempoChild::Text("Allegro".to_string()));
        tempo.tempo_log.tstamp = Some(DataBeat::from(1.0));
        tempo.tempo_log.staff = vec![1];

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
        use tusk_model::generated::data::{DataBeat, DataTempovalue};

        let mut tempo = Tempo::default();
        tempo.tempo_log.mm = Some(DataTempovalue::from(120.0));
        tempo.tempo_log.mm_unit = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        tempo.tempo_log.tstamp = Some(DataBeat::from(1.0));
        tempo.tempo_log.staff = vec![1];

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
        use tusk_model::generated::data::{DataBeat, DataTempovalue};

        let mut tempo = Tempo::default();
        tempo.tempo_log.mm = Some(DataTempovalue::from(90.0));
        tempo.tempo_log.tstamp = Some(DataBeat::from(1.0));
        tempo.tempo_log.staff = vec![1];

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let direction = convert_mei_tempo(&tempo, &mut ctx);

        assert!(direction.is_some());
        let d = direction.unwrap();
        // Should have sound element with tempo
        assert!(d.sound.is_some());
        assert_eq!(d.sound.as_ref().unwrap().tempo, Some(90.0));
    }
}

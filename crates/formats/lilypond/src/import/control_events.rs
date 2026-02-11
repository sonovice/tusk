//! MEI control event builders for LilyPond import.
//!
//! Creates MEI elements (Slur, Dynam, Hairpin, TupletSpan, Dir, Trill, Mordent,
//! Turn, Fermata, Ornam, BTrem, Harm) from LilyPond AST data.

use tusk_model::elements::{
    BTrem, BTremChild, Dir, DirChild, Dynam, DynamChild, F, FChild, Fb, FbChild, Fermata, Hairpin,
    Harm, HarmChild, Layer, LayerChild, MeasureChild, Mordent, Ornam, OrnamChild, Slur, Trill,
    TupletSpan, Turn,
};
use tusk_model::generated::data::DataUri;

use crate::model::note::{
    BassFigure, ChordModeEvent, Direction, FigureAlteration, FigureEvent, FiguredBassModification,
};

/// Convert a LilyPond Direction to an extension DirectionExt.
fn direction_to_ext(dir: Direction) -> Option<tusk_model::DirectionExt> {
    match dir {
        Direction::Up => Some(tusk_model::DirectionExt::Up),
        Direction::Down => Some(tusk_model::DirectionExt::Down),
        Direction::Neutral => None,
    }
}

/// Build a typed JSON ornament label.
fn ornament_label(name: &str, direction: Direction) -> String {
    let info = tusk_model::OrnamentInfo {
        name: name.to_string(),
        direction: direction_to_ext(direction),
    };
    let json = serde_json::to_string(&info).unwrap_or_default();
    format!("tusk:ornament,{json}")
}

/// Convert a LilyPond RepeatType to an extension RepeatTypeExt.
fn repeat_type_to_ext(rt: crate::model::RepeatType) -> tusk_model::RepeatTypeExt {
    match rt {
        crate::model::RepeatType::Volta => tusk_model::RepeatTypeExt::Volta,
        crate::model::RepeatType::Unfold => tusk_model::RepeatTypeExt::Unfold,
        crate::model::RepeatType::Percent => tusk_model::RepeatTypeExt::Percent,
        crate::model::RepeatType::Tremolo => tusk_model::RepeatTypeExt::Tremolo,
        crate::model::RepeatType::Segno => tusk_model::RepeatTypeExt::Segno,
    }
}

/// Build a typed JSON articulation/fingering/string label.
fn artic_label(kind: tusk_model::ArticulationKind, value: &str, direction: Direction) -> String {
    let info = tusk_model::ArticulationInfo {
        kind,
        value: value.to_string(),
        direction: direction_to_ext(direction),
    };
    let json = serde_json::to_string(&info).unwrap_or_default();
    format!("tusk:artic,{json}")
}

/// Compute the number of tremolo slashes from the subdivision value.
///
/// E.g., 32 → 3 slashes (32nd notes = 3 beams), 16 → 2, 8 → 1.
/// Value 0 (bare `:`) → 0 (unmeasured).
fn tremolo_slash_count(value: u32) -> u32 {
    if value == 0 {
        return 0;
    }
    value.trailing_zeros().saturating_sub(2)
}

/// Create an MEI Slur control event.
pub(super) fn make_slur(
    start_id: &str,
    end_id: &str,
    staff_n: u32,
    slur_id: u32,
    is_phrase: bool,
) -> Slur {
    let mut slur = Slur::default();
    slur.common.xml_id = Some(format!("ly-slur-{slur_id}"));
    slur.slur_log.startid = Some(DataUri(format!("#{start_id}")));
    slur.slur_log.endid = Some(DataUri(format!("#{end_id}")));
    slur.slur_log.staff = Some(staff_n.to_string());
    if is_phrase {
        let json = serde_json::to_string(&tusk_model::PhrasingSlur).unwrap_or_default();
        slur.common.label = Some(format!("tusk:phrase,{json}"));
    }
    slur
}

/// Create an MEI Dynam control event.
pub(super) fn make_dynam(name: &str, startid: &str, staff_n: u32, dynam_id: u32) -> Dynam {
    let mut dynam = Dynam::default();
    dynam.common.xml_id = Some(format!("ly-dynam-{dynam_id}"));
    dynam.dynam_log.startid = Some(DataUri(format!("#{startid}")));
    dynam.dynam_log.staff = Some(staff_n.to_string());
    dynam.children.push(DynamChild::Text(name.to_string()));
    dynam
}

/// Create an MEI Hairpin control event.
pub(super) fn make_hairpin(
    start_id: &str,
    end_id: &str,
    staff_n: u32,
    form: &str,
    hairpin_id: u32,
) -> Hairpin {
    let mut hairpin = Hairpin::default();
    hairpin.common.xml_id = Some(format!("ly-hairpin-{hairpin_id}"));
    hairpin.hairpin_log.startid = Some(DataUri(format!("#{start_id}")));
    hairpin.hairpin_log.endid = Some(DataUri(format!("#{end_id}")));
    hairpin.hairpin_log.staff = Some(staff_n.to_string());
    hairpin.hairpin_log.form = Some(form.to_string());
    hairpin
}

/// Create an MEI TupletSpan control event.
///
/// Label stores the LilyPond-specific data for lossless roundtrip:
/// `tusk:tuplet,{json}` (typed `TupletInfo` JSON)
pub(super) fn make_tuplet_span(
    start_id: &str,
    end_id: &str,
    staff_n: u32,
    num: u32,
    numbase: u32,
    span_duration: Option<&crate::model::Duration>,
    tuplet_id: u32,
) -> TupletSpan {
    let mut ts = TupletSpan::default();
    ts.common.xml_id = Some(format!("ly-tuplet-{tuplet_id}"));
    ts.tuplet_span_log.startid = Some(DataUri(format!("#{start_id}")));
    ts.tuplet_span_log.endid = Some(DataUri(format!("#{end_id}")));
    ts.tuplet_span_log.staff = Some(staff_n.to_string());
    ts.tuplet_span_log.num = Some(num.to_string());
    ts.tuplet_span_log.numbase = Some(numbase.to_string());

    // Build typed JSON label for roundtrip
    let info = tusk_model::TupletInfo {
        num,
        denom: numbase,
        span_duration: span_duration.map(|dur| tusk_model::DurationInfo {
            base: dur.base,
            dots: dur.dots,
            multipliers: dur.multipliers.clone(),
        }),
    };
    let json = serde_json::to_string(&info).unwrap_or_default();
    ts.common.label = Some(format!("tusk:tuplet,{json}"));

    ts
}

/// Classify an ornament name and create the appropriate MEI control event.
///
/// Returns `Some(MeasureChild)` for ornaments with native MEI elements (trill, mordent,
/// turn, fermata, generic ornam). Returns `None` for names that should use `<dir>` instead.
pub(super) fn make_ornament_control_event(
    name: &str,
    direction: Direction,
    startid: &str,
    staff_n: u32,
    counter: &mut u32,
) -> Option<MeasureChild> {
    match name {
        "trill" => {
            *counter += 1;
            Some(MeasureChild::Trill(Box::new(make_trill(
                startid, staff_n, direction, *counter,
            ))))
        }
        "mordent" => {
            *counter += 1;
            Some(MeasureChild::Mordent(Box::new(make_mordent(
                startid, staff_n, direction, "lower", false, *counter, None,
            ))))
        }
        "prall" => {
            *counter += 1;
            Some(MeasureChild::Mordent(Box::new(make_mordent(
                startid, staff_n, direction, "upper", false, *counter, None,
            ))))
        }
        "prallprall" | "prallmordent" | "upprall" | "downprall" | "upmordent" | "downmordent"
        | "pralldown" | "prallup" | "lineprall" => {
            *counter += 1;
            Some(MeasureChild::Ornam(Box::new(make_ornam(
                name, startid, staff_n, direction, *counter,
            ))))
        }
        "turn" => {
            *counter += 1;
            Some(MeasureChild::Turn(Box::new(make_turn(
                startid, staff_n, direction, "upper", *counter,
            ))))
        }
        "reverseturn" => {
            *counter += 1;
            Some(MeasureChild::Turn(Box::new(make_turn(
                startid, staff_n, direction, "lower", *counter,
            ))))
        }
        "fermata" | "shortfermata" | "longfermata" | "verylongfermata" => {
            *counter += 1;
            Some(MeasureChild::Fermata(Box::new(make_fermata(
                name, startid, staff_n, direction, *counter,
            ))))
        }
        _ => None,
    }
}

/// Create an MEI Trill control event.
fn make_trill(startid: &str, staff_n: u32, direction: Direction, id: u32) -> Trill {
    let mut trill = Trill::default();
    trill.common.xml_id = Some(format!("ly-ornam-{id}"));
    trill.trill_log.startid = Some(DataUri(format!("#{startid}")));
    trill.trill_log.staff = Some(staff_n.to_string());
    trill.common.label = Some(ornament_label("trill", direction));
    trill
}

/// Create an MEI Mordent control event.
fn make_mordent(
    startid: &str,
    staff_n: u32,
    direction: Direction,
    form: &str,
    long: bool,
    id: u32,
    ornament_name: Option<&str>,
) -> Mordent {
    let mut mordent = Mordent::default();
    mordent.common.xml_id = Some(format!("ly-ornam-{id}"));
    mordent.mordent_log.startid = Some(DataUri(format!("#{startid}")));
    mordent.mordent_log.staff = Some(staff_n.to_string());
    mordent.mordent_log.form = Some(form.to_string());
    if long {
        mordent.mordent_log.long = Some(tusk_model::generated::data::DataBoolean::True);
    }
    let name = ornament_name.unwrap_or(if form == "upper" { "prall" } else { "mordent" });
    mordent.common.label = Some(ornament_label(name, direction));
    mordent
}

/// Create an MEI Turn control event.
fn make_turn(startid: &str, staff_n: u32, direction: Direction, form: &str, id: u32) -> Turn {
    let mut turn = Turn::default();
    turn.common.xml_id = Some(format!("ly-ornam-{id}"));
    turn.turn_log.startid = Some(DataUri(format!("#{startid}")));
    turn.turn_log.staff = Some(staff_n.to_string());
    turn.turn_log.form = Some(form.to_string());
    let name = if form == "lower" {
        "reverseturn"
    } else {
        "turn"
    };
    turn.common.label = Some(ornament_label(name, direction));
    turn
}

/// Create an MEI Fermata control event.
fn make_fermata(name: &str, startid: &str, staff_n: u32, direction: Direction, id: u32) -> Fermata {
    let mut fermata = Fermata::default();
    fermata.common.xml_id = Some(format!("ly-ornam-{id}"));
    fermata.fermata_log.startid = Some(DataUri(format!("#{startid}")));
    fermata.fermata_log.staff = Some(staff_n.to_string());
    let shape = match name {
        "shortfermata" => Some("angular"),
        "longfermata" => Some("square"),
        "verylongfermata" => Some("square"),
        _ => None,
    };
    if let Some(s) = shape {
        fermata.fermata_vis.shape = Some(s.to_string());
    }
    fermata.common.label = Some(ornament_label(name, direction));
    fermata
}

/// Create an MEI Ornam (generic ornament) control event.
fn make_ornam(name: &str, startid: &str, staff_n: u32, direction: Direction, id: u32) -> Ornam {
    let mut ornam = Ornam::default();
    ornam.common.xml_id = Some(format!("ly-ornam-{id}"));
    ornam.ornam_log.startid = Some(DataUri(format!("#{startid}")));
    ornam.ornam_log.staff = Some(staff_n.to_string());
    ornam.common.label = Some(ornament_label(name, direction));
    ornam.children.push(OrnamChild::Text(name.to_string()));
    ornam
}

/// Wrap the last-added LayerChild in a `<bTrem>` element for single-note tremolo.
pub(super) fn wrap_last_in_btrem(layer: &mut Layer, value: u32, counter: &mut u32) {
    if let Some(last) = layer.children.pop() {
        *counter += 1;
        let mut btrem = BTrem::default();
        btrem.common.xml_id = Some(format!("ly-btrem-{}", *counter));
        let trem_info = tusk_model::TremoloInfo { value };
        let json = serde_json::to_string(&trem_info).unwrap_or_default();
        btrem.common.label = Some(format!("tusk:tremolo,{json}"));
        let num = tremolo_slash_count(value);
        if num > 0 {
            btrem.b_trem_log.num = Some(num.to_string());
        }
        match last {
            LayerChild::Note(n) => btrem.children.push(BTremChild::Note(n)),
            LayerChild::Chord(c) => btrem.children.push(BTremChild::Chord(c)),
            other => {
                layer.children.push(other);
                return;
            }
        }
        layer.children.push(LayerChild::BTrem(Box::new(btrem)));
    }
}

/// Create an MEI Dir for a LilyPond articulation.
pub(super) fn make_artic_dir(
    name: &str,
    direction: Direction,
    startid: &str,
    staff_n: u32,
    id: u32,
) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-artic-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    dir.common.label = Some(artic_label(
        tusk_model::ArticulationKind::Articulation,
        name,
        direction,
    ));
    dir.children.push(DirChild::Text(name.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond fingering.
pub(super) fn make_fing_dir(
    digit: u8,
    direction: Direction,
    startid: &str,
    staff_n: u32,
    id: u32,
) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-artic-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    dir.common.label = Some(artic_label(
        tusk_model::ArticulationKind::Fingering,
        &digit.to_string(),
        direction,
    ));
    dir.children.push(DirChild::Text(digit.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond string number.
pub(super) fn make_string_dir(
    number: u8,
    direction: Direction,
    startid: &str,
    staff_n: u32,
    id: u32,
) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-artic-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    dir.common.label = Some(artic_label(
        tusk_model::ArticulationKind::StringNumber,
        &number.to_string(),
        direction,
    ));
    dir.children.push(DirChild::Text(number.to_string()));
    dir
}

/// Create an MEI Tempo control event from a LilyPond `\tempo`.
///
/// Maps metronome data to `@mm`, `@mm.unit`, `@mm.dots`.
/// Display text goes to children. The full serialized form is stored in
/// `@label` for lossless roundtrip.
pub(super) fn make_tempo(
    tempo: &crate::model::signature::Tempo,
    startid: &str,
    staff_n: u32,
    id: u32,
) -> tusk_model::elements::Tempo {
    use tusk_model::elements::{Tempo, TempoChild};
    use tusk_model::generated::data::{
        DataAugmentdot, DataDuration, DataDurationCmn, DataTempovalue,
    };

    let mut mei_tempo = Tempo::default();
    mei_tempo.common.xml_id = Some(format!("ly-tempo-{id}"));
    mei_tempo.tempo_log.startid = Some(DataUri(format!("#{startid}")));
    mei_tempo.tempo_log.staff = Some(staff_n.to_string());

    // Map metronome mark to @mm, @mm.unit, @mm.dots
    if let Some(dur) = &tempo.duration {
        let mm_unit = match dur.base {
            1 => Some(DataDurationCmn::N1),
            2 => Some(DataDurationCmn::N2),
            4 => Some(DataDurationCmn::N4),
            8 => Some(DataDurationCmn::N8),
            16 => Some(DataDurationCmn::N16),
            32 => Some(DataDurationCmn::N32),
            64 => Some(DataDurationCmn::N64),
            128 => Some(DataDurationCmn::N128),
            _ => None,
        };
        if let Some(unit) = mm_unit {
            mei_tempo.tempo_log.mm_unit = Some(DataDuration::MeiDataDurationCmn(unit));
        }
        if dur.dots > 0 {
            mei_tempo.tempo_log.mm_dots = Some(DataAugmentdot(dur.dots as u64));
        }
    }
    if let Some(bpm) = &tempo.bpm {
        let bpm_val = match bpm {
            crate::model::signature::TempoRange::Single(v) => *v as f64,
            crate::model::signature::TempoRange::Range(lo, _) => *lo as f64,
        };
        mei_tempo.tempo_log.mm = Some(DataTempovalue(bpm_val));
    }

    // Text content
    if let Some(text) = &tempo.text {
        let text_str = crate::serializer::serialize_markup(text);
        mei_tempo
            .children
            .push(TempoChild::Text(text_str.trim().to_string()));
    }

    // Store full serialized form as typed JSON label for lossless roundtrip
    let serialized = crate::serializer::serialize_tempo(tempo);
    let info = tusk_model::TempoInfo { serialized };
    let json = serde_json::to_string(&info).unwrap_or_default();
    mei_tempo.common.label = Some(format!("tusk:tempo,{json}"));

    mei_tempo
}

/// Create an MEI Dir for a LilyPond `\mark`.
pub(super) fn make_mark_dir(serialized: &str, startid: &str, staff_n: u32, id: u32) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-mark-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    let info = tusk_model::MarkInfo {
        serialized: serialized.to_string(),
    };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&info).unwrap_or_default());
    dir.common.label = Some(format!("tusk:mark,{json}"));
    dir.children.push(DirChild::Text(serialized.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond `\textMark`.
pub(super) fn make_textmark_dir(serialized: &str, startid: &str, staff_n: u32, id: u32) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-mark-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    let info = tusk_model::TextMarkInfo {
        serialized: serialized.to_string(),
    };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&info).unwrap_or_default());
    dir.common.label = Some(format!("tusk:textmark,{json}"));
    dir.children.push(DirChild::Text(serialized.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond repeat structure.
///
/// Uses startid/endid to delimit the repeat body range.
pub(super) fn make_repeat_dir(
    start_id: &str,
    end_id: &str,
    staff_n: u32,
    repeat_type: crate::model::RepeatType,
    count: u32,
    num_alternatives: u32,
    id: u32,
) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-repeat-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{start_id}")));
    dir.dir_log.endid = Some(DataUri(format!("#{end_id}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    let info = tusk_model::RepeatInfo {
        repeat_type: repeat_type_to_ext(repeat_type),
        count,
        alternative_count: if num_alternatives > 0 {
            Some(num_alternatives as usize)
        } else {
            None
        },
        ending_index: None,
    };
    let json = serde_json::to_string(&info).unwrap_or_default();
    dir.common.label = Some(format!("tusk:repeat,{json}"));
    dir.children.push(DirChild::Text(format!(
        "repeat {} {count}",
        repeat_type.as_str()
    )));
    dir
}

/// Create an MEI Dir for a LilyPond alternative ending.
///
/// Uses startid/endid to delimit the alternative range.
pub(super) fn make_ending_dir(
    start_id: &str,
    end_id: &str,
    staff_n: u32,
    index: u32,
    id: u32,
) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-repeat-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{start_id}")));
    dir.dir_log.endid = Some(DataUri(format!("#{end_id}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    let info = tusk_model::EndingInfo { index };
    let json = serde_json::to_string(&info).unwrap_or_default();
    dir.common.label = Some(format!("tusk:ending,{json}"));
    dir.children
        .push(DirChild::Text(format!("ending {}", index + 1)));
    dir
}

/// Create an MEI Harm control event from a LilyPond chord-mode event.
///
/// Text child: human-readable chord symbol (e.g. "c:m7/e").
pub(super) fn make_harm(ce: &ChordModeEvent, startid: &str, staff_n: u32, id: u32) -> Harm {
    let mut harm = Harm::default();
    harm.common.xml_id = Some(format!("ly-harm-{id}"));
    harm.harm_log.startid = Some(DataUri(format!("#{startid}")));
    harm.harm_log.staff = Some(staff_n.to_string());

    // Serialize the chord mode event as typed JSON label for lossless roundtrip
    let serialized = crate::serializer::serialize_chord_mode_event(ce);
    let info = tusk_model::ChordModeInfo {
        serialized: serialized.clone(),
    };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&info).unwrap_or_default());
    harm.common.label = Some(format!("tusk:chord-mode,{json}"));

    // Human-readable text child
    harm.children.push(HarmChild::Text(serialized));

    harm
}

/// Create an MEI `<fb>` control event from a LilyPond figure event.
///
/// `<f>` children carry human-readable text (e.g. "6+", "4", "_").
pub(super) fn make_fb(fe: &FigureEvent, _staff_n: u32, id: u32) -> Fb {
    let mut fb = Fb::default();
    fb.common.xml_id = Some(format!("ly-fb-{id}"));

    // Serialize the figure event as typed JSON label for lossless roundtrip
    let serialized = crate::serializer::serialize_figure_event(fe);
    let info = tusk_model::FiguredBassInfo { serialized };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&info).unwrap_or_default());
    fb.common.label = Some(format!("tusk:figure,{json}"));

    // Create <f> children with human-readable text
    for fig in &fe.figures {
        let mut mei_f = F::default();
        let text = bass_figure_to_text(fig);
        if !text.is_empty() {
            mei_f.children.push(FChild::Text(text));
        }
        fb.children.push(FbChild::F(Box::new(mei_f)));
    }

    fb
}

/// Generate human-readable text for a single bass figure.
fn bass_figure_to_text(fig: &BassFigure) -> String {
    let mut text = String::new();
    match fig.number {
        Some(n) => text.push_str(&n.to_string()),
        None => text.push('_'),
    }
    match fig.alteration {
        FigureAlteration::Natural => {}
        FigureAlteration::Sharp => text.push('#'),
        FigureAlteration::Flat => text.push('b'),
        FigureAlteration::ForcedNatural => text.push('n'),
        FigureAlteration::DoubleSharp => text.push_str("##"),
        FigureAlteration::DoubleFlat => text.push_str("bb"),
    }
    for m in &fig.modifications {
        match m {
            FiguredBassModification::Augmented => text.push('+'),
            FiguredBassModification::NoContinuation => text.push('!'),
            FiguredBassModification::Diminished => text.push('/'),
            FiguredBassModification::AugmentedSlash => text.push('\\'),
        }
    }
    text
}

/// Create an MEI Dir for a generic LilyPond music function call.
pub(super) fn make_function_dir(serialized: &str, startid: &str, staff_n: u32, id: u32) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-func-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    let info = tusk_model::FunctionCallInfo {
        serialized: serialized.to_string(),
    };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&info).unwrap_or_default());
    dir.common.label = Some(format!("tusk:func,{json}"));
    dir
}

/// Create an MEI Dir for a LilyPond property operation (`\override`, `\set`, etc.).
pub(super) fn make_property_dir(serialized: &str, startid: &str, staff_n: u32, id: u32) -> Dir {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(format!("ly-prop-{id}"));
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    let info = tusk_model::PropertyOpInfo {
        serialized: serialized.to_string(),
    };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&info).unwrap_or_default());
    dir.common.label = Some(format!("tusk:prop,{json}"));
    dir
}

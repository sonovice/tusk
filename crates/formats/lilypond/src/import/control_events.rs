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
use tusk_model::ExtensionStore;

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

/// Build an OrnamentInfo struct.
fn ornament_info(name: &str, direction: Direction) -> tusk_model::OrnamentInfo {
    tusk_model::OrnamentInfo {
        name: name.to_string(),
        direction: direction_to_ext(direction),
    }
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

/// Build an ArticulationInfo struct.
fn artic_info(kind: tusk_model::ArticulationKind, value: &str, direction: Direction) -> tusk_model::ArticulationInfo {
    tusk_model::ArticulationInfo {
        kind,
        value: value.to_string(),
        direction: direction_to_ext(direction),
    }
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
    ext_store: &mut ExtensionStore,
) -> Slur {
    let mut slur = Slur::default();
    let id = format!("ly-slur-{slur_id}");
    slur.common.xml_id = Some(id.clone());
    slur.slur_log.startid = Some(DataUri(format!("#{start_id}")));
    slur.slur_log.endid = Some(DataUri(format!("#{end_id}")));
    slur.slur_log.staff = Some(staff_n.to_string());
    if is_phrase {
        ext_store.insert_phrasing_slur(id, tusk_model::PhrasingSlur);
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
pub(super) fn make_tuplet_span(
    start_id: &str,
    end_id: &str,
    staff_n: u32,
    num: u32,
    numbase: u32,
    span_duration: Option<&crate::model::Duration>,
    tuplet_id: u32,
    ext_store: &mut ExtensionStore,
) -> TupletSpan {
    let mut ts = TupletSpan::default();
    let id = format!("ly-tuplet-{tuplet_id}");
    ts.common.xml_id = Some(id.clone());
    ts.tuplet_span_log.startid = Some(DataUri(format!("#{start_id}")));
    ts.tuplet_span_log.endid = Some(DataUri(format!("#{end_id}")));
    ts.tuplet_span_log.staff = Some(staff_n.to_string());
    ts.tuplet_span_log.num = Some(num.to_string());
    ts.tuplet_span_log.numbase = Some(numbase.to_string());

    let info = tusk_model::TupletInfo {
        num,
        denom: numbase,
        span_duration: span_duration.map(|dur| tusk_model::DurationInfo {
            base: dur.base,
            dots: dur.dots,
            multipliers: dur.multipliers.clone(),
        }),
    };
    ext_store.insert_tuplet_info(id, info);

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
    ext_store: &mut ExtensionStore,
) -> Option<MeasureChild> {
    match name {
        "trill" => {
            *counter += 1;
            Some(MeasureChild::Trill(Box::new(make_trill(
                startid, staff_n, direction, *counter, ext_store,
            ))))
        }
        "mordent" => {
            *counter += 1;
            Some(MeasureChild::Mordent(Box::new(make_mordent(
                startid, staff_n, direction, "lower", false, *counter, None, ext_store,
            ))))
        }
        "prall" => {
            *counter += 1;
            Some(MeasureChild::Mordent(Box::new(make_mordent(
                startid, staff_n, direction, "upper", false, *counter, None, ext_store,
            ))))
        }
        "prallprall" | "prallmordent" | "upprall" | "downprall" | "upmordent" | "downmordent"
        | "pralldown" | "prallup" | "lineprall" => {
            *counter += 1;
            Some(MeasureChild::Ornam(Box::new(make_ornam(
                name, startid, staff_n, direction, *counter, ext_store,
            ))))
        }
        "turn" => {
            *counter += 1;
            Some(MeasureChild::Turn(Box::new(make_turn(
                startid, staff_n, direction, "upper", *counter, ext_store,
            ))))
        }
        "reverseturn" => {
            *counter += 1;
            Some(MeasureChild::Turn(Box::new(make_turn(
                startid, staff_n, direction, "lower", *counter, ext_store,
            ))))
        }
        "fermata" | "shortfermata" | "longfermata" | "verylongfermata" => {
            *counter += 1;
            Some(MeasureChild::Fermata(Box::new(make_fermata(
                name, startid, staff_n, direction, *counter, ext_store,
            ))))
        }
        _ => None,
    }
}

/// Create an MEI Trill control event.
fn make_trill(startid: &str, staff_n: u32, direction: Direction, id: u32, ext_store: &mut ExtensionStore) -> Trill {
    let mut trill = Trill::default();
    let eid = format!("ly-ornam-{id}");
    trill.common.xml_id = Some(eid.clone());
    trill.trill_log.startid = Some(DataUri(format!("#{startid}")));
    trill.trill_log.staff = Some(staff_n.to_string());
    ext_store.insert_ornament_info(eid, ornament_info("trill", direction));
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
    ext_store: &mut ExtensionStore,
) -> Mordent {
    let mut mordent = Mordent::default();
    let eid = format!("ly-ornam-{id}");
    mordent.common.xml_id = Some(eid.clone());
    mordent.mordent_log.startid = Some(DataUri(format!("#{startid}")));
    mordent.mordent_log.staff = Some(staff_n.to_string());
    mordent.mordent_log.form = Some(form.to_string());
    if long {
        mordent.mordent_log.long = Some(tusk_model::generated::data::DataBoolean::True);
    }
    let name = ornament_name.unwrap_or(if form == "upper" { "prall" } else { "mordent" });
    ext_store.insert_ornament_info(eid, ornament_info(name, direction));
    mordent
}

/// Create an MEI Turn control event.
fn make_turn(startid: &str, staff_n: u32, direction: Direction, form: &str, id: u32, ext_store: &mut ExtensionStore) -> Turn {
    let mut turn = Turn::default();
    let eid = format!("ly-ornam-{id}");
    turn.common.xml_id = Some(eid.clone());
    turn.turn_log.startid = Some(DataUri(format!("#{startid}")));
    turn.turn_log.staff = Some(staff_n.to_string());
    turn.turn_log.form = Some(form.to_string());
    let name = if form == "lower" {
        "reverseturn"
    } else {
        "turn"
    };
    ext_store.insert_ornament_info(eid, ornament_info(name, direction));
    turn
}

/// Create an MEI Fermata control event.
fn make_fermata(name: &str, startid: &str, staff_n: u32, direction: Direction, id: u32, ext_store: &mut ExtensionStore) -> Fermata {
    let mut fermata = Fermata::default();
    let eid = format!("ly-ornam-{id}");
    fermata.common.xml_id = Some(eid.clone());
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
    ext_store.insert_ornament_info(eid, ornament_info(name, direction));
    fermata
}

/// Create an MEI Ornam (generic ornament) control event.
fn make_ornam(name: &str, startid: &str, staff_n: u32, direction: Direction, id: u32, ext_store: &mut ExtensionStore) -> Ornam {
    let mut ornam = Ornam::default();
    let eid = format!("ly-ornam-{id}");
    ornam.common.xml_id = Some(eid.clone());
    ornam.ornam_log.startid = Some(DataUri(format!("#{startid}")));
    ornam.ornam_log.staff = Some(staff_n.to_string());
    ext_store.insert_ornament_info(eid, ornament_info(name, direction));
    ornam.children.push(OrnamChild::Text(name.to_string()));
    ornam
}

/// Wrap the last-added LayerChild in a `<bTrem>` element for single-note tremolo.
pub(super) fn wrap_last_in_btrem(layer: &mut Layer, value: u32, counter: &mut u32, ext_store: &mut ExtensionStore) {
    if let Some(last) = layer.children.pop() {
        *counter += 1;
        let mut btrem = BTrem::default();
        let eid = format!("ly-btrem-{}", *counter);
        btrem.common.xml_id = Some(eid.clone());
        ext_store.insert_tremolo_info(eid, tusk_model::TremoloInfo { value });
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
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-artic-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_articulation_info(eid, artic_info(
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
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-artic-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_articulation_info(eid, artic_info(
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
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-artic-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_articulation_info(eid, artic_info(
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
    ext_store: &mut ExtensionStore,
) -> tusk_model::elements::Tempo {
    use tusk_model::elements::{Tempo, TempoChild};
    use tusk_model::generated::data::{
        DataAugmentdot, DataDuration, DataDurationCmn, DataTempovalue,
    };

    let mut mei_tempo = Tempo::default();
    let eid = format!("ly-tempo-{id}");
    mei_tempo.common.xml_id = Some(eid.clone());
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

    // Store full serialized form in ext_store for lossless roundtrip
    let serialized = crate::serializer::serialize_tempo(tempo);
    ext_store.insert_tempo_info(eid, tusk_model::TempoInfo { serialized });

    mei_tempo
}

/// Create an MEI Dir for a LilyPond `\mark`.
pub(super) fn make_mark_dir(serialized: &str, startid: &str, staff_n: u32, id: u32, ext_store: &mut ExtensionStore) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-mark-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_mark_info(eid, tusk_model::MarkInfo {
        serialized: serialized.to_string(),
    });
    dir.children.push(DirChild::Text(serialized.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond `\textMark`.
pub(super) fn make_textmark_dir(serialized: &str, startid: &str, staff_n: u32, id: u32, ext_store: &mut ExtensionStore) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-mark-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_textmark_info(eid, tusk_model::TextMarkInfo {
        serialized: serialized.to_string(),
    });
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
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-repeat-{id}");
    dir.common.xml_id = Some(eid.clone());
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
    ext_store.insert_repeat_info(eid, info);
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
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-repeat-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{start_id}")));
    dir.dir_log.endid = Some(DataUri(format!("#{end_id}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_ending_info(eid, tusk_model::EndingInfo { index });
    dir.children
        .push(DirChild::Text(format!("ending {}", index + 1)));
    dir
}

/// Create an MEI Harm control event from a LilyPond chord-mode event.
///
/// Text child: human-readable chord symbol (e.g. "c:m7/e").
pub(super) fn make_harm(ce: &ChordModeEvent, startid: &str, staff_n: u32, id: u32, ext_store: &mut ExtensionStore) -> Harm {
    let mut harm = Harm::default();
    let eid = format!("ly-harm-{id}");
    harm.common.xml_id = Some(eid.clone());
    harm.harm_log.startid = Some(DataUri(format!("#{startid}")));
    harm.harm_log.staff = Some(staff_n.to_string());

    let serialized = crate::serializer::serialize_chord_mode_event(ce);
    ext_store.insert_chord_mode_info(eid, tusk_model::ChordModeInfo {
        serialized: serialized.clone(),
    });

    // Human-readable text child
    harm.children.push(HarmChild::Text(serialized));

    harm
}

/// Create an MEI `<fb>` control event from a LilyPond figure event.
///
/// `<f>` children carry human-readable text (e.g. "6+", "4", "_").
pub(super) fn make_fb(fe: &FigureEvent, _staff_n: u32, id: u32, ext_store: &mut ExtensionStore) -> Fb {
    let mut fb = Fb::default();
    let eid = format!("ly-fb-{id}");
    fb.common.xml_id = Some(eid.clone());

    let serialized = crate::serializer::serialize_figure_event(fe);
    ext_store.insert_figured_bass_info(eid, tusk_model::FiguredBassInfo { serialized });

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
pub(super) fn make_function_dir(
    fc: &tusk_model::FunctionCall,
    startid: &str,
    staff_n: u32,
    id: u32,
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-func-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_function_call(eid, fc.clone());
    dir
}

/// Create an MEI Dir for a LilyPond property operation (`\override`, `\set`, etc.).
pub(super) fn make_property_dir(serialized: &str, startid: &str, staff_n: u32, id: u32, ext_store: &mut ExtensionStore) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-prop-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_property_op_info(eid, tusk_model::PropertyOpInfo {
        serialized: serialized.to_string(),
    });
    dir
}

/// Create an MEI Dir for a LilyPond text script post-event.
///
/// The text is stored both as human-readable `<dir>` text content and as a typed
/// ExtensionStore entry for lossless markup roundtrip.
/// Direction maps to `@place` (above/below) on the native MEI element.
pub(super) fn make_text_script_dir(
    text: &crate::model::markup::Markup,
    direction: Direction,
    startid: &str,
    staff_n: u32,
    id: u32,
    ext_store: &mut ExtensionStore,
) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-textscript-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());

    // Map direction to @place
    use tusk_model::generated::data::{DataStaffrel, DataStaffrelBasic};
    match direction {
        Direction::Up => {
            dir.dir_vis.place = Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above));
        }
        Direction::Down => {
            dir.dir_vis.place = Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below));
        }
        Direction::Neutral => {}
    }

    // Serialize text for display
    let display_text = match text {
        crate::model::markup::Markup::String(s) => s.clone(),
        other => crate::serializer::serialize_markup(other),
    };
    if !display_text.is_empty() {
        dir.children.push(DirChild::Text(display_text));
    }

    // Store in ext_store for lossless roundtrip
    let serialized = crate::serializer::serialize_text_script_text(text);
    ext_store.insert_text_script_info(eid, tusk_model::TextScriptInfo {
        serialized,
        direction: direction_to_ext(direction),
    });

    dir
}

/// Create an MEI Dir for a Scheme expression in music position (`#expr`).
pub(super) fn make_scheme_music_dir(serialized: &str, startid: &str, staff_n: u32, id: u32, ext_store: &mut ExtensionStore) -> Dir {
    let mut dir = Dir::default();
    let eid = format!("ly-scm-{id}");
    dir.common.xml_id = Some(eid.clone());
    dir.dir_log.startid = Some(DataUri(format!("#{startid}")));
    dir.dir_log.staff = Some(staff_n.to_string());
    ext_store.insert_scheme_music_info(eid, tusk_model::SchemeMusicInfo {
        serialized: serialized.to_string(),
    });
    dir
}

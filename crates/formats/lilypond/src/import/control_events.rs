//! MEI control event builders for LilyPond import.
//!
//! Creates MEI elements (Slur, Dynam, Hairpin, TupletSpan, Dir, Trill, Mordent,
//! Turn, Fermata, Ornam, BTrem) from LilyPond AST data.

use tusk_model::elements::{
    BTrem, BTremChild, Dir, DirChild, Dynam, DynamChild, Fermata, Hairpin, Layer, LayerChild,
    MeasureChild, Mordent, Ornam, OrnamChild, Slur, Trill, TupletSpan, Turn,
};
use tusk_model::generated::data::DataUri;

use crate::model::note::Direction;

/// Encode a Direction into a label suffix.
pub(super) fn direction_label_suffix(dir: Direction) -> &'static str {
    match dir {
        Direction::Up => ",dir=up",
        Direction::Down => ",dir=down",
        Direction::Neutral => "",
    }
}

/// Format a LilyPond Duration as a compact string for label storage.
///
/// Format: `BASE[.DOTS][*N][*N/M]` e.g. `4`, `8.`, `4*3`, `8*2/3`
pub(super) fn format_duration_for_label(dur: &crate::model::Duration) -> String {
    let mut s = dur.base.to_string();
    for _ in 0..dur.dots {
        s.push('.');
    }
    for &(n, d) in &dur.multipliers {
        if d == 1 {
            s.push_str(&format!("*{n}"));
        } else {
            s.push_str(&format!("*{n}/{d}"));
        }
    }
    s
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
        slur.common.label = Some("lilypond:phrase".to_string());
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
/// `lilypond:tuplet,NUM/DENOM[,span=DUR]`
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

    // Build roundtrip label
    let mut label = format!("lilypond:tuplet,{num}/{numbase}");
    if let Some(dur) = span_duration {
        let dur_str = format_duration_for_label(dur);
        label.push_str(&format!(",span={dur_str}"));
    }
    ts.common.label = Some(label);

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
    if direction != Direction::Neutral {
        trill.common.label = Some(format!(
            "lilypond:trill{}",
            direction_label_suffix(direction)
        ));
    }
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
    label: Option<&str>,
) -> Mordent {
    let mut mordent = Mordent::default();
    mordent.common.xml_id = Some(format!("ly-ornam-{id}"));
    mordent.mordent_log.startid = Some(DataUri(format!("#{startid}")));
    mordent.mordent_log.staff = Some(staff_n.to_string());
    mordent.mordent_log.form = Some(form.to_string());
    if long {
        mordent.mordent_log.long = Some(tusk_model::generated::data::DataBoolean::True);
    }
    if direction != Direction::Neutral || label.is_some() {
        let mut lbl = label.unwrap_or("").to_string();
        if direction != Direction::Neutral {
            if !lbl.is_empty() {
                lbl.push_str(direction_label_suffix(direction));
            } else {
                lbl = format!("lilypond:mordent{}", direction_label_suffix(direction));
            }
        }
        if !lbl.is_empty() {
            mordent.common.label = Some(lbl);
        }
    }
    mordent
}

/// Create an MEI Turn control event.
fn make_turn(startid: &str, staff_n: u32, direction: Direction, form: &str, id: u32) -> Turn {
    let mut turn = Turn::default();
    turn.common.xml_id = Some(format!("ly-ornam-{id}"));
    turn.turn_log.startid = Some(DataUri(format!("#{startid}")));
    turn.turn_log.staff = Some(staff_n.to_string());
    turn.turn_log.form = Some(form.to_string());
    if direction != Direction::Neutral {
        turn.common.label = Some(format!(
            "lilypond:turn{}",
            direction_label_suffix(direction)
        ));
    }
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
    let variant_str = if name != "fermata" {
        Some(format!("lilypond:fermata,{name}"))
    } else {
        None
    };
    if direction != Direction::Neutral || variant_str.is_some() {
        let base = variant_str.unwrap_or_default();
        let dir_suffix = direction_label_suffix(direction);
        if !base.is_empty() || !dir_suffix.is_empty() {
            let label = if base.is_empty() {
                format!("lilypond:fermata{dir_suffix}")
            } else {
                format!("{base}{dir_suffix}")
            };
            fermata.common.label = Some(label);
        }
    }
    fermata
}

/// Create an MEI Ornam (generic ornament) control event.
fn make_ornam(name: &str, startid: &str, staff_n: u32, direction: Direction, id: u32) -> Ornam {
    let mut ornam = Ornam::default();
    ornam.common.xml_id = Some(format!("ly-ornam-{id}"));
    ornam.ornam_log.startid = Some(DataUri(format!("#{startid}")));
    ornam.ornam_log.staff = Some(staff_n.to_string());
    ornam.common.label = Some(format!(
        "lilypond:ornam,{name}{}",
        direction_label_suffix(direction)
    ));
    ornam.children.push(OrnamChild::Text(name.to_string()));
    ornam
}

/// Wrap the last-added LayerChild in a `<bTrem>` element for single-note tremolo.
pub(super) fn wrap_last_in_btrem(layer: &mut Layer, value: u32, counter: &mut u32) {
    if let Some(last) = layer.children.pop() {
        *counter += 1;
        let mut btrem = BTrem::default();
        btrem.common.xml_id = Some(format!("ly-btrem-{}", *counter));
        btrem.common.label = Some(format!("lilypond:tremolo,{value}"));
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
///
/// Label format: `lilypond:artic,NAME[,dir=up|down]`
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
    dir.common.label = Some(format!(
        "lilypond:artic,{name}{}",
        direction_label_suffix(direction)
    ));
    dir.children.push(DirChild::Text(name.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond fingering.
///
/// Label format: `lilypond:fing,DIGIT[,dir=up|down]`
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
    dir.common.label = Some(format!(
        "lilypond:fing,{digit}{}",
        direction_label_suffix(direction)
    ));
    dir.children.push(DirChild::Text(digit.to_string()));
    dir
}

/// Create an MEI Dir for a LilyPond string number.
///
/// Label format: `lilypond:string,NUMBER[,dir=up|down]`
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
    dir.common.label = Some(format!(
        "lilypond:string,{number}{}",
        direction_label_suffix(direction)
    ));
    dir.children.push(DirChild::Text(number.to_string()));
    dir
}

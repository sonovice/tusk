//! Serializer implementations for MusicXML technical notation types.

use std::io::Write;

use crate::model::technical;
use crate::serializer::{MusicXmlWriter, SerializeResult};

use super::score::{
    above_below_str, push_opt_attr_start, push_opt_str_attr_start, start_stop_str, yes_no_str,
};

/// Serialize a `<technical>` container element with all its children.
pub fn serialize_technical<W: Write>(
    w: &mut MusicXmlWriter<W>,
    tech: &technical::Technical,
) -> SerializeResult<()> {
    let start = w.start_element("technical");
    w.write_start(start)?;

    for v in &tech.up_bow {
        serialize_empty_placement(w, "up-bow", v)?;
    }
    for v in &tech.down_bow {
        serialize_empty_placement(w, "down-bow", v)?;
    }
    for v in &tech.harmonic {
        serialize_harmonic(w, v)?;
    }
    for v in &tech.open_string {
        serialize_empty_placement(w, "open-string", v)?;
    }
    for v in &tech.thumb_position {
        serialize_empty_placement(w, "thumb-position", v)?;
    }
    for v in &tech.fingering {
        serialize_fingering(w, v)?;
    }
    for v in &tech.pluck {
        serialize_placement_text(w, "pluck", v)?;
    }
    for v in &tech.double_tongue {
        serialize_empty_placement(w, "double-tongue", v)?;
    }
    for v in &tech.triple_tongue {
        serialize_empty_placement(w, "triple-tongue", v)?;
    }
    for v in &tech.stopped {
        serialize_empty_placement_smufl(w, "stopped", v)?;
    }
    for v in &tech.snap_pizzicato {
        serialize_empty_placement(w, "snap-pizzicato", v)?;
    }
    for v in &tech.fret {
        serialize_fret(w, v)?;
    }
    for v in &tech.string {
        serialize_tech_string(w, v)?;
    }
    for v in &tech.hammer_on {
        serialize_hammer_on_pull_off(w, "hammer-on", v)?;
    }
    for v in &tech.pull_off {
        serialize_hammer_on_pull_off(w, "pull-off", v)?;
    }
    for v in &tech.bend {
        serialize_bend(w, v)?;
    }
    for v in &tech.tap {
        serialize_tap(w, v)?;
    }
    for v in &tech.heel {
        serialize_heel_toe(w, "heel", v)?;
    }
    for v in &tech.toe {
        serialize_heel_toe(w, "toe", v)?;
    }
    for v in &tech.fingernails {
        serialize_empty_placement(w, "fingernails", v)?;
    }
    for v in &tech.hole {
        serialize_hole(w, v)?;
    }
    for v in &tech.arrow {
        serialize_arrow(w, v)?;
    }
    for v in &tech.handbell {
        serialize_handbell(w, v)?;
    }
    for v in &tech.brass_bend {
        serialize_empty_placement(w, "brass-bend", v)?;
    }
    for v in &tech.flip {
        serialize_empty_placement(w, "flip", v)?;
    }
    for v in &tech.smear {
        serialize_empty_placement(w, "smear", v)?;
    }
    for v in &tech.open {
        serialize_empty_placement_smufl(w, "open", v)?;
    }
    for v in &tech.half_muted {
        serialize_empty_placement_smufl(w, "half-muted", v)?;
    }
    for v in &tech.harmon_mute {
        serialize_harmon_mute(w, v)?;
    }
    for v in &tech.golpe {
        serialize_empty_placement(w, "golpe", v)?;
    }
    for v in &tech.other_technical {
        serialize_other_technical(w, v)?;
    }

    w.write_end("technical")?;
    Ok(())
}

// ============================================================================
// Simple element serializers
// ============================================================================

use crate::model::notations::EmptyPlacement;

fn serialize_empty_placement<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    ep: &EmptyPlacement,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = ep.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &ep.default_x);
    push_opt_attr_start(&mut elem, "default-y", &ep.default_y);
    push_opt_str_attr_start(&mut elem, "color", &ep.color);
    w.write_empty(elem)?;
    Ok(())
}

fn serialize_empty_placement_smufl<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    ep: &technical::EmptyPlacementSmufl,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = ep.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &ep.default_x);
    push_opt_attr_start(&mut elem, "default-y", &ep.default_y);
    push_opt_str_attr_start(&mut elem, "color", &ep.color);
    push_opt_str_attr_start(&mut elem, "smufl", &ep.smufl);
    w.write_empty(elem)?;
    Ok(())
}

fn serialize_placement_text<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    pt: &technical::PlacementText,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = pt.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &pt.default_x);
    push_opt_attr_start(&mut elem, "default-y", &pt.default_y);
    push_opt_str_attr_start(&mut elem, "font-style", &pt.font_style);
    push_opt_str_attr_start(&mut elem, "color", &pt.color);
    if pt.value.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&pt.value)?;
        w.write_end(name)?;
    }
    Ok(())
}

// ============================================================================
// Fingering, Fret, String
// ============================================================================

fn serialize_fingering<W: Write>(
    w: &mut MusicXmlWriter<W>,
    f: &technical::Fingering,
) -> SerializeResult<()> {
    let mut elem = w.start_element("fingering");
    if let Some(ref s) = f.substitution {
        elem.push_attribute(("substitution", yes_no_str(s)));
    }
    if let Some(ref a) = f.alternate {
        elem.push_attribute(("alternate", yes_no_str(a)));
    }
    if let Some(ref p) = f.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &f.default_x);
    push_opt_attr_start(&mut elem, "default-y", &f.default_y);
    push_opt_str_attr_start(&mut elem, "color", &f.color);
    if f.value.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&f.value)?;
        w.write_end("fingering")?;
    }
    Ok(())
}

fn serialize_fret<W: Write>(w: &mut MusicXmlWriter<W>, f: &technical::Fret) -> SerializeResult<()> {
    let mut elem = w.start_element("fret");
    push_opt_str_attr_start(&mut elem, "color", &f.color);
    w.write_start(elem)?;
    w.write_text(&f.value.to_string())?;
    w.write_end("fret")?;
    Ok(())
}

fn serialize_tech_string<W: Write>(
    w: &mut MusicXmlWriter<W>,
    s: &technical::TechString,
) -> SerializeResult<()> {
    let mut elem = w.start_element("string");
    if let Some(ref p) = s.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &s.default_x);
    push_opt_attr_start(&mut elem, "default-y", &s.default_y);
    push_opt_str_attr_start(&mut elem, "color", &s.color);
    w.write_start(elem)?;
    w.write_text(&s.value.to_string())?;
    w.write_end("string")?;
    Ok(())
}

// ============================================================================
// Hammer-on/Pull-off, Tap, Heel/Toe
// ============================================================================

fn serialize_hammer_on_pull_off<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    h: &technical::HammerOnPullOff,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    elem.push_attribute(("type", start_stop_str(&h.ho_type)));
    if let Some(n) = h.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    if let Some(ref p) = h.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &h.default_x);
    push_opt_attr_start(&mut elem, "default-y", &h.default_y);
    push_opt_str_attr_start(&mut elem, "color", &h.color);
    if h.text.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&h.text)?;
        w.write_end(name)?;
    }
    Ok(())
}

fn serialize_tap<W: Write>(w: &mut MusicXmlWriter<W>, t: &technical::Tap) -> SerializeResult<()> {
    let mut elem = w.start_element("tap");
    if let Some(ref hand) = t.hand {
        elem.push_attribute(("hand", tap_hand_str(hand)));
    }
    if let Some(ref p) = t.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &t.default_x);
    push_opt_attr_start(&mut elem, "default-y", &t.default_y);
    push_opt_str_attr_start(&mut elem, "color", &t.color);
    if t.value.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&t.value)?;
        w.write_end("tap")?;
    }
    Ok(())
}

fn tap_hand_str(hand: &technical::TapHand) -> &'static str {
    match hand {
        technical::TapHand::Left => "left",
        technical::TapHand::Right => "right",
    }
}

fn serialize_heel_toe<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    ht: &technical::HeelToe,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref s) = ht.substitution {
        elem.push_attribute(("substitution", yes_no_str(s)));
    }
    if let Some(ref p) = ht.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &ht.default_x);
    push_opt_attr_start(&mut elem, "default-y", &ht.default_y);
    push_opt_str_attr_start(&mut elem, "color", &ht.color);
    w.write_empty(elem)?;
    Ok(())
}

// ============================================================================
// Bend
// ============================================================================

fn serialize_bend<W: Write>(w: &mut MusicXmlWriter<W>, b: &technical::Bend) -> SerializeResult<()> {
    let mut elem = w.start_element("bend");
    if let Some(ref shape) = b.shape {
        elem.push_attribute(("shape", bend_shape_str(shape)));
    }
    push_opt_attr_start(&mut elem, "default-x", &b.default_x);
    push_opt_attr_start(&mut elem, "default-y", &b.default_y);
    push_opt_str_attr_start(&mut elem, "color", &b.color);
    w.write_start(elem)?;

    w.write_text_element("bend-alter", &b.bend_alter.to_string())?;

    if b.pre_bend.is_some() {
        w.write_empty(w.start_element("pre-bend"))?;
    }
    if let Some(ref release) = b.release {
        let mut rel_elem = w.start_element("release");
        push_opt_attr_start(&mut rel_elem, "offset", &release.offset);
        w.write_empty(rel_elem)?;
    }
    if let Some(ref wb) = b.with_bar {
        serialize_placement_text(w, "with-bar", wb)?;
    }

    w.write_end("bend")?;
    Ok(())
}

fn bend_shape_str(shape: &technical::BendShape) -> &'static str {
    match shape {
        technical::BendShape::Straight => "straight",
        technical::BendShape::Curved => "curved",
    }
}

// ============================================================================
// Hole
// ============================================================================

fn serialize_hole<W: Write>(w: &mut MusicXmlWriter<W>, h: &technical::Hole) -> SerializeResult<()> {
    let mut elem = w.start_element("hole");
    if let Some(ref p) = h.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &h.default_x);
    push_opt_attr_start(&mut elem, "default-y", &h.default_y);
    push_opt_str_attr_start(&mut elem, "color", &h.color);
    w.write_start(elem)?;

    if let Some(ref ht) = h.hole_type {
        w.write_text_element("hole-type", ht)?;
    }
    serialize_hole_closed(w, &h.hole_closed)?;
    if let Some(ref hs) = h.hole_shape {
        w.write_text_element("hole-shape", hs)?;
    }

    w.write_end("hole")?;
    Ok(())
}

fn serialize_hole_closed<W: Write>(
    w: &mut MusicXmlWriter<W>,
    hc: &technical::HoleClosed,
) -> SerializeResult<()> {
    let mut elem = w.start_element("hole-closed");
    if let Some(ref loc) = hc.location {
        elem.push_attribute(("location", hole_closed_location_str(loc)));
    }
    w.write_start(elem)?;
    w.write_text(hole_closed_value_str(&hc.value))?;
    w.write_end("hole-closed")?;
    Ok(())
}

fn hole_closed_value_str(v: &technical::HoleClosedValue) -> &'static str {
    match v {
        technical::HoleClosedValue::Yes => "yes",
        technical::HoleClosedValue::No => "no",
        technical::HoleClosedValue::Half => "half",
    }
}

fn hole_closed_location_str(loc: &technical::HoleClosedLocation) -> &'static str {
    match loc {
        technical::HoleClosedLocation::Right => "right",
        technical::HoleClosedLocation::Bottom => "bottom",
        technical::HoleClosedLocation::Left => "left",
        technical::HoleClosedLocation::Top => "top",
    }
}

// ============================================================================
// Arrow
// ============================================================================

fn serialize_arrow<W: Write>(
    w: &mut MusicXmlWriter<W>,
    a: &technical::Arrow,
) -> SerializeResult<()> {
    let mut elem = w.start_element("arrow");
    if let Some(ref p) = a.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &a.default_x);
    push_opt_attr_start(&mut elem, "default-y", &a.default_y);
    push_opt_str_attr_start(&mut elem, "color", &a.color);
    push_opt_str_attr_start(&mut elem, "smufl", &a.smufl);
    w.write_start(elem)?;

    match &a.content {
        technical::ArrowContent::Directional {
            direction,
            style,
            arrowhead,
        } => {
            w.write_text_element("arrow-direction", direction)?;
            if let Some(s) = style {
                w.write_text_element("arrow-style", s)?;
            }
            if *arrowhead {
                w.write_empty(w.start_element("arrowhead"))?;
            }
        }
        technical::ArrowContent::Circular(value) => {
            w.write_text_element("circular-arrow", value)?;
        }
    }

    w.write_end("arrow")?;
    Ok(())
}

// ============================================================================
// Handbell, Harmon Mute, Harmonic
// ============================================================================

fn serialize_handbell<W: Write>(
    w: &mut MusicXmlWriter<W>,
    h: &technical::Handbell,
) -> SerializeResult<()> {
    let mut elem = w.start_element("handbell");
    if let Some(ref p) = h.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &h.default_x);
    push_opt_attr_start(&mut elem, "default-y", &h.default_y);
    push_opt_str_attr_start(&mut elem, "color", &h.color);
    w.write_start(elem)?;
    w.write_text(&h.value)?;
    w.write_end("handbell")?;
    Ok(())
}

fn serialize_harmon_mute<W: Write>(
    w: &mut MusicXmlWriter<W>,
    hm: &technical::HarmonMute,
) -> SerializeResult<()> {
    let mut elem = w.start_element("harmon-mute");
    if let Some(ref p) = hm.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &hm.default_x);
    push_opt_attr_start(&mut elem, "default-y", &hm.default_y);
    push_opt_str_attr_start(&mut elem, "color", &hm.color);
    w.write_start(elem)?;

    serialize_harmon_closed(w, &hm.harmon_closed)?;

    w.write_end("harmon-mute")?;
    Ok(())
}

fn serialize_harmon_closed<W: Write>(
    w: &mut MusicXmlWriter<W>,
    hc: &technical::HarmonClosed,
) -> SerializeResult<()> {
    let mut elem = w.start_element("harmon-closed");
    if let Some(ref loc) = hc.location {
        elem.push_attribute(("location", harmon_closed_location_str(loc)));
    }
    w.write_start(elem)?;
    w.write_text(harmon_closed_value_str(&hc.value))?;
    w.write_end("harmon-closed")?;
    Ok(())
}

fn harmon_closed_value_str(v: &technical::HarmonClosedValue) -> &'static str {
    match v {
        technical::HarmonClosedValue::Yes => "yes",
        technical::HarmonClosedValue::No => "no",
        technical::HarmonClosedValue::Half => "half",
    }
}

fn harmon_closed_location_str(loc: &technical::HarmonClosedLocation) -> &'static str {
    match loc {
        technical::HarmonClosedLocation::Right => "right",
        technical::HarmonClosedLocation::Bottom => "bottom",
        technical::HarmonClosedLocation::Left => "left",
        technical::HarmonClosedLocation::Top => "top",
    }
}

fn serialize_harmonic<W: Write>(
    w: &mut MusicXmlWriter<W>,
    h: &technical::Harmonic,
) -> SerializeResult<()> {
    let mut elem = w.start_element("harmonic");
    if let Some(ref p) = h.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    if let Some(ref po) = h.print_object {
        elem.push_attribute(("print-object", yes_no_str(po)));
    }
    push_opt_attr_start(&mut elem, "default-x", &h.default_x);
    push_opt_attr_start(&mut elem, "default-y", &h.default_y);
    push_opt_str_attr_start(&mut elem, "color", &h.color);

    let has_children = h.natural.is_some()
        || h.artificial.is_some()
        || h.base_pitch.is_some()
        || h.touching_pitch.is_some()
        || h.sounding_pitch.is_some();

    if has_children {
        w.write_start(elem)?;
        if h.natural.is_some() {
            w.write_empty(w.start_element("natural"))?;
        }
        if h.artificial.is_some() {
            w.write_empty(w.start_element("artificial"))?;
        }
        if h.base_pitch.is_some() {
            w.write_empty(w.start_element("base-pitch"))?;
        }
        if h.touching_pitch.is_some() {
            w.write_empty(w.start_element("touching-pitch"))?;
        }
        if h.sounding_pitch.is_some() {
            w.write_empty(w.start_element("sounding-pitch"))?;
        }
        w.write_end("harmonic")?;
    } else {
        w.write_empty(elem)?;
    }
    Ok(())
}

// ============================================================================
// Other Technical
// ============================================================================

fn serialize_other_technical<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ot: &technical::OtherTechnical,
) -> SerializeResult<()> {
    let mut elem = w.start_element("other-technical");
    if let Some(ref p) = ot.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_str_attr_start(&mut elem, "smufl", &ot.smufl);
    push_opt_attr_start(&mut elem, "default-x", &ot.default_x);
    push_opt_attr_start(&mut elem, "default-y", &ot.default_y);
    push_opt_str_attr_start(&mut elem, "color", &ot.color);
    if ot.value.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&ot.value)?;
        w.write_end("other-technical")?;
    }
    Ok(())
}

//! Serializer implementations for MusicXML notation types.
//!
//! Contains `MusicXmlSerialize` implementations for:
//! - Notations container
//! - Slur, Tied, Tuplet
//! - Articulations
//! - Ornaments (trills, mordents, turns, tremolos, etc.)
//! - Fermata, Arpeggiate, Glissando, Slide, AccidentalMark, OtherNotation

use std::io::Write;

use crate::model::data::*;
use crate::model::notations;
use crate::serializer::{
    MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr,
};

use super::score::{
    above_below_str, fermata_shape_str, line_shape_str, line_type_str, over_under_str,
    push_opt_attr_start, push_opt_str_attr_start, show_tuplet_str, start_stop_continue_str,
    start_stop_single_str, start_stop_str, tied_type_str, top_bottom_str, up_down_str,
    upright_inverted_str, yes_no_str,
};

// ============================================================================
// Notations
// ============================================================================

impl MusicXmlSerialize for notations::Notations {
    fn element_name(&self) -> &'static str {
        "notations"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Editorial: footnote and level (before other notations per XSD)
        if let Some(ref ft) = self.footnote {
            super::elements::serialize_formatted_text(w, "footnote", ft)?;
        }
        if let Some(ref lv) = self.level {
            super::elements::serialize_level(w, lv)?;
        }

        for tied in &self.tied {
            tied.serialize(w)?;
        }
        for slur in &self.slurs {
            slur.serialize(w)?;
        }
        for tuplet in &self.tuplets {
            tuplet.serialize(w)?;
        }
        if let Some(ref artics) = self.articulations {
            artics.serialize(w)?;
        }
        if let Some(ref ornaments) = self.ornaments {
            ornaments.serialize(w)?;
        }
        for dyn_elem in &self.dynamics {
            serialize_dynamics_notation(w, dyn_elem)?;
        }
        for fermata in &self.fermatas {
            serialize_fermata(w, fermata)?;
        }
        if let Some(ref arp) = self.arpeggiate {
            serialize_arpeggiate(w, arp)?;
        }
        if let Some(ref narp) = self.non_arpeggiate {
            serialize_non_arpeggiate(w, narp)?;
        }
        for gliss in &self.glissandos {
            serialize_glissando(w, gliss)?;
        }
        for slide in &self.slides {
            serialize_slide(w, slide)?;
        }
        if let Some(ref tech) = self.technical {
            super::technical::serialize_technical(w, tech)?;
        }
        for am in &self.accidental_marks {
            serialize_accidental_mark(w, am)?;
        }
        for on in &self.other_notations {
            serialize_other_notation(w, on)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for notations::Tuplet {
    fn element_name(&self) -> &'static str {
        "tuplet"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.push(("type", start_stop_str(&self.tuplet_type).to_string()));
        if let Some(n) = self.number {
            attrs.push(("number", n.to_string()));
        }
        if let Some(ref b) = self.bracket {
            attrs.push(("bracket", yes_no_str(b).to_string()));
        }
        if let Some(ref sn) = self.show_number {
            attrs.push(("show-number", show_tuplet_str(sn).to_string()));
        }
        if let Some(ref st) = self.show_type {
            attrs.push(("show-type", show_tuplet_str(st).to_string()));
        }
        if let Some(ref ls) = self.line_shape {
            attrs.push(("line-shape", line_shape_str(ls).to_string()));
        }
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        self.tuplet_actual.is_some() || self.tuplet_normal.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref actual) = self.tuplet_actual {
            serialize_tuplet_portion(w, "tuplet-actual", actual)?;
        }
        if let Some(ref normal) = self.tuplet_normal {
            serialize_tuplet_portion(w, "tuplet-normal", normal)?;
        }
        Ok(())
    }
}

fn serialize_tuplet_portion<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    portion: &notations::TupletPortion,
) -> SerializeResult<()> {
    let start = w.start_element(name);
    w.write_start(start)?;
    if let Some(ref tn) = portion.tuplet_number {
        w.write_text_element("tuplet-number", &tn.value.to_string())?;
    }
    if let Some(ref tt) = portion.tuplet_type {
        w.write_text_element("tuplet-type", &tt.value.to_string())?;
    }
    for _ in &portion.tuplet_dots {
        w.write_empty(w.start_element("tuplet-dot"))?;
    }
    w.write_end(name)?;
    Ok(())
}

impl MusicXmlSerialize for notations::Slur {
    fn element_name(&self) -> &'static str {
        "slur"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.push(("type", start_stop_continue_str(&self.slur_type).to_string()));
        if let Some(n) = self.number {
            attrs.push(("number", n.to_string()));
        }
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        if let Some(ref o) = self.orientation {
            attrs.push(("orientation", over_under_str(o).to_string()));
        }
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "bezier-x", self.bezier_x);
        push_opt_attr!(attrs, "bezier-y", self.bezier_y);
        push_opt_attr!(attrs, "bezier-x2", self.bezier_x2);
        push_opt_attr!(attrs, "bezier-y2", self.bezier_y2);
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for notations::Tied {
    fn element_name(&self) -> &'static str {
        "tied"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.push(("type", tied_type_str(&self.tied_type).to_string()));
        if let Some(n) = self.number {
            attrs.push(("number", n.to_string()));
        }
        if let Some(ref o) = self.orientation {
            attrs.push(("orientation", over_under_str(o).to_string()));
        }
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "bezier-x", self.bezier_x);
        push_opt_attr!(attrs, "bezier-y", self.bezier_y);
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for notations::Articulations {
    fn element_name(&self) -> &'static str {
        "articulations"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref a) = self.accent {
            serialize_empty_placement(w, "accent", a)?;
        }
        if let Some(ref a) = self.strong_accent {
            serialize_strong_accent(w, a)?;
        }
        if let Some(ref a) = self.staccato {
            serialize_empty_placement(w, "staccato", a)?;
        }
        if let Some(ref a) = self.tenuto {
            serialize_empty_placement(w, "tenuto", a)?;
        }
        if let Some(ref a) = self.detached_legato {
            serialize_empty_placement(w, "detached-legato", a)?;
        }
        if let Some(ref a) = self.staccatissimo {
            serialize_empty_placement(w, "staccatissimo", a)?;
        }
        if let Some(ref a) = self.spiccato {
            serialize_empty_placement(w, "spiccato", a)?;
        }
        if let Some(ref a) = self.scoop {
            serialize_empty_placement(w, "scoop", a)?;
        }
        if let Some(ref a) = self.plop {
            serialize_empty_placement(w, "plop", a)?;
        }
        if let Some(ref a) = self.doit {
            serialize_empty_placement(w, "doit", a)?;
        }
        if let Some(ref a) = self.falloff {
            serialize_empty_placement(w, "falloff", a)?;
        }
        if let Some(ref a) = self.stress {
            serialize_empty_placement(w, "stress", a)?;
        }
        if let Some(ref a) = self.unstress {
            serialize_empty_placement(w, "unstress", a)?;
        }
        if let Some(ref a) = self.soft_accent {
            serialize_empty_placement(w, "soft-accent", a)?;
        }
        if let Some(ref bm) = self.breath_mark {
            serialize_breath_mark(w, bm)?;
        }
        if let Some(ref c) = self.caesura {
            serialize_caesura(w, c)?;
        }
        for oa in &self.other_articulation {
            serialize_other_articulation(w, oa)?;
        }
        Ok(())
    }
}

/// Serialize an empty-placement articulation element.
fn serialize_empty_placement<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    ep: &notations::EmptyPlacement,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = ep.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    if let Some(dx) = ep.default_x {
        let s = dx.to_string();
        elem.push_attribute(("default-x", s.as_str()));
    }
    if let Some(dy) = ep.default_y {
        let s = dy.to_string();
        elem.push_attribute(("default-y", s.as_str()));
    }
    if let Some(ref c) = ep.color {
        elem.push_attribute(("color", c.as_str()));
    }
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a strong-accent element.
fn serialize_strong_accent<W: Write>(
    w: &mut MusicXmlWriter<W>,
    sa: &notations::StrongAccent,
) -> SerializeResult<()> {
    let mut elem = w.start_element("strong-accent");
    if let Some(ref t) = sa.accent_type {
        elem.push_attribute(("type", up_down_str(t)));
    }
    if let Some(ref p) = sa.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    if let Some(dx) = sa.default_x {
        let s = dx.to_string();
        elem.push_attribute(("default-x", s.as_str()));
    }
    if let Some(dy) = sa.default_y {
        let s = dy.to_string();
        elem.push_attribute(("default-y", s.as_str()));
    }
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a breath-mark element with optional text content and placement.
fn serialize_breath_mark<W: Write>(
    w: &mut MusicXmlWriter<W>,
    bm: &notations::BreathMark,
) -> SerializeResult<()> {
    let mut elem = w.start_element("breath-mark");
    if let Some(ref p) = bm.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    let text = bm.value.map(|v| match v {
        notations::BreathMarkValue::Empty => "",
        notations::BreathMarkValue::Comma => "comma",
        notations::BreathMarkValue::Tick => "tick",
        notations::BreathMarkValue::Upbow => "upbow",
        notations::BreathMarkValue::Salzedo => "salzedo",
    });
    match text {
        Some(t) if !t.is_empty() => {
            w.write_start(elem)?;
            w.write_text(t)?;
            w.write_end("breath-mark")?;
        }
        _ => {
            w.write_empty(elem)?;
        }
    }
    Ok(())
}

/// Serialize a caesura element with optional text content and placement.
fn serialize_caesura<W: Write>(
    w: &mut MusicXmlWriter<W>,
    c: &notations::Caesura,
) -> SerializeResult<()> {
    let mut elem = w.start_element("caesura");
    if let Some(ref p) = c.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    let text = c.value.map(|v| match v {
        notations::CaesuraValue::Empty => "",
        notations::CaesuraValue::Normal => "normal",
        notations::CaesuraValue::Thick => "thick",
        notations::CaesuraValue::Short => "short",
        notations::CaesuraValue::Curved => "curved",
        notations::CaesuraValue::Single => "single",
    });
    match text {
        Some(t) if !t.is_empty() => {
            w.write_start(elem)?;
            w.write_text(t)?;
            w.write_end("caesura")?;
        }
        _ => {
            w.write_empty(elem)?;
        }
    }
    Ok(())
}

/// Serialize an other-articulation element.
fn serialize_other_articulation<W: Write>(
    w: &mut MusicXmlWriter<W>,
    oa: &notations::OtherArticulation,
) -> SerializeResult<()> {
    let mut elem = w.start_element("other-articulation");
    if let Some(ref p) = oa.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_str_attr_start(&mut elem, "smufl", &oa.smufl);
    push_opt_attr_start(&mut elem, "default-x", &oa.default_x);
    push_opt_attr_start(&mut elem, "default-y", &oa.default_y);
    push_opt_str_attr_start(&mut elem, "color", &oa.color);
    if oa.value.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&oa.value)?;
        w.write_end("other-articulation")?;
    }
    Ok(())
}

// ============================================================================
// Ornaments
// ============================================================================

impl MusicXmlSerialize for notations::Ornaments {
    fn element_name(&self) -> &'static str {
        "ornaments"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref t) = self.trill_mark {
            serialize_empty_trill_sound(w, "trill-mark", t)?;
        }
        if let Some(ref t) = self.turn {
            serialize_horizontal_turn(w, "turn", t)?;
        }
        if let Some(ref t) = self.delayed_turn {
            serialize_horizontal_turn(w, "delayed-turn", t)?;
        }
        if let Some(ref t) = self.inverted_turn {
            serialize_horizontal_turn(w, "inverted-turn", t)?;
        }
        if let Some(ref t) = self.delayed_inverted_turn {
            serialize_horizontal_turn(w, "delayed-inverted-turn", t)?;
        }
        if let Some(ref t) = self.vertical_turn {
            serialize_empty_trill_sound(w, "vertical-turn", t)?;
        }
        if let Some(ref t) = self.inverted_vertical_turn {
            serialize_empty_trill_sound(w, "inverted-vertical-turn", t)?;
        }
        if let Some(ref t) = self.shake {
            serialize_empty_trill_sound(w, "shake", t)?;
        }
        if let Some(ref wl) = self.wavy_line {
            serialize_wavy_line(w, wl)?;
        }
        if let Some(ref m) = self.mordent {
            serialize_mordent(w, "mordent", m)?;
        }
        if let Some(ref m) = self.inverted_mordent {
            serialize_mordent(w, "inverted-mordent", m)?;
        }
        if let Some(ref s) = self.schleifer {
            serialize_empty_placement(w, "schleifer", s)?;
        }
        if let Some(ref t) = self.tremolo {
            serialize_tremolo(w, t)?;
        }
        if let Some(ref h) = self.haydn {
            serialize_empty_trill_sound(w, "haydn", h)?;
        }
        if let Some(ref o) = self.other_ornament {
            serialize_other_ornament(w, o)?;
        }
        for am in &self.accidental_marks {
            serialize_accidental_mark(w, am)?;
        }
        Ok(())
    }
}

/// Push trill-sound attributes onto a BytesStart element.
fn push_trill_sound_attrs(
    elem: &mut quick_xml::events::BytesStart<'_>,
    ts: &notations::TrillSound,
) {
    if let Some(ref sn) = ts.start_note {
        elem.push_attribute(("start-note", sn.to_string().as_str()));
    }
    if let Some(ref step) = ts.trill_step {
        elem.push_attribute(("trill-step", step.to_string().as_str()));
    }
    if let Some(ref tnt) = ts.two_note_turn {
        elem.push_attribute(("two-note-turn", tnt.to_string().as_str()));
    }
    if let Some(ref acc) = ts.accelerate {
        elem.push_attribute(("accelerate", yes_no_str(acc)));
    }
    if let Some(beats) = ts.beats {
        elem.push_attribute(("beats", beats.to_string().as_str()));
    }
    if let Some(sb) = ts.second_beat {
        elem.push_attribute(("second-beat", sb.to_string().as_str()));
    }
    if let Some(lb) = ts.last_beat {
        elem.push_attribute(("last-beat", lb.to_string().as_str()));
    }
}

/// Serialize an empty-trill-sound element (trill-mark, vertical-turn, shake, haydn).
fn serialize_empty_trill_sound<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    ets: &notations::EmptyTrillSound,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = ets.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &ets.default_x);
    push_opt_attr_start(&mut elem, "default-y", &ets.default_y);
    push_opt_str_attr_start(&mut elem, "color", &ets.color);
    push_trill_sound_attrs(&mut elem, &ets.trill_sound);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a horizontal-turn element (turn, delayed-turn, inverted-turn, etc.).
fn serialize_horizontal_turn<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    ht: &notations::HorizontalTurn,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = ht.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &ht.default_x);
    push_opt_attr_start(&mut elem, "default-y", &ht.default_y);
    push_opt_str_attr_start(&mut elem, "color", &ht.color);
    if let Some(ref s) = ht.slash {
        elem.push_attribute(("slash", yes_no_str(s)));
    }
    push_trill_sound_attrs(&mut elem, &ht.trill_sound);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a mordent element (mordent or inverted-mordent).
fn serialize_mordent<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    m: &notations::Mordent,
) -> SerializeResult<()> {
    let mut elem = w.start_element(name);
    if let Some(ref p) = m.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &m.default_x);
    push_opt_attr_start(&mut elem, "default-y", &m.default_y);
    push_opt_str_attr_start(&mut elem, "color", &m.color);
    if let Some(ref l) = m.long {
        elem.push_attribute(("long", yes_no_str(l)));
    }
    if let Some(ref a) = m.approach {
        elem.push_attribute(("approach", above_below_str(a)));
    }
    if let Some(ref d) = m.departure {
        elem.push_attribute(("departure", above_below_str(d)));
    }
    push_trill_sound_attrs(&mut elem, &m.trill_sound);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a wavy-line element.
pub(crate) fn serialize_wavy_line<W: Write>(
    w: &mut MusicXmlWriter<W>,
    wl: &notations::WavyLine,
) -> SerializeResult<()> {
    let mut elem = w.start_element("wavy-line");
    elem.push_attribute(("type", start_stop_continue_str(&wl.wavy_line_type)));
    if let Some(n) = wl.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    if let Some(ref p) = wl.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &wl.default_x);
    push_opt_attr_start(&mut elem, "default-y", &wl.default_y);
    push_opt_str_attr_start(&mut elem, "color", &wl.color);
    push_opt_str_attr_start(&mut elem, "smufl", &wl.smufl);
    push_trill_sound_attrs(&mut elem, &wl.trill_sound);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a tremolo element (has text content for marks count).
fn serialize_tremolo<W: Write>(
    w: &mut MusicXmlWriter<W>,
    t: &notations::Tremolo,
) -> SerializeResult<()> {
    let mut elem = w.start_element("tremolo");
    elem.push_attribute(("type", tremolo_type_str(&t.tremolo_type)));
    if let Some(ref p) = t.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_attr_start(&mut elem, "default-x", &t.default_x);
    push_opt_attr_start(&mut elem, "default-y", &t.default_y);
    push_opt_str_attr_start(&mut elem, "color", &t.color);
    push_opt_str_attr_start(&mut elem, "smufl", &t.smufl);
    if let Some(v) = t.value {
        w.write_start(elem)?;
        w.write_text(&v.to_string())?;
        w.write_end("tremolo")?;
    } else {
        w.write_empty(elem)?;
    }
    Ok(())
}

/// Serialize an other-ornament element.
fn serialize_other_ornament<W: Write>(
    w: &mut MusicXmlWriter<W>,
    o: &notations::OtherOrnament,
) -> SerializeResult<()> {
    let mut elem = w.start_element("other-ornament");
    if let Some(ref p) = o.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    w.write_start(elem)?;
    w.write_text(&o.value)?;
    w.write_end("other-ornament")?;
    Ok(())
}

/// Serialize an accidental-mark element within ornaments.
fn serialize_accidental_mark<W: Write>(
    w: &mut MusicXmlWriter<W>,
    am: &notations::AccidentalMark,
) -> SerializeResult<()> {
    let mut elem = w.start_element("accidental-mark");
    if let Some(ref p) = am.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    w.write_start(elem)?;
    w.write_text(&am.value)?;
    w.write_end("accidental-mark")?;
    Ok(())
}

fn tremolo_type_str(tt: &TremoloType) -> &'static str {
    match tt {
        TremoloType::Single => "single",
        TremoloType::Start => "start",
        TremoloType::Stop => "stop",
        TremoloType::Unmeasured => "unmeasured",
    }
}

// ============================================================================
// Dynamics (within notations)
// ============================================================================

/// Serialize a `<dynamics>` element within `<notations>`.
fn serialize_dynamics_notation<W: Write>(
    w: &mut MusicXmlWriter<W>,
    dynamics: &crate::model::direction::Dynamics,
) -> SerializeResult<()> {
    let mut elem = w.start_element("dynamics");
    if let Some(ref p) = dynamics.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    if dynamics.values.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        for d in &dynamics.values {
            super::elements::serialize_dynamics_value(w, d)?;
        }
        w.write_end("dynamics")?;
    }
    Ok(())
}

// ============================================================================
// Fermata, Arpeggiate, Glissando, Slide, OtherNotation
// ============================================================================

/// Serialize a fermata element (may have text content for shape).
pub(crate) fn serialize_fermata<W: Write>(
    w: &mut MusicXmlWriter<W>,
    f: &notations::Fermata,
) -> SerializeResult<()> {
    let mut elem = w.start_element("fermata");
    if let Some(ref t) = f.fermata_type {
        elem.push_attribute(("type", upright_inverted_str(t)));
    }
    push_opt_attr_start(&mut elem, "default-x", &f.default_x);
    push_opt_attr_start(&mut elem, "default-y", &f.default_y);
    push_opt_attr_start(&mut elem, "relative-x", &f.relative_x);
    push_opt_attr_start(&mut elem, "relative-y", &f.relative_y);
    push_opt_str_attr_start(&mut elem, "color", &f.color);
    if let Some(ref shape) = f.shape {
        let s = fermata_shape_str(shape);
        if s.is_empty() {
            // Empty shape = empty fermata element
            w.write_empty(elem)?;
        } else {
            w.write_start(elem)?;
            w.write_text(s)?;
            w.write_end("fermata")?;
        }
    } else {
        w.write_empty(elem)?;
    }
    Ok(())
}

/// Serialize an arpeggiate element.
fn serialize_arpeggiate<W: Write>(
    w: &mut MusicXmlWriter<W>,
    a: &notations::Arpeggiate,
) -> SerializeResult<()> {
    let mut elem = w.start_element("arpeggiate");
    if let Some(n) = a.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    if let Some(ref d) = a.direction {
        elem.push_attribute(("direction", up_down_str(d)));
    }
    if let Some(ref u) = a.unbroken {
        elem.push_attribute(("unbroken", yes_no_str(u)));
    }
    push_opt_attr_start(&mut elem, "default-x", &a.default_x);
    push_opt_attr_start(&mut elem, "default-y", &a.default_y);
    if let Some(ref p) = a.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_str_attr_start(&mut elem, "color", &a.color);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a non-arpeggiate element.
fn serialize_non_arpeggiate<W: Write>(
    w: &mut MusicXmlWriter<W>,
    na: &notations::NonArpeggiate,
) -> SerializeResult<()> {
    let mut elem = w.start_element("non-arpeggiate");
    elem.push_attribute(("type", top_bottom_str(&na.non_arpeggiate_type)));
    if let Some(n) = na.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    push_opt_attr_start(&mut elem, "default-x", &na.default_x);
    push_opt_attr_start(&mut elem, "default-y", &na.default_y);
    if let Some(ref p) = na.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_str_attr_start(&mut elem, "color", &na.color);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize a glissando element (may have text content).
fn serialize_glissando<W: Write>(
    w: &mut MusicXmlWriter<W>,
    g: &notations::Glissando,
) -> SerializeResult<()> {
    let mut elem = w.start_element("glissando");
    if let Some(ref lt) = g.line_type {
        elem.push_attribute(("line-type", line_type_str(lt)));
    }
    if let Some(n) = g.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    elem.push_attribute(("type", start_stop_str(&g.glissando_type)));
    push_opt_attr_start(&mut elem, "default-x", &g.default_x);
    push_opt_attr_start(&mut elem, "default-y", &g.default_y);
    push_opt_str_attr_start(&mut elem, "color", &g.color);
    if g.text.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&g.text)?;
        w.write_end("glissando")?;
    }
    Ok(())
}

/// Serialize a slide element (may have text content).
fn serialize_slide<W: Write>(
    w: &mut MusicXmlWriter<W>,
    s: &notations::Slide,
) -> SerializeResult<()> {
    let mut elem = w.start_element("slide");
    if let Some(ref lt) = s.line_type {
        elem.push_attribute(("line-type", line_type_str(lt)));
    }
    if let Some(n) = s.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    elem.push_attribute(("type", start_stop_str(&s.slide_type)));
    push_opt_attr_start(&mut elem, "default-x", &s.default_x);
    push_opt_attr_start(&mut elem, "default-y", &s.default_y);
    push_opt_str_attr_start(&mut elem, "color", &s.color);
    if s.text.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&s.text)?;
        w.write_end("slide")?;
    }
    Ok(())
}

/// Serialize an other-notation element.
fn serialize_other_notation<W: Write>(
    w: &mut MusicXmlWriter<W>,
    on: &notations::OtherNotation,
) -> SerializeResult<()> {
    let mut elem = w.start_element("other-notation");
    elem.push_attribute(("type", start_stop_single_str(&on.notation_type)));
    if let Some(n) = on.number {
        elem.push_attribute(("number", n.to_string().as_str()));
    }
    if let Some(ref p) = on.placement {
        elem.push_attribute(("placement", above_below_str(p)));
    }
    push_opt_str_attr_start(&mut elem, "smufl", &on.smufl);
    if on.text.is_empty() {
        w.write_empty(elem)?;
    } else {
        w.write_start(elem)?;
        w.write_text(&on.text)?;
        w.write_end("other-notation")?;
    }
    Ok(())
}

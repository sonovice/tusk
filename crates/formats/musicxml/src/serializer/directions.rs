//! Serialization for MusicXML direction types.
//!
//! Handles serialization of all direction-type children:
//! Rehearsal, Segno, Coda, Symbol, Dashes, Bracket, Pedal, OctaveShift,
//! HarpPedals, Damp, DampAll, Eyeglasses, StringMute, Scordatura,
//! Image, PrincipalVoice, Percussion, AccordionRegistration, StaffDivide,
//! OtherDirection.

use std::io::Write;

use crate::model::direction::{
    Accord, AccordionRegistration, Bracket, Coda, Damp, DampAll, Dashes, DirectionImage,
    Eyeglasses, HarpPedals, LineEnd, OctaveShift, OctaveShiftType, OtherDirection, PedalTuning,
    PedalType, Percussion, PercussionContent, PrincipalVoice, PrincipalVoiceSymbol, Rehearsal,
    Scordatura, Segno, StaffDivide, StaffDivideType, StringMute, StringMuteType, Symbol,
    TipDirection,
};

use super::score::{
    font_size_str, font_style_str, font_weight_str, left_center_right_str, start_stop_continue_str,
    start_stop_str, valign_str, yes_no_str,
};
use super::{MusicXmlWriter, SerializeResult};

// ============================================================================
// Attribute helpers
// ============================================================================

fn push_opt_f64(
    start: &mut quick_xml::events::BytesStart<'_>,
    name: &'static str,
    val: &Option<f64>,
) {
    if let Some(v) = val {
        start.push_attribute((name, v.to_string().as_str()));
    }
}

fn push_opt_str(
    start: &mut quick_xml::events::BytesStart<'_>,
    name: &'static str,
    val: &Option<String>,
) {
    if let Some(v) = val {
        start.push_attribute((name, v.as_str()));
    }
}

// ============================================================================
// Rehearsal
// ============================================================================

pub(crate) fn serialize_rehearsal<W: Write>(
    w: &mut MusicXmlWriter<W>,
    r: &Rehearsal,
) -> SerializeResult<()> {
    let mut start = w.start_element("rehearsal");
    push_opt_str(&mut start, "font-family", &r.font_family);
    if let Some(ref fs) = r.font_style {
        start.push_attribute(("font-style", font_style_str(fs)));
    }
    if let Some(ref fs) = r.font_size {
        start.push_attribute(("font-size", font_size_str(fs).as_str()));
    }
    if let Some(ref fw) = r.font_weight {
        start.push_attribute(("font-weight", font_weight_str(fw)));
    }
    push_opt_str(&mut start, "color", &r.color);
    if let Some(ref h) = r.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = r.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    if let Some(ref enc) = r.enclosure {
        start.push_attribute(("enclosure", super::elements::enclosure_shape_str(enc)));
    }
    push_opt_str(&mut start, "id", &r.id);
    w.write_start(start)?;
    w.write_text(&r.value)?;
    w.write_end("rehearsal")?;
    Ok(())
}

// ============================================================================
// Segno / Coda (empty elements with optional attributes)
// ============================================================================

pub(crate) fn serialize_segno<W: Write>(
    w: &mut MusicXmlWriter<W>,
    s: &Segno,
) -> SerializeResult<()> {
    let mut start = w.start_element("segno");
    push_opt_str(&mut start, "smufl", &s.smufl);
    push_opt_f64(&mut start, "default-x", &s.default_x);
    push_opt_f64(&mut start, "default-y", &s.default_y);
    push_opt_str(&mut start, "color", &s.color);
    if let Some(ref h) = s.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = s.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &s.id);
    w.write_empty(start)?;
    Ok(())
}

pub(crate) fn serialize_coda<W: Write>(w: &mut MusicXmlWriter<W>, c: &Coda) -> SerializeResult<()> {
    let mut start = w.start_element("coda");
    push_opt_str(&mut start, "smufl", &c.smufl);
    push_opt_f64(&mut start, "default-x", &c.default_x);
    push_opt_f64(&mut start, "default-y", &c.default_y);
    push_opt_str(&mut start, "color", &c.color);
    if let Some(ref h) = c.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = c.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &c.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Symbol
// ============================================================================

pub(crate) fn serialize_symbol<W: Write>(
    w: &mut MusicXmlWriter<W>,
    s: &Symbol,
) -> SerializeResult<()> {
    let mut start = w.start_element("symbol");
    push_opt_str(&mut start, "font-family", &s.font_family);
    if let Some(ref fs) = s.font_size {
        start.push_attribute(("font-size", font_size_str(fs).as_str()));
    }
    push_opt_str(&mut start, "color", &s.color);
    if let Some(ref h) = s.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = s.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &s.id);
    w.write_start(start)?;
    w.write_text(&s.value)?;
    w.write_end("symbol")?;
    Ok(())
}

// ============================================================================
// Dashes
// ============================================================================

pub(crate) fn serialize_dashes<W: Write>(
    w: &mut MusicXmlWriter<W>,
    d: &Dashes,
) -> SerializeResult<()> {
    let mut start = w.start_element("dashes");
    start.push_attribute(("type", start_stop_continue_str(&d.dash_type)));
    if let Some(n) = d.number {
        start.push_attribute(("number", n.to_string().as_str()));
    }
    push_opt_f64(&mut start, "dash-length", &d.dash_length);
    push_opt_f64(&mut start, "space-length", &d.space_length);
    push_opt_f64(&mut start, "default-x", &d.default_x);
    push_opt_f64(&mut start, "default-y", &d.default_y);
    push_opt_str(&mut start, "color", &d.color);
    push_opt_str(&mut start, "id", &d.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Bracket
// ============================================================================

fn line_end_str(le: &LineEnd) -> &'static str {
    match le {
        LineEnd::Up => "up",
        LineEnd::Down => "down",
        LineEnd::Both => "both",
        LineEnd::Arrow => "arrow",
        LineEnd::None => "none",
    }
}

pub(crate) fn serialize_bracket<W: Write>(
    w: &mut MusicXmlWriter<W>,
    b: &Bracket,
) -> SerializeResult<()> {
    let mut start = w.start_element("bracket");
    start.push_attribute(("type", start_stop_continue_str(&b.bracket_type)));
    if let Some(n) = b.number {
        start.push_attribute(("number", n.to_string().as_str()));
    }
    start.push_attribute(("line-end", line_end_str(&b.line_end)));
    push_opt_f64(&mut start, "end-length", &b.end_length);
    if let Some(ref lt) = b.line_type {
        start.push_attribute(("line-type", super::score::line_type_str(lt)));
    }
    push_opt_f64(&mut start, "dash-length", &b.dash_length);
    push_opt_f64(&mut start, "space-length", &b.space_length);
    push_opt_f64(&mut start, "default-x", &b.default_x);
    push_opt_f64(&mut start, "default-y", &b.default_y);
    push_opt_str(&mut start, "color", &b.color);
    push_opt_str(&mut start, "id", &b.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Pedal
// ============================================================================

fn pedal_type_str(pt: &PedalType) -> &'static str {
    match pt {
        PedalType::Start => "start",
        PedalType::Stop => "stop",
        PedalType::Sostenuto => "sostenuto",
        PedalType::Change => "change",
        PedalType::Continue => "continue",
        PedalType::Discontinue => "discontinue",
        PedalType::Resume => "resume",
    }
}

pub(crate) fn serialize_pedal<W: Write>(
    w: &mut MusicXmlWriter<W>,
    p: &crate::model::direction::Pedal,
) -> SerializeResult<()> {
    let mut start = w.start_element("pedal");
    start.push_attribute(("type", pedal_type_str(&p.pedal_type)));
    if let Some(n) = p.number {
        start.push_attribute(("number", n.to_string().as_str()));
    }
    if let Some(ref l) = p.line {
        start.push_attribute(("line", yes_no_str(l)));
    }
    if let Some(ref s) = p.sign {
        start.push_attribute(("sign", yes_no_str(s)));
    }
    if let Some(ref a) = p.abbreviated {
        start.push_attribute(("abbreviated", yes_no_str(a)));
    }
    push_opt_f64(&mut start, "default-x", &p.default_x);
    push_opt_f64(&mut start, "default-y", &p.default_y);
    push_opt_f64(&mut start, "relative-x", &p.relative_x);
    push_opt_f64(&mut start, "relative-y", &p.relative_y);
    if let Some(ref h) = p.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = p.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "color", &p.color);
    push_opt_str(&mut start, "id", &p.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Octave Shift
// ============================================================================

fn octave_shift_type_str(t: &OctaveShiftType) -> &'static str {
    match t {
        OctaveShiftType::Up => "up",
        OctaveShiftType::Down => "down",
        OctaveShiftType::Stop => "stop",
        OctaveShiftType::Continue => "continue",
    }
}

pub(crate) fn serialize_octave_shift<W: Write>(
    w: &mut MusicXmlWriter<W>,
    o: &OctaveShift,
) -> SerializeResult<()> {
    let mut start = w.start_element("octave-shift");
    start.push_attribute(("type", octave_shift_type_str(&o.shift_type)));
    if let Some(n) = o.number {
        start.push_attribute(("number", n.to_string().as_str()));
    }
    if let Some(s) = o.size {
        start.push_attribute(("size", s.to_string().as_str()));
    }
    push_opt_f64(&mut start, "dash-length", &o.dash_length);
    push_opt_f64(&mut start, "space-length", &o.space_length);
    push_opt_f64(&mut start, "default-x", &o.default_x);
    push_opt_f64(&mut start, "default-y", &o.default_y);
    push_opt_str(&mut start, "font-family", &o.font_family);
    if let Some(ref fs) = o.font_size {
        start.push_attribute(("font-size", font_size_str(fs).as_str()));
    }
    push_opt_str(&mut start, "color", &o.color);
    push_opt_str(&mut start, "id", &o.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Harp Pedals
// ============================================================================

fn serialize_pedal_tuning<W: Write>(
    w: &mut MusicXmlWriter<W>,
    pt: &PedalTuning,
) -> SerializeResult<()> {
    let start = w.start_element("pedal-tuning");
    w.write_start(start)?;
    w.write_text_element("pedal-step", &pt.pedal_step)?;
    w.write_text_element("pedal-alter", &pt.pedal_alter.to_string())?;
    w.write_end("pedal-tuning")?;
    Ok(())
}

pub(crate) fn serialize_harp_pedals<W: Write>(
    w: &mut MusicXmlWriter<W>,
    hp: &HarpPedals,
) -> SerializeResult<()> {
    let mut start = w.start_element("harp-pedals");
    push_opt_f64(&mut start, "default-x", &hp.default_x);
    push_opt_f64(&mut start, "default-y", &hp.default_y);
    if let Some(ref h) = hp.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = hp.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &hp.id);
    w.write_start(start)?;
    for pt in &hp.pedal_tunings {
        serialize_pedal_tuning(w, pt)?;
    }
    w.write_end("harp-pedals")?;
    Ok(())
}

// ============================================================================
// Simple empty direction types: Damp, DampAll, Eyeglasses
// ============================================================================

pub(crate) fn serialize_damp<W: Write>(w: &mut MusicXmlWriter<W>, d: &Damp) -> SerializeResult<()> {
    let mut start = w.start_element("damp");
    push_opt_f64(&mut start, "default-x", &d.default_x);
    push_opt_f64(&mut start, "default-y", &d.default_y);
    if let Some(ref h) = d.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = d.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &d.id);
    w.write_empty(start)?;
    Ok(())
}

pub(crate) fn serialize_damp_all<W: Write>(
    w: &mut MusicXmlWriter<W>,
    d: &DampAll,
) -> SerializeResult<()> {
    let mut start = w.start_element("damp-all");
    push_opt_f64(&mut start, "default-x", &d.default_x);
    push_opt_f64(&mut start, "default-y", &d.default_y);
    if let Some(ref h) = d.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = d.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &d.id);
    w.write_empty(start)?;
    Ok(())
}

pub(crate) fn serialize_eyeglasses<W: Write>(
    w: &mut MusicXmlWriter<W>,
    e: &Eyeglasses,
) -> SerializeResult<()> {
    let mut start = w.start_element("eyeglasses");
    push_opt_f64(&mut start, "default-x", &e.default_x);
    push_opt_f64(&mut start, "default-y", &e.default_y);
    if let Some(ref h) = e.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = e.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &e.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// String Mute
// ============================================================================

fn string_mute_type_str(t: &StringMuteType) -> &'static str {
    match t {
        StringMuteType::On => "on",
        StringMuteType::Off => "off",
    }
}

pub(crate) fn serialize_string_mute<W: Write>(
    w: &mut MusicXmlWriter<W>,
    sm: &StringMute,
) -> SerializeResult<()> {
    let mut start = w.start_element("string-mute");
    start.push_attribute(("type", string_mute_type_str(&sm.mute_type)));
    push_opt_f64(&mut start, "default-x", &sm.default_x);
    push_opt_f64(&mut start, "default-y", &sm.default_y);
    if let Some(ref h) = sm.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = sm.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &sm.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Scordatura
// ============================================================================

fn serialize_accord<W: Write>(w: &mut MusicXmlWriter<W>, a: &Accord) -> SerializeResult<()> {
    let mut start = w.start_element("accord");
    start.push_attribute(("string", a.string.to_string().as_str()));
    w.write_start(start)?;
    w.write_text_element("tuning-step", &a.tuning_step)?;
    if let Some(alter) = a.tuning_alter {
        w.write_text_element("tuning-alter", &alter.to_string())?;
    }
    w.write_text_element("tuning-octave", &a.tuning_octave.to_string())?;
    w.write_end("accord")?;
    Ok(())
}

pub(crate) fn serialize_scordatura<W: Write>(
    w: &mut MusicXmlWriter<W>,
    s: &Scordatura,
) -> SerializeResult<()> {
    let mut start = w.start_element("scordatura");
    push_opt_str(&mut start, "id", &s.id);
    w.write_start(start)?;
    for a in &s.accords {
        serialize_accord(w, a)?;
    }
    w.write_end("scordatura")?;
    Ok(())
}

// ============================================================================
// Image
// ============================================================================

fn valign_image_str(v: &crate::model::data::ValignImage) -> &'static str {
    match v {
        crate::model::data::ValignImage::Top => "top",
        crate::model::data::ValignImage::Middle => "middle",
        crate::model::data::ValignImage::Bottom => "bottom",
    }
}

pub(crate) fn serialize_image<W: Write>(
    w: &mut MusicXmlWriter<W>,
    img: &DirectionImage,
) -> SerializeResult<()> {
    let mut start = w.start_element("image");
    start.push_attribute(("source", img.source.as_str()));
    start.push_attribute(("type", img.image_type.as_str()));
    push_opt_f64(&mut start, "height", &img.height);
    push_opt_f64(&mut start, "width", &img.width);
    push_opt_f64(&mut start, "default-x", &img.default_x);
    push_opt_f64(&mut start, "default-y", &img.default_y);
    if let Some(ref h) = img.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = img.valign {
        start.push_attribute(("valign", valign_image_str(v)));
    }
    push_opt_str(&mut start, "id", &img.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Principal Voice
// ============================================================================

fn principal_voice_symbol_str(s: &PrincipalVoiceSymbol) -> &'static str {
    match s {
        PrincipalVoiceSymbol::Hauptstimme => "Hauptstimme",
        PrincipalVoiceSymbol::Nebenstimme => "Nebenstimme",
        PrincipalVoiceSymbol::Plain => "plain",
        PrincipalVoiceSymbol::None => "none",
    }
}

pub(crate) fn serialize_principal_voice<W: Write>(
    w: &mut MusicXmlWriter<W>,
    pv: &PrincipalVoice,
) -> SerializeResult<()> {
    let mut start = w.start_element("principal-voice");
    start.push_attribute(("type", start_stop_str(&pv.voice_type)));
    start.push_attribute(("symbol", principal_voice_symbol_str(&pv.symbol)));
    push_opt_f64(&mut start, "default-x", &pv.default_x);
    push_opt_f64(&mut start, "default-y", &pv.default_y);
    if let Some(ref h) = pv.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = pv.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &pv.id);
    if let Some(ref text) = pv.value {
        w.write_start(start)?;
        w.write_text(text)?;
        w.write_end("principal-voice")?;
    } else {
        w.write_empty(start)?;
    }
    Ok(())
}

// ============================================================================
// Percussion
// ============================================================================

fn tip_direction_str(t: &TipDirection) -> &'static str {
    match t {
        TipDirection::Up => "up",
        TipDirection::Down => "down",
        TipDirection::Left => "left",
        TipDirection::Right => "right",
        TipDirection::Northwest => "northwest",
        TipDirection::Northeast => "northeast",
        TipDirection::Southeast => "southeast",
        TipDirection::Southwest => "southwest",
    }
}

pub(crate) fn serialize_percussion<W: Write>(
    w: &mut MusicXmlWriter<W>,
    p: &Percussion,
) -> SerializeResult<()> {
    let mut start = w.start_element("percussion");
    if let Some(ref enc) = p.enclosure {
        start.push_attribute(("enclosure", super::elements::enclosure_shape_str(enc)));
    }
    push_opt_f64(&mut start, "default-x", &p.default_x);
    push_opt_f64(&mut start, "default-y", &p.default_y);
    if let Some(ref h) = p.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = p.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &p.id);
    w.write_start(start)?;
    serialize_percussion_content(w, &p.content)?;
    w.write_end("percussion")?;
    Ok(())
}

fn serialize_percussion_content<W: Write>(
    w: &mut MusicXmlWriter<W>,
    content: &PercussionContent,
) -> SerializeResult<()> {
    match content {
        PercussionContent::Glass(v) => w.write_text_element("glass", v)?,
        PercussionContent::Metal(v) => w.write_text_element("metal", v)?,
        PercussionContent::Wood(v) => w.write_text_element("wood", v)?,
        PercussionContent::Pitched(v) => w.write_text_element("pitched", v)?,
        PercussionContent::Membrane(v) => w.write_text_element("membrane", v)?,
        PercussionContent::Effect(v) => w.write_text_element("effect", v)?,
        PercussionContent::Timpani => {
            w.write_empty(w.start_element("timpani"))?;
        }
        PercussionContent::Beater(b) => {
            let mut start = w.start_element("beater");
            if let Some(ref tip) = b.tip {
                start.push_attribute(("tip", tip_direction_str(tip)));
            }
            w.write_start(start)?;
            w.write_text(&b.value)?;
            w.write_end("beater")?;
        }
        PercussionContent::Stick(s) => {
            let mut start = w.start_element("stick");
            if let Some(ref tip) = s.tip {
                start.push_attribute(("tip", tip_direction_str(tip)));
            }
            if let Some(ref p) = s.parentheses {
                start.push_attribute(("parentheses", yes_no_str(p)));
            }
            if let Some(ref dc) = s.dashed_circle {
                start.push_attribute(("dashed-circle", yes_no_str(dc)));
            }
            w.write_start(start)?;
            w.write_text_element("stick-type", &s.stick_type)?;
            w.write_text_element("stick-material", &s.stick_material)?;
            w.write_end("stick")?;
        }
        PercussionContent::StickLocation(v) => w.write_text_element("stick-location", v)?,
        PercussionContent::OtherPercussion(v) => w.write_text_element("other-percussion", v)?,
    }
    Ok(())
}

// ============================================================================
// Accordion Registration
// ============================================================================

pub(crate) fn serialize_accordion_registration<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ar: &AccordionRegistration,
) -> SerializeResult<()> {
    let mut start = w.start_element("accordion-registration");
    push_opt_f64(&mut start, "default-x", &ar.default_x);
    push_opt_f64(&mut start, "default-y", &ar.default_y);
    if let Some(ref h) = ar.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = ar.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &ar.id);
    // Has children if any section is present
    let has_children =
        ar.accordion_high.is_some() || ar.accordion_middle.is_some() || ar.accordion_low.is_some();
    if has_children {
        w.write_start(start)?;
        if ar.accordion_high.is_some() {
            w.write_empty(w.start_element("accordion-high"))?;
        }
        if let Some(m) = ar.accordion_middle {
            w.write_text_element("accordion-middle", &m.to_string())?;
        }
        if ar.accordion_low.is_some() {
            w.write_empty(w.start_element("accordion-low"))?;
        }
        w.write_end("accordion-registration")?;
    } else {
        w.write_empty(start)?;
    }
    Ok(())
}

// ============================================================================
// Staff Divide
// ============================================================================

fn staff_divide_type_str(t: &StaffDivideType) -> &'static str {
    match t {
        StaffDivideType::Down => "down",
        StaffDivideType::Up => "up",
        StaffDivideType::UpDown => "up-down",
    }
}

pub(crate) fn serialize_staff_divide<W: Write>(
    w: &mut MusicXmlWriter<W>,
    sd: &StaffDivide,
) -> SerializeResult<()> {
    let mut start = w.start_element("staff-divide");
    start.push_attribute(("type", staff_divide_type_str(&sd.divide_type)));
    push_opt_f64(&mut start, "default-x", &sd.default_x);
    push_opt_f64(&mut start, "default-y", &sd.default_y);
    if let Some(ref h) = sd.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = sd.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &sd.id);
    w.write_empty(start)?;
    Ok(())
}

// ============================================================================
// Other Direction
// ============================================================================

pub(crate) fn serialize_other_direction<W: Write>(
    w: &mut MusicXmlWriter<W>,
    od: &OtherDirection,
) -> SerializeResult<()> {
    let mut start = w.start_element("other-direction");
    if let Some(ref po) = od.print_object {
        start.push_attribute(("print-object", yes_no_str(po)));
    }
    push_opt_str(&mut start, "smufl", &od.smufl);
    push_opt_f64(&mut start, "default-x", &od.default_x);
    push_opt_f64(&mut start, "default-y", &od.default_y);
    if let Some(ref h) = od.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = od.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str(&mut start, "id", &od.id);
    if let Some(ref text) = od.value {
        w.write_start(start)?;
        w.write_text(text)?;
        w.write_end("other-direction")?;
    } else {
        w.write_empty(start)?;
    }
    Ok(())
}

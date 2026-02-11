//! Serializer implementations for MusicXML harmony types.

use std::io::Write;

use crate::model::harmony::*;
use crate::serializer::{
    MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr,
};

use super::score::{
    above_below_str, left_center_right_str, left_right_str, push_opt_attr_start,
    push_opt_str_attr_start, start_stop_str, valign_str, yes_no_str,
};

impl MusicXmlSerialize for Harmony {
    fn element_name(&self) -> &'static str {
        "harmony"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref t) = self.harmony_type {
            attrs.push(("type", harmony_type_str(t).to_string()));
        }
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        if let Some(ref pf) = self.print_frame {
            attrs.push(("print-frame", yes_no_str(pf).to_string()));
        }
        if let Some(ref a) = self.arrangement {
            attrs.push(("arrangement", harmony_arrangement_str(a).to_string()));
        }
        push_opt_str_attr!(attrs, "font-family", self.font_family);
        push_opt_attr!(attrs, "font-size", self.font_size);
        push_opt_str_attr!(attrs, "font-style", self.font_style);
        push_opt_str_attr!(attrs, "font-weight", self.font_weight);
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for chord in &self.chords {
            serialize_harmony_chord(w, chord)?;
        }
        if let Some(ref frame) = self.frame {
            serialize_frame(w, frame)?;
        }
        if let Some(ref offset) = self.offset {
            let mut start = w.start_element("offset");
            if let Some(ref sound) = offset.sound {
                start.push_attribute(("sound", yes_no_str(sound)));
            }
            w.write_start(start)?;
            w.write_text(&format_decimal(offset.value))?;
            w.write_end("offset")?;
        }
        // Editorial (footnote, level) — XSD: after offset, before staff
        if let Some(ref ft) = self.footnote {
            super::elements::serialize_formatted_text(w, "footnote", ft)?;
        }
        if let Some(ref lv) = self.level {
            super::elements::serialize_level(w, lv)?;
        }
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }
        Ok(())
    }
}

fn serialize_harmony_chord<W: Write>(
    w: &mut MusicXmlWriter<W>,
    chord: &HarmonyChord,
) -> SerializeResult<()> {
    // Root type (root, numeral, or function)
    match &chord.root_type {
        HarmonyChordRoot::Root(root) => serialize_root(w, root)?,
        HarmonyChordRoot::Numeral(numeral) => serialize_numeral(w, numeral)?,
        HarmonyChordRoot::Function(func) => serialize_style_text(w, "function", func)?,
    }

    // Kind
    serialize_kind(w, &chord.kind)?;

    // Inversion
    if let Some(ref inv) = chord.inversion {
        let mut start = w.start_element("inversion");
        push_opt_str_attr_start(&mut start, "text", &inv.text);
        w.write_start(start)?;
        w.write_text(&inv.value.to_string())?;
        w.write_end("inversion")?;
    }

    // Bass
    if let Some(ref bass) = chord.bass {
        serialize_bass(w, bass)?;
    }

    // Degrees
    for degree in &chord.degrees {
        serialize_degree(w, degree)?;
    }

    Ok(())
}

// ============================================================================
// Root
// ============================================================================

fn serialize_root<W: Write>(w: &mut MusicXmlWriter<W>, root: &Root) -> SerializeResult<()> {
    let start = w.start_element("root");
    w.write_start(start)?;

    // root-step
    let mut step_start = w.start_element("root-step");
    push_opt_str_attr_start(&mut step_start, "text", &root.root_step.text);
    w.write_start(step_start)?;
    w.write_text(&root.root_step.value.to_string())?;
    w.write_end("root-step")?;

    // root-alter
    if let Some(ref alter) = root.root_alter {
        serialize_harmony_alter(w, "root-alter", alter)?;
    }

    w.write_end("root")?;
    Ok(())
}

// ============================================================================
// HarmonyAlter (shared for root-alter, bass-alter, numeral-alter)
// ============================================================================

fn serialize_harmony_alter<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    alter: &HarmonyAlter,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    if let Some(ref po) = alter.print_object {
        start.push_attribute(("print-object", yes_no_str(po)));
    }
    if let Some(ref loc) = alter.location {
        start.push_attribute(("location", left_right_str(loc)));
    }
    w.write_start(start)?;
    w.write_text(&format_decimal(alter.value))?;
    w.write_end(name)?;
    Ok(())
}

// ============================================================================
// Kind
// ============================================================================

fn serialize_kind<W: Write>(w: &mut MusicXmlWriter<W>, kind: &Kind) -> SerializeResult<()> {
    let mut start = w.start_element("kind");
    push_opt_str_attr_start(&mut start, "text", &kind.text);
    if let Some(ref us) = kind.use_symbols {
        start.push_attribute(("use-symbols", yes_no_str(us)));
    }
    if let Some(ref sd) = kind.stack_degrees {
        start.push_attribute(("stack-degrees", yes_no_str(sd)));
    }
    if let Some(ref pd) = kind.parentheses_degrees {
        start.push_attribute(("parentheses-degrees", yes_no_str(pd)));
    }
    if let Some(ref bd) = kind.bracket_degrees {
        start.push_attribute(("bracket-degrees", yes_no_str(bd)));
    }
    if let Some(ref h) = kind.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = kind.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    w.write_start(start)?;
    w.write_text(kind.value.as_str())?;
    w.write_end("kind")?;
    Ok(())
}

// ============================================================================
// Bass
// ============================================================================

fn serialize_bass<W: Write>(w: &mut MusicXmlWriter<W>, bass: &Bass) -> SerializeResult<()> {
    let mut start = w.start_element("bass");
    if let Some(ref arr) = bass.arrangement {
        start.push_attribute(("arrangement", harmony_arrangement_str(arr)));
    }
    w.write_start(start)?;

    // bass-separator
    if let Some(ref sep) = bass.bass_separator {
        serialize_style_text(w, "bass-separator", sep)?;
    }

    // bass-step
    let mut step_start = w.start_element("bass-step");
    push_opt_str_attr_start(&mut step_start, "text", &bass.bass_step.text);
    w.write_start(step_start)?;
    w.write_text(&bass.bass_step.value.to_string())?;
    w.write_end("bass-step")?;

    // bass-alter
    if let Some(ref alter) = bass.bass_alter {
        serialize_harmony_alter(w, "bass-alter", alter)?;
    }

    w.write_end("bass")?;
    Ok(())
}

// ============================================================================
// Degree
// ============================================================================

fn serialize_degree<W: Write>(w: &mut MusicXmlWriter<W>, degree: &Degree) -> SerializeResult<()> {
    let mut start = w.start_element("degree");
    if let Some(ref po) = degree.print_object {
        start.push_attribute(("print-object", yes_no_str(po)));
    }
    w.write_start(start)?;

    // degree-value
    let mut dv_start = w.start_element("degree-value");
    if let Some(ref sym) = degree.degree_value.symbol {
        dv_start.push_attribute(("symbol", sym.as_str()));
    }
    push_opt_str_attr_start(&mut dv_start, "text", &degree.degree_value.text);
    w.write_start(dv_start)?;
    w.write_text(&degree.degree_value.value.to_string())?;
    w.write_end("degree-value")?;

    // degree-alter
    let mut da_start = w.start_element("degree-alter");
    if let Some(ref pm) = degree.degree_alter.plus_minus {
        da_start.push_attribute(("plus-minus", yes_no_str(pm)));
    }
    w.write_start(da_start)?;
    w.write_text(&format_decimal(degree.degree_alter.value))?;
    w.write_end("degree-alter")?;

    // degree-type
    let mut dt_start = w.start_element("degree-type");
    push_opt_str_attr_start(&mut dt_start, "text", &degree.degree_type.text);
    w.write_start(dt_start)?;
    w.write_text(degree.degree_type.value.as_str())?;
    w.write_end("degree-type")?;

    w.write_end("degree")?;
    Ok(())
}

// ============================================================================
// Numeral
// ============================================================================

fn serialize_numeral<W: Write>(
    w: &mut MusicXmlWriter<W>,
    numeral: &Numeral,
) -> SerializeResult<()> {
    let start = w.start_element("numeral");
    w.write_start(start)?;

    // numeral-root
    let mut nr_start = w.start_element("numeral-root");
    push_opt_str_attr_start(&mut nr_start, "text", &numeral.numeral_root.text);
    w.write_start(nr_start)?;
    w.write_text(&numeral.numeral_root.value.to_string())?;
    w.write_end("numeral-root")?;

    // numeral-alter
    if let Some(ref alter) = numeral.numeral_alter {
        serialize_harmony_alter(w, "numeral-alter", alter)?;
    }

    // numeral-key
    if let Some(ref key) = numeral.numeral_key {
        let mut nk_start = w.start_element("numeral-key");
        if let Some(ref po) = key.print_object {
            nk_start.push_attribute(("print-object", yes_no_str(po)));
        }
        w.write_start(nk_start)?;
        w.write_text_element("numeral-fifths", &key.numeral_fifths.to_string())?;
        w.write_text_element("numeral-mode", key.numeral_mode.as_str())?;
        w.write_end("numeral-key")?;
    }

    w.write_end("numeral")?;
    Ok(())
}

// ============================================================================
// Frame
// ============================================================================

fn serialize_frame<W: Write>(w: &mut MusicXmlWriter<W>, frame: &Frame) -> SerializeResult<()> {
    let mut start = w.start_element("frame");
    push_opt_attr_start(&mut start, "default-x", &frame.default_x);
    push_opt_attr_start(&mut start, "default-y", &frame.default_y);
    if let Some(ref h) = frame.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = frame.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    push_opt_attr_start(&mut start, "height", &frame.height);
    push_opt_attr_start(&mut start, "width", &frame.width);
    push_opt_str_attr_start(&mut start, "unplayed", &frame.unplayed);
    push_opt_str_attr_start(&mut start, "color", &frame.color);
    push_opt_str_attr_start(&mut start, "id", &frame.id);
    w.write_start(start)?;

    w.write_text_element("frame-strings", &frame.frame_strings.to_string())?;
    w.write_text_element("frame-frets", &frame.frame_frets.to_string())?;

    if let Some(ref ff) = frame.first_fret {
        let mut ff_start = w.start_element("first-fret");
        push_opt_str_attr_start(&mut ff_start, "text", &ff.text);
        if let Some(ref loc) = ff.location {
            ff_start.push_attribute(("location", left_right_str(loc)));
        }
        w.write_start(ff_start)?;
        w.write_text(&ff.value.to_string())?;
        w.write_end("first-fret")?;
    }

    for note in &frame.frame_notes {
        serialize_frame_note(w, note)?;
    }

    w.write_end("frame")?;
    Ok(())
}

fn serialize_frame_note<W: Write>(
    w: &mut MusicXmlWriter<W>,
    note: &FrameNote,
) -> SerializeResult<()> {
    let start = w.start_element("frame-note");
    w.write_start(start)?;

    // string
    let mut s_start = w.start_element("string");
    if let Some(ref p) = note.string.placement {
        s_start.push_attribute(("placement", above_below_str(p)));
    }
    w.write_start(s_start)?;
    w.write_text(&note.string.value.to_string())?;
    w.write_end("string")?;

    // fret
    w.write_text_element("fret", &note.fret.value.to_string())?;

    // fingering
    if let Some(ref fing) = note.fingering {
        let mut f_start = w.start_element("fingering");
        if let Some(ref sub) = fing.substitution {
            f_start.push_attribute(("substitution", yes_no_str(sub)));
        }
        if let Some(ref alt) = fing.alternate {
            f_start.push_attribute(("alternate", yes_no_str(alt)));
        }
        w.write_start(f_start)?;
        w.write_text(&fing.value)?;
        w.write_end("fingering")?;
    }

    // barre
    if let Some(ref barre) = note.barre {
        let mut b_start = w.start_element("barre");
        b_start.push_attribute(("type", start_stop_str(&barre.barre_type)));
        push_opt_str_attr_start(&mut b_start, "color", &barre.color);
        w.write_empty(b_start)?;
    }

    w.write_end("frame-note")?;
    Ok(())
}

// ============================================================================
// StyleText
// ============================================================================

fn serialize_style_text<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    st: &StyleText,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    push_opt_str_attr_start(&mut start, "font-family", &st.font_family);
    push_opt_str_attr_start(&mut start, "font-style", &st.font_style);
    push_opt_attr_start(&mut start, "font-size", &st.font_size);
    push_opt_str_attr_start(&mut start, "font-weight", &st.font_weight);
    push_opt_str_attr_start(&mut start, "color", &st.color);
    w.write_start(start)?;
    w.write_text(&st.value)?;
    w.write_end(name)?;
    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

fn harmony_type_str(t: &HarmonyType) -> &'static str {
    match t {
        HarmonyType::Explicit => "explicit",
        HarmonyType::Implied => "implied",
        HarmonyType::Alternate => "alternate",
    }
}

pub(crate) fn harmony_arrangement_str(a: &HarmonyArrangement) -> &'static str {
    match a {
        HarmonyArrangement::Vertical => "vertical",
        HarmonyArrangement::Horizontal => "horizontal",
        HarmonyArrangement::Diagonal => "diagonal",
    }
}

/// Format a decimal value: if integer, omit fractional part.
pub(crate) fn format_decimal(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{}", v as i64)
    } else {
        v.to_string()
    }
}

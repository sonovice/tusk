//! Serializer implementations for MusicXML note, attributes, and direction types.
//!
//! Contains `MusicXmlSerialize` implementations for:
//! - Note, Pitch, Rest, Unpitched, Tie, Stem, Beam, etc.
//! - Backup, Forward
//! - Attributes, Key, Time, Clef
//! - Direction, DirectionType, Wedge, Metronome

use std::io::Write;

use crate::model::*;
use crate::serializer::{
    MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr,
};

use super::score::{
    above_below_str, accidental_value_str, beam_value_str, clef_sign_str, fan_str, font_size_str,
    font_style_str, font_weight_str, left_center_right_str, mode_str, notehead_value_str,
    push_opt_attr_start, push_opt_str_attr_start, start_stop_continue_str, start_stop_str,
    stem_value_str, step_str, symbol_size_str, time_symbol_str, valign_str, wedge_type_str,
    yes_no_str,
};

// ============================================================================
// Note
// ============================================================================

impl MusicXmlSerialize for Note {
    fn element_name(&self) -> &'static str {
        "note"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-x", self.relative_x);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        if let Some(ref pl) = self.print_leger {
            attrs.push(("print-leger", yes_no_str(pl).to_string()));
        }
        if let Some(ref ps) = self.print_spacing {
            attrs.push(("print-spacing", yes_no_str(ps).to_string()));
        }
        push_opt_attr!(attrs, "dynamics", self.dynamics);
        push_opt_attr!(attrs, "end-dynamics", self.end_dynamics);
        push_opt_attr!(attrs, "attack", self.attack);
        push_opt_attr!(attrs, "release", self.release);
        if let Some(ref piz) = self.pizzicato {
            attrs.push(("pizzicato", yes_no_str(piz).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Grace
        if let Some(ref grace) = self.grace {
            grace.serialize(w)?;
        }

        // Cue
        if self.cue.is_some() {
            w.write_empty(w.start_element("cue"))?;
        }

        // Chord
        if self.chord.is_some() {
            w.write_empty(w.start_element("chord"))?;
        }

        // Pitch/Unpitched/Rest
        match &self.content {
            FullNoteContent::Pitch(pitch) => pitch.serialize(w)?,
            FullNoteContent::Unpitched(unpitched) => unpitched.serialize(w)?,
            FullNoteContent::Rest(rest) => rest.serialize(w)?,
        }

        // Duration
        if let Some(dur) = self.duration {
            w.write_text_element("duration", &dur.to_string())?;
        }

        // Ties
        for tie in &self.ties {
            tie.serialize(w)?;
        }

        // Voice
        w.write_opt_text_element("voice", &self.voice)?;

        // Type
        if let Some(ref nt) = self.note_type {
            nt.serialize(w)?;
        }

        // Dots
        for dot in &self.dots {
            dot.serialize(w)?;
        }

        // Accidental
        if let Some(ref acc) = self.accidental {
            acc.serialize(w)?;
        }

        // Time modification
        if let Some(ref tm) = self.time_modification {
            tm.serialize(w)?;
        }

        // Stem
        if let Some(ref stem) = self.stem {
            stem.serialize(w)?;
        }

        // Notehead
        if let Some(ref nh) = self.notehead {
            nh.serialize(w)?;
        }

        // Staff
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }

        // Beams
        for beam in &self.beams {
            beam.serialize(w)?;
        }

        // Notations
        if let Some(ref notations) = self.notations {
            notations.serialize(w)?;
        }

        // Lyrics
        for lyric in &self.lyrics {
            lyric.serialize(w)?;
        }

        Ok(())
    }
}

impl MusicXmlSerialize for Grace {
    fn element_name(&self) -> &'static str {
        "grace"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "steal-time-previous", self.steal_time_previous);
        push_opt_attr!(attrs, "steal-time-following", self.steal_time_following);
        push_opt_attr!(attrs, "make-time", self.make_time);
        if let Some(ref s) = self.slash {
            attrs.push(("slash", yes_no_str(s).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Pitch {
    fn element_name(&self) -> &'static str {
        "pitch"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("step", step_str(&self.step))?;
        if let Some(alter) = self.alter {
            w.write_text_element("alter", &alter.to_string())?;
        }
        w.write_text_element("octave", &self.octave.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for Unpitched {
    fn element_name(&self) -> &'static str {
        "unpitched"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        self.display_step.is_some() || self.display_octave.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref step) = self.display_step {
            w.write_text_element("display-step", step_str(step))?;
        }
        if let Some(oct) = self.display_octave {
            w.write_text_element("display-octave", &oct.to_string())?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Rest {
    fn element_name(&self) -> &'static str {
        "rest"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref m) = self.measure {
            attrs.push(("measure", yes_no_str(m).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        self.display_step.is_some() || self.display_octave.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref step) = self.display_step {
            w.write_text_element("display-step", step_str(step))?;
        }
        if let Some(oct) = self.display_octave {
            w.write_text_element("display-octave", &oct.to_string())?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Tie {
    fn element_name(&self) -> &'static str {
        "tie"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", start_stop_str(&self.tie_type).to_string())];
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for NoteType {
    fn element_name(&self) -> &'static str {
        "type"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref size) = self.size {
            attrs.push(("size", symbol_size_str(size).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(&self.value.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for Dot {
    fn element_name(&self) -> &'static str {
        "dot"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-x", self.relative_x);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Accidental {
    fn element_name(&self) -> &'static str {
        "accidental"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref c) = self.cautionary {
            attrs.push(("cautionary", yes_no_str(c).to_string()));
        }
        if let Some(ref e) = self.editorial {
            attrs.push(("editorial", yes_no_str(e).to_string()));
        }
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        if let Some(ref b) = self.bracket {
            attrs.push(("bracket", yes_no_str(b).to_string()));
        }
        if let Some(ref s) = self.size {
            attrs.push(("size", symbol_size_str(s).to_string()));
        }
        push_opt_str_attr!(attrs, "smufl", self.smufl);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(accidental_value_str(&self.value))?;
        Ok(())
    }
}

impl MusicXmlSerialize for TimeModification {
    fn element_name(&self) -> &'static str {
        "time-modification"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("actual-notes", &self.actual_notes.to_string())?;
        w.write_text_element("normal-notes", &self.normal_notes.to_string())?;
        if let Some(ref nt) = self.normal_type {
            w.write_text_element("normal-type", &nt.to_string())?;
        }
        for _ in &self.normal_dots {
            w.write_empty(w.start_element("normal-dot"))?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Stem {
    fn element_name(&self) -> &'static str {
        "stem"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        push_opt_str_attr!(attrs, "color", self.color);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(stem_value_str(&self.value))?;
        Ok(())
    }
}

impl MusicXmlSerialize for Notehead {
    fn element_name(&self) -> &'static str {
        "notehead"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref f) = self.filled {
            attrs.push(("filled", yes_no_str(f).to_string()));
        }
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "font-family", self.font_family);
        if let Some(ref style) = self.font_style {
            attrs.push(("font-style", font_style_str(style).to_string()));
        }
        if let Some(ref size) = self.font_size {
            attrs.push(("font-size", font_size_str(size)));
        }
        if let Some(ref weight) = self.font_weight {
            attrs.push(("font-weight", font_weight_str(weight).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "smufl", self.smufl);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(notehead_value_str(&self.value))?;
        Ok(())
    }
}

impl MusicXmlSerialize for Beam {
    fn element_name(&self) -> &'static str {
        "beam"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref r) = self.repeater {
            attrs.push(("repeater", yes_no_str(r).to_string()));
        }
        if let Some(ref f) = self.fan {
            attrs.push(("fan", fan_str(f).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(beam_value_str(&self.value))?;
        Ok(())
    }
}

// ============================================================================
// Backup and Forward
// ============================================================================

impl MusicXmlSerialize for Backup {
    fn element_name(&self) -> &'static str {
        "backup"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("duration", &self.duration.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for Forward {
    fn element_name(&self) -> &'static str {
        "forward"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("duration", &self.duration.to_string())?;
        w.write_opt_text_element("voice", &self.voice)?;
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }
        Ok(())
    }
}

// ============================================================================
// Attributes
// ============================================================================

impl MusicXmlSerialize for Attributes {
    fn element_name(&self) -> &'static str {
        "attributes"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Divisions
        if let Some(div) = self.divisions {
            w.write_text_element("divisions", &div.to_string())?;
        }

        // Key
        for key in &self.keys {
            key.serialize(w)?;
        }

        // Time
        for time in &self.times {
            time.serialize(w)?;
        }

        // Staves
        if let Some(staves) = self.staves {
            w.write_text_element("staves", &staves.to_string())?;
        }

        // Clef
        for clef in &self.clefs {
            clef.serialize(w)?;
        }

        Ok(())
    }
}

impl MusicXmlSerialize for Key {
    fn element_name(&self) -> &'static str {
        "key"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            KeyContent::Traditional(trad) => {
                if let Some(ref cancel) = trad.cancel {
                    w.write_text_element("cancel", &cancel.fifths.to_string())?;
                }
                w.write_text_element("fifths", &trad.fifths.to_string())?;
                if let Some(ref mode) = trad.mode {
                    w.write_text_element("mode", mode_str(mode))?;
                }
            }
            KeyContent::NonTraditional(nt) => {
                for alt in &nt.alterations {
                    w.write_text_element("key-step", step_str(&alt.key_step))?;
                    w.write_text_element("key-alter", &alt.key_alter.to_string())?;
                }
            }
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Time {
    fn element_name(&self) -> &'static str {
        "time"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref sym) = self.symbol {
            attrs.push(("symbol", time_symbol_str(sym).to_string()));
        }
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            TimeContent::Standard(std) => {
                for sig in &std.signatures {
                    w.write_text_element("beats", &sig.beats)?;
                    w.write_text_element("beat-type", &sig.beat_type)?;
                }
            }
            TimeContent::SenzaMisura(sm) => {
                if let Some(ref symbol) = sm.symbol {
                    w.write_text_element("senza-misura", symbol)?;
                } else {
                    w.write_empty(w.start_element("senza-misura"))?;
                }
            }
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Clef {
    fn element_name(&self) -> &'static str {
        "clef"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("sign", clef_sign_str(&self.sign))?;
        if let Some(line) = self.line {
            w.write_text_element("line", &line.to_string())?;
        }
        if let Some(oct) = self.clef_octave_change {
            w.write_text_element("clef-octave-change", &oct.to_string())?;
        }
        Ok(())
    }
}

// ============================================================================
// Direction
// ============================================================================

impl MusicXmlSerialize for Direction {
    fn element_name(&self) -> &'static str {
        "direction"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Direction types
        for dt in &self.direction_types {
            dt.serialize(w)?;
        }

        // Offset
        if let Some(ref offset) = self.offset {
            let mut start = w.start_element("offset");
            if let Some(ref sound) = offset.sound {
                start.push_attribute(("sound", yes_no_str(sound)));
            }
            w.write_start(start)?;
            w.write_text(&offset.value.to_string())?;
            w.write_end("offset")?;
        }

        // Staff
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }

        // Sound (if present, needs further implementation)

        Ok(())
    }
}

impl MusicXmlSerialize for DirectionType {
    fn element_name(&self) -> &'static str {
        "direction-type"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            DirectionTypeContent::Dynamics(dynamics) => {
                let start = w.start_element("dynamics");
                w.write_start(start)?;
                for d in &dynamics.values {
                    serialize_dynamics_value(w, d)?;
                }
                w.write_end("dynamics")?;
            }
            DirectionTypeContent::Wedge(wedge) => {
                wedge.serialize(w)?;
            }
            DirectionTypeContent::Metronome(metronome) => {
                metronome.serialize(w)?;
            }
            DirectionTypeContent::Words(words) => {
                for word in words {
                    serialize_words(w, word)?;
                }
            }
            _ => {
                // TODO: implement other direction types (Rehearsal, Segno, Coda, etc.)
            }
        }
        Ok(())
    }
}

pub(crate) fn serialize_dynamics_value<W: Write>(
    w: &mut MusicXmlWriter<W>,
    value: &DynamicsValue,
) -> SerializeResult<()> {
    let name = match value {
        DynamicsValue::P => "p",
        DynamicsValue::Pp => "pp",
        DynamicsValue::Ppp => "ppp",
        DynamicsValue::Pppp => "pppp",
        DynamicsValue::Ppppp => "ppppp",
        DynamicsValue::Pppppp => "pppppp",
        DynamicsValue::F => "f",
        DynamicsValue::Ff => "ff",
        DynamicsValue::Fff => "fff",
        DynamicsValue::Ffff => "ffff",
        DynamicsValue::Fffff => "fffff",
        DynamicsValue::Ffffff => "ffffff",
        DynamicsValue::Mp => "mp",
        DynamicsValue::Mf => "mf",
        DynamicsValue::Sf => "sf",
        DynamicsValue::Sfp => "sfp",
        DynamicsValue::Sfpp => "sfpp",
        DynamicsValue::Fp => "fp",
        DynamicsValue::Pf => "pf",
        DynamicsValue::Rf => "rf",
        DynamicsValue::Rfz => "rfz",
        DynamicsValue::Sfz => "sfz",
        DynamicsValue::Sffz => "sffz",
        DynamicsValue::Fz => "fz",
        DynamicsValue::N => "n",
        DynamicsValue::Sfzp => "sfzp",
        DynamicsValue::OtherDynamics(s) => {
            let start = w.start_element("other-dynamics");
            w.write_start(start)?;
            w.write_text(s)?;
            w.write_end("other-dynamics")?;
            return Ok(());
        }
    };
    w.write_empty(w.start_element(name))?;
    Ok(())
}

fn serialize_words<W: Write>(w: &mut MusicXmlWriter<W>, words: &Words) -> SerializeResult<()> {
    let mut start = w.start_element("words");
    push_opt_str_attr_start(&mut start, "font-family", &words.font_family);
    if let Some(ref style) = words.font_style {
        start.push_attribute(("font-style", font_style_str(style)));
    }
    if let Some(ref size) = words.font_size {
        start.push_attribute(("font-size", font_size_str(size).as_str()));
    }
    if let Some(ref weight) = words.font_weight {
        start.push_attribute(("font-weight", font_weight_str(weight)));
    }
    push_opt_attr_start(&mut start, "default-x", &words.default_x);
    push_opt_attr_start(&mut start, "default-y", &words.default_y);
    if let Some(ref j) = words.justify {
        start.push_attribute(("justify", left_center_right_str(j)));
    }
    if let Some(ref h) = words.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = words.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    w.write_start(start)?;
    w.write_text(&words.value)?;
    w.write_end("words")?;
    Ok(())
}

impl MusicXmlSerialize for Wedge {
    fn element_name(&self) -> &'static str {
        "wedge"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", wedge_type_str(&self.wedge_type).to_string())];
        push_opt_attr!(attrs, "number", self.number);
        push_opt_attr!(attrs, "spread", self.spread);
        if let Some(ref niente) = self.niente {
            attrs.push(("niente", yes_no_str(niente).to_string()));
        }
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

impl MusicXmlSerialize for Metronome {
    fn element_name(&self) -> &'static str {
        "metronome"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            MetronomeContent::BeatUnit {
                beat_unit,
                beat_unit_dots,
                per_minute,
            } => {
                w.write_text_element("beat-unit", beat_unit)?;
                for _ in beat_unit_dots {
                    w.write_empty(w.start_element("beat-unit-dot"))?;
                }
                w.write_text_element("per-minute", per_minute)?;
            }
            MetronomeContent::BeatUnitEquivalent(modulation) => {
                w.write_text_element("beat-unit", &modulation.beat_unit_1)?;
                for _ in &modulation.beat_unit_dots_1 {
                    w.write_empty(w.start_element("beat-unit-dot"))?;
                }
                w.write_text_element("beat-unit", &modulation.beat_unit_2)?;
                for _ in &modulation.beat_unit_dots_2 {
                    w.write_empty(w.start_element("beat-unit-dot"))?;
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Lyric
// ============================================================================

fn syllabic_str(s: &Syllabic) -> &'static str {
    match s {
        Syllabic::Single => "single",
        Syllabic::Begin => "begin",
        Syllabic::Middle => "middle",
        Syllabic::End => "end",
    }
}

fn serialize_lyric_text<W: Write>(
    w: &mut MusicXmlWriter<W>,
    text: &LyricText,
) -> SerializeResult<()> {
    let mut start = w.start_element("text");
    push_opt_str_attr_start(&mut start, "font-family", &text.font_family);
    if let Some(ref fs) = text.font_size {
        start.push_attribute(("font-size", font_size_str(fs).as_str()));
    }
    if let Some(ref fs) = text.font_style {
        start.push_attribute(("font-style", font_style_str(fs)));
    }
    if let Some(ref fw) = text.font_weight {
        start.push_attribute(("font-weight", font_weight_str(fw)));
    }
    push_opt_str_attr_start(&mut start, "color", &text.color);
    w.write_start(start)?;
    w.write_text(&text.value)?;
    w.write_end("text")?;
    Ok(())
}

fn serialize_elision<W: Write>(
    w: &mut MusicXmlWriter<W>,
    elision: &Elision,
) -> SerializeResult<()> {
    let mut start = w.start_element("elision");
    push_opt_str_attr_start(&mut start, "font-family", &elision.font_family);
    if let Some(ref fs) = elision.font_size {
        start.push_attribute(("font-size", font_size_str(fs).as_str()));
    }
    if let Some(ref fs) = elision.font_style {
        start.push_attribute(("font-style", font_style_str(fs)));
    }
    if let Some(ref fw) = elision.font_weight {
        start.push_attribute(("font-weight", font_weight_str(fw)));
    }
    push_opt_str_attr_start(&mut start, "color", &elision.color);
    w.write_start(start)?;
    w.write_text(&elision.value)?;
    w.write_end("elision")?;
    Ok(())
}

fn serialize_extend<W: Write>(w: &mut MusicXmlWriter<W>, extend: &Extend) -> SerializeResult<()> {
    let mut start = w.start_element("extend");
    if let Some(ref t) = extend.extend_type {
        start.push_attribute(("type", start_stop_continue_str(t)));
    }
    w.write_empty(start)?;
    Ok(())
}

impl MusicXmlSerialize for Lyric {
    fn element_name(&self) -> &'static str {
        "lyric"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_str_attr!(attrs, "number", self.number);
        push_opt_str_attr!(attrs, "name", self.name);
        if let Some(ref j) = self.justify {
            attrs.push(("justify", left_center_right_str(j).to_string()));
        }
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-x", self.relative_x);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            LyricContent::Text {
                syllable_groups,
                extend,
            } => {
                for group in syllable_groups {
                    if let Some(ref elision) = group.elision {
                        serialize_elision(w, elision)?;
                    }
                    if let Some(ref syl) = group.syllabic {
                        w.write_text_element("syllabic", syllabic_str(syl))?;
                    }
                    serialize_lyric_text(w, &group.text)?;
                }
                if let Some(ext) = extend {
                    serialize_extend(w, ext)?;
                }
            }
            LyricContent::ExtendOnly(ext) => {
                serialize_extend(w, ext)?;
            }
            LyricContent::Laughing => {
                w.write_empty(w.start_element("laughing"))?;
            }
            LyricContent::Humming => {
                w.write_empty(w.start_element("humming"))?;
            }
        }
        if self.end_line {
            w.write_empty(w.start_element("end-line"))?;
        }
        if self.end_paragraph {
            w.write_empty(w.start_element("end-paragraph"))?;
        }
        Ok(())
    }
}

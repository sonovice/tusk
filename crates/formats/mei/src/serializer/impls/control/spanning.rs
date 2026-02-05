//! Serializer implementations for spanning/continuation elements:
//! BeamSpan, Octave, Gliss, Lv, BracketSpan, BTrem, FTrem.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBTremAnl, AttBTremGes, AttBTremLog, AttBTremVis, AttBeamSpanAnl, AttBeamSpanGes,
    AttBeamSpanLog, AttBeamSpanVis, AttBracketSpanAnl, AttBracketSpanGes, AttBracketSpanLog,
    AttBracketSpanVis, AttFTremAnl, AttFTremGes, AttFTremLog, AttFTremVis, AttGlissAnl,
    AttGlissGes, AttGlissLog, AttGlissVis, AttLvAnl, AttLvGes, AttLvLog, AttLvVis, AttOctaveAnl,
    AttOctaveGes, AttOctaveLog, AttOctaveVis,
};
use tusk_model::elements::{
    BTrem, BTremChild, BeamSpan, BracketSpan, FTrem, FTremChild, Gliss, Lv, Octave,
};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// BeamSpan attribute class implementations
// ============================================================================

impl CollectAttributes for AttBeamSpanLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "beam.with", self.beam_with);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttBeamSpanVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "slash", self.slash);
        if let Some(v) = &self.slope {
            attrs.push(("slope", v.to_string()));
        }
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}

impl CollectAttributes for AttBeamSpanGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttBeamSpanAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for BeamSpan {
    fn element_name(&self) -> &'static str {
        "beamSpan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.beam_span_log.collect_attributes());
        attrs.extend(self.beam_span_vis.collect_attributes());
        attrs.extend(self.beam_span_ges.collect_attributes());
        attrs.extend(self.beam_span_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // BeamSpan is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Octave attribute class implementations
// ============================================================================

impl CollectAttributes for AttOctaveLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "dis", self.dis);
        push_attr!(attrs, "dis.place", self.dis_place);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "coll", self.coll);
        attrs
    }
}

impl CollectAttributes for AttOctaveVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttOctaveGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttOctaveAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Octave {
    fn element_name(&self) -> &'static str {
        "octave"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.octave_log.collect_attributes());
        attrs.extend(self.octave_vis.collect_attributes());
        attrs.extend(self.octave_ges.collect_attributes());
        attrs.extend(self.octave_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children are not serialized for now
        Ok(())
    }
}

// ============================================================================
// Gliss attribute class implementations
// ============================================================================

impl CollectAttributes for AttGlissLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttGlissVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        if let Some(v) = &self.x2 {
            attrs.push(("x2", v.to_string()));
        }
        if let Some(v) = &self.y2 {
            attrs.push(("y2", v.to_string()));
        }
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        attrs
    }
}

impl CollectAttributes for AttGlissGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttGlissAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Gliss {
    fn element_name(&self) -> &'static str {
        "gliss"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.gliss_log.collect_attributes());
        attrs.extend(self.gliss_vis.collect_attributes());
        attrs.extend(self.gliss_ges.collect_attributes());
        attrs.extend(self.gliss_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children are not serialized for now
        Ok(())
    }
}

// ============================================================================
// Lv attribute class implementations
// ============================================================================

impl CollectAttributes for AttLvLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttLvVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        if let Some(v) = &self.x2 {
            attrs.push(("x2", v.to_string()));
        }
        if let Some(v) = &self.y2 {
            attrs.push(("y2", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttLvGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttLvAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Lv {
    fn element_name(&self) -> &'static str {
        "lv"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lv_log.collect_attributes());
        attrs.extend(self.lv_vis.collect_attributes());
        attrs.extend(self.lv_ges.collect_attributes());
        attrs.extend(self.lv_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children (Curve) are not serialized for now
        Ok(())
    }
}

// ============================================================================
// BracketSpan attribute class implementations
// ============================================================================

impl CollectAttributes for AttBracketSpanLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttBracketSpanVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        if let Some(v) = &self.x2 {
            attrs.push(("x2", v.to_string()));
        }
        if let Some(v) = &self.y2 {
            attrs.push(("y2", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttBracketSpanGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttBracketSpanAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for BracketSpan {
    fn element_name(&self) -> &'static str {
        "bracketSpan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.bracket_span_log.collect_attributes());
        attrs.extend(self.bracket_span_vis.collect_attributes());
        attrs.extend(self.bracket_span_ges.collect_attributes());
        attrs.extend(self.bracket_span_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children are not serialized for now
        Ok(())
    }
}

// ============================================================================
// BTrem (bowed tremolo) attribute class implementations
// ============================================================================

impl CollectAttributes for AttBTremLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "dur", self.dur);
        if let Some(v) = &self.num {
            attrs.push(("num", v.to_string()));
        }
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttBTremVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        attrs
    }
}

impl CollectAttributes for AttBTremGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unitdur", self.unitdur);
        attrs
    }
}

impl CollectAttributes for AttBTremAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for BTrem {
    fn element_name(&self) -> &'static str {
        "bTrem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.b_trem_log.collect_attributes());
        attrs.extend(self.b_trem_vis.collect_attributes());
        attrs.extend(self.b_trem_ges.collect_attributes());
        attrs.extend(self.b_trem_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for BTremChild {
    fn element_name(&self) -> &'static str {
        match self {
            BTremChild::Note(_) => "note",
            BTremChild::Chord(_) => "chord",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            BTremChild::Note(elem) => elem.collect_all_attributes(),
            BTremChild::Chord(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            BTremChild::Note(elem) => elem.has_children(),
            BTremChild::Chord(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BTremChild::Note(elem) => elem.serialize_children(writer),
            BTremChild::Chord(elem) => elem.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BTremChild::Note(elem) => elem.serialize_mei(writer),
            BTremChild::Chord(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// FTrem (fingered tremolo) attribute class implementations
// ============================================================================

impl CollectAttributes for AttFTremLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttFTremVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(v) = &self.beams {
            attrs.push(("beams", v.to_string()));
        }
        if let Some(v) = &self.beams_float {
            attrs.push(("beams.float", v.to_string()));
        }
        push_attr!(attrs, "float.gap", self.float_gap);
        attrs
    }
}

impl CollectAttributes for AttFTremGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unitdur", self.unitdur);
        attrs
    }
}

impl CollectAttributes for AttFTremAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for FTrem {
    fn element_name(&self) -> &'static str {
        "fTrem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.f_trem_log.collect_attributes());
        attrs.extend(self.f_trem_vis.collect_attributes());
        attrs.extend(self.f_trem_ges.collect_attributes());
        attrs.extend(self.f_trem_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for FTremChild {
    fn element_name(&self) -> &'static str {
        match self {
            FTremChild::Note(_) => "note",
            FTremChild::Chord(_) => "chord",
            FTremChild::Clef(_) => "clef",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FTremChild::Note(elem) => elem.collect_all_attributes(),
            FTremChild::Chord(elem) => elem.collect_all_attributes(),
            FTremChild::Clef(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FTremChild::Note(elem) => elem.has_children(),
            FTremChild::Chord(elem) => elem.has_children(),
            FTremChild::Clef(_) => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FTremChild::Note(elem) => elem.serialize_children(writer),
            FTremChild::Chord(elem) => elem.serialize_children(writer),
            FTremChild::Clef(_) => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FTremChild::Note(elem) => elem.serialize_mei(writer),
            FTremChild::Chord(elem) => elem.serialize_mei(writer),
            FTremChild::Clef(elem) => elem.serialize_mei(writer),
        }
    }
}

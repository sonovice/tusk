//! Serializer implementations for control event MEI elements.
//!
//! This module contains implementations for Slur, Tie, Dynam, Hairpin,
//! Dir, Tempo, and Fermata elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAnchoredTextAnl, AttAnchoredTextGes, AttAnchoredTextLog, AttAnchoredTextVis,
};
use tusk_model::att::{
    AttBibl, AttDirAnl, AttDirGes, AttDirLog, AttDirVis, AttDynamAnl, AttDynamGes, AttDynamLog,
    AttDynamVis, AttFAnl, AttFGes, AttFLog, AttFVis, AttFermataAnl, AttFermataGes, AttFermataLog,
    AttFermataVis, AttHairpinAnl, AttHairpinGes, AttHairpinLog, AttHairpinVis, AttHarmAnl,
    AttHarmGes, AttHarmLog, AttHarmVis, AttLang, AttPedalAnl, AttPedalGes, AttPedalLog,
    AttPedalVis, AttSlurAnl, AttSlurGes, AttSlurLog, AttSlurVis, AttSymbolAnl, AttSymbolGes,
    AttSymbolLog, AttSymbolVis, AttTempoAnl, AttTempoGes, AttTempoLog, AttTempoVis, AttTieAnl,
    AttTieGes, AttTieLog, AttTieVis, AttTrillAnl, AttTrillGes, AttTrillLog, AttTrillVis,
};
use tusk_model::elements::{
    AnchoredText, AnchoredTextChild, Dir, DirChild, Dynam, DynamChild, F, FChild, Fb, FbChild,
    Fermata, Hairpin, Harm, HarmChild, Pedal, Slur, SlurChild, Symbol, Tempo, TempoChild, Tie,
    TieChild, Trill,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Control event attribute class implementations
// ============================================================================

impl CollectAttributes for AttLang {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:lang", clone self.xml_lang);
        push_attr!(attrs, "translit", clone self.translit);
        attrs
    }
}

impl CollectAttributes for AttBibl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "analog", clone self.analog);
        attrs
    }
}

// ============================================================================
// Slur attribute class implementations
// ============================================================================

impl CollectAttributes for AttSlurLog {
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

impl CollectAttributes for AttSlurVis {
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

impl CollectAttributes for AttSlurGes {
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

impl CollectAttributes for AttSlurAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}

// ============================================================================
// Tie attribute class implementations
// ============================================================================

impl CollectAttributes for AttTieLog {
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

impl CollectAttributes for AttTieVis {
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

impl CollectAttributes for AttTieGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttTieAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Dynam attribute class implementations
// ============================================================================

impl CollectAttributes for AttDynamLog {
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

impl CollectAttributes for AttDynamVis {
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
        push_attr!(attrs, "place", self.place);
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

impl CollectAttributes for AttDynamGes {
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
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttDynamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Hairpin attribute class implementations
// ============================================================================

impl CollectAttributes for AttHairpinLog {
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
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "niente", self.niente);
        attrs
    }
}

impl CollectAttributes for AttHairpinVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "place", self.place);
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
        push_attr!(attrs, "opening", self.opening);
        push_attr!(attrs, "closed", self.closed);
        push_attr!(attrs, "opening.vertical", self.opening_vertical);
        push_attr!(attrs, "angle.optimize", self.angle_optimize);
        attrs
    }
}

impl CollectAttributes for AttHairpinGes {
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
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttHairpinAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Dir attribute class implementations
// ============================================================================

impl CollectAttributes for AttDirLog {
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

impl CollectAttributes for AttDirVis {
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
        push_attr!(attrs, "place", self.place);
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

impl CollectAttributes for AttDirGes {
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

impl CollectAttributes for AttDirAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Tempo attribute class implementations
// ============================================================================

impl CollectAttributes for AttTempoLog {
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
        push_attr!(attrs, "mm", self.mm);
        push_attr!(attrs, "mm.unit", self.mm_unit);
        push_attr!(attrs, "mm.dots", self.mm_dots);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttTempoVis {
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
        push_attr!(attrs, "place", self.place);
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

impl CollectAttributes for AttTempoGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.bpm", self.midi_bpm);
        push_attr!(attrs, "midi.mspb", self.midi_mspb);
        attrs
    }
}

impl CollectAttributes for AttTempoAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Fermata attribute class implementations
// ============================================================================

impl CollectAttributes for AttFermataLog {
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
        attrs
    }
}

impl CollectAttributes for AttFermataVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "shape", self.shape);
        attrs
    }
}

impl CollectAttributes for AttFermataGes {
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
        attrs
    }
}

impl CollectAttributes for AttFermataAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Control event element implementations
// ============================================================================

impl MeiSerialize for Slur {
    fn element_name(&self) -> &'static str {
        "slur"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.slur_log.collect_attributes());
        attrs.extend(self.slur_vis.collect_attributes());
        attrs.extend(self.slur_ges.collect_attributes());
        attrs.extend(self.slur_anl.collect_attributes());
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

impl MeiSerialize for SlurChild {
    fn element_name(&self) -> &'static str {
        match self {
            SlurChild::Curve(_) => "curve",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SlurChild::Curve(_) => Vec::new(), // Curve serialization not yet implemented
        }
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Tie {
    fn element_name(&self) -> &'static str {
        "tie"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.tie_log.collect_attributes());
        attrs.extend(self.tie_vis.collect_attributes());
        attrs.extend(self.tie_ges.collect_attributes());
        attrs.extend(self.tie_anl.collect_attributes());
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

impl MeiSerialize for TieChild {
    fn element_name(&self) -> &'static str {
        match self {
            TieChild::Curve(_) => "curve",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TieChild::Curve(_) => Vec::new(), // Curve serialization not yet implemented
        }
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Dynam {
    fn element_name(&self) -> &'static str {
        "dynam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.dynam_log.collect_attributes());
        attrs.extend(self.dynam_vis.collect_attributes());
        attrs.extend(self.dynam_ges.collect_attributes());
        attrs.extend(self.dynam_anl.collect_attributes());
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

impl MeiSerialize for DynamChild {
    fn element_name(&self) -> &'static str {
        match self {
            DynamChild::Text(_) => "$text",
            DynamChild::Rend(_) => "rend",
            DynamChild::Lb(_) => "lb",
            DynamChild::Ref(_) => "ref",
            DynamChild::PersName(_) => "persName",
            DynamChild::CorpName(_) => "corpName",
            DynamChild::Name(_) => "name",
            DynamChild::Seg(_) => "seg",
            DynamChild::Date(_) => "date",
            DynamChild::Identifier(_) => "identifier",
            DynamChild::Num(_) => "num",
            DynamChild::Ptr(_) => "ptr",
            DynamChild::Annot(_) => "annot",
            DynamChild::Title(_) => "title",
            // Other variants - return element name for error messages
            DynamChild::Stamp(_) => "stamp",
            DynamChild::Street(_) => "street",
            DynamChild::Gap(_) => "gap",
            DynamChild::Abbr(_) => "abbr",
            DynamChild::Sic(_) => "sic",
            DynamChild::PostBox(_) => "postBox",
            DynamChild::Q(_) => "q",
            DynamChild::Term(_) => "term",
            DynamChild::Corr(_) => "corr",
            DynamChild::PeriodName(_) => "periodName",
            DynamChild::BiblStruct(_) => "biblStruct",
            DynamChild::Signatures(_) => "signatures",
            DynamChild::Stack(_) => "stack",
            DynamChild::Unclear(_) => "unclear",
            DynamChild::Settlement(_) => "settlement",
            DynamChild::Depth(_) => "depth",
            DynamChild::Restore(_) => "restore",
            DynamChild::Dimensions(_) => "dimensions",
            DynamChild::PostCode(_) => "postCode",
            DynamChild::Damage(_) => "damage",
            DynamChild::Heraldry(_) => "heraldry",
            DynamChild::RelationList(_) => "relationList",
            DynamChild::Bloc(_) => "bloc",
            DynamChild::StyleName(_) => "styleName",
            DynamChild::Reg(_) => "reg",
            DynamChild::HandShift(_) => "handShift",
            DynamChild::Catchwords(_) => "catchwords",
            DynamChild::Country(_) => "country",
            DynamChild::Add(_) => "add",
            DynamChild::Bibl(_) => "bibl",
            DynamChild::LocusGrp(_) => "locusGrp",
            DynamChild::GeogFeat(_) => "geogFeat",
            DynamChild::Orig(_) => "orig",
            DynamChild::Height(_) => "height",
            DynamChild::Locus(_) => "locus",
            DynamChild::District(_) => "district",
            DynamChild::Expan(_) => "expan",
            DynamChild::GeogName(_) => "geogName",
            DynamChild::Relation(_) => "relation",
            DynamChild::Repository(_) => "repository",
            DynamChild::Del(_) => "del",
            DynamChild::Extent(_) => "extent",
            DynamChild::Width(_) => "width",
            DynamChild::Region(_) => "region",
            DynamChild::Symbol(_) => "symbol",
            DynamChild::Subst(_) => "subst",
            DynamChild::Supplied(_) => "supplied",
            DynamChild::Fig(_) => "fig",
            DynamChild::SecFolio(_) => "secFolio",
            DynamChild::Dim(_) => "dim",
            DynamChild::Address(_) => "address",
            DynamChild::Choice(_) => "choice",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            DynamChild::Text(_) => Vec::new(),
            DynamChild::Rend(elem) => elem.collect_all_attributes(),
            DynamChild::Lb(elem) => elem.collect_all_attributes(),
            DynamChild::Ref(elem) => elem.collect_all_attributes(),
            DynamChild::PersName(elem) => elem.collect_all_attributes(),
            DynamChild::CorpName(elem) => elem.collect_all_attributes(),
            DynamChild::Name(elem) => elem.collect_all_attributes(),
            DynamChild::Seg(elem) => elem.collect_all_attributes(),
            DynamChild::Date(elem) => elem.collect_all_attributes(),
            DynamChild::Identifier(elem) => elem.collect_all_attributes(),
            DynamChild::Num(elem) => elem.collect_all_attributes(),
            DynamChild::Ptr(elem) => elem.collect_all_attributes(),
            DynamChild::Annot(elem) => elem.collect_all_attributes(),
            DynamChild::Title(elem) => elem.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            DynamChild::Text(_) => false,
            DynamChild::Rend(elem) => elem.has_children(),
            DynamChild::Lb(_) => false,
            DynamChild::Ref(elem) => elem.has_children(),
            DynamChild::PersName(elem) => elem.has_children(),
            DynamChild::CorpName(elem) => elem.has_children(),
            DynamChild::Name(elem) => elem.has_children(),
            DynamChild::Seg(elem) => elem.has_children(),
            DynamChild::Date(elem) => elem.has_children(),
            DynamChild::Identifier(elem) => elem.has_children(),
            DynamChild::Num(elem) => elem.has_children(),
            DynamChild::Ptr(_) => false,
            DynamChild::Annot(elem) => elem.has_children(),
            DynamChild::Title(elem) => elem.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DynamChild::Text(_) => Ok(()),
            DynamChild::Rend(elem) => elem.serialize_children(writer),
            DynamChild::Lb(_) => Ok(()),
            DynamChild::Ref(elem) => elem.serialize_children(writer),
            DynamChild::PersName(elem) => elem.serialize_children(writer),
            DynamChild::CorpName(elem) => elem.serialize_children(writer),
            DynamChild::Name(elem) => elem.serialize_children(writer),
            DynamChild::Seg(elem) => elem.serialize_children(writer),
            DynamChild::Date(elem) => elem.serialize_children(writer),
            DynamChild::Identifier(elem) => elem.serialize_children(writer),
            DynamChild::Num(elem) => elem.serialize_children(writer),
            DynamChild::Ptr(_) => Ok(()),
            DynamChild::Annot(elem) => elem.serialize_children(writer),
            DynamChild::Title(elem) => elem.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DynamChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DynamChild::Rend(elem) => elem.serialize_mei(writer),
            DynamChild::Lb(elem) => elem.serialize_mei(writer),
            DynamChild::Ref(elem) => elem.serialize_mei(writer),
            DynamChild::PersName(elem) => elem.serialize_mei(writer),
            DynamChild::CorpName(elem) => elem.serialize_mei(writer),
            DynamChild::Name(elem) => elem.serialize_mei(writer),
            DynamChild::Seg(elem) => elem.serialize_mei(writer),
            DynamChild::Date(elem) => elem.serialize_mei(writer),
            DynamChild::Identifier(elem) => elem.serialize_mei(writer),
            DynamChild::Num(elem) => elem.serialize_mei(writer),
            DynamChild::Ptr(elem) => elem.serialize_mei(writer),
            DynamChild::Annot(elem) => elem.serialize_mei(writer),
            DynamChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DynamChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Hairpin {
    fn element_name(&self) -> &'static str {
        "hairpin"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hairpin_log.collect_attributes());
        attrs.extend(self.hairpin_vis.collect_attributes());
        attrs.extend(self.hairpin_ges.collect_attributes());
        attrs.extend(self.hairpin_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Hairpin is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Dir {
    fn element_name(&self) -> &'static str {
        "dir"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.dir_log.collect_attributes());
        attrs.extend(self.dir_vis.collect_attributes());
        attrs.extend(self.dir_ges.collect_attributes());
        attrs.extend(self.dir_anl.collect_attributes());
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

impl MeiSerialize for DirChild {
    fn element_name(&self) -> &'static str {
        match self {
            DirChild::Text(_) => "$text",
            DirChild::Rend(_) => "rend",
            DirChild::Lb(_) => "lb",
            DirChild::Ref(_) => "ref",
            DirChild::PersName(_) => "persName",
            DirChild::CorpName(_) => "corpName",
            DirChild::Name(_) => "name",
            DirChild::Date(_) => "date",
            DirChild::Title(_) => "title",
            DirChild::Identifier(_) => "identifier",
            DirChild::Num(_) => "num",
            DirChild::Ptr(_) => "ptr",
            DirChild::Annot(_) => "annot",
            // Other variants - return element name for error messages
            DirChild::Stack(_) => "stack",
            DirChild::RelationList(_) => "relationList",
            DirChild::Locus(_) => "locus",
            DirChild::Width(_) => "width",
            DirChild::Orig(_) => "orig",
            DirChild::Address(_) => "address",
            DirChild::Curve(_) => "curve",
            DirChild::Restore(_) => "restore",
            DirChild::Relation(_) => "relation",
            DirChild::Term(_) => "term",
            DirChild::Choice(_) => "choice",
            DirChild::PostBox(_) => "postBox",
            DirChild::Corr(_) => "corr",
            DirChild::GeogName(_) => "geogName",
            DirChild::Add(_) => "add",
            DirChild::Bloc(_) => "bloc",
            DirChild::AnchoredText(_) => "anchoredText",
            DirChild::Bibl(_) => "bibl",
            DirChild::Sic(_) => "sic",
            DirChild::BiblStruct(_) => "biblStruct",
            DirChild::Symbol(_) => "symbol",
            DirChild::Dim(_) => "dim",
            DirChild::Reg(_) => "reg",
            DirChild::PeriodName(_) => "periodName",
            DirChild::Subst(_) => "subst",
            DirChild::Unclear(_) => "unclear",
            DirChild::Height(_) => "height",
            DirChild::Street(_) => "street",
            DirChild::Stamp(_) => "stamp",
            DirChild::LocusGrp(_) => "locusGrp",
            DirChild::Del(_) => "del",
            DirChild::HandShift(_) => "handShift",
            DirChild::Depth(_) => "depth",
            DirChild::Heraldry(_) => "heraldry",
            DirChild::PostCode(_) => "postCode",
            DirChild::Catchwords(_) => "catchwords",
            DirChild::Line(_) => "line",
            DirChild::Region(_) => "region",
            DirChild::District(_) => "district",
            DirChild::Extent(_) => "extent",
            DirChild::Abbr(_) => "abbr",
            DirChild::Expan(_) => "expan",
            DirChild::SecFolio(_) => "secFolio",
            DirChild::Fig(_) => "fig",
            DirChild::GeogFeat(_) => "geogFeat",
            DirChild::Q(_) => "q",
            DirChild::Seg(_) => "seg",
            DirChild::Gap(_) => "gap",
            DirChild::StyleName(_) => "styleName",
            DirChild::Dimensions(_) => "dimensions",
            DirChild::Country(_) => "country",
            DirChild::Repository(_) => "repository",
            DirChild::Signatures(_) => "signatures",
            DirChild::Supplied(_) => "supplied",
            DirChild::Settlement(_) => "settlement",
            DirChild::Damage(_) => "damage",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            DirChild::Text(_) => Vec::new(),
            DirChild::Rend(elem) => elem.collect_all_attributes(),
            DirChild::Lb(elem) => elem.collect_all_attributes(),
            DirChild::Ref(elem) => elem.collect_all_attributes(),
            DirChild::PersName(elem) => elem.collect_all_attributes(),
            DirChild::CorpName(elem) => elem.collect_all_attributes(),
            DirChild::Name(elem) => elem.collect_all_attributes(),
            DirChild::Date(elem) => elem.collect_all_attributes(),
            DirChild::Title(elem) => elem.collect_all_attributes(),
            DirChild::Identifier(elem) => elem.collect_all_attributes(),
            DirChild::Num(elem) => elem.collect_all_attributes(),
            DirChild::Ptr(elem) => elem.collect_all_attributes(),
            DirChild::Annot(elem) => elem.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            DirChild::Text(_) => false,
            DirChild::Rend(elem) => elem.has_children(),
            DirChild::Lb(_) => false,
            DirChild::Ref(elem) => elem.has_children(),
            DirChild::PersName(elem) => elem.has_children(),
            DirChild::CorpName(elem) => elem.has_children(),
            DirChild::Name(elem) => elem.has_children(),
            DirChild::Date(elem) => elem.has_children(),
            DirChild::Title(elem) => elem.has_children(),
            DirChild::Identifier(elem) => elem.has_children(),
            DirChild::Num(elem) => elem.has_children(),
            DirChild::Ptr(_) => false,
            DirChild::Annot(elem) => elem.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DirChild::Text(_) => Ok(()),
            DirChild::Rend(elem) => elem.serialize_children(writer),
            DirChild::Lb(_) => Ok(()),
            DirChild::Ref(elem) => elem.serialize_children(writer),
            DirChild::PersName(elem) => elem.serialize_children(writer),
            DirChild::CorpName(elem) => elem.serialize_children(writer),
            DirChild::Name(elem) => elem.serialize_children(writer),
            DirChild::Date(elem) => elem.serialize_children(writer),
            DirChild::Title(elem) => elem.serialize_children(writer),
            DirChild::Identifier(elem) => elem.serialize_children(writer),
            DirChild::Num(elem) => elem.serialize_children(writer),
            DirChild::Ptr(_) => Ok(()),
            DirChild::Annot(elem) => elem.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DirChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DirChild::Rend(elem) => elem.serialize_mei(writer),
            DirChild::Lb(elem) => elem.serialize_mei(writer),
            DirChild::Ref(elem) => elem.serialize_mei(writer),
            DirChild::PersName(elem) => elem.serialize_mei(writer),
            DirChild::CorpName(elem) => elem.serialize_mei(writer),
            DirChild::Name(elem) => elem.serialize_mei(writer),
            DirChild::Date(elem) => elem.serialize_mei(writer),
            DirChild::Title(elem) => elem.serialize_mei(writer),
            DirChild::Identifier(elem) => elem.serialize_mei(writer),
            DirChild::Num(elem) => elem.serialize_mei(writer),
            DirChild::Ptr(elem) => elem.serialize_mei(writer),
            DirChild::Annot(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DirChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Tempo {
    fn element_name(&self) -> &'static str {
        "tempo"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.tempo_log.collect_attributes());
        attrs.extend(self.tempo_vis.collect_attributes());
        attrs.extend(self.tempo_ges.collect_attributes());
        attrs.extend(self.tempo_anl.collect_attributes());
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

impl MeiSerialize for TempoChild {
    fn element_name(&self) -> &'static str {
        match self {
            TempoChild::Text(_) => "$text",
            _ => "unknown", // Other children not yet fully implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TempoChild::Text(text) => {
                writer.write_text(text)?;
            }
            _ => {
                // Other children not yet fully implemented
            }
        }
        Ok(())
    }
}

impl MeiSerialize for Fermata {
    fn element_name(&self) -> &'static str {
        "fermata"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.fermata_log.collect_attributes());
        attrs.extend(self.fermata_vis.collect_attributes());
        attrs.extend(self.fermata_ges.collect_attributes());
        attrs.extend(self.fermata_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Fermata is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Trill attribute class implementations
// ============================================================================

impl CollectAttributes for AttTrillLog {
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
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttTrillVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
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
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
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

impl CollectAttributes for AttTrillGes {
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
        attrs
    }
}

impl CollectAttributes for AttTrillAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Trill {
    fn element_name(&self) -> &'static str {
        "trill"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.trill_log.collect_attributes());
        attrs.extend(self.trill_vis.collect_attributes());
        attrs.extend(self.trill_ges.collect_attributes());
        attrs.extend(self.trill_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Trill is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// AnchoredText attribute class implementations
// ============================================================================

impl CollectAttributes for AttAnchoredTextLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttAnchoredTextVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttAnchoredTextGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttAnchoredTextGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttAnchoredTextAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttAnchoredTextAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for AnchoredText {
    fn element_name(&self) -> &'static str {
        "anchoredText"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.anchored_text_log.collect_attributes());
        attrs.extend(self.anchored_text_vis.collect_attributes());
        attrs.extend(self.anchored_text_ges.collect_attributes());
        attrs.extend(self.anchored_text_anl.collect_attributes());
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

impl MeiSerialize for AnchoredTextChild {
    fn element_name(&self) -> &'static str {
        match self {
            AnchoredTextChild::Text(_) => "#text",
            AnchoredTextChild::Rend(_) => "rend",
            AnchoredTextChild::Lb(_) => "lb",
            AnchoredTextChild::Ref(_) => "ref",
            AnchoredTextChild::PersName(_) => "persName",
            AnchoredTextChild::CorpName(_) => "corpName",
            AnchoredTextChild::Name(_) => "name",
            AnchoredTextChild::Seg(_) => "seg",
            AnchoredTextChild::Title(_) => "title",
            AnchoredTextChild::Identifier(_) => "identifier",
            AnchoredTextChild::Date(_) => "date",
            AnchoredTextChild::Ptr(_) => "ptr",
            // Many other child types exist but are not commonly used
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            AnchoredTextChild::Rend(r) => r.collect_all_attributes(),
            AnchoredTextChild::Lb(lb) => lb.collect_all_attributes(),
            AnchoredTextChild::Ref(r) => r.collect_all_attributes(),
            AnchoredTextChild::PersName(pn) => pn.collect_all_attributes(),
            AnchoredTextChild::CorpName(cn) => cn.collect_all_attributes(),
            AnchoredTextChild::Name(n) => n.collect_all_attributes(),
            AnchoredTextChild::Seg(s) => s.collect_all_attributes(),
            AnchoredTextChild::Title(t) => t.collect_all_attributes(),
            AnchoredTextChild::Identifier(i) => i.collect_all_attributes(),
            AnchoredTextChild::Date(d) => d.collect_all_attributes(),
            AnchoredTextChild::Ptr(p) => p.collect_all_attributes(),
            // Text and other elements - no attributes
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            AnchoredTextChild::Text(_) => false,
            AnchoredTextChild::Rend(r) => !r.children.is_empty(),
            AnchoredTextChild::Lb(_) => false,
            AnchoredTextChild::Ref(r) => !r.children.is_empty(),
            AnchoredTextChild::PersName(pn) => !pn.children.is_empty(),
            AnchoredTextChild::CorpName(cn) => !cn.children.is_empty(),
            AnchoredTextChild::Name(n) => !n.children.is_empty(),
            AnchoredTextChild::Seg(s) => !s.children.is_empty(),
            AnchoredTextChild::Title(t) => !t.children.is_empty(),
            AnchoredTextChild::Identifier(i) => !i.children.is_empty(),
            AnchoredTextChild::Date(d) => !d.children.is_empty(),
            AnchoredTextChild::Ptr(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AnchoredTextChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AnchoredTextChild::Rend(r) => r.serialize_children(writer),
            AnchoredTextChild::Lb(_) => Ok(()),
            AnchoredTextChild::Ref(r) => r.serialize_children(writer),
            AnchoredTextChild::PersName(pn) => pn.serialize_children(writer),
            AnchoredTextChild::CorpName(cn) => cn.serialize_children(writer),
            AnchoredTextChild::Name(n) => n.serialize_children(writer),
            AnchoredTextChild::Seg(s) => s.serialize_children(writer),
            AnchoredTextChild::Title(t) => t.serialize_children(writer),
            AnchoredTextChild::Identifier(i) => i.serialize_children(writer),
            AnchoredTextChild::Date(d) => d.serialize_children(writer),
            AnchoredTextChild::Ptr(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "AnchoredTextChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Harm (Harmony) element serialization
// ============================================================================

impl CollectAttributes for AttHarmLog {
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
        push_attr!(attrs, "chordref", self.chordref);
        attrs
    }
}

impl CollectAttributes for AttHarmVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "rendgrid", self.rendgrid);
        attrs
    }
}

impl CollectAttributes for AttHarmGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttHarmAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "inth", vec self.inth);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl MeiSerialize for Harm {
    fn element_name(&self) -> &'static str {
        "harm"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.harm_log.collect_attributes());
        attrs.extend(self.harm_vis.collect_attributes());
        attrs.extend(self.harm_ges.collect_attributes());
        attrs.extend(self.harm_anl.collect_attributes());
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

impl MeiSerialize for HarmChild {
    fn element_name(&self) -> &'static str {
        match self {
            HarmChild::Text(_) => "$text",
            HarmChild::Fb(_) => "fb",
            HarmChild::Rend(_) => "rend",
            HarmChild::Lb(_) => "lb",
            HarmChild::Ref(_) => "ref",
            HarmChild::Line(_) => "line",
            HarmChild::Curve(_) => "curve",
            HarmChild::AnchoredText(_) => "anchoredText",
            HarmChild::Title(_) => "title",
            HarmChild::Num(_) => "num",
            HarmChild::Date(_) => "date",
            HarmChild::Address(_) => "address",
            HarmChild::Annot(_) => "annot",
            HarmChild::Bibl(_) => "bibl",
            HarmChild::BiblStruct(_) => "biblStruct",
            HarmChild::PersName(_) => "persName",
            HarmChild::CorpName(_) => "corpName",
            HarmChild::Name(_) => "name",
            HarmChild::GeogName(_) => "geogName",
            HarmChild::Identifier(_) => "identifier",
            HarmChild::Ptr(_) => "ptr",
            HarmChild::Extent(_) => "extent",
            HarmChild::Fig(_) => "fig",
            HarmChild::Seg(_) => "seg",
            HarmChild::Symbol(_) => "symbol",
            HarmChild::Term(_) => "term",
            HarmChild::Stack(_) => "stack",
            HarmChild::PostBox(_) => "postBox",
            HarmChild::PostCode(_) => "postCode",
            HarmChild::Street(_) => "street",
            HarmChild::Bloc(_) => "bloc",
            HarmChild::Country(_) => "country",
            HarmChild::District(_) => "district",
            HarmChild::GeogFeat(_) => "geogFeat",
            HarmChild::Region(_) => "region",
            HarmChild::Settlement(_) => "settlement",
            HarmChild::PeriodName(_) => "periodName",
            HarmChild::StyleName(_) => "styleName",
            HarmChild::Catchwords(_) => "catchwords",
            HarmChild::Dim(_) => "dim",
            HarmChild::Dimensions(_) => "dimensions",
            HarmChild::Depth(_) => "depth",
            HarmChild::Height(_) => "height",
            HarmChild::Width(_) => "width",
            HarmChild::Heraldry(_) => "heraldry",
            HarmChild::Locus(_) => "locus",
            HarmChild::LocusGrp(_) => "locusGrp",
            HarmChild::Repository(_) => "repository",
            HarmChild::SecFolio(_) => "secFolio",
            HarmChild::Signatures(_) => "signatures",
            HarmChild::Stamp(_) => "stamp",
            HarmChild::Relation(_) => "relation",
            HarmChild::RelationList(_) => "relationList",
            // Editorial elements
            HarmChild::Abbr(_) => "abbr",
            HarmChild::Add(_) => "add",
            HarmChild::Choice(_) => "choice",
            HarmChild::Corr(_) => "corr",
            HarmChild::Damage(_) => "damage",
            HarmChild::Del(_) => "del",
            HarmChild::Expan(_) => "expan",
            HarmChild::Gap(_) => "gap",
            HarmChild::HandShift(_) => "handShift",
            HarmChild::Orig(_) => "orig",
            HarmChild::Q(_) => "q",
            HarmChild::Reg(_) => "reg",
            HarmChild::Restore(_) => "restore",
            HarmChild::Sic(_) => "sic",
            HarmChild::Subst(_) => "subst",
            HarmChild::Supplied(_) => "supplied",
            HarmChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            HarmChild::Text(_) => Vec::new(),
            HarmChild::Fb(fb) => fb.collect_all_attributes(),
            HarmChild::Rend(r) => r.collect_all_attributes(),
            HarmChild::Lb(lb) => lb.collect_all_attributes(),
            HarmChild::Ref(r) => r.collect_all_attributes(),
            HarmChild::Symbol(s) => s.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            HarmChild::Text(_) => false,
            HarmChild::Fb(fb) => fb.has_children(),
            HarmChild::Rend(r) => r.has_children(),
            HarmChild::Lb(_) => false,
            HarmChild::Ref(r) => r.has_children(),
            HarmChild::Symbol(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            HarmChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            HarmChild::Fb(fb) => fb.serialize_children(writer),
            HarmChild::Rend(r) => r.serialize_children(writer),
            HarmChild::Lb(_) => Ok(()),
            HarmChild::Ref(r) => r.serialize_children(writer),
            HarmChild::Symbol(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "HarmChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Fb (Figured Bass) element serialization
// ============================================================================

impl MeiSerialize for Fb {
    fn element_name(&self) -> &'static str {
        "fb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
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

impl MeiSerialize for FbChild {
    fn element_name(&self) -> &'static str {
        match self {
            FbChild::F(_) => "f",
            FbChild::Gap(_) => "gap",
            FbChild::Sic(_) => "sic",
            FbChild::Damage(_) => "damage",
            FbChild::Unclear(_) => "unclear",
            FbChild::Orig(_) => "orig",
            FbChild::Corr(_) => "corr",
            FbChild::Restore(_) => "restore",
            FbChild::Subst(_) => "subst",
            FbChild::Reg(_) => "reg",
            FbChild::HandShift(_) => "handShift",
            FbChild::Add(_) => "add",
            FbChild::Choice(_) => "choice",
            FbChild::Supplied(_) => "supplied",
            FbChild::Del(_) => "del",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FbChild::F(f) => f.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FbChild::F(f) => f.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FbChild::F(f) => f.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FbChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// F (Figure) element serialization
// ============================================================================

impl CollectAttributes for AttFLog {
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

impl CollectAttributes for AttFVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttFGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttFAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttFAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for F {
    fn element_name(&self) -> &'static str {
        "f"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.f_log.collect_attributes());
        attrs.extend(self.f_vis.collect_attributes());
        attrs.extend(self.f_ges.collect_attributes());
        attrs.extend(self.f_anl.collect_attributes());
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

impl MeiSerialize for FChild {
    fn element_name(&self) -> &'static str {
        match self {
            FChild::Text(_) => "$text",
            FChild::Symbol(_) => "symbol",
            FChild::Rend(_) => "rend",
            FChild::Lb(_) => "lb",
            FChild::Title(_) => "title",
            FChild::Num(_) => "num",
            FChild::Date(_) => "date",
            FChild::Ref(_) => "ref",
            FChild::Ptr(_) => "ptr",
            FChild::Annot(_) => "annot",
            FChild::Bibl(_) => "bibl",
            FChild::BiblStruct(_) => "biblStruct",
            FChild::PersName(_) => "persName",
            FChild::CorpName(_) => "corpName",
            FChild::Name(_) => "name",
            FChild::GeogName(_) => "geogName",
            FChild::Identifier(_) => "identifier",
            FChild::Extent(_) => "extent",
            FChild::Fig(_) => "fig",
            FChild::Seg(_) => "seg",
            FChild::Term(_) => "term",
            FChild::Stack(_) => "stack",
            FChild::PostBox(_) => "postBox",
            FChild::PostCode(_) => "postCode",
            FChild::Street(_) => "street",
            FChild::Bloc(_) => "bloc",
            FChild::Country(_) => "country",
            FChild::District(_) => "district",
            FChild::GeogFeat(_) => "geogFeat",
            FChild::Region(_) => "region",
            FChild::Settlement(_) => "settlement",
            FChild::PeriodName(_) => "periodName",
            FChild::StyleName(_) => "styleName",
            FChild::Catchwords(_) => "catchwords",
            FChild::Dim(_) => "dim",
            FChild::Dimensions(_) => "dimensions",
            FChild::Depth(_) => "depth",
            FChild::Height(_) => "height",
            FChild::Width(_) => "width",
            FChild::Heraldry(_) => "heraldry",
            FChild::Locus(_) => "locus",
            FChild::LocusGrp(_) => "locusGrp",
            FChild::Repository(_) => "repository",
            FChild::SecFolio(_) => "secFolio",
            FChild::Signatures(_) => "signatures",
            FChild::Stamp(_) => "stamp",
            FChild::Relation(_) => "relation",
            FChild::RelationList(_) => "relationList",
            FChild::Address(_) => "address",
            // Editorial elements
            FChild::Abbr(_) => "abbr",
            FChild::Add(_) => "add",
            FChild::Choice(_) => "choice",
            FChild::Corr(_) => "corr",
            FChild::Damage(_) => "damage",
            FChild::Del(_) => "del",
            FChild::Expan(_) => "expan",
            FChild::Gap(_) => "gap",
            FChild::HandShift(_) => "handShift",
            FChild::Orig(_) => "orig",
            FChild::Q(_) => "q",
            FChild::Reg(_) => "reg",
            FChild::Restore(_) => "restore",
            FChild::Sic(_) => "sic",
            FChild::Subst(_) => "subst",
            FChild::Supplied(_) => "supplied",
            FChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FChild::Text(_) => Vec::new(),
            FChild::Symbol(s) => s.collect_all_attributes(),
            FChild::Rend(r) => r.collect_all_attributes(),
            FChild::Lb(lb) => lb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FChild::Text(_) => false,
            FChild::Symbol(_) => false,
            FChild::Rend(r) => r.has_children(),
            FChild::Lb(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FChild::Text(_) => Ok(()),
            FChild::Symbol(_) => Ok(()),
            FChild::Rend(r) => r.serialize_children(writer),
            FChild::Lb(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            FChild::Symbol(elem) => elem.serialize_mei(writer),
            FChild::Rend(elem) => elem.serialize_mei(writer),
            FChild::Lb(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Symbol element serialization
// ============================================================================

impl CollectAttributes for AttSymbolLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        attrs
    }
}

impl CollectAttributes for AttSymbolVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "scale", self.scale);
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
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttSymbolGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSymbolGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttSymbolAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSymbolAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Symbol {
    fn element_name(&self) -> &'static str {
        "symbol"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.symbol_log.collect_attributes());
        attrs.extend(self.symbol_vis.collect_attributes());
        attrs.extend(self.symbol_ges.collect_attributes());
        attrs.extend(self.symbol_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Pedal attribute class implementations
// ============================================================================

impl CollectAttributes for AttPedalLog {
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
        push_attr!(attrs, "dir", self.dir);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttPedalVis {
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
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttPedalGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttPedalAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttPedalAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Pedal {
    fn element_name(&self) -> &'static str {
        "pedal"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pedal_log.collect_attributes());
        attrs.extend(self.pedal_vis.collect_attributes());
        attrs.extend(self.pedal_ges.collect_attributes());
        attrs.extend(self.pedal_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

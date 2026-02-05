//! Deserializer implementations for control event MEI elements.
//!
//! This module contains implementations for Slur, Tie, Dynam, Hairpin,
//! Dir, Tempo, and Fermata elements.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttArpegAnl, AttArpegGes, AttArpegLog, AttArpegVis, AttDirAnl, AttDirGes, AttDirLog, AttDirVis,
    AttDynamAnl, AttDynamGes, AttDynamLog, AttDynamVis, AttFermataAnl, AttFermataGes,
    AttFermataLog, AttFermataVis, AttHairpinAnl, AttHairpinGes, AttHairpinLog, AttHairpinVis,
    AttMordentAnl, AttMordentGes, AttMordentLog, AttMordentVis, AttPedalAnl, AttPedalGes,
    AttPedalLog, AttPedalVis, AttSlurAnl, AttSlurGes, AttSlurLog, AttSlurVis, AttTempoAnl,
    AttTempoGes, AttTempoLog, AttTempoVis, AttTieAnl, AttTieGes, AttTieLog, AttTieVis, AttTrillAnl,
    AttTrillGes, AttTrillLog, AttTrillVis, AttTupletSpanAnl, AttTupletSpanGes, AttTupletSpanLog,
    AttTupletSpanVis,
};
use tusk_model::elements::{
    Arpeg, Dir, Dynam, Fermata, Hairpin, Mordent, Pedal, Slur, Tempo, Tie, Trill, TupletSpan,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Arpeg attribute class implementations
// ============================================================================

impl ExtractAttributes for AttArpegLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "order", self.order);
        Ok(())
    }
}

impl ExtractAttributes for AttArpegVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "arrow", self.arrow);
        extract_attr!(attrs, "arrow.shape", self.arrow_shape);
        extract_attr!(attrs, "arrow.size", self.arrow_size);
        extract_attr!(attrs, "arrow.color", self.arrow_color);
        extract_attr!(attrs, "arrow.fillcolor", self.arrow_fillcolor);
        Ok(())
    }
}

impl ExtractAttributes for AttArpegGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttArpegGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttArpegAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttArpegAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for Arpeg {
    fn element_name() -> &'static str {
        "arpeg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut arpeg = Arpeg::default();

        // Extract attributes into each attribute class
        arpeg.common.extract_attributes(&mut attrs)?;
        arpeg.facsimile.extract_attributes(&mut attrs)?;
        arpeg.arpeg_log.extract_attributes(&mut attrs)?;
        arpeg.arpeg_vis.extract_attributes(&mut attrs)?;
        arpeg.arpeg_ges.extract_attributes(&mut attrs)?;
        arpeg.arpeg_anl.extract_attributes(&mut attrs)?;

        // Arpeg has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("arpeg")?;
        }

        Ok(arpeg)
    }
}

// ============================================================================
// Slur attribute class implementations
// ============================================================================

impl ExtractAttributes for AttSlurLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttSlurVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "bezier", self.bezier);
        extract_attr!(attrs, "bulge", self.bulge);
        extract_attr!(attrs, "curvedir", self.curvedir);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        Ok(())
    }
}

impl ExtractAttributes for AttSlurGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttSlurAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "join", vec self.join);
        Ok(())
    }
}

// ============================================================================
// Tie attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTieLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttTieVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "bezier", self.bezier);
        extract_attr!(attrs, "bulge", self.bulge);
        extract_attr!(attrs, "curvedir", self.curvedir);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        Ok(())
    }
}

impl ExtractAttributes for AttTieGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttTieAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTieAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Fermata attribute class implementations
// ============================================================================

impl ExtractAttributes for AttFermataLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        Ok(())
    }
}

impl ExtractAttributes for AttFermataVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "shape", self.shape);
        Ok(())
    }
}

impl ExtractAttributes for AttFermataGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        Ok(())
    }
}

impl ExtractAttributes for AttFermataAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFermataAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Hairpin attribute class implementations
// ============================================================================

impl ExtractAttributes for AttHairpinLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "niente", self.niente);
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        extract_attr!(attrs, "opening", self.opening);
        extract_attr!(attrs, "closed", self.closed);
        extract_attr!(attrs, "opening.vertical", self.opening_vertical);
        extract_attr!(attrs, "angle.optimize", self.angle_optimize);
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "val", self.val);
        extract_attr!(attrs, "val2", self.val2);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttHairpinAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Dir (directive) attribute class implementations
// ============================================================================

impl ExtractAttributes for AttDirLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttDirGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttDirVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttDirAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttDirAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Tempo attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTempoLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "mm", self.mm);
        extract_attr!(attrs, "mm.unit", self.mm_unit);
        extract_attr!(attrs, "mm.dots", self.mm_dots);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttTempoGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "midi.bpm", self.midi_bpm);
        extract_attr!(attrs, "midi.mspb", self.midi_mspb);
        Ok(())
    }
}

impl ExtractAttributes for AttTempoVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttTempoAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTempoAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Dynam attribute class implementations
// ============================================================================

impl ExtractAttributes for AttDynamLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttDynamVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttDynamGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "val", self.val);
        extract_attr!(attrs, "val2", self.val2);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttDynamAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttDynamAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for Slur {
    fn element_name() -> &'static str {
        "slur"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut slur = Slur::default();

        // Extract attributes into each attribute class
        slur.common.extract_attributes(&mut attrs)?;
        slur.facsimile.extract_attributes(&mut attrs)?;
        slur.slur_log.extract_attributes(&mut attrs)?;
        slur.slur_vis.extract_attributes(&mut attrs)?;
        slur.slur_ges.extract_attributes(&mut attrs)?;
        slur.slur_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Skip to end if not empty (slur can contain curve children but we skip for now)
        if !is_empty {
            reader.skip_to_end("slur")?;
        }

        Ok(slur)
    }
}

impl MeiDeserialize for Tie {
    fn element_name() -> &'static str {
        "tie"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tie = Tie::default();

        // Extract attributes into each attribute class
        tie.common.extract_attributes(&mut attrs)?;
        tie.facsimile.extract_attributes(&mut attrs)?;
        tie.tie_log.extract_attributes(&mut attrs)?;
        tie.tie_vis.extract_attributes(&mut attrs)?;
        tie.tie_ges.extract_attributes(&mut attrs)?;
        tie.tie_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Skip to end if not empty (tie can contain curve children but we skip for now)
        if !is_empty {
            reader.skip_to_end("tie")?;
        }

        Ok(tie)
    }
}

impl MeiDeserialize for Dynam {
    fn element_name() -> &'static str {
        "dynam"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::DynamChild;

        let mut dynam = Dynam::default();

        // Extract attributes into each attribute class
        dynam.common.extract_attributes(&mut attrs)?;
        dynam.facsimile.extract_attributes(&mut attrs)?;
        dynam.lang.extract_attributes(&mut attrs)?;
        dynam.dynam_log.extract_attributes(&mut attrs)?;
        dynam.dynam_vis.extract_attributes(&mut attrs)?;
        dynam.dynam_ges.extract_attributes(&mut attrs)?;
        dynam.dynam_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("dynam")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            dynam.children.push(DynamChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend =
                                    super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                dynam.children.push(DynamChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                dynam.children.push(DynamChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam
                                    .children
                                    .push(DynamChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam
                                    .children
                                    .push(DynamChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Name(Box::new(name_elem)));
                            }
                            "seg" => {
                                let seg = super::text::parse_seg_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Seg(Box::new(seg)));
                            }
                            _ => {
                                // Skip unknown child elements
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(dynam)
    }
}

impl MeiDeserialize for Hairpin {
    fn element_name() -> &'static str {
        "hairpin"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut hairpin = Hairpin::default();

        // Extract attributes into each attribute class
        hairpin.common.extract_attributes(&mut attrs)?;
        hairpin.facsimile.extract_attributes(&mut attrs)?;
        hairpin.hairpin_log.extract_attributes(&mut attrs)?;
        hairpin.hairpin_vis.extract_attributes(&mut attrs)?;
        hairpin.hairpin_ges.extract_attributes(&mut attrs)?;
        hairpin.hairpin_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Hairpin is an empty element per MEI spec, but skip to end if not empty
        if !is_empty {
            reader.skip_to_end("hairpin")?;
        }

        Ok(hairpin)
    }
}

impl MeiDeserialize for Dir {
    fn element_name() -> &'static str {
        "dir"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::DirChild;

        let mut dir = Dir::default();

        // Extract attributes into each attribute class
        dir.common.extract_attributes(&mut attrs)?;
        dir.facsimile.extract_attributes(&mut attrs)?;
        dir.lang.extract_attributes(&mut attrs)?;
        dir.dir_log.extract_attributes(&mut attrs)?;
        dir.dir_vis.extract_attributes(&mut attrs)?;
        dir.dir_ges.extract_attributes(&mut attrs)?;
        dir.dir_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("dir")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            dir.children.push(DirChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend =
                                    super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                dir.children.push(DirChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                dir.children.push(DirChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Name(Box::new(name_elem)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Date(Box::new(date)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Title(Box::new(title)));
                            }
                            "identifier" => {
                                let identifier = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children
                                    .push(DirChild::Identifier(Box::new(identifier)));
                            }
                            // Skip unknown child elements
                            _ => {
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(dir)
    }
}

impl MeiDeserialize for Tempo {
    fn element_name() -> &'static str {
        "tempo"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::TempoChild;

        let mut tempo = Tempo::default();

        // Extract attributes into each attribute class
        tempo.common.extract_attributes(&mut attrs)?;
        tempo.bibl.extract_attributes(&mut attrs)?;
        tempo.facsimile.extract_attributes(&mut attrs)?;
        tempo.lang.extract_attributes(&mut attrs)?;
        tempo.tempo_log.extract_attributes(&mut attrs)?;
        tempo.tempo_vis.extract_attributes(&mut attrs)?;
        tempo.tempo_ges.extract_attributes(&mut attrs)?;
        tempo.tempo_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("tempo")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            tempo.children.push(TempoChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend =
                                    super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                tempo.children.push(TempoChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                tempo.children.push(TempoChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo
                                    .children
                                    .push(TempoChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo
                                    .children
                                    .push(TempoChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Name(Box::new(name_elem)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Date(Box::new(date)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Title(Box::new(title)));
                            }
                            "identifier" => {
                                let identifier = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo
                                    .children
                                    .push(TempoChild::Identifier(Box::new(identifier)));
                            }
                            // Skip unknown child elements
                            _ => {
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(tempo)
    }
}

impl MeiDeserialize for Fermata {
    fn element_name() -> &'static str {
        "fermata"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut fermata = Fermata::default();

        // Extract attributes into each attribute class
        fermata.common.extract_attributes(&mut attrs)?;
        fermata.facsimile.extract_attributes(&mut attrs)?;
        fermata.fermata_log.extract_attributes(&mut attrs)?;
        fermata.fermata_vis.extract_attributes(&mut attrs)?;
        fermata.fermata_ges.extract_attributes(&mut attrs)?;
        fermata.fermata_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Fermata has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("fermata")?;
        }

        Ok(fermata)
    }
}

// ============================================================================
// Trill attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTrillLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        extract_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        extract_attr!(attrs, "accidupper", self.accidupper);
        extract_attr!(attrs, "accidlower", self.accidlower);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttTrillVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttTrillGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", self.dur_recip);
        Ok(())
    }
}

impl ExtractAttributes for AttTrillAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTrillAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Trill {
    fn element_name() -> &'static str {
        "trill"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut trill = Trill::default();

        // Extract attributes into each attribute class
        trill.common.extract_attributes(&mut attrs)?;
        trill.facsimile.extract_attributes(&mut attrs)?;
        trill.trill_log.extract_attributes(&mut attrs)?;
        trill.trill_vis.extract_attributes(&mut attrs)?;
        trill.trill_ges.extract_attributes(&mut attrs)?;
        trill.trill_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Trill has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("trill")?;
        }

        Ok(trill)
    }
}

// ============================================================================
// Mordent attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMordentLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        extract_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        extract_attr!(attrs, "accidupper", self.accidupper);
        extract_attr!(attrs, "accidlower", self.accidlower);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "long", self.long);
        Ok(())
    }
}

impl ExtractAttributes for AttMordentVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        Ok(())
    }
}

impl ExtractAttributes for AttMordentGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMordentGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttMordentAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMordentAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Mordent {
    fn element_name() -> &'static str {
        "mordent"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut mordent = Mordent::default();

        // Extract attributes into each attribute class
        mordent.common.extract_attributes(&mut attrs)?;
        mordent.facsimile.extract_attributes(&mut attrs)?;
        mordent.mordent_log.extract_attributes(&mut attrs)?;
        mordent.mordent_vis.extract_attributes(&mut attrs)?;
        mordent.mordent_ges.extract_attributes(&mut attrs)?;
        mordent.mordent_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Mordent has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("mordent")?;
        }

        Ok(mordent)
    }
}

// ============================================================================
// Reh (rehearsal mark) attribute class implementations
// ============================================================================

use tusk_model::att::{AttRehAnl, AttRehGes, AttRehLog, AttRehVis};
use tusk_model::elements::{Reh, RehChild};

impl ExtractAttributes for AttRehLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttRehVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttRehGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRehGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRehAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRehAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Reh {
    fn element_name() -> &'static str {
        "reh"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut reh = Reh::default();

        // Extract attributes into each attribute class
        reh.common.extract_attributes(&mut attrs)?;
        reh.facsimile.extract_attributes(&mut attrs)?;
        reh.lang.extract_attributes(&mut attrs)?;
        reh.reh_log.extract_attributes(&mut attrs)?;
        reh.reh_vis.extract_attributes(&mut attrs)?;
        reh.reh_ges.extract_attributes(&mut attrs)?;
        reh.reh_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Reh can contain mixed content (text, rend, stack, lb)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("reh")? {
                match content {
                    MixedContent::Text(text) => {
                        reh.children.push(RehChild::Text(text));
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            reh.children.push(RehChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            reh.children.push(RehChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Unknown/unsupported element (including stack) - skip it
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    },
                }
            }
        }

        Ok(reh)
    }
}

// ============================================================================
// AnchoredText attribute class implementations
// ============================================================================

use tusk_model::att::{
    AttAnchoredTextAnl, AttAnchoredTextGes, AttAnchoredTextLog, AttAnchoredTextVis,
};
use tusk_model::elements::{AnchoredText, AnchoredTextChild};

impl ExtractAttributes for AttAnchoredTextLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttAnchoredTextVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttAnchoredTextGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAnchoredTextGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttAnchoredTextAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAnchoredTextAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for AnchoredText {
    fn element_name() -> &'static str {
        "anchoredText"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut anchored_text = AnchoredText::default();

        // Extract attributes into each attribute class
        anchored_text.common.extract_attributes(&mut attrs)?;
        anchored_text.facsimile.extract_attributes(&mut attrs)?;
        anchored_text.lang.extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_log
            .extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_vis
            .extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_ges
            .extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_anl
            .extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("anchoredText")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            anchored_text.children.push(AnchoredTextChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend =
                                    super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Name(Box::new(name_elem)));
                            }
                            "seg" => {
                                let seg = super::text::parse_seg_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Seg(Box::new(seg)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Title(Box::new(title)));
                            }
                            "identifier" => {
                                let identifier = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Identifier(Box::new(identifier)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Date(Box::new(date)));
                            }
                            "ptr" => {
                                let ptr = super::header::parse_ptr_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Ptr(Box::new(ptr)));
                            }
                            _ => {
                                // Skip unknown child elements
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(anchored_text)
    }
}

// ============================================================================
// Harm (Harmony) element deserialization
// ============================================================================

use tusk_model::att::{
    AttFAnl, AttFGes, AttFLog, AttFVis, AttHarmAnl, AttHarmGes, AttHarmLog, AttHarmVis,
    AttSymbolAnl, AttSymbolGes, AttSymbolLog, AttSymbolVis,
};
use tusk_model::elements::{F, FChild, Fb, FbChild, Harm, HarmChild, Symbol};

impl ExtractAttributes for AttHarmLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "chordref", self.chordref);
        Ok(())
    }
}

impl ExtractAttributes for AttHarmVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "rendgrid", self.rendgrid);
        Ok(())
    }
}

impl ExtractAttributes for AttHarmGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttHarmAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "inth", vec self.inth);
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl MeiDeserialize for Harm {
    fn element_name() -> &'static str {
        "harm"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut harm = Harm::default();

        // Extract attributes into each attribute class
        harm.common.extract_attributes(&mut attrs)?;
        harm.facsimile.extract_attributes(&mut attrs)?;
        harm.harm_log.extract_attributes(&mut attrs)?;
        harm.harm_vis.extract_attributes(&mut attrs)?;
        harm.harm_ges.extract_attributes(&mut attrs)?;
        harm.harm_anl.extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("harm")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            harm.children.push(HarmChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "fb" => {
                                let fb = parse_fb_from_event(reader, child_attrs, child_empty)?;
                                harm.children.push(HarmChild::Fb(Box::new(fb)));
                            }
                            "rend" => {
                                let rend =
                                    super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                harm.children.push(HarmChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                harm.children.push(HarmChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                harm.children.push(HarmChild::Ref(Box::new(ref_elem)));
                            }
                            _ => {
                                // Skip unknown child elements
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(harm)
    }
}

// ============================================================================
// Fb (Figured Bass) element deserialization
// ============================================================================

/// Parse a `<fb>` element from an event.
pub fn parse_fb_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Fb> {
    let mut fb = Fb::default();

    // Extract attributes
    fb.common.extract_attributes(&mut attrs)?;
    fb.facsimile.extract_attributes(&mut attrs)?;

    // Parse children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("fb")? {
            match name.as_str() {
                "f" => {
                    let f = parse_f_from_event(reader, child_attrs, child_empty)?;
                    fb.children.push(FbChild::F(Box::new(f)));
                }
                _ => {
                    // Skip unknown child elements
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(fb)
}

// ============================================================================
// F (Figure) element deserialization
// ============================================================================

impl ExtractAttributes for AttFLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttFVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttFGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttFAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFAnl has no attributes
        Ok(())
    }
}

/// Parse a `<f>` (figure) element from an event.
pub fn parse_f_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<F> {
    let mut f = F::default();

    // Extract attributes
    f.common.extract_attributes(&mut attrs)?;
    f.facsimile.extract_attributes(&mut attrs)?;
    f.f_log.extract_attributes(&mut attrs)?;
    f.f_vis.extract_attributes(&mut attrs)?;
    f.f_ges.extract_attributes(&mut attrs)?;
    f.f_anl.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements like symbol)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("f")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        f.children.push(FChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "symbol" => {
                        let symbol = parse_symbol_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Symbol(Box::new(symbol)));
                    }
                    "rend" => {
                        let rend = super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Rend(Box::new(rend)));
                    }
                    "lb" => {
                        let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Lb(Box::new(lb)));
                    }
                    _ => {
                        // Skip unknown child elements
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(f)
}

// ============================================================================
// Symbol element deserialization
// ============================================================================

impl ExtractAttributes for AttSymbolLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "startid", self.startid);
        Ok(())
    }
}

impl ExtractAttributes for AttSymbolVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "scale", self.scale);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttSymbolGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSymbolGes has no attributes in the ODD spec
        Ok(())
    }
}

impl ExtractAttributes for AttSymbolAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSymbolAnl has no attributes in the ODD spec
        Ok(())
    }
}

/// Parse a `<symbol>` element from an event.
pub fn parse_symbol_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Symbol> {
    let mut symbol = Symbol::default();

    // Extract attributes
    symbol.common.extract_attributes(&mut attrs)?;
    symbol.facsimile.extract_attributes(&mut attrs)?;
    symbol.symbol_log.extract_attributes(&mut attrs)?;
    symbol.symbol_vis.extract_attributes(&mut attrs)?;
    symbol.symbol_ges.extract_attributes(&mut attrs)?;
    symbol.symbol_anl.extract_attributes(&mut attrs)?;

    // Symbol has no children, but handle non-empty gracefully
    if !is_empty {
        reader.skip_to_end("symbol")?;
    }

    Ok(symbol)
}

// ============================================================================
// Pedal attribute class implementations
// ============================================================================

impl ExtractAttributes for AttPedalLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "dir", self.dir);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttPedalVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl ExtractAttributes for AttPedalGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttPedalAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttPedalAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for Pedal {
    fn element_name() -> &'static str {
        "pedal"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut pedal = Pedal::default();

        // Extract attributes into each attribute class
        pedal.common.extract_attributes(&mut attrs)?;
        pedal.facsimile.extract_attributes(&mut attrs)?;
        pedal.pedal_log.extract_attributes(&mut attrs)?;
        pedal.pedal_vis.extract_attributes(&mut attrs)?;
        pedal.pedal_ges.extract_attributes(&mut attrs)?;
        pedal.pedal_anl.extract_attributes(&mut attrs)?;

        // Pedal has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("pedal")?;
        }

        Ok(pedal)
    }
}

// ============================================================================
// TupletSpan attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTupletSpanLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam.with", self.beam_with);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletSpanVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "num.place", self.num_place);
        extract_attr!(attrs, "num.visible", self.num_visible);
        extract_attr!(attrs, "bracket.place", self.bracket_place);
        extract_attr!(attrs, "bracket.visible", self.bracket_visible);
        extract_attr!(attrs, "num.format", self.num_format);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletSpanGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletSpanAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTupletSpanAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for TupletSpan {
    fn element_name() -> &'static str {
        "tupletSpan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tuplet_span = TupletSpan::default();

        // Extract attributes into each attribute class
        tuplet_span.common.extract_attributes(&mut attrs)?;
        tuplet_span.facsimile.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_log.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_vis.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_ges.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_anl.extract_attributes(&mut attrs)?;

        // TupletSpan has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("tupletSpan")?;
        }

        Ok(tuplet_span)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::Fermata;

    // ============================================================================
    // Slur deserialization tests
    // ============================================================================

    #[test]
    fn slur_deserializes_from_empty_element() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.common.xml_id.is_none());
        assert!(slur.slur_log.startid.is_none());
        assert!(slur.slur_log.endid.is_none());
        assert!(slur.children.is_empty());
    }

    #[test]
    fn slur_deserializes_xml_id() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur xml:id="s1"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn slur_deserializes_startid_endid() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur startid="#n1" endid="#n2"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.startid.is_some());
        assert!(slur.slur_log.endid.is_some());
    }

    #[test]
    fn slur_deserializes_staff_layer() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur staff="1" layer="1"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.slur_log.staff, vec![1]);
        assert_eq!(slur.slur_log.layer, vec![1]);
    }

    #[test]
    fn slur_deserializes_tstamp_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur tstamp="1" tstamp2="0m+2"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.tstamp.is_some());
        assert!(slur.slur_log.tstamp2.is_some());
    }

    #[test]
    fn slur_deserializes_visual_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur curvedir="above" lform="solid"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_vis.curvedir.is_some());
        assert!(slur.slur_vis.lform.is_some());
    }

    #[test]
    fn slur_deserializes_gestural_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur dur.ges="4" dur.ppq="480"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_ges.dur_ges.is_some());
        assert_eq!(slur.slur_ges.dur_ppq, Some(480));
    }

    #[test]
    fn slur_deserializes_analytical_attributes() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur join="#s2"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(!slur.slur_anl.join.is_empty());
    }

    #[test]
    fn slur_deserializes_full_attributes() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur xml:id="s1" startid="#n1" endid="#n2" staff="1" layer="1" curvedir="below"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
        assert!(slur.slur_log.startid.is_some());
        assert!(slur.slur_log.endid.is_some());
        assert_eq!(slur.slur_log.staff, vec![1]);
        assert!(slur.slur_vis.curvedir.is_some());
    }

    #[test]
    fn slur_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur xml:id="s1" unknown="value"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn slur_deserializes_evaluate_attribute() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur evaluate="all"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.evaluate.is_some());
    }

    #[test]
    fn slur_deserializes_coordinate_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur x="100" y="200" x2="300" y2="250"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.slur_vis.x, Some(100.0));
        assert_eq!(slur.slur_vis.y, Some(200.0));
        assert_eq!(slur.slur_vis.x2, Some(300.0));
        assert_eq!(slur.slur_vis.y2, Some(250.0));
    }

    #[test]
    fn slur_deserializes_offset_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_vis.startho.is_some());
        assert!(slur.slur_vis.endho.is_some());
        assert!(slur.slur_vis.startvo.is_some());
        assert!(slur.slur_vis.endvo.is_some());
    }

    // ============================================================================
    // Tie deserialization tests
    // ============================================================================

    #[test]
    fn tie_deserializes_from_empty_element() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.common.xml_id.is_none());
        assert!(tie.tie_log.startid.is_none());
        assert!(tie.tie_log.endid.is_none());
        assert!(tie.children.is_empty());
    }

    #[test]
    fn tie_deserializes_xml_id() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie xml:id="t1"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tie_deserializes_startid_and_endid() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie startid="#n1" endid="#n2"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.startid.is_some());
        assert!(tie.tie_log.endid.is_some());
    }

    #[test]
    fn tie_deserializes_tstamp_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie tstamp="1" tstamp2="0m+2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.tstamp.is_some());
        assert!(tie.tie_log.tstamp2.is_some());
    }

    #[test]
    fn tie_deserializes_staff_and_layer() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie staff="1" layer="1"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.staff, vec![1]);
        assert_eq!(tie.tie_log.layer, vec![1]);
    }

    #[test]
    fn tie_deserializes_multiple_staff_values() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie staff="1 2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.staff, vec![1, 2]);
    }

    #[test]
    fn tie_deserializes_visual_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie curvedir="above" color="red"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.curvedir.is_some());
        assert!(tie.tie_vis.color.is_some());
    }

    #[test]
    fn tie_deserializes_bezier_attribute() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie bezier="19 45 -32 118"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.bezier.is_some());
    }

    #[test]
    fn tie_deserializes_gestural_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie tstamp2.ges="0m+2.5"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_ges.tstamp2_ges.is_some());
    }

    #[test]
    fn tie_deserializes_full_attributes() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie xml:id="t1" startid="#n1" endid="#n2" staff="1" layer="1" curvedir="below"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
        assert!(tie.tie_log.startid.is_some());
        assert!(tie.tie_log.endid.is_some());
        assert_eq!(tie.tie_log.staff, vec![1]);
        assert!(tie.tie_vis.curvedir.is_some());
    }

    #[test]
    fn tie_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie xml:id="t1" unknown="value"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tie_deserializes_evaluate_attribute() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie evaluate="all"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.evaluate.is_some());
    }

    #[test]
    fn tie_deserializes_coordinate_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie x="100" y="200" x2="300" y2="250"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_vis.x, Some(100.0));
        assert_eq!(tie.tie_vis.y, Some(200.0));
        assert_eq!(tie.tie_vis.x2, Some(300.0));
        assert_eq!(tie.tie_vis.y2, Some(250.0));
    }

    #[test]
    fn tie_deserializes_offset_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.startho.is_some());
        assert!(tie.tie_vis.endho.is_some());
        assert!(tie.tie_vis.startvo.is_some());
        assert!(tie.tie_vis.endvo.is_some());
    }

    #[test]
    fn tie_deserializes_plist_attribute() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie plist="#n1 #n2"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.plist.len(), 2);
    }

    #[test]
    fn tie_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie lform="dashed" lwidth="medium"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.lform.is_some());
        assert!(tie.tie_vis.lwidth.is_some());
    }

    // ============================================================================
    // Dynam deserialization tests
    // ============================================================================

    #[test]
    fn dynam_deserializes_from_empty_element() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam/>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.common.xml_id.is_none());
        assert!(dynam.dynam_log.startid.is_none());
        assert!(dynam.children.is_empty());
    }

    #[test]
    fn dynam_deserializes_with_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r#"<dynam>f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "f"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_deserializes_longer_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r#"<dynam>cresc. poco a poco</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "cresc. poco a poco"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_deserializes_xml_id() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:id="d1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dynam_deserializes_startid() {
        use tusk_model::elements::Dynam;

        let xml = r##"<dynam startid="#n1">f</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_log.startid.is_some());
    }

    #[test]
    fn dynam_deserializes_staff_and_layer() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam staff="1" layer="1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.staff, vec![1]);
        assert_eq!(dynam.dynam_log.layer, vec![1]);
    }

    #[test]
    fn dynam_deserializes_tstamp_attributes() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam tstamp="2" tstamp2="1m+1">cresc.</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_log.tstamp.is_some());
        assert!(dynam.dynam_log.tstamp2.is_some());
    }

    #[test]
    fn dynam_deserializes_place_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="above" staff="1" tstamp="1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_vis.place.is_some());
    }

    #[test]
    fn dynam_deserializes_extender_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam extender="true" tstamp="1" tstamp2="2m+1">dim.</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_vis.extender.is_some());
    }

    #[test]
    fn dynam_deserializes_val_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam val="84" staff="1" tstamp="1">f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_ges.val.is_some());
    }

    #[test]
    fn dynam_deserializes_plist_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r##"<dynam plist="#n1 #n2 #n3 #n4" startid="#n1">cresc.</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.plist.len(), 4);
    }

    #[test]
    fn dynam_deserializes_full_attributes() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r##"<dynam xml:id="d1" staff="2" place="above" startid="#n1" endid="#n4" plist="#n1 #n2 #n3 #n4">cresc. poco a poco</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
        assert_eq!(dynam.dynam_log.staff, vec![2]);
        assert!(dynam.dynam_vis.place.is_some());
        assert!(dynam.dynam_log.startid.is_some());
        assert!(dynam.dynam_log.endid.is_some());
        assert_eq!(dynam.dynam_log.plist.len(), 4);

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "cresc. poco a poco"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:id="d1" unknown="value">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dynam_deserializes_multiple_staff_values() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="between" staff="1 2" tstamp="1">f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.staff, vec![1, 2]);
    }

    #[test]
    fn dynam_deserializes_vgrp_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="below" staff="1" tstamp="2" vgrp="40">sf</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_vis.vgrp, Some(40));
    }

    #[test]
    fn dynam_deserializes_coordinate_attributes() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam x="100" y="200">mf</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_vis.x, Some(100.0));
        assert_eq!(dynam.dynam_vis.y, Some(200.0));
    }

    #[test]
    fn dynam_deserializes_duration_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam staff="2" tstamp="3" dur="1">cresc. poco a poco</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(!dynam.dynam_log.dur.is_empty());
    }

    #[test]
    fn dynam_deserializes_lang_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:lang="it">forte</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.lang.xml_lang, Some("it".to_string()));
    }

    // ============================================================================
    // Hairpin deserialization tests
    // ============================================================================

    #[test]
    fn hairpin_deserializes_from_empty_element() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.common.xml_id.is_none());
        assert!(hairpin.hairpin_log.startid.is_none());
        assert!(hairpin.hairpin_log.endid.is_none());
        assert!(hairpin.hairpin_log.form.is_none());
    }

    #[test]
    fn hairpin_deserializes_xml_id() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin xml:id="h1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn hairpin_deserializes_form_cres() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin form="cres"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Cres));
    }

    #[test]
    fn hairpin_deserializes_form_dim() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin form="dim"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    }

    #[test]
    fn hairpin_deserializes_niente_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin niente="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.niente, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_startid_endid() {
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin startid="#n1" endid="#n2"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.startid.is_some());
        assert!(hairpin.hairpin_log.endid.is_some());
    }

    #[test]
    fn hairpin_deserializes_staff_and_layer() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin staff="1" layer="1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.staff, vec![1]);
        assert_eq!(hairpin.hairpin_log.layer, vec![1]);
    }

    #[test]
    fn hairpin_deserializes_multiple_staff_values() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin staff="1 2"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.staff, vec![1, 2]);
    }

    #[test]
    fn hairpin_deserializes_tstamp_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin tstamp="1" tstamp2="0m+4"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.tstamp.is_some());
        assert!(hairpin.hairpin_log.tstamp2.is_some());
    }

    #[test]
    fn hairpin_deserializes_visual_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin place="above" color="red"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.place.is_some());
        assert!(hairpin.hairpin_vis.color.is_some());
    }

    #[test]
    fn hairpin_deserializes_opening_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin opening="1.5"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.opening.is_some());
    }

    #[test]
    fn hairpin_deserializes_closed_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin closed="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.closed, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_opening_vertical_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin opening.vertical="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            hairpin.hairpin_vis.opening_vertical,
            Some(DataBoolean::True)
        );
    }

    #[test]
    fn hairpin_deserializes_angle_optimize_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin angle.optimize="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.angle_optimize, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_line_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin lform="solid" lwidth="medium"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.lform.is_some());
        assert!(hairpin.hairpin_vis.lwidth.is_some());
    }

    #[test]
    fn hairpin_deserializes_coordinate_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin x="100" y="200" x2="300" y2="250"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.x, Some(100.0));
        assert_eq!(hairpin.hairpin_vis.y, Some(200.0));
        assert_eq!(hairpin.hairpin_vis.x2, Some(300.0));
        assert_eq!(hairpin.hairpin_vis.y2, Some(250.0));
    }

    #[test]
    fn hairpin_deserializes_offset_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.startho.is_some());
        assert!(hairpin.hairpin_vis.endho.is_some());
        assert!(hairpin.hairpin_vis.startvo.is_some());
        assert!(hairpin.hairpin_vis.endvo.is_some());
    }

    #[test]
    fn hairpin_deserializes_gestural_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin dur.ges="4" dur.ppq="480"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_ges.dur_ges.is_some());
        assert_eq!(hairpin.hairpin_ges.dur_ppq, Some(480));
    }

    #[test]
    fn hairpin_deserializes_midi_val_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin val="64" val2="100"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_ges.val.is_some());
        assert!(hairpin.hairpin_ges.val2.is_some());
    }

    #[test]
    fn hairpin_deserializes_full_attributes() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin xml:id="h1" form="cres" startid="#n1" endid="#n2" staff="1" layer="1" place="below" opening="2"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Cres));
        assert!(hairpin.hairpin_log.startid.is_some());
        assert!(hairpin.hairpin_log.endid.is_some());
        assert_eq!(hairpin.hairpin_log.staff, vec![1]);
        assert!(hairpin.hairpin_vis.place.is_some());
        assert!(hairpin.hairpin_vis.opening.is_some());
    }

    #[test]
    fn hairpin_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin xml:id="h1" unknown="value"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn hairpin_deserializes_evaluate_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin evaluate="all"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.evaluate.is_some());
    }

    #[test]
    fn hairpin_deserializes_vgrp_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin vgrp="1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.vgrp, Some(1));
    }

    #[test]
    fn hairpin_deserializes_dur_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin dur="4"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(!hairpin.hairpin_log.dur.is_empty());
    }

    #[test]
    fn hairpin_deserializes_plist_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin plist="#n1 #n2 #n3"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.plist.len(), 3);
    }

    // ============================================================================
    // Dir (directive) deserialization tests
    // ============================================================================

    #[test]
    fn dir_deserializes_from_empty_element() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir/>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.common.xml_id.is_none());
        assert!(dir.dir_log.startid.is_none());
        assert!(dir.children.is_empty());
    }

    #[test]
    fn dir_deserializes_with_text_content() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir>affettuoso</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "affettuoso"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dir_deserializes_xml_id() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:id="dir1">arco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
    }

    #[test]
    fn dir_deserializes_startid() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir startid="#n1">pizz.</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.startid.is_some());
    }

    #[test]
    fn dir_deserializes_endid() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir startid="#n1" endid="#n4">legato</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.startid.is_some());
        assert!(dir.dir_log.endid.is_some());
    }

    #[test]
    fn dir_deserializes_staff_and_layer() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir staff="1" layer="1">dolce</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.staff, vec![1]);
        assert_eq!(dir.dir_log.layer, vec![1]);
    }

    #[test]
    fn dir_deserializes_tstamp_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir tstamp="1" tstamp2="0m+4">rit.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.tstamp.is_some());
        assert!(dir.dir_log.tstamp2.is_some());
    }

    #[test]
    fn dir_deserializes_place_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir place="above" staff="1" tstamp="1">sul G</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.place.is_some());
    }

    #[test]
    fn dir_deserializes_extender_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir extender="true" tstamp="1" tstamp2="1m+1">accel.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.extender.is_some());
    }

    #[test]
    fn dir_deserializes_lang_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:lang="it">con fuoco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.lang.xml_lang, Some("it".to_string()));
    }

    #[test]
    fn dir_deserializes_dur_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir tstamp="1" dur="2">poco a poco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(!dir.dir_log.dur.is_empty());
    }

    #[test]
    fn dir_deserializes_plist_attribute() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir plist="#n1 #n2 #n3">espressivo</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.plist.len(), 3);
    }

    #[test]
    fn dir_deserializes_visual_color_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir color="red">important</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.color.is_some());
    }

    #[test]
    fn dir_deserializes_coordinate_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir x="100" y="200">text</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.ho.is_some() || dir.dir_vis.x.is_some());
    }

    #[test]
    fn dir_deserializes_vgrp_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir vgrp="1" tstamp="1">align group</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_vis.vgrp, Some(1));
    }

    #[test]
    fn dir_deserializes_gestural_duration_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir dur.ges="4" dur.ppq="480">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_ges.dur_ges.is_some());
        assert_eq!(dir.dir_ges.dur_ppq, Some(480));
    }

    #[test]
    fn dir_deserializes_multiple_staff_values() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir staff="1 2" place="between">between staves</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.staff, vec![1, 2]);
    }

    #[test]
    fn dir_deserializes_full_attributes() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r##"<dir xml:id="dir1" staff="1" place="above" startid="#n1" endid="#n4" plist="#n1 #n2 #n3 #n4" extender="true">molto espressivo</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
        assert_eq!(dir.dir_log.staff, vec![1]);
        assert!(dir.dir_vis.place.is_some());
        assert!(dir.dir_log.startid.is_some());
        assert!(dir.dir_log.endid.is_some());
        assert_eq!(dir.dir_log.plist.len(), 4);
        assert!(dir.dir_vis.extender.is_some());

        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "molto espressivo"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dir_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:id="dir1" unknown="value">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
    }

    #[test]
    fn dir_deserializes_evaluate_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir evaluate="all">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.evaluate.is_some());
    }

    #[test]
    fn dir_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir lform="dashed" lwidth="medium" extender="true">dim.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.lform.is_some());
        assert!(dir.dir_vis.lwidth.is_some());
    }

    #[test]
    fn dir_deserializes_rend_children() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir xml:id="d1"><rend fontweight="bold">forte</rend></dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("d1".to_string()));
        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Rend(rend) => {
                assert_eq!(rend.children.len(), 1);
            }
            _ => panic!("Expected Rend child"),
        }
    }

    #[test]
    fn dir_deserializes_mixed_content_with_rend() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir xml:id="d2">play <rend fontstyle="italic">quietly</rend> here</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("d2".to_string()));
        assert_eq!(dir.children.len(), 3);
        match &dir.children[0] {
            DirChild::Text(text) => assert!(text.starts_with("play")),
            _ => panic!("Expected Text child first"),
        }
        match &dir.children[1] {
            DirChild::Rend(_) => {}
            _ => panic!("Expected Rend child second"),
        }
        match &dir.children[2] {
            DirChild::Text(text) => assert!(text.ends_with("here")),
            _ => panic!("Expected Text child third"),
        }
    }

    #[test]
    fn dir_deserializes_lb_children() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir>line one<lb/>line two</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.children.len(), 3);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "line one"),
            _ => panic!("Expected Text child first"),
        }
        match &dir.children[1] {
            DirChild::Lb(_) => {}
            _ => panic!("Expected Lb child second"),
        }
        match &dir.children[2] {
            DirChild::Text(text) => assert_eq!(text, "line two"),
            _ => panic!("Expected Text child third"),
        }
    }

    // ============================================================================
    // Tempo deserialization tests
    // ============================================================================

    #[test]
    fn tempo_deserializes_from_empty_element() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo/>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.common.xml_id.is_none());
        assert!(tempo.tempo_log.startid.is_none());
        assert!(tempo.children.is_empty());
    }

    #[test]
    fn tempo_deserializes_with_text_content() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text, "Allegro"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn tempo_deserializes_xml_id() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:id="tempo1">Andante</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
    }

    #[test]
    fn tempo_deserializes_startid() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo startid="#n1">Moderato</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.startid.is_some());
    }

    #[test]
    fn tempo_deserializes_staff_and_tstamp() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo staff="1" tstamp="1">Presto</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.staff, vec![1]);
        assert!(tempo.tempo_log.tstamp.is_some());
    }

    #[test]
    fn tempo_deserializes_mm_attributes() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo mm="120" mm.unit="4" mm.dots="0">♩ = 120</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.mm.is_some());
        assert!(tempo.tempo_log.mm_unit.is_some());
        assert!(tempo.tempo_log.mm_dots.is_some());
    }

    #[test]
    fn tempo_deserializes_func_instantaneous() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="instantaneous">Largo</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
    }

    #[test]
    fn tempo_deserializes_func_continuous() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="continuous" tstamp="1" tstamp2="0m+4">accel.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Continuous));
        assert!(tempo.tempo_log.tstamp2.is_some());
    }

    #[test]
    fn tempo_deserializes_func_metricmod() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="metricmod">♩ = ♪</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Metricmod));
    }

    #[test]
    fn tempo_deserializes_place_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo place="above" staff="1" tstamp="1">Vivace</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.place.is_some());
    }

    #[test]
    fn tempo_deserializes_extender_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo extender="true" tstamp="1" tstamp2="1m+1">rit.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.extender.is_some());
    }

    #[test]
    fn tempo_deserializes_lang_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:lang="it">Allegro con brio</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.lang.xml_lang, Some("it".to_string()));
    }

    #[test]
    fn tempo_deserializes_midi_bpm() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo midi.bpm="120">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_ges.midi_bpm.is_some());
    }

    #[test]
    fn tempo_deserializes_midi_mspb() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo midi.mspb="500000">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_ges.midi_mspb.is_some());
    }

    #[test]
    fn tempo_deserializes_visual_color_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo color="red">Largo</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.color.is_some());
    }

    #[test]
    fn tempo_deserializes_coordinate_attributes() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo x="100" y="200">Adagio</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.x.is_some());
        assert!(tempo.tempo_vis.y.is_some());
    }

    #[test]
    fn tempo_deserializes_layer_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo layer="1" staff="1" tstamp="1">Andante</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.layer, vec![1]);
    }

    #[test]
    fn tempo_deserializes_endid() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo startid="#n1" endid="#n4" func="continuous">rallentando</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.startid.is_some());
        assert!(tempo.tempo_log.endid.is_some());
    }

    #[test]
    fn tempo_deserializes_plist_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo plist="#n1 #n2 #n3">Presto</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.plist.len(), 3);
    }

    #[test]
    fn tempo_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:id="tempo1" unknown="value">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
    }

    #[test]
    fn tempo_deserializes_all_common_attributes() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo xml:id="tempo1" staff="1" tstamp="1" mm="120" mm.unit="4" func="instantaneous" place="above" extender="false" xml:lang="de">Schnell</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
        assert_eq!(tempo.tempo_log.staff, vec![1]);
        assert!(tempo.tempo_log.tstamp.is_some());
        assert!(tempo.tempo_log.mm.is_some());
        assert!(tempo.tempo_log.mm_unit.is_some());
        assert!(tempo.tempo_log.func.is_some());
        assert!(tempo.tempo_vis.place.is_some());
        assert!(tempo.tempo_vis.extender.is_some());
        assert_eq!(tempo.lang.xml_lang, Some("de".to_string()));

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text, "Schnell"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn tempo_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo lform="dashed" lwidth="medium" extender="true">accel.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.lform.is_some());
        assert!(tempo.tempo_vis.lwidth.is_some());
    }

    #[test]
    fn tempo_deserializes_rend_child() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo><rend fontsize="6.9pt" fontweight="bold">A</rend></tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Rend(rend) => {
                assert!(rend.typography.fontsize.is_some());
                assert!(rend.typography.fontweight.is_some());
                assert_eq!(rend.children.len(), 1);
            }
            _ => panic!("Expected Rend child, got {:?}", tempo.children[0]),
        }
    }

    #[test]
    fn tempo_deserializes_mixed_text_and_rend_children() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Text before <rend fontweight="bold">bold</rend> text after</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 3);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text.trim(), "Text before"),
            _ => panic!("Expected Text child"),
        }
        match &tempo.children[1] {
            TempoChild::Rend(_) => {}
            _ => panic!("Expected Rend child"),
        }
        match &tempo.children[2] {
            TempoChild::Text(text) => assert_eq!(text.trim(), "text after"),
            _ => panic!("Expected Text child"),
        }
    }

    #[test]
    fn tempo_deserializes_lb_child() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Tempo<lb/>marking</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.children.len() >= 2);
        let has_lb = tempo
            .children
            .iter()
            .any(|c| matches!(c, TempoChild::Lb(_)));
        assert!(has_lb, "Expected Lb child element");
    }

    // ============================================================================
    // Fermata tests
    // ============================================================================

    #[test]
    fn fermata_deserializes_from_empty_element() {
        let xml = r#"<fermata/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.common.xml_id.is_none());
        assert!(fermata.fermata_log.startid.is_none());
        assert!(fermata.fermata_vis.form.is_none());
    }

    #[test]
    fn fermata_deserializes_xml_id() {
        let xml = r#"<fermata xml:id="f1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    #[test]
    fn fermata_deserializes_startid() {
        let xml = r##"<fermata startid="#note1"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_log.startid.is_some());
    }

    #[test]
    fn fermata_deserializes_staff_and_tstamp() {
        let xml = r#"<fermata staff="1" tstamp="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.staff, vec![1]);
        assert!(fermata.fermata_log.tstamp.is_some());
    }

    #[test]
    fn fermata_deserializes_form_norm() {
        use tusk_model::att::AttFermataVisForm;

        let xml = r#"<fermata form="norm"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Norm));
    }

    #[test]
    fn fermata_deserializes_form_inv() {
        use tusk_model::att::AttFermataVisForm;

        let xml = r#"<fermata form="inv"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Inv));
    }

    #[test]
    fn fermata_deserializes_shape_curved() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="curved"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Curved));
    }

    #[test]
    fn fermata_deserializes_shape_square() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="square"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Square));
    }

    #[test]
    fn fermata_deserializes_shape_angular() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="angular"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Angular));
    }

    #[test]
    fn fermata_deserializes_place_attribute() {
        let xml = r#"<fermata place="above"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.place.is_some());
    }

    #[test]
    fn fermata_deserializes_color_attribute() {
        let xml = r#"<fermata color="red"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.color.is_some());
    }

    #[test]
    fn fermata_deserializes_coordinate_attributes() {
        let xml = r#"<fermata x="100" y="200"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.x, Some(100.0));
        assert_eq!(fermata.fermata_vis.y, Some(200.0));
    }

    #[test]
    fn fermata_deserializes_layer_attribute() {
        let xml = r#"<fermata layer="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.layer, vec![1]);
    }

    #[test]
    fn fermata_deserializes_endid() {
        let xml = r##"<fermata startid="#note1" endid="#note2"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_log.startid.is_some());
        assert!(fermata.fermata_log.endid.is_some());
    }

    #[test]
    fn fermata_deserializes_plist_attribute() {
        let xml = r##"<fermata plist="#note1 #note2"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.plist.len(), 2);
    }

    #[test]
    fn fermata_deserializes_gestural_duration() {
        let xml = r#"<fermata dur.ppq="480" dur.real="2.5"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_ges.dur_ppq, Some(480));
        assert_eq!(fermata.fermata_ges.dur_real, Some(2.5));
    }

    #[test]
    fn fermata_deserializes_glyph_attributes() {
        use tusk_model::att::AttFermataVisGlyphAuth;

        let xml = r#"<fermata glyph.auth="smufl" glyph.name="fermataAbove"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            fermata.fermata_vis.glyph_auth,
            Some(AttFermataVisGlyphAuth::Smufl)
        );
        assert_eq!(
            fermata.fermata_vis.glyph_name,
            Some("fermataAbove".to_string())
        );
    }

    #[test]
    fn fermata_deserializes_visual_offset_attributes() {
        let xml = r#"<fermata ho="2" vo="-1" to="0.5"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.ho.is_some());
        assert!(fermata.fermata_vis.vo.is_some());
        assert!(fermata.fermata_vis.to.is_some());
    }

    #[test]
    fn fermata_deserializes_vgrp_attribute() {
        let xml = r#"<fermata vgrp="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.vgrp, Some(1));
    }

    #[test]
    fn fermata_handles_unknown_attributes_leniently() {
        let xml = r#"<fermata xml:id="f1" unknown="value"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    #[test]
    fn fermata_deserializes_all_common_attributes() {
        use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};

        let xml = r##"<fermata xml:id="f1" startid="#note1" staff="1" tstamp="2.5" form="norm" shape="curved" place="above" color="blue"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
        assert!(fermata.fermata_log.startid.is_some());
        assert_eq!(fermata.fermata_log.staff, vec![1]);
        assert!(fermata.fermata_log.tstamp.is_some());
        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Norm));
        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Curved));
        assert!(fermata.fermata_vis.place.is_some());
        assert!(fermata.fermata_vis.color.is_some());
    }

    #[test]
    fn fermata_deserializes_enclose_attribute() {
        let xml = r#"<fermata enclose="paren"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.enclose.is_some());
    }

    #[test]
    fn fermata_deserializes_altsym_attribute() {
        let xml = r##"<fermata altsym="#mySymbol"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.altsym.is_some());
    }

    #[test]
    fn fermata_deserializes_with_non_empty_element() {
        // Even though fermata has empty content, we handle non-empty elements gracefully
        let xml = r#"<fermata xml:id="f1">   </fermata>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    // ============================================================================
    // Reh (rehearsal mark) tests
    // ============================================================================

    #[test]
    fn reh_deserializes_basic_text() {
        use tusk_model::elements::{Reh, RehChild};

        let xml = r#"<reh xml:id="r1">A</reh>"#;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.common.xml_id, Some("r1".to_string()));
        assert_eq!(reh.children.len(), 1);
        match &reh.children[0] {
            RehChild::Text(text) => assert_eq!(text, "A"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn reh_deserializes_with_attributes() {
        use tusk_model::elements::Reh;

        let xml = r##"<reh xml:id="r1" staff="1" tstamp="1" place="above">1</reh>"##;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.common.xml_id, Some("r1".to_string()));
        assert_eq!(reh.reh_log.staff, vec![1]);
        assert!(reh.reh_log.tstamp.is_some());
        assert!(reh.reh_vis.place.is_some());
    }

    #[test]
    fn reh_deserializes_with_rend_child() {
        use tusk_model::elements::{Reh, RehChild};

        let xml = r#"<reh xml:id="r1"><rend fontweight="bold">A</rend></reh>"#;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.children.len(), 1);
        match &reh.children[0] {
            RehChild::Rend(rend) => {
                assert!(rend.typography.fontweight.is_some());
            }
            _ => panic!("Expected rend child"),
        }
    }

    #[test]
    fn reh_deserializes_empty_element() {
        use tusk_model::elements::Reh;

        let xml = r#"<reh xml:id="r1"/>"#;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.common.xml_id, Some("r1".to_string()));
        assert!(reh.children.is_empty());
    }
}

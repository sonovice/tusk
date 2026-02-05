//! Deserializer implementations for spanning/continuation elements:
//! BeamSpan, Octave, Gliss, Lv, BracketSpan, BTrem, FTrem.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBTremAnl, AttBTremGes, AttBTremLog, AttBTremVis, AttBeamSpanAnl, AttBeamSpanGes,
    AttBeamSpanLog, AttBeamSpanVis, AttBracketSpanAnl, AttBracketSpanGes, AttBracketSpanLog,
    AttBracketSpanVis, AttFTremAnl, AttFTremGes, AttFTremLog, AttFTremVis, AttGlissAnl,
    AttGlissGes, AttGlissLog, AttGlissVis, AttLvAnl, AttLvGes, AttLvLog, AttLvVis, AttOctaveAnl,
    AttOctaveGes, AttOctaveLog, AttOctaveVis,
};
use tusk_model::elements::{BTrem, BeamSpan, BracketSpan, FTrem, Gliss, Lv, Octave};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// BeamSpan attribute class implementations
// ============================================================================

impl ExtractAttributes for AttBeamSpanLog {
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
        extract_attr!(attrs, "beam.with", self.beam_with);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttBeamSpanVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "slash", self.slash);
        extract_attr!(attrs, "slope", self.slope);
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttBeamSpanGes {
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

impl ExtractAttributes for AttBeamSpanAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeamSpanAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for BeamSpan {
    fn element_name() -> &'static str {
        "beamSpan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut beam_span = BeamSpan::default();

        // Extract attributes into each attribute class
        beam_span.common.extract_attributes(&mut attrs)?;
        beam_span.facsimile.extract_attributes(&mut attrs)?;
        beam_span.beam_span_log.extract_attributes(&mut attrs)?;
        beam_span.beam_span_vis.extract_attributes(&mut attrs)?;
        beam_span.beam_span_ges.extract_attributes(&mut attrs)?;
        beam_span.beam_span_anl.extract_attributes(&mut attrs)?;

        // BeamSpan is an empty element per MEI spec
        if !is_empty {
            reader.skip_to_end("beamSpan")?;
        }

        Ok(beam_span)
    }
}

// ============================================================================
// Octave attribute class implementations
// ============================================================================

impl ExtractAttributes for AttOctaveLog {
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
        extract_attr!(attrs, "dis", self.dis);
        extract_attr!(attrs, "dis.place", self.dis_place);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "coll", self.coll);
        Ok(())
    }
}

impl ExtractAttributes for AttOctaveVis {
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

impl ExtractAttributes for AttOctaveGes {
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

impl ExtractAttributes for AttOctaveAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttOctaveAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Octave {
    fn element_name() -> &'static str {
        "octave"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut octave = Octave::default();

        // Extract attributes into each attribute class
        octave.common.extract_attributes(&mut attrs)?;
        octave.facsimile.extract_attributes(&mut attrs)?;
        octave.octave_log.extract_attributes(&mut attrs)?;
        octave.octave_vis.extract_attributes(&mut attrs)?;
        octave.octave_ges.extract_attributes(&mut attrs)?;
        octave.octave_anl.extract_attributes(&mut attrs)?;

        // Octave can have children but we skip them for now
        if !is_empty {
            reader.skip_to_end("octave")?;
        }

        Ok(octave)
    }
}

// ============================================================================
// Gliss attribute class implementations
// ============================================================================

impl ExtractAttributes for AttGlissLog {
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

impl ExtractAttributes for AttGlissVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
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
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        Ok(())
    }
}

impl ExtractAttributes for AttGlissGes {
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

impl ExtractAttributes for AttGlissAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttGlissAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Gliss {
    fn element_name() -> &'static str {
        "gliss"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut gliss = Gliss::default();

        // Extract attributes into each attribute class
        gliss.common.extract_attributes(&mut attrs)?;
        gliss.facsimile.extract_attributes(&mut attrs)?;
        gliss.gliss_log.extract_attributes(&mut attrs)?;
        gliss.gliss_vis.extract_attributes(&mut attrs)?;
        gliss.gliss_ges.extract_attributes(&mut attrs)?;
        gliss.gliss_anl.extract_attributes(&mut attrs)?;

        // Gliss can have children but we skip them for now
        if !is_empty {
            reader.skip_to_end("gliss")?;
        }

        Ok(gliss)
    }
}

// ============================================================================
// Lv attribute class implementations
// ============================================================================

impl ExtractAttributes for AttLvLog {
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

impl ExtractAttributes for AttLvVis {
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

impl ExtractAttributes for AttLvGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttLvAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLvAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Lv {
    fn element_name() -> &'static str {
        "lv"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut lv = Lv::default();

        // Extract attributes into each attribute class
        lv.common.extract_attributes(&mut attrs)?;
        lv.facsimile.extract_attributes(&mut attrs)?;
        lv.lv_log.extract_attributes(&mut attrs)?;
        lv.lv_vis.extract_attributes(&mut attrs)?;
        lv.lv_ges.extract_attributes(&mut attrs)?;
        lv.lv_anl.extract_attributes(&mut attrs)?;

        // Lv can have curve children but we skip them for now
        if !is_empty {
            reader.skip_to_end("lv")?;
        }

        Ok(lv)
    }
}

// ============================================================================
// BracketSpan attribute class implementations
// ============================================================================

impl ExtractAttributes for AttBracketSpanLog {
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
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttBracketSpanVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
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

impl ExtractAttributes for AttBracketSpanGes {
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

impl ExtractAttributes for AttBracketSpanAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBracketSpanAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for BracketSpan {
    fn element_name() -> &'static str {
        "bracketSpan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut bracket_span = BracketSpan::default();

        // Extract attributes into each attribute class
        bracket_span.common.extract_attributes(&mut attrs)?;
        bracket_span.facsimile.extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_log
            .extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_vis
            .extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_ges
            .extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_anl
            .extract_attributes(&mut attrs)?;

        // BracketSpan can have children but we skip them for now
        if !is_empty {
            reader.skip_to_end("bracketSpan")?;
        }

        Ok(bracket_span)
    }
}

// ============================================================================
// BTrem (bowed tremolo) attribute class implementations
// ============================================================================

impl ExtractAttributes for AttBTremLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl ExtractAttributes for AttBTremVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "num.place", self.num_place);
        extract_attr!(attrs, "num.visible", self.num_visible);
        Ok(())
    }
}

impl ExtractAttributes for AttBTremGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "unitdur", self.unitdur);
        Ok(())
    }
}

impl ExtractAttributes for AttBTremAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBTremAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for BTrem {
    fn element_name() -> &'static str {
        "bTrem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::{BTremChild, Chord, Note};

        let mut b_trem = BTrem::default();

        // Extract attributes into each attribute class
        b_trem.common.extract_attributes(&mut attrs)?;
        b_trem.facsimile.extract_attributes(&mut attrs)?;
        b_trem.b_trem_log.extract_attributes(&mut attrs)?;
        b_trem.b_trem_vis.extract_attributes(&mut attrs)?;
        b_trem.b_trem_ges.extract_attributes(&mut attrs)?;
        b_trem.b_trem_anl.extract_attributes(&mut attrs)?;

        // BTrem contains note or chord children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("bTrem")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        b_trem.children.push(BTremChild::Note(Box::new(note)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        b_trem.children.push(BTremChild::Chord(Box::new(chord)));
                    }
                    _ => {
                        // Skip unknown children
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(b_trem)
    }
}

// ============================================================================
// FTrem (fingered tremolo) attribute class implementations
// ============================================================================

impl ExtractAttributes for AttFTremLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl ExtractAttributes for AttFTremVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beams", self.beams);
        extract_attr!(attrs, "beams.float", self.beams_float);
        extract_attr!(attrs, "float.gap", self.float_gap);
        Ok(())
    }
}

impl ExtractAttributes for AttFTremGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "unitdur", self.unitdur);
        Ok(())
    }
}

impl ExtractAttributes for AttFTremAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFTremAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for FTrem {
    fn element_name() -> &'static str {
        "fTrem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::{Chord, FTremChild, Note};

        let mut f_trem = FTrem::default();

        // Extract attributes into each attribute class
        f_trem.common.extract_attributes(&mut attrs)?;
        f_trem.facsimile.extract_attributes(&mut attrs)?;
        f_trem.f_trem_log.extract_attributes(&mut attrs)?;
        f_trem.f_trem_vis.extract_attributes(&mut attrs)?;
        f_trem.f_trem_ges.extract_attributes(&mut attrs)?;
        f_trem.f_trem_anl.extract_attributes(&mut attrs)?;

        // FTrem contains note, chord, or clef children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("fTrem")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        f_trem.children.push(FTremChild::Note(Box::new(note)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        f_trem.children.push(FTremChild::Chord(Box::new(chord)));
                    }
                    _ => {
                        // Skip clef and unknown children (clef deserializer not yet implemented)
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(f_trem)
    }
}

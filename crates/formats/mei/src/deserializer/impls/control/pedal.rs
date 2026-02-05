//! Deserializer implementations for pedal and tuplet elements: Pedal, TupletSpan.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttPedalAnl, AttPedalGes, AttPedalLog, AttPedalVis, AttTupletSpanAnl, AttTupletSpanGes,
    AttTupletSpanLog, AttTupletSpanVis,
};
use tusk_model::elements::{Pedal, TupletSpan};

use super::super::{extract_attr, from_attr_string};

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

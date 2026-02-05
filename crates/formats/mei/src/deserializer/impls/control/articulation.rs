//! Deserializer implementations for articulation elements: Fermata, Arpeg, Breath, Caesura.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttArpegAnl, AttArpegGes, AttArpegLog, AttArpegVis, AttBreathAnl, AttBreathGes, AttBreathLog,
    AttBreathVis, AttCaesuraAnl, AttCaesuraGes, AttCaesuraLog, AttCaesuraVis, AttFermataAnl,
    AttFermataGes, AttFermataLog, AttFermataVis,
};
use tusk_model::elements::{Arpeg, Breath, Caesura, Fermata};

use super::super::{extract_attr, from_attr_string};

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
// Breath attribute class implementations
// ============================================================================

impl ExtractAttributes for AttBreathLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "tstamp", self.tstamp);
        Ok(())
    }
}

impl ExtractAttributes for AttBreathVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
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
        Ok(())
    }
}

impl ExtractAttributes for AttBreathGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttBreathAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBreathAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for Breath {
    fn element_name() -> &'static str {
        "breath"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut breath = Breath::default();

        // Extract attributes into each attribute class
        breath.common.extract_attributes(&mut attrs)?;
        breath.facsimile.extract_attributes(&mut attrs)?;
        breath.breath_log.extract_attributes(&mut attrs)?;
        breath.breath_vis.extract_attributes(&mut attrs)?;
        breath.breath_ges.extract_attributes(&mut attrs)?;
        breath.breath_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Breath has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("breath")?;
        }

        Ok(breath)
    }
}

// ============================================================================
// Caesura attribute class implementations
// ============================================================================

impl ExtractAttributes for AttCaesuraLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "tstamp", self.tstamp);
        Ok(())
    }
}

impl ExtractAttributes for AttCaesuraVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
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
        Ok(())
    }
}

impl ExtractAttributes for AttCaesuraGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", self.dur_recip);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttCaesuraAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttCaesuraAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for Caesura {
    fn element_name() -> &'static str {
        "caesura"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut caesura = Caesura::default();

        // Extract attributes into each attribute class
        caesura.common.extract_attributes(&mut attrs)?;
        caesura.facsimile.extract_attributes(&mut attrs)?;
        caesura.caesura_log.extract_attributes(&mut attrs)?;
        caesura.caesura_vis.extract_attributes(&mut attrs)?;
        caesura.caesura_ges.extract_attributes(&mut attrs)?;
        caesura.caesura_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Caesura has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("caesura")?;
        }

        Ok(caesura)
    }
}

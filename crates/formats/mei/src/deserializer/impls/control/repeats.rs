//! Deserializer implementations for repeat elements: RepeatMark, Volta, MRpt, MRpt2, BeatRpt,
//! HalfmRpt, MultiRpt, MultiRest, MSpace, MNum.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBeatRptAnl, AttBeatRptGes, AttBeatRptLog, AttBeatRptVis, AttHalfmRptAnl, AttHalfmRptGes,
    AttHalfmRptLog, AttHalfmRptVis, AttMNumAnl, AttMNumGes, AttMNumLog, AttMNumVis, AttMRpt2Anl,
    AttMRpt2Ges, AttMRpt2Log, AttMRpt2Vis, AttMRptAnl, AttMRptGes, AttMRptLog, AttMRptVis,
    AttMSpaceAnl, AttMSpaceGes, AttMSpaceLog, AttMSpaceVis, AttMultiRestAnl, AttMultiRestGes,
    AttMultiRestLog, AttMultiRestVis, AttMultiRptAnl, AttMultiRptGes, AttMultiRptLog,
    AttMultiRptVis, AttRepeatMarkAnl, AttRepeatMarkGes, AttRepeatMarkLog, AttRepeatMarkVis,
    AttVoltaAnl, AttVoltaGes, AttVoltaLog, AttVoltaVis,
};
use tusk_model::elements::{
    BeatRpt, HalfmRpt, MNum, MNumChild, MRpt, MRpt2, MSpace, MultiRest, MultiRpt, RepeatMark,
    RepeatMarkChild, Volta, VoltaChild,
};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// RepeatMark attribute class implementations
// ============================================================================

impl ExtractAttributes for AttRepeatMarkLog {
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

impl ExtractAttributes for AttRepeatMarkVis {
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
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
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

impl ExtractAttributes for AttRepeatMarkGes {
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

impl ExtractAttributes for AttRepeatMarkAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRepeatMarkAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for RepeatMark {
    fn element_name() -> &'static str {
        "repeatMark"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut repeat_mark = RepeatMark::default();

        // Extract attributes into each attribute class
        repeat_mark.common.extract_attributes(&mut attrs)?;
        repeat_mark.facsimile.extract_attributes(&mut attrs)?;
        repeat_mark.lang.extract_attributes(&mut attrs)?;
        repeat_mark.repeat_mark_log.extract_attributes(&mut attrs)?;
        repeat_mark.repeat_mark_vis.extract_attributes(&mut attrs)?;
        repeat_mark.repeat_mark_ges.extract_attributes(&mut attrs)?;
        repeat_mark.repeat_mark_anl.extract_attributes(&mut attrs)?;

        // RepeatMark can contain mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("repeatMark")? {
                match content {
                    MixedContent::Text(text) => {
                        repeat_mark.children.push(RepeatMarkChild::Text(text));
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            repeat_mark
                                .children
                                .push(RepeatMarkChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::text::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            repeat_mark.children.push(RepeatMarkChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Unknown/unsupported element - skip it
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    },
                }
            }
        }

        Ok(repeat_mark)
    }
}

// ============================================================================
// Volta attribute class implementations
// ============================================================================

impl ExtractAttributes for AttVoltaLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttVoltaLog has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttVoltaVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttVoltaGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttVoltaGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttVoltaAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttVoltaAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for Volta {
    fn element_name() -> &'static str {
        "volta"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut volta = Volta::default();

        // Extract attributes into each attribute class
        volta.common.extract_attributes(&mut attrs)?;
        volta.facsimile.extract_attributes(&mut attrs)?;
        volta.lang.extract_attributes(&mut attrs)?;
        volta.volta_log.extract_attributes(&mut attrs)?;
        volta.volta_vis.extract_attributes(&mut attrs)?;
        volta.volta_ges.extract_attributes(&mut attrs)?;
        volta.volta_anl.extract_attributes(&mut attrs)?;

        // Volta can contain child elements like dynam, dir, tempo, syl, etc.
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("volta")?
            {
                match name.as_str() {
                    "lb" => {
                        let lb = super::super::text::parse_lb_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        volta.children.push(VoltaChild::Lb(Box::new(lb)));
                    }
                    _ => {
                        // Unknown/unsupported element - skip it
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(volta)
    }
}

// ============================================================================
// MRpt attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMRptLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "num", self.num);
        Ok(())
    }
}

impl ExtractAttributes for AttMRptVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "expand", self.expand);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "num.place", self.num_place);
        extract_attr!(attrs, "num.visible", self.num_visible);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        Ok(())
    }
}

impl ExtractAttributes for AttMRptGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMRptGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttMRptAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMRptAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for MRpt {
    fn element_name() -> &'static str {
        "mRpt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut m_rpt = MRpt::default();

        // Extract attributes into each attribute class
        m_rpt.common.extract_attributes(&mut attrs)?;
        m_rpt.facsimile.extract_attributes(&mut attrs)?;
        m_rpt.m_rpt_log.extract_attributes(&mut attrs)?;
        m_rpt.m_rpt_vis.extract_attributes(&mut attrs)?;
        m_rpt.m_rpt_ges.extract_attributes(&mut attrs)?;
        m_rpt.m_rpt_anl.extract_attributes(&mut attrs)?;

        // MRpt has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("mRpt")?;
        }

        Ok(m_rpt)
    }
}

// ============================================================================
// MRpt2 attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMRpt2Log {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        Ok(())
    }
}

impl ExtractAttributes for AttMRpt2Vis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "expand", self.expand);
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
        Ok(())
    }
}

impl ExtractAttributes for AttMRpt2Ges {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMRpt2Ges has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttMRpt2Anl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMRpt2Anl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for MRpt2 {
    fn element_name() -> &'static str {
        "mRpt2"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut m_rpt2 = MRpt2::default();

        // Extract attributes into each attribute class
        m_rpt2.common.extract_attributes(&mut attrs)?;
        m_rpt2.facsimile.extract_attributes(&mut attrs)?;
        m_rpt2.m_rpt2_log.extract_attributes(&mut attrs)?;
        m_rpt2.m_rpt2_vis.extract_attributes(&mut attrs)?;
        m_rpt2.m_rpt2_ges.extract_attributes(&mut attrs)?;
        m_rpt2.m_rpt2_anl.extract_attributes(&mut attrs)?;

        // MRpt2 has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("mRpt2")?;
        }

        Ok(m_rpt2)
    }
}

// ============================================================================
// BeatRpt attribute class implementations
// ============================================================================

impl ExtractAttributes for AttBeatRptLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "beatdef", self.beatdef);
        Ok(())
    }
}

impl ExtractAttributes for AttBeatRptVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "expand", self.expand);
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
        extract_attr!(attrs, "slash", self.slash);
        Ok(())
    }
}

impl ExtractAttributes for AttBeatRptGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeatRptGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttBeatRptAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeatRptAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for BeatRpt {
    fn element_name() -> &'static str {
        "beatRpt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut beat_rpt = BeatRpt::default();

        // Extract attributes into each attribute class
        beat_rpt.common.extract_attributes(&mut attrs)?;
        beat_rpt.facsimile.extract_attributes(&mut attrs)?;
        beat_rpt.beat_rpt_log.extract_attributes(&mut attrs)?;
        beat_rpt.beat_rpt_vis.extract_attributes(&mut attrs)?;
        beat_rpt.beat_rpt_ges.extract_attributes(&mut attrs)?;
        beat_rpt.beat_rpt_anl.extract_attributes(&mut attrs)?;
        beat_rpt.plist.extract_attributes(&mut attrs)?;
        beat_rpt.target_eval.extract_attributes(&mut attrs)?;

        // BeatRpt has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("beatRpt")?;
        }

        Ok(beat_rpt)
    }
}

// ============================================================================
// HalfmRpt attribute class implementations
// ============================================================================

impl ExtractAttributes for AttHalfmRptLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "dur", vec self.dur);
        Ok(())
    }
}

impl ExtractAttributes for AttHalfmRptVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "expand", self.expand);
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
        Ok(())
    }
}

impl ExtractAttributes for AttHalfmRptGes {
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

impl ExtractAttributes for AttHalfmRptAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttHalfmRptAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for HalfmRpt {
    fn element_name() -> &'static str {
        "halfmRpt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut halfm_rpt = HalfmRpt::default();

        // Extract attributes into each attribute class
        halfm_rpt.common.extract_attributes(&mut attrs)?;
        halfm_rpt.facsimile.extract_attributes(&mut attrs)?;
        halfm_rpt.halfm_rpt_log.extract_attributes(&mut attrs)?;
        halfm_rpt.halfm_rpt_vis.extract_attributes(&mut attrs)?;
        halfm_rpt.halfm_rpt_ges.extract_attributes(&mut attrs)?;
        halfm_rpt.halfm_rpt_anl.extract_attributes(&mut attrs)?;

        // HalfmRpt has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("halfmRpt")?;
        }

        Ok(halfm_rpt)
    }
}

// ============================================================================
// MultiRpt attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMultiRptLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "num", self.num);
        Ok(())
    }
}

impl ExtractAttributes for AttMultiRptVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "expand", self.expand);
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
        Ok(())
    }
}

impl ExtractAttributes for AttMultiRptGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMultiRptGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttMultiRptAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMultiRptAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for MultiRpt {
    fn element_name() -> &'static str {
        "multiRpt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut multi_rpt = MultiRpt::default();

        // Extract attributes into each attribute class
        multi_rpt.common.extract_attributes(&mut attrs)?;
        multi_rpt.facsimile.extract_attributes(&mut attrs)?;
        multi_rpt.multi_rpt_log.extract_attributes(&mut attrs)?;
        multi_rpt.multi_rpt_vis.extract_attributes(&mut attrs)?;
        multi_rpt.multi_rpt_ges.extract_attributes(&mut attrs)?;
        multi_rpt.multi_rpt_anl.extract_attributes(&mut attrs)?;

        // MultiRpt has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("multiRpt")?;
        }

        Ok(multi_rpt)
    }
}

// ============================================================================
// MultiRest attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMultiRestLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "num", self.num);
        Ok(())
    }
}

impl ExtractAttributes for AttMultiRestVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "num.place", self.num_place);
        extract_attr!(attrs, "num.visible", self.num_visible);
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
        extract_attr!(attrs, "visible", self.visible);
        extract_attr!(attrs, "width", self.width);
        extract_attr!(attrs, "block", self.block);
        Ok(())
    }
}

impl ExtractAttributes for AttMultiRestGes {
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

impl ExtractAttributes for AttMultiRestAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMultiRestAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for MultiRest {
    fn element_name() -> &'static str {
        "multiRest"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut multi_rest = MultiRest::default();

        // Extract attributes into each attribute class
        multi_rest.common.extract_attributes(&mut attrs)?;
        multi_rest.facsimile.extract_attributes(&mut attrs)?;
        multi_rest.multi_rest_log.extract_attributes(&mut attrs)?;
        multi_rest.multi_rest_vis.extract_attributes(&mut attrs)?;
        multi_rest.multi_rest_ges.extract_attributes(&mut attrs)?;
        multi_rest.multi_rest_anl.extract_attributes(&mut attrs)?;

        // MultiRest has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("multiRest")?;
        }

        Ok(multi_rest)
    }
}

// ============================================================================
// MSpace attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMSpaceLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        Ok(())
    }
}

impl ExtractAttributes for AttMSpaceVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "cutout", self.cutout);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttMSpaceGes {
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

impl ExtractAttributes for AttMSpaceAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "fermata", self.fermata);
        Ok(())
    }
}

impl MeiDeserialize for MSpace {
    fn element_name() -> &'static str {
        "mSpace"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut m_space = MSpace::default();

        // Extract attributes into each attribute class
        m_space.common.extract_attributes(&mut attrs)?;
        m_space.facsimile.extract_attributes(&mut attrs)?;
        m_space.m_space_log.extract_attributes(&mut attrs)?;
        m_space.m_space_vis.extract_attributes(&mut attrs)?;
        m_space.m_space_ges.extract_attributes(&mut attrs)?;
        m_space.m_space_anl.extract_attributes(&mut attrs)?;

        // MSpace has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("mSpace")?;
        }

        Ok(m_space)
    }
}

// ============================================================================
// MNum attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMNumLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMNumLog has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttMNumVis {
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

impl ExtractAttributes for AttMNumGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMNumGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttMNumAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMNumAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for MNum {
    fn element_name() -> &'static str {
        "mNum"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut m_num = MNum::default();

        // Extract attributes into each attribute class
        m_num.common.extract_attributes(&mut attrs)?;
        m_num.facsimile.extract_attributes(&mut attrs)?;
        m_num.lang.extract_attributes(&mut attrs)?;
        m_num.m_num_log.extract_attributes(&mut attrs)?;
        m_num.m_num_vis.extract_attributes(&mut attrs)?;
        m_num.m_num_ges.extract_attributes(&mut attrs)?;
        m_num.m_num_anl.extract_attributes(&mut attrs)?;

        // MNum can contain mixed content (text, stack, lb, rend)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("mNum")? {
                match content {
                    MixedContent::Text(text) => {
                        m_num.children.push(MNumChild::Text(text));
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            m_num.children.push(MNumChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::text::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            m_num.children.push(MNumChild::Lb(Box::new(lb)));
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

        Ok(m_num)
    }
}

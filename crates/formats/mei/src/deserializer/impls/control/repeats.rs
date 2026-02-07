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

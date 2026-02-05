//! Serializer implementations for repeat elements: RepeatMark, Volta, MRpt, MRpt2, BeatRpt,
//! HalfmRpt, MultiRpt, MultiRest, MSpace, MNum.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBeatRptAnl, AttBeatRptGes, AttBeatRptLog, AttBeatRptVis, AttHalfmRptAnl, AttHalfmRptGes,
    AttHalfmRptLog, AttHalfmRptVis, AttMNumAnl, AttMNumGes, AttMNumLog, AttMNumVis, AttMRpt2Anl,
    AttMRpt2Ges, AttMRpt2Log, AttMRpt2Vis, AttMRptAnl, AttMRptGes, AttMRptLog, AttMRptVis,
    AttMSpaceAnl, AttMSpaceGes, AttMSpaceLog, AttMSpaceVis, AttMultiRptAnl, AttMultiRptGes,
    AttMultiRptLog, AttMultiRptVis, AttMultiRestAnl, AttMultiRestGes, AttMultiRestLog,
    AttMultiRestVis, AttRepeatMarkAnl, AttRepeatMarkGes, AttRepeatMarkLog, AttRepeatMarkVis,
    AttVoltaAnl, AttVoltaGes, AttVoltaLog, AttVoltaVis,
};
use tusk_model::elements::{
    BeatRpt, HalfmRpt, MNum, MNumChild, MRpt, MRpt2, MSpace, MultiRpt, MultiRest, RepeatMark,
    RepeatMarkChild, Volta, VoltaChild,
};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// RepeatMark attribute class implementations
// ============================================================================
impl CollectAttributes for AttRepeatMarkLog {
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

impl CollectAttributes for AttRepeatMarkVis {
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
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
        attrs
    }
}

impl CollectAttributes for AttRepeatMarkGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttRepeatMarkAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRepeatMarkAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for RepeatMark {
    fn element_name(&self) -> &'static str {
        "repeatMark"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.repeat_mark_log.collect_attributes());
        attrs.extend(self.repeat_mark_vis.collect_attributes());
        attrs.extend(self.repeat_mark_ges.collect_attributes());
        attrs.extend(self.repeat_mark_anl.collect_attributes());
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

impl MeiSerialize for RepeatMarkChild {
    fn element_name(&self) -> &'static str {
        match self {
            RepeatMarkChild::Text(_) => "",
            RepeatMarkChild::Rend(_) => "rend",
            RepeatMarkChild::Lb(_) => "lb",
            RepeatMarkChild::Symbol(_) => "symbol",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RepeatMarkChild::Rend(rend) => rend.collect_all_attributes(),
            RepeatMarkChild::Lb(lb) => lb.collect_all_attributes(),
            RepeatMarkChild::Symbol(symbol) => symbol.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RepeatMarkChild::Rend(rend) => rend.has_children(),
            RepeatMarkChild::Lb(lb) => lb.has_children(),
            RepeatMarkChild::Symbol(symbol) => symbol.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RepeatMarkChild::Rend(rend) => rend.serialize_children(writer),
            RepeatMarkChild::Lb(lb) => lb.serialize_children(writer),
            RepeatMarkChild::Symbol(symbol) => symbol.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RepeatMarkChild::Text(text) => writer.write_text(text),
            RepeatMarkChild::Rend(rend) => rend.serialize_mei(writer),
            RepeatMarkChild::Lb(lb) => lb.serialize_mei(writer),
            RepeatMarkChild::Symbol(symbol) => symbol.serialize_mei(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Volta attribute class implementations
// ============================================================================
impl CollectAttributes for AttVoltaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttVoltaLog has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttVoltaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttVoltaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttVoltaGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttVoltaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttVoltaAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Volta {
    fn element_name(&self) -> &'static str {
        "volta"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.volta_log.collect_attributes());
        attrs.extend(self.volta_vis.collect_attributes());
        attrs.extend(self.volta_ges.collect_attributes());
        attrs.extend(self.volta_anl.collect_attributes());
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

impl MeiSerialize for VoltaChild {
    fn element_name(&self) -> &'static str {
        match self {
            VoltaChild::Lb(_) => "lb",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            VoltaChild::Lb(lb) => lb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            VoltaChild::Lb(lb) => lb.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            VoltaChild::Lb(lb) => lb.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            VoltaChild::Lb(lb) => lb.serialize_mei(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// MRpt attribute class implementations
// ============================================================================
impl CollectAttributes for AttMRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "num", self.num);
        attrs
    }
}

impl CollectAttributes for AttMRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}

impl CollectAttributes for AttMRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMRptGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMRptAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for MRpt {
    fn element_name(&self) -> &'static str {
        "mRpt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.m_rpt_log.collect_attributes());
        attrs.extend(self.m_rpt_vis.collect_attributes());
        attrs.extend(self.m_rpt_ges.collect_attributes());
        attrs.extend(self.m_rpt_anl.collect_attributes());
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
// MRpt2 attribute class implementations
// ============================================================================
impl CollectAttributes for AttMRpt2Log {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttMRpt2Vis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
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
        attrs
    }
}

impl CollectAttributes for AttMRpt2Ges {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMRpt2Ges has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMRpt2Anl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMRpt2Anl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for MRpt2 {
    fn element_name(&self) -> &'static str {
        "mRpt2"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.m_rpt2_log.collect_attributes());
        attrs.extend(self.m_rpt2_vis.collect_attributes());
        attrs.extend(self.m_rpt2_ges.collect_attributes());
        attrs.extend(self.m_rpt2_anl.collect_attributes());
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
// BeatRpt attribute class implementations
// ============================================================================
impl CollectAttributes for AttBeatRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "beatdef", self.beatdef);
        attrs
    }
}

impl CollectAttributes for AttBeatRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
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
        push_attr!(attrs, "slash", self.slash);
        attrs
    }
}

impl CollectAttributes for AttBeatRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttBeatRptGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttBeatRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttBeatRptAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for BeatRpt {
    fn element_name(&self) -> &'static str {
        "beatRpt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.beat_rpt_log.collect_attributes());
        attrs.extend(self.beat_rpt_vis.collect_attributes());
        attrs.extend(self.beat_rpt_ges.collect_attributes());
        attrs.extend(self.beat_rpt_anl.collect_attributes());
        attrs.extend(self.plist.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
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
// HalfmRpt attribute class implementations
// ============================================================================
impl CollectAttributes for AttHalfmRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "dur", vec self.dur);
        attrs
    }
}

impl CollectAttributes for AttHalfmRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
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
        attrs
    }
}

impl CollectAttributes for AttHalfmRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttHalfmRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttHalfmRptAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for HalfmRpt {
    fn element_name(&self) -> &'static str {
        "halfmRpt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.halfm_rpt_log.collect_attributes());
        attrs.extend(self.halfm_rpt_vis.collect_attributes());
        attrs.extend(self.halfm_rpt_ges.collect_attributes());
        attrs.extend(self.halfm_rpt_anl.collect_attributes());
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
// MultiRpt attribute class implementations
// ============================================================================
impl CollectAttributes for AttMultiRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "num", self.num);
        attrs
    }
}

impl CollectAttributes for AttMultiRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
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
        attrs
    }
}

impl CollectAttributes for AttMultiRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMultiRptGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMultiRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMultiRptAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for MultiRpt {
    fn element_name(&self) -> &'static str {
        "multiRpt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.multi_rpt_log.collect_attributes());
        attrs.extend(self.multi_rpt_vis.collect_attributes());
        attrs.extend(self.multi_rpt_ges.collect_attributes());
        attrs.extend(self.multi_rpt_anl.collect_attributes());
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
// MultiRest attribute class implementations
// ============================================================================
impl CollectAttributes for AttMultiRestLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "num", self.num);
        attrs
    }
}

impl CollectAttributes for AttMultiRestVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "ploc", self.ploc);
        push_attr!(attrs, "oloc", self.oloc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "width", self.width);
        push_attr!(attrs, "block", self.block);
        attrs
    }
}

impl CollectAttributes for AttMultiRestGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttMultiRestAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMultiRestAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for MultiRest {
    fn element_name(&self) -> &'static str {
        "multiRest"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.multi_rest_log.collect_attributes());
        attrs.extend(self.multi_rest_vis.collect_attributes());
        attrs.extend(self.multi_rest_ges.collect_attributes());
        attrs.extend(self.multi_rest_anl.collect_attributes());
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
// MSpace attribute class implementations
// ============================================================================
impl CollectAttributes for AttMSpaceLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttMSpaceVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "cutout", self.cutout);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttMSpaceGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttMSpaceAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fermata", self.fermata);
        attrs
    }
}

impl MeiSerialize for MSpace {
    fn element_name(&self) -> &'static str {
        "mSpace"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.m_space_log.collect_attributes());
        attrs.extend(self.m_space_vis.collect_attributes());
        attrs.extend(self.m_space_ges.collect_attributes());
        attrs.extend(self.m_space_anl.collect_attributes());
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
// MNum attribute class implementations
// ============================================================================
impl CollectAttributes for AttMNumLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMNumLog has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMNumVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "place", self.place);
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

impl CollectAttributes for AttMNumGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMNumGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMNumAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMNumAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for MNum {
    fn element_name(&self) -> &'static str {
        "mNum"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.m_num_log.collect_attributes());
        attrs.extend(self.m_num_vis.collect_attributes());
        attrs.extend(self.m_num_ges.collect_attributes());
        attrs.extend(self.m_num_anl.collect_attributes());
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

impl MeiSerialize for MNumChild {
    fn element_name(&self) -> &'static str {
        match self {
            MNumChild::Text(_) => "",
            MNumChild::Rend(_) => "rend",
            MNumChild::Lb(_) => "lb",
            MNumChild::Stack(_) => "stack",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MNumChild::Rend(rend) => rend.collect_all_attributes(),
            MNumChild::Lb(lb) => lb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MNumChild::Rend(rend) => rend.has_children(),
            MNumChild::Lb(lb) => lb.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MNumChild::Rend(rend) => rend.serialize_children(writer),
            MNumChild::Lb(lb) => lb.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MNumChild::Text(text) => writer.write_text(text),
            MNumChild::Rend(rend) => rend.serialize_mei(writer),
            MNumChild::Lb(lb) => lb.serialize_mei(writer),
            MNumChild::Stack(_) => Ok(()), // Stack not yet implemented
        }
    }
}

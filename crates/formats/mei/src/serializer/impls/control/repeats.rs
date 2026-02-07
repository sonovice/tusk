//! Serializer implementations for repeat elements: RepeatMark, Volta, MRpt, MRpt2, BeatRpt,
//! HalfmRpt, MultiRpt, MultiRest, MSpace, MNum.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
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

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

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

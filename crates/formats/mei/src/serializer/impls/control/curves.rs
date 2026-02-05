//! Serializer implementations for curve-based control elements: Slur, Tie, Bend.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBendAnl, AttBendGes, AttBendLog, AttBendVis, AttSlurAnl, AttSlurGes, AttSlurLog, AttSlurVis,
    AttTieAnl, AttTieGes, AttTieLog, AttTieVis,
};
use tusk_model::elements::{Bend, Slur, SlurChild, Tie, TieChild};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

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
// Bend attribute class implementations
// ============================================================================

impl CollectAttributes for AttBendLog {
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

impl CollectAttributes for AttBendVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}

impl CollectAttributes for AttBendGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        push_attr!(attrs, "amount", self.amount);
        attrs
    }
}

impl CollectAttributes for AttBendAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttBendAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Bend {
    fn element_name(&self) -> &'static str {
        "bend"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.bend_log.collect_attributes());
        attrs.extend(self.bend_vis.collect_attributes());
        attrs.extend(self.bend_ges.collect_attributes());
        attrs.extend(self.bend_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

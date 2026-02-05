//! Serializer implementations for articulation elements: Fermata, Arpeg, Breath, Caesura.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttArpegAnl, AttArpegGes, AttArpegLog, AttArpegVis, AttBreathAnl, AttBreathGes, AttBreathLog,
    AttBreathVis, AttCaesuraAnl, AttCaesuraGes, AttCaesuraLog, AttCaesuraVis, AttFermataAnl,
    AttFermataGes, AttFermataLog, AttFermataVis,
};
use tusk_model::elements::{Arpeg, Breath, Caesura, Fermata};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Arpeg attribute class implementations
// ============================================================================

impl CollectAttributes for AttArpegLog {
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
        push_attr!(attrs, "order", self.order);
        attrs
    }
}

impl CollectAttributes for AttArpegVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
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
        push_attr!(attrs, "arrow", self.arrow);
        push_attr!(attrs, "arrow.shape", self.arrow_shape);
        push_attr!(attrs, "arrow.size", self.arrow_size);
        push_attr!(attrs, "arrow.color", self.arrow_color);
        push_attr!(attrs, "arrow.fillcolor", self.arrow_fillcolor);
        attrs
    }
}

impl CollectAttributes for AttArpegGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttArpegGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttArpegAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttArpegAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Arpeg {
    fn element_name(&self) -> &'static str {
        "arpeg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.arpeg_log.collect_attributes());
        attrs.extend(self.arpeg_vis.collect_attributes());
        attrs.extend(self.arpeg_ges.collect_attributes());
        attrs.extend(self.arpeg_anl.collect_attributes());
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
// Breath attribute class implementations
// ============================================================================

impl CollectAttributes for AttBreathLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttBreathVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttBreathGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}

impl CollectAttributes for AttBreathAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttBreathAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Breath {
    fn element_name(&self) -> &'static str {
        "breath"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.breath_log.collect_attributes());
        attrs.extend(self.breath_vis.collect_attributes());
        attrs.extend(self.breath_ges.collect_attributes());
        attrs.extend(self.breath_anl.collect_attributes());
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
// Caesura attribute class implementations
// ============================================================================

impl CollectAttributes for AttCaesuraLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttCaesuraVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttCaesuraGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}

impl CollectAttributes for AttCaesuraAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttCaesuraAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Caesura {
    fn element_name(&self) -> &'static str {
        "caesura"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.caesura_log.collect_attributes());
        attrs.extend(self.caesura_vis.collect_attributes());
        attrs.extend(self.caesura_ges.collect_attributes());
        attrs.extend(self.caesura_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

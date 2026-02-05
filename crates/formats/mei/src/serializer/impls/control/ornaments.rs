//! Serializer implementations for ornament elements: Trill, Mordent, Turn, Ornam.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttMordentAnl, AttMordentGes, AttMordentLog, AttMordentVis, AttOrnamAnl, AttOrnamGes,
    AttOrnamLog, AttOrnamVis, AttTrillAnl, AttTrillGes, AttTrillLog, AttTrillVis, AttTurnAnl,
    AttTurnGes, AttTurnLog, AttTurnVis,
};
use tusk_model::elements::{Mordent, Ornam, OrnamChild, Trill, Turn};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Trill attribute class implementations
// ============================================================================

impl CollectAttributes for AttTrillLog {
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
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttTrillVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
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
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttTrillGes {
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

impl CollectAttributes for AttTrillAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Trill {
    fn element_name(&self) -> &'static str {
        "trill"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.trill_log.collect_attributes());
        attrs.extend(self.trill_vis.collect_attributes());
        attrs.extend(self.trill_ges.collect_attributes());
        attrs.extend(self.trill_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Trill is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
// ============================================================================
// Mordent attribute class implementations
// ============================================================================

impl CollectAttributes for AttMordentLog {
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
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "long", self.long);
        attrs
    }
}

impl CollectAttributes for AttMordentVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        if let Some(v) = &self.glyph_name {
            attrs.push(("glyph.name", v.clone()));
        }
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        if let Some(v) = self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        attrs
    }
}

impl CollectAttributes for AttMordentGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMordentGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMordentAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMordentAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Mordent {
    fn element_name(&self) -> &'static str {
        "mordent"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.mordent_log.collect_attributes());
        attrs.extend(self.mordent_vis.collect_attributes());
        attrs.extend(self.mordent_ges.collect_attributes());
        attrs.extend(self.mordent_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Mordent is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
// ============================================================================
// Turn attribute class implementations
// ============================================================================

impl CollectAttributes for AttTurnLog {
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
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "delayed", self.delayed);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttTurnVis {
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttTurnGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttTurnGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttTurnAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttTurnAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Turn {
    fn element_name(&self) -> &'static str {
        "turn"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.turn_log.collect_attributes());
        attrs.extend(self.turn_vis.collect_attributes());
        attrs.extend(self.turn_ges.collect_attributes());
        attrs.extend(self.turn_anl.collect_attributes());
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
// Ornam attribute class implementations
// ============================================================================

impl CollectAttributes for AttOrnamLog {
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
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttOrnamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "place", self.place);
        if let Some(v) = self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        if let Some(v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttOrnamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttOrnamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

fn serialize_ornam_child<W: Write>(
    child: &OrnamChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        OrnamChild::Text(text) => writer.write_text(text),
        OrnamChild::BiblStruct(elem) => elem.serialize_mei(writer),
        OrnamChild::Unclear(elem) => elem.serialize_mei(writer),
        OrnamChild::District(elem) => elem.serialize_mei(writer),
        OrnamChild::Line(elem) => elem.serialize_mei(writer),
        OrnamChild::Title(elem) => elem.serialize_mei(writer),
        OrnamChild::Symbol(elem) => elem.serialize_mei(writer),
        OrnamChild::StyleName(elem) => elem.serialize_mei(writer),
        OrnamChild::PersName(elem) => elem.serialize_mei(writer),
        OrnamChild::Orig(elem) => elem.serialize_mei(writer),
        OrnamChild::Stack(elem) => elem.serialize_mei(writer),
        OrnamChild::CorpName(elem) => elem.serialize_mei(writer),
        OrnamChild::Seg(elem) => elem.serialize_mei(writer),
        OrnamChild::Ptr(elem) => elem.serialize_mei(writer),
        OrnamChild::PostCode(elem) => elem.serialize_mei(writer),
        OrnamChild::Supplied(elem) => elem.serialize_mei(writer),
        OrnamChild::RelationList(elem) => elem.serialize_mei(writer),
        OrnamChild::Date(elem) => elem.serialize_mei(writer),
        OrnamChild::Dim(elem) => elem.serialize_mei(writer),
        OrnamChild::Expan(elem) => elem.serialize_mei(writer),
        OrnamChild::Name(elem) => elem.serialize_mei(writer),
        OrnamChild::Annot(elem) => elem.serialize_mei(writer),
        OrnamChild::Heraldry(elem) => elem.serialize_mei(writer),
        OrnamChild::Settlement(elem) => elem.serialize_mei(writer),
        OrnamChild::PeriodName(elem) => elem.serialize_mei(writer),
        OrnamChild::Curve(elem) => elem.serialize_mei(writer),
        OrnamChild::Locus(elem) => elem.serialize_mei(writer),
        OrnamChild::Num(elem) => elem.serialize_mei(writer),
        OrnamChild::Corr(elem) => elem.serialize_mei(writer),
        OrnamChild::Ref(elem) => elem.serialize_mei(writer),
        OrnamChild::Country(elem) => elem.serialize_mei(writer),
        OrnamChild::Width(elem) => elem.serialize_mei(writer),
        OrnamChild::Fig(elem) => elem.serialize_mei(writer),
        OrnamChild::Restore(elem) => elem.serialize_mei(writer),
        OrnamChild::PostBox(elem) => elem.serialize_mei(writer),
        OrnamChild::Relation(elem) => elem.serialize_mei(writer),
        OrnamChild::Bloc(elem) => elem.serialize_mei(writer),
        OrnamChild::Sic(elem) => elem.serialize_mei(writer),
        OrnamChild::Extent(elem) => elem.serialize_mei(writer),
        OrnamChild::Region(elem) => elem.serialize_mei(writer),
        OrnamChild::Stamp(elem) => elem.serialize_mei(writer),
        OrnamChild::LocusGrp(elem) => elem.serialize_mei(writer),
        OrnamChild::Gap(elem) => elem.serialize_mei(writer),
        OrnamChild::Term(elem) => elem.serialize_mei(writer),
        OrnamChild::Abbr(elem) => elem.serialize_mei(writer),
        OrnamChild::Identifier(elem) => elem.serialize_mei(writer),
        OrnamChild::Address(elem) => elem.serialize_mei(writer),
        OrnamChild::Bibl(elem) => elem.serialize_mei(writer),
        OrnamChild::Choice(elem) => elem.serialize_mei(writer),
        OrnamChild::Subst(elem) => elem.serialize_mei(writer),
        OrnamChild::Depth(elem) => elem.serialize_mei(writer),
        OrnamChild::Q(elem) => elem.serialize_mei(writer),
        OrnamChild::Del(elem) => elem.serialize_mei(writer),
        OrnamChild::Add(elem) => elem.serialize_mei(writer),
        OrnamChild::HandShift(elem) => elem.serialize_mei(writer),
        OrnamChild::Reg(elem) => elem.serialize_mei(writer),
        OrnamChild::Street(elem) => elem.serialize_mei(writer),
        OrnamChild::Damage(elem) => elem.serialize_mei(writer),
        OrnamChild::SecFolio(elem) => elem.serialize_mei(writer),
        OrnamChild::Repository(elem) => elem.serialize_mei(writer),
        OrnamChild::GeogName(elem) => elem.serialize_mei(writer),
        OrnamChild::Catchwords(elem) => elem.serialize_mei(writer),
        OrnamChild::Rend(elem) => elem.serialize_mei(writer),
        OrnamChild::GeogFeat(elem) => elem.serialize_mei(writer),
        OrnamChild::Signatures(elem) => elem.serialize_mei(writer),
        OrnamChild::AnchoredText(elem) => elem.serialize_mei(writer),
        OrnamChild::Height(elem) => elem.serialize_mei(writer),
        OrnamChild::Dimensions(elem) => elem.serialize_mei(writer),
        OrnamChild::Lb(elem) => elem.serialize_mei(writer),
    }
}

impl MeiSerialize for Ornam {
    fn element_name(&self) -> &'static str {
        "ornam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.ornam_log.collect_attributes());
        attrs.extend(self.ornam_vis.collect_attributes());
        attrs.extend(self.ornam_ges.collect_attributes());
        attrs.extend(self.ornam_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            serialize_ornam_child(child, writer)?;
        }
        Ok(())
    }
}

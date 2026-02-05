//! Serializer implementations for dynamics elements: Dynam, Hairpin.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttDynamAnl, AttDynamGes, AttDynamLog, AttDynamVis, AttHairpinAnl, AttHairpinGes,
    AttHairpinLog, AttHairpinVis,
};
use tusk_model::elements::{Dynam, DynamChild, Hairpin};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Dynam attribute class implementations
// ============================================================================

impl CollectAttributes for AttDynamLog {
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

impl CollectAttributes for AttDynamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
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
        push_attr!(attrs, "place", self.place);
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

impl CollectAttributes for AttDynamGes {
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
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttDynamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Hairpin attribute class implementations
// ============================================================================

impl CollectAttributes for AttHairpinLog {
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
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "niente", self.niente);
        attrs
    }
}

impl CollectAttributes for AttHairpinVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "place", self.place);
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
        push_attr!(attrs, "opening", self.opening);
        push_attr!(attrs, "closed", self.closed);
        push_attr!(attrs, "opening.vertical", self.opening_vertical);
        push_attr!(attrs, "angle.optimize", self.angle_optimize);
        attrs
    }
}

impl CollectAttributes for AttHairpinGes {
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
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttHairpinAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Dynam {
    fn element_name(&self) -> &'static str {
        "dynam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.dynam_log.collect_attributes());
        attrs.extend(self.dynam_vis.collect_attributes());
        attrs.extend(self.dynam_ges.collect_attributes());
        attrs.extend(self.dynam_anl.collect_attributes());
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

impl MeiSerialize for DynamChild {
    fn element_name(&self) -> &'static str {
        match self {
            DynamChild::Text(_) => "$text",
            DynamChild::Rend(_) => "rend",
            DynamChild::Lb(_) => "lb",
            DynamChild::Ref(_) => "ref",
            DynamChild::PersName(_) => "persName",
            DynamChild::CorpName(_) => "corpName",
            DynamChild::Name(_) => "name",
            DynamChild::Seg(_) => "seg",
            DynamChild::Date(_) => "date",
            DynamChild::Identifier(_) => "identifier",
            DynamChild::Num(_) => "num",
            DynamChild::Ptr(_) => "ptr",
            DynamChild::Annot(_) => "annot",
            DynamChild::Title(_) => "title",
            // Other variants - return element name for error messages
            DynamChild::Stamp(_) => "stamp",
            DynamChild::Street(_) => "street",
            DynamChild::Gap(_) => "gap",
            DynamChild::Abbr(_) => "abbr",
            DynamChild::Sic(_) => "sic",
            DynamChild::PostBox(_) => "postBox",
            DynamChild::Q(_) => "q",
            DynamChild::Term(_) => "term",
            DynamChild::Corr(_) => "corr",
            DynamChild::PeriodName(_) => "periodName",
            DynamChild::BiblStruct(_) => "biblStruct",
            DynamChild::Signatures(_) => "signatures",
            DynamChild::Stack(_) => "stack",
            DynamChild::Unclear(_) => "unclear",
            DynamChild::Settlement(_) => "settlement",
            DynamChild::Depth(_) => "depth",
            DynamChild::Restore(_) => "restore",
            DynamChild::Dimensions(_) => "dimensions",
            DynamChild::PostCode(_) => "postCode",
            DynamChild::Damage(_) => "damage",
            DynamChild::Heraldry(_) => "heraldry",
            DynamChild::RelationList(_) => "relationList",
            DynamChild::Bloc(_) => "bloc",
            DynamChild::StyleName(_) => "styleName",
            DynamChild::Reg(_) => "reg",
            DynamChild::HandShift(_) => "handShift",
            DynamChild::Catchwords(_) => "catchwords",
            DynamChild::Country(_) => "country",
            DynamChild::Add(_) => "add",
            DynamChild::Bibl(_) => "bibl",
            DynamChild::LocusGrp(_) => "locusGrp",
            DynamChild::GeogFeat(_) => "geogFeat",
            DynamChild::Orig(_) => "orig",
            DynamChild::Height(_) => "height",
            DynamChild::Locus(_) => "locus",
            DynamChild::District(_) => "district",
            DynamChild::Expan(_) => "expan",
            DynamChild::GeogName(_) => "geogName",
            DynamChild::Relation(_) => "relation",
            DynamChild::Repository(_) => "repository",
            DynamChild::Del(_) => "del",
            DynamChild::Extent(_) => "extent",
            DynamChild::Width(_) => "width",
            DynamChild::Region(_) => "region",
            DynamChild::Symbol(_) => "symbol",
            DynamChild::Subst(_) => "subst",
            DynamChild::Supplied(_) => "supplied",
            DynamChild::Fig(_) => "fig",
            DynamChild::SecFolio(_) => "secFolio",
            DynamChild::Dim(_) => "dim",
            DynamChild::Address(_) => "address",
            DynamChild::Choice(_) => "choice",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            DynamChild::Text(_) => Vec::new(),
            DynamChild::Rend(elem) => elem.collect_all_attributes(),
            DynamChild::Lb(elem) => elem.collect_all_attributes(),
            DynamChild::Ref(elem) => elem.collect_all_attributes(),
            DynamChild::PersName(elem) => elem.collect_all_attributes(),
            DynamChild::CorpName(elem) => elem.collect_all_attributes(),
            DynamChild::Name(elem) => elem.collect_all_attributes(),
            DynamChild::Seg(elem) => elem.collect_all_attributes(),
            DynamChild::Date(elem) => elem.collect_all_attributes(),
            DynamChild::Identifier(elem) => elem.collect_all_attributes(),
            DynamChild::Num(elem) => elem.collect_all_attributes(),
            DynamChild::Ptr(elem) => elem.collect_all_attributes(),
            DynamChild::Annot(elem) => elem.collect_all_attributes(),
            DynamChild::Title(elem) => elem.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            DynamChild::Text(_) => false,
            DynamChild::Rend(elem) => elem.has_children(),
            DynamChild::Lb(_) => false,
            DynamChild::Ref(elem) => elem.has_children(),
            DynamChild::PersName(elem) => elem.has_children(),
            DynamChild::CorpName(elem) => elem.has_children(),
            DynamChild::Name(elem) => elem.has_children(),
            DynamChild::Seg(elem) => elem.has_children(),
            DynamChild::Date(elem) => elem.has_children(),
            DynamChild::Identifier(elem) => elem.has_children(),
            DynamChild::Num(elem) => elem.has_children(),
            DynamChild::Ptr(_) => false,
            DynamChild::Annot(elem) => elem.has_children(),
            DynamChild::Title(elem) => elem.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DynamChild::Text(_) => Ok(()),
            DynamChild::Rend(elem) => elem.serialize_children(writer),
            DynamChild::Lb(_) => Ok(()),
            DynamChild::Ref(elem) => elem.serialize_children(writer),
            DynamChild::PersName(elem) => elem.serialize_children(writer),
            DynamChild::CorpName(elem) => elem.serialize_children(writer),
            DynamChild::Name(elem) => elem.serialize_children(writer),
            DynamChild::Seg(elem) => elem.serialize_children(writer),
            DynamChild::Date(elem) => elem.serialize_children(writer),
            DynamChild::Identifier(elem) => elem.serialize_children(writer),
            DynamChild::Num(elem) => elem.serialize_children(writer),
            DynamChild::Ptr(_) => Ok(()),
            DynamChild::Annot(elem) => elem.serialize_children(writer),
            DynamChild::Title(elem) => elem.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DynamChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DynamChild::Rend(elem) => elem.serialize_mei(writer),
            DynamChild::Lb(elem) => elem.serialize_mei(writer),
            DynamChild::Ref(elem) => elem.serialize_mei(writer),
            DynamChild::PersName(elem) => elem.serialize_mei(writer),
            DynamChild::CorpName(elem) => elem.serialize_mei(writer),
            DynamChild::Name(elem) => elem.serialize_mei(writer),
            DynamChild::Seg(elem) => elem.serialize_mei(writer),
            DynamChild::Date(elem) => elem.serialize_mei(writer),
            DynamChild::Identifier(elem) => elem.serialize_mei(writer),
            DynamChild::Num(elem) => elem.serialize_mei(writer),
            DynamChild::Ptr(elem) => elem.serialize_mei(writer),
            DynamChild::Annot(elem) => elem.serialize_mei(writer),
            DynamChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DynamChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Hairpin {
    fn element_name(&self) -> &'static str {
        "hairpin"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hairpin_log.collect_attributes());
        attrs.extend(self.hairpin_vis.collect_attributes());
        attrs.extend(self.hairpin_ges.collect_attributes());
        attrs.extend(self.hairpin_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Hairpin is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

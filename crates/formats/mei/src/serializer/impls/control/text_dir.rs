//! Serializer implementations for text directive elements: Dir, Tempo.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBibl, AttDirAnl, AttDirGes, AttDirLog, AttDirVis, AttTempoAnl, AttTempoGes, AttTempoLog,
    AttTempoVis,
};
use tusk_model::elements::{Dir, DirChild, Tempo, TempoChild};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Dir attribute class implementations
// ============================================================================

impl CollectAttributes for AttDirLog {
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

impl CollectAttributes for AttDirVis {
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

impl CollectAttributes for AttDirGes {
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

impl CollectAttributes for AttDirAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Tempo attribute class implementations
// ============================================================================

impl CollectAttributes for AttTempoLog {
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
        push_attr!(attrs, "mm", self.mm);
        push_attr!(attrs, "mm.unit", self.mm_unit);
        push_attr!(attrs, "mm.dots", self.mm_dots);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttTempoVis {
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

impl CollectAttributes for AttTempoGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.bpm", self.midi_bpm);
        push_attr!(attrs, "midi.mspb", self.midi_mspb);
        attrs
    }
}

impl CollectAttributes for AttTempoAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Dir {
    fn element_name(&self) -> &'static str {
        "dir"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.dir_log.collect_attributes());
        attrs.extend(self.dir_vis.collect_attributes());
        attrs.extend(self.dir_ges.collect_attributes());
        attrs.extend(self.dir_anl.collect_attributes());
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

impl MeiSerialize for DirChild {
    fn element_name(&self) -> &'static str {
        match self {
            DirChild::Text(_) => "$text",
            DirChild::Rend(_) => "rend",
            DirChild::Lb(_) => "lb",
            DirChild::Ref(_) => "ref",
            DirChild::PersName(_) => "persName",
            DirChild::CorpName(_) => "corpName",
            DirChild::Name(_) => "name",
            DirChild::Date(_) => "date",
            DirChild::Title(_) => "title",
            DirChild::Identifier(_) => "identifier",
            DirChild::Num(_) => "num",
            DirChild::Ptr(_) => "ptr",
            DirChild::Annot(_) => "annot",
            // Other variants - return element name for error messages
            DirChild::Stack(_) => "stack",
            DirChild::RelationList(_) => "relationList",
            DirChild::Locus(_) => "locus",
            DirChild::Width(_) => "width",
            DirChild::Orig(_) => "orig",
            DirChild::Address(_) => "address",
            DirChild::Curve(_) => "curve",
            DirChild::Restore(_) => "restore",
            DirChild::Relation(_) => "relation",
            DirChild::Term(_) => "term",
            DirChild::Choice(_) => "choice",
            DirChild::PostBox(_) => "postBox",
            DirChild::Corr(_) => "corr",
            DirChild::GeogName(_) => "geogName",
            DirChild::Add(_) => "add",
            DirChild::Bloc(_) => "bloc",
            DirChild::AnchoredText(_) => "anchoredText",
            DirChild::Bibl(_) => "bibl",
            DirChild::Sic(_) => "sic",
            DirChild::BiblStruct(_) => "biblStruct",
            DirChild::Symbol(_) => "symbol",
            DirChild::Dim(_) => "dim",
            DirChild::Reg(_) => "reg",
            DirChild::PeriodName(_) => "periodName",
            DirChild::Subst(_) => "subst",
            DirChild::Unclear(_) => "unclear",
            DirChild::Height(_) => "height",
            DirChild::Street(_) => "street",
            DirChild::Stamp(_) => "stamp",
            DirChild::LocusGrp(_) => "locusGrp",
            DirChild::Del(_) => "del",
            DirChild::HandShift(_) => "handShift",
            DirChild::Depth(_) => "depth",
            DirChild::Heraldry(_) => "heraldry",
            DirChild::PostCode(_) => "postCode",
            DirChild::Catchwords(_) => "catchwords",
            DirChild::Line(_) => "line",
            DirChild::Region(_) => "region",
            DirChild::District(_) => "district",
            DirChild::Extent(_) => "extent",
            DirChild::Abbr(_) => "abbr",
            DirChild::Expan(_) => "expan",
            DirChild::SecFolio(_) => "secFolio",
            DirChild::Fig(_) => "fig",
            DirChild::GeogFeat(_) => "geogFeat",
            DirChild::Q(_) => "q",
            DirChild::Seg(_) => "seg",
            DirChild::Gap(_) => "gap",
            DirChild::StyleName(_) => "styleName",
            DirChild::Dimensions(_) => "dimensions",
            DirChild::Country(_) => "country",
            DirChild::Repository(_) => "repository",
            DirChild::Signatures(_) => "signatures",
            DirChild::Supplied(_) => "supplied",
            DirChild::Settlement(_) => "settlement",
            DirChild::Damage(_) => "damage",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            DirChild::Text(_) => Vec::new(),
            DirChild::Rend(elem) => elem.collect_all_attributes(),
            DirChild::Lb(elem) => elem.collect_all_attributes(),
            DirChild::Ref(elem) => elem.collect_all_attributes(),
            DirChild::PersName(elem) => elem.collect_all_attributes(),
            DirChild::CorpName(elem) => elem.collect_all_attributes(),
            DirChild::Name(elem) => elem.collect_all_attributes(),
            DirChild::Date(elem) => elem.collect_all_attributes(),
            DirChild::Title(elem) => elem.collect_all_attributes(),
            DirChild::Identifier(elem) => elem.collect_all_attributes(),
            DirChild::Num(elem) => elem.collect_all_attributes(),
            DirChild::Ptr(elem) => elem.collect_all_attributes(),
            DirChild::Annot(elem) => elem.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            DirChild::Text(_) => false,
            DirChild::Rend(elem) => elem.has_children(),
            DirChild::Lb(_) => false,
            DirChild::Ref(elem) => elem.has_children(),
            DirChild::PersName(elem) => elem.has_children(),
            DirChild::CorpName(elem) => elem.has_children(),
            DirChild::Name(elem) => elem.has_children(),
            DirChild::Date(elem) => elem.has_children(),
            DirChild::Title(elem) => elem.has_children(),
            DirChild::Identifier(elem) => elem.has_children(),
            DirChild::Num(elem) => elem.has_children(),
            DirChild::Ptr(_) => false,
            DirChild::Annot(elem) => elem.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DirChild::Text(_) => Ok(()),
            DirChild::Rend(elem) => elem.serialize_children(writer),
            DirChild::Lb(_) => Ok(()),
            DirChild::Ref(elem) => elem.serialize_children(writer),
            DirChild::PersName(elem) => elem.serialize_children(writer),
            DirChild::CorpName(elem) => elem.serialize_children(writer),
            DirChild::Name(elem) => elem.serialize_children(writer),
            DirChild::Date(elem) => elem.serialize_children(writer),
            DirChild::Title(elem) => elem.serialize_children(writer),
            DirChild::Identifier(elem) => elem.serialize_children(writer),
            DirChild::Num(elem) => elem.serialize_children(writer),
            DirChild::Ptr(_) => Ok(()),
            DirChild::Annot(elem) => elem.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DirChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DirChild::Rend(elem) => elem.serialize_mei(writer),
            DirChild::Lb(elem) => elem.serialize_mei(writer),
            DirChild::Ref(elem) => elem.serialize_mei(writer),
            DirChild::PersName(elem) => elem.serialize_mei(writer),
            DirChild::CorpName(elem) => elem.serialize_mei(writer),
            DirChild::Name(elem) => elem.serialize_mei(writer),
            DirChild::Date(elem) => elem.serialize_mei(writer),
            DirChild::Title(elem) => elem.serialize_mei(writer),
            DirChild::Identifier(elem) => elem.serialize_mei(writer),
            DirChild::Num(elem) => elem.serialize_mei(writer),
            DirChild::Ptr(elem) => elem.serialize_mei(writer),
            DirChild::Annot(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DirChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Tempo {
    fn element_name(&self) -> &'static str {
        "tempo"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.tempo_log.collect_attributes());
        attrs.extend(self.tempo_vis.collect_attributes());
        attrs.extend(self.tempo_ges.collect_attributes());
        attrs.extend(self.tempo_anl.collect_attributes());
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

impl MeiSerialize for TempoChild {
    fn element_name(&self) -> &'static str {
        match self {
            TempoChild::Text(_) => "$text",
            TempoChild::Rend(_) => "rend",
            TempoChild::Lb(_) => "lb",
            TempoChild::Ref(_) => "ref",
            TempoChild::PersName(_) => "persName",
            TempoChild::CorpName(_) => "corpName",
            TempoChild::Name(_) => "name",
            TempoChild::Date(_) => "date",
            TempoChild::Title(_) => "title",
            TempoChild::Identifier(_) => "identifier",
            TempoChild::Num(_) => "num",
            TempoChild::Ptr(_) => "ptr",
            TempoChild::Annot(_) => "annot",
            TempoChild::AnchoredText(_) => "anchoredText",
            TempoChild::Seg(_) => "seg",
            TempoChild::Symbol(_) => "symbol",
            // Other variants - return element name for error messages
            TempoChild::Stack(_) => "stack",
            TempoChild::RelationList(_) => "relationList",
            TempoChild::Locus(_) => "locus",
            TempoChild::Width(_) => "width",
            TempoChild::Orig(_) => "orig",
            TempoChild::Address(_) => "address",
            TempoChild::Curve(_) => "curve",
            TempoChild::Restore(_) => "restore",
            TempoChild::Relation(_) => "relation",
            TempoChild::Term(_) => "term",
            TempoChild::Choice(_) => "choice",
            TempoChild::PostBox(_) => "postBox",
            TempoChild::Corr(_) => "corr",
            TempoChild::GeogName(_) => "geogName",
            TempoChild::Add(_) => "add",
            TempoChild::Bloc(_) => "bloc",
            TempoChild::Bibl(_) => "bibl",
            TempoChild::Sic(_) => "sic",
            TempoChild::BiblStruct(_) => "biblStruct",
            TempoChild::Dim(_) => "dim",
            TempoChild::Reg(_) => "reg",
            TempoChild::PeriodName(_) => "periodName",
            TempoChild::Subst(_) => "subst",
            TempoChild::Unclear(_) => "unclear",
            TempoChild::Height(_) => "height",
            TempoChild::Street(_) => "street",
            TempoChild::Stamp(_) => "stamp",
            TempoChild::LocusGrp(_) => "locusGrp",
            TempoChild::Del(_) => "del",
            TempoChild::HandShift(_) => "handShift",
            TempoChild::Depth(_) => "depth",
            TempoChild::Heraldry(_) => "heraldry",
            TempoChild::PostCode(_) => "postCode",
            TempoChild::Catchwords(_) => "catchwords",
            TempoChild::Line(_) => "line",
            TempoChild::Region(_) => "region",
            TempoChild::District(_) => "district",
            TempoChild::Extent(_) => "extent",
            TempoChild::Abbr(_) => "abbr",
            TempoChild::Expan(_) => "expan",
            TempoChild::SecFolio(_) => "secFolio",
            TempoChild::Fig(_) => "fig",
            TempoChild::GeogFeat(_) => "geogFeat",
            TempoChild::Q(_) => "q",
            TempoChild::Gap(_) => "gap",
            TempoChild::StyleName(_) => "styleName",
            TempoChild::Dimensions(_) => "dimensions",
            TempoChild::Country(_) => "country",
            TempoChild::Repository(_) => "repository",
            TempoChild::Signatures(_) => "signatures",
            TempoChild::Supplied(_) => "supplied",
            TempoChild::Settlement(_) => "settlement",
            TempoChild::Damage(_) => "damage",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TempoChild::Text(_) => Vec::new(),
            TempoChild::Rend(elem) => elem.collect_all_attributes(),
            TempoChild::Lb(elem) => elem.collect_all_attributes(),
            TempoChild::Ref(elem) => elem.collect_all_attributes(),
            TempoChild::PersName(elem) => elem.collect_all_attributes(),
            TempoChild::CorpName(elem) => elem.collect_all_attributes(),
            TempoChild::Name(elem) => elem.collect_all_attributes(),
            TempoChild::Date(elem) => elem.collect_all_attributes(),
            TempoChild::Title(elem) => elem.collect_all_attributes(),
            TempoChild::Identifier(elem) => elem.collect_all_attributes(),
            TempoChild::Num(elem) => elem.collect_all_attributes(),
            TempoChild::Ptr(elem) => elem.collect_all_attributes(),
            TempoChild::Annot(elem) => elem.collect_all_attributes(),
            TempoChild::AnchoredText(elem) => elem.collect_all_attributes(),
            TempoChild::Seg(elem) => elem.collect_all_attributes(),
            TempoChild::Symbol(elem) => elem.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TempoChild::Text(_) => false,
            TempoChild::Rend(elem) => elem.has_children(),
            TempoChild::Lb(_) => false,
            TempoChild::Ref(elem) => elem.has_children(),
            TempoChild::PersName(elem) => elem.has_children(),
            TempoChild::CorpName(elem) => elem.has_children(),
            TempoChild::Name(elem) => elem.has_children(),
            TempoChild::Date(elem) => elem.has_children(),
            TempoChild::Title(elem) => elem.has_children(),
            TempoChild::Identifier(elem) => elem.has_children(),
            TempoChild::Num(elem) => elem.has_children(),
            TempoChild::Ptr(_) => false,
            TempoChild::Annot(elem) => elem.has_children(),
            TempoChild::AnchoredText(elem) => elem.has_children(),
            TempoChild::Seg(elem) => elem.has_children(),
            TempoChild::Symbol(elem) => elem.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TempoChild::Text(_) => Ok(()),
            TempoChild::Rend(elem) => elem.serialize_children(writer),
            TempoChild::Lb(_) => Ok(()),
            TempoChild::Ref(elem) => elem.serialize_children(writer),
            TempoChild::PersName(elem) => elem.serialize_children(writer),
            TempoChild::CorpName(elem) => elem.serialize_children(writer),
            TempoChild::Name(elem) => elem.serialize_children(writer),
            TempoChild::Date(elem) => elem.serialize_children(writer),
            TempoChild::Title(elem) => elem.serialize_children(writer),
            TempoChild::Identifier(elem) => elem.serialize_children(writer),
            TempoChild::Num(elem) => elem.serialize_children(writer),
            TempoChild::Ptr(_) => Ok(()),
            TempoChild::Annot(elem) => elem.serialize_children(writer),
            TempoChild::AnchoredText(elem) => elem.serialize_children(writer),
            TempoChild::Seg(elem) => elem.serialize_children(writer),
            TempoChild::Symbol(elem) => elem.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TempoChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            TempoChild::Rend(elem) => elem.serialize_mei(writer),
            TempoChild::Lb(elem) => elem.serialize_mei(writer),
            TempoChild::Ref(elem) => elem.serialize_mei(writer),
            TempoChild::PersName(elem) => elem.serialize_mei(writer),
            TempoChild::CorpName(elem) => elem.serialize_mei(writer),
            TempoChild::Name(elem) => elem.serialize_mei(writer),
            TempoChild::Date(elem) => elem.serialize_mei(writer),
            TempoChild::Title(elem) => elem.serialize_mei(writer),
            TempoChild::Identifier(elem) => elem.serialize_mei(writer),
            TempoChild::Num(elem) => elem.serialize_mei(writer),
            TempoChild::Ptr(elem) => elem.serialize_mei(writer),
            TempoChild::Annot(elem) => elem.serialize_mei(writer),
            TempoChild::AnchoredText(elem) => elem.serialize_mei(writer),
            TempoChild::Seg(elem) => elem.serialize_mei(writer),
            TempoChild::Symbol(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "TempoChild::{}",
                other.element_name()
            ))),
        }
    }
}

//! Serializer implementations for tablature notation MEI elements.
//!
//! This module contains implementations for TabGrp, TabDurSym, Fing, FingGrp,
//! String, Course, and Tuning elements used in string tablature notation.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttCourseAnl, AttCourseGes, AttCourseLog, AttCourseVis, AttFingAnl, AttFingGes, AttFingGrpAnl,
    AttFingGrpGes, AttFingGrpLog, AttFingGrpVis, AttFingLog, AttFingVis, AttStringtab,
    AttTabDurSymAnl, AttTabDurSymGes, AttTabDurSymLog, AttTabDurSymVis, AttTabGrpAnl, AttTabGrpGes,
    AttTabGrpLog, AttTabGrpVis, AttTuningAnl, AttTuningGes, AttTuningLog, AttTuningVis,
};
use tusk_model::elements::{
    Course, CourseChild, Fing, FingChild, FingGrp, FingGrpChild, String as MeiString, StringChild,
    TabDurSym, TabGrp, TabGrpChild, Tuning, TuningChild,
};

use super::push_attr;

// ============================================================================
// TabGrp attribute class implementations
// ============================================================================

// ============================================================================
// TabDurSym attribute class implementations
// ============================================================================

// ============================================================================
// Fing attribute class implementations
// ============================================================================

// ============================================================================
// FingGrp attribute class implementations
// ============================================================================

// ============================================================================
// Course/String attribute class implementations
// ============================================================================

// ============================================================================
// Tuning attribute class implementations
// ============================================================================

// ============================================================================
// TabGrp element implementation
// ============================================================================

impl MeiSerialize for TabGrp {
    fn element_name(&self) -> &'static str {
        "tabGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.tab_grp_log.collect_attributes());
        attrs.extend(self.tab_grp_vis.collect_attributes());
        attrs.extend(self.tab_grp_ges.collect_attributes());
        attrs.extend(self.tab_grp_anl.collect_attributes());
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

impl MeiSerialize for TabGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            TabGrpChild::Note(_) => "note",
            TabGrpChild::Rest(_) => "rest",
            TabGrpChild::TabDurSym(_) => "tabDurSym",
            TabGrpChild::Add(_) => "add",
            TabGrpChild::App(_) => "app",
            TabGrpChild::Choice(_) => "choice",
            TabGrpChild::Corr(_) => "corr",
            TabGrpChild::Damage(_) => "damage",
            TabGrpChild::Del(_) => "del",
            TabGrpChild::Gap(_) => "gap",
            TabGrpChild::HandShift(_) => "handShift",
            TabGrpChild::Orig(_) => "orig",
            TabGrpChild::Reg(_) => "reg",
            TabGrpChild::Restore(_) => "restore",
            TabGrpChild::Sic(_) => "sic",
            TabGrpChild::Subst(_) => "subst",
            TabGrpChild::Supplied(_) => "supplied",
            TabGrpChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TabGrpChild::Note(e) => e.collect_all_attributes(),
            TabGrpChild::Rest(e) => e.collect_all_attributes(),
            TabGrpChild::TabDurSym(e) => e.collect_all_attributes(),
            TabGrpChild::Add(e) => e.collect_all_attributes(),
            TabGrpChild::App(e) => e.collect_all_attributes(),
            TabGrpChild::Choice(e) => e.collect_all_attributes(),
            TabGrpChild::Corr(e) => e.collect_all_attributes(),
            TabGrpChild::Damage(e) => e.collect_all_attributes(),
            TabGrpChild::Del(e) => e.collect_all_attributes(),
            TabGrpChild::Gap(e) => e.collect_all_attributes(),
            TabGrpChild::HandShift(e) => e.collect_all_attributes(),
            TabGrpChild::Orig(e) => e.collect_all_attributes(),
            TabGrpChild::Reg(e) => e.collect_all_attributes(),
            TabGrpChild::Restore(e) => e.collect_all_attributes(),
            TabGrpChild::Sic(e) => e.collect_all_attributes(),
            TabGrpChild::Subst(e) => e.collect_all_attributes(),
            TabGrpChild::Supplied(e) => e.collect_all_attributes(),
            TabGrpChild::Unclear(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TabGrpChild::Note(e) => e.has_children(),
            TabGrpChild::Rest(e) => e.has_children(),
            TabGrpChild::TabDurSym(_) => false,
            TabGrpChild::Add(e) => e.has_children(),
            TabGrpChild::App(e) => e.has_children(),
            TabGrpChild::Choice(e) => e.has_children(),
            TabGrpChild::Corr(e) => e.has_children(),
            TabGrpChild::Damage(e) => e.has_children(),
            TabGrpChild::Del(e) => e.has_children(),
            TabGrpChild::Gap(_) => false,
            TabGrpChild::HandShift(_) => false,
            TabGrpChild::Orig(e) => e.has_children(),
            TabGrpChild::Reg(e) => e.has_children(),
            TabGrpChild::Restore(e) => e.has_children(),
            TabGrpChild::Sic(e) => e.has_children(),
            TabGrpChild::Subst(e) => e.has_children(),
            TabGrpChild::Supplied(e) => e.has_children(),
            TabGrpChild::Unclear(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TabGrpChild::Note(e) => e.serialize_children(writer),
            TabGrpChild::Rest(e) => e.serialize_children(writer),
            TabGrpChild::TabDurSym(_) => Ok(()),
            TabGrpChild::Add(e) => e.serialize_children(writer),
            TabGrpChild::App(e) => e.serialize_children(writer),
            TabGrpChild::Choice(e) => e.serialize_children(writer),
            TabGrpChild::Corr(e) => e.serialize_children(writer),
            TabGrpChild::Damage(e) => e.serialize_children(writer),
            TabGrpChild::Del(e) => e.serialize_children(writer),
            TabGrpChild::Gap(_) => Ok(()),
            TabGrpChild::HandShift(_) => Ok(()),
            TabGrpChild::Orig(e) => e.serialize_children(writer),
            TabGrpChild::Reg(e) => e.serialize_children(writer),
            TabGrpChild::Restore(e) => e.serialize_children(writer),
            TabGrpChild::Sic(e) => e.serialize_children(writer),
            TabGrpChild::Subst(e) => e.serialize_children(writer),
            TabGrpChild::Supplied(e) => e.serialize_children(writer),
            TabGrpChild::Unclear(e) => e.serialize_children(writer),
        }
    }
}

// ============================================================================
// TabDurSym element implementation
// ============================================================================

impl MeiSerialize for TabDurSym {
    fn element_name(&self) -> &'static str {
        "tabDurSym"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.stringtab.collect_attributes());
        attrs.extend(self.tab_dur_sym_log.collect_attributes());
        attrs.extend(self.tab_dur_sym_vis.collect_attributes());
        attrs.extend(self.tab_dur_sym_ges.collect_attributes());
        attrs.extend(self.tab_dur_sym_anl.collect_attributes());
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
// Fing element implementation
// ============================================================================

impl MeiSerialize for Fing {
    fn element_name(&self) -> &'static str {
        "fing"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.fing_anl.collect_attributes());
        attrs.extend(self.fing_ges.collect_attributes());
        attrs.extend(self.fing_log.collect_attributes());
        attrs.extend(self.fing_vis.collect_attributes());
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

impl MeiSerialize for FingChild {
    fn element_name(&self) -> &'static str {
        match self {
            FingChild::Text(_) => "#text",
            FingChild::Rend(_) => "rend",
            FingChild::Lb(_) => "lb",
            FingChild::Seg(_) => "seg",
            FingChild::Num(_) => "num",
            FingChild::Symbol(_) => "symbol",
            FingChild::Ref(_) => "ref",
            FingChild::Ptr(_) => "ptr",
            FingChild::Fig(_) => "fig",
            FingChild::Annot(_) => "annot",
            FingChild::Name(_) => "name",
            FingChild::Term(_) => "term",
            FingChild::Title(_) => "title",
            FingChild::Q(_) => "q",
            FingChild::Stack(_) => "stack",
            FingChild::Add(_) => "add",
            FingChild::Choice(_) => "choice",
            FingChild::Corr(_) => "corr",
            FingChild::Damage(_) => "damage",
            FingChild::Del(_) => "del",
            FingChild::Gap(_) => "gap",
            FingChild::HandShift(_) => "handShift",
            FingChild::Orig(_) => "orig",
            FingChild::Reg(_) => "reg",
            FingChild::Restore(_) => "restore",
            FingChild::Sic(_) => "sic",
            FingChild::Subst(_) => "subst",
            FingChild::Supplied(_) => "supplied",
            FingChild::Unclear(_) => "unclear",
            // These children have deserializers but may not all be used commonly
            FingChild::Identifier(_) => "identifier",
            FingChild::Date(_) => "date",
            FingChild::Repository(_) => "repository",
            FingChild::Catchwords(_) => "catchwords",
            FingChild::Locus(_) => "locus",
            FingChild::RelationList(_) => "relationList",
            FingChild::Height(_) => "height",
            FingChild::Street(_) => "street",
            FingChild::SecFolio(_) => "secFolio",
            FingChild::Abbr(_) => "abbr",
            FingChild::GeogFeat(_) => "geogFeat",
            FingChild::Expan(_) => "expan",
            FingChild::Settlement(_) => "settlement",
            FingChild::LocusGrp(_) => "locusGrp",
            FingChild::Country(_) => "country",
            FingChild::Address(_) => "address",
            FingChild::Dim(_) => "dim",
            FingChild::GeogName(_) => "geogName",
            FingChild::PostCode(_) => "postCode",
            FingChild::Depth(_) => "depth",
            FingChild::Heraldry(_) => "heraldry",
            FingChild::PersName(_) => "persName",
            FingChild::Stamp(_) => "stamp",
            FingChild::Bloc(_) => "bloc",
            FingChild::CorpName(_) => "corpName",
            FingChild::PostBox(_) => "postBox",
            FingChild::Relation(_) => "relation",
            FingChild::BiblStruct(_) => "biblStruct",
            FingChild::Bibl(_) => "bibl",
            FingChild::Dimensions(_) => "dimensions",
            FingChild::Region(_) => "region",
            FingChild::District(_) => "district",
            FingChild::PeriodName(_) => "periodName",
            FingChild::Signatures(_) => "signatures",
            FingChild::Width(_) => "width",
            FingChild::StyleName(_) => "styleName",
            FingChild::Extent(_) => "extent",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FingChild::Text(_) => Vec::new(),
            FingChild::Rend(e) => e.collect_all_attributes(),
            FingChild::Lb(e) => e.collect_all_attributes(),
            FingChild::Seg(e) => e.collect_all_attributes(),
            FingChild::Num(e) => e.collect_all_attributes(),
            FingChild::Symbol(e) => e.collect_all_attributes(),
            FingChild::Ref(e) => e.collect_all_attributes(),
            FingChild::Ptr(e) => e.collect_all_attributes(),
            FingChild::Fig(e) => e.collect_all_attributes(),
            FingChild::Annot(e) => e.collect_all_attributes(),
            FingChild::Name(e) => e.collect_all_attributes(),
            FingChild::Term(e) => e.collect_all_attributes(),
            FingChild::Title(e) => e.collect_all_attributes(),
            FingChild::Q(e) => e.collect_all_attributes(),
            FingChild::Stack(e) => e.collect_all_attributes(),
            FingChild::Add(e) => e.collect_all_attributes(),
            FingChild::Choice(e) => e.collect_all_attributes(),
            FingChild::Corr(e) => e.collect_all_attributes(),
            FingChild::Damage(e) => e.collect_all_attributes(),
            FingChild::Del(e) => e.collect_all_attributes(),
            FingChild::Gap(e) => e.collect_all_attributes(),
            FingChild::HandShift(e) => e.collect_all_attributes(),
            FingChild::Orig(e) => e.collect_all_attributes(),
            FingChild::Reg(e) => e.collect_all_attributes(),
            FingChild::Restore(e) => e.collect_all_attributes(),
            FingChild::Sic(e) => e.collect_all_attributes(),
            FingChild::Subst(e) => e.collect_all_attributes(),
            FingChild::Supplied(e) => e.collect_all_attributes(),
            FingChild::Unclear(e) => e.collect_all_attributes(),
            // Remaining child types
            FingChild::Identifier(e) => e.collect_all_attributes(),
            FingChild::Date(e) => e.collect_all_attributes(),
            FingChild::Repository(_) => Vec::new(),
            FingChild::Catchwords(_) => Vec::new(),
            FingChild::Locus(e) => e.collect_all_attributes(),
            FingChild::RelationList(_) => Vec::new(),
            FingChild::Height(_) => Vec::new(),
            FingChild::Street(e) => e.collect_all_attributes(),
            FingChild::SecFolio(_) => Vec::new(),
            FingChild::Abbr(e) => e.collect_all_attributes(),
            FingChild::GeogFeat(e) => e.collect_all_attributes(),
            FingChild::Expan(e) => e.collect_all_attributes(),
            FingChild::Settlement(e) => e.collect_all_attributes(),
            FingChild::LocusGrp(e) => e.collect_all_attributes(),
            FingChild::Country(e) => e.collect_all_attributes(),
            FingChild::Address(e) => e.collect_all_attributes(),
            FingChild::Dim(_) => Vec::new(),
            FingChild::GeogName(e) => e.collect_all_attributes(),
            FingChild::PostCode(e) => e.collect_all_attributes(),
            FingChild::Depth(_) => Vec::new(),
            FingChild::Heraldry(_) => Vec::new(),
            FingChild::PersName(e) => e.collect_all_attributes(),
            FingChild::Stamp(e) => e.collect_all_attributes(),
            FingChild::Bloc(e) => e.collect_all_attributes(),
            FingChild::CorpName(e) => e.collect_all_attributes(),
            FingChild::PostBox(e) => e.collect_all_attributes(),
            FingChild::Relation(_) => Vec::new(),
            FingChild::BiblStruct(e) => e.collect_all_attributes(),
            FingChild::Bibl(e) => e.collect_all_attributes(),
            FingChild::Dimensions(_) => Vec::new(),
            FingChild::Region(e) => e.collect_all_attributes(),
            FingChild::District(e) => e.collect_all_attributes(),
            FingChild::PeriodName(_) => Vec::new(),
            FingChild::Signatures(_) => Vec::new(),
            FingChild::Width(_) => Vec::new(),
            FingChild::StyleName(_) => Vec::new(),
            FingChild::Extent(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FingChild::Text(_) => false,
            FingChild::Rend(e) => e.has_children(),
            FingChild::Lb(_) => false,
            FingChild::Seg(e) => e.has_children(),
            FingChild::Num(e) => e.has_children(),
            FingChild::Symbol(_) => false,
            FingChild::Ref(e) => e.has_children(),
            FingChild::Ptr(_) => false,
            FingChild::Fig(e) => e.has_children(),
            FingChild::Annot(e) => e.has_children(),
            FingChild::Name(e) => e.has_children(),
            FingChild::Term(e) => e.has_children(),
            FingChild::Title(e) => e.has_children(),
            FingChild::Q(e) => e.has_children(),
            FingChild::Stack(e) => e.has_children(),
            FingChild::Add(e) => e.has_children(),
            FingChild::Choice(e) => e.has_children(),
            FingChild::Corr(e) => e.has_children(),
            FingChild::Damage(e) => e.has_children(),
            FingChild::Del(e) => e.has_children(),
            FingChild::Gap(_) => false,
            FingChild::HandShift(_) => false,
            FingChild::Orig(e) => e.has_children(),
            FingChild::Reg(e) => e.has_children(),
            FingChild::Restore(e) => e.has_children(),
            FingChild::Sic(e) => e.has_children(),
            FingChild::Subst(e) => e.has_children(),
            FingChild::Supplied(e) => e.has_children(),
            FingChild::Unclear(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FingChild::Text(_) => Ok(()),
            FingChild::Rend(e) => e.serialize_children(writer),
            FingChild::Lb(_) => Ok(()),
            FingChild::Seg(e) => e.serialize_children(writer),
            FingChild::Num(e) => e.serialize_children(writer),
            FingChild::Symbol(_) => Ok(()),
            FingChild::Ref(e) => e.serialize_children(writer),
            FingChild::Ptr(_) => Ok(()),
            FingChild::Fig(e) => e.serialize_children(writer),
            FingChild::Annot(e) => e.serialize_children(writer),
            FingChild::Name(e) => e.serialize_children(writer),
            FingChild::Term(e) => e.serialize_children(writer),
            FingChild::Title(e) => e.serialize_children(writer),
            FingChild::Q(e) => e.serialize_children(writer),
            FingChild::Stack(e) => e.serialize_children(writer),
            FingChild::Add(e) => e.serialize_children(writer),
            FingChild::Choice(e) => e.serialize_children(writer),
            FingChild::Corr(e) => e.serialize_children(writer),
            FingChild::Damage(e) => e.serialize_children(writer),
            FingChild::Del(e) => e.serialize_children(writer),
            FingChild::Gap(_) => Ok(()),
            FingChild::HandShift(_) => Ok(()),
            FingChild::Orig(e) => e.serialize_children(writer),
            FingChild::Reg(e) => e.serialize_children(writer),
            FingChild::Restore(e) => e.serialize_children(writer),
            FingChild::Sic(e) => e.serialize_children(writer),
            FingChild::Subst(e) => e.serialize_children(writer),
            FingChild::Supplied(e) => e.serialize_children(writer),
            FingChild::Unclear(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FingChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            FingChild::Rend(e) => e.serialize_mei(writer),
            FingChild::Lb(e) => e.serialize_mei(writer),
            FingChild::Seg(e) => e.serialize_mei(writer),
            FingChild::Num(e) => e.serialize_mei(writer),
            FingChild::Add(e) => e.serialize_mei(writer),
            FingChild::Choice(e) => e.serialize_mei(writer),
            FingChild::Corr(e) => e.serialize_mei(writer),
            FingChild::Damage(e) => e.serialize_mei(writer),
            FingChild::Del(e) => e.serialize_mei(writer),
            FingChild::Gap(e) => e.serialize_mei(writer),
            FingChild::HandShift(e) => e.serialize_mei(writer),
            FingChild::Orig(e) => e.serialize_mei(writer),
            FingChild::Reg(e) => e.serialize_mei(writer),
            FingChild::Restore(e) => e.serialize_mei(writer),
            FingChild::Sic(e) => e.serialize_mei(writer),
            FingChild::Subst(e) => e.serialize_mei(writer),
            FingChild::Supplied(e) => e.serialize_mei(writer),
            FingChild::Unclear(e) => e.serialize_mei(writer),
            _ => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FingChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// FingGrp element implementation
// ============================================================================

impl MeiSerialize for FingGrp {
    fn element_name(&self) -> &'static str {
        "fingGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.fing_grp_anl.collect_attributes());
        attrs.extend(self.fing_grp_ges.collect_attributes());
        attrs.extend(self.fing_grp_log.collect_attributes());
        attrs.extend(self.fing_grp_vis.collect_attributes());
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

impl MeiSerialize for FingGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            FingGrpChild::Fing(_) => "fing",
            FingGrpChild::FingGrp(_) => "fingGrp",
            FingGrpChild::Add(_) => "add",
            FingGrpChild::Choice(_) => "choice",
            FingGrpChild::Corr(_) => "corr",
            FingGrpChild::Damage(_) => "damage",
            FingGrpChild::Del(_) => "del",
            FingGrpChild::Gap(_) => "gap",
            FingGrpChild::HandShift(_) => "handShift",
            FingGrpChild::Orig(_) => "orig",
            FingGrpChild::Reg(_) => "reg",
            FingGrpChild::Restore(_) => "restore",
            FingGrpChild::Sic(_) => "sic",
            FingGrpChild::Subst(_) => "subst",
            FingGrpChild::Supplied(_) => "supplied",
            FingGrpChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FingGrpChild::Fing(e) => e.collect_all_attributes(),
            FingGrpChild::FingGrp(e) => e.collect_all_attributes(),
            FingGrpChild::Add(e) => e.collect_all_attributes(),
            FingGrpChild::Choice(e) => e.collect_all_attributes(),
            FingGrpChild::Corr(e) => e.collect_all_attributes(),
            FingGrpChild::Damage(e) => e.collect_all_attributes(),
            FingGrpChild::Del(e) => e.collect_all_attributes(),
            FingGrpChild::Gap(e) => e.collect_all_attributes(),
            FingGrpChild::HandShift(e) => e.collect_all_attributes(),
            FingGrpChild::Orig(e) => e.collect_all_attributes(),
            FingGrpChild::Reg(e) => e.collect_all_attributes(),
            FingGrpChild::Restore(e) => e.collect_all_attributes(),
            FingGrpChild::Sic(e) => e.collect_all_attributes(),
            FingGrpChild::Subst(e) => e.collect_all_attributes(),
            FingGrpChild::Supplied(e) => e.collect_all_attributes(),
            FingGrpChild::Unclear(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FingGrpChild::Fing(e) => e.has_children(),
            FingGrpChild::FingGrp(e) => e.has_children(),
            FingGrpChild::Add(e) => e.has_children(),
            FingGrpChild::Choice(e) => e.has_children(),
            FingGrpChild::Corr(e) => e.has_children(),
            FingGrpChild::Damage(e) => e.has_children(),
            FingGrpChild::Del(e) => e.has_children(),
            FingGrpChild::Gap(_) => false,
            FingGrpChild::HandShift(_) => false,
            FingGrpChild::Orig(e) => e.has_children(),
            FingGrpChild::Reg(e) => e.has_children(),
            FingGrpChild::Restore(e) => e.has_children(),
            FingGrpChild::Sic(e) => e.has_children(),
            FingGrpChild::Subst(e) => e.has_children(),
            FingGrpChild::Supplied(e) => e.has_children(),
            FingGrpChild::Unclear(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FingGrpChild::Fing(e) => e.serialize_children(writer),
            FingGrpChild::FingGrp(e) => e.serialize_children(writer),
            FingGrpChild::Add(e) => e.serialize_children(writer),
            FingGrpChild::Choice(e) => e.serialize_children(writer),
            FingGrpChild::Corr(e) => e.serialize_children(writer),
            FingGrpChild::Damage(e) => e.serialize_children(writer),
            FingGrpChild::Del(e) => e.serialize_children(writer),
            FingGrpChild::Gap(_) => Ok(()),
            FingGrpChild::HandShift(_) => Ok(()),
            FingGrpChild::Orig(e) => e.serialize_children(writer),
            FingGrpChild::Reg(e) => e.serialize_children(writer),
            FingGrpChild::Restore(e) => e.serialize_children(writer),
            FingGrpChild::Sic(e) => e.serialize_children(writer),
            FingGrpChild::Subst(e) => e.serialize_children(writer),
            FingGrpChild::Supplied(e) => e.serialize_children(writer),
            FingGrpChild::Unclear(e) => e.serialize_children(writer),
        }
    }
}

// ============================================================================
// String element implementation
// ============================================================================

impl MeiSerialize for MeiString {
    fn element_name(&self) -> &'static str {
        "string"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.course_log.collect_attributes());
        attrs.extend(self.course_vis.collect_attributes());
        attrs.extend(self.course_ges.collect_attributes());
        attrs.extend(self.course_anl.collect_attributes());
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

impl MeiSerialize for StringChild {
    fn element_name(&self) -> &'static str {
        match self {
            StringChild::String(_) => "string",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            StringChild::String(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            StringChild::String(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StringChild::String(e) => e.serialize_children(writer),
        }
    }
}

// ============================================================================
// Course element implementation
// ============================================================================

impl MeiSerialize for Course {
    fn element_name(&self) -> &'static str {
        "course"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.course_log.collect_attributes());
        attrs.extend(self.course_vis.collect_attributes());
        attrs.extend(self.course_ges.collect_attributes());
        attrs.extend(self.course_anl.collect_attributes());
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

impl MeiSerialize for CourseChild {
    fn element_name(&self) -> &'static str {
        match self {
            CourseChild::String(_) => "string",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            CourseChild::String(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            CourseChild::String(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CourseChild::String(e) => e.serialize_children(writer),
        }
    }
}

// ============================================================================
// Tuning element implementation
// ============================================================================

impl MeiSerialize for Tuning {
    fn element_name(&self) -> &'static str {
        "tuning"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.tuning_log.collect_attributes());
        attrs.extend(self.tuning_vis.collect_attributes());
        attrs.extend(self.tuning_ges.collect_attributes());
        attrs.extend(self.tuning_anl.collect_attributes());
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

impl MeiSerialize for TuningChild {
    fn element_name(&self) -> &'static str {
        match self {
            TuningChild::Course(_) => "course",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TuningChild::Course(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TuningChild::Course(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TuningChild::Course(e) => e.serialize_children(writer),
        }
    }
}

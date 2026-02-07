//! Serializer implementations for drama MEI elements.
//!
//! This module contains implementations for Sp, Speaker, StageDir, Role, RoleName
//! and related attribute classes used in dramatic/performance texts.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttSpAnl, AttSpGes, AttSpLog, AttSpVis, AttStageDirAnl, AttStageDirGes, AttStageDirLog,
    AttStageDirVis,
};
use tusk_model::elements::{
    Role, RoleChild, RoleName, RoleNameChild, Sp, SpChild, Speaker, SpeakerChild, StageDir,
    StageDirChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiSerialize for Sp {
    fn element_name(&self) -> &'static str {
        "sp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.sp_anl.collect_attributes());
        attrs.extend(self.sp_ges.collect_attributes());
        attrs.extend(self.sp_log.collect_attributes());
        attrs.extend(self.sp_vis.collect_attributes());
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

impl MeiSerialize for SpChild {
    fn element_name(&self) -> &'static str {
        match self {
            SpChild::Quote(_) => "quote",
            SpChild::List(_) => "list",
            SpChild::Speaker(_) => "speaker",
            SpChild::Lb(_) => "lb",
            SpChild::Cb(_) => "cb",
            SpChild::ColLayout(_) => "colLayout",
            SpChild::Pb(_) => "pb",
            SpChild::P(_) => "p",
            SpChild::L(_) => "l",
            SpChild::StageDir(_) => "stageDir",
            SpChild::Annot(_) => "annot",
            SpChild::App(_) => "app",
            SpChild::Fig(_) => "fig",
            SpChild::Lg(_) => "lg",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SpChild::Speaker(s) => s.collect_all_attributes(),
            SpChild::StageDir(sd) => sd.collect_all_attributes(),
            SpChild::P(p) => p.collect_all_attributes(),
            SpChild::L(l) => l.collect_all_attributes(),
            SpChild::Lg(lg) => lg.collect_all_attributes(),
            SpChild::List(l) => l.collect_all_attributes(),
            SpChild::Lb(lb) => lb.collect_all_attributes(),
            SpChild::Pb(pb) => pb.collect_all_attributes(),
            SpChild::Annot(a) => a.collect_all_attributes(),
            SpChild::Fig(f) => f.collect_all_attributes(),
            SpChild::App(a) => a.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SpChild::Speaker(s) => s.has_children(),
            SpChild::StageDir(sd) => sd.has_children(),
            SpChild::P(p) => p.has_children(),
            SpChild::L(l) => l.has_children(),
            SpChild::Lg(lg) => lg.has_children(),
            SpChild::List(l) => l.has_children(),
            SpChild::Lb(_) => false,
            SpChild::Pb(pb) => pb.has_children(),
            SpChild::Annot(a) => a.has_children(),
            SpChild::Fig(f) => f.has_children(),
            SpChild::App(a) => a.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SpChild::Speaker(s) => s.serialize_children(writer),
            SpChild::StageDir(sd) => sd.serialize_children(writer),
            SpChild::P(p) => p.serialize_children(writer),
            SpChild::L(l) => l.serialize_children(writer),
            SpChild::Lg(lg) => lg.serialize_children(writer),
            SpChild::List(l) => l.serialize_children(writer),
            SpChild::Pb(pb) => pb.serialize_children(writer),
            SpChild::Annot(a) => a.serialize_children(writer),
            SpChild::Fig(f) => f.serialize_children(writer),
            SpChild::App(a) => a.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

impl MeiSerialize for Speaker {
    fn element_name(&self) -> &'static str {
        "speaker"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for SpeakerChild {
    fn element_name(&self) -> &'static str {
        match self {
            SpeakerChild::Text(_) => "$text",
            SpeakerChild::Symbol(_) => "symbol",
            SpeakerChild::Lb(_) => "lb",
            SpeakerChild::Dimensions(_) => "dimensions",
            SpeakerChild::Address(_) => "address",
            SpeakerChild::RelationList(_) => "relationList",
            SpeakerChild::Damage(_) => "damage",
            SpeakerChild::Identifier(_) => "identifier",
            SpeakerChild::Fig(_) => "fig",
            SpeakerChild::Num(_) => "num",
            SpeakerChild::Name(_) => "name",
            SpeakerChild::Depth(_) => "depth",
            SpeakerChild::Add(_) => "add",
            SpeakerChild::Extent(_) => "extent",
            SpeakerChild::Settlement(_) => "settlement",
            SpeakerChild::Signatures(_) => "signatures",
            SpeakerChild::Annot(_) => "annot",
            SpeakerChild::LocusGrp(_) => "locusGrp",
            SpeakerChild::Ref(_) => "ref",
            SpeakerChild::Street(_) => "street",
            SpeakerChild::Supplied(_) => "supplied",
            SpeakerChild::PersName(_) => "persName",
            SpeakerChild::Sic(_) => "sic",
            SpeakerChild::Gap(_) => "gap",
            SpeakerChild::Dim(_) => "dim",
            SpeakerChild::Rend(_) => "rend",
            SpeakerChild::BiblStruct(_) => "biblStruct",
            SpeakerChild::Seg(_) => "seg",
            SpeakerChild::Choice(_) => "choice",
            SpeakerChild::PostCode(_) => "postCode",
            SpeakerChild::HandShift(_) => "handShift",
            SpeakerChild::SecFolio(_) => "secFolio",
            SpeakerChild::Relation(_) => "relation",
            SpeakerChild::Repository(_) => "repository",
            SpeakerChild::Bloc(_) => "bloc",
            SpeakerChild::District(_) => "district",
            SpeakerChild::Width(_) => "width",
            SpeakerChild::Q(_) => "q",
            SpeakerChild::StyleName(_) => "styleName",
            SpeakerChild::Ptr(_) => "ptr",
            SpeakerChild::Stamp(_) => "stamp",
            SpeakerChild::Subst(_) => "subst",
            SpeakerChild::GeogFeat(_) => "geogFeat",
            SpeakerChild::Region(_) => "region",
            SpeakerChild::Stack(_) => "stack",
            SpeakerChild::Term(_) => "term",
            SpeakerChild::Locus(_) => "locus",
            SpeakerChild::Orig(_) => "orig",
            SpeakerChild::Bibl(_) => "bibl",
            SpeakerChild::Del(_) => "del",
            SpeakerChild::Catchwords(_) => "catchwords",
            SpeakerChild::Country(_) => "country",
            SpeakerChild::Title(_) => "title",
            SpeakerChild::Expan(_) => "expan",
            SpeakerChild::Corr(_) => "corr",
            SpeakerChild::Heraldry(_) => "heraldry",
            SpeakerChild::Unclear(_) => "unclear",
            SpeakerChild::CorpName(_) => "corpName",
            SpeakerChild::Height(_) => "height",
            SpeakerChild::Date(_) => "date",
            SpeakerChild::Restore(_) => "restore",
            SpeakerChild::Reg(_) => "reg",
            SpeakerChild::Abbr(_) => "abbr",
            SpeakerChild::PostBox(_) => "postBox",
            SpeakerChild::GeogName(_) => "geogName",
            SpeakerChild::PeriodName(_) => "periodName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SpeakerChild::Text(_) => Vec::new(),
            SpeakerChild::Rend(r) => r.collect_all_attributes(),
            SpeakerChild::Lb(lb) => lb.collect_all_attributes(),
            SpeakerChild::Name(n) => n.collect_all_attributes(),
            SpeakerChild::PersName(p) => p.collect_all_attributes(),
            SpeakerChild::Seg(s) => s.collect_all_attributes(),
            SpeakerChild::Num(n) => n.collect_all_attributes(),
            SpeakerChild::Date(d) => d.collect_all_attributes(),
            SpeakerChild::Ref(r) => r.collect_all_attributes(),
            SpeakerChild::Ptr(p) => p.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SpeakerChild::Text(_) => false,
            SpeakerChild::Rend(r) => r.has_children(),
            SpeakerChild::Lb(_) => false,
            SpeakerChild::Name(n) => n.has_children(),
            SpeakerChild::PersName(p) => p.has_children(),
            SpeakerChild::Seg(s) => s.has_children(),
            SpeakerChild::Num(n) => n.has_children(),
            SpeakerChild::Date(d) => d.has_children(),
            SpeakerChild::Ref(r) => r.has_children(),
            SpeakerChild::Ptr(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SpeakerChild::Text(_) => Ok(()),
            SpeakerChild::Rend(r) => r.serialize_children(writer),
            SpeakerChild::Name(n) => n.serialize_children(writer),
            SpeakerChild::PersName(p) => p.serialize_children(writer),
            SpeakerChild::Seg(s) => s.serialize_children(writer),
            SpeakerChild::Num(n) => n.serialize_children(writer),
            SpeakerChild::Date(d) => d.serialize_children(writer),
            SpeakerChild::Ref(r) => r.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SpeakerChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SpeakerChild::Rend(elem) => elem.serialize_mei(writer),
            SpeakerChild::Lb(elem) => elem.serialize_mei(writer),
            SpeakerChild::Name(elem) => elem.serialize_mei(writer),
            SpeakerChild::PersName(elem) => elem.serialize_mei(writer),
            SpeakerChild::Seg(elem) => elem.serialize_mei(writer),
            SpeakerChild::Num(elem) => elem.serialize_mei(writer),
            SpeakerChild::Date(elem) => elem.serialize_mei(writer),
            SpeakerChild::Ref(elem) => elem.serialize_mei(writer),
            SpeakerChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SpeakerChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for StageDir {
    fn element_name(&self) -> &'static str {
        "stageDir"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.stage_dir_anl.collect_attributes());
        attrs.extend(self.stage_dir_ges.collect_attributes());
        attrs.extend(self.stage_dir_log.collect_attributes());
        attrs.extend(self.stage_dir_vis.collect_attributes());
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

impl MeiSerialize for StageDirChild {
    fn element_name(&self) -> &'static str {
        match self {
            StageDirChild::Text(_) => "$text",
            StageDirChild::Unclear(_) => "unclear",
            StageDirChild::Heraldry(_) => "heraldry",
            StageDirChild::Title(_) => "title",
            StageDirChild::District(_) => "district",
            StageDirChild::Sic(_) => "sic",
            StageDirChild::Restore(_) => "restore",
            StageDirChild::Region(_) => "region",
            StageDirChild::Extent(_) => "extent",
            StageDirChild::StyleName(_) => "styleName",
            StageDirChild::Reg(_) => "reg",
            StageDirChild::Bibl(_) => "bibl",
            StageDirChild::Rend(_) => "rend",
            StageDirChild::Repository(_) => "repository",
            StageDirChild::Seg(_) => "seg",
            StageDirChild::Stamp(_) => "stamp",
            StageDirChild::Abbr(_) => "abbr",
            StageDirChild::Expan(_) => "expan",
            StageDirChild::Dimensions(_) => "dimensions",
            StageDirChild::Gap(_) => "gap",
            StageDirChild::Orig(_) => "orig",
            StageDirChild::PostCode(_) => "postCode",
            StageDirChild::Ref(_) => "ref",
            StageDirChild::Settlement(_) => "settlement",
            StageDirChild::Name(_) => "name",
            StageDirChild::Add(_) => "add",
            StageDirChild::Address(_) => "address",
            StageDirChild::CorpName(_) => "corpName",
            StageDirChild::Dim(_) => "dim",
            StageDirChild::Fig(_) => "fig",
            StageDirChild::SecFolio(_) => "secFolio",
            StageDirChild::Depth(_) => "depth",
            StageDirChild::Country(_) => "country",
            StageDirChild::GeogFeat(_) => "geogFeat",
            StageDirChild::Width(_) => "width",
            StageDirChild::Lb(_) => "lb",
            StageDirChild::Ptr(_) => "ptr",
            StageDirChild::Relation(_) => "relation",
            StageDirChild::PeriodName(_) => "periodName",
            StageDirChild::Corr(_) => "corr",
            StageDirChild::GeogName(_) => "geogName",
            StageDirChild::Street(_) => "street",
            StageDirChild::Choice(_) => "choice",
            StageDirChild::Subst(_) => "subst",
            StageDirChild::Damage(_) => "damage",
            StageDirChild::HandShift(_) => "handShift",
            StageDirChild::Bloc(_) => "bloc",
            StageDirChild::Supplied(_) => "supplied",
            StageDirChild::LocusGrp(_) => "locusGrp",
            StageDirChild::Identifier(_) => "identifier",
            StageDirChild::Catchwords(_) => "catchwords",
            StageDirChild::Locus(_) => "locus",
            StageDirChild::Annot(_) => "annot",
            StageDirChild::Height(_) => "height",
            StageDirChild::Stack(_) => "stack",
            StageDirChild::RelationList(_) => "relationList",
            StageDirChild::Symbol(_) => "symbol",
            StageDirChild::Date(_) => "date",
            StageDirChild::Num(_) => "num",
            StageDirChild::Q(_) => "q",
            StageDirChild::Del(_) => "del",
            StageDirChild::BiblStruct(_) => "biblStruct",
            StageDirChild::Signatures(_) => "signatures",
            StageDirChild::PostBox(_) => "postBox",
            StageDirChild::PersName(_) => "persName",
            StageDirChild::Term(_) => "term",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            StageDirChild::Text(_) => Vec::new(),
            StageDirChild::Rend(r) => r.collect_all_attributes(),
            StageDirChild::Lb(lb) => lb.collect_all_attributes(),
            StageDirChild::Name(n) => n.collect_all_attributes(),
            StageDirChild::PersName(p) => p.collect_all_attributes(),
            StageDirChild::Seg(s) => s.collect_all_attributes(),
            StageDirChild::Ref(r) => r.collect_all_attributes(),
            StageDirChild::Fig(f) => f.collect_all_attributes(),
            StageDirChild::Num(n) => n.collect_all_attributes(),
            StageDirChild::Date(d) => d.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            StageDirChild::Text(_) => false,
            StageDirChild::Rend(r) => r.has_children(),
            StageDirChild::Lb(_) => false,
            StageDirChild::Name(n) => n.has_children(),
            StageDirChild::PersName(p) => p.has_children(),
            StageDirChild::Seg(s) => s.has_children(),
            StageDirChild::Ref(r) => r.has_children(),
            StageDirChild::Fig(f) => f.has_children(),
            StageDirChild::Num(n) => n.has_children(),
            StageDirChild::Date(d) => d.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StageDirChild::Text(_) => Ok(()),
            StageDirChild::Rend(r) => r.serialize_children(writer),
            StageDirChild::Name(n) => n.serialize_children(writer),
            StageDirChild::PersName(p) => p.serialize_children(writer),
            StageDirChild::Seg(s) => s.serialize_children(writer),
            StageDirChild::Ref(r) => r.serialize_children(writer),
            StageDirChild::Fig(f) => f.serialize_children(writer),
            StageDirChild::Num(n) => n.serialize_children(writer),
            StageDirChild::Date(d) => d.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StageDirChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            StageDirChild::Rend(elem) => elem.serialize_mei(writer),
            StageDirChild::Lb(elem) => elem.serialize_mei(writer),
            StageDirChild::Name(elem) => elem.serialize_mei(writer),
            StageDirChild::PersName(elem) => elem.serialize_mei(writer),
            StageDirChild::Seg(elem) => elem.serialize_mei(writer),
            StageDirChild::Ref(elem) => elem.serialize_mei(writer),
            StageDirChild::Fig(elem) => elem.serialize_mei(writer),
            StageDirChild::Num(elem) => elem.serialize_mei(writer),
            StageDirChild::Date(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "StageDirChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Role {
    fn element_name(&self) -> &'static str {
        "role"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for RoleChild {
    fn element_name(&self) -> &'static str {
        match self {
            RoleChild::Text(_) => "$text",
            RoleChild::Catchwords(_) => "catchwords",
            RoleChild::Heraldry(_) => "heraldry",
            RoleChild::PeriodName(_) => "periodName",
            RoleChild::Height(_) => "height",
            RoleChild::Ref(_) => "ref",
            RoleChild::Relation(_) => "relation",
            RoleChild::Signatures(_) => "signatures",
            RoleChild::Title(_) => "title",
            RoleChild::Settlement(_) => "settlement",
            RoleChild::Depth(_) => "depth",
            RoleChild::Term(_) => "term",
            RoleChild::Width(_) => "width",
            RoleChild::StyleName(_) => "styleName",
            RoleChild::RelationList(_) => "relationList",
            RoleChild::Seg(_) => "seg",
            RoleChild::GeogFeat(_) => "geogFeat",
            RoleChild::Locus(_) => "locus",
            RoleChild::Lb(_) => "lb",
            RoleChild::Date(_) => "date",
            RoleChild::Dimensions(_) => "dimensions",
            RoleChild::Num(_) => "num",
            RoleChild::Ptr(_) => "ptr",
            RoleChild::PersName(_) => "persName",
            RoleChild::Rend(_) => "rend",
            RoleChild::Street(_) => "street",
            RoleChild::PostBox(_) => "postBox",
            RoleChild::Q(_) => "q",
            RoleChild::Stamp(_) => "stamp",
            RoleChild::BiblStruct(_) => "biblStruct",
            RoleChild::Country(_) => "country",
            RoleChild::Bloc(_) => "bloc",
            RoleChild::Stack(_) => "stack",
            RoleChild::Extent(_) => "extent",
            RoleChild::PostCode(_) => "postCode",
            RoleChild::Address(_) => "address",
            RoleChild::GeogName(_) => "geogName",
            RoleChild::Name(_) => "name",
            RoleChild::Expan(_) => "expan",
            RoleChild::Annot(_) => "annot",
            RoleChild::Repository(_) => "repository",
            RoleChild::LocusGrp(_) => "locusGrp",
            RoleChild::Fig(_) => "fig",
            RoleChild::CorpName(_) => "corpName",
            RoleChild::Symbol(_) => "symbol",
            RoleChild::Bibl(_) => "bibl",
            RoleChild::Dim(_) => "dim",
            RoleChild::District(_) => "district",
            RoleChild::SecFolio(_) => "secFolio",
            RoleChild::Abbr(_) => "abbr",
            RoleChild::Region(_) => "region",
            RoleChild::Identifier(_) => "identifier",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RoleChild::Text(_) => Vec::new(),
            RoleChild::Rend(r) => r.collect_all_attributes(),
            RoleChild::Lb(lb) => lb.collect_all_attributes(),
            RoleChild::Name(n) => n.collect_all_attributes(),
            RoleChild::PersName(p) => p.collect_all_attributes(),
            RoleChild::Seg(s) => s.collect_all_attributes(),
            RoleChild::Ref(r) => r.collect_all_attributes(),
            RoleChild::Num(n) => n.collect_all_attributes(),
            RoleChild::Date(d) => d.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RoleChild::Text(_) => false,
            RoleChild::Rend(r) => r.has_children(),
            RoleChild::Lb(_) => false,
            RoleChild::Name(n) => n.has_children(),
            RoleChild::PersName(p) => p.has_children(),
            RoleChild::Seg(s) => s.has_children(),
            RoleChild::Ref(r) => r.has_children(),
            RoleChild::Num(n) => n.has_children(),
            RoleChild::Date(d) => d.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RoleChild::Text(_) => Ok(()),
            RoleChild::Rend(r) => r.serialize_children(writer),
            RoleChild::Name(n) => n.serialize_children(writer),
            RoleChild::PersName(p) => p.serialize_children(writer),
            RoleChild::Seg(s) => s.serialize_children(writer),
            RoleChild::Ref(r) => r.serialize_children(writer),
            RoleChild::Num(n) => n.serialize_children(writer),
            RoleChild::Date(d) => d.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RoleChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RoleChild::Rend(elem) => elem.serialize_mei(writer),
            RoleChild::Lb(elem) => elem.serialize_mei(writer),
            RoleChild::Name(elem) => elem.serialize_mei(writer),
            RoleChild::PersName(elem) => elem.serialize_mei(writer),
            RoleChild::Seg(elem) => elem.serialize_mei(writer),
            RoleChild::Ref(elem) => elem.serialize_mei(writer),
            RoleChild::Num(elem) => elem.serialize_mei(writer),
            RoleChild::Date(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RoleChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for RoleName {
    fn element_name(&self) -> &'static str {
        "roleName"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for RoleNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            RoleNameChild::Text(_) => "$text",
            RoleNameChild::Damage(_) => "damage",
            RoleNameChild::Pb(_) => "pb",
            RoleNameChild::Name(_) => "name",
            RoleNameChild::Title(_) => "title",
            RoleNameChild::Unclear(_) => "unclear",
            RoleNameChild::GeogFeat(_) => "geogFeat",
            RoleNameChild::CorpName(_) => "corpName",
            RoleNameChild::Settlement(_) => "settlement",
            RoleNameChild::Width(_) => "width",
            RoleNameChild::Relation(_) => "relation",
            RoleNameChild::Heraldry(_) => "heraldry",
            RoleNameChild::Abbr(_) => "abbr",
            RoleNameChild::Height(_) => "height",
            RoleNameChild::Num(_) => "num",
            RoleNameChild::Ptr(_) => "ptr",
            RoleNameChild::PeriodName(_) => "periodName",
            RoleNameChild::Supplied(_) => "supplied",
            RoleNameChild::Orig(_) => "orig",
            RoleNameChild::Identifier(_) => "identifier",
            RoleNameChild::Stack(_) => "stack",
            RoleNameChild::District(_) => "district",
            RoleNameChild::Depth(_) => "depth",
            RoleNameChild::Signatures(_) => "signatures",
            RoleNameChild::Lb(_) => "lb",
            RoleNameChild::Address(_) => "address",
            RoleNameChild::Dim(_) => "dim",
            RoleNameChild::Extent(_) => "extent",
            RoleNameChild::StyleName(_) => "styleName",
            RoleNameChild::Fig(_) => "fig",
            RoleNameChild::Stamp(_) => "stamp",
            RoleNameChild::Dimensions(_) => "dimensions",
            RoleNameChild::Bloc(_) => "bloc",
            RoleNameChild::Rend(_) => "rend",
            RoleNameChild::Gap(_) => "gap",
            RoleNameChild::Expan(_) => "expan",
            RoleNameChild::Corr(_) => "corr",
            RoleNameChild::BiblStruct(_) => "biblStruct",
            RoleNameChild::Annot(_) => "annot",
            RoleNameChild::RelationList(_) => "relationList",
            RoleNameChild::Date(_) => "date",
            RoleNameChild::Repository(_) => "repository",
            RoleNameChild::Seg(_) => "seg",
            RoleNameChild::Region(_) => "region",
            RoleNameChild::LocusGrp(_) => "locusGrp",
            RoleNameChild::Street(_) => "street",
            RoleNameChild::Term(_) => "term",
            RoleNameChild::Symbol(_) => "symbol",
            RoleNameChild::PersName(_) => "persName",
            RoleNameChild::Ref(_) => "ref",
            RoleNameChild::Del(_) => "del",
            RoleNameChild::Reg(_) => "reg",
            RoleNameChild::PostBox(_) => "postBox",
            RoleNameChild::SecFolio(_) => "secFolio",
            RoleNameChild::Sic(_) => "sic",
            RoleNameChild::Bibl(_) => "bibl",
            RoleNameChild::Add(_) => "add",
            RoleNameChild::Restore(_) => "restore",
            RoleNameChild::Country(_) => "country",
            RoleNameChild::GeogName(_) => "geogName",
            RoleNameChild::HandShift(_) => "handShift",
            RoleNameChild::Locus(_) => "locus",
            RoleNameChild::Subst(_) => "subst",
            RoleNameChild::Q(_) => "q",
            RoleNameChild::Catchwords(_) => "catchwords",
            RoleNameChild::PostCode(_) => "postCode",
            RoleNameChild::Choice(_) => "choice",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RoleNameChild::Text(_) => Vec::new(),
            RoleNameChild::Rend(r) => r.collect_all_attributes(),
            RoleNameChild::Lb(lb) => lb.collect_all_attributes(),
            RoleNameChild::Pb(pb) => pb.collect_all_attributes(),
            RoleNameChild::Name(n) => n.collect_all_attributes(),
            RoleNameChild::PersName(p) => p.collect_all_attributes(),
            RoleNameChild::Seg(s) => s.collect_all_attributes(),
            RoleNameChild::Ref(r) => r.collect_all_attributes(),
            RoleNameChild::Date(d) => d.collect_all_attributes(),
            RoleNameChild::Num(n) => n.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RoleNameChild::Text(_) => false,
            RoleNameChild::Rend(r) => r.has_children(),
            RoleNameChild::Lb(_) => false,
            RoleNameChild::Pb(pb) => pb.has_children(),
            RoleNameChild::Name(n) => n.has_children(),
            RoleNameChild::PersName(p) => p.has_children(),
            RoleNameChild::Seg(s) => s.has_children(),
            RoleNameChild::Ref(r) => r.has_children(),
            RoleNameChild::Date(d) => d.has_children(),
            RoleNameChild::Num(n) => n.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RoleNameChild::Text(_) => Ok(()),
            RoleNameChild::Rend(r) => r.serialize_children(writer),
            RoleNameChild::Pb(pb) => pb.serialize_children(writer),
            RoleNameChild::Name(n) => n.serialize_children(writer),
            RoleNameChild::PersName(p) => p.serialize_children(writer),
            RoleNameChild::Seg(s) => s.serialize_children(writer),
            RoleNameChild::Ref(r) => r.serialize_children(writer),
            RoleNameChild::Date(d) => d.serialize_children(writer),
            RoleNameChild::Num(n) => n.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RoleNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RoleNameChild::Rend(elem) => elem.serialize_mei(writer),
            RoleNameChild::Lb(elem) => elem.serialize_mei(writer),
            RoleNameChild::Pb(elem) => elem.serialize_mei(writer),
            RoleNameChild::Name(elem) => elem.serialize_mei(writer),
            RoleNameChild::PersName(elem) => elem.serialize_mei(writer),
            RoleNameChild::Seg(elem) => elem.serialize_mei(writer),
            RoleNameChild::Ref(elem) => elem.serialize_mei(writer),
            RoleNameChild::Date(elem) => elem.serialize_mei(writer),
            RoleNameChild::Num(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RoleNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

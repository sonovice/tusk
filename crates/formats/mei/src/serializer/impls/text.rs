//! Serializer implementations for text and front/back matter MEI elements.
//!
//! This module contains implementations for Front, Back, TitlePage, Argument,
//! Epigraph, Dedication, Imprimatur, Colophon, and their child elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Argument, ArgumentChild, Back, BackChild, Colophon, ColophonChild, Dedication, DedicationChild,
    Epigraph, EpigraphChild, Front, FrontChild, Imprimatur, ImprimaturChild, TitlePage,
    TitlePageChild,
};

// ============================================================================
// Front element implementation
// ============================================================================

impl MeiSerialize for Front {
    fn element_name(&self) -> &'static str {
        "front"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for FrontChild {
    fn element_name(&self) -> &'static str {
        match self {
            FrontChild::Relation(_) => "relation",
            FrontChild::Cb(_) => "cb",
            FrontChild::ColLayout(_) => "colLayout",
            FrontChild::Lb(_) => "lb",
            FrontChild::Pb(_) => "pb",
            FrontChild::TitlePage(_) => "titlePage",
            FrontChild::Div(_) => "div",
            FrontChild::RelationList(_) => "relationList",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FrontChild::TitlePage(tp) => tp.collect_all_attributes(),
            FrontChild::Div(div) => div.collect_all_attributes(),
            FrontChild::Lb(lb) => lb.collect_all_attributes(),
            FrontChild::Pb(pb) => pb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FrontChild::TitlePage(tp) => tp.has_children(),
            FrontChild::Div(div) => div.has_children(),
            FrontChild::Lb(_) => false,
            FrontChild::Pb(pb) => pb.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FrontChild::TitlePage(tp) => tp.serialize_children(writer),
            FrontChild::Div(div) => div.serialize_children(writer),
            FrontChild::Pb(pb) => pb.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Back element implementation
// ============================================================================

impl MeiSerialize for Back {
    fn element_name(&self) -> &'static str {
        "back"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for BackChild {
    fn element_name(&self) -> &'static str {
        match self {
            BackChild::Relation(_) => "relation",
            BackChild::TitlePage(_) => "titlePage",
            BackChild::Cb(_) => "cb",
            BackChild::Lb(_) => "lb",
            BackChild::Pb(_) => "pb",
            BackChild::RelationList(_) => "relationList",
            BackChild::ColLayout(_) => "colLayout",
            BackChild::Div(_) => "div",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            BackChild::TitlePage(tp) => tp.collect_all_attributes(),
            BackChild::Div(div) => div.collect_all_attributes(),
            BackChild::Lb(lb) => lb.collect_all_attributes(),
            BackChild::Pb(pb) => pb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            BackChild::TitlePage(tp) => tp.has_children(),
            BackChild::Div(div) => div.has_children(),
            BackChild::Lb(_) => false,
            BackChild::Pb(pb) => pb.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BackChild::TitlePage(tp) => tp.serialize_children(writer),
            BackChild::Div(div) => div.serialize_children(writer),
            BackChild::Pb(pb) => pb.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// TitlePage element implementation
// ============================================================================

impl MeiSerialize for TitlePage {
    fn element_name(&self) -> &'static str {
        "titlePage"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for TitlePageChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitlePageChild::Contents(_) => "contents",
            TitlePageChild::Lb(_) => "lb",
            TitlePageChild::AccessRestrict(_) => "accessRestrict",
            TitlePageChild::Imprimatur(_) => "imprimatur",
            TitlePageChild::Argument(_) => "argument",
            TitlePageChild::Contributor(_) => "contributor",
            TitlePageChild::Fig(_) => "fig",
            TitlePageChild::Editor(_) => "editor",
            TitlePageChild::Epigraph(_) => "epigraph",
            TitlePageChild::Head(_) => "head",
            TitlePageChild::Cb(_) => "cb",
            TitlePageChild::List(_) => "list",
            TitlePageChild::Imprint(_) => "imprint",
            TitlePageChild::Series(_) => "series",
            TitlePageChild::Unpub(_) => "unpub",
            TitlePageChild::Creator(_) => "creator",
            TitlePageChild::EventList(_) => "eventList",
            TitlePageChild::Lg(_) => "lg",
            TitlePageChild::P(_) => "p",
            TitlePageChild::Dedication(_) => "dedication",
            TitlePageChild::Date(_) => "date",
            TitlePageChild::Edition(_) => "edition",
            TitlePageChild::Damage(_) => "damage",
            TitlePageChild::Unclear(_) => "unclear",
            TitlePageChild::SysReq(_) => "sysReq",
            TitlePageChild::Identifier(_) => "identifier",
            TitlePageChild::Sic(_) => "sic",
            TitlePageChild::PubPlace(_) => "pubPlace",
            TitlePageChild::Pb(_) => "pb",
            TitlePageChild::PerfDuration(_) => "perfDuration",
            TitlePageChild::Title(_) => "title",
            TitlePageChild::Del(_) => "del",
            TitlePageChild::Orig(_) => "orig",
            TitlePageChild::ColLayout(_) => "colLayout",
            TitlePageChild::PerfMedium(_) => "perfMedium",
            TitlePageChild::CastList(_) => "castList",
            TitlePageChild::Publisher(_) => "publisher",
            TitlePageChild::Quote(_) => "quote",
            TitlePageChild::PlateNum(_) => "plateNum",
            TitlePageChild::BiblList(_) => "biblList",
            TitlePageChild::Price(_) => "price",
            TitlePageChild::Add(_) => "add",
            TitlePageChild::Sponsor(_) => "sponsor",
            TitlePageChild::Corr(_) => "corr",
            TitlePageChild::Reg(_) => "reg",
            TitlePageChild::TitlePart(_) => "titlePart",
            TitlePageChild::Distributor(_) => "distributor",
            TitlePageChild::UseRestrict(_) => "useRestrict",
            TitlePageChild::HandShift(_) => "handShift",
            TitlePageChild::Restore(_) => "restore",
            TitlePageChild::Supplied(_) => "supplied",
            TitlePageChild::Availability(_) => "availability",
            TitlePageChild::Table(_) => "table",
            TitlePageChild::Gap(_) => "gap",
            TitlePageChild::Funder(_) => "funder",
            TitlePageChild::Byline(_) => "byline",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TitlePageChild::Imprimatur(imp) => imp.collect_all_attributes(),
            TitlePageChild::Argument(arg) => arg.collect_all_attributes(),
            TitlePageChild::Epigraph(epi) => epi.collect_all_attributes(),
            TitlePageChild::Dedication(ded) => ded.collect_all_attributes(),
            TitlePageChild::Title(t) => t.collect_all_attributes(),
            TitlePageChild::P(p) => p.collect_all_attributes(),
            TitlePageChild::Lg(lg) => lg.collect_all_attributes(),
            TitlePageChild::Head(h) => h.collect_all_attributes(),
            TitlePageChild::List(l) => l.collect_all_attributes(),
            TitlePageChild::Table(t) => t.collect_all_attributes(),
            TitlePageChild::Fig(f) => f.collect_all_attributes(),
            TitlePageChild::Lb(lb) => lb.collect_all_attributes(),
            TitlePageChild::Pb(pb) => pb.collect_all_attributes(),
            TitlePageChild::Date(d) => d.collect_all_attributes(),
            TitlePageChild::Identifier(i) => i.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TitlePageChild::Imprimatur(imp) => imp.has_children(),
            TitlePageChild::Argument(arg) => arg.has_children(),
            TitlePageChild::Epigraph(epi) => epi.has_children(),
            TitlePageChild::Dedication(ded) => ded.has_children(),
            TitlePageChild::Title(t) => t.has_children(),
            TitlePageChild::P(p) => p.has_children(),
            TitlePageChild::Lg(lg) => lg.has_children(),
            TitlePageChild::Head(h) => h.has_children(),
            TitlePageChild::List(l) => l.has_children(),
            TitlePageChild::Table(t) => t.has_children(),
            TitlePageChild::Fig(f) => f.has_children(),
            TitlePageChild::Lb(_) => false,
            TitlePageChild::Pb(pb) => pb.has_children(),
            TitlePageChild::Date(d) => d.has_children(),
            TitlePageChild::Identifier(i) => i.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TitlePageChild::Imprimatur(imp) => imp.serialize_children(writer),
            TitlePageChild::Argument(arg) => arg.serialize_children(writer),
            TitlePageChild::Epigraph(epi) => epi.serialize_children(writer),
            TitlePageChild::Dedication(ded) => ded.serialize_children(writer),
            TitlePageChild::Title(t) => t.serialize_children(writer),
            TitlePageChild::P(p) => p.serialize_children(writer),
            TitlePageChild::Lg(lg) => lg.serialize_children(writer),
            TitlePageChild::Head(h) => h.serialize_children(writer),
            TitlePageChild::List(l) => l.serialize_children(writer),
            TitlePageChild::Table(t) => t.serialize_children(writer),
            TitlePageChild::Fig(f) => f.serialize_children(writer),
            TitlePageChild::Pb(pb) => pb.serialize_children(writer),
            TitlePageChild::Date(d) => d.serialize_children(writer),
            TitlePageChild::Identifier(i) => i.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Argument element implementation
// ============================================================================

impl MeiSerialize for Argument {
    fn element_name(&self) -> &'static str {
        "argument"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for ArgumentChild {
    fn element_name(&self) -> &'static str {
        match self {
            ArgumentChild::Fig(_) => "fig",
            ArgumentChild::Table(_) => "table",
            ArgumentChild::Cb(_) => "cb",
            ArgumentChild::BiblList(_) => "biblList",
            ArgumentChild::EventList(_) => "eventList",
            ArgumentChild::Sp(_) => "sp",
            ArgumentChild::List(_) => "list",
            ArgumentChild::Quote(_) => "quote",
            ArgumentChild::Lb(_) => "lb",
            ArgumentChild::CastList(_) => "castList",
            ArgumentChild::Head(_) => "head",
            ArgumentChild::Pb(_) => "pb",
            ArgumentChild::ColLayout(_) => "colLayout",
            ArgumentChild::P(_) => "p",
            ArgumentChild::Lg(_) => "lg",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ArgumentChild::P(p) => p.collect_all_attributes(),
            ArgumentChild::Lg(lg) => lg.collect_all_attributes(),
            ArgumentChild::Head(h) => h.collect_all_attributes(),
            ArgumentChild::List(l) => l.collect_all_attributes(),
            ArgumentChild::Table(t) => t.collect_all_attributes(),
            ArgumentChild::Fig(f) => f.collect_all_attributes(),
            ArgumentChild::Lb(lb) => lb.collect_all_attributes(),
            ArgumentChild::Pb(pb) => pb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ArgumentChild::P(p) => p.has_children(),
            ArgumentChild::Lg(lg) => lg.has_children(),
            ArgumentChild::Head(h) => h.has_children(),
            ArgumentChild::List(l) => l.has_children(),
            ArgumentChild::Table(t) => t.has_children(),
            ArgumentChild::Fig(f) => f.has_children(),
            ArgumentChild::Lb(_) => false,
            ArgumentChild::Pb(pb) => pb.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ArgumentChild::P(p) => p.serialize_children(writer),
            ArgumentChild::Lg(lg) => lg.serialize_children(writer),
            ArgumentChild::Head(h) => h.serialize_children(writer),
            ArgumentChild::List(l) => l.serialize_children(writer),
            ArgumentChild::Table(t) => t.serialize_children(writer),
            ArgumentChild::Fig(f) => f.serialize_children(writer),
            ArgumentChild::Pb(pb) => pb.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Epigraph element implementation
// ============================================================================

impl MeiSerialize for Epigraph {
    fn element_name(&self) -> &'static str {
        "epigraph"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for EpigraphChild {
    fn element_name(&self) -> &'static str {
        match self {
            EpigraphChild::Text(_) => "$text",
            EpigraphChild::Address(_) => "address",
            EpigraphChild::Fig(_) => "fig",
            EpigraphChild::SecFolio(_) => "secFolio",
            EpigraphChild::Repository(_) => "repository",
            EpigraphChild::Stack(_) => "stack",
            EpigraphChild::Title(_) => "title",
            EpigraphChild::Date(_) => "date",
            EpigraphChild::Expan(_) => "expan",
            EpigraphChild::Stamp(_) => "stamp",
            EpigraphChild::Relation(_) => "relation",
            EpigraphChild::Lg(_) => "lg",
            EpigraphChild::Catchwords(_) => "catchwords",
            EpigraphChild::StageDir(_) => "stageDir",
            EpigraphChild::Subst(_) => "subst",
            EpigraphChild::HandShift(_) => "handShift",
            EpigraphChild::Width(_) => "width",
            EpigraphChild::District(_) => "district",
            EpigraphChild::Identifier(_) => "identifier",
            EpigraphChild::Q(_) => "q",
            EpigraphChild::Depth(_) => "depth",
            EpigraphChild::Lb(_) => "lb",
            EpigraphChild::Orig(_) => "orig",
            EpigraphChild::LocusGrp(_) => "locusGrp",
            EpigraphChild::PostCode(_) => "postCode",
            EpigraphChild::Sic(_) => "sic",
            EpigraphChild::Settlement(_) => "settlement",
            EpigraphChild::Dimensions(_) => "dimensions",
            EpigraphChild::BiblList(_) => "biblList",
            EpigraphChild::List(_) => "list",
            EpigraphChild::Symbol(_) => "symbol",
            EpigraphChild::Table(_) => "table",
            EpigraphChild::GeogName(_) => "geogName",
            EpigraphChild::P(_) => "p",
            EpigraphChild::Gap(_) => "gap",
            EpigraphChild::Pb(_) => "pb",
            EpigraphChild::Restore(_) => "restore",
            EpigraphChild::StyleName(_) => "styleName",
            EpigraphChild::Locus(_) => "locus",
            EpigraphChild::Del(_) => "del",
            EpigraphChild::PeriodName(_) => "periodName",
            EpigraphChild::Ref(_) => "ref",
            EpigraphChild::Region(_) => "region",
            EpigraphChild::Street(_) => "street",
            EpigraphChild::Ptr(_) => "ptr",
            EpigraphChild::Abbr(_) => "abbr",
            EpigraphChild::PersName(_) => "persName",
            EpigraphChild::Bloc(_) => "bloc",
            EpigraphChild::Term(_) => "term",
            EpigraphChild::Damage(_) => "damage",
            EpigraphChild::Unclear(_) => "unclear",
            EpigraphChild::Rend(_) => "rend",
            EpigraphChild::Dim(_) => "dim",
            EpigraphChild::Name(_) => "name",
            EpigraphChild::Choice(_) => "choice",
            EpigraphChild::Corr(_) => "corr",
            EpigraphChild::EventList(_) => "eventList",
            EpigraphChild::Num(_) => "num",
            EpigraphChild::PostBox(_) => "postBox",
            EpigraphChild::Add(_) => "add",
            EpigraphChild::Country(_) => "country",
            EpigraphChild::GeogFeat(_) => "geogFeat",
            EpigraphChild::Annot(_) => "annot",
            EpigraphChild::Extent(_) => "extent",
            EpigraphChild::Height(_) => "height",
            EpigraphChild::Quote(_) => "quote",
            EpigraphChild::Reg(_) => "reg",
            EpigraphChild::CastList(_) => "castList",
            EpigraphChild::Seg(_) => "seg",
            EpigraphChild::Signatures(_) => "signatures",
            EpigraphChild::Supplied(_) => "supplied",
            EpigraphChild::Heraldry(_) => "heraldry",
            EpigraphChild::Bibl(_) => "bibl",
            EpigraphChild::BiblStruct(_) => "biblStruct",
            EpigraphChild::RelationList(_) => "relationList",
            EpigraphChild::CorpName(_) => "corpName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            EpigraphChild::Text(_) => Vec::new(),
            EpigraphChild::P(p) => p.collect_all_attributes(),
            EpigraphChild::Lg(lg) => lg.collect_all_attributes(),
            EpigraphChild::Rend(r) => r.collect_all_attributes(),
            EpigraphChild::Lb(lb) => lb.collect_all_attributes(),
            EpigraphChild::Pb(pb) => pb.collect_all_attributes(),
            EpigraphChild::Title(t) => t.collect_all_attributes(),
            EpigraphChild::Date(d) => d.collect_all_attributes(),
            EpigraphChild::Name(n) => n.collect_all_attributes(),
            EpigraphChild::PersName(p) => p.collect_all_attributes(),
            EpigraphChild::Ref(r) => r.collect_all_attributes(),
            EpigraphChild::Ptr(p) => p.collect_all_attributes(),
            EpigraphChild::Num(n) => n.collect_all_attributes(),
            EpigraphChild::Seg(s) => s.collect_all_attributes(),
            EpigraphChild::Identifier(i) => i.collect_all_attributes(),
            EpigraphChild::Bibl(b) => b.collect_all_attributes(),
            EpigraphChild::List(l) => l.collect_all_attributes(),
            EpigraphChild::Table(t) => t.collect_all_attributes(),
            EpigraphChild::Fig(f) => f.collect_all_attributes(),
            EpigraphChild::Annot(a) => a.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            EpigraphChild::Text(_) => false,
            EpigraphChild::P(p) => p.has_children(),
            EpigraphChild::Lg(lg) => lg.has_children(),
            EpigraphChild::Rend(r) => r.has_children(),
            EpigraphChild::Lb(_) => false,
            EpigraphChild::Pb(pb) => pb.has_children(),
            EpigraphChild::Title(t) => t.has_children(),
            EpigraphChild::Date(d) => d.has_children(),
            EpigraphChild::Name(n) => n.has_children(),
            EpigraphChild::PersName(p) => p.has_children(),
            EpigraphChild::Ref(r) => r.has_children(),
            EpigraphChild::Ptr(_) => false,
            EpigraphChild::Num(n) => n.has_children(),
            EpigraphChild::Seg(s) => s.has_children(),
            EpigraphChild::Identifier(i) => i.has_children(),
            EpigraphChild::Bibl(b) => b.has_children(),
            EpigraphChild::List(l) => l.has_children(),
            EpigraphChild::Table(t) => t.has_children(),
            EpigraphChild::Fig(f) => f.has_children(),
            EpigraphChild::Annot(a) => a.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EpigraphChild::Text(t) => {
                writer.write_text(t)?;
                Ok(())
            }
            EpigraphChild::P(p) => p.serialize_children(writer),
            EpigraphChild::Lg(lg) => lg.serialize_children(writer),
            EpigraphChild::Rend(r) => r.serialize_children(writer),
            EpigraphChild::Pb(pb) => pb.serialize_children(writer),
            EpigraphChild::Title(t) => t.serialize_children(writer),
            EpigraphChild::Date(d) => d.serialize_children(writer),
            EpigraphChild::Name(n) => n.serialize_children(writer),
            EpigraphChild::PersName(p) => p.serialize_children(writer),
            EpigraphChild::Ref(r) => r.serialize_children(writer),
            EpigraphChild::Num(n) => n.serialize_children(writer),
            EpigraphChild::Seg(s) => s.serialize_children(writer),
            EpigraphChild::Identifier(i) => i.serialize_children(writer),
            EpigraphChild::Bibl(b) => b.serialize_children(writer),
            EpigraphChild::List(l) => l.serialize_children(writer),
            EpigraphChild::Table(t) => t.serialize_children(writer),
            EpigraphChild::Fig(f) => f.serialize_children(writer),
            EpigraphChild::Annot(a) => a.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EpigraphChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
                for (attr_name, value) in attrs {
                    start.push_attribute((attr_name, value.as_str()));
                }

                if self.has_children() {
                    writer.write_start(start)?;
                    self.serialize_children(writer)?;
                    writer.write_end(name)?;
                } else {
                    writer.write_empty(start)?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Dedication element implementation
// ============================================================================

impl MeiSerialize for Dedication {
    fn element_name(&self) -> &'static str {
        "dedication"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for DedicationChild {
    fn element_name(&self) -> &'static str {
        match self {
            DedicationChild::Text(_) => "$text",
            DedicationChild::Expan(_) => "expan",
            DedicationChild::Lg(_) => "lg",
            DedicationChild::Height(_) => "height",
            DedicationChild::LocusGrp(_) => "locusGrp",
            DedicationChild::Fig(_) => "fig",
            DedicationChild::Quote(_) => "quote",
            DedicationChild::Heraldry(_) => "heraldry",
            DedicationChild::Ref(_) => "ref",
            DedicationChild::SecFolio(_) => "secFolio",
            DedicationChild::Settlement(_) => "settlement",
            DedicationChild::Bibl(_) => "bibl",
            DedicationChild::Identifier(_) => "identifier",
            DedicationChild::Num(_) => "num",
            DedicationChild::Ptr(_) => "ptr",
            DedicationChild::Relation(_) => "relation",
            DedicationChild::GeogFeat(_) => "geogFeat",
            DedicationChild::Title(_) => "title",
            DedicationChild::Annot(_) => "annot",
            DedicationChild::List(_) => "list",
            DedicationChild::Lb(_) => "lb",
            DedicationChild::RelationList(_) => "relationList",
            DedicationChild::Repository(_) => "repository",
            DedicationChild::Stack(_) => "stack",
            DedicationChild::BiblStruct(_) => "biblStruct",
            DedicationChild::Abbr(_) => "abbr",
            DedicationChild::Head(_) => "head",
            DedicationChild::BiblList(_) => "biblList",
            DedicationChild::Address(_) => "address",
            DedicationChild::Q(_) => "q",
            DedicationChild::Country(_) => "country",
            DedicationChild::PostCode(_) => "postCode",
            DedicationChild::Rend(_) => "rend",
            DedicationChild::Symbol(_) => "symbol",
            DedicationChild::GeogName(_) => "geogName",
            DedicationChild::Term(_) => "term",
            DedicationChild::Name(_) => "name",
            DedicationChild::Seg(_) => "seg",
            DedicationChild::Dimensions(_) => "dimensions",
            DedicationChild::Locus(_) => "locus",
            DedicationChild::StyleName(_) => "styleName",
            DedicationChild::District(_) => "district",
            DedicationChild::PeriodName(_) => "periodName",
            DedicationChild::Extent(_) => "extent",
            DedicationChild::PersName(_) => "persName",
            DedicationChild::P(_) => "p",
            DedicationChild::Street(_) => "street",
            DedicationChild::Bloc(_) => "bloc",
            DedicationChild::Dim(_) => "dim",
            DedicationChild::Depth(_) => "depth",
            DedicationChild::CastList(_) => "castList",
            DedicationChild::EventList(_) => "eventList",
            DedicationChild::Catchwords(_) => "catchwords",
            DedicationChild::CorpName(_) => "corpName",
            DedicationChild::Width(_) => "width",
            DedicationChild::Region(_) => "region",
            DedicationChild::PostBox(_) => "postBox",
            DedicationChild::Date(_) => "date",
            DedicationChild::Signatures(_) => "signatures",
            DedicationChild::Table(_) => "table",
            DedicationChild::Stamp(_) => "stamp",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            DedicationChild::Text(_) => Vec::new(),
            DedicationChild::P(p) => p.collect_all_attributes(),
            DedicationChild::Lg(lg) => lg.collect_all_attributes(),
            DedicationChild::Rend(r) => r.collect_all_attributes(),
            DedicationChild::Lb(lb) => lb.collect_all_attributes(),
            DedicationChild::Title(t) => t.collect_all_attributes(),
            DedicationChild::Date(d) => d.collect_all_attributes(),
            DedicationChild::Name(n) => n.collect_all_attributes(),
            DedicationChild::PersName(p) => p.collect_all_attributes(),
            DedicationChild::Ref(r) => r.collect_all_attributes(),
            DedicationChild::Ptr(p) => p.collect_all_attributes(),
            DedicationChild::Num(n) => n.collect_all_attributes(),
            DedicationChild::Seg(s) => s.collect_all_attributes(),
            DedicationChild::Identifier(i) => i.collect_all_attributes(),
            DedicationChild::Head(h) => h.collect_all_attributes(),
            DedicationChild::Bibl(b) => b.collect_all_attributes(),
            DedicationChild::List(l) => l.collect_all_attributes(),
            DedicationChild::Table(t) => t.collect_all_attributes(),
            DedicationChild::Fig(f) => f.collect_all_attributes(),
            DedicationChild::Annot(a) => a.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            DedicationChild::Text(_) => false,
            DedicationChild::P(p) => p.has_children(),
            DedicationChild::Lg(lg) => lg.has_children(),
            DedicationChild::Rend(r) => r.has_children(),
            DedicationChild::Lb(_) => false,
            DedicationChild::Title(t) => t.has_children(),
            DedicationChild::Date(d) => d.has_children(),
            DedicationChild::Name(n) => n.has_children(),
            DedicationChild::PersName(p) => p.has_children(),
            DedicationChild::Ref(r) => r.has_children(),
            DedicationChild::Ptr(_) => false,
            DedicationChild::Num(n) => n.has_children(),
            DedicationChild::Seg(s) => s.has_children(),
            DedicationChild::Identifier(i) => i.has_children(),
            DedicationChild::Head(h) => h.has_children(),
            DedicationChild::Bibl(b) => b.has_children(),
            DedicationChild::List(l) => l.has_children(),
            DedicationChild::Table(t) => t.has_children(),
            DedicationChild::Fig(f) => f.has_children(),
            DedicationChild::Annot(a) => a.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DedicationChild::Text(t) => {
                writer.write_text(t)?;
                Ok(())
            }
            DedicationChild::P(p) => p.serialize_children(writer),
            DedicationChild::Lg(lg) => lg.serialize_children(writer),
            DedicationChild::Rend(r) => r.serialize_children(writer),
            DedicationChild::Title(t) => t.serialize_children(writer),
            DedicationChild::Date(d) => d.serialize_children(writer),
            DedicationChild::Name(n) => n.serialize_children(writer),
            DedicationChild::PersName(p) => p.serialize_children(writer),
            DedicationChild::Ref(r) => r.serialize_children(writer),
            DedicationChild::Num(n) => n.serialize_children(writer),
            DedicationChild::Seg(s) => s.serialize_children(writer),
            DedicationChild::Identifier(i) => i.serialize_children(writer),
            DedicationChild::Head(h) => h.serialize_children(writer),
            DedicationChild::Bibl(b) => b.serialize_children(writer),
            DedicationChild::List(l) => l.serialize_children(writer),
            DedicationChild::Table(t) => t.serialize_children(writer),
            DedicationChild::Fig(f) => f.serialize_children(writer),
            DedicationChild::Annot(a) => a.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DedicationChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
                for (attr_name, value) in attrs {
                    start.push_attribute((attr_name, value.as_str()));
                }

                if self.has_children() {
                    writer.write_start(start)?;
                    self.serialize_children(writer)?;
                    writer.write_end(name)?;
                } else {
                    writer.write_empty(start)?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Imprimatur element implementation
// ============================================================================

impl MeiSerialize for Imprimatur {
    fn element_name(&self) -> &'static str {
        "imprimatur"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for ImprimaturChild {
    fn element_name(&self) -> &'static str {
        match self {
            ImprimaturChild::Text(_) => "$text",
            ImprimaturChild::Repository(_) => "repository",
            ImprimaturChild::Name(_) => "name",
            ImprimaturChild::LocusGrp(_) => "locusGrp",
            ImprimaturChild::CorpName(_) => "corpName",
            ImprimaturChild::Num(_) => "num",
            ImprimaturChild::Identifier(_) => "identifier",
            ImprimaturChild::PeriodName(_) => "periodName",
            ImprimaturChild::PostBox(_) => "postBox",
            ImprimaturChild::BiblStruct(_) => "biblStruct",
            ImprimaturChild::Dimensions(_) => "dimensions",
            ImprimaturChild::Fig(_) => "fig",
            ImprimaturChild::GeogFeat(_) => "geogFeat",
            ImprimaturChild::Symbol(_) => "symbol",
            ImprimaturChild::Restore(_) => "restore",
            ImprimaturChild::Unclear(_) => "unclear",
            ImprimaturChild::Choice(_) => "choice",
            ImprimaturChild::Abbr(_) => "abbr",
            ImprimaturChild::Q(_) => "q",
            ImprimaturChild::Bloc(_) => "bloc",
            ImprimaturChild::Seg(_) => "seg",
            ImprimaturChild::Table(_) => "table",
            ImprimaturChild::Quote(_) => "quote",
            ImprimaturChild::Region(_) => "region",
            ImprimaturChild::Depth(_) => "depth",
            ImprimaturChild::Ref(_) => "ref",
            ImprimaturChild::BiblList(_) => "biblList",
            ImprimaturChild::Dim(_) => "dim",
            ImprimaturChild::Stamp(_) => "stamp",
            ImprimaturChild::Supplied(_) => "supplied",
            ImprimaturChild::Del(_) => "del",
            ImprimaturChild::Height(_) => "height",
            ImprimaturChild::Lg(_) => "lg",
            ImprimaturChild::Relation(_) => "relation",
            ImprimaturChild::Sic(_) => "sic",
            ImprimaturChild::StageDir(_) => "stageDir",
            ImprimaturChild::Expan(_) => "expan",
            ImprimaturChild::Street(_) => "street",
            ImprimaturChild::Term(_) => "term",
            ImprimaturChild::Settlement(_) => "settlement",
            ImprimaturChild::Stack(_) => "stack",
            ImprimaturChild::Subst(_) => "subst",
            ImprimaturChild::PostCode(_) => "postCode",
            ImprimaturChild::Locus(_) => "locus",
            ImprimaturChild::StyleName(_) => "styleName",
            ImprimaturChild::Catchwords(_) => "catchwords",
            ImprimaturChild::Lb(_) => "lb",
            ImprimaturChild::EventList(_) => "eventList",
            ImprimaturChild::Title(_) => "title",
            ImprimaturChild::Annot(_) => "annot",
            ImprimaturChild::Add(_) => "add",
            ImprimaturChild::Date(_) => "date",
            ImprimaturChild::Extent(_) => "extent",
            ImprimaturChild::HandShift(_) => "handShift",
            ImprimaturChild::PersName(_) => "persName",
            ImprimaturChild::Signatures(_) => "signatures",
            ImprimaturChild::Heraldry(_) => "heraldry",
            ImprimaturChild::SecFolio(_) => "secFolio",
            ImprimaturChild::Gap(_) => "gap",
            ImprimaturChild::Damage(_) => "damage",
            ImprimaturChild::Bibl(_) => "bibl",
            ImprimaturChild::Address(_) => "address",
            ImprimaturChild::Reg(_) => "reg",
            ImprimaturChild::GeogName(_) => "geogName",
            ImprimaturChild::Corr(_) => "corr",
            ImprimaturChild::CastList(_) => "castList",
            ImprimaturChild::List(_) => "list",
            ImprimaturChild::Pb(_) => "pb",
            ImprimaturChild::District(_) => "district",
            ImprimaturChild::Orig(_) => "orig",
            ImprimaturChild::RelationList(_) => "relationList",
            ImprimaturChild::Ptr(_) => "ptr",
            ImprimaturChild::Rend(_) => "rend",
            ImprimaturChild::Width(_) => "width",
            ImprimaturChild::Country(_) => "country",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ImprimaturChild::Text(_) => Vec::new(),
            ImprimaturChild::Rend(r) => r.collect_all_attributes(),
            ImprimaturChild::Lb(lb) => lb.collect_all_attributes(),
            ImprimaturChild::Pb(pb) => pb.collect_all_attributes(),
            ImprimaturChild::Title(t) => t.collect_all_attributes(),
            ImprimaturChild::Date(d) => d.collect_all_attributes(),
            ImprimaturChild::Name(n) => n.collect_all_attributes(),
            ImprimaturChild::PersName(p) => p.collect_all_attributes(),
            ImprimaturChild::Ref(r) => r.collect_all_attributes(),
            ImprimaturChild::Ptr(p) => p.collect_all_attributes(),
            ImprimaturChild::Num(n) => n.collect_all_attributes(),
            ImprimaturChild::Seg(s) => s.collect_all_attributes(),
            ImprimaturChild::Identifier(i) => i.collect_all_attributes(),
            ImprimaturChild::Lg(lg) => lg.collect_all_attributes(),
            ImprimaturChild::Bibl(b) => b.collect_all_attributes(),
            ImprimaturChild::List(l) => l.collect_all_attributes(),
            ImprimaturChild::Table(t) => t.collect_all_attributes(),
            ImprimaturChild::Fig(f) => f.collect_all_attributes(),
            ImprimaturChild::Annot(a) => a.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ImprimaturChild::Text(_) => false,
            ImprimaturChild::Rend(r) => r.has_children(),
            ImprimaturChild::Lb(_) => false,
            ImprimaturChild::Pb(pb) => pb.has_children(),
            ImprimaturChild::Title(t) => t.has_children(),
            ImprimaturChild::Date(d) => d.has_children(),
            ImprimaturChild::Name(n) => n.has_children(),
            ImprimaturChild::PersName(p) => p.has_children(),
            ImprimaturChild::Ref(r) => r.has_children(),
            ImprimaturChild::Ptr(_) => false,
            ImprimaturChild::Num(n) => n.has_children(),
            ImprimaturChild::Seg(s) => s.has_children(),
            ImprimaturChild::Identifier(i) => i.has_children(),
            ImprimaturChild::Lg(lg) => lg.has_children(),
            ImprimaturChild::Bibl(b) => b.has_children(),
            ImprimaturChild::List(l) => l.has_children(),
            ImprimaturChild::Table(t) => t.has_children(),
            ImprimaturChild::Fig(f) => f.has_children(),
            ImprimaturChild::Annot(a) => a.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ImprimaturChild::Text(t) => {
                writer.write_text(t)?;
                Ok(())
            }
            ImprimaturChild::Rend(r) => r.serialize_children(writer),
            ImprimaturChild::Pb(pb) => pb.serialize_children(writer),
            ImprimaturChild::Title(t) => t.serialize_children(writer),
            ImprimaturChild::Date(d) => d.serialize_children(writer),
            ImprimaturChild::Name(n) => n.serialize_children(writer),
            ImprimaturChild::PersName(p) => p.serialize_children(writer),
            ImprimaturChild::Ref(r) => r.serialize_children(writer),
            ImprimaturChild::Num(n) => n.serialize_children(writer),
            ImprimaturChild::Seg(s) => s.serialize_children(writer),
            ImprimaturChild::Identifier(i) => i.serialize_children(writer),
            ImprimaturChild::Lg(lg) => lg.serialize_children(writer),
            ImprimaturChild::Bibl(b) => b.serialize_children(writer),
            ImprimaturChild::List(l) => l.serialize_children(writer),
            ImprimaturChild::Table(t) => t.serialize_children(writer),
            ImprimaturChild::Fig(f) => f.serialize_children(writer),
            ImprimaturChild::Annot(a) => a.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ImprimaturChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
                for (attr_name, value) in attrs {
                    start.push_attribute((attr_name, value.as_str()));
                }

                if self.has_children() {
                    writer.write_start(start)?;
                    self.serialize_children(writer)?;
                    writer.write_end(name)?;
                } else {
                    writer.write_empty(start)?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Colophon element implementation
// ============================================================================

impl MeiSerialize for Colophon {
    fn element_name(&self) -> &'static str {
        "colophon"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for ColophonChild {
    fn element_name(&self) -> &'static str {
        match self {
            ColophonChild::Text(_) => "$text",
            ColophonChild::Extent(_) => "extent",
            ColophonChild::Bibl(_) => "bibl",
            ColophonChild::Address(_) => "address",
            ColophonChild::PostBox(_) => "postBox",
            ColophonChild::Fig(_) => "fig",
            ColophonChild::LocusGrp(_) => "locusGrp",
            ColophonChild::Region(_) => "region",
            ColophonChild::StyleName(_) => "styleName",
            ColophonChild::Symbol(_) => "symbol",
            ColophonChild::GeogFeat(_) => "geogFeat",
            ColophonChild::Name(_) => "name",
            ColophonChild::Date(_) => "date",
            ColophonChild::Ref(_) => "ref",
            ColophonChild::Term(_) => "term",
            ColophonChild::Seg(_) => "seg",
            ColophonChild::Dimensions(_) => "dimensions",
            ColophonChild::Abbr(_) => "abbr",
            ColophonChild::Relation(_) => "relation",
            ColophonChild::Bloc(_) => "bloc",
            ColophonChild::GeogName(_) => "geogName",
            ColophonChild::CorpName(_) => "corpName",
            ColophonChild::Annot(_) => "annot",
            ColophonChild::Expan(_) => "expan",
            ColophonChild::Depth(_) => "depth",
            ColophonChild::Lb(_) => "lb",
            ColophonChild::SecFolio(_) => "secFolio",
            ColophonChild::Rend(_) => "rend",
            ColophonChild::BiblStruct(_) => "biblStruct",
            ColophonChild::P(_) => "p",
            ColophonChild::PostCode(_) => "postCode",
            ColophonChild::Dim(_) => "dim",
            ColophonChild::Identifier(_) => "identifier",
            ColophonChild::District(_) => "district",
            ColophonChild::Catchwords(_) => "catchwords",
            ColophonChild::Heraldry(_) => "heraldry",
            ColophonChild::Num(_) => "num",
            ColophonChild::PeriodName(_) => "periodName",
            ColophonChild::RelationList(_) => "relationList",
            ColophonChild::Signatures(_) => "signatures",
            ColophonChild::Ptr(_) => "ptr",
            ColophonChild::Head(_) => "head",
            ColophonChild::Repository(_) => "repository",
            ColophonChild::Stack(_) => "stack",
            ColophonChild::Q(_) => "q",
            ColophonChild::Stamp(_) => "stamp",
            ColophonChild::Street(_) => "street",
            ColophonChild::Width(_) => "width",
            ColophonChild::Locus(_) => "locus",
            ColophonChild::PersName(_) => "persName",
            ColophonChild::Settlement(_) => "settlement",
            ColophonChild::Title(_) => "title",
            ColophonChild::Height(_) => "height",
            ColophonChild::Country(_) => "country",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ColophonChild::Text(_) => Vec::new(),
            ColophonChild::P(p) => p.collect_all_attributes(),
            ColophonChild::Rend(r) => r.collect_all_attributes(),
            ColophonChild::Lb(lb) => lb.collect_all_attributes(),
            ColophonChild::Title(t) => t.collect_all_attributes(),
            ColophonChild::Date(d) => d.collect_all_attributes(),
            ColophonChild::Name(n) => n.collect_all_attributes(),
            ColophonChild::PersName(p) => p.collect_all_attributes(),
            ColophonChild::Ref(r) => r.collect_all_attributes(),
            ColophonChild::Ptr(p) => p.collect_all_attributes(),
            ColophonChild::Num(n) => n.collect_all_attributes(),
            ColophonChild::Seg(s) => s.collect_all_attributes(),
            ColophonChild::Identifier(i) => i.collect_all_attributes(),
            ColophonChild::Head(h) => h.collect_all_attributes(),
            ColophonChild::Bibl(b) => b.collect_all_attributes(),
            ColophonChild::Fig(f) => f.collect_all_attributes(),
            ColophonChild::Annot(a) => a.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ColophonChild::Text(_) => false,
            ColophonChild::P(p) => p.has_children(),
            ColophonChild::Rend(r) => r.has_children(),
            ColophonChild::Lb(_) => false,
            ColophonChild::Title(t) => t.has_children(),
            ColophonChild::Date(d) => d.has_children(),
            ColophonChild::Name(n) => n.has_children(),
            ColophonChild::PersName(p) => p.has_children(),
            ColophonChild::Ref(r) => r.has_children(),
            ColophonChild::Ptr(_) => false,
            ColophonChild::Num(n) => n.has_children(),
            ColophonChild::Seg(s) => s.has_children(),
            ColophonChild::Identifier(i) => i.has_children(),
            ColophonChild::Head(h) => h.has_children(),
            ColophonChild::Bibl(b) => b.has_children(),
            ColophonChild::Fig(f) => f.has_children(),
            ColophonChild::Annot(a) => a.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ColophonChild::Text(t) => {
                writer.write_text(t)?;
                Ok(())
            }
            ColophonChild::P(p) => p.serialize_children(writer),
            ColophonChild::Rend(r) => r.serialize_children(writer),
            ColophonChild::Title(t) => t.serialize_children(writer),
            ColophonChild::Date(d) => d.serialize_children(writer),
            ColophonChild::Name(n) => n.serialize_children(writer),
            ColophonChild::PersName(p) => p.serialize_children(writer),
            ColophonChild::Ref(r) => r.serialize_children(writer),
            ColophonChild::Num(n) => n.serialize_children(writer),
            ColophonChild::Seg(s) => s.serialize_children(writer),
            ColophonChild::Identifier(i) => i.serialize_children(writer),
            ColophonChild::Head(h) => h.serialize_children(writer),
            ColophonChild::Bibl(b) => b.serialize_children(writer),
            ColophonChild::Fig(f) => f.serialize_children(writer),
            ColophonChild::Annot(a) => a.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ColophonChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
                for (attr_name, value) in attrs {
                    start.push_attribute((attr_name, value.as_str()));
                }

                if self.has_children() {
                    writer.write_start(start)?;
                    self.serialize_children(writer)?;
                    writer.write_end(name)?;
                } else {
                    writer.write_empty(start)?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use crate::serializer::MeiSerialize;
    use tusk_model::elements::{
        Argument, Back, Colophon, Dedication, Epigraph, Front, Imprimatur, TitlePage,
    };

    /// Helper to test roundtrip: parse XML -> serialize -> check result contains expected content
    fn roundtrip_test<T: MeiDeserialize + MeiSerialize>(xml: &str) -> String {
        let parsed = T::from_mei_str(xml).expect("should parse");
        parsed.to_mei_string().expect("should serialize")
    }

    #[test]
    fn front_roundtrip() {
        let xml = r#"<front xml:id="front1"><titlePage><p>My Work</p></titlePage></front>"#;
        let result = roundtrip_test::<Front>(xml);
        assert!(result.contains("front"));
        assert!(result.contains("titlePage"));
        assert!(result.contains("My Work"));
    }

    #[test]
    fn back_roundtrip() {
        let xml = r#"<back xml:id="back1"><div><head>Appendix</head></div></back>"#;
        let result = roundtrip_test::<Back>(xml);
        assert!(result.contains("back"));
        assert!(result.contains("div"));
        assert!(result.contains("Appendix"));
    }

    #[test]
    fn title_page_roundtrip() {
        let xml = r#"<titlePage xml:id="tp1"><p>Sonata No. 1</p></titlePage>"#;
        let result = roundtrip_test::<TitlePage>(xml);
        assert!(result.contains("titlePage"));
        assert!(result.contains("Sonata No. 1"));
    }

    #[test]
    fn argument_roundtrip() {
        let xml = r#"<argument xml:id="arg1"><p>The story begins</p></argument>"#;
        let result = roundtrip_test::<Argument>(xml);
        assert!(result.contains("argument"));
        assert!(result.contains("The story begins"));
    }

    #[test]
    fn epigraph_roundtrip() {
        let xml = r#"<epigraph xml:id="ep1">A wise saying</epigraph>"#;
        let result = roundtrip_test::<Epigraph>(xml);
        assert!(result.contains("epigraph"));
        assert!(result.contains("A wise saying"));
    }

    #[test]
    fn dedication_roundtrip() {
        let xml = r#"<dedication xml:id="ded1">To my beloved</dedication>"#;
        let result = roundtrip_test::<Dedication>(xml);
        assert!(result.contains("dedication"));
        assert!(result.contains("To my beloved"));
    }

    #[test]
    fn imprimatur_roundtrip() {
        let xml = r#"<imprimatur xml:id="imp1">Approved</imprimatur>"#;
        let result = roundtrip_test::<Imprimatur>(xml);
        assert!(result.contains("imprimatur"));
        assert!(result.contains("Approved"));
    }

    #[test]
    fn colophon_roundtrip() {
        let xml = r#"<colophon xml:id="col1">Printed in Leipzig</colophon>"#;
        let result = roundtrip_test::<Colophon>(xml);
        assert!(result.contains("colophon"));
        assert!(result.contains("Printed in Leipzig"));
    }

    #[test]
    fn front_with_multiple_children() {
        let xml = r#"<front><titlePage><p>Title</p></titlePage><div><head>Preface</head><p>Text</p></div></front>"#;
        let result = roundtrip_test::<Front>(xml);
        assert!(result.contains("front"));
        assert!(result.contains("titlePage"));
        assert!(result.contains("div"));
        assert!(result.contains("Preface"));
    }
}

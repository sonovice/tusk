//! Serializer implementations for metadata text MEI elements.
//!
//! This module contains implementations for Rubric, Explicit, Byline, and Stamp elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Byline, BylineChild, Explicit, ExplicitChild, Rubric, RubricChild, Stamp, StampChild,
};

// ============================================================================
// Rubric element implementation
// ============================================================================

impl MeiSerialize for Rubric {
    fn element_name(&self) -> &'static str {
        "rubric"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Rubric-specific func attribute
        if let Some(ref func) = self.func {
            attrs.push(("func", func.clone()));
        }
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

impl MeiSerialize for RubricChild {
    fn element_name(&self) -> &'static str {
        match self {
            RubricChild::Text(_) => "",
            RubricChild::StyleName(_) => "styleName",
            RubricChild::Seg(_) => "seg",
            RubricChild::Date(_) => "date",
            RubricChild::SecFolio(_) => "secFolio",
            RubricChild::Settlement(_) => "settlement",
            RubricChild::Bloc(_) => "bloc",
            RubricChild::Lb(_) => "lb",
            RubricChild::Name(_) => "name",
            RubricChild::Title(_) => "title",
            RubricChild::PersName(_) => "persName",
            RubricChild::Ptr(_) => "ptr",
            RubricChild::PeriodName(_) => "periodName",
            RubricChild::Bibl(_) => "bibl",
            RubricChild::Abbr(_) => "abbr",
            RubricChild::Depth(_) => "depth",
            RubricChild::Expan(_) => "expan",
            RubricChild::Ref(_) => "ref",
            RubricChild::Q(_) => "q",
            RubricChild::GeogName(_) => "geogName",
            RubricChild::Symbol(_) => "symbol",
            RubricChild::Annot(_) => "annot",
            RubricChild::Height(_) => "height",
            RubricChild::Region(_) => "region",
            RubricChild::Catchwords(_) => "catchwords",
            RubricChild::Stamp(_) => "stamp",
            RubricChild::Identifier(_) => "identifier",
            RubricChild::Street(_) => "street",
            RubricChild::BiblStruct(_) => "biblStruct",
            RubricChild::LocusGrp(_) => "locusGrp",
            RubricChild::PostCode(_) => "postCode",
            RubricChild::Rend(_) => "rend",
            RubricChild::PostBox(_) => "postBox",
            RubricChild::Term(_) => "term",
            RubricChild::Width(_) => "width",
            RubricChild::Locus(_) => "locus",
            RubricChild::Num(_) => "num",
            RubricChild::Fig(_) => "fig",
            RubricChild::Head(_) => "head",
            RubricChild::RelationList(_) => "relationList",
            RubricChild::Signatures(_) => "signatures",
            RubricChild::Stack(_) => "stack",
            RubricChild::Address(_) => "address",
            RubricChild::Heraldry(_) => "heraldry",
            RubricChild::Dim(_) => "dim",
            RubricChild::CorpName(_) => "corpName",
            RubricChild::Dimensions(_) => "dimensions",
            RubricChild::District(_) => "district",
            RubricChild::Country(_) => "country",
            RubricChild::P(_) => "p",
            RubricChild::Extent(_) => "extent",
            RubricChild::GeogFeat(_) => "geogFeat",
            RubricChild::Relation(_) => "relation",
            RubricChild::Repository(_) => "repository",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RubricChild::Text(_) => Vec::new(),
            RubricChild::Seg(e) => e.collect_all_attributes(),
            RubricChild::Date(e) => e.collect_all_attributes(),
            RubricChild::Lb(e) => e.collect_all_attributes(),
            RubricChild::Name(e) => e.collect_all_attributes(),
            RubricChild::Title(e) => e.collect_all_attributes(),
            RubricChild::PersName(e) => e.collect_all_attributes(),
            RubricChild::Ptr(e) => e.collect_all_attributes(),
            RubricChild::Bibl(e) => e.collect_all_attributes(),
            RubricChild::Abbr(e) => e.collect_all_attributes(),
            RubricChild::Expan(e) => e.collect_all_attributes(),
            RubricChild::Ref(e) => e.collect_all_attributes(),
            RubricChild::Q(e) => e.collect_all_attributes(),
            RubricChild::Annot(e) => e.collect_all_attributes(),
            RubricChild::Stamp(e) => e.collect_all_attributes(),
            RubricChild::Rend(e) => e.collect_all_attributes(),
            RubricChild::Num(e) => e.collect_all_attributes(),
            RubricChild::Fig(e) => e.collect_all_attributes(),
            RubricChild::Head(e) => e.collect_all_attributes(),
            RubricChild::Stack(e) => e.collect_all_attributes(),
            RubricChild::CorpName(e) => e.collect_all_attributes(),
            RubricChild::P(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RubricChild::Text(_) => false,
            RubricChild::Seg(e) => e.has_children(),
            RubricChild::Date(e) => e.has_children(),
            RubricChild::Lb(_) => false,
            RubricChild::Name(e) => e.has_children(),
            RubricChild::Title(e) => e.has_children(),
            RubricChild::PersName(e) => e.has_children(),
            RubricChild::Ptr(_) => false,
            RubricChild::Bibl(e) => e.has_children(),
            RubricChild::Abbr(e) => e.has_children(),
            RubricChild::Expan(e) => e.has_children(),
            RubricChild::Ref(e) => e.has_children(),
            RubricChild::Q(e) => e.has_children(),
            RubricChild::Annot(e) => e.has_children(),
            RubricChild::Stamp(e) => e.has_children(),
            RubricChild::Rend(e) => e.has_children(),
            RubricChild::Num(e) => e.has_children(),
            RubricChild::Fig(e) => e.has_children(),
            RubricChild::Head(e) => e.has_children(),
            RubricChild::Stack(e) => e.has_children(),
            RubricChild::CorpName(e) => e.has_children(),
            RubricChild::P(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RubricChild::Seg(e) => e.serialize_children(writer),
            RubricChild::Date(e) => e.serialize_children(writer),
            RubricChild::Name(e) => e.serialize_children(writer),
            RubricChild::Title(e) => e.serialize_children(writer),
            RubricChild::PersName(e) => e.serialize_children(writer),
            RubricChild::Bibl(e) => e.serialize_children(writer),
            RubricChild::Abbr(e) => e.serialize_children(writer),
            RubricChild::Expan(e) => e.serialize_children(writer),
            RubricChild::Ref(e) => e.serialize_children(writer),
            RubricChild::Q(e) => e.serialize_children(writer),
            RubricChild::Annot(e) => e.serialize_children(writer),
            RubricChild::Stamp(e) => e.serialize_children(writer),
            RubricChild::Rend(e) => e.serialize_children(writer),
            RubricChild::Num(e) => e.serialize_children(writer),
            RubricChild::Fig(e) => e.serialize_children(writer),
            RubricChild::Head(e) => e.serialize_children(writer),
            RubricChild::Stack(e) => e.serialize_children(writer),
            RubricChild::CorpName(e) => e.serialize_children(writer),
            RubricChild::P(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RubricChild::Text(text) => {
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
// Explicit element implementation
// ============================================================================

impl MeiSerialize for Explicit {
    fn element_name(&self) -> &'static str {
        "explicit"
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

impl MeiSerialize for ExplicitChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExplicitChild::Text(_) => "",
            ExplicitChild::GeogName(_) => "geogName",
            ExplicitChild::Ref(_) => "ref",
            ExplicitChild::Symbol(_) => "symbol",
            ExplicitChild::Term(_) => "term",
            ExplicitChild::District(_) => "district",
            ExplicitChild::CorpName(_) => "corpName",
            ExplicitChild::Extent(_) => "extent",
            ExplicitChild::Settlement(_) => "settlement",
            ExplicitChild::Signatures(_) => "signatures",
            ExplicitChild::Country(_) => "country",
            ExplicitChild::Lb(_) => "lb",
            ExplicitChild::Dimensions(_) => "dimensions",
            ExplicitChild::Relation(_) => "relation",
            ExplicitChild::Dim(_) => "dim",
            ExplicitChild::Stamp(_) => "stamp",
            ExplicitChild::StyleName(_) => "styleName",
            ExplicitChild::GeogFeat(_) => "geogFeat",
            ExplicitChild::Abbr(_) => "abbr",
            ExplicitChild::Q(_) => "q",
            ExplicitChild::Title(_) => "title",
            ExplicitChild::PostBox(_) => "postBox",
            ExplicitChild::Heraldry(_) => "heraldry",
            ExplicitChild::PersName(_) => "persName",
            ExplicitChild::Ptr(_) => "ptr",
            ExplicitChild::Expan(_) => "expan",
            ExplicitChild::Head(_) => "head",
            ExplicitChild::Num(_) => "num",
            ExplicitChild::Address(_) => "address",
            ExplicitChild::Date(_) => "date",
            ExplicitChild::Identifier(_) => "identifier",
            ExplicitChild::Locus(_) => "locus",
            ExplicitChild::BiblStruct(_) => "biblStruct",
            ExplicitChild::Bloc(_) => "bloc",
            ExplicitChild::Street(_) => "street",
            ExplicitChild::Depth(_) => "depth",
            ExplicitChild::LocusGrp(_) => "locusGrp",
            ExplicitChild::Rend(_) => "rend",
            ExplicitChild::Catchwords(_) => "catchwords",
            ExplicitChild::SecFolio(_) => "secFolio",
            ExplicitChild::Stack(_) => "stack",
            ExplicitChild::Height(_) => "height",
            ExplicitChild::P(_) => "p",
            ExplicitChild::PeriodName(_) => "periodName",
            ExplicitChild::PostCode(_) => "postCode",
            ExplicitChild::Fig(_) => "fig",
            ExplicitChild::RelationList(_) => "relationList",
            ExplicitChild::Width(_) => "width",
            ExplicitChild::Repository(_) => "repository",
            ExplicitChild::Bibl(_) => "bibl",
            ExplicitChild::Seg(_) => "seg",
            ExplicitChild::Annot(_) => "annot",
            ExplicitChild::Name(_) => "name",
            ExplicitChild::Region(_) => "region",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ExplicitChild::Text(_) => Vec::new(),
            ExplicitChild::Ref(e) => e.collect_all_attributes(),
            ExplicitChild::CorpName(e) => e.collect_all_attributes(),
            ExplicitChild::Lb(e) => e.collect_all_attributes(),
            ExplicitChild::Stamp(e) => e.collect_all_attributes(),
            ExplicitChild::Abbr(e) => e.collect_all_attributes(),
            ExplicitChild::Q(e) => e.collect_all_attributes(),
            ExplicitChild::Title(e) => e.collect_all_attributes(),
            ExplicitChild::PersName(e) => e.collect_all_attributes(),
            ExplicitChild::Ptr(e) => e.collect_all_attributes(),
            ExplicitChild::Expan(e) => e.collect_all_attributes(),
            ExplicitChild::Head(e) => e.collect_all_attributes(),
            ExplicitChild::Num(e) => e.collect_all_attributes(),
            ExplicitChild::Date(e) => e.collect_all_attributes(),
            ExplicitChild::Rend(e) => e.collect_all_attributes(),
            ExplicitChild::Stack(e) => e.collect_all_attributes(),
            ExplicitChild::P(e) => e.collect_all_attributes(),
            ExplicitChild::Fig(e) => e.collect_all_attributes(),
            ExplicitChild::Bibl(e) => e.collect_all_attributes(),
            ExplicitChild::Seg(e) => e.collect_all_attributes(),
            ExplicitChild::Annot(e) => e.collect_all_attributes(),
            ExplicitChild::Name(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ExplicitChild::Text(_) => false,
            ExplicitChild::Ref(e) => e.has_children(),
            ExplicitChild::CorpName(e) => e.has_children(),
            ExplicitChild::Lb(_) => false,
            ExplicitChild::Stamp(e) => e.has_children(),
            ExplicitChild::Abbr(e) => e.has_children(),
            ExplicitChild::Q(e) => e.has_children(),
            ExplicitChild::Title(e) => e.has_children(),
            ExplicitChild::PersName(e) => e.has_children(),
            ExplicitChild::Ptr(_) => false,
            ExplicitChild::Expan(e) => e.has_children(),
            ExplicitChild::Head(e) => e.has_children(),
            ExplicitChild::Num(e) => e.has_children(),
            ExplicitChild::Date(e) => e.has_children(),
            ExplicitChild::Rend(e) => e.has_children(),
            ExplicitChild::Stack(e) => e.has_children(),
            ExplicitChild::P(e) => e.has_children(),
            ExplicitChild::Fig(e) => e.has_children(),
            ExplicitChild::Bibl(e) => e.has_children(),
            ExplicitChild::Seg(e) => e.has_children(),
            ExplicitChild::Annot(e) => e.has_children(),
            ExplicitChild::Name(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ExplicitChild::Ref(e) => e.serialize_children(writer),
            ExplicitChild::CorpName(e) => e.serialize_children(writer),
            ExplicitChild::Stamp(e) => e.serialize_children(writer),
            ExplicitChild::Abbr(e) => e.serialize_children(writer),
            ExplicitChild::Q(e) => e.serialize_children(writer),
            ExplicitChild::Title(e) => e.serialize_children(writer),
            ExplicitChild::PersName(e) => e.serialize_children(writer),
            ExplicitChild::Expan(e) => e.serialize_children(writer),
            ExplicitChild::Head(e) => e.serialize_children(writer),
            ExplicitChild::Num(e) => e.serialize_children(writer),
            ExplicitChild::Date(e) => e.serialize_children(writer),
            ExplicitChild::Rend(e) => e.serialize_children(writer),
            ExplicitChild::Stack(e) => e.serialize_children(writer),
            ExplicitChild::P(e) => e.serialize_children(writer),
            ExplicitChild::Fig(e) => e.serialize_children(writer),
            ExplicitChild::Bibl(e) => e.serialize_children(writer),
            ExplicitChild::Seg(e) => e.serialize_children(writer),
            ExplicitChild::Annot(e) => e.serialize_children(writer),
            ExplicitChild::Name(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ExplicitChild::Text(text) => {
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
// Byline element implementation
// ============================================================================

impl MeiSerialize for Byline {
    fn element_name(&self) -> &'static str {
        "byline"
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

impl MeiSerialize for BylineChild {
    fn element_name(&self) -> &'static str {
        match self {
            BylineChild::Text(_) => "",
            BylineChild::Identifier(_) => "identifier",
            BylineChild::Stack(_) => "stack",
            BylineChild::Date(_) => "date",
            BylineChild::Street(_) => "street",
            BylineChild::Title(_) => "title",
            BylineChild::Bibl(_) => "bibl",
            BylineChild::PersName(_) => "persName",
            BylineChild::Fig(_) => "fig",
            BylineChild::Country(_) => "country",
            BylineChild::Extent(_) => "extent",
            BylineChild::Dim(_) => "dim",
            BylineChild::PostBox(_) => "postBox",
            BylineChild::LocusGrp(_) => "locusGrp",
            BylineChild::Q(_) => "q",
            BylineChild::Catchwords(_) => "catchwords",
            BylineChild::Expan(_) => "expan",
            BylineChild::Ptr(_) => "ptr",
            BylineChild::Region(_) => "region",
            BylineChild::Signatures(_) => "signatures",
            BylineChild::Term(_) => "term",
            BylineChild::Name(_) => "name",
            BylineChild::Address(_) => "address",
            BylineChild::Contributor(_) => "contributor",
            BylineChild::Depth(_) => "depth",
            BylineChild::GeogFeat(_) => "geogFeat",
            BylineChild::Editor(_) => "editor",
            BylineChild::BiblStruct(_) => "biblStruct",
            BylineChild::Funder(_) => "funder",
            BylineChild::GeogName(_) => "geogName",
            BylineChild::RelationList(_) => "relationList",
            BylineChild::Sponsor(_) => "sponsor",
            BylineChild::Annot(_) => "annot",
            BylineChild::Ref(_) => "ref",
            BylineChild::Heraldry(_) => "heraldry",
            BylineChild::Repository(_) => "repository",
            BylineChild::SecFolio(_) => "secFolio",
            BylineChild::Dimensions(_) => "dimensions",
            BylineChild::Height(_) => "height",
            BylineChild::StyleName(_) => "styleName",
            BylineChild::District(_) => "district",
            BylineChild::Num(_) => "num",
            BylineChild::Creator(_) => "creator",
            BylineChild::Settlement(_) => "settlement",
            BylineChild::PeriodName(_) => "periodName",
            BylineChild::PostCode(_) => "postCode",
            BylineChild::Relation(_) => "relation",
            BylineChild::Stamp(_) => "stamp",
            BylineChild::Width(_) => "width",
            BylineChild::Bloc(_) => "bloc",
            BylineChild::CorpName(_) => "corpName",
            BylineChild::Lb(_) => "lb",
            BylineChild::Abbr(_) => "abbr",
            BylineChild::Locus(_) => "locus",
            BylineChild::Symbol(_) => "symbol",
            BylineChild::Seg(_) => "seg",
            BylineChild::Rend(_) => "rend",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            BylineChild::Text(_) => Vec::new(),
            BylineChild::Stack(e) => e.collect_all_attributes(),
            BylineChild::Date(e) => e.collect_all_attributes(),
            BylineChild::Title(e) => e.collect_all_attributes(),
            BylineChild::Bibl(e) => e.collect_all_attributes(),
            BylineChild::PersName(e) => e.collect_all_attributes(),
            BylineChild::Fig(e) => e.collect_all_attributes(),
            BylineChild::Q(e) => e.collect_all_attributes(),
            BylineChild::Expan(e) => e.collect_all_attributes(),
            BylineChild::Ptr(e) => e.collect_all_attributes(),
            BylineChild::Name(e) => e.collect_all_attributes(),
            BylineChild::Contributor(e) => e.collect_all_attributes(),
            BylineChild::Editor(e) => e.collect_all_attributes(),
            BylineChild::Funder(e) => e.collect_all_attributes(),
            BylineChild::Sponsor(e) => e.collect_all_attributes(),
            BylineChild::Annot(e) => e.collect_all_attributes(),
            BylineChild::Ref(e) => e.collect_all_attributes(),
            BylineChild::Num(e) => e.collect_all_attributes(),
            BylineChild::Creator(e) => e.collect_all_attributes(),
            BylineChild::Stamp(e) => e.collect_all_attributes(),
            BylineChild::CorpName(e) => e.collect_all_attributes(),
            BylineChild::Lb(e) => e.collect_all_attributes(),
            BylineChild::Abbr(e) => e.collect_all_attributes(),
            BylineChild::Seg(e) => e.collect_all_attributes(),
            BylineChild::Rend(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            BylineChild::Text(_) => false,
            BylineChild::Stack(e) => e.has_children(),
            BylineChild::Date(e) => e.has_children(),
            BylineChild::Title(e) => e.has_children(),
            BylineChild::Bibl(e) => e.has_children(),
            BylineChild::PersName(e) => e.has_children(),
            BylineChild::Fig(e) => e.has_children(),
            BylineChild::Q(e) => e.has_children(),
            BylineChild::Expan(e) => e.has_children(),
            BylineChild::Ptr(_) => false,
            BylineChild::Name(e) => e.has_children(),
            BylineChild::Contributor(e) => e.has_children(),
            BylineChild::Editor(e) => e.has_children(),
            BylineChild::Funder(e) => e.has_children(),
            BylineChild::Sponsor(e) => e.has_children(),
            BylineChild::Annot(e) => e.has_children(),
            BylineChild::Ref(e) => e.has_children(),
            BylineChild::Num(e) => e.has_children(),
            BylineChild::Creator(e) => e.has_children(),
            BylineChild::Stamp(e) => e.has_children(),
            BylineChild::CorpName(e) => e.has_children(),
            BylineChild::Lb(_) => false,
            BylineChild::Abbr(e) => e.has_children(),
            BylineChild::Seg(e) => e.has_children(),
            BylineChild::Rend(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BylineChild::Stack(e) => e.serialize_children(writer),
            BylineChild::Date(e) => e.serialize_children(writer),
            BylineChild::Title(e) => e.serialize_children(writer),
            BylineChild::Bibl(e) => e.serialize_children(writer),
            BylineChild::PersName(e) => e.serialize_children(writer),
            BylineChild::Fig(e) => e.serialize_children(writer),
            BylineChild::Q(e) => e.serialize_children(writer),
            BylineChild::Expan(e) => e.serialize_children(writer),
            BylineChild::Name(e) => e.serialize_children(writer),
            BylineChild::Contributor(e) => e.serialize_children(writer),
            BylineChild::Editor(e) => e.serialize_children(writer),
            BylineChild::Funder(e) => e.serialize_children(writer),
            BylineChild::Sponsor(e) => e.serialize_children(writer),
            BylineChild::Annot(e) => e.serialize_children(writer),
            BylineChild::Ref(e) => e.serialize_children(writer),
            BylineChild::Num(e) => e.serialize_children(writer),
            BylineChild::Creator(e) => e.serialize_children(writer),
            BylineChild::Stamp(e) => e.serialize_children(writer),
            BylineChild::CorpName(e) => e.serialize_children(writer),
            BylineChild::Abbr(e) => e.serialize_children(writer),
            BylineChild::Seg(e) => e.serialize_children(writer),
            BylineChild::Rend(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BylineChild::Text(text) => {
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
// Stamp element implementation
// ============================================================================

impl MeiSerialize for Stamp {
    fn element_name(&self) -> &'static str {
        "stamp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for StampChild {
    fn element_name(&self) -> &'static str {
        match self {
            StampChild::Text(_) => "",
            StampChild::Q(_) => "q",
            StampChild::Heraldry(_) => "heraldry",
            StampChild::Repository(_) => "repository",
            StampChild::StyleName(_) => "styleName",
            StampChild::RelationList(_) => "relationList",
            StampChild::Address(_) => "address",
            StampChild::Expan(_) => "expan",
            StampChild::Lb(_) => "lb",
            StampChild::CorpName(_) => "corpName",
            StampChild::District(_) => "district",
            StampChild::PostCode(_) => "postCode",
            StampChild::Ref(_) => "ref",
            StampChild::GeogName(_) => "geogName",
            StampChild::Bloc(_) => "bloc",
            StampChild::Bibl(_) => "bibl",
            StampChild::PersName(_) => "persName",
            StampChild::Street(_) => "street",
            StampChild::Width(_) => "width",
            StampChild::Term(_) => "term",
            StampChild::BiblStruct(_) => "biblStruct",
            StampChild::Catchwords(_) => "catchwords",
            StampChild::Rend(_) => "rend",
            StampChild::LocusGrp(_) => "locusGrp",
            StampChild::Num(_) => "num",
            StampChild::Relation(_) => "relation",
            StampChild::Stamp(_) => "stamp",
            StampChild::GeogFeat(_) => "geogFeat",
            StampChild::Head(_) => "head",
            StampChild::Date(_) => "date",
            StampChild::Dimensions(_) => "dimensions",
            StampChild::PostBox(_) => "postBox",
            StampChild::Seg(_) => "seg",
            StampChild::Dim(_) => "dim",
            StampChild::SecFolio(_) => "secFolio",
            StampChild::Settlement(_) => "settlement",
            StampChild::Stack(_) => "stack",
            StampChild::Height(_) => "height",
            StampChild::Identifier(_) => "identifier",
            StampChild::Locus(_) => "locus",
            StampChild::Country(_) => "country",
            StampChild::PeriodName(_) => "periodName",
            StampChild::Ptr(_) => "ptr",
            StampChild::Annot(_) => "annot",
            StampChild::Depth(_) => "depth",
            StampChild::Fig(_) => "fig",
            StampChild::Signatures(_) => "signatures",
            StampChild::Title(_) => "title",
            StampChild::P(_) => "p",
            StampChild::Name(_) => "name",
            StampChild::Symbol(_) => "symbol",
            StampChild::Abbr(_) => "abbr",
            StampChild::Extent(_) => "extent",
            StampChild::Region(_) => "region",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            StampChild::Text(_) => Vec::new(),
            StampChild::Q(e) => e.collect_all_attributes(),
            StampChild::Expan(e) => e.collect_all_attributes(),
            StampChild::Lb(e) => e.collect_all_attributes(),
            StampChild::CorpName(e) => e.collect_all_attributes(),
            StampChild::Ref(e) => e.collect_all_attributes(),
            StampChild::Bibl(e) => e.collect_all_attributes(),
            StampChild::PersName(e) => e.collect_all_attributes(),
            StampChild::Rend(e) => e.collect_all_attributes(),
            StampChild::Num(e) => e.collect_all_attributes(),
            StampChild::Stamp(e) => e.collect_all_attributes(),
            StampChild::Head(e) => e.collect_all_attributes(),
            StampChild::Date(e) => e.collect_all_attributes(),
            StampChild::Seg(e) => e.collect_all_attributes(),
            StampChild::Stack(e) => e.collect_all_attributes(),
            StampChild::Ptr(e) => e.collect_all_attributes(),
            StampChild::Annot(e) => e.collect_all_attributes(),
            StampChild::Fig(e) => e.collect_all_attributes(),
            StampChild::Title(e) => e.collect_all_attributes(),
            StampChild::P(e) => e.collect_all_attributes(),
            StampChild::Name(e) => e.collect_all_attributes(),
            StampChild::Abbr(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            StampChild::Text(_) => false,
            StampChild::Q(e) => e.has_children(),
            StampChild::Expan(e) => e.has_children(),
            StampChild::Lb(_) => false,
            StampChild::CorpName(e) => e.has_children(),
            StampChild::Ref(e) => e.has_children(),
            StampChild::Bibl(e) => e.has_children(),
            StampChild::PersName(e) => e.has_children(),
            StampChild::Rend(e) => e.has_children(),
            StampChild::Num(e) => e.has_children(),
            StampChild::Stamp(e) => e.has_children(),
            StampChild::Head(e) => e.has_children(),
            StampChild::Date(e) => e.has_children(),
            StampChild::Seg(e) => e.has_children(),
            StampChild::Stack(e) => e.has_children(),
            StampChild::Ptr(_) => false,
            StampChild::Annot(e) => e.has_children(),
            StampChild::Fig(e) => e.has_children(),
            StampChild::Title(e) => e.has_children(),
            StampChild::P(e) => e.has_children(),
            StampChild::Name(e) => e.has_children(),
            StampChild::Abbr(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StampChild::Q(e) => e.serialize_children(writer),
            StampChild::Expan(e) => e.serialize_children(writer),
            StampChild::CorpName(e) => e.serialize_children(writer),
            StampChild::Ref(e) => e.serialize_children(writer),
            StampChild::Bibl(e) => e.serialize_children(writer),
            StampChild::PersName(e) => e.serialize_children(writer),
            StampChild::Rend(e) => e.serialize_children(writer),
            StampChild::Num(e) => e.serialize_children(writer),
            StampChild::Stamp(e) => e.serialize_children(writer),
            StampChild::Head(e) => e.serialize_children(writer),
            StampChild::Date(e) => e.serialize_children(writer),
            StampChild::Seg(e) => e.serialize_children(writer),
            StampChild::Stack(e) => e.serialize_children(writer),
            StampChild::Annot(e) => e.serialize_children(writer),
            StampChild::Fig(e) => e.serialize_children(writer),
            StampChild::Title(e) => e.serialize_children(writer),
            StampChild::P(e) => e.serialize_children(writer),
            StampChild::Name(e) => e.serialize_children(writer),
            StampChild::Abbr(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StampChild::Text(text) => {
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

//! Serializer implementations for bibliographic elements.
//!
//! Contains: SeriesStmt, EditionStmt, Edition, NotesStmt, Annot, Extent, BiblScope, Contents.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Annot, AnnotChild, Bibl, BiblChild, BiblScope, BiblScopeChild, BiblStruct, BiblStructChild,
    Contents, ContentsChild, Edition, EditionChild, EditionStmt, EditionStmtChild, Extent,
    ExtentChild, Imprint, ImprintChild, Locus, LocusChild, LocusGrp, LocusGrpChild, NotesStmt,
    NotesStmtChild, SeriesStmt, SeriesStmtChild,
};

// ============================================================================
// SeriesStmt
// ============================================================================

impl MeiSerialize for SeriesStmt {
    fn element_name(&self) -> &'static str {
        "seriesStmt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for SeriesStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            SeriesStmtChild::Contributor(_) => "contributor",
            SeriesStmtChild::Identifier(_) => "identifier",
            SeriesStmtChild::Head(_) => "head",
            SeriesStmtChild::Editor(_) => "editor",
            SeriesStmtChild::BiblScope(_) => "biblScope",
            SeriesStmtChild::Contents(_) => "contents",
            SeriesStmtChild::Creator(_) => "creator",
            SeriesStmtChild::Sponsor(_) => "sponsor",
            SeriesStmtChild::Funder(_) => "funder",
            SeriesStmtChild::RespStmt(_) => "respStmt",
            SeriesStmtChild::SeriesStmt(_) => "seriesStmt",
            SeriesStmtChild::Title(_) => "title",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SeriesStmtChild::Contributor(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Identifier(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Head(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Editor(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::BiblScope(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Contents(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Creator(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Sponsor(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Funder(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::RespStmt(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::SeriesStmt(elem) => elem.serialize_mei(writer),
            SeriesStmtChild::Title(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// EditionStmt
// ============================================================================

impl MeiSerialize for EditionStmt {
    fn element_name(&self) -> &'static str {
        "editionStmt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for EditionStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            EditionStmtChild::Head(_) => "head",
            EditionStmtChild::Editor(_) => "editor",
            EditionStmtChild::Edition(_) => "edition",
            EditionStmtChild::RespStmt(_) => "respStmt",
            EditionStmtChild::Sponsor(_) => "sponsor",
            EditionStmtChild::Contributor(_) => "contributor",
            EditionStmtChild::Funder(_) => "funder",
            EditionStmtChild::Creator(_) => "creator",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EditionStmtChild::Head(elem) => elem.serialize_mei(writer),
            EditionStmtChild::Editor(elem) => elem.serialize_mei(writer),
            EditionStmtChild::Edition(elem) => elem.serialize_mei(writer),
            EditionStmtChild::RespStmt(elem) => elem.serialize_mei(writer),
            EditionStmtChild::Sponsor(elem) => elem.serialize_mei(writer),
            EditionStmtChild::Contributor(elem) => elem.serialize_mei(writer),
            EditionStmtChild::Funder(elem) => elem.serialize_mei(writer),
            EditionStmtChild::Creator(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Edition
// ============================================================================

impl MeiSerialize for Edition {
    fn element_name(&self) -> &'static str {
        "edition"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for EditionChild {
    fn element_name(&self) -> &'static str {
        match self {
            EditionChild::Text(_) => "#text",
            EditionChild::Rend(_) => "rend",
            EditionChild::Title(_) => "title",
            EditionChild::Name(_) => "name",
            EditionChild::PersName(_) => "persName",
            EditionChild::CorpName(_) => "corpName",
            EditionChild::Date(_) => "date",
            EditionChild::Identifier(_) => "identifier",
            EditionChild::Editor(_) => "editor",
            EditionChild::RespStmt(_) => "respStmt",
            EditionChild::Lb(_) => "lb",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, EditionChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EditionChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            EditionChild::Rend(elem) => elem.serialize_mei(writer),
            EditionChild::Title(elem) => elem.serialize_mei(writer),
            EditionChild::Name(elem) => elem.serialize_mei(writer),
            EditionChild::PersName(elem) => elem.serialize_mei(writer),
            EditionChild::CorpName(elem) => elem.serialize_mei(writer),
            EditionChild::Date(elem) => elem.serialize_mei(writer),
            EditionChild::Identifier(elem) => elem.serialize_mei(writer),
            EditionChild::Editor(elem) => elem.serialize_mei(writer),
            EditionChild::RespStmt(elem) => elem.serialize_mei(writer),
            EditionChild::Lb(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "EditionChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// NotesStmt
// ============================================================================

impl MeiSerialize for NotesStmt {
    fn element_name(&self) -> &'static str {
        "notesStmt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for NotesStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            NotesStmtChild::Head(_) => "head",
            NotesStmtChild::Annot(_) => "annot",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NotesStmtChild::Head(elem) => elem.serialize_mei(writer),
            NotesStmtChild::Annot(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Annot
// ============================================================================

impl MeiSerialize for Annot {
    fn element_name(&self) -> &'static str {
        "annot"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        // Note: audience, plist, annot_anl, annot_ges, annot_log, annot_vis
        // don't have CollectAttributes impls yet
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

impl MeiSerialize for AnnotChild {
    fn element_name(&self) -> &'static str {
        match self {
            AnnotChild::Text(_) => "#text",
            AnnotChild::P(_) => "p",
            AnnotChild::Head(_) => "head",
            AnnotChild::Rend(_) => "rend",
            AnnotChild::Name(_) => "name",
            AnnotChild::PersName(_) => "persName",
            AnnotChild::CorpName(_) => "corpName",
            AnnotChild::Date(_) => "date",
            AnnotChild::Identifier(_) => "identifier",
            AnnotChild::Lb(_) => "lb",
            AnnotChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AnnotChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AnnotChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AnnotChild::P(elem) => elem.serialize_mei(writer),
            AnnotChild::Head(elem) => elem.serialize_mei(writer),
            AnnotChild::Rend(elem) => elem.serialize_mei(writer),
            AnnotChild::Name(elem) => elem.serialize_mei(writer),
            AnnotChild::PersName(elem) => elem.serialize_mei(writer),
            AnnotChild::CorpName(elem) => elem.serialize_mei(writer),
            AnnotChild::Date(elem) => elem.serialize_mei(writer),
            AnnotChild::Identifier(elem) => elem.serialize_mei(writer),
            AnnotChild::Lb(elem) => elem.serialize_mei(writer),
            AnnotChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "AnnotChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Extent
// ============================================================================

impl MeiSerialize for Extent {
    fn element_name(&self) -> &'static str {
        "extent"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Note: quantity doesn't have CollectAttributes impl yet
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

impl MeiSerialize for ExtentChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExtentChild::Text(_) => "#text",
            ExtentChild::Rend(_) => "rend",
            ExtentChild::Name(_) => "name",
            ExtentChild::PersName(_) => "persName",
            ExtentChild::CorpName(_) => "corpName",
            ExtentChild::Date(_) => "date",
            ExtentChild::Identifier(_) => "identifier",
            ExtentChild::Lb(_) => "lb",
            ExtentChild::Title(_) => "title",
            ExtentChild::Address(_) => "address",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, ExtentChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ExtentChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            ExtentChild::Rend(elem) => elem.serialize_mei(writer),
            ExtentChild::Name(elem) => elem.serialize_mei(writer),
            ExtentChild::PersName(elem) => elem.serialize_mei(writer),
            ExtentChild::CorpName(elem) => elem.serialize_mei(writer),
            ExtentChild::Date(elem) => elem.serialize_mei(writer),
            ExtentChild::Identifier(elem) => elem.serialize_mei(writer),
            ExtentChild::Lb(elem) => elem.serialize_mei(writer),
            ExtentChild::Title(elem) => elem.serialize_mei(writer),
            ExtentChild::Address(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ExtentChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// BiblScope
// ============================================================================

impl MeiSerialize for BiblScope {
    fn element_name(&self) -> &'static str {
        "biblScope"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        // Note: extent doesn't have CollectAttributes impl yet
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

impl MeiSerialize for BiblScopeChild {
    fn element_name(&self) -> &'static str {
        match self {
            BiblScopeChild::Text(_) => "#text",
            BiblScopeChild::Rend(_) => "rend",
            BiblScopeChild::Name(_) => "name",
            BiblScopeChild::PersName(_) => "persName",
            BiblScopeChild::CorpName(_) => "corpName",
            BiblScopeChild::Date(_) => "date",
            BiblScopeChild::Identifier(_) => "identifier",
            BiblScopeChild::Lb(_) => "lb",
            BiblScopeChild::Title(_) => "title",
            BiblScopeChild::Address(_) => "address",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, BiblScopeChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BiblScopeChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            BiblScopeChild::Rend(elem) => elem.serialize_mei(writer),
            BiblScopeChild::Name(elem) => elem.serialize_mei(writer),
            BiblScopeChild::PersName(elem) => elem.serialize_mei(writer),
            BiblScopeChild::CorpName(elem) => elem.serialize_mei(writer),
            BiblScopeChild::Date(elem) => elem.serialize_mei(writer),
            BiblScopeChild::Identifier(elem) => elem.serialize_mei(writer),
            BiblScopeChild::Lb(elem) => elem.serialize_mei(writer),
            BiblScopeChild::Title(elem) => elem.serialize_mei(writer),
            BiblScopeChild::Address(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "BiblScopeChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Contents
// ============================================================================

impl MeiSerialize for Contents {
    fn element_name(&self) -> &'static str {
        "contents"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for ContentsChild {
    fn element_name(&self) -> &'static str {
        match self {
            ContentsChild::P(_) => "p",
            ContentsChild::ContentItem(_) => "contentItem",
            ContentsChild::Head(_) => "head",
            ContentsChild::Label(_) => "label",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ContentsChild::P(elem) => elem.serialize_mei(writer),
            ContentsChild::Head(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ContentsChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Bibl
// ============================================================================

impl MeiSerialize for Bibl {
    fn element_name(&self) -> &'static str {
        "bibl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for BiblChild {
    fn element_name(&self) -> &'static str {
        match self {
            BiblChild::Text(_) => "#text",
            BiblChild::Editor(_) => "editor",
            BiblChild::Bibl(_) => "bibl",
            BiblChild::BiblStruct(_) => "biblStruct",
            BiblChild::Distributor(_) => "distributor",
            BiblChild::Height(_) => "height",
            BiblChild::Stamp(_) => "stamp",
            BiblChild::Identifier(_) => "identifier",
            BiblChild::Repository(_) => "repository",
            BiblChild::RelationList(_) => "relationList",
            BiblChild::SecFolio(_) => "secFolio",
            BiblChild::Heraldry(_) => "heraldry",
            BiblChild::BiblScope(_) => "biblScope",
            BiblChild::Publisher(_) => "publisher",
            BiblChild::Ref(_) => "ref",
            BiblChild::Title(_) => "title",
            BiblChild::LocusGrp(_) => "locusGrp",
            BiblChild::PubPlace(_) => "pubPlace",
            BiblChild::Bloc(_) => "bloc",
            BiblChild::Street(_) => "street",
            BiblChild::Address(_) => "address",
            BiblChild::Fig(_) => "fig",
            BiblChild::PersName(_) => "persName",
            BiblChild::PhysLoc(_) => "physLoc",
            BiblChild::Q(_) => "q",
            BiblChild::Relation(_) => "relation",
            BiblChild::CorpName(_) => "corpName",
            BiblChild::PerfDuration(_) => "perfDuration",
            BiblChild::PostBox(_) => "postBox",
            BiblChild::Abbr(_) => "abbr",
            BiblChild::Term(_) => "term",
            BiblChild::Pb(_) => "pb",
            BiblChild::Annot(_) => "annot",
            BiblChild::Lb(_) => "lb",
            BiblChild::StyleName(_) => "styleName",
            BiblChild::Stack(_) => "stack",
            BiblChild::Country(_) => "country",
            BiblChild::Ptr(_) => "ptr",
            BiblChild::GeogFeat(_) => "geogFeat",
            BiblChild::TextLang(_) => "textLang",
            BiblChild::RespStmt(_) => "respStmt",
            BiblChild::Dim(_) => "dim",
            BiblChild::District(_) => "district",
            BiblChild::Date(_) => "date",
            BiblChild::Num(_) => "num",
            BiblChild::Contributor(_) => "contributor",
            BiblChild::Availability(_) => "availability",
            BiblChild::Settlement(_) => "settlement",
            BiblChild::Creation(_) => "creation",
            BiblChild::Series(_) => "series",
            BiblChild::Edition(_) => "edition",
            BiblChild::Dimensions(_) => "dimensions",
            BiblChild::Creator(_) => "creator",
            BiblChild::Expan(_) => "expan",
            BiblChild::Width(_) => "width",
            BiblChild::Seg(_) => "seg",
            BiblChild::Funder(_) => "funder",
            BiblChild::Rend(_) => "rend",
            BiblChild::Imprint(_) => "imprint",
            BiblChild::Region(_) => "region",
            BiblChild::Symbol(_) => "symbol",
            BiblChild::Genre(_) => "genre",
            BiblChild::RelatedItem(_) => "relatedItem",
            BiblChild::GeogName(_) => "geogName",
            BiblChild::Locus(_) => "locus",
            BiblChild::Signatures(_) => "signatures",
            BiblChild::Catchwords(_) => "catchwords",
            BiblChild::Depth(_) => "depth",
            BiblChild::PostCode(_) => "postCode",
            BiblChild::Unpub(_) => "unpub",
            BiblChild::Name(_) => "name",
            BiblChild::PeriodName(_) => "periodName",
            BiblChild::Extent(_) => "extent",
            BiblChild::Recipient(_) => "recipient",
            BiblChild::Sponsor(_) => "sponsor",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, BiblChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BiblChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            BiblChild::Editor(elem) => elem.serialize_mei(writer),
            BiblChild::Bibl(elem) => elem.serialize_mei(writer),
            BiblChild::BiblStruct(elem) => elem.serialize_mei(writer),
            BiblChild::Distributor(elem) => elem.serialize_mei(writer),
            BiblChild::Identifier(elem) => elem.serialize_mei(writer),
            BiblChild::BiblScope(elem) => elem.serialize_mei(writer),
            BiblChild::Publisher(elem) => elem.serialize_mei(writer),
            BiblChild::Ref(elem) => elem.serialize_mei(writer),
            BiblChild::Title(elem) => elem.serialize_mei(writer),
            BiblChild::LocusGrp(elem) => elem.serialize_mei(writer),
            BiblChild::PubPlace(elem) => elem.serialize_mei(writer),
            BiblChild::Address(elem) => elem.serialize_mei(writer),
            BiblChild::PersName(elem) => elem.serialize_mei(writer),
            BiblChild::CorpName(elem) => elem.serialize_mei(writer),
            BiblChild::Annot(elem) => elem.serialize_mei(writer),
            BiblChild::Lb(elem) => elem.serialize_mei(writer),
            BiblChild::Country(elem) => elem.serialize_mei(writer),
            BiblChild::Ptr(elem) => elem.serialize_mei(writer),
            BiblChild::RespStmt(elem) => elem.serialize_mei(writer),
            BiblChild::Date(elem) => elem.serialize_mei(writer),
            BiblChild::Num(elem) => elem.serialize_mei(writer),
            BiblChild::Contributor(elem) => elem.serialize_mei(writer),
            BiblChild::Availability(elem) => elem.serialize_mei(writer),
            BiblChild::Settlement(elem) => elem.serialize_mei(writer),
            BiblChild::Creation(elem) => elem.serialize_mei(writer),
            BiblChild::Edition(elem) => elem.serialize_mei(writer),
            BiblChild::Creator(elem) => elem.serialize_mei(writer),
            BiblChild::Funder(elem) => elem.serialize_mei(writer),
            BiblChild::Rend(elem) => elem.serialize_mei(writer),
            BiblChild::GeogName(elem) => elem.serialize_mei(writer),
            BiblChild::Locus(elem) => elem.serialize_mei(writer),
            BiblChild::PostCode(elem) => elem.serialize_mei(writer),
            BiblChild::Unpub(elem) => elem.serialize_mei(writer),
            BiblChild::Name(elem) => elem.serialize_mei(writer),
            BiblChild::Extent(elem) => elem.serialize_mei(writer),
            BiblChild::Sponsor(elem) => elem.serialize_mei(writer),
            BiblChild::Imprint(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "BiblChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// BiblStruct
// ============================================================================

impl MeiSerialize for BiblStruct {
    fn element_name(&self) -> &'static str {
        "biblStruct"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.record_type.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
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

impl MeiSerialize for BiblStructChild {
    fn element_name(&self) -> &'static str {
        match self {
            BiblStructChild::RelatedItem(_) => "relatedItem",
            BiblStructChild::Analytic(_) => "analytic",
            BiblStructChild::Monogr(_) => "monogr",
            BiblStructChild::Annot(_) => "annot",
            BiblStructChild::Series(_) => "series",
            BiblStructChild::Identifier(_) => "identifier",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BiblStructChild::Annot(elem) => elem.serialize_mei(writer),
            BiblStructChild::Identifier(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "BiblStructChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Locus
// ============================================================================

impl MeiSerialize for Locus {
    fn element_name(&self) -> &'static str {
        "locus"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.foliation_scheme.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        if let Some(ref from) = self.from {
            attrs.push(("from", from.to_string()));
        }
        if let Some(ref to) = self.to {
            attrs.push(("to", to.to_string()));
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

impl MeiSerialize for LocusChild {
    fn element_name(&self) -> &'static str {
        match self {
            LocusChild::Text(_) => "#text",
            LocusChild::Rend(_) => "rend",
            LocusChild::Locus(_) => "locus",
            LocusChild::Symbol(_) => "symbol",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, LocusChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LocusChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            LocusChild::Rend(elem) => elem.serialize_mei(writer),
            LocusChild::Locus(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "LocusChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// LocusGrp
// ============================================================================

impl MeiSerialize for LocusGrp {
    fn element_name(&self) -> &'static str {
        "locusGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.foliation_scheme.collect_attributes());
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

impl MeiSerialize for LocusGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            LocusGrpChild::Locus(_) => "locus",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LocusGrpChild::Locus(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Imprint
// ============================================================================

impl MeiSerialize for Imprint {
    fn element_name(&self) -> &'static str {
        "imprint"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
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

impl MeiSerialize for ImprintChild {
    fn element_name(&self) -> &'static str {
        match self {
            ImprintChild::Text(_) => "#text",
            ImprintChild::SecFolio(_) => "secFolio",
            ImprintChild::Term(_) => "term",
            ImprintChild::Depth(_) => "depth",
            ImprintChild::PostBox(_) => "postBox",
            ImprintChild::Settlement(_) => "settlement",
            ImprintChild::Width(_) => "width",
            ImprintChild::Title(_) => "title",
            ImprintChild::RespStmt(_) => "respStmt",
            ImprintChild::Signatures(_) => "signatures",
            ImprintChild::Restore(_) => "restore",
            ImprintChild::Address(_) => "address",
            ImprintChild::CorpName(_) => "corpName",
            ImprintChild::Identifier(_) => "identifier",
            ImprintChild::Add(_) => "add",
            ImprintChild::StyleName(_) => "styleName",
            ImprintChild::BiblStruct(_) => "biblStruct",
            ImprintChild::Availability(_) => "availability",
            ImprintChild::Damage(_) => "damage",
            ImprintChild::Del(_) => "del",
            ImprintChild::PersName(_) => "persName",
            ImprintChild::Distributor(_) => "distributor",
            ImprintChild::Region(_) => "region",
            ImprintChild::Stamp(_) => "stamp",
            ImprintChild::LocusGrp(_) => "locusGrp",
            ImprintChild::Lb(_) => "lb",
            ImprintChild::Ptr(_) => "ptr",
            ImprintChild::Repository(_) => "repository",
            ImprintChild::Stack(_) => "stack",
            ImprintChild::PostCode(_) => "postCode",
            ImprintChild::District(_) => "district",
            ImprintChild::Dimensions(_) => "dimensions",
            ImprintChild::Gap(_) => "gap",
            ImprintChild::Height(_) => "height",
            ImprintChild::Unclear(_) => "unclear",
            ImprintChild::Ref(_) => "ref",
            ImprintChild::GeogFeat(_) => "geogFeat",
            ImprintChild::Annot(_) => "annot",
            ImprintChild::GeogName(_) => "geogName",
            ImprintChild::Date(_) => "date",
            ImprintChild::Heraldry(_) => "heraldry",
            ImprintChild::Unpub(_) => "unpub",
            ImprintChild::HandShift(_) => "handShift",
            ImprintChild::Country(_) => "country",
            ImprintChild::Locus(_) => "locus",
            ImprintChild::Abbr(_) => "abbr",
            ImprintChild::Pb(_) => "pb",
            ImprintChild::Dim(_) => "dim",
            ImprintChild::Expan(_) => "expan",
            ImprintChild::Seg(_) => "seg",
            ImprintChild::PubPlace(_) => "pubPlace",
            ImprintChild::PeriodName(_) => "periodName",
            ImprintChild::Publisher(_) => "publisher",
            ImprintChild::Q(_) => "q",
            ImprintChild::Symbol(_) => "symbol",
            ImprintChild::Bloc(_) => "bloc",
            ImprintChild::Relation(_) => "relation",
            ImprintChild::Street(_) => "street",
            ImprintChild::Supplied(_) => "supplied",
            ImprintChild::RelationList(_) => "relationList",
            ImprintChild::Name(_) => "name",
            ImprintChild::Bibl(_) => "bibl",
            ImprintChild::Reg(_) => "reg",
            ImprintChild::Fig(_) => "fig",
            ImprintChild::Num(_) => "num",
            ImprintChild::Catchwords(_) => "catchwords",
            ImprintChild::Extent(_) => "extent",
            ImprintChild::Orig(_) => "orig",
            ImprintChild::Rend(_) => "rend",
            ImprintChild::Corr(_) => "corr",
            ImprintChild::Sic(_) => "sic",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, ImprintChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ImprintChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            ImprintChild::Publisher(elem) => elem.serialize_mei(writer),
            ImprintChild::PubPlace(elem) => elem.serialize_mei(writer),
            ImprintChild::Date(elem) => elem.serialize_mei(writer),
            ImprintChild::Distributor(elem) => elem.serialize_mei(writer),
            ImprintChild::RespStmt(elem) => elem.serialize_mei(writer),
            ImprintChild::Identifier(elem) => elem.serialize_mei(writer),
            ImprintChild::Title(elem) => elem.serialize_mei(writer),
            ImprintChild::Availability(elem) => elem.serialize_mei(writer),
            ImprintChild::Extent(elem) => elem.serialize_mei(writer),
            ImprintChild::Address(elem) => elem.serialize_mei(writer),
            ImprintChild::Bibl(elem) => elem.serialize_mei(writer),
            ImprintChild::BiblStruct(elem) => elem.serialize_mei(writer),
            ImprintChild::PersName(elem) => elem.serialize_mei(writer),
            ImprintChild::CorpName(elem) => elem.serialize_mei(writer),
            ImprintChild::Name(elem) => elem.serialize_mei(writer),
            ImprintChild::GeogName(elem) => elem.serialize_mei(writer),
            ImprintChild::Annot(elem) => elem.serialize_mei(writer),
            ImprintChild::Lb(elem) => elem.serialize_mei(writer),
            ImprintChild::Ptr(elem) => elem.serialize_mei(writer),
            ImprintChild::Ref(elem) => elem.serialize_mei(writer),
            ImprintChild::Rend(elem) => elem.serialize_mei(writer),
            ImprintChild::Num(elem) => elem.serialize_mei(writer),
            ImprintChild::Unpub(elem) => elem.serialize_mei(writer),
            ImprintChild::PostCode(elem) => elem.serialize_mei(writer),
            ImprintChild::Country(elem) => elem.serialize_mei(writer),
            ImprintChild::Locus(elem) => elem.serialize_mei(writer),
            ImprintChild::LocusGrp(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ImprintChild::{}",
                other.element_name()
            ))),
        }
    }
}

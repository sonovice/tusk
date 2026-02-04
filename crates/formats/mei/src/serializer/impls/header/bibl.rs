//! Serializer implementations for bibliographic elements.
//!
//! Contains: SeriesStmt, EditionStmt, Edition, NotesStmt, Annot, Extent, BiblScope, Contents.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Annot, AnnotChild, BiblScope, BiblScopeChild, Contents, ContentsChild, Edition, EditionChild,
    EditionStmt, EditionStmtChild, Extent, ExtentChild, NotesStmt, NotesStmtChild, SeriesStmt,
    SeriesStmtChild,
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

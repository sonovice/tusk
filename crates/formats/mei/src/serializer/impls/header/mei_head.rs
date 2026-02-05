//! Serializer implementations for main MEI header container elements.
//!
//! Contains: MeiHead, FileDesc, TitleStmt, SourceDesc and their child enums.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    FileDesc, FileDescChild, Head, HeadChild, MeiHead, MeiHeadChild, Source, SourceChild,
    SourceDesc, SourceDescChild, Title, TitleChild, TitlePart, TitlePartChild, TitleStmt,
    TitleStmtChild,
};

// ============================================================================
// MeiHead
// ============================================================================

impl MeiSerialize for MeiHead {
    fn element_name(&self) -> &'static str {
        "meiHead"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.mei_version.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
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

impl MeiSerialize for MeiHeadChild {
    fn element_name(&self) -> &'static str {
        match self {
            MeiHeadChild::FileDesc(_) => "fileDesc",
            MeiHeadChild::EncodingDesc(_) => "encodingDesc",
            MeiHeadChild::WorkList(_) => "workList",
            MeiHeadChild::RevisionDesc(_) => "revisionDesc",
            MeiHeadChild::ManifestationList(_) => "manifestationList",
            MeiHeadChild::AltId(_) => "altId",
            MeiHeadChild::ExtMeta(_) => "extMeta",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeiHeadChild::FileDesc(elem) => elem.serialize_mei(writer),
            MeiHeadChild::EncodingDesc(elem) => elem.serialize_mei(writer),
            MeiHeadChild::RevisionDesc(elem) => elem.serialize_mei(writer),
            MeiHeadChild::WorkList(elem) => elem.serialize_mei(writer),
            MeiHeadChild::ManifestationList(elem) => elem.serialize_mei(writer),
            MeiHeadChild::AltId(elem) => elem.serialize_mei(writer),
            MeiHeadChild::ExtMeta(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// FileDesc
// ============================================================================

impl MeiSerialize for FileDesc {
    fn element_name(&self) -> &'static str {
        "fileDesc"
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

impl MeiSerialize for FileDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            FileDescChild::TitleStmt(_) => "titleStmt",
            FileDescChild::PubStmt(_) => "pubStmt",
            FileDescChild::SourceDesc(_) => "sourceDesc",
            FileDescChild::Extent(_) => "extent",
            FileDescChild::EditionStmt(_) => "editionStmt",
            FileDescChild::SeriesStmt(_) => "seriesStmt",
            FileDescChild::NotesStmt(_) => "notesStmt",
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
            FileDescChild::TitleStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::PubStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::SourceDesc(elem) => elem.serialize_mei(writer),
            FileDescChild::SeriesStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::EditionStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::NotesStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::Extent(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// TitleStmt
// ============================================================================

impl MeiSerialize for TitleStmt {
    fn element_name(&self) -> &'static str {
        "titleStmt"
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

impl MeiSerialize for TitleStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitleStmtChild::Title(_) => "title",
            TitleStmtChild::Creator(_) => "creator",
            TitleStmtChild::Editor(_) => "editor",
            TitleStmtChild::Funder(_) => "funder",
            TitleStmtChild::Head(_) => "head",
            TitleStmtChild::RespStmt(_) => "respStmt",
            TitleStmtChild::Contributor(_) => "contributor",
            TitleStmtChild::Sponsor(_) => "sponsor",
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
            TitleStmtChild::Title(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Head(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Creator(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Editor(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Funder(elem) => elem.serialize_mei(writer),
            TitleStmtChild::RespStmt(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Contributor(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Sponsor(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Title
// ============================================================================

impl MeiSerialize for Title {
    fn element_name(&self) -> &'static str {
        "title"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.filing.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        // Element-local attributes
        if let Some(level) = &self.level {
            attrs.push(("level", level.clone()));
        }
        if let Some(t) = &self.r#type {
            attrs.push(("type", t.clone()));
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

impl MeiSerialize for TitleChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitleChild::Text(_) => "#text",
            TitleChild::TitlePart(_) => "titlePart",
            TitleChild::Rend(_) => "rend",
            TitleChild::CorpName(_) => "corpName",
            TitleChild::Name(_) => "name",
            TitleChild::PersName(_) => "persName",
            TitleChild::Date(_) => "date",
            TitleChild::Lb(_) => "lb",
            TitleChild::Ref(_) => "ref",
            TitleChild::Identifier(_) => "identifier",
            TitleChild::Ptr(_) => "ptr",
            TitleChild::Address(_) => "address",
            TitleChild::Bibl(_) => "bibl",
            TitleChild::BiblStruct(_) => "biblStruct",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TitleChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            TitleChild::TitlePart(elem) => elem.serialize_mei(writer),
            TitleChild::Rend(elem) => elem.serialize_mei(writer),
            TitleChild::CorpName(elem) => elem.serialize_mei(writer),
            TitleChild::Name(elem) => elem.serialize_mei(writer),
            TitleChild::PersName(elem) => elem.serialize_mei(writer),
            TitleChild::Date(elem) => elem.serialize_mei(writer),
            TitleChild::Lb(elem) => elem.serialize_mei(writer),
            TitleChild::Ref(elem) => elem.serialize_mei(writer),
            TitleChild::Identifier(elem) => elem.serialize_mei(writer),
            TitleChild::Ptr(elem) => elem.serialize_mei(writer),
            TitleChild::Address(elem) => elem.serialize_mei(writer),
            TitleChild::Bibl(elem) => elem.serialize_mei(writer),
            TitleChild::BiblStruct(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "TitleChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// TitlePart
// ============================================================================

impl MeiSerialize for TitlePart {
    fn element_name(&self) -> &'static str {
        "titlePart"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.filing.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_integer.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        // Element-local attribute
        if let Some(ref type_) = self.r#type {
            attrs.push(("type", type_.clone()));
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

impl MeiSerialize for TitlePartChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitlePartChild::Text(_) => "#text",
            TitlePartChild::Date(_) => "date",
            TitlePartChild::Name(_) => "name",
            TitlePartChild::PersName(_) => "persName",
            TitlePartChild::CorpName(_) => "corpName",
            TitlePartChild::Rend(_) => "rend",
            TitlePartChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, TitlePartChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TitlePartChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            TitlePartChild::Date(elem) => elem.serialize_mei(writer),
            TitlePartChild::Name(elem) => elem.serialize_mei(writer),
            TitlePartChild::PersName(elem) => elem.serialize_mei(writer),
            TitlePartChild::CorpName(elem) => elem.serialize_mei(writer),
            TitlePartChild::Rend(elem) => elem.serialize_mei(writer),
            TitlePartChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "TitlePartChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Head
// ============================================================================

impl MeiSerialize for Head {
    fn element_name(&self) -> &'static str {
        "head"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
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

impl MeiSerialize for HeadChild {
    fn element_name(&self) -> &'static str {
        match self {
            HeadChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            HeadChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "HeadChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// SourceDesc
// ============================================================================

impl MeiSerialize for SourceDesc {
    fn element_name(&self) -> &'static str {
        "sourceDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
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

impl MeiSerialize for SourceDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            SourceDescChild::Head(_) => "head",
            SourceDescChild::Source(_) => "source",
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
            SourceDescChild::Head(elem) => elem.serialize_mei(writer),
            SourceDescChild::Source(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Source
// ============================================================================

impl MeiSerialize for Source {
    fn element_name(&self) -> &'static str {
        "source"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.component_type.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for SourceChild {
    fn element_name(&self) -> &'static str {
        match self {
            SourceChild::Head(_) => "head",
            SourceChild::Bibl(_) => "bibl",
            SourceChild::BiblStruct(_) => "biblStruct",
            SourceChild::Locus(_) => "locus",
            SourceChild::LocusGrp(_) => "locusGrp",
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
            SourceChild::Head(elem) => elem.serialize_mei(writer),
            SourceChild::Bibl(elem) => elem.serialize_mei(writer),
            SourceChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SourceChild::Locus(elem) => elem.serialize_mei(writer),
            SourceChild::LocusGrp(elem) => elem.serialize_mei(writer),
        }
    }
}

//! Serializer implementations for MEI header elements.
//!
//! This module contains implementations for MeiHead, FileDesc, TitleStmt, PubStmt,
//! and their child elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Address, AddressChild, AltId, AltIdChild, Annot, AnnotChild, AppInfo, AppInfoChild,
    Application, ApplicationChild, Availability, AvailabilityChild, BiblScope, BiblScopeChild,
    Category, CategoryChild, Change, ChangeChild, ChangeDesc, ChangeDescChild, ClassDecls,
    ClassDeclsChild, Contents, ContentsChild, Contributor, ContributorChild, CorpName,
    CorpNameChild, Correction, CorrectionChild, Creator, CreatorChild, Date, DateChild,
    Distributor, DistributorChild, Edition, EditionChild, EditionStmt, EditionStmtChild, Editor,
    EditorChild, EditorialDecl, EditorialDeclChild, EncodingDesc, EncodingDescChild, ExtMeta,
    ExtMetaChild, Extent, ExtentChild, FileDesc, FileDescChild, Funder, FunderChild, Head,
    HeadChild, Identifier, IdentifierChild, Interpretation, InterpretationChild, Lb, Manifestation,
    ManifestationChild, ManifestationList, ManifestationListChild, MeiHead, MeiHeadChild, Name,
    NameChild, Normalization, NormalizationChild, NotesStmt, NotesStmtChild, P, PChild, PersName,
    PersNameChild, ProjectDesc, ProjectDescChild, PubPlace, PubPlaceChild, PubStmt, PubStmtChild,
    Publisher, PublisherChild, Rend, RendChild, Resp, RespChild, RespStmt, RespStmtChild,
    RevisionDesc, RevisionDescChild, SamplingDecl, SamplingDeclChild, Segmentation,
    SegmentationChild, SeriesStmt, SeriesStmtChild, SourceDesc, SourceDescChild, Sponsor,
    SponsorChild, StdVals, StdValsChild, Taxonomy, TaxonomyChild, Title, TitleChild, TitlePart,
    TitlePartChild, TitleStmt, TitleStmtChild, Unpub, UnpubChild, Work, WorkChild, WorkList,
    WorkListChild,
};

// ============================================================================
// Header element implementations
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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for PubStmt {
    fn element_name(&self) -> &'static str {
        "pubStmt"
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

impl MeiSerialize for PubStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            PubStmtChild::Date(_) => "date",
            PubStmtChild::Publisher(_) => "publisher",
            PubStmtChild::Address(_) => "address",
            PubStmtChild::PubPlace(_) => "pubPlace",
            PubStmtChild::RespStmt(_) => "respStmt",
            PubStmtChild::Availability(_) => "availability",
            PubStmtChild::Identifier(_) => "identifier",
            PubStmtChild::Distributor(_) => "distributor",
            PubStmtChild::Head(_) => "head",
            PubStmtChild::Unpub(_) => "unpub",
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
            PubStmtChild::Date(elem) => elem.serialize_mei(writer),
            PubStmtChild::Head(elem) => elem.serialize_mei(writer),
            PubStmtChild::Publisher(elem) => elem.serialize_mei(writer),
            PubStmtChild::Address(elem) => elem.serialize_mei(writer),
            PubStmtChild::PubPlace(elem) => elem.serialize_mei(writer),
            PubStmtChild::RespStmt(elem) => elem.serialize_mei(writer),
            PubStmtChild::Availability(elem) => elem.serialize_mei(writer),
            PubStmtChild::Identifier(elem) => elem.serialize_mei(writer),
            PubStmtChild::Distributor(elem) => elem.serialize_mei(writer),
            PubStmtChild::Unpub(elem) => elem.serialize_mei(writer),
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for EncodingDesc {
    fn element_name(&self) -> &'static str {
        "encodingDesc"
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

impl MeiSerialize for EncodingDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            EncodingDescChild::AppInfo(_) => "appInfo",
            EncodingDescChild::EditorialDecl(_) => "editorialDecl",
            EncodingDescChild::ProjectDesc(_) => "projectDesc",
            EncodingDescChild::SamplingDecl(_) => "samplingDecl",
            EncodingDescChild::TagsDecl(_) => "tagsDecl",
            EncodingDescChild::ClassDecls(_) => "classDecls",
            EncodingDescChild::DomainsDecl(_) => "domainsDecl",
            EncodingDescChild::Head(_) => "head",
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
            EncodingDescChild::Head(elem) => elem.serialize_mei(writer),
            EncodingDescChild::AppInfo(elem) => elem.serialize_mei(writer),
            EncodingDescChild::ClassDecls(elem) => elem.serialize_mei(writer),
            EncodingDescChild::EditorialDecl(elem) => elem.serialize_mei(writer),
            EncodingDescChild::ProjectDesc(elem) => elem.serialize_mei(writer),
            EncodingDescChild::SamplingDecl(elem) => elem.serialize_mei(writer),
            EncodingDescChild::TagsDecl(_) => Ok(()), // TODO: implement TagsDecl serializer
            EncodingDescChild::DomainsDecl(_) => Ok(()), // TODO: implement DomainsDecl serializer
        }
    }
}

// ============================================================================
// AppInfo and related elements
// ============================================================================

impl MeiSerialize for AppInfo {
    fn element_name(&self) -> &'static str {
        "appInfo"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        self.common.collect_attributes()
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

impl MeiSerialize for AppInfoChild {
    fn element_name(&self) -> &'static str {
        match self {
            AppInfoChild::Application(_) => "application",
            AppInfoChild::Head(_) => "head",
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
            AppInfoChild::Application(elem) => elem.serialize_mei(writer),
            AppInfoChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Application {
    fn element_name(&self) -> &'static str {
        "application"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for ApplicationChild {
    fn element_name(&self) -> &'static str {
        match self {
            ApplicationChild::Name(_) => "name",
            ApplicationChild::Ptr(_) => "ptr",
            ApplicationChild::Ref(_) => "ref",
            ApplicationChild::P(_) => "p",
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
            ApplicationChild::Name(elem) => elem.serialize_mei(writer),
            ApplicationChild::P(elem) => elem.serialize_mei(writer),
            ApplicationChild::Ptr(_) => Ok(()), // TODO: implement Ptr serializer
            ApplicationChild::Ref(_) => Ok(()), // TODO: implement Ref serializer
        }
    }
}

// ============================================================================
// ClassDecls and related elements
// ============================================================================

impl MeiSerialize for ClassDecls {
    fn element_name(&self) -> &'static str {
        "classDecls"
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

impl MeiSerialize for ClassDeclsChild {
    fn element_name(&self) -> &'static str {
        match self {
            ClassDeclsChild::Head(_) => "head",
            ClassDeclsChild::Taxonomy(_) => "taxonomy",
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
            ClassDeclsChild::Head(elem) => elem.serialize_mei(writer),
            ClassDeclsChild::Taxonomy(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Taxonomy {
    fn element_name(&self) -> &'static str {
        "taxonomy"
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

impl MeiSerialize for TaxonomyChild {
    fn element_name(&self) -> &'static str {
        match self {
            TaxonomyChild::Head(_) => "head",
            TaxonomyChild::Bibl(_) => "bibl",
            TaxonomyChild::Taxonomy(_) => "taxonomy",
            TaxonomyChild::Desc(_) => "desc",
            TaxonomyChild::Category(_) => "category",
            TaxonomyChild::BiblStruct(_) => "biblStruct",
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
            TaxonomyChild::Head(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Taxonomy(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Category(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Bibl(_) => Ok(()), // TODO: implement Bibl serializer
            TaxonomyChild::Desc(_) => Ok(()), // TODO: implement Desc serializer
            TaxonomyChild::BiblStruct(_) => Ok(()), // TODO: implement BiblStruct serializer
        }
    }
}

impl MeiSerialize for Category {
    fn element_name(&self) -> &'static str {
        "category"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for CategoryChild {
    fn element_name(&self) -> &'static str {
        match self {
            CategoryChild::Label(_) => "label",
            CategoryChild::Desc(_) => "desc",
            CategoryChild::CatRel(_) => "catRel",
            CategoryChild::AltId(_) => "altId",
            CategoryChild::Category(_) => "category",
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
            CategoryChild::AltId(elem) => elem.serialize_mei(writer),
            CategoryChild::Category(elem) => elem.serialize_mei(writer),
            CategoryChild::Label(_) => Ok(()), // TODO: implement Label serializer
            CategoryChild::Desc(_) => Ok(()),  // TODO: implement Desc serializer
            CategoryChild::CatRel(_) => Ok(()), // TODO: implement CatRel serializer
        }
    }
}

// ============================================================================
// EditorialDecl and related elements
// ============================================================================

impl MeiSerialize for EditorialDecl {
    fn element_name(&self) -> &'static str {
        "editorialDecl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for EditorialDeclChild {
    fn element_name(&self) -> &'static str {
        match self {
            EditorialDeclChild::Segmentation(_) => "segmentation",
            EditorialDeclChild::StdVals(_) => "stdVals",
            EditorialDeclChild::Interpretation(_) => "interpretation",
            EditorialDeclChild::Normalization(_) => "normalization",
            EditorialDeclChild::Correction(_) => "correction",
            EditorialDeclChild::Head(_) => "head",
            EditorialDeclChild::P(_) => "p",
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
            EditorialDeclChild::Segmentation(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::StdVals(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Interpretation(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Normalization(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Correction(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Head(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Segmentation {
    fn element_name(&self) -> &'static str {
        "segmentation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for SegmentationChild {
    fn element_name(&self) -> &'static str {
        match self {
            SegmentationChild::P(_) => "p",
            SegmentationChild::Head(_) => "head",
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
            SegmentationChild::P(elem) => elem.serialize_mei(writer),
            SegmentationChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for StdVals {
    fn element_name(&self) -> &'static str {
        "stdVals"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for StdValsChild {
    fn element_name(&self) -> &'static str {
        match self {
            StdValsChild::P(_) => "p",
            StdValsChild::Head(_) => "head",
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
            StdValsChild::P(elem) => elem.serialize_mei(writer),
            StdValsChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Interpretation {
    fn element_name(&self) -> &'static str {
        "interpretation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for InterpretationChild {
    fn element_name(&self) -> &'static str {
        match self {
            InterpretationChild::Head(_) => "head",
            InterpretationChild::P(_) => "p",
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
            InterpretationChild::Head(elem) => elem.serialize_mei(writer),
            InterpretationChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Normalization {
    fn element_name(&self) -> &'static str {
        "normalization"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.regular_method.collect_attributes());
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

impl MeiSerialize for NormalizationChild {
    fn element_name(&self) -> &'static str {
        match self {
            NormalizationChild::Head(_) => "head",
            NormalizationChild::P(_) => "p",
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
            NormalizationChild::Head(elem) => elem.serialize_mei(writer),
            NormalizationChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Correction {
    fn element_name(&self) -> &'static str {
        "correction"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.regular_method.collect_attributes());
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

impl MeiSerialize for CorrectionChild {
    fn element_name(&self) -> &'static str {
        match self {
            CorrectionChild::Head(_) => "head",
            CorrectionChild::P(_) => "p",
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
            CorrectionChild::Head(elem) => elem.serialize_mei(writer),
            CorrectionChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ProjectDesc and related elements
// ============================================================================

impl MeiSerialize for ProjectDesc {
    fn element_name(&self) -> &'static str {
        "projectDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for ProjectDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            ProjectDescChild::P(_) => "p",
            ProjectDescChild::Head(_) => "head",
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
            ProjectDescChild::P(elem) => elem.serialize_mei(writer),
            ProjectDescChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// SamplingDecl and related elements
// ============================================================================

impl MeiSerialize for SamplingDecl {
    fn element_name(&self) -> &'static str {
        "samplingDecl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for SamplingDeclChild {
    fn element_name(&self) -> &'static str {
        match self {
            SamplingDeclChild::P(_) => "p",
            SamplingDeclChild::Head(_) => "head",
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
            SamplingDeclChild::P(elem) => elem.serialize_mei(writer),
            SamplingDeclChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for RevisionDesc {
    fn element_name(&self) -> &'static str {
        "revisionDesc"
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

impl MeiSerialize for RevisionDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            RevisionDescChild::Head(_) => "head",
            RevisionDescChild::Change(_) => "change",
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
            RevisionDescChild::Head(elem) => elem.serialize_mei(writer),
            RevisionDescChild::Change(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Change {
    fn element_name(&self) -> &'static str {
        "change"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for ChangeChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChangeChild::Date(_) => "date",
            ChangeChild::ChangeDesc(_) => "changeDesc",
            ChangeChild::RespStmt(_) => "respStmt",
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
            ChangeChild::Date(elem) => elem.serialize_mei(writer),
            ChangeChild::ChangeDesc(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for ChangeDesc {
    fn element_name(&self) -> &'static str {
        "changeDesc"
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

impl MeiSerialize for ChangeDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChangeDescChild::P(_) => "p",
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
            ChangeDescChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Date {
    fn element_name(&self) -> &'static str {
        "date"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.calendared.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
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

impl MeiSerialize for DateChild {
    fn element_name(&self) -> &'static str {
        match self {
            DateChild::Text(_) => "#text",
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
            DateChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for P {
    fn element_name(&self) -> &'static str {
        "p"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for PChild {
    fn element_name(&self) -> &'static str {
        match self {
            PChild::Text(_) => "#text",
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
            PChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// TitleStmt child element implementations (Creator, Editor, Funder, RespStmt, Contributor, Sponsor)
// ============================================================================

impl MeiSerialize for Creator {
    fn element_name(&self) -> &'static str {
        "creator"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for CreatorChild {
    fn element_name(&self) -> &'static str {
        match self {
            CreatorChild::Text(_) => "#text",
            CreatorChild::CorpName(_) => "corpName",
            CreatorChild::Name(_) => "name",
            CreatorChild::PersName(_) => "persName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, CreatorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CreatorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CreatorChild::CorpName(elem) => elem.serialize_mei(writer),
            CreatorChild::Name(elem) => elem.serialize_mei(writer),
            CreatorChild::PersName(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Editor {
    fn element_name(&self) -> &'static str {
        "editor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for EditorChild {
    fn element_name(&self) -> &'static str {
        match self {
            EditorChild::Text(_) => "#text",
            EditorChild::Name(_) => "name",
            EditorChild::CorpName(_) => "corpName",
            EditorChild::PersName(_) => "persName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, EditorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EditorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            EditorChild::Name(elem) => elem.serialize_mei(writer),
            EditorChild::CorpName(elem) => elem.serialize_mei(writer),
            EditorChild::PersName(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Funder {
    fn element_name(&self) -> &'static str {
        "funder"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for FunderChild {
    fn element_name(&self) -> &'static str {
        match self {
            FunderChild::Text(_) => "#text",
            FunderChild::CorpName(_) => "corpName",
            FunderChild::Name(_) => "name",
            FunderChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, FunderChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FunderChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            FunderChild::CorpName(elem) => elem.serialize_mei(writer),
            FunderChild::Name(elem) => elem.serialize_mei(writer),
            FunderChild::PersName(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for RespStmt {
    fn element_name(&self) -> &'static str {
        "respStmt"
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

impl MeiSerialize for RespStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            RespStmtChild::Resp(_) => "resp",
            RespStmtChild::CorpName(_) => "corpName",
            RespStmtChild::Annot(_) => "annot",
            RespStmtChild::Name(_) => "name",
            RespStmtChild::PersName(_) => "persName",
            RespStmtChild::Head(_) => "head",
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
            RespStmtChild::Resp(elem) => elem.serialize_mei(writer),
            RespStmtChild::CorpName(elem) => elem.serialize_mei(writer),
            RespStmtChild::Name(elem) => elem.serialize_mei(writer),
            RespStmtChild::PersName(elem) => elem.serialize_mei(writer),
            RespStmtChild::Head(elem) => elem.serialize_mei(writer),
            RespStmtChild::Annot(_) => Ok(()), // Annot needs separate implementation
        }
    }
}

impl MeiSerialize for Resp {
    fn element_name(&self) -> &'static str {
        "resp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
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

impl MeiSerialize for RespChild {
    fn element_name(&self) -> &'static str {
        match self {
            RespChild::Text(_) => "#text",
            RespChild::PersName(_) => "persName",
            RespChild::CorpName(_) => "corpName",
            RespChild::Name(_) => "name",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, RespChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RespChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RespChild::PersName(elem) => elem.serialize_mei(writer),
            RespChild::CorpName(elem) => elem.serialize_mei(writer),
            RespChild::Name(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Contributor {
    fn element_name(&self) -> &'static str {
        "contributor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for ContributorChild {
    fn element_name(&self) -> &'static str {
        match self {
            ContributorChild::Text(_) => "#text",
            ContributorChild::CorpName(_) => "corpName",
            ContributorChild::Name(_) => "name",
            ContributorChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, ContributorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ContributorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            ContributorChild::CorpName(elem) => elem.serialize_mei(writer),
            ContributorChild::Name(elem) => elem.serialize_mei(writer),
            ContributorChild::PersName(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Sponsor {
    fn element_name(&self) -> &'static str {
        "sponsor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for SponsorChild {
    fn element_name(&self) -> &'static str {
        match self {
            SponsorChild::Text(_) => "#text",
            SponsorChild::CorpName(_) => "corpName",
            SponsorChild::Name(_) => "name",
            SponsorChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, SponsorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SponsorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SponsorChild::CorpName(elem) => elem.serialize_mei(writer),
            SponsorChild::Name(elem) => elem.serialize_mei(writer),
            SponsorChild::PersName(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// Name-related element implementations (PersName, CorpName, Name)
// ============================================================================

impl MeiSerialize for PersName {
    fn element_name(&self) -> &'static str {
        "persName"
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

impl MeiSerialize for PersNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            PersNameChild::Text(_) => "#text",
            PersNameChild::CorpName(_) => "corpName",
            PersNameChild::Name(_) => "name",
            PersNameChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PersNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PersNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PersNameChild::CorpName(elem) => elem.serialize_mei(writer),
            PersNameChild::Name(elem) => elem.serialize_mei(writer),
            PersNameChild::PersName(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for CorpName {
    fn element_name(&self) -> &'static str {
        "corpName"
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

impl MeiSerialize for CorpNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            CorpNameChild::Text(_) => "#text",
            CorpNameChild::CorpName(_) => "corpName",
            CorpNameChild::Name(_) => "name",
            CorpNameChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, CorpNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CorpNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CorpNameChild::CorpName(elem) => elem.serialize_mei(writer),
            CorpNameChild::Name(elem) => elem.serialize_mei(writer),
            CorpNameChild::PersName(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Name {
    fn element_name(&self) -> &'static str {
        "name"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.name.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
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

impl MeiSerialize for NameChild {
    fn element_name(&self) -> &'static str {
        match self {
            NameChild::Text(_) => "#text",
            NameChild::CorpName(_) => "corpName",
            NameChild::Name(_) => "name",
            NameChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, NameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            NameChild::CorpName(elem) => elem.serialize_mei(writer),
            NameChild::Name(elem) => elem.serialize_mei(writer),
            NameChild::PersName(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// Rend element implementation (used by Title, TitlePart, and others)
// ============================================================================

impl MeiSerialize for Rend {
    fn element_name(&self) -> &'static str {
        "rend"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.color.collect_attributes());
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.ext_sym_auth.collect_attributes());
        attrs.extend(self.horizontal_align.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.text_rendition.collect_attributes());
        attrs.extend(self.typography.collect_attributes());
        attrs.extend(self.vertical_align.collect_attributes());
        attrs.extend(self.whitespace.collect_attributes());
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

impl MeiSerialize for RendChild {
    fn element_name(&self) -> &'static str {
        match self {
            RendChild::Text(_) => "#text",
            RendChild::Rend(_) => "rend",
            RendChild::Date(_) => "date",
            RendChild::Name(_) => "name",
            RendChild::PersName(_) => "persName",
            RendChild::CorpName(_) => "corpName",
            RendChild::Title(_) => "title",
            RendChild::Lb(_) => "lb",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, RendChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RendChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RendChild::Rend(elem) => elem.serialize_mei(writer),
            RendChild::Date(elem) => elem.serialize_mei(writer),
            RendChild::Name(elem) => elem.serialize_mei(writer),
            RendChild::PersName(elem) => elem.serialize_mei(writer),
            RendChild::CorpName(elem) => elem.serialize_mei(writer),
            RendChild::Title(elem) => elem.serialize_mei(writer),
            RendChild::Lb(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Lb {
    fn element_name(&self) -> &'static str {
        "lb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.source.collect_attributes());
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
// PubStmt child element implementations (Publisher, Address, PubPlace, Availability, Identifier, Distributor, Unpub)
// ============================================================================

impl MeiSerialize for Publisher {
    fn element_name(&self) -> &'static str {
        "publisher"
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

impl MeiSerialize for PublisherChild {
    fn element_name(&self) -> &'static str {
        match self {
            PublisherChild::Text(_) => "#text",
            PublisherChild::Date(_) => "date",
            PublisherChild::Name(_) => "name",
            PublisherChild::PersName(_) => "persName",
            PublisherChild::CorpName(_) => "corpName",
            PublisherChild::Address(_) => "address",
            PublisherChild::Identifier(_) => "identifier",
            PublisherChild::Rend(_) => "rend",
            PublisherChild::Lb(_) => "lb",
            PublisherChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PublisherChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PublisherChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PublisherChild::Date(elem) => elem.serialize_mei(writer),
            PublisherChild::Name(elem) => elem.serialize_mei(writer),
            PublisherChild::PersName(elem) => elem.serialize_mei(writer),
            PublisherChild::CorpName(elem) => elem.serialize_mei(writer),
            PublisherChild::Address(elem) => elem.serialize_mei(writer),
            PublisherChild::Identifier(elem) => elem.serialize_mei(writer),
            PublisherChild::Rend(elem) => elem.serialize_mei(writer),
            PublisherChild::Lb(elem) => elem.serialize_mei(writer),
            PublisherChild::Title(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Address {
    fn element_name(&self) -> &'static str {
        "address"
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

impl MeiSerialize for AddressChild {
    fn element_name(&self) -> &'static str {
        match self {
            AddressChild::Settlement(_) => "settlement",
            AddressChild::Country(_) => "country",
            AddressChild::PostCode(_) => "postCode",
            AddressChild::Street(_) => "street",
            AddressChild::District(_) => "district",
            AddressChild::Bloc(_) => "bloc",
            AddressChild::GeogFeat(_) => "geogFeat",
            AddressChild::PostBox(_) => "postBox",
            AddressChild::Region(_) => "region",
            AddressChild::AddrLine(_) => "addrLine",
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

    fn serialize_mei<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Address children need their own serializers - skip for now
        Ok(())
    }
}

impl MeiSerialize for PubPlace {
    fn element_name(&self) -> &'static str {
        "pubPlace"
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

impl MeiSerialize for PubPlaceChild {
    fn element_name(&self) -> &'static str {
        match self {
            PubPlaceChild::Text(_) => "#text",
            PubPlaceChild::Date(_) => "date",
            PubPlaceChild::Name(_) => "name",
            PubPlaceChild::PersName(_) => "persName",
            PubPlaceChild::CorpName(_) => "corpName",
            PubPlaceChild::Address(_) => "address",
            PubPlaceChild::Identifier(_) => "identifier",
            PubPlaceChild::Rend(_) => "rend",
            PubPlaceChild::Lb(_) => "lb",
            PubPlaceChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PubPlaceChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PubPlaceChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PubPlaceChild::Date(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Name(elem) => elem.serialize_mei(writer),
            PubPlaceChild::PersName(elem) => elem.serialize_mei(writer),
            PubPlaceChild::CorpName(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Address(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Identifier(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Rend(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Lb(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Title(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Availability {
    fn element_name(&self) -> &'static str {
        "availability"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for AvailabilityChild {
    fn element_name(&self) -> &'static str {
        match self {
            AvailabilityChild::Text(_) => "#text",
            AvailabilityChild::SysReq(_) => "sysReq",
            AvailabilityChild::Distributor(_) => "distributor",
            AvailabilityChild::Price(_) => "price",
            AvailabilityChild::Date(_) => "date",
            AvailabilityChild::Identifier(_) => "identifier",
            AvailabilityChild::AccessRestrict(_) => "accessRestrict",
            AvailabilityChild::Address(_) => "address",
            AvailabilityChild::Head(_) => "head",
            AvailabilityChild::UseRestrict(_) => "useRestrict",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AvailabilityChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AvailabilityChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AvailabilityChild::Date(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Distributor(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Identifier(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Address(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Head(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Identifier {
    fn element_name(&self) -> &'static str {
        "identifier"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
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

impl MeiSerialize for IdentifierChild {
    fn element_name(&self) -> &'static str {
        match self {
            IdentifierChild::Text(_) => "#text",
            IdentifierChild::Date(_) => "date",
            IdentifierChild::Name(_) => "name",
            IdentifierChild::PersName(_) => "persName",
            IdentifierChild::CorpName(_) => "corpName",
            IdentifierChild::Address(_) => "address",
            IdentifierChild::Identifier(_) => "identifier",
            IdentifierChild::Rend(_) => "rend",
            IdentifierChild::Lb(_) => "lb",
            IdentifierChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, IdentifierChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            IdentifierChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            IdentifierChild::Date(elem) => elem.serialize_mei(writer),
            IdentifierChild::Name(elem) => elem.serialize_mei(writer),
            IdentifierChild::PersName(elem) => elem.serialize_mei(writer),
            IdentifierChild::CorpName(elem) => elem.serialize_mei(writer),
            IdentifierChild::Address(elem) => elem.serialize_mei(writer),
            IdentifierChild::Identifier(elem) => elem.serialize_mei(writer),
            IdentifierChild::Rend(elem) => elem.serialize_mei(writer),
            IdentifierChild::Lb(elem) => elem.serialize_mei(writer),
            IdentifierChild::Title(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Distributor {
    fn element_name(&self) -> &'static str {
        "distributor"
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

impl MeiSerialize for DistributorChild {
    fn element_name(&self) -> &'static str {
        match self {
            DistributorChild::Text(_) => "#text",
            DistributorChild::Date(_) => "date",
            DistributorChild::Name(_) => "name",
            DistributorChild::PersName(_) => "persName",
            DistributorChild::CorpName(_) => "corpName",
            DistributorChild::Address(_) => "address",
            DistributorChild::Identifier(_) => "identifier",
            DistributorChild::Rend(_) => "rend",
            DistributorChild::Lb(_) => "lb",
            DistributorChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, DistributorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DistributorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DistributorChild::Date(elem) => elem.serialize_mei(writer),
            DistributorChild::Name(elem) => elem.serialize_mei(writer),
            DistributorChild::PersName(elem) => elem.serialize_mei(writer),
            DistributorChild::CorpName(elem) => elem.serialize_mei(writer),
            DistributorChild::Address(elem) => elem.serialize_mei(writer),
            DistributorChild::Identifier(elem) => elem.serialize_mei(writer),
            DistributorChild::Rend(elem) => elem.serialize_mei(writer),
            DistributorChild::Lb(elem) => elem.serialize_mei(writer),
            DistributorChild::Title(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Unpub {
    fn element_name(&self) -> &'static str {
        "unpub"
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

impl MeiSerialize for UnpubChild {
    fn element_name(&self) -> &'static str {
        match self {
            UnpubChild::Text(_) => "#text",
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
            UnpubChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// FileDescChild element implementations (SeriesStmt, EditionStmt, NotesStmt, Extent)
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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // ContentItem and Label need their own serializers
        }
    }
}

// ============================================================================
// WorkList element implementations
// ============================================================================

impl MeiSerialize for WorkList {
    fn element_name(&self) -> &'static str {
        "workList"
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

impl MeiSerialize for WorkListChild {
    fn element_name(&self) -> &'static str {
        match self {
            WorkListChild::Head(_) => "head",
            WorkListChild::Work(_) => "work",
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
            WorkListChild::Head(elem) => elem.serialize_mei(writer),
            WorkListChild::Work(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Work {
    fn element_name(&self) -> &'static str {
        "work"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for WorkChild {
    fn element_name(&self) -> &'static str {
        match self {
            WorkChild::Incip(_) => "incip",
            WorkChild::Meter(_) => "meter",
            WorkChild::Creation(_) => "creation",
            WorkChild::History(_) => "history",
            WorkChild::Mensuration(_) => "mensuration",
            WorkChild::PerfDuration(_) => "perfDuration",
            WorkChild::Context(_) => "context",
            WorkChild::NotesStmt(_) => "notesStmt",
            WorkChild::ExtMeta(_) => "extMeta",
            WorkChild::Dedication(_) => "dedication",
            WorkChild::BiblList(_) => "biblList",
            WorkChild::Title(_) => "title",
            WorkChild::Classification(_) => "classification",
            WorkChild::Head(_) => "head",
            WorkChild::Tempo(_) => "tempo",
            WorkChild::OtherChar(_) => "otherChar",
            WorkChild::RespStmt(_) => "respStmt",
            WorkChild::PerfMedium(_) => "perfMedium",
            WorkChild::Audience(_) => "audience",
            WorkChild::Key(_) => "key",
            WorkChild::Contents(_) => "contents",
            WorkChild::ExpressionList(_) => "expressionList",
            WorkChild::RelationList(_) => "relationList",
            WorkChild::ComponentList(_) => "componentList",
            WorkChild::LangUsage(_) => "langUsage",
            WorkChild::Identifier(_) => "identifier",
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
            WorkChild::Title(elem) => elem.serialize_mei(writer),
            WorkChild::Head(elem) => elem.serialize_mei(writer),
            WorkChild::RespStmt(elem) => elem.serialize_mei(writer),
            WorkChild::NotesStmt(elem) => elem.serialize_mei(writer),
            WorkChild::ExtMeta(elem) => elem.serialize_mei(writer),
            WorkChild::Identifier(elem) => elem.serialize_mei(writer),
            WorkChild::Contents(elem) => elem.serialize_mei(writer),
            // The following children need dedicated serializers - for now write empty element
            _ => {
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// ManifestationList element implementations
// ============================================================================

impl MeiSerialize for ManifestationList {
    fn element_name(&self) -> &'static str {
        "manifestationList"
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

impl MeiSerialize for ManifestationListChild {
    fn element_name(&self) -> &'static str {
        match self {
            ManifestationListChild::Head(_) => "head",
            ManifestationListChild::Manifestation(_) => "manifestation",
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
            ManifestationListChild::Head(elem) => elem.serialize_mei(writer),
            ManifestationListChild::Manifestation(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Manifestation {
    fn element_name(&self) -> &'static str {
        "manifestation"
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

impl MeiSerialize for ManifestationChild {
    fn element_name(&self) -> &'static str {
        match self {
            ManifestationChild::Contents(_) => "contents",
            ManifestationChild::Availability(_) => "availability",
            ManifestationChild::BiblList(_) => "biblList",
            ManifestationChild::Classification(_) => "classification",
            ManifestationChild::RelationList(_) => "relationList",
            ManifestationChild::SeriesStmt(_) => "seriesStmt",
            ManifestationChild::PubStmt(_) => "pubStmt",
            ManifestationChild::NotesStmt(_) => "notesStmt",
            ManifestationChild::LocusGrp(_) => "locusGrp",
            ManifestationChild::LangUsage(_) => "langUsage",
            ManifestationChild::ExtMeta(_) => "extMeta",
            ManifestationChild::Identifier(_) => "identifier",
            ManifestationChild::TitleStmt(_) => "titleStmt",
            ManifestationChild::Creation(_) => "creation",
            ManifestationChild::PhysLoc(_) => "physLoc",
            ManifestationChild::ComponentList(_) => "componentList",
            ManifestationChild::ItemList(_) => "itemList",
            ManifestationChild::EditionStmt(_) => "editionStmt",
            ManifestationChild::Dedication(_) => "dedication",
            ManifestationChild::History(_) => "history",
            ManifestationChild::Head(_) => "head",
            ManifestationChild::Locus(_) => "locus",
            ManifestationChild::PhysDesc(_) => "physDesc",
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
            ManifestationChild::Contents(elem) => elem.serialize_mei(writer),
            ManifestationChild::Availability(elem) => elem.serialize_mei(writer),
            ManifestationChild::SeriesStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::PubStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::NotesStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::ExtMeta(elem) => elem.serialize_mei(writer),
            ManifestationChild::Identifier(elem) => elem.serialize_mei(writer),
            ManifestationChild::TitleStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::EditionStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::Head(elem) => elem.serialize_mei(writer),
            // The following children need dedicated serializers - for now write empty element
            _ => {
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// AltId element implementations
// ============================================================================

impl MeiSerialize for AltId {
    fn element_name(&self) -> &'static str {
        "altId"
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

impl MeiSerialize for AltIdChild {
    fn element_name(&self) -> &'static str {
        match self {
            AltIdChild::Text(_) => "#text",
            AltIdChild::Stack(_) => "stack",
            AltIdChild::Lb(_) => "lb",
            AltIdChild::Rend(_) => "rend",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AltIdChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AltIdChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AltIdChild::Lb(elem) => elem.serialize_mei(writer),
            AltIdChild::Rend(elem) => elem.serialize_mei(writer),
            // Stack needs its own serializer - for now write empty element
            AltIdChild::Stack(_) => {
                let start = writer.start_element("stack")?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// ExtMeta element implementations
// ============================================================================

impl MeiSerialize for ExtMeta {
    fn element_name(&self) -> &'static str {
        "extMeta"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.whitespace.collect_attributes());
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

impl MeiSerialize for ExtMetaChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExtMetaChild::Text(_) => "#text",
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
            ExtMetaChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

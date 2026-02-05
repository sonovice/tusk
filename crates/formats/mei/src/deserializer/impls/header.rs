//! Deserializer implementations for MEI header elements.
//!
//! This module contains implementations for MeiHead, FileDesc, TitleStmt, PubStmt,
//! and their child elements.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    AddrLine, AddrLineChild, AltId, AltIdChild, Annot, AnnotChild, AppInfo, AppInfoChild,
    Application, ApplicationChild, Availability, AvailabilityChild, Bibl, BiblScope, BiblStruct,
    CatRel, CatRelChild, Category, CategoryChild, ClassDecls, ClassDeclsChild, Contributor,
    ContributorChild, CorpName, CorpNameChild, Correction, CorrectionChild, Creator, CreatorChild,
    Date, Distributor, Editor, EditorChild, EditorialDecl, EditorialDeclChild, EncodingDesc,
    EncodingDescChild, FileDesc, FileDescChild, Funder, FunderChild, GeogName, GeogNameChild, Head,
    HeadChild, Identifier, Imprint, Interpretation, InterpretationChild, Locus, LocusGrp, MeiHead,
    MeiHeadChild, Name, NameChild, Normalization, NormalizationChild, P, PChild, PersName,
    PersNameChild, ProjectDesc, ProjectDescChild, Ptr, PubPlace, PubStmt, PubStmtChild, Publisher,
    PublisherChild, Ref, RefChild, Resp, RespChild, RespStmt, RespStmtChild, SamplingDecl,
    SamplingDeclChild, Segmentation, SegmentationChild, Source, SourceChild, SourceDesc,
    SourceDescChild, Sponsor, SponsorChild, StdVals, StdValsChild, Taxonomy, TaxonomyChild, Title,
    TitleChild, TitlePart, TitlePartChild, TitleStmt, TitleStmtChild, Unpub, UseRestrict,
    UseRestrictChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// MeiHead element
// ============================================================================

impl MeiDeserialize for MeiHead {
    fn element_name() -> &'static str {
        "meiHead"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut mei_head = MeiHead::default();

        // Extract attributes into each attribute class
        mei_head.basic.extract_attributes(&mut attrs)?;
        mei_head.bibl.extract_attributes(&mut attrs)?;
        mei_head.labelled.extract_attributes(&mut attrs)?;
        mei_head.lang.extract_attributes(&mut attrs)?;
        mei_head.mei_version.extract_attributes(&mut attrs)?;
        mei_head.responsibility.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // meiHead can contain: altId, fileDesc, encodingDesc, workList,
        // manifestationList, extMeta, revisionDesc
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("meiHead")?
            {
                match name.as_str() {
                    "fileDesc" => {
                        let file_desc =
                            parse_file_desc_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
                    }
                    "encodingDesc" => {
                        let encoding_desc =
                            parse_encoding_desc_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::EncodingDesc(Box::new(encoding_desc)));
                    }
                    "workList" => {
                        let work_list =
                            super::parse_work_list_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::WorkList(Box::new(work_list)));
                    }
                    "revisionDesc" => {
                        let revision_desc = super::parse_revision_desc_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei_head
                            .children
                            .push(MeiHeadChild::RevisionDesc(Box::new(revision_desc)));
                    }
                    "manifestationList" => {
                        let manifestation_list = super::parse_manifestation_list_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei_head
                            .children
                            .push(MeiHeadChild::ManifestationList(Box::new(
                                manifestation_list,
                            )));
                    }
                    // Other child elements (extMeta, etc.) are not
                    // yet implemented for parsing. Skip them in lenient mode.
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(mei_head)
    }
}

/// Parse a `<fileDesc>` element from within another element.
pub(crate) fn parse_file_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<FileDesc> {
    let mut file_desc = FileDesc::default();

    // Extract attributes into each attribute class
    file_desc.common.extract_attributes(&mut attrs)?;
    file_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // fileDesc can contain: titleStmt, editionStmt, extent, pubStmt, seriesStmt,
    // notesStmt, sourceDesc
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("fileDesc")?
        {
            match name.as_str() {
                "titleStmt" => {
                    let title_stmt = parse_title_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
                }
                "pubStmt" => {
                    let pub_stmt = parse_pub_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));
                }
                "sourceDesc" => {
                    let source_desc =
                        parse_source_desc_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::SourceDesc(Box::new(source_desc)));
                }
                "seriesStmt" => {
                    let series_stmt =
                        super::parse_series_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::SeriesStmt(Box::new(series_stmt)));
                }
                "editionStmt" => {
                    let edition_stmt =
                        super::parse_edition_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::EditionStmt(Box::new(edition_stmt)));
                }
                "notesStmt" => {
                    let notes_stmt =
                        super::parse_notes_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::NotesStmt(Box::new(notes_stmt)));
                }
                "extent" => {
                    let extent = super::parse_extent_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::Extent(Box::new(extent)));
                }
                // Other child elements are not yet implemented for parsing.
                // Skip them in lenient mode.
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(file_desc)
}

/// Parse a `<titleStmt>` element from within another element.
pub(crate) fn parse_title_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TitleStmt> {
    let mut title_stmt = TitleStmt::default();

    // Extract attributes into each attribute class
    title_stmt.common.extract_attributes(&mut attrs)?;
    title_stmt.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // titleStmt can contain: head*, title+, respStmt*, and model.respLikePart
    // (editor, funder, sponsor, contributor, creator)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("titleStmt")?
        {
            match name.as_str() {
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Title(Box::new(title)));
                }
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Head(Box::new(head)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "editor" => {
                    let editor = parse_editor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Editor(Box::new(editor)));
                }
                "creator" => {
                    let creator = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                "funder" => {
                    let funder = parse_funder_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Funder(Box::new(funder)));
                }
                "sponsor" => {
                    let sponsor = parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Sponsor(Box::new(sponsor)));
                }
                "contributor" => {
                    let contributor =
                        parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Contributor(Box::new(contributor)));
                }
                // Handle deprecated MEI elements by converting to Creator with appropriate role
                "composer" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "composer",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Cmp,
                    )?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                "lyricist" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "lyricist",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Lyr,
                    )?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                "arranger" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "arranger",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Arr,
                    )?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                "author" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "author",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Aut,
                    )?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(title_stmt)
}

/// Parse a `<pubStmt>` element from within another element.
pub(crate) fn parse_pub_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PubStmt> {
    let mut pub_stmt = PubStmt::default();

    // Extract attributes into each attribute class
    pub_stmt.common.extract_attributes(&mut attrs)?;
    pub_stmt.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // pubStmt can contain: head*, (unpub | model.pubStmtPart*)
    // model.pubStmtPart includes: availability, address, date, identifier,
    // distributor, publisher, pubPlace, respStmt
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("pubStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Head(Box::new(head)));
                }
                "unpub" => {
                    let unpub = parse_unpub_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Unpub(Box::new(unpub)));
                }
                "publisher" => {
                    let publisher = parse_publisher_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Publisher(Box::new(publisher)));
                }
                "pubPlace" => {
                    let pub_place = parse_pub_place_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::PubPlace(Box::new(pub_place)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Date(Box::new(date)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Identifier(Box::new(identifier)));
                }
                "availability" => {
                    let availability =
                        parse_availability_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Availability(Box::new(availability)));
                }
                "distributor" => {
                    let distributor =
                        parse_distributor_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Distributor(Box::new(distributor)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "address" => {
                    let address = parse_address_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Address(Box::new(address)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(pub_stmt)
}

/// Parse a `<sourceDesc>` element from within another element.
pub(crate) fn parse_source_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SourceDesc> {
    let mut source_desc = SourceDesc::default();

    // Extract attributes into AttCommon (sourceDesc only has common attributes)
    source_desc.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // sourceDesc can contain: head*, source+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("sourceDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    source_desc
                        .children
                        .push(SourceDescChild::Head(Box::new(head)));
                }
                "source" => {
                    let source = parse_source_from_event(reader, child_attrs, child_empty)?;
                    source_desc
                        .children
                        .push(SourceDescChild::Source(Box::new(source)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(source_desc)
}

/// Parse a `<source>` element from within another element.
pub(crate) fn parse_source_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Source> {
    let mut source = Source::default();

    // Extract attributes into each attribute class
    source.common.extract_attributes(&mut attrs)?;
    source.authorized.extract_attributes(&mut attrs)?;
    source.bibl.extract_attributes(&mut attrs)?;
    source.component_type.extract_attributes(&mut attrs)?;
    source.data_pointing.extract_attributes(&mut attrs)?;
    source.pointing.extract_attributes(&mut attrs)?;
    source.record_type.extract_attributes(&mut attrs)?;
    source.target_eval.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // source can contain: head*, (locus | locusGrp)*, (bibl | biblStruct)*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("source")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Head(Box::new(head)));
                }
                "locus" => {
                    let locus = parse_locus_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Locus(Box::new(locus)));
                }
                "locusGrp" => {
                    let locus_grp = parse_locus_grp_from_event(reader, child_attrs, child_empty)?;
                    source
                        .children
                        .push(SourceChild::LocusGrp(Box::new(locus_grp)));
                }
                "bibl" => {
                    let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    source
                        .children
                        .push(SourceChild::BiblStruct(Box::new(bibl_struct)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(source)
}

/// Parse a `<bibl>` element from within another element.
///
/// Bibl can contain mixed content (text and many child elements).
pub(crate) fn parse_bibl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Bibl> {
    use tusk_model::elements::BiblChild;

    let mut bibl = Bibl::default();

    // Extract attributes
    bibl.common.extract_attributes(&mut attrs)?;
    bibl.bibl.extract_attributes(&mut attrs)?;
    bibl.facsimile.extract_attributes(&mut attrs)?;
    bibl.lang.extract_attributes(&mut attrs)?;
    bibl.pointing.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("bibl")? {
            match content {
                MixedContent::Text(text) => {
                    bibl.children.push(BiblChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            bibl.children.push(BiblChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            bibl.children
                                .push(BiblChild::Identifier(Box::new(identifier)));
                        }
                        "creator" => {
                            let creator =
                                parse_creator_from_event(reader, child_attrs, child_empty)?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        // Handle deprecated MEI elements by converting to Creator
                        "composer" => {
                            let creator = parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "composer",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Cmp,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "lyricist" => {
                            let creator = parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "lyricist",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Lyr,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "arranger" => {
                            let creator = parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "arranger",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Arr,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "author" => {
                            let creator = parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "author",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Aut,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "librettist" => {
                            let creator = parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "librettist",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Lbt,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "imprint" => {
                            let imprint =
                                parse_imprint_from_event(reader, child_attrs, child_empty)?;
                            bibl.children.push(BiblChild::Imprint(Box::new(imprint)));
                        }
                        "editor" => {
                            let editor = parse_editor_from_event(reader, child_attrs, child_empty)?;
                            bibl.children.push(BiblChild::Editor(Box::new(editor)));
                        }
                        "biblScope" => {
                            let bibl_scope =
                                parse_bibl_scope_from_event(reader, child_attrs, child_empty)?;
                            bibl.children
                                .push(BiblChild::BiblScope(Box::new(bibl_scope)));
                        }
                        // Unknown children are skipped in lenient mode
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(bibl)
}

/// Parse an `<imprint>` element from within another element.
///
/// Imprint can contain mixed content (text and child elements like publisher, pubPlace, date, etc.)
pub(crate) fn parse_imprint_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Imprint> {
    use tusk_model::elements::ImprintChild;

    let mut imprint = Imprint::default();

    // Extract attributes
    imprint.common.extract_attributes(&mut attrs)?;
    imprint.bibl.extract_attributes(&mut attrs)?;
    imprint.facsimile.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("imprint")? {
            match content {
                MixedContent::Text(text) => {
                    imprint.children.push(ImprintChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "publisher" => {
                            let publisher =
                                parse_publisher_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Publisher(Box::new(publisher)));
                        }
                        "pubPlace" => {
                            let pub_place =
                                parse_pub_place_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::PubPlace(Box::new(pub_place)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Date(Box::new(date)));
                        }
                        "distributor" => {
                            let distributor =
                                parse_distributor_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Distributor(Box::new(distributor)));
                        }
                        "respStmt" => {
                            let resp_stmt =
                                parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::RespStmt(Box::new(resp_stmt)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Identifier(Box::new(identifier)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Title(Box::new(title)));
                        }
                        "availability" => {
                            let availability =
                                parse_availability_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Availability(Box::new(availability)));
                        }
                        "extent" => {
                            let extent =
                                super::parse_extent_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Extent(Box::new(extent)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Address(Box::new(address)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Bibl(Box::new(bibl)));
                        }
                        "biblStruct" => {
                            let bibl_struct =
                                parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::BiblStruct(Box::new(bibl_struct)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Name(Box::new(name_elem)));
                        }
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::GeogName(Box::new(geog_name)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Annot(Box::new(annot)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Lb(Box::new(lb)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Ptr(Box::new(ptr)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Ref(Box::new(ref_elem)));
                        }
                        "rend" => {
                            let rend =
                                super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Rend(Box::new(rend)));
                        }
                        // Skip unknown children in lenient mode
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(imprint)
}

/// Parse a `<locus>` element from within another element.
pub(crate) fn parse_locus_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Locus> {
    let mut locus = Locus::default();

    // Extract attributes
    locus.common.extract_attributes(&mut attrs)?;
    locus.bibl.extract_attributes(&mut attrs)?;
    locus.foliation_scheme.extract_attributes(&mut attrs)?;
    locus.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // locus can contain text and some child elements (for now, just text)
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("locus")? {
            if !text.trim().is_empty() {
                locus
                    .children
                    .push(tusk_model::elements::LocusChild::Text(text));
            }
        }
    }

    Ok(locus)
}

/// Parse a `<locusGrp>` element from within another element.
pub(crate) fn parse_locus_grp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LocusGrp> {
    let mut locus_grp = LocusGrp::default();

    // Extract attributes
    locus_grp.common.extract_attributes(&mut attrs)?;
    locus_grp.bibl.extract_attributes(&mut attrs)?;
    locus_grp.foliation_scheme.extract_attributes(&mut attrs)?;
    locus_grp.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // locusGrp can contain: locus+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("locusGrp")?
        {
            match name.as_str() {
                "locus" => {
                    let locus = parse_locus_from_event(reader, child_attrs, child_empty)?;
                    locus_grp
                        .children
                        .push(tusk_model::elements::LocusGrpChild::Locus(Box::new(locus)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(locus_grp)
}

/// Parse a `<biblStruct>` element from within another element.
pub(crate) fn parse_bibl_struct_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<BiblStruct> {
    let mut bibl_struct = BiblStruct::default();

    // Extract attributes
    bibl_struct.common.extract_attributes(&mut attrs)?;
    bibl_struct.bibl.extract_attributes(&mut attrs)?;
    bibl_struct.data_pointing.extract_attributes(&mut attrs)?;
    bibl_struct.lang.extract_attributes(&mut attrs)?;
    bibl_struct.pointing.extract_attributes(&mut attrs)?;
    bibl_struct.record_type.extract_attributes(&mut attrs)?;
    bibl_struct.target_eval.extract_attributes(&mut attrs)?;

    // For now, skip all children (biblStruct can contain analytic, monogr, series, etc.)
    // In lenient mode, we just skip unknown children
    if !is_empty {
        while let Some((name, _child_attrs, child_empty)) =
            reader.read_next_child_start("biblStruct")?
        {
            // Skip all children for now - biblStruct children are complex
            if !child_empty {
                reader.skip_to_end(&name)?;
            }
        }
    }

    Ok(bibl_struct)
}

/// Parse a `<biblScope>` element from within another element.
///
/// BiblScope defines the scope of a bibliographic reference (page numbers, subdivisions, etc.)
/// It can contain mixed content (text and child elements).
pub(crate) fn parse_bibl_scope_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<BiblScope> {
    use tusk_model::elements::BiblScopeChild;

    let mut bibl_scope = BiblScope::default();

    // Extract attributes
    bibl_scope.common.extract_attributes(&mut attrs)?;
    bibl_scope.bibl.extract_attributes(&mut attrs)?;
    bibl_scope.facsimile.extract_attributes(&mut attrs)?;
    bibl_scope.extent.extract_attributes(&mut attrs)?;
    bibl_scope.lang.extract_attributes(&mut attrs)?;

    // Extract element-specific attributes
    extract_attr!(attrs, "from", string bibl_scope.from);
    extract_attr!(attrs, "to", string bibl_scope.to);

    // BiblScope has mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("biblScope")? {
            match content {
                MixedContent::Text(text) => {
                    bibl_scope.children.push(BiblScopeChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Date(Box::new(date)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Name(Box::new(name_elem)));
                        }
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::GeogName(Box::new(geog_name)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Address(Box::new(address)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Bibl(Box::new(bibl)));
                        }
                        "biblStruct" => {
                            let bibl_struct =
                                parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::BiblStruct(Box::new(bibl_struct)));
                        }
                        "locus" => {
                            let locus = parse_locus_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Locus(Box::new(locus)));
                        }
                        "locusGrp" => {
                            let locus_grp =
                                parse_locus_grp_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::LocusGrp(Box::new(locus_grp)));
                        }
                        "rend" => {
                            let rend =
                                super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope.children.push(BiblScopeChild::Lb(Box::new(lb)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Annot(Box::new(annot)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope.children.push(BiblScopeChild::Ptr(Box::new(ptr)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Ref(Box::new(ref_elem)));
                        }
                        "symbol" => {
                            let symbol = super::control::parse_symbol_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Symbol(Box::new(symbol)));
                        }
                        // Skip unknown children in lenient mode
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(bibl_scope)
}

/// Parse an `<encodingDesc>` element from within another element.
pub(crate) fn parse_encoding_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EncodingDesc> {
    let mut encoding_desc = EncodingDesc::default();

    // Extract attributes
    encoding_desc.common.extract_attributes(&mut attrs)?;
    encoding_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // encodingDesc can contain: head*, appInfo?, editorialDecl?, projectDesc?,
    // samplingDecl?, domainsDecl*, tagsDecl?, classDecls?
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("encodingDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::Head(Box::new(head)));
                }
                "appInfo" => {
                    let app_info = parse_app_info_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::AppInfo(Box::new(app_info)));
                }
                "editorialDecl" => {
                    let editorial_decl =
                        parse_editorial_decl_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::EditorialDecl(Box::new(editorial_decl)));
                }
                "projectDesc" => {
                    let project_desc =
                        parse_project_desc_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::ProjectDesc(Box::new(project_desc)));
                }
                "samplingDecl" => {
                    let sampling_decl =
                        parse_sampling_decl_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::SamplingDecl(Box::new(sampling_decl)));
                }
                "classDecls" => {
                    let class_decls =
                        parse_class_decls_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::ClassDecls(Box::new(class_decls)));
                }
                // domainsDecl, tagsDecl are more complex - skip for now
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(encoding_desc)
}

/// Parse an `<appInfo>` element from within another element.
pub(crate) fn parse_app_info_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AppInfo> {
    let mut app_info = AppInfo::default();

    // Extract attributes
    app_info.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // appInfo can contain: head*, application*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("appInfo")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    app_info.children.push(AppInfoChild::Head(Box::new(head)));
                }
                "application" => {
                    let application =
                        parse_application_from_event(reader, child_attrs, child_empty)?;
                    app_info
                        .children
                        .push(AppInfoChild::Application(Box::new(application)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(app_info)
}

/// Parse an `<application>` element from within another element.
pub(crate) fn parse_application_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Application> {
    let mut application = Application::default();

    // Extract attributes
    application.common.extract_attributes(&mut attrs)?;
    application.datable.extract_attributes(&mut attrs)?;

    // Element-local attribute: @version
    extract_attr!(attrs, "version", string application.version);

    // Read children if not an empty element
    // application can contain: name+, then (ptr* | ref* | p*)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("application")?
        {
            match name.as_str() {
                "name" => {
                    let name_elem = parse_name_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Name(Box::new(name_elem)));
                }
                "ptr" => {
                    let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Ptr(Box::new(ptr)));
                }
                "ref" => {
                    let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Ref(Box::new(ref_elem)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    application.children.push(ApplicationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(application)
}

/// Parse a `<name>` element from within another element.
///
/// Name can contain mixed content (text and many child elements).
pub(crate) fn parse_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Name> {
    let mut name_elem = Name::default();

    // Extract attributes
    name_elem.basic.extract_attributes(&mut attrs)?;
    name_elem.bibl.extract_attributes(&mut attrs)?;
    name_elem.classed.extract_attributes(&mut attrs)?;
    name_elem.edit.extract_attributes(&mut attrs)?;
    name_elem.facsimile.extract_attributes(&mut attrs)?;
    name_elem.labelled.extract_attributes(&mut attrs)?;
    name_elem.lang.extract_attributes(&mut attrs)?;
    name_elem.linking.extract_attributes(&mut attrs)?;
    name_elem.name.extract_attributes(&mut attrs)?;
    name_elem.n_number_like.extract_attributes(&mut attrs)?;
    name_elem.responsibility.extract_attributes(&mut attrs)?;

    // Name has mixed content - text and various child elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("name")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        name_elem.children.push(NameChild::Text(text));
                    }
                }
                MixedContent::Element(elem_name, child_attrs, child_empty) => {
                    match elem_name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            name_elem.children.push(NameChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::PersName(Box::new(pers)));
                        }
                        "corpName" => {
                            let corp =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::CorpName(Box::new(corp)));
                        }
                        "name" => {
                            let nested = parse_name_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Name(Box::new(nested)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Ptr(Box::new(ptr)));
                        }
                        "geogName" => {
                            let geog =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::GeogName(Box::new(geog)));
                        }
                        "identifier" => {
                            let ident =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            name_elem
                                .children
                                .push(NameChild::Identifier(Box::new(ident)));
                        }
                        _ => {
                            // Skip unknown child elements
                            if !child_empty {
                                reader.skip_to_end(&elem_name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(name_elem)
}

/// Parse a `<ptr>` element from within another element.
pub(crate) fn parse_ptr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Ptr> {
    let mut ptr = Ptr::default();

    // Extract attributes
    ptr.common.extract_attributes(&mut attrs)?;
    ptr.internet_media.extract_attributes(&mut attrs)?;
    ptr.metadata_pointing.extract_attributes(&mut attrs)?;
    ptr.pointing.extract_attributes(&mut attrs)?;
    ptr.target_eval.extract_attributes(&mut attrs)?;

    // ptr has no children, but we still need to consume the end tag if not empty
    if !is_empty {
        reader.skip_to_end("ptr")?;
    }

    Ok(ptr)
}

/// Parse a `<ref>` (reference) element from within another element.
///
/// Ref can contain mixed content (text and many child elements).
pub(crate) fn parse_ref_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Ref> {
    let mut ref_elem = Ref::default();

    // Extract attributes
    ref_elem.common.extract_attributes(&mut attrs)?;
    ref_elem.internet_media.extract_attributes(&mut attrs)?;
    ref_elem.lang.extract_attributes(&mut attrs)?;
    ref_elem.metadata_pointing.extract_attributes(&mut attrs)?;
    ref_elem.pointing.extract_attributes(&mut attrs)?;
    ref_elem.target_eval.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("ref")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content including whitespace
                    if !text.trim().is_empty() {
                        ref_elem.children.push(RefChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            ref_elem
                                .children
                                .push(RefChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            ref_elem
                                .children
                                .push(RefChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            ref_elem
                                .children
                                .push(RefChild::GeogName(Box::new(geog_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            ref_elem
                                .children
                                .push(RefChild::Identifier(Box::new(identifier)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Title(Box::new(title)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Annot(Box::new(annot)));
                        }
                        "rend" => {
                            let rend =
                                super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Rend(Box::new(rend)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Ptr(Box::new(ptr)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Lb(Box::new(lb)));
                        }
                        // Other child elements not yet implemented - skip
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(ref_elem)
}

/// Parse a `<p>` (paragraph) element from within another element.
///
/// P can contain mixed content (text and many child elements like ref, rend, etc.)
pub(crate) fn parse_p_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<P> {
    let mut p = P::default();

    // Extract attributes
    p.common.extract_attributes(&mut attrs)?;
    p.facsimile.extract_attributes(&mut attrs)?;
    p.lang.extract_attributes(&mut attrs)?;
    p.metadata_pointing.extract_attributes(&mut attrs)?;
    p.xy.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("p")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve all text content
                    if !text.trim().is_empty() {
                        p.children.push(PChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend =
                                super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Rend(Box::new(rend)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::GeogName(Box::new(geog_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Title(Box::new(title)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Annot(Box::new(annot)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Lb(Box::new(lb)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Bibl(Box::new(bibl)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Identifier(Box::new(identifier)));
                        }
                        "list" => {
                            let list =
                                super::parse_list_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::List(Box::new(list)));
                        }
                        // Other child elements not yet implemented - skip
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(p)
}

/// Parse a `<correction>` element from within another element.
pub(crate) fn parse_correction_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Correction> {
    let mut correction = Correction::default();

    // Extract attributes
    correction.common.extract_attributes(&mut attrs)?;
    correction.bibl.extract_attributes(&mut attrs)?;
    correction.data_pointing.extract_attributes(&mut attrs)?;
    correction.lang.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "method", correction.regular_method.method);

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // correction can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("correction")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    correction
                        .children
                        .push(CorrectionChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    correction.children.push(CorrectionChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(correction)
}

/// Parse an `<interpretation>` element from within another element.
pub(crate) fn parse_interpretation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Interpretation> {
    let mut interpretation = Interpretation::default();

    // Extract attributes
    interpretation.common.extract_attributes(&mut attrs)?;
    interpretation.bibl.extract_attributes(&mut attrs)?;
    interpretation
        .data_pointing
        .extract_attributes(&mut attrs)?;
    interpretation.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // interpretation can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("interpretation")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    interpretation
                        .children
                        .push(InterpretationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    interpretation
                        .children
                        .push(InterpretationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(interpretation)
}

/// Parse a `<normalization>` element from within another element.
pub(crate) fn parse_normalization_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Normalization> {
    let mut normalization = Normalization::default();

    // Extract attributes
    normalization.common.extract_attributes(&mut attrs)?;
    normalization.bibl.extract_attributes(&mut attrs)?;
    normalization.data_pointing.extract_attributes(&mut attrs)?;
    normalization.lang.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "method", normalization.regular_method.method);

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // normalization can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("normalization")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    normalization
                        .children
                        .push(NormalizationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    normalization
                        .children
                        .push(NormalizationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(normalization)
}

/// Parse a `<segmentation>` element from within another element.
pub(crate) fn parse_segmentation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Segmentation> {
    let mut segmentation = Segmentation::default();

    // Extract attributes
    segmentation.common.extract_attributes(&mut attrs)?;
    segmentation.bibl.extract_attributes(&mut attrs)?;
    segmentation.data_pointing.extract_attributes(&mut attrs)?;
    segmentation.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // segmentation can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("segmentation")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    segmentation
                        .children
                        .push(SegmentationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    segmentation
                        .children
                        .push(SegmentationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(segmentation)
}

/// Parse a `<stdVals>` element from within another element.
pub(crate) fn parse_std_vals_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StdVals> {
    let mut std_vals = StdVals::default();

    // Extract attributes
    std_vals.common.extract_attributes(&mut attrs)?;
    std_vals.bibl.extract_attributes(&mut attrs)?;
    std_vals.data_pointing.extract_attributes(&mut attrs)?;
    std_vals.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // stdVals can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("stdVals")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    std_vals.children.push(StdValsChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    std_vals.children.push(StdValsChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(std_vals)
}

/// Parse an `<editorialDecl>` element from within another element.
pub(crate) fn parse_editorial_decl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EditorialDecl> {
    let mut editorial_decl = EditorialDecl::default();

    // Extract attributes
    editorial_decl.common.extract_attributes(&mut attrs)?;
    editorial_decl.bibl.extract_attributes(&mut attrs)?;
    editorial_decl
        .data_pointing
        .extract_attributes(&mut attrs)?;
    editorial_decl.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // editorialDecl can contain: head*, (correction | interpretation | normalization |
    // p | segmentation | stdVals)*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("editorialDecl")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::P(Box::new(p)));
                }
                "correction" => {
                    let correction = parse_correction_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Correction(Box::new(correction)));
                }
                "interpretation" => {
                    let interpretation =
                        parse_interpretation_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Interpretation(Box::new(interpretation)));
                }
                "normalization" => {
                    let normalization =
                        parse_normalization_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Normalization(Box::new(normalization)));
                }
                "segmentation" => {
                    let segmentation =
                        parse_segmentation_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Segmentation(Box::new(segmentation)));
                }
                "stdVals" => {
                    let std_vals = parse_std_vals_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::StdVals(Box::new(std_vals)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(editorial_decl)
}

/// Parse a `<projectDesc>` element from within another element.
pub(crate) fn parse_project_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ProjectDesc> {
    let mut project_desc = ProjectDesc::default();

    // Extract attributes
    project_desc.common.extract_attributes(&mut attrs)?;
    project_desc.bibl.extract_attributes(&mut attrs)?;
    project_desc.data_pointing.extract_attributes(&mut attrs)?;
    project_desc.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // projectDesc can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("projectDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    project_desc
                        .children
                        .push(ProjectDescChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    project_desc.children.push(ProjectDescChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(project_desc)
}

/// Parse a `<samplingDecl>` element from within another element.
pub(crate) fn parse_sampling_decl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SamplingDecl> {
    let mut sampling_decl = SamplingDecl::default();

    // Extract attributes
    sampling_decl.common.extract_attributes(&mut attrs)?;
    sampling_decl.bibl.extract_attributes(&mut attrs)?;
    sampling_decl.data_pointing.extract_attributes(&mut attrs)?;
    sampling_decl.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // samplingDecl can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("samplingDecl")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    sampling_decl
                        .children
                        .push(SamplingDeclChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    sampling_decl
                        .children
                        .push(SamplingDeclChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(sampling_decl)
}

/// Parse a `<classDecls>` element from within another element.
pub(crate) fn parse_class_decls_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ClassDecls> {
    let mut class_decls = ClassDecls::default();

    // Extract attributes
    class_decls.common.extract_attributes(&mut attrs)?;
    class_decls.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // classDecls can contain: head*, taxonomy+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("classDecls")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    class_decls
                        .children
                        .push(ClassDeclsChild::Head(Box::new(head)));
                }
                "taxonomy" => {
                    let taxonomy = parse_taxonomy_from_event(reader, child_attrs, child_empty)?;
                    class_decls
                        .children
                        .push(ClassDeclsChild::Taxonomy(Box::new(taxonomy)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(class_decls)
}

/// Parse a `<taxonomy>` element from within another element.
pub(crate) fn parse_taxonomy_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Taxonomy> {
    let mut taxonomy = Taxonomy::default();

    // Extract attributes
    taxonomy.common.extract_attributes(&mut attrs)?;
    taxonomy.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // taxonomy can contain: category*, bibl?, biblStruct?, taxonomy*, head*, desc*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("taxonomy")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    taxonomy.children.push(TaxonomyChild::Head(Box::new(head)));
                }
                "bibl" => {
                    let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    taxonomy.children.push(TaxonomyChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    taxonomy
                        .children
                        .push(TaxonomyChild::BiblStruct(Box::new(bibl_struct)));
                }
                "category" => {
                    let category = parse_category_from_event(reader, child_attrs, child_empty)?;
                    taxonomy
                        .children
                        .push(TaxonomyChild::Category(Box::new(category)));
                }
                "taxonomy" => {
                    // Recursive taxonomy
                    let nested_taxonomy =
                        parse_taxonomy_from_event(reader, child_attrs, child_empty)?;
                    taxonomy
                        .children
                        .push(TaxonomyChild::Taxonomy(Box::new(nested_taxonomy)));
                }
                "desc" => {
                    let desc = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    taxonomy.children.push(TaxonomyChild::Desc(Box::new(desc)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(taxonomy)
}

/// Parse a `<category>` element from within another element.
pub(crate) fn parse_category_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Category> {
    let mut category = Category::default();

    // Extract attributes
    category.common.extract_attributes(&mut attrs)?;
    category.authorized.extract_attributes(&mut attrs)?;
    category.bibl.extract_attributes(&mut attrs)?;
    category.data_pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // category can contain: altId*, desc*, category*, label*, catRel*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("category")?
        {
            match name.as_str() {
                "altId" => {
                    let alt_id = parse_alt_id_from_event(reader, child_attrs, child_empty)?;
                    category
                        .children
                        .push(CategoryChild::AltId(Box::new(alt_id)));
                }
                "desc" => {
                    let desc = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    category.children.push(CategoryChild::Desc(Box::new(desc)));
                }
                "category" => {
                    // Recursive category
                    let nested_category =
                        parse_category_from_event(reader, child_attrs, child_empty)?;
                    category
                        .children
                        .push(CategoryChild::Category(Box::new(nested_category)));
                }
                "label" => {
                    let label = super::parse_label_from_event(reader, child_attrs, child_empty)?;
                    category
                        .children
                        .push(CategoryChild::Label(Box::new(label)));
                }
                "catRel" => {
                    let cat_rel = parse_cat_rel_from_event(reader, child_attrs, child_empty)?;
                    category
                        .children
                        .push(CategoryChild::CatRel(Box::new(cat_rel)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(category)
}

/// Parse an `<altId>` element from within another element.
pub(crate) fn parse_alt_id_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AltId> {
    let mut alt_id = AltId::default();

    // Extract attributes
    alt_id.common.extract_attributes(&mut attrs)?;
    alt_id.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // altId can contain mixed content: text, lb, rend, stack
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("altId")? {
            match content {
                MixedContent::Text(text) => {
                    alt_id.children.push(AltIdChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        alt_id.children.push(AltIdChild::Lb(Box::new(lb)));
                    }
                    "rend" => {
                        let rend = super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        alt_id.children.push(AltIdChild::Rend(Box::new(rend)));
                    }
                    // stack is more complex - skip for now
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(alt_id)
}

/// Parse a `<catRel>` element from within another element.
pub(crate) fn parse_cat_rel_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CatRel> {
    let mut cat_rel = CatRel::default();

    // Extract attributes
    cat_rel.authorized.extract_attributes(&mut attrs)?;
    cat_rel.basic.extract_attributes(&mut attrs)?;
    cat_rel.bibl.extract_attributes(&mut attrs)?;
    cat_rel.labelled.extract_attributes(&mut attrs)?;
    cat_rel.linking.extract_attributes(&mut attrs)?;
    cat_rel.n_number_like.extract_attributes(&mut attrs)?;
    cat_rel.responsibility.extract_attributes(&mut attrs)?;

    // Extract @type attribute
    if let Some(type_val) = attrs.remove("type") {
        cat_rel.r#type = Some(type_val);
    }

    // Read children if not an empty element
    // catRel can contain: desc*, label*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("catRel")? {
            match name.as_str() {
                "desc" => {
                    let desc = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    cat_rel.children.push(CatRelChild::Desc(Box::new(desc)));
                }
                "label" => {
                    let label = super::parse_label_from_event(reader, child_attrs, child_empty)?;
                    cat_rel.children.push(CatRelChild::Label(Box::new(label)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(cat_rel)
}

/// Parse a `<desc>` element from within another element.
pub(crate) fn parse_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Desc> {
    let mut desc = tusk_model::elements::Desc::default();

    // Extract attributes
    desc.common.extract_attributes(&mut attrs)?;
    desc.facsimile.extract_attributes(&mut attrs)?;
    desc.lang.extract_attributes(&mut attrs)?;
    desc.source.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // desc can contain mixed content with many possible child elements
    // For now, just capture text content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("desc")? {
            match content {
                MixedContent::Text(text) => {
                    desc.children
                        .push(tusk_model::elements::DescChild::Text(text));
                }
                MixedContent::Element(name, _child_attrs, child_empty) => {
                    // desc has many possible children - skip unknown for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(desc)
}

/// Parse an `<unpub>` element from within another element.
pub(crate) fn parse_unpub_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Unpub> {
    let mut unpub = Unpub::default();

    // Extract attributes
    unpub.common.extract_attributes(&mut attrs)?;
    unpub.bibl.extract_attributes(&mut attrs)?;
    unpub.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("unpub")? {
            if !text.trim().is_empty() {
                unpub
                    .children
                    .push(tusk_model::elements::UnpubChild::Text(text));
            }
        }
    }

    Ok(unpub)
}

/// Parse a `<publisher>` element from within another element.
pub(crate) fn parse_publisher_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Publisher> {
    let mut publisher = Publisher::default();

    // Extract attributes
    publisher.common.extract_attributes(&mut attrs)?;
    publisher.bibl.extract_attributes(&mut attrs)?;
    publisher.facsimile.extract_attributes(&mut attrs)?;
    publisher.lang.extract_attributes(&mut attrs)?;

    // Publisher can contain text and various child elements like corpName, persName, address, etc.
    // Use mixed content reading to handle both text and elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("publisher")? {
            match content {
                MixedContent::Text(text) => {
                    publisher.children.push(PublisherChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "corpName" => {
                            let corp =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::CorpName(Box::new(corp)));
                        }
                        "persName" => {
                            let pers =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::PersName(Box::new(pers)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let addr = parse_address_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Address(Box::new(addr)));
                        }
                        "identifier" => {
                            let ident =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Identifier(Box::new(ident)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(publisher)
}

/// Parse a `<pubPlace>` element from within another element.
pub(crate) fn parse_pub_place_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PubPlace> {
    let mut pub_place = PubPlace::default();

    // Extract attributes
    pub_place.common.extract_attributes(&mut attrs)?;
    pub_place.bibl.extract_attributes(&mut attrs)?;
    pub_place.facsimile.extract_attributes(&mut attrs)?;
    pub_place.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // pubPlace can contain text and various child elements
    // For now, we collect text content as PubPlaceChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("pubPlace")? {
            if !text.trim().is_empty() {
                pub_place
                    .children
                    .push(tusk_model::elements::PubPlaceChild::Text(text));
            }
        }
    }

    Ok(pub_place)
}

/// Parse a `<date>` element from within another element.
pub(crate) fn parse_date_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Date> {
    let mut date = Date::default();

    // Extract attributes
    date.common.extract_attributes(&mut attrs)?;
    date.bibl.extract_attributes(&mut attrs)?;
    date.calendared.extract_attributes(&mut attrs)?;
    date.datable.extract_attributes(&mut attrs)?;
    date.edit.extract_attributes(&mut attrs)?;
    date.facsimile.extract_attributes(&mut attrs)?;
    date.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // date can contain text and various child elements
    // For now, we collect text content as DateChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("date")? {
            if !text.trim().is_empty() {
                date.children
                    .push(tusk_model::elements::DateChild::Text(text));
            }
        }
    }

    Ok(date)
}

/// Parse an `<identifier>` element from within another element.
///
/// Identifier is a mixed content element that can contain text and various child elements
/// such as ref, ptr, rend, name, persName, corpName, date, lb, etc.
pub(crate) fn parse_identifier_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Identifier> {
    use tusk_model::elements::IdentifierChild;

    let mut identifier = Identifier::default();

    // Extract attributes
    identifier.common.extract_attributes(&mut attrs)?;
    identifier.authorized.extract_attributes(&mut attrs)?;
    identifier.bibl.extract_attributes(&mut attrs)?;
    identifier.facsimile.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("identifier")? {
            match content {
                MixedContent::Text(text) => {
                    identifier.children.push(IdentifierChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            identifier
                                .children
                                .push(IdentifierChild::Rend(Box::new(rend)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::CorpName(Box::new(corp_name)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Date(Box::new(date)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            identifier.children.push(IdentifierChild::Lb(Box::new(lb)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Title(Box::new(title)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Bibl(Box::new(bibl)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Annot(Box::new(annot)));
                        }
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::GeogName(Box::new(geog_name)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Address(Box::new(address)));
                        }
                        "identifier" => {
                            // Recursive: identifier can contain identifier
                            let nested_identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Identifier(Box::new(nested_identifier)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(identifier)
}

/// Parse an `<availability>` element from within another element.
///
/// Availability can contain: identifier, distributor, head, useRestrict, date,
/// accessRestrict, price, address, sysReq (and text content).
pub(crate) fn parse_availability_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Availability> {
    let mut availability = Availability::default();

    // Extract attributes
    availability.common.extract_attributes(&mut attrs)?;
    availability.bibl.extract_attributes(&mut attrs)?;
    availability.data_pointing.extract_attributes(&mut attrs)?;

    // Parse children using mixed content reader since availability can contain text
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("availability")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        availability.children.push(AvailabilityChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "useRestrict" => {
                            let use_restrict =
                                parse_use_restrict_from_event(reader, child_attrs, child_empty)?;
                            availability
                                .children
                                .push(AvailabilityChild::UseRestrict(Box::new(use_restrict)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            availability
                                .children
                                .push(AvailabilityChild::Identifier(Box::new(identifier)));
                        }
                        "distributor" => {
                            let distributor =
                                parse_distributor_from_event(reader, child_attrs, child_empty)?;
                            availability
                                .children
                                .push(AvailabilityChild::Distributor(Box::new(distributor)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            availability
                                .children
                                .push(AvailabilityChild::Date(Box::new(date)));
                        }
                        "head" => {
                            let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                            availability
                                .children
                                .push(AvailabilityChild::Head(Box::new(head)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            availability
                                .children
                                .push(AvailabilityChild::Address(Box::new(address)));
                        }
                        // accessRestrict, price, sysReq not yet implemented - skip for now
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(availability)
}

/// Parse a `<useRestrict>` element from within another element.
///
/// UseRestrict can contain mixed content (text and many child elements).
pub(crate) fn parse_use_restrict_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<UseRestrict> {
    let mut use_restrict = UseRestrict::default();

    // Extract attributes
    use_restrict.common.extract_attributes(&mut attrs)?;
    use_restrict.authorized.extract_attributes(&mut attrs)?;
    use_restrict.bibl.extract_attributes(&mut attrs)?;
    use_restrict.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("useRestrict")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content including whitespace between elements
                    if !text.trim().is_empty() {
                        use_restrict.children.push(UseRestrictChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "p" => {
                            let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                            use_restrict.children.push(UseRestrictChild::P(Box::new(p)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Ptr(Box::new(ptr)));
                        }
                        "head" => {
                            let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Head(Box::new(head)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::GeogName(Box::new(geog_name)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Identifier(Box::new(identifier)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Title(Box::new(title)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Annot(Box::new(annot)));
                        }
                        "rend" => {
                            let rend =
                                super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Rend(Box::new(rend)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Address(Box::new(address)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Lb(Box::new(lb)));
                        }
                        // Other child elements not yet implemented - skip
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(use_restrict)
}

/// Parse a `<distributor>` element from within another element.
pub(crate) fn parse_distributor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Distributor> {
    let mut distributor = Distributor::default();

    // Extract attributes
    distributor.common.extract_attributes(&mut attrs)?;
    distributor.bibl.extract_attributes(&mut attrs)?;
    distributor.facsimile.extract_attributes(&mut attrs)?;
    distributor.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // distributor can contain text and various child elements
    // For now, we collect text content as DistributorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("distributor")? {
            if !text.trim().is_empty() {
                distributor
                    .children
                    .push(tusk_model::elements::DistributorChild::Text(text));
            }
        }
    }

    Ok(distributor)
}

/// Parse a `<title>` element from within another element.
///
/// Title can contain mixed content (text and child elements like titlePart, rend, etc.)
pub(crate) fn parse_title_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Title> {
    let mut title = Title::default();

    // Extract attributes into each attribute class
    title.authorized.extract_attributes(&mut attrs)?;
    title.basic.extract_attributes(&mut attrs)?;
    title.bibl.extract_attributes(&mut attrs)?;
    title.classed.extract_attributes(&mut attrs)?;
    title.facsimile.extract_attributes(&mut attrs)?;
    title.filing.extract_attributes(&mut attrs)?;
    title.labelled.extract_attributes(&mut attrs)?;
    title.lang.extract_attributes(&mut attrs)?;
    title.linking.extract_attributes(&mut attrs)?;
    title.n_number_like.extract_attributes(&mut attrs)?;
    title.responsibility.extract_attributes(&mut attrs)?;

    // Element-local attributes
    title.level = attrs.remove("level");
    title.r#type = attrs.remove("type");

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Title can contain mixed content (text + elements like titlePart, rend, corpName, etc.)
    // Use mixed content reading to handle both text and elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("title")? {
            match content {
                MixedContent::Text(text) => {
                    title.children.push(TitleChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "titlePart" => {
                            let title_part =
                                parse_title_part_from_event(reader, child_attrs, child_empty)?;
                            title
                                .children
                                .push(TitleChild::TitlePart(Box::new(title_part)));
                        }
                        "corpName" => {
                            let corp =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::CorpName(Box::new(corp)));
                        }
                        "persName" => {
                            let pers =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::PersName(Box::new(pers)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            title
                                .children
                                .push(TitleChild::Identifier(Box::new(identifier)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Ptr(Box::new(ptr)));
                        }
                        "address" => {
                            let addr = parse_address_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Address(Box::new(addr)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Bibl(Box::new(bibl)));
                        }
                        "biblStruct" => {
                            let bibl_struct =
                                parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                            title
                                .children
                                .push(TitleChild::BiblStruct(Box::new(bibl_struct)));
                        }
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title.children.push(TitleChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Lb(Box::new(lb)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Ref(Box::new(ref_elem)));
                        }
                        "num" => {
                            let num = super::misc::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title.children.push(TitleChild::Num(Box::new(num)));
                        }
                        // Skip unknown elements
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(title)
}

/// Parse a `<titlePart>` element from within another element.
///
/// TitlePart can contain mixed content (text and child elements).
pub(crate) fn parse_title_part_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TitlePart> {
    let mut title_part = TitlePart::default();

    // Extract attributes into each attribute class
    title_part.authorized.extract_attributes(&mut attrs)?;
    title_part.basic.extract_attributes(&mut attrs)?;
    title_part.bibl.extract_attributes(&mut attrs)?;
    title_part.classed.extract_attributes(&mut attrs)?;
    title_part.facsimile.extract_attributes(&mut attrs)?;
    title_part.filing.extract_attributes(&mut attrs)?;
    title_part.labelled.extract_attributes(&mut attrs)?;
    title_part.lang.extract_attributes(&mut attrs)?;
    title_part.linking.extract_attributes(&mut attrs)?;
    title_part.n_integer.extract_attributes(&mut attrs)?;
    title_part.responsibility.extract_attributes(&mut attrs)?;

    // Element-local attribute: @type
    extract_attr!(attrs, "type", string title_part.r#type);

    // TitlePart can contain mixed content (text + elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("titlePart")? {
            match content {
                MixedContent::Text(text) => {
                    title_part.children.push(TitlePartChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "title" => {
                            let nested_title =
                                parse_title_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Title(Box::new(nested_title)));
                        }
                        "corpName" => {
                            let corp =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::CorpName(Box::new(corp)));
                        }
                        "persName" => {
                            let pers =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::PersName(Box::new(pers)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Identifier(Box::new(identifier)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            title_part.children.push(TitlePartChild::Ptr(Box::new(ptr)));
                        }
                        "address" => {
                            let addr = parse_address_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Address(Box::new(addr)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Bibl(Box::new(bibl)));
                        }
                        "biblStruct" => {
                            let bibl_struct =
                                parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::BiblStruct(Box::new(bibl_struct)));
                        }
                        // Skip unknown elements
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(title_part)
}

/// Parse a `<head>` element from within another element.
pub(crate) fn parse_head_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Head> {
    let mut head = Head::default();

    // Extract attributes into each attribute class
    head.common.extract_attributes(&mut attrs)?;
    head.facsimile.extract_attributes(&mut attrs)?;
    head.lang.extract_attributes(&mut attrs)?;
    head.xy.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // head can contain text and various child elements
    // For now, we collect text content as HeadChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("head")? {
            if !text.trim().is_empty() {
                head.children.push(HeadChild::Text(text));
            }
        }
    }

    Ok(head)
}

/// Parse a `<respStmt>` element from within another element.
pub(crate) fn parse_resp_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RespStmt> {
    let mut resp_stmt = RespStmt::default();

    // Extract attributes into each attribute class
    resp_stmt.common.extract_attributes(&mut attrs)?;
    resp_stmt.bibl.extract_attributes(&mut attrs)?;
    resp_stmt.facsimile.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // respStmt can contain: name, resp, corpName, annot, head, persName
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("respStmt")?
        {
            match name.as_str() {
                "name" => {
                    let name_elem = parse_name_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::Name(Box::new(name_elem)));
                }
                "resp" => {
                    let resp = parse_resp_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt.children.push(RespStmtChild::Resp(Box::new(resp)));
                }
                "corpName" => {
                    let corp = parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::CorpName(Box::new(corp)));
                }
                "annot" => {
                    let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::Annot(Box::new(annot)));
                }
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt.children.push(RespStmtChild::Head(Box::new(head)));
                }
                "persName" => {
                    let pers = parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::PersName(Box::new(pers)));
                }
                _ => {
                    // Skip unknown children in lenient mode
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(resp_stmt)
}

/// Parse a `<resp>` element from within another element.
pub(crate) fn parse_resp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Resp> {
    let mut resp = Resp::default();

    // Extract attributes into each attribute class
    resp.common.extract_attributes(&mut attrs)?;
    resp.authorized.extract_attributes(&mut attrs)?;
    resp.bibl.extract_attributes(&mut attrs)?;
    resp.datable.extract_attributes(&mut attrs)?;
    resp.facsimile.extract_attributes(&mut attrs)?;
    resp.lang.extract_attributes(&mut attrs)?;

    // resp can contain text and many child elements (mixed content)
    // For now, we collect text content as RespChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("resp")? {
            if !text.trim().is_empty() {
                resp.children.push(RespChild::Text(text));
            }
        }
    }

    Ok(resp)
}

/// Parse an `<annot>` element from within another element.
pub(crate) fn parse_annot_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Annot> {
    let mut annot = Annot::default();

    // Extract attributes into each attribute class
    annot.common.extract_attributes(&mut attrs)?;
    annot.audience.extract_attributes(&mut attrs)?;
    annot.bibl.extract_attributes(&mut attrs)?;
    annot.data_pointing.extract_attributes(&mut attrs)?;
    annot.facsimile.extract_attributes(&mut attrs)?;
    annot.lang.extract_attributes(&mut attrs)?;
    annot.plist.extract_attributes(&mut attrs)?;
    annot.source.extract_attributes(&mut attrs)?;
    annot.target_eval.extract_attributes(&mut attrs)?;
    annot.annot_anl.extract_attributes(&mut attrs)?;
    annot.annot_ges.extract_attributes(&mut attrs)?;
    annot.annot_log.extract_attributes(&mut attrs)?;
    annot.annot_vis.extract_attributes(&mut attrs)?;

    // annot can contain text and many child elements (mixed content)
    // For now, we collect text content as AnnotChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("annot")? {
            if !text.trim().is_empty() {
                annot.children.push(AnnotChild::Text(text));
            }
        }
    }

    Ok(annot)
}

/// Parse an `<editor>` element from within another element.
pub(crate) fn parse_editor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Editor> {
    let mut editor = Editor::default();

    // Extract attributes into each attribute class
    editor.common.extract_attributes(&mut attrs)?;
    editor.bibl.extract_attributes(&mut attrs)?;
    editor.evidence.extract_attributes(&mut attrs)?;
    editor.facsimile.extract_attributes(&mut attrs)?;
    editor.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // editor has mixed content: text and child elements (persName, name, corpName)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("editor")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        editor.children.push(EditorChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "persName" => {
                        let pers = parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                        editor.children.push(EditorChild::PersName(Box::new(pers)));
                    }
                    "name" => {
                        let name_elem = parse_name_from_event(reader, child_attrs, child_empty)?;
                        editor.children.push(EditorChild::Name(Box::new(name_elem)));
                    }
                    "corpName" => {
                        let corp = parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                        editor.children.push(EditorChild::CorpName(Box::new(corp)));
                    }
                    _ => {
                        // Skip unknown children in lenient mode
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(editor)
}

/// Parse a `<creator>` element from within another element.
pub(crate) fn parse_creator_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Creator> {
    let mut creator = Creator::default();

    // Extract attributes into each attribute class
    creator.common.extract_attributes(&mut attrs)?;
    creator.bibl.extract_attributes(&mut attrs)?;
    creator.evidence.extract_attributes(&mut attrs)?;
    creator.facsimile.extract_attributes(&mut attrs)?;
    creator.lang.extract_attributes(&mut attrs)?;
    creator.name.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // creator can contain text and various child elements
    // For now, we collect text content as CreatorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("creator")? {
            if !text.trim().is_empty() {
                creator.children.push(CreatorChild::Text(text));
            }
        }
    }

    Ok(creator)
}

/// Parse a deprecated MEI element (composer, lyricist, arranger, author, librettist) as a Creator.
///
/// MEI 5.1 deprecated composer, lyricist, arranger, author, and librettist in favor of creator.
/// This function parses these deprecated elements and converts them to Creator with
/// the appropriate role attribute set.
pub(crate) fn parse_deprecated_creator_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
    element_name: &str,
    role: tusk_model::generated::data::DataMarcrelatorsBasic,
) -> DeserializeResult<Creator> {
    let mut creator = Creator::default();

    // Extract attributes into each attribute class
    creator.common.extract_attributes(&mut attrs)?;
    creator.bibl.extract_attributes(&mut attrs)?;
    creator.evidence.extract_attributes(&mut attrs)?;
    creator.facsimile.extract_attributes(&mut attrs)?;
    creator.lang.extract_attributes(&mut attrs)?;
    creator.name.extract_attributes(&mut attrs)?;

    // Set the role based on the deprecated element type, but only if role is not already set
    if creator.name.role.is_empty() {
        creator
            .name
            .role
            .push(tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role));
    }

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse mixed content: deprecated elements can contain text and child elements like persName, corpName
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content(element_name)? {
            match content {
                MixedContent::Text(text) => {
                    creator.children.push(CreatorChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "persName" => {
                            let pers =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::PersName(Box::new(pers)));
                        }
                        "corpName" => {
                            let corp =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::CorpName(Box::new(corp)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::Name(Box::new(name_elem)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(creator)
}

/// Parse a `<funder>` element from within another element.
pub(crate) fn parse_funder_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Funder> {
    let mut funder = Funder::default();

    // Extract attributes into each attribute class
    funder.common.extract_attributes(&mut attrs)?;
    funder.bibl.extract_attributes(&mut attrs)?;
    funder.evidence.extract_attributes(&mut attrs)?;
    funder.facsimile.extract_attributes(&mut attrs)?;
    funder.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Funder is a mixed content element - can have text and child elements like corpName, persName, name, etc.
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("funder")? {
            match content {
                MixedContent::Text(text) => {
                    funder.children.push(FunderChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::PersName(Box::new(pers_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::Address(Box::new(address)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Ref(Box::new(ref_elem)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Date(Box::new(date)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            funder.children.push(FunderChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(funder)
}

/// Parse a `<sponsor>` element from within another element.
pub(crate) fn parse_sponsor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Sponsor> {
    let mut sponsor = Sponsor::default();

    // Extract attributes into each attribute class
    sponsor.common.extract_attributes(&mut attrs)?;
    sponsor.bibl.extract_attributes(&mut attrs)?;
    sponsor.evidence.extract_attributes(&mut attrs)?;
    sponsor.facsimile.extract_attributes(&mut attrs)?;
    sponsor.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // sponsor can contain text and various child elements
    // For now, we collect text content as SponsorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("sponsor")? {
            if !text.trim().is_empty() {
                sponsor.children.push(SponsorChild::Text(text));
            }
        }
    }

    Ok(sponsor)
}

/// Parse a `<contributor>` element from within another element.
pub(crate) fn parse_contributor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Contributor> {
    let mut contributor = Contributor::default();

    // Extract attributes into each attribute class
    contributor.common.extract_attributes(&mut attrs)?;
    contributor.bibl.extract_attributes(&mut attrs)?;
    contributor.evidence.extract_attributes(&mut attrs)?;
    contributor.facsimile.extract_attributes(&mut attrs)?;
    contributor.lang.extract_attributes(&mut attrs)?;
    contributor.name.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // contributor can contain text and various child elements
    // For now, we collect text content as ContributorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("contributor")? {
            if !text.trim().is_empty() {
                contributor.children.push(ContributorChild::Text(text));
            }
        }
    }

    Ok(contributor)
}

/// Parse a `<corpName>` element from within another element.
pub(crate) fn parse_corp_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CorpName> {
    let mut corp_name = CorpName::default();

    // Extract attributes
    corp_name.common.extract_attributes(&mut attrs)?;
    corp_name.bibl.extract_attributes(&mut attrs)?;
    corp_name.edit.extract_attributes(&mut attrs)?;
    corp_name.facsimile.extract_attributes(&mut attrs)?;
    corp_name.lang.extract_attributes(&mut attrs)?;
    corp_name.name.extract_attributes(&mut attrs)?;

    // CorpName can contain text and various child elements like address, persName, etc.
    // Use mixed content reading to handle both text and elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("corpName")? {
            match content {
                MixedContent::Text(text) => {
                    corp_name.children.push(CorpNameChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "corpName" => {
                            let nested =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            corp_name
                                .children
                                .push(CorpNameChild::CorpName(Box::new(nested)));
                        }
                        "persName" => {
                            let pers =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            corp_name
                                .children
                                .push(CorpNameChild::PersName(Box::new(pers)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            corp_name
                                .children
                                .push(CorpNameChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let addr = parse_address_from_event(reader, child_attrs, child_empty)?;
                            corp_name
                                .children
                                .push(CorpNameChild::Address(Box::new(addr)));
                        }
                        "geogName" => {
                            let geog =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            corp_name
                                .children
                                .push(CorpNameChild::GeogName(Box::new(geog)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(corp_name)
}

/// Parse a `<persName>` element from within another element.
pub(crate) fn parse_pers_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PersName> {
    let mut pers_name = PersName::default();

    // Extract attributes
    pers_name.common.extract_attributes(&mut attrs)?;
    pers_name.bibl.extract_attributes(&mut attrs)?;
    pers_name.edit.extract_attributes(&mut attrs)?;
    pers_name.facsimile.extract_attributes(&mut attrs)?;
    pers_name.lang.extract_attributes(&mut attrs)?;
    pers_name.name.extract_attributes(&mut attrs)?;

    // PersName can contain text and various child elements
    // Use mixed content reading to handle both text and elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("persName")? {
            match content {
                MixedContent::Text(text) => {
                    pers_name.children.push(PersNameChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "corpName" => {
                            let corp =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            pers_name
                                .children
                                .push(PersNameChild::CorpName(Box::new(corp)));
                        }
                        "persName" => {
                            let nested =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            pers_name
                                .children
                                .push(PersNameChild::PersName(Box::new(nested)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            pers_name
                                .children
                                .push(PersNameChild::Name(Box::new(name_elem)));
                        }
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pers_name.children.push(PersNameChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            pers_name.children.push(PersNameChild::Lb(Box::new(lb)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            pers_name.children.push(PersNameChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            pers_name
                                .children
                                .push(PersNameChild::Identifier(Box::new(identifier)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(pers_name)
}

/// Parse an `<address>` element from within another element.
pub(crate) fn parse_address_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Address> {
    use tusk_model::elements::{Address, AddressChild};

    let mut address = Address::default();

    // Extract attributes
    address.common.extract_attributes(&mut attrs)?;
    address.facsimile.extract_attributes(&mut attrs)?;
    address.lang.extract_attributes(&mut attrs)?;

    // Address can contain: addrLine, street, postCode, settlement, country, region, bloc, geogFeat, district, postBox
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("address")?
        {
            match name.as_str() {
                "addrLine" => {
                    let addr_line = parse_addr_line_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::AddrLine(Box::new(addr_line)));
                }
                "street" => {
                    let street = parse_street_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Street(Box::new(street)));
                }
                "postCode" => {
                    let post_code = parse_post_code_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::PostCode(Box::new(post_code)));
                }
                "settlement" => {
                    let settlement = parse_settlement_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Settlement(Box::new(settlement)));
                }
                "country" => {
                    let country = parse_country_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Country(Box::new(country)));
                }
                "region" => {
                    let region = parse_region_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Region(Box::new(region)));
                }
                "bloc" => {
                    let bloc = parse_bloc_from_event(reader, child_attrs, child_empty)?;
                    address.children.push(AddressChild::Bloc(Box::new(bloc)));
                }
                "geogFeat" => {
                    let geog_feat = parse_geog_feat_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::GeogFeat(Box::new(geog_feat)));
                }
                "district" => {
                    let district = parse_district_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::District(Box::new(district)));
                }
                "postBox" => {
                    let post_box = parse_post_box_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::PostBox(Box::new(post_box)));
                }
                // Skip unknown children in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(address)
}

/// Parse an `<addrLine>` element from within another element.
pub(crate) fn parse_addr_line_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AddrLine> {
    let mut addr_line = AddrLine::default();

    // Extract attributes
    addr_line.common.extract_attributes(&mut attrs)?;
    addr_line.facsimile.extract_attributes(&mut attrs)?;
    addr_line.lang.extract_attributes(&mut attrs)?;

    // addrLine is a mixed content element - can have text and child elements like geogName
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("addrLine")? {
            match content {
                MixedContent::Text(text) => {
                    addr_line.children.push(AddrLineChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::GeogName(Box::new(geog_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::PersName(Box::new(pers_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::Address(Box::new(address)));
                        }
                        "street" => {
                            let street = parse_street_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::Street(Box::new(street)));
                        }
                        "postCode" => {
                            let post_code =
                                parse_post_code_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::PostCode(Box::new(post_code)));
                        }
                        "postBox" => {
                            let post_box =
                                parse_post_box_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::PostBox(Box::new(post_box)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(addr_line)
}

/// Parse a `<geogName>` element from within another element.
pub(crate) fn parse_geog_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<GeogName> {
    let mut geog_name = GeogName::default();

    // Extract attributes
    geog_name.common.extract_attributes(&mut attrs)?;
    geog_name.bibl.extract_attributes(&mut attrs)?;
    geog_name.edit.extract_attributes(&mut attrs)?;
    geog_name.facsimile.extract_attributes(&mut attrs)?;
    geog_name.lang.extract_attributes(&mut attrs)?;
    geog_name.name.extract_attributes(&mut attrs)?;

    // geogName is a mixed content element - can have text and child elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("geogName")? {
            match content {
                MixedContent::Text(text) => {
                    geog_name.children.push(GeogNameChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "geogName" => {
                            // Nested geogName
                            let nested =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::GeogName(Box::new(nested)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::PersName(Box::new(pers_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::Address(Box::new(address)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(geog_name)
}

/// Parse a `<street>` element from within another element.
pub(crate) fn parse_street_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Street> {
    use tusk_model::elements::{Street, StreetChild};

    let mut street = Street::default();

    // Extract attributes
    street.common.extract_attributes(&mut attrs)?;
    street.facsimile.extract_attributes(&mut attrs)?;
    street.lang.extract_attributes(&mut attrs)?;

    // street is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("street")? {
            if !text.is_empty() {
                street.children.push(StreetChild::Text(text));
            }
        }
    }

    Ok(street)
}

/// Parse a `<postCode>` element from within another element.
pub(crate) fn parse_post_code_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::PostCode> {
    use tusk_model::elements::{PostCode, PostCodeChild};

    let mut post_code = PostCode::default();

    // Extract attributes
    post_code.common.extract_attributes(&mut attrs)?;
    post_code.facsimile.extract_attributes(&mut attrs)?;
    post_code.lang.extract_attributes(&mut attrs)?;

    // postCode is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("postCode")? {
            if !text.is_empty() {
                post_code.children.push(PostCodeChild::Text(text));
            }
        }
    }

    Ok(post_code)
}

/// Parse a `<settlement>` element from within another element.
pub(crate) fn parse_settlement_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Settlement> {
    use tusk_model::elements::{Settlement, SettlementChild};

    let mut settlement = Settlement::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    settlement.common.extract_attributes(&mut attrs)?;
    settlement.bibl.extract_attributes(&mut attrs)?;
    settlement.edit.extract_attributes(&mut attrs)?;
    settlement.facsimile.extract_attributes(&mut attrs)?;
    settlement.lang.extract_attributes(&mut attrs)?;
    settlement.name.extract_attributes(&mut attrs)?;

    // settlement is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("settlement")? {
            if !text.is_empty() {
                settlement.children.push(SettlementChild::Text(text));
            }
        }
    }

    Ok(settlement)
}

/// Parse a `<country>` element from within another element.
pub(crate) fn parse_country_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Country> {
    use tusk_model::elements::{Country, CountryChild};

    let mut country = Country::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    country.common.extract_attributes(&mut attrs)?;
    country.bibl.extract_attributes(&mut attrs)?;
    country.edit.extract_attributes(&mut attrs)?;
    country.facsimile.extract_attributes(&mut attrs)?;
    country.lang.extract_attributes(&mut attrs)?;
    country.name.extract_attributes(&mut attrs)?;

    // country is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("country")? {
            if !text.is_empty() {
                country.children.push(CountryChild::Text(text));
            }
        }
    }

    Ok(country)
}

/// Parse a `<region>` element from within another element.
pub(crate) fn parse_region_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Region> {
    use tusk_model::elements::{Region, RegionChild};

    let mut region = Region::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    region.common.extract_attributes(&mut attrs)?;
    region.bibl.extract_attributes(&mut attrs)?;
    region.edit.extract_attributes(&mut attrs)?;
    region.facsimile.extract_attributes(&mut attrs)?;
    region.lang.extract_attributes(&mut attrs)?;
    region.name.extract_attributes(&mut attrs)?;

    // region is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("region")? {
            if !text.is_empty() {
                region.children.push(RegionChild::Text(text));
            }
        }
    }

    Ok(region)
}

/// Parse a `<bloc>` element from within another element.
pub(crate) fn parse_bloc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Bloc> {
    use tusk_model::elements::{Bloc, BlocChild};

    let mut bloc = Bloc::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    bloc.common.extract_attributes(&mut attrs)?;
    bloc.bibl.extract_attributes(&mut attrs)?;
    bloc.edit.extract_attributes(&mut attrs)?;
    bloc.facsimile.extract_attributes(&mut attrs)?;
    bloc.lang.extract_attributes(&mut attrs)?;
    bloc.name.extract_attributes(&mut attrs)?;

    // bloc is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("bloc")? {
            if !text.is_empty() {
                bloc.children.push(BlocChild::Text(text));
            }
        }
    }

    Ok(bloc)
}

/// Parse a `<geogFeat>` element from within another element.
pub(crate) fn parse_geog_feat_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::GeogFeat> {
    use tusk_model::elements::{GeogFeat, GeogFeatChild};

    let mut geog_feat = GeogFeat::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    geog_feat.common.extract_attributes(&mut attrs)?;
    geog_feat.bibl.extract_attributes(&mut attrs)?;
    geog_feat.edit.extract_attributes(&mut attrs)?;
    geog_feat.facsimile.extract_attributes(&mut attrs)?;
    geog_feat.lang.extract_attributes(&mut attrs)?;
    geog_feat.name.extract_attributes(&mut attrs)?;

    // geogFeat is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("geogFeat")? {
            if !text.is_empty() {
                geog_feat.children.push(GeogFeatChild::Text(text));
            }
        }
    }

    Ok(geog_feat)
}

/// Parse a `<district>` element from within another element.
pub(crate) fn parse_district_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::District> {
    use tusk_model::elements::{District, DistrictChild};

    let mut district = District::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    district.common.extract_attributes(&mut attrs)?;
    district.bibl.extract_attributes(&mut attrs)?;
    district.edit.extract_attributes(&mut attrs)?;
    district.facsimile.extract_attributes(&mut attrs)?;
    district.lang.extract_attributes(&mut attrs)?;
    district.name.extract_attributes(&mut attrs)?;

    // district is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("district")? {
            if !text.is_empty() {
                district.children.push(DistrictChild::Text(text));
            }
        }
    }

    Ok(district)
}

/// Parse a `<postBox>` element from within another element.
pub(crate) fn parse_post_box_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::PostBox> {
    use tusk_model::elements::{PostBox, PostBoxChild};

    let mut post_box = PostBox::default();

    // Extract attributes
    post_box.common.extract_attributes(&mut attrs)?;
    post_box.facsimile.extract_attributes(&mut attrs)?;
    post_box.lang.extract_attributes(&mut attrs)?;

    // postBox is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("postBox")? {
            if !text.is_empty() {
                post_box.children.push(PostBoxChild::Text(text));
            }
        }
    }

    Ok(post_box)
}

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for FileDesc {
    fn element_name() -> &'static str {
        "fileDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_file_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TitleStmt {
    fn element_name() -> &'static str {
        "titleStmt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_title_stmt_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Title {
    fn element_name() -> &'static str {
        "title"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_title_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TitlePart {
    fn element_name() -> &'static str {
        "titlePart"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_title_part_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PubStmt {
    fn element_name() -> &'static str {
        "pubStmt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pub_stmt_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SourceDesc {
    fn element_name() -> &'static str {
        "sourceDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_source_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Publisher {
    fn element_name() -> &'static str {
        "publisher"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_publisher_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for CorpName {
    fn element_name() -> &'static str {
        "corpName"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_corp_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PersName {
    fn element_name() -> &'static str {
        "persName"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pers_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for EncodingDesc {
    fn element_name() -> &'static str {
        "encodingDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_encoding_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AppInfo {
    fn element_name() -> &'static str {
        "appInfo"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_app_info_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Application {
    fn element_name() -> &'static str {
        "application"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_application_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for EditorialDecl {
    fn element_name() -> &'static str {
        "editorialDecl"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_editorial_decl_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ProjectDesc {
    fn element_name() -> &'static str {
        "projectDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_project_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Source {
    fn element_name() -> &'static str {
        "source"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_source_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Identifier {
    fn element_name() -> &'static str {
        "identifier"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_identifier_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Creator {
    fn element_name() -> &'static str {
        "creator"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_creator_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Editor {
    fn element_name() -> &'static str {
        "editor"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_editor_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Funder {
    fn element_name() -> &'static str {
        "funder"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_funder_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Sponsor {
    fn element_name() -> &'static str {
        "sponsor"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sponsor_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Contributor {
    fn element_name() -> &'static str {
        "contributor"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_contributor_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Resp {
    fn element_name() -> &'static str {
        "resp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_resp_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for RespStmt {
    fn element_name() -> &'static str {
        "respStmt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_resp_stmt_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for BiblScope {
    fn element_name() -> &'static str {
        "biblScope"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_bibl_scope_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Bibl {
    fn element_name() -> &'static str {
        "bibl"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_bibl_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for BiblStruct {
    fn element_name() -> &'static str {
        "biblStruct"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_bibl_struct_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Imprint {
    fn element_name() -> &'static str {
        "imprint"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_imprint_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Locus {
    fn element_name() -> &'static str {
        "locus"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_locus_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for LocusGrp {
    fn element_name() -> &'static str {
        "locusGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_locus_grp_from_event(reader, attrs, is_empty)
    }
}

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{
        AppInfo, AppInfoChild, Application, ApplicationChild, CreatorChild, EditorChild,
        EditorialDecl, EditorialDeclChild, EncodingDesc, EncodingDescChild, FileDesc,
        FileDescChild, MeiHead, MeiHeadChild, PChild, ProjectDesc, ProjectDescChild, PubStmt,
        PubStmtChild, Source, SourceChild, SourceDesc, SourceDescChild, TitleChild, TitleStmt,
        TitleStmtChild,
    };

    // ============================================================================
    // MeiHead element tests
    // ============================================================================

    #[test]
    fn mei_head_deserializes_from_empty_element() {
        let xml = r#"<meiHead/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
        assert!(mei_head.basic.xml_id.is_none());
        assert!(mei_head.children.is_empty());
    }

    #[test]
    fn mei_head_deserializes_xml_id() {
        let xml = r#"<meiHead xml:id="header1"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
        assert_eq!(mei_head.basic.xml_id, Some("header1".to_string()));
    }

    #[test]
    fn mei_head_deserializes_file_desc_child() {
        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1"/>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    // ========== FileDesc tests ==========

    #[test]
    fn file_desc_deserializes_empty_element() {
        let xml = r#"<fileDesc/>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
        assert!(file_desc.common.xml_id.is_none());
        assert!(file_desc.children.is_empty());
    }

    #[test]
    fn file_desc_deserializes_xml_id() {
        let xml = r#"<fileDesc xml:id="fd1"/>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
    }

    #[test]
    fn file_desc_deserializes_title_stmt_child() {
        let xml = r#"<fileDesc xml:id="fd1">
            <titleStmt>
                <title>My Composition</title>
            </titleStmt>
        </fileDesc>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
        assert_eq!(file_desc.children.len(), 1);
        match &file_desc.children[0] {
            FileDescChild::TitleStmt(ts) => {
                assert_eq!(ts.children.len(), 1);
                assert!(matches!(&ts.children[0], TitleStmtChild::Title(_)));
            }
            _ => panic!("expected TitleStmt child"),
        }
    }

    // ========== TitleStmt tests ==========

    #[test]
    fn title_stmt_deserializes_empty_element() {
        let xml = r#"<titleStmt/>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert!(title_stmt.common.xml_id.is_none());
        assert!(title_stmt.children.is_empty());
    }

    #[test]
    fn title_stmt_deserializes_with_title_child() {
        let xml = r#"<titleStmt>
            <title>Test Title</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(title_stmt.children.len(), 1);
        match &title_stmt.children[0] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.children.len(), 1);
            }
            _ => panic!("expected Title child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_title_text_content() {
        let xml = r#"<titleStmt>
            <title>My Composition</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(title_stmt.children.len(), 1);
        match &title_stmt.children[0] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.children.len(), 1);
                match &t.children[0] {
                    TitleChild::Text(text) => assert_eq!(text.trim(), "My Composition"),
                    _ => panic!("expected text child in title"),
                }
            }
            _ => panic!("expected Title child"),
        }
    }

    // ========== PubStmt tests ==========

    #[test]
    fn pub_stmt_deserializes_empty_element() {
        let xml = r#"<pubStmt/>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
        assert!(pub_stmt.common.xml_id.is_none());
        assert!(pub_stmt.children.is_empty());
    }

    #[test]
    fn pub_stmt_deserializes_publisher_child() {
        let xml = r#"<pubStmt>
            <publisher>Music Press</publisher>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Publisher(p) => {
                assert!(!p.children.is_empty());
            }
            _ => panic!("expected Publisher child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_multiple_children() {
        let xml = r#"<pubStmt xml:id="ps1">
            <publisher xml:id="pub1">Music Press</publisher>
            <pubPlace>Vienna</pubPlace>
            <date>1800</date>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(pub_stmt.common.xml_id, Some("ps1".to_string()));
        assert_eq!(pub_stmt.children.len(), 3);
    }

    #[test]
    fn pub_stmt_deserializes_address_child() {
        let xml = r#"<pubStmt>
            <address>
                <addrLine>123 Music Street</addrLine>
            </address>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Address(addr) => {
                assert!(!addr.children.is_empty());
            }
            _ => panic!("expected Address child"),
        }
    }

    #[test]
    fn publisher_deserializes_with_corp_name_child() {
        use tusk_model::elements::{CorpNameChild, Publisher, PublisherChild};

        let xml = r#"<publisher>
            <corpName role="publisher">Musikwissenschaftliches Seminar, Detmold</corpName>
        </publisher>"#;
        let publisher = Publisher::from_mei_str(xml).expect("should deserialize");
        assert_eq!(publisher.children.len(), 1);
        match &publisher.children[0] {
            PublisherChild::CorpName(cn) => {
                assert_eq!(cn.name.role.len(), 1);
                assert_eq!(cn.children.len(), 1);
                match &cn.children[0] {
                    CorpNameChild::Text(text) => {
                        assert_eq!(text.trim(), "Musikwissenschaftliches Seminar, Detmold");
                    }
                    _ => panic!("expected Text child in corpName"),
                }
            }
            _ => panic!("expected CorpName child"),
        }
    }

    #[test]
    fn publisher_deserializes_with_mixed_content() {
        use tusk_model::elements::{Publisher, PublisherChild};

        let xml =
            r#"<publisher>Some text before <corpName>My Corp</corpName> and after</publisher>"#;
        let publisher = Publisher::from_mei_str(xml).expect("should deserialize");
        assert_eq!(publisher.children.len(), 3);
        match &publisher.children[0] {
            PublisherChild::Text(text) => {
                // XML whitespace before the corpName tag
                assert!(text.contains("Some text before"));
            }
            _ => panic!("expected Text child first"),
        }
        match &publisher.children[1] {
            PublisherChild::CorpName(_) => {}
            _ => panic!("expected CorpName child second"),
        }
        match &publisher.children[2] {
            PublisherChild::Text(text) => {
                // Text after the corpName
                assert!(text.contains("and after"));
            }
            _ => panic!("expected Text child third"),
        }
    }

    // ========== SourceDesc Tests ==========

    #[test]
    fn source_desc_deserializes_empty_element() {
        let xml = r#"<sourceDesc/>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert!(source_desc.common.xml_id.is_none());
        assert!(source_desc.children.is_empty());
    }

    #[test]
    fn source_desc_deserializes_source_child() {
        let xml = r#"<sourceDesc>
            <source xml:id="src1"/>
        </sourceDesc>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.children.len(), 1);
        match &source_desc.children[0] {
            SourceDescChild::Source(src) => {
                assert_eq!(src.common.xml_id, Some("src1".to_string()));
            }
            _ => panic!("expected Source child"),
        }
    }

    // ========== EncodingDesc tests ==========

    #[test]
    fn encoding_desc_deserializes_empty_element() {
        let xml = r#"<encodingDesc/>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");
        assert!(encoding_desc.common.xml_id.is_none());
        assert!(encoding_desc.children.is_empty());
    }

    #[test]
    fn encoding_desc_deserializes_app_info_child() {
        let xml = r#"<encodingDesc>
            <appInfo xml:id="ai1"/>
        </encodingDesc>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(encoding_desc.children.len(), 1);
        match &encoding_desc.children[0] {
            EncodingDescChild::AppInfo(ai) => {
                assert_eq!(ai.common.xml_id, Some("ai1".to_string()));
            }
            _ => panic!("expected AppInfo child"),
        }
    }

    // ========== AppInfo tests ==========

    #[test]
    fn app_info_deserializes_empty_element() {
        let xml = r#"<appInfo/>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");
        assert!(app_info.common.xml_id.is_none());
        assert!(app_info.children.is_empty());
    }

    #[test]
    fn app_info_deserializes_application_child() {
        let xml = r#"<appInfo>
            <application xml:id="app1">
                <name>Tusk</name>
            </application>
        </appInfo>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");
        assert_eq!(app_info.children.len(), 1);
        match &app_info.children[0] {
            AppInfoChild::Application(app) => {
                assert_eq!(app.common.xml_id, Some("app1".to_string()));
            }
            _ => panic!("expected Application child"),
        }
    }

    // ========== EditorialDecl tests ==========

    #[test]
    fn editorial_decl_deserializes_empty_element() {
        let xml = r#"<editorialDecl/>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");
        assert!(editorial_decl.common.xml_id.is_none());
        assert!(editorial_decl.children.is_empty());
    }

    // ========== ProjectDesc tests ==========

    #[test]
    fn project_desc_deserializes_empty_element() {
        let xml = r#"<projectDesc/>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");
        assert!(project_desc.common.xml_id.is_none());
        assert!(project_desc.children.is_empty());
    }

    #[test]
    fn project_desc_deserializes_with_p_child() {
        let xml = r#"<projectDesc xml:id="pd1">
            <p>This project aims to create a digital edition.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(project_desc.children.len(), 1);
        match &project_desc.children[0] {
            ProjectDescChild::P(p) => {
                assert_eq!(p.children.len(), 1);
                match &p.children[0] {
                    PChild::Text(text) => {
                        assert!(text.contains("digital edition"));
                    }
                    _ => panic!("expected Text child"),
                }
            }
            _ => panic!("expected P child"),
        }
    }

    // ========== Integration tests ==========

    #[test]
    fn mei_head_file_desc_title_stmt_integration() {
        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1">
                <titleStmt xml:id="ts1">
                    <title>Symphony No. 5</title>
                    <creator>Ludwig van Beethoven</creator>
                </titleStmt>
            </fileDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
                assert_eq!(fd.children.len(), 1);
                match &fd.children[0] {
                    FileDescChild::TitleStmt(ts) => {
                        assert_eq!(ts.common.xml_id, Some("ts1".to_string()));
                        assert_eq!(ts.children.len(), 2);
                    }
                    _ => panic!("expected TitleStmt child"),
                }
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    // ========== Deprecated element migration tests ==========

    #[test]
    fn title_stmt_deserializes_deprecated_composer_as_creator() {
        let xml = r#"<titleStmt>
            <title>Walzer G-Dur</title>
            <composer>Dionisio Aguado</composer>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(title_stmt.children.len(), 2);

        // First child should be title
        assert!(matches!(&title_stmt.children[0], TitleStmtChild::Title(_)));

        // Second child should be Creator (migrated from composer)
        match &title_stmt.children[1] {
            TitleStmtChild::Creator(creator) => {
                // Verify the role was set to composer (Cmp)
                assert_eq!(creator.name.role.len(), 1);
                match &creator.name.role[0] {
                    tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                        assert_eq!(
                            *role,
                            tusk_model::generated::data::DataMarcrelatorsBasic::Cmp
                        );
                    }
                    _ => panic!("expected DataMarcrelatorsBasic role"),
                }
                // Verify text content was captured
                assert!(!creator.children.is_empty());
            }
            _ => panic!("expected Creator child (migrated from composer)"),
        }
    }

    #[test]
    fn title_stmt_deserializes_deprecated_lyricist_as_creator() {
        let xml = r#"<titleStmt>
            <title>A Song</title>
            <lyricist>A Poet</lyricist>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(title_stmt.children.len(), 2);

        match &title_stmt.children[1] {
            TitleStmtChild::Creator(creator) => {
                assert_eq!(creator.name.role.len(), 1);
                match &creator.name.role[0] {
                    tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                        assert_eq!(
                            *role,
                            tusk_model::generated::data::DataMarcrelatorsBasic::Lyr
                        );
                    }
                    _ => panic!("expected DataMarcrelatorsBasic role"),
                }
            }
            _ => panic!("expected Creator child (migrated from lyricist)"),
        }
    }

    #[test]
    fn title_stmt_deserializes_deprecated_arranger_as_creator() {
        let xml = r#"<titleStmt>
            <title>Arranged Work</title>
            <arranger>An Arranger</arranger>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(title_stmt.children.len(), 2);

        match &title_stmt.children[1] {
            TitleStmtChild::Creator(creator) => {
                assert_eq!(creator.name.role.len(), 1);
                match &creator.name.role[0] {
                    tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                        assert_eq!(
                            *role,
                            tusk_model::generated::data::DataMarcrelatorsBasic::Arr
                        );
                    }
                    _ => panic!("expected DataMarcrelatorsBasic role"),
                }
            }
            _ => panic!("expected Creator child (migrated from arranger)"),
        }
    }

    #[test]
    fn title_stmt_deserializes_deprecated_author_as_creator() {
        let xml = r#"<titleStmt>
            <title>A Text Work</title>
            <author>An Author</author>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
        assert_eq!(title_stmt.children.len(), 2);

        match &title_stmt.children[1] {
            TitleStmtChild::Creator(creator) => {
                assert_eq!(creator.name.role.len(), 1);
                match &creator.name.role[0] {
                    tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                        assert_eq!(
                            *role,
                            tusk_model::generated::data::DataMarcrelatorsBasic::Aut
                        );
                    }
                    _ => panic!("expected DataMarcrelatorsBasic role"),
                }
            }
            _ => panic!("expected Creator child (migrated from author)"),
        }
    }

    // ========== Title mixed content tests ==========

    #[test]
    fn title_deserializes_with_title_part_child() {
        use tusk_model::elements::{Title, TitleChild, TitlePartChild};

        // Note: @type attribute on titlePart is a local attribute not yet generated in the model
        // (tracked as CODEGEN_BUG in tasks_mei_roundtrip.md)
        let xml =
            r#"<title>Walzer G-Dur<titlePart>an electronic transcription</titlePart></title>"#;
        let title = Title::from_mei_str(xml).expect("should deserialize");

        // Should have 2 children: text "Walzer G-Dur" and titlePart element
        assert_eq!(title.children.len(), 2);

        // First child should be text
        match &title.children[0] {
            TitleChild::Text(text) => {
                assert_eq!(text, "Walzer G-Dur");
            }
            _ => panic!("expected Text child first"),
        }

        // Second child should be titlePart
        match &title.children[1] {
            TitleChild::TitlePart(tp) => {
                assert_eq!(tp.children.len(), 1);
                match &tp.children[0] {
                    TitlePartChild::Text(text) => {
                        assert_eq!(text, "an electronic transcription");
                    }
                    _ => panic!("expected Text child in titlePart"),
                }
            }
            _ => panic!("expected TitlePart child second"),
        }
    }

    #[test]
    fn title_deserializes_text_only() {
        use tusk_model::elements::{Title, TitleChild};

        let xml = r#"<title>Simple Title</title>"#;
        let title = Title::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title.children.len(), 1);
        match &title.children[0] {
            TitleChild::Text(text) => {
                assert_eq!(text, "Simple Title");
            }
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn title_deserializes_empty_element() {
        use tusk_model::elements::Title;

        let xml = r#"<title/>"#;
        let title = Title::from_mei_str(xml).expect("should deserialize");
        assert!(title.children.is_empty());
    }

    // ========== Funder tests (via TitleStmt wrapper) ==========

    #[test]
    fn funder_deserializes_with_corp_name_child() {
        use tusk_model::elements::{CorpNameChild, FunderChild};

        let xml = r#"<titleStmt>
          <title>Test</title>
          <funder>
            <corpName role="funder" codedval="2007744-0" auth.uri="http://d-nb.info/gnd/" auth="GND">German Research Foundation</corpName>
          </funder>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        // Find the funder child
        let funder = title_stmt
            .children
            .iter()
            .find_map(|c| {
                if let TitleStmtChild::Funder(f) = c {
                    Some(f)
                } else {
                    None
                }
            })
            .expect("should have funder child");

        // Should have one child: corpName
        assert_eq!(funder.children.len(), 1);
        match &funder.children[0] {
            FunderChild::CorpName(cn) => {
                assert_eq!(cn.name.role.len(), 1);
                // Check the text content
                assert_eq!(cn.children.len(), 1);
                match &cn.children[0] {
                    CorpNameChild::Text(text) => {
                        assert_eq!(text, "German Research Foundation");
                    }
                    _ => panic!("expected Text child in corpName"),
                }
            }
            _ => panic!("expected CorpName child, got {:?}", funder.children[0]),
        }
    }

    #[test]
    fn funder_deserializes_corp_name_with_nested_address() {
        use tusk_model::elements::{CorpNameChild, FunderChild};

        let xml = r#"<titleStmt>
          <title>Test</title>
          <funder>
            <corpName role="funder">German Research Foundation
              <address>
                <addrLine>Kennedyallee 40</addrLine>
              </address>
            </corpName>
          </funder>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        // Find the funder child
        let funder = title_stmt
            .children
            .iter()
            .find_map(|c| {
                if let TitleStmtChild::Funder(f) = c {
                    Some(f)
                } else {
                    None
                }
            })
            .expect("should have funder child");

        assert_eq!(funder.children.len(), 1);
        match &funder.children[0] {
            FunderChild::CorpName(cn) => {
                // Should have text and address children
                assert!(
                    cn.children.len() >= 2,
                    "expected at least 2 children (text + address)"
                );

                // Check for text content
                let has_text = cn
                    .children
                    .iter()
                    .any(|c| matches!(c, CorpNameChild::Text(_)));
                assert!(has_text, "should have text content");

                // Check for address child
                let has_address = cn.children.iter().any(|c| {
                    if let CorpNameChild::Address(addr) = c {
                        !addr.children.is_empty()
                    } else {
                        false
                    }
                });
                assert!(has_address, "should have address child with addrLine");
            }
            _ => panic!("expected CorpName child"),
        }
    }

    #[test]
    fn funder_deserializes_text_only() {
        use tusk_model::elements::FunderChild;

        let xml = r#"<titleStmt>
          <title>Test</title>
          <funder>Anonymous Donor</funder>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        // Find the funder child
        let funder = title_stmt
            .children
            .iter()
            .find_map(|c| {
                if let TitleStmtChild::Funder(f) = c {
                    Some(f)
                } else {
                    None
                }
            })
            .expect("should have funder child");

        assert_eq!(funder.children.len(), 1);
        match &funder.children[0] {
            FunderChild::Text(text) => {
                assert_eq!(text, "Anonymous Donor");
            }
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn funder_deserializes_empty_element() {
        let xml = r#"<titleStmt>
          <title>Test</title>
          <funder/>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        // Find the funder child
        let funder = title_stmt
            .children
            .iter()
            .find_map(|c| {
                if let TitleStmtChild::Funder(f) = c {
                    Some(f)
                } else {
                    None
                }
            })
            .expect("should have funder child");

        assert!(funder.children.is_empty());
    }

    // ========== Identifier tests (via PubStmt wrapper) ==========

    #[test]
    fn identifier_deserializes_with_ref_child() {
        use tusk_model::elements::IdentifierChild;

        let xml = r#"<pubStmt>
          <identifier>
            <ref target="http://music-encoding.org/Support/MEI_Sample_Collection"/>
          </identifier>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        // Find the identifier child
        let identifier = pub_stmt
            .children
            .iter()
            .find_map(|c| {
                if let PubStmtChild::Identifier(id) = c {
                    Some(id)
                } else {
                    None
                }
            })
            .expect("should have identifier child");

        // Should have a ref child
        assert_eq!(identifier.children.len(), 1);
        match &identifier.children[0] {
            IdentifierChild::Ref(r) => {
                assert_eq!(r.pointing.target.len(), 1);
                assert_eq!(
                    r.pointing.target[0].0,
                    "http://music-encoding.org/Support/MEI_Sample_Collection"
                );
            }
            other => panic!("expected Ref child, got {:?}", other),
        }
    }

    #[test]
    fn identifier_deserializes_text_only() {
        use tusk_model::elements::IdentifierChild;

        let xml = r#"<pubStmt>
          <identifier type="URI">http://example.com/test</identifier>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        let identifier = pub_stmt
            .children
            .iter()
            .find_map(|c| {
                if let PubStmtChild::Identifier(id) = c {
                    Some(id)
                } else {
                    None
                }
            })
            .expect("should have identifier child");

        assert_eq!(identifier.children.len(), 1);
        match &identifier.children[0] {
            IdentifierChild::Text(text) => {
                assert_eq!(text.trim(), "http://example.com/test");
            }
            other => panic!("expected Text child, got {:?}", other),
        }
    }

    #[test]
    fn identifier_deserializes_mixed_content() {
        use tusk_model::elements::IdentifierChild;

        let xml = r#"<pubStmt>
          <identifier>ISMN <ref target="http://ismn.org/">979-0-1234-5678-9</ref></identifier>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        let identifier = pub_stmt
            .children
            .iter()
            .find_map(|c| {
                if let PubStmtChild::Identifier(id) = c {
                    Some(id)
                } else {
                    None
                }
            })
            .expect("should have identifier child");

        // Should have text followed by ref child
        assert_eq!(identifier.children.len(), 2);
        match &identifier.children[0] {
            IdentifierChild::Text(text) => {
                assert_eq!(text.trim(), "ISMN");
            }
            other => panic!("expected Text child first, got {:?}", other),
        }
        match &identifier.children[1] {
            IdentifierChild::Ref(r) => {
                assert_eq!(r.pointing.target.len(), 1);
                assert_eq!(r.pointing.target[0].0, "http://ismn.org/");
            }
            other => panic!("expected Ref child second, got {:?}", other),
        }
    }

    #[test]
    fn identifier_deserializes_empty_element() {
        let xml = r#"<pubStmt>
          <identifier/>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        let identifier = pub_stmt
            .children
            .iter()
            .find_map(|c| {
                if let PubStmtChild::Identifier(id) = c {
                    Some(id)
                } else {
                    None
                }
            })
            .expect("should have identifier child");

        assert!(identifier.children.is_empty());
    }

    // ============================================================================
    // Bibl element tests
    // ============================================================================

    #[test]
    fn bibl_deserializes_text_content() {
        use tusk_model::elements::BiblChild;

        // bibl with text content and attributes (similar to Aguado_Walzer_G-major.mei line 122)
        let xml = r#"<source>
          <bibl xml:id="OCLC_DDC" target="http://example.com">OCLC_DDC</bibl>
        </source>"#;

        let source = Source::from_mei_str(xml).expect("should deserialize");

        let bibl = source
            .children
            .iter()
            .find_map(|c| {
                if let SourceChild::Bibl(b) = c {
                    Some(b)
                } else {
                    None
                }
            })
            .expect("should have bibl child");

        // Check attributes
        assert_eq!(bibl.common.xml_id, Some("OCLC_DDC".to_string()));
        assert!(!bibl.pointing.target.is_empty());

        // Check text content is preserved
        assert_eq!(bibl.children.len(), 1);
        match &bibl.children[0] {
            BiblChild::Text(text) => assert_eq!(text, "OCLC_DDC"),
            other => panic!("expected Text child, got {:?}", other),
        }
    }

    #[test]
    fn bibl_deserializes_mixed_content() {
        use tusk_model::elements::BiblChild;

        // bibl with both child elements and text content
        let xml = r#"<source>
          <bibl>
            <title>Some Title</title>
            with some text
          </bibl>
        </source>"#;

        let source = Source::from_mei_str(xml).expect("should deserialize");

        let bibl = source
            .children
            .iter()
            .find_map(|c| {
                if let SourceChild::Bibl(b) = c {
                    Some(b)
                } else {
                    None
                }
            })
            .expect("should have bibl child");

        // Should have both title element and text content
        assert_eq!(bibl.children.len(), 2);

        // First child should be Title
        match &bibl.children[0] {
            BiblChild::Title(_t) => {
                // Title text check can be added if needed
            }
            other => panic!("expected Title child first, got {:?}", other),
        }

        // Second child should be Text
        match &bibl.children[1] {
            BiblChild::Text(text) => {
                assert!(text.contains("with some text"));
            }
            other => panic!("expected Text child second, got {:?}", other),
        }
    }

    #[test]
    fn bibl_deserializes_editor_child() {
        use tusk_model::elements::BiblChild;

        // bibl with editor child element
        let xml = r#"<source>
          <bibl>
            <title>Test Work</title>
            <editor>John Smith</editor>
          </bibl>
        </source>"#;

        let source = Source::from_mei_str(xml).expect("should deserialize");

        let bibl = source
            .children
            .iter()
            .find_map(|c| {
                if let SourceChild::Bibl(b) = c {
                    Some(b)
                } else {
                    None
                }
            })
            .expect("should have bibl child");

        // Should have title and editor
        assert_eq!(bibl.children.len(), 2);

        // First child should be Title
        assert!(matches!(&bibl.children[0], BiblChild::Title(_)));

        // Second child should be Editor
        match &bibl.children[1] {
            BiblChild::Editor(editor) => {
                // Check text content
                assert!(editor.children.iter().any(|c| {
                    if let tusk_model::elements::EditorChild::Text(t) = c {
                        t.contains("John Smith")
                    } else {
                        false
                    }
                }));
            }
            other => panic!("expected Editor child, got {:?}", other),
        }
    }

    #[test]
    fn bibl_deserializes_deprecated_librettist_as_creator() {
        use tusk_model::elements::BiblChild;

        // bibl with deprecated librettist element (MEI 5.x)
        let xml = r#"<source>
          <bibl>
            <title>Test Work</title>
            <librettist>
              <persName role="librettist">John Doe</persName>
            </librettist>
          </bibl>
        </source>"#;

        let source = Source::from_mei_str(xml).expect("should deserialize");

        let bibl = source
            .children
            .iter()
            .find_map(|c| {
                if let SourceChild::Bibl(b) = c {
                    Some(b)
                } else {
                    None
                }
            })
            .expect("should have bibl child");

        // Should have title and creator (migrated from librettist)
        assert_eq!(bibl.children.len(), 2);

        // First child should be Title
        assert!(matches!(&bibl.children[0], BiblChild::Title(_)));

        // Second child should be Creator (migrated from librettist)
        match &bibl.children[1] {
            BiblChild::Creator(creator) => {
                // Verify the role was set to librettist (Lbt)
                assert_eq!(creator.name.role.len(), 1);
                match &creator.name.role[0] {
                    tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                        assert_eq!(
                            *role,
                            tusk_model::generated::data::DataMarcrelatorsBasic::Lbt
                        );
                    }
                    _ => panic!("expected DataMarcrelatorsBasic role"),
                }
                // Verify persName child was parsed
                assert!(
                    creator
                        .children
                        .iter()
                        .any(|c| { matches!(c, tusk_model::elements::CreatorChild::PersName(_)) })
                );
            }
            other => panic!(
                "expected Creator child (migrated from librettist), got {:?}",
                other
            ),
        }
    }

    #[test]
    fn editor_deserializes_pers_name_child() {
        use tusk_model::elements::BiblChild;

        // editor with persName child element
        let xml = r#"<source>
          <bibl>
            <editor>
              <persName>John Smith</persName>
            </editor>
          </bibl>
        </source>"#;

        let source = Source::from_mei_str(xml).expect("should deserialize");

        let bibl = source
            .children
            .iter()
            .find_map(|c| {
                if let SourceChild::Bibl(b) = c {
                    Some(b)
                } else {
                    None
                }
            })
            .expect("should have bibl child");

        // Should have one editor
        assert_eq!(bibl.children.len(), 1);

        // First child should be Editor
        match &bibl.children[0] {
            BiblChild::Editor(editor) => {
                // Should have one persName child
                assert_eq!(editor.children.len(), 1);
                match &editor.children[0] {
                    EditorChild::PersName(pers_name) => {
                        // Check text content of persName
                        assert!(pers_name.children.iter().any(|c| {
                            if let tusk_model::elements::PersNameChild::Text(t) = c {
                                t.contains("John Smith")
                            } else {
                                false
                            }
                        }));
                    }
                    other => panic!("expected PersName child, got {:?}", other),
                }
            }
            other => panic!("expected Editor child, got {:?}", other),
        }
    }
}

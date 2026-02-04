//! Deserializer implementations for MEI header elements.
//!
//! This module contains implementations for MeiHead, FileDesc, TitleStmt, PubStmt,
//! and their child elements.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    AppInfo, AppInfoChild, Application, ApplicationChild, Availability, Bibl, BiblStruct,
    Contributor, ContributorChild, CorpName, CorpNameChild, Correction, CorrectionChild, Creator,
    CreatorChild, Date, Distributor, Editor, EditorChild, EditorialDecl, EditorialDeclChild,
    EncodingDesc, EncodingDescChild, FileDesc, FileDescChild, Funder, FunderChild, Head, HeadChild,
    Identifier, Interpretation, InterpretationChild, Locus, LocusGrp, MeiHead, MeiHeadChild, Name,
    NameChild, Normalization, NormalizationChild, P, PChild, PersName, PersNameChild, ProjectDesc,
    ProjectDescChild, Ptr, PubPlace, PubStmt, PubStmtChild, Publisher, PublisherChild, RespStmt,
    SamplingDecl, SamplingDeclChild, Segmentation, SegmentationChild, Source, SourceChild,
    SourceDesc, SourceDescChild, Sponsor, SponsorChild, StdVals, StdValsChild, Title, TitleChild,
    TitleStmt, TitleStmtChild, Unpub,
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
                    // Other child elements (manifestationList, etc.) are not
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
                // Other child elements (editionStmt, etc.) are not
                // yet implemented for parsing. Skip them in lenient mode.
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
                // address is part of model.pubStmtPart but more complex - skip for now
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
pub(crate) fn parse_bibl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Bibl> {
    let mut bibl = Bibl::default();

    // Extract attributes
    bibl.common.extract_attributes(&mut attrs)?;
    bibl.bibl.extract_attributes(&mut attrs)?;
    bibl.facsimile.extract_attributes(&mut attrs)?;
    bibl.lang.extract_attributes(&mut attrs)?;
    bibl.pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // bibl can contain text and many child elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("bibl")? {
            match name.as_str() {
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Title(Box::new(title)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Identifier(Box::new(
                            identifier,
                        )));
                }
                "creator" => {
                    let creator = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Creator(Box::new(creator)));
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
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Creator(Box::new(creator)));
                }
                "lyricist" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "lyricist",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Lyr,
                    )?;
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Creator(Box::new(creator)));
                }
                "arranger" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "arranger",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Arr,
                    )?;
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Creator(Box::new(creator)));
                }
                "author" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "author",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Aut,
                    )?;
                    bibl.children
                        .push(tusk_model::elements::BiblChild::Creator(Box::new(creator)));
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

    Ok(bibl)
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
                // domainsDecl, tagsDecl, classDecls are more complex - skip for now
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

    // Remaining attributes (like @version) are unknown - ignore in lenient mode

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
                    // ref is more complex - for now just skip it
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                "p" => {
                    // p is complex - for now just skip it
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
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

    // Read text content if not an empty element
    // name can contain text and many element types - for simplicity, just handle text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("name")? {
            if !text.trim().is_empty() {
                name_elem.children.push(NameChild::Text(text));
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

/// Parse a `<p>` (paragraph) element from within another element.
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

    // Read text content if not an empty element
    // p can contain text and many element types - for simplicity, just handle text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("p")? {
            if !text.trim().is_empty() {
                p.children.push(PChild::Text(text));
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
                // p elements are more complex - skip for now
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
pub(crate) fn parse_identifier_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Identifier> {
    let mut identifier = Identifier::default();

    // Extract attributes
    identifier.common.extract_attributes(&mut attrs)?;
    identifier.authorized.extract_attributes(&mut attrs)?;
    identifier.bibl.extract_attributes(&mut attrs)?;
    identifier.facsimile.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // identifier can contain text and various child elements
    // For now, we collect text content as IdentifierChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("identifier")? {
            if !text.trim().is_empty() {
                identifier
                    .children
                    .push(tusk_model::elements::IdentifierChild::Text(text));
            }
        }
    }

    Ok(identifier)
}

/// Parse an `<availability>` element from within another element.
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

    // availability doesn't have children in the generated model
    // Skip any content if present
    if !is_empty {
        reader.skip_to_end("availability")?;
    }

    Ok(availability)
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

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // title can contain text and various child elements
    // For now, we collect text content as TitleChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("title")? {
            if !text.trim().is_empty() {
                title.children.push(TitleChild::Text(text));
            }
        }
    }

    Ok(title)
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

    // respStmt can contain various child elements
    // For now, we skip children in lenient mode
    if !is_empty {
        reader.skip_to_end("respStmt")?;
    }

    Ok(resp_stmt)
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

    // Parse text content if not empty
    // editor can contain text and various child elements
    // For now, we collect text content as EditorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("editor")? {
            if !text.trim().is_empty() {
                editor.children.push(EditorChild::Text(text));
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

/// Parse a deprecated MEI element (composer, lyricist, arranger, author) as a Creator.
///
/// MEI 5.1 deprecated composer, lyricist, arranger, and author in favor of creator.
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

    // Parse text content if not empty
    // These deprecated elements can contain text and child elements like persName, corpName
    // For now, we collect text content as CreatorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end(element_name)? {
            if !text.trim().is_empty() {
                creator.children.push(CreatorChild::Text(text));
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

    // Parse text content if not empty
    // funder can contain text and various child elements
    // For now, we collect text content as FunderChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("funder")? {
            if !text.trim().is_empty() {
                funder.children.push(FunderChild::Text(text));
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
    let mut address = tusk_model::elements::Address::default();

    // Extract attributes
    address.common.extract_attributes(&mut attrs)?;
    address.facsimile.extract_attributes(&mut attrs)?;
    address.lang.extract_attributes(&mut attrs)?;

    // Address can contain addrLine, street, postCode, settlement, country, etc.
    // For now, just skip children in lenient mode
    if !is_empty {
        reader.skip_to_end("address")?;
    }

    Ok(address)
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
}

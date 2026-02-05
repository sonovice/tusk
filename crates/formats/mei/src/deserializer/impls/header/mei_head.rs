//! Core header elements (MeiHead, FileDesc, TitleStmt, SourceDesc, Source, Title, Head).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    FileDesc, FileDescChild, Head, HeadChild, MeiHead, MeiHeadChild, Source, SourceChild,
    SourceDesc, SourceDescChild, Title, TitleChild, TitlePart, TitlePartChild, TitleStmt,
    TitleStmtChild,
};

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

        // Read children if not an empty element
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
                        let encoding_desc = super::parse_encoding_desc_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei_head
                            .children
                            .push(MeiHeadChild::EncodingDesc(Box::new(encoding_desc)));
                    }
                    "workList" => {
                        let work_list = super::super::parse_work_list_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei_head
                            .children
                            .push(MeiHeadChild::WorkList(Box::new(work_list)));
                    }
                    "revisionDesc" => {
                        let revision_desc = super::super::parse_revision_desc_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei_head
                            .children
                            .push(MeiHeadChild::RevisionDesc(Box::new(revision_desc)));
                    }
                    "manifestationList" => {
                        let manifestation_list = super::super::parse_manifestation_list_from_event(
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

    // Read children if not an empty element
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
                    let pub_stmt =
                        super::parse_pub_stmt_from_event(reader, child_attrs, child_empty)?;
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
                    let series_stmt = super::super::parse_series_stmt_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    file_desc
                        .children
                        .push(FileDescChild::SeriesStmt(Box::new(series_stmt)));
                }
                "editionStmt" => {
                    let edition_stmt = super::super::parse_edition_stmt_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    file_desc
                        .children
                        .push(FileDescChild::EditionStmt(Box::new(edition_stmt)));
                }
                "notesStmt" => {
                    let notes_stmt = super::super::parse_notes_stmt_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    file_desc
                        .children
                        .push(FileDescChild::NotesStmt(Box::new(notes_stmt)));
                }
                "extent" => {
                    let extent =
                        super::super::parse_extent_from_event(reader, child_attrs, child_empty)?;
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

    // Read children if not an empty element
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
                    let resp_stmt =
                        super::parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "editor" => {
                    let editor = super::parse_editor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Editor(Box::new(editor)));
                }
                "creator" => {
                    let creator =
                        super::parse_creator_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                "funder" => {
                    let funder = super::parse_funder_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Funder(Box::new(funder)));
                }
                "sponsor" => {
                    let sponsor =
                        super::parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Sponsor(Box::new(sponsor)));
                }
                "contributor" => {
                    let contributor =
                        super::parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Contributor(Box::new(contributor)));
                }
                // Handle deprecated MEI elements by converting to Creator with appropriate role
                "composer" => {
                    let creator = super::parse_deprecated_creator_from_event(
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
                    let creator = super::parse_deprecated_creator_from_event(
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
                    let creator = super::parse_deprecated_creator_from_event(
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
                    let creator = super::parse_deprecated_creator_from_event(
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

/// Parse a `<sourceDesc>` element from within another element.
pub(crate) fn parse_source_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SourceDesc> {
    let mut source_desc = SourceDesc::default();

    // Extract attributes into AttCommon (sourceDesc only has common attributes)
    source_desc.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
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

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("source")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Head(Box::new(head)));
                }
                "locus" => {
                    let locus = super::parse_locus_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Locus(Box::new(locus)));
                }
                "locusGrp" => {
                    let locus_grp =
                        super::parse_locus_grp_from_event(reader, child_attrs, child_empty)?;
                    source
                        .children
                        .push(SourceChild::LocusGrp(Box::new(locus_grp)));
                }
                "bibl" => {
                    let bibl = super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        super::parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
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

/// Parse a `<title>` element from within another element.
pub(crate) fn parse_title_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Title> {
    let mut title = Title::default();

    // Extract attributes
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

    // title can contain mixed content (text and child elements)
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
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title.children.push(TitleChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title.children.push(TitleChild::Lb(Box::new(lb)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Name(Box::new(name_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Ptr(Box::new(ptr)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Ref(Box::new(ref_elem)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title
                                .children
                                .push(TitleChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title
                                .children
                                .push(TitleChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title
                                .children
                                .push(TitleChild::GeogName(Box::new(geog_name)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title
                                .children
                                .push(TitleChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Date(Box::new(date)));
                        }
                        "annot" => {
                            let annot =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            title.children.push(TitleChild::Annot(Box::new(annot)));
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

    Ok(title)
}

/// Parse a `<titlePart>` element from within another element.
pub(crate) fn parse_title_part_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TitlePart> {
    let mut title_part = TitlePart::default();

    // Extract attributes
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

    // titlePart can contain mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("titlePart")? {
            match content {
                MixedContent::Text(text) => {
                    title_part.children.push(TitlePartChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title_part
                                .children
                                .push(TitlePartChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title_part.children.push(TitlePartChild::Lb(Box::new(lb)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Name(Box::new(name_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            title_part.children.push(TitlePartChild::Ptr(Box::new(ptr)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Ref(Box::new(ref_elem)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title_part
                                .children
                                .push(TitlePartChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title_part
                                .children
                                .push(TitlePartChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title_part
                                .children
                                .push(TitlePartChild::GeogName(Box::new(geog_name)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            title_part
                                .children
                                .push(TitlePartChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Date(Box::new(date)));
                        }
                        "annot" => {
                            let annot =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            title_part
                                .children
                                .push(TitlePartChild::Annot(Box::new(annot)));
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

    Ok(title_part)
}

/// Parse a `<head>` element from within another element.
pub(crate) fn parse_head_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Head> {
    let mut head = Head::default();

    // Extract attributes
    head.common.extract_attributes(&mut attrs)?;
    head.facsimile.extract_attributes(&mut attrs)?;
    head.lang.extract_attributes(&mut attrs)?;

    // head can contain mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("head")? {
            match content {
                MixedContent::Text(text) => {
                    head.children.push(HeadChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            head.children.push(HeadChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            head.children.push(HeadChild::Lb(Box::new(lb)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            head.children.push(HeadChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            head.children.push(HeadChild::Ptr(Box::new(ptr)));
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

    Ok(head)
}

//! Agent/responsibility elements (Creator, Editor, RespStmt, PersName, CorpName, etc.).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    Contributor, ContributorChild, CorpName, CorpNameChild, Creator, CreatorChild, Editor,
    EditorChild, Funder, FunderChild, Name, NameChild, PersName, PersNameChild, Resp, RespChild,
    RespStmt, RespStmtChild, Sponsor, SponsorChild,
};

// MeiDeserialize trait implementations
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

/// Parse a `<respStmt>` element from within another element.
pub(crate) fn parse_resp_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RespStmt> {
    let mut resp_stmt = RespStmt::default();

    // Extract attributes
    resp_stmt.common.extract_attributes(&mut attrs)?;
    resp_stmt.bibl.extract_attributes(&mut attrs)?;
    resp_stmt.facsimile.extract_attributes(&mut attrs)?;

    // respStmt can contain: resp, name, persName, corpName
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("respStmt")?
        {
            match name.as_str() {
                "resp" => {
                    let resp = parse_resp_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt.children.push(RespStmtChild::Resp(Box::new(resp)));
                }
                "name" => {
                    let name_elem = parse_name_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::Name(Box::new(name_elem)));
                }
                "persName" => {
                    let pers_name = parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::PersName(Box::new(pers_name)));
                }
                "corpName" => {
                    let corp_name = parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                    resp_stmt
                        .children
                        .push(RespStmtChild::CorpName(Box::new(corp_name)));
                }
                _ => {
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

    // Extract attributes
    resp.common.extract_attributes(&mut attrs)?;
    resp.facsimile.extract_attributes(&mut attrs)?;
    resp.lang.extract_attributes(&mut attrs)?;

    // resp can contain text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("resp")? {
            if !text.is_empty() {
                resp.children.push(RespChild::Text(text));
            }
        }
    }

    Ok(resp)
}

/// Parse a `<creator>` element from within another element.
pub(crate) fn parse_creator_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Creator> {
    let mut creator = Creator::default();

    // Extract attributes
    creator.common.extract_attributes(&mut attrs)?;
    creator.bibl.extract_attributes(&mut attrs)?;
    creator.evidence.extract_attributes(&mut attrs)?;
    creator.facsimile.extract_attributes(&mut attrs)?;
    creator.lang.extract_attributes(&mut attrs)?;
    creator.name.extract_attributes(&mut attrs)?;

    // creator can contain mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("creator")? {
            match content {
                MixedContent::Text(text) => {
                    creator.children.push(CreatorChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::Name(Box::new(name_elem)));
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

    Ok(creator)
}

/// Parse a deprecated MEI element (composer, lyricist, arranger, author, librettist)
/// and convert it to a Creator with the appropriate role.
pub(crate) fn parse_deprecated_creator_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
    element_name: &str,
    role: tusk_model::generated::data::DataMarcrelatorsBasic,
) -> DeserializeResult<Creator> {
    let mut creator = Creator::default();

    // Extract attributes
    creator.common.extract_attributes(&mut attrs)?;
    creator.bibl.extract_attributes(&mut attrs)?;
    creator.evidence.extract_attributes(&mut attrs)?;
    creator.facsimile.extract_attributes(&mut attrs)?;
    creator.lang.extract_attributes(&mut attrs)?;
    creator.name.extract_attributes(&mut attrs)?;

    // Set the role based on the deprecated element type
    creator.name.role =
        vec![tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role)];

    // Parse mixed content (same as creator)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content(element_name)? {
            match content {
                MixedContent::Text(text) => {
                    creator.children.push(CreatorChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            creator
                                .children
                                .push(CreatorChild::Name(Box::new(name_elem)));
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

    Ok(creator)
}

/// Parse an `<editor>` element from within another element.
pub(crate) fn parse_editor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Editor> {
    let mut editor = Editor::default();

    // Extract attributes
    editor.common.extract_attributes(&mut attrs)?;
    editor.bibl.extract_attributes(&mut attrs)?;
    editor.evidence.extract_attributes(&mut attrs)?;
    editor.facsimile.extract_attributes(&mut attrs)?;
    editor.lang.extract_attributes(&mut attrs)?;

    // editor can contain mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("editor")? {
            match content {
                MixedContent::Text(text) => {
                    editor.children.push(EditorChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            editor
                                .children
                                .push(EditorChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            editor
                                .children
                                .push(EditorChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                parse_name_from_event(reader, child_attrs, child_empty)?;
                            editor.children.push(EditorChild::Name(Box::new(name_elem)));
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

    Ok(editor)
}

/// Parse a `<funder>` element from within another element.
pub(crate) fn parse_funder_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Funder> {
    let mut funder = Funder::default();

    // Extract attributes
    funder.common.extract_attributes(&mut attrs)?;
    funder.bibl.extract_attributes(&mut attrs)?;
    funder.evidence.extract_attributes(&mut attrs)?;
    funder.facsimile.extract_attributes(&mut attrs)?;
    funder.lang.extract_attributes(&mut attrs)?;

    // funder can contain mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("funder")? {
            match content {
                MixedContent::Text(text) => {
                    funder.children.push(FunderChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "persName" => {
                            let pers_name =
                                parse_pers_name_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name =
                                parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::CorpName(Box::new(corp_name)));
                        }
                        "address" => {
                            let address =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
                            funder
                                .children
                                .push(FunderChild::Address(Box::new(address)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Ref(Box::new(ref_elem)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            funder.children.push(FunderChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            funder.children.push(FunderChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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

    // Parse text content if not empty
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

    // Parse text content if not empty
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
                            let addr =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
                            corp_name
                                .children
                                .push(CorpNameChild::Address(Box::new(addr)));
                        }
                        "geogName" => {
                            let geog = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            corp_name
                                .children
                                .push(CorpNameChild::GeogName(Box::new(geog)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            corp_name.children.push(CorpNameChild::Date(Box::new(date)));
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
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pers_name.children.push(PersNameChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::text::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pers_name.children.push(PersNameChild::Lb(Box::new(lb)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            pers_name.children.push(PersNameChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            name_elem.children.push(NameChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::text::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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
                            let title =
                                super::parse_title_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            name_elem.children.push(NameChild::Ptr(Box::new(ptr)));
                        }
                        "geogName" => {
                            let geog = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            name_elem.children.push(NameChild::GeogName(Box::new(geog)));
                        }
                        "identifier" => {
                            let ident = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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

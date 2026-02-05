//! Miscellaneous deserializers for remaining MEI elements.
//!
//! This module contains deserializers for:
//! - Work and WorkList elements and their children
//! - RevisionDesc, Change, ChangeDesc elements
//! - Expression, ExpressionList, ComponentList, RelationList
//! - Various supporting elements (Dedication, Creation, History, etc.)

use super::header::parse_annot_from_event;
use super::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, extract_attr,
    from_attr_string, parse_bibl_from_event, parse_bibl_struct_from_event, parse_clef_from_event,
    parse_date_from_event, parse_deprecated_creator_from_event, parse_head_from_event,
    parse_identifier_from_event, parse_label_from_event, parse_lb_from_event, parse_p_from_event,
    parse_rend_from_event, parse_resp_stmt_from_event, parse_title_from_event,
};
use crate::deserializer::MixedContent;
use std::io::BufRead;
use tusk_model::elements::{
    Audience, BiblList, BiblListChild, CastGrp, CastGrpChild, CastItem, CastItemChild, CastList,
    CastListChild, Change, ChangeChild, ChangeDesc, ChangeDescChild, Classification,
    ClassificationChild, ComponentList, ComponentListChild, ContentItem, ContentItemChild,
    Contents, ContentsChild, Context, Creation, CreationChild, Creator, Dedication, Edition,
    EditionStmt, EditionStmtChild, Event, EventChild, EventList, EventListChild, Expression,
    ExpressionChild, ExpressionList, ExpressionListChild, ExtMeta, Extent, History, HistoryChild,
    Incip, IncipChild, IncipCode, IncipCodeChild, IncipText, IncipTextChild, Key, LangUsage,
    LangUsageChild, Language, Lg, Manifestation, ManifestationChild, ManifestationList,
    ManifestationListChild, Mensuration, Meter, NotesStmt, NotesStmtChild, Num, NumChild,
    OtherChar, PerfDuration, PerfMedium, PerfMediumChild, PerfRes, PerfResChild, PerfResList,
    PerfResListChild, PhysDesc, PhysDescChild, PlateNum, PlateNumChild, RelationList,
    RelationListChild, RevisionDesc, RevisionDescChild, RoleDesc, RoleDescChild, Score,
    ScoreFormat, SeriesStmt, SeriesStmtChild, Tempo, TempoChild, Term, TermChild, TermList,
    TermListChild, Work, WorkChild, WorkList, WorkListChild,
};

// ============================================================================
// Work child elements - parse functions
// ============================================================================

/// Parse a `<dedication>` element from within another element.
fn parse_dedication_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Dedication> {
    let mut dedication = Dedication::default();

    // Extract attributes
    dedication.common.extract_attributes(&mut attrs)?;
    dedication.bibl.extract_attributes(&mut attrs)?;
    dedication.facsimile.extract_attributes(&mut attrs)?;
    dedication.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("dedication")? {
            if !text.trim().is_empty() {
                dedication
                    .children
                    .push(tusk_model::elements::DedicationChild::Text(text));
            }
        }
    }

    Ok(dedication)
}

/// Parse a `<creation>` element from within another element.
fn parse_creation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Creation> {
    let mut creation = Creation::default();

    // Extract attributes
    creation.common.extract_attributes(&mut attrs)?;
    creation.bibl.extract_attributes(&mut attrs)?;
    creation.datable.extract_attributes(&mut attrs)?;
    creation.facsimile.extract_attributes(&mut attrs)?;
    creation.lang.extract_attributes(&mut attrs)?;

    // Creation has mixed content - text and child elements like date, head, persName, etc.
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("creation")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        creation.children.push(CreationChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                        creation.children.push(CreationChild::Head(Box::new(head)));
                    }
                    "date" => {
                        let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                        creation.children.push(CreationChild::Date(Box::new(date)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(creation)
}

/// Parse a `<history>` element from within another element.
fn parse_history_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<History> {
    let mut history = History::default();

    // Extract attributes
    history.common.extract_attributes(&mut attrs)?;
    history.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // history can contain: head*, p*, eventList*, etc.
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("history")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    history.children.push(HistoryChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    history.children.push(HistoryChild::P(Box::new(p)));
                }
                "eventList" => {
                    let event_list = parse_event_list_from_event(reader, child_attrs, child_empty)?;
                    history
                        .children
                        .push(HistoryChild::EventList(Box::new(event_list)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(history)
}

/// Parse an `<eventList>` element from within another element.
fn parse_event_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EventList> {
    let mut event_list = EventList::default();

    // Extract attributes
    event_list.common.extract_attributes(&mut attrs)?;
    event_list.bibl.extract_attributes(&mut attrs)?;
    event_list.facsimile.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // eventList can contain: head*, (event | eventList | biblList | date | corpName | geogName | name | persName | address)*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("eventList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::Head(Box::new(head)));
                }
                "event" => {
                    let event = parse_event_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::Event(Box::new(event)));
                }
                "eventList" => {
                    let nested = parse_event_list_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::EventList(Box::new(nested)));
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::BiblList(Box::new(bibl_list)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::Date(Box::new(date)));
                }
                "corpName" => {
                    let corp_name = super::header::parse_corp_name_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    event_list
                        .children
                        .push(EventListChild::CorpName(Box::new(corp_name)));
                }
                "geogName" => {
                    let geog_name = super::header::parse_geog_name_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    event_list
                        .children
                        .push(EventListChild::GeogName(Box::new(geog_name)));
                }
                "name" => {
                    let name_elem =
                        super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::Name(Box::new(name_elem)));
                }
                "persName" => {
                    let pers_name = super::header::parse_pers_name_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    event_list
                        .children
                        .push(EventListChild::PersName(Box::new(pers_name)));
                }
                "address" => {
                    let address =
                        super::header::parse_address_from_event(reader, child_attrs, child_empty)?;
                    event_list
                        .children
                        .push(EventListChild::Address(Box::new(address)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(event_list)
}

/// Parse an `<event>` element from within another element.
fn parse_event_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Event> {
    let mut event = Event::default();

    // Extract attributes
    event.common.extract_attributes(&mut attrs)?;
    event.bibl.extract_attributes(&mut attrs)?;
    event.calendared.extract_attributes(&mut attrs)?;
    event.datable.extract_attributes(&mut attrs)?;
    event.edit.extract_attributes(&mut attrs)?;
    event.facsimile.extract_attributes(&mut attrs)?;
    event.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // event can contain: head*, (p | table | list | castList | biblList | date | desc | eventList | address | corpName | geogName | name | persName)*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("event")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::P(Box::new(p)));
                }
                "table" => {
                    let table =
                        super::text::parse_table_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::Table(Box::new(table)));
                }
                "list" => {
                    let list =
                        super::text::parse_list_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::List(Box::new(list)));
                }
                "castList" => {
                    // TODO: implement parse_cast_list_from_event
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    event
                        .children
                        .push(EventChild::BiblList(Box::new(bibl_list)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::Date(Box::new(date)));
                }
                "desc" => {
                    let desc =
                        super::header::parse_desc_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::Desc(Box::new(desc)));
                }
                "eventList" => {
                    let event_list = parse_event_list_from_event(reader, child_attrs, child_empty)?;
                    event
                        .children
                        .push(EventChild::EventList(Box::new(event_list)));
                }
                "address" => {
                    let address =
                        super::header::parse_address_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::Address(Box::new(address)));
                }
                "corpName" => {
                    let corp_name = super::header::parse_corp_name_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    event
                        .children
                        .push(EventChild::CorpName(Box::new(corp_name)));
                }
                "geogName" => {
                    let geog_name = super::header::parse_geog_name_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    event
                        .children
                        .push(EventChild::GeogName(Box::new(geog_name)));
                }
                "name" => {
                    let name_elem =
                        super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
                    event.children.push(EventChild::Name(Box::new(name_elem)));
                }
                "persName" => {
                    let pers_name = super::header::parse_pers_name_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    event
                        .children
                        .push(EventChild::PersName(Box::new(pers_name)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(event)
}

/// Parse a `<langUsage>` element from within another element.
fn parse_lang_usage_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LangUsage> {
    let mut lang_usage = LangUsage::default();

    // Extract attributes
    lang_usage.common.extract_attributes(&mut attrs)?;
    lang_usage.bibl.extract_attributes(&mut attrs)?;
    lang_usage.data_pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // langUsage can contain: head*, language+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("langUsage")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    lang_usage
                        .children
                        .push(LangUsageChild::Head(Box::new(head)));
                }
                "language" => {
                    let language = parse_language_from_event(reader, child_attrs, child_empty)?;
                    lang_usage
                        .children
                        .push(LangUsageChild::Language(Box::new(language)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(lang_usage)
}

/// Parse a `<language>` element from within another element.
fn parse_language_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Language> {
    let mut language = Language::default();

    // Extract attributes
    language.common.extract_attributes(&mut attrs)?;
    language.authorized.extract_attributes(&mut attrs)?;
    language.bibl.extract_attributes(&mut attrs)?;
    language.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("language")? {
            if !text.trim().is_empty() {
                language
                    .children
                    .push(tusk_model::elements::LanguageChild::Text(text));
            }
        }
    }

    Ok(language)
}

/// Parse a `<perfMedium>` element from within another element.
fn parse_perf_medium_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfMedium> {
    let mut perf_medium = PerfMedium::default();

    // Extract attributes
    perf_medium.common.extract_attributes(&mut attrs)?;
    perf_medium.authorized.extract_attributes(&mut attrs)?;
    perf_medium.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // perfMedium can contain: head*, annot*, castList*, perfResList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("perfMedium")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    perf_medium
                        .children
                        .push(PerfMediumChild::Head(Box::new(head)));
                }
                "annot" => {
                    let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                    perf_medium
                        .children
                        .push(PerfMediumChild::Annot(Box::new(annot)));
                }
                "perfResList" => {
                    let perf_res_list =
                        parse_perf_res_list_from_event(reader, child_attrs, child_empty)?;
                    perf_medium
                        .children
                        .push(PerfMediumChild::PerfResList(Box::new(perf_res_list)));
                }
                "castList" => {
                    let cast_list = parse_cast_list_from_event(reader, child_attrs, child_empty)?;
                    perf_medium
                        .children
                        .push(PerfMediumChild::CastList(Box::new(cast_list)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(perf_medium)
}

/// Parse a `<castList>` element from within another element.
fn parse_cast_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CastList> {
    let mut cast_list = CastList::default();

    // Extract attributes
    cast_list.common.extract_attributes(&mut attrs)?;
    cast_list.bibl.extract_attributes(&mut attrs)?;
    cast_list.facsimile.extract_attributes(&mut attrs)?;
    cast_list.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // castList can contain: head*, castItem*, castGrp*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("castList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    cast_list.children.push(CastListChild::Head(Box::new(head)));
                }
                "castItem" => {
                    let cast_item = parse_cast_item_from_event(reader, child_attrs, child_empty)?;
                    cast_list
                        .children
                        .push(CastListChild::CastItem(Box::new(cast_item)));
                }
                "castGrp" => {
                    let cast_grp = parse_cast_grp_from_event(reader, child_attrs, child_empty)?;
                    cast_list
                        .children
                        .push(CastListChild::CastGrp(Box::new(cast_grp)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(cast_list)
}

/// Parse a `<castItem>` element from within another element.
fn parse_cast_item_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CastItem> {
    let mut cast_item = CastItem::default();

    // Extract attributes
    cast_item.common.extract_attributes(&mut attrs)?;
    cast_item.bibl.extract_attributes(&mut attrs)?;
    cast_item.facsimile.extract_attributes(&mut attrs)?;
    cast_item.lang.extract_attributes(&mut attrs)?;

    // castItem has mixed content - text and child elements (actor, roleDesc, perfRes, role)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("castItem")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        cast_item.children.push(CastItemChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "perfRes" => {
                            let perf_res =
                                parse_perf_res_from_event(reader, child_attrs, child_empty)?;
                            cast_item
                                .children
                                .push(CastItemChild::PerfRes(Box::new(perf_res)));
                        }
                        "roleDesc" => {
                            let role_desc =
                                parse_role_desc_from_event(reader, child_attrs, child_empty)?;
                            cast_item
                                .children
                                .push(CastItemChild::RoleDesc(Box::new(role_desc)));
                        }
                        // actor and role elements are skipped for now - can be added later
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

    Ok(cast_item)
}

/// Parse a `<castGrp>` element from within another element.
fn parse_cast_grp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CastGrp> {
    let mut cast_grp = CastGrp::default();

    // Extract attributes
    cast_grp.common.extract_attributes(&mut attrs)?;
    cast_grp.facsimile.extract_attributes(&mut attrs)?;
    cast_grp.lang.extract_attributes(&mut attrs)?;

    // castGrp can contain: castGrp* (recursive), roleDesc*, castItem*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("castGrp")?
        {
            match name.as_str() {
                "castGrp" => {
                    // Recursive call for nested castGrp
                    let nested = parse_cast_grp_from_event(reader, child_attrs, child_empty)?;
                    cast_grp
                        .children
                        .push(CastGrpChild::CastGrp(Box::new(nested)));
                }
                "roleDesc" => {
                    let role_desc = parse_role_desc_from_event(reader, child_attrs, child_empty)?;
                    cast_grp
                        .children
                        .push(CastGrpChild::RoleDesc(Box::new(role_desc)));
                }
                "castItem" => {
                    let cast_item = parse_cast_item_from_event(reader, child_attrs, child_empty)?;
                    cast_grp
                        .children
                        .push(CastGrpChild::CastItem(Box::new(cast_item)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(cast_grp)
}

/// Parse a `<roleDesc>` element from within another element.
fn parse_role_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RoleDesc> {
    let mut role_desc = RoleDesc::default();

    // Extract attributes
    role_desc.common.extract_attributes(&mut attrs)?;
    role_desc.facsimile.extract_attributes(&mut attrs)?;
    role_desc.lang.extract_attributes(&mut attrs)?;

    // roleDesc has mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("roleDesc")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        role_desc.children.push(RoleDescChild::Text(text));
                    }
                }
                MixedContent::Element(name, _child_attrs, child_empty) => {
                    // Skip unknown child elements for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(role_desc)
}

/// Parse a `<perfResList>` element from within another element.
fn parse_perf_res_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfResList> {
    let mut perf_res_list = PerfResList::default();

    // Extract attributes
    perf_res_list.common.extract_attributes(&mut attrs)?;
    perf_res_list.authorized.extract_attributes(&mut attrs)?;
    perf_res_list.bibl.extract_attributes(&mut attrs)?;
    perf_res_list.edit.extract_attributes(&mut attrs)?;
    perf_res_list.lang.extract_attributes(&mut attrs)?;
    perf_res_list
        .perf_res_basic
        .extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // perfResList can contain: annot*, perfRes*, head*, perfResList* (recursive)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("perfResList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    perf_res_list
                        .children
                        .push(PerfResListChild::Head(Box::new(head)));
                }
                "annot" => {
                    let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                    perf_res_list
                        .children
                        .push(PerfResListChild::Annot(Box::new(annot)));
                }
                "perfRes" => {
                    let perf_res = parse_perf_res_from_event(reader, child_attrs, child_empty)?;
                    perf_res_list
                        .children
                        .push(PerfResListChild::PerfRes(Box::new(perf_res)));
                }
                "perfResList" => {
                    // Recursive call for nested perfResList
                    let nested = parse_perf_res_list_from_event(reader, child_attrs, child_empty)?;
                    perf_res_list
                        .children
                        .push(PerfResListChild::PerfResList(Box::new(nested)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(perf_res_list)
}

/// Parse a `<perfRes>` element from within another element.
fn parse_perf_res_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfRes> {
    let mut perf_res = PerfRes::default();

    // Extract attributes
    perf_res.common.extract_attributes(&mut attrs)?;
    perf_res.authorized.extract_attributes(&mut attrs)?;
    perf_res.bibl.extract_attributes(&mut attrs)?;
    perf_res.edit.extract_attributes(&mut attrs)?;
    perf_res.lang.extract_attributes(&mut attrs)?;
    perf_res.perf_res.extract_attributes(&mut attrs)?;

    // perfRes has mixed content - text and many possible child elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("perfRes")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        perf_res.children.push(PerfResChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "perfRes" => {
                            // Recursive call for nested perfRes
                            let nested =
                                parse_perf_res_from_event(reader, child_attrs, child_empty)?;
                            perf_res
                                .children
                                .push(PerfResChild::PerfRes(Box::new(nested)));
                        }
                        // For now, skip other complex child elements
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

    Ok(perf_res)
}

/// Parse a `<perfDuration>` element from within another element.
fn parse_perf_duration_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfDuration> {
    let mut perf_duration = PerfDuration::default();

    // Extract attributes
    perf_duration.common.extract_attributes(&mut attrs)?;
    perf_duration.bibl.extract_attributes(&mut attrs)?;
    perf_duration.facsimile.extract_attributes(&mut attrs)?;
    perf_duration.lang.extract_attributes(&mut attrs)?;

    // perfDuration has no children in the model
    if !is_empty {
        reader.skip_to_end("perfDuration")?;
    }

    Ok(perf_duration)
}

/// Parse an `<extent>` element from within another element.
pub(crate) fn parse_extent_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Extent> {
    let mut extent = Extent::default();

    // Extract attributes
    extent.common.extract_attributes(&mut attrs)?;
    extent.bibl.extract_attributes(&mut attrs)?;
    extent.facsimile.extract_attributes(&mut attrs)?;
    extent.lang.extract_attributes(&mut attrs)?;
    extent.quantity.extract_attributes(&mut attrs)?;

    // extent can have text and various child elements - for now just collect text
    if !is_empty {
        reader.skip_to_end("extent")?;
    }

    Ok(extent)
}

/// Parse a `<scoreFormat>` element from within another element.
fn parse_score_format_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ScoreFormat> {
    let mut score_format = ScoreFormat::default();

    // Extract attributes
    score_format.common.extract_attributes(&mut attrs)?;
    score_format.authorized.extract_attributes(&mut attrs)?;
    score_format.bibl.extract_attributes(&mut attrs)?;
    score_format.lang.extract_attributes(&mut attrs)?;

    // scoreFormat has no children
    if !is_empty {
        reader.skip_to_end("scoreFormat")?;
    }

    Ok(score_format)
}

/// Parse an `<audience>` element from within another element.
fn parse_audience_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Audience> {
    let mut audience = Audience::default();

    // Extract attributes
    audience.common.extract_attributes(&mut attrs)?;
    audience.authorized.extract_attributes(&mut attrs)?;
    audience.bibl.extract_attributes(&mut attrs)?;
    audience.lang.extract_attributes(&mut attrs)?;

    // audience has no children in the model
    if !is_empty {
        reader.skip_to_end("audience")?;
    }

    Ok(audience)
}

/// Parse a `<contents>` element from within another element.
fn parse_contents_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Contents> {
    let mut contents = Contents::default();

    // Extract attributes
    contents.common.extract_attributes(&mut attrs)?;
    contents.bibl.extract_attributes(&mut attrs)?;
    contents.pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // contents can contain: head*, p*, contentItem*, label*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("contents")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    contents.children.push(ContentsChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    contents.children.push(ContentsChild::P(Box::new(p)));
                }
                "contentItem" => {
                    let content_item =
                        parse_content_item_from_event(reader, child_attrs, child_empty)?;
                    contents
                        .children
                        .push(ContentsChild::ContentItem(Box::new(content_item)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    contents
                        .children
                        .push(ContentsChild::Label(Box::new(label)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(contents)
}

/// Parse a `<contentItem>` element from within another element.
///
/// contentItem has mixed content - text and various child elements like
/// title, persName, corpName, rend, etc.
fn parse_content_item_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ContentItem> {
    let mut content_item = ContentItem::default();

    // Extract attributes
    content_item.common.extract_attributes(&mut attrs)?;
    content_item.bibl.extract_attributes(&mut attrs)?;
    content_item.lang.extract_attributes(&mut attrs)?;

    // contentItem has mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("contentItem")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        content_item.children.push(ContentItemChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            content_item
                                .children
                                .push(ContentItemChild::Title(Box::new(title)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            content_item
                                .children
                                .push(ContentItemChild::Bibl(Box::new(bibl)));
                        }
                        // Skip other complex children for now - they can be added as needed
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

    Ok(content_item)
}

/// Parse a `<context>` element from within another element.
fn parse_context_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Context> {
    let mut context = Context::default();

    // Extract attributes
    context.common.extract_attributes(&mut attrs)?;
    context.authorized.extract_attributes(&mut attrs)?;
    context.bibl.extract_attributes(&mut attrs)?;
    context.lang.extract_attributes(&mut attrs)?;

    // context has no children in the model
    if !is_empty {
        reader.skip_to_end("context")?;
    }

    Ok(context)
}

/// Parse a `<biblList>` element from within another element.
fn parse_bibl_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<BiblList> {
    let mut bibl_list = BiblList::default();

    // Extract attributes
    bibl_list.common.extract_attributes(&mut attrs)?;
    bibl_list.bibl.extract_attributes(&mut attrs)?;
    bibl_list.facsimile.extract_attributes(&mut attrs)?;
    bibl_list.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // biblList can contain: head*, bibl*, biblStruct*, label*, biblList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("biblList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    bibl_list.children.push(BiblListChild::Head(Box::new(head)));
                }
                "bibl" => {
                    let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    bibl_list.children.push(BiblListChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    bibl_list
                        .children
                        .push(BiblListChild::BiblStruct(Box::new(bibl_struct)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    bibl_list
                        .children
                        .push(BiblListChild::Label(Box::new(label)));
                }
                "biblList" => {
                    let nested_bibl_list =
                        parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    bibl_list
                        .children
                        .push(BiblListChild::BiblList(Box::new(nested_bibl_list)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(bibl_list)
}

/// Parse a `<notesStmt>` element from within another element.
pub(crate) fn parse_notes_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<NotesStmt> {
    let mut notes_stmt = NotesStmt::default();

    // Extract attributes
    notes_stmt.common.extract_attributes(&mut attrs)?;
    notes_stmt.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // notesStmt can contain: head*, annot*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("notesStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    notes_stmt
                        .children
                        .push(NotesStmtChild::Head(Box::new(head)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(notes_stmt)
}

/// Parse an `<edition>` element from within another element.
pub(crate) fn parse_edition_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Edition> {
    let mut edition = Edition::default();

    // Extract attributes
    edition.common.extract_attributes(&mut attrs)?;
    edition.bibl.extract_attributes(&mut attrs)?;
    edition.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // edition can contain text and various child elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("edition")?
        {
            match name.as_str() {
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    edition
                        .children
                        .push(tusk_model::elements::EditionChild::RespStmt(Box::new(
                            resp_stmt,
                        )));
                }
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    edition
                        .children
                        .push(tusk_model::elements::EditionChild::Title(Box::new(title)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    edition
                        .children
                        .push(tusk_model::elements::EditionChild::Date(Box::new(date)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(edition)
}

/// Parse an `<editionStmt>` element from within another element.
pub(crate) fn parse_edition_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EditionStmt> {
    use super::{
        parse_contributor_from_event, parse_creator_from_event, parse_editor_from_event,
        parse_funder_from_event, parse_sponsor_from_event,
    };

    let mut edition_stmt = EditionStmt::default();

    // Extract attributes
    edition_stmt.common.extract_attributes(&mut attrs)?;
    edition_stmt.bibl.extract_attributes(&mut attrs)?;
    edition_stmt.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // editionStmt can contain: head*, edition*, respStmt*, model.respLikePart
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("editionStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Head(Box::new(head)));
                }
                "edition" => {
                    let edition = parse_edition_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Edition(Box::new(edition)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "creator" => {
                    let creator = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Creator(Box::new(creator)));
                }
                "editor" => {
                    let editor = parse_editor_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Editor(Box::new(editor)));
                }
                "funder" => {
                    let funder = parse_funder_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Funder(Box::new(funder)));
                }
                "sponsor" => {
                    let sponsor = parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Sponsor(Box::new(sponsor)));
                }
                "contributor" => {
                    let contributor =
                        parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    edition_stmt
                        .children
                        .push(EditionStmtChild::Contributor(Box::new(contributor)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(edition_stmt)
}

/// Parse a `<seriesStmt>` element from within another element.
pub(crate) fn parse_series_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SeriesStmt> {
    use super::{
        parse_contributor_from_event, parse_creator_from_event, parse_editor_from_event,
        parse_funder_from_event, parse_sponsor_from_event,
    };

    let mut series_stmt = SeriesStmt::default();

    // Extract attributes
    series_stmt.common.extract_attributes(&mut attrs)?;
    series_stmt.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // seriesStmt can contain: head*, title*, respStmt*, biblScope*, identifier*, contents*,
    // seriesStmt*, model.respLikePart
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("seriesStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Head(Box::new(head)));
                }
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Title(Box::new(title)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Identifier(Box::new(identifier)));
                }
                "contents" => {
                    let contents = parse_contents_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Contents(Box::new(contents)));
                }
                "seriesStmt" => {
                    // Recursive - nested seriesStmt
                    let nested = parse_series_stmt_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::SeriesStmt(Box::new(nested)));
                }
                "creator" => {
                    let creator = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Creator(Box::new(creator)));
                }
                "editor" => {
                    let editor = parse_editor_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Editor(Box::new(editor)));
                }
                "funder" => {
                    let funder = parse_funder_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Funder(Box::new(funder)));
                }
                "sponsor" => {
                    let sponsor = parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Sponsor(Box::new(sponsor)));
                }
                "contributor" => {
                    let contributor =
                        parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    series_stmt
                        .children
                        .push(SeriesStmtChild::Contributor(Box::new(contributor)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(series_stmt)
}

/// Parse a `<term>` element from within another element.
///
/// Term is a mixed-content element that can contain text and child elements.
/// For simplicity, we primarily support text content with the `@class` attribute,
/// which is the common usage pattern in classification contexts.
fn parse_term_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Term> {
    let mut term = Term::default();

    // Extract attributes
    // Term has: att.common, att.bibl, att.dataPointing, att.lang
    term.common.extract_attributes(&mut attrs)?;
    term.bibl.extract_attributes(&mut attrs)?;
    term.data_pointing.extract_attributes(&mut attrs)?;
    term.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("term")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content
                    if !text.trim().is_empty() {
                        term.children.push(TermChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            term.children.push(TermChild::Ref(Box::new(ref_elem)));
                        }
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            term.children.push(TermChild::Rend(Box::new(rend)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            term.children.push(TermChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            term.children.push(TermChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            term.children.push(TermChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            term.children.push(TermChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            term.children
                                .push(TermChild::Identifier(Box::new(identifier)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            term.children.push(TermChild::Title(Box::new(title)));
                        }
                        "bibl" => {
                            let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            term.children.push(TermChild::Bibl(Box::new(bibl)));
                        }
                        "biblStruct" => {
                            let bibl_struct =
                                parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                            term.children
                                .push(TermChild::BiblStruct(Box::new(bibl_struct)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            term.children.push(TermChild::Lb(Box::new(lb)));
                        }
                        // Nested term
                        "term" => {
                            let nested_term =
                                parse_term_from_event(reader, child_attrs, child_empty)?;
                            term.children.push(TermChild::Term(Box::new(nested_term)));
                        }
                        // Skip other child elements for now
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

    Ok(term)
}

/// Parse a `<termList>` element from within another element.
fn parse_term_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TermList> {
    let mut term_list = TermList::default();

    // Extract attributes
    // TermList has: att.common, att.bibl, att.pointing
    term_list.common.extract_attributes(&mut attrs)?;
    term_list.bibl.extract_attributes(&mut attrs)?;
    term_list.pointing.extract_attributes(&mut attrs)?;

    // Parse children if not empty
    // termList can contain: head*, label*, term*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("termList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    term_list.children.push(TermListChild::Head(Box::new(head)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    term_list
                        .children
                        .push(TermListChild::Label(Box::new(label)));
                }
                "term" => {
                    let term = parse_term_from_event(reader, child_attrs, child_empty)?;
                    term_list.children.push(TermListChild::Term(Box::new(term)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(term_list)
}

/// Parse a `<classification>` element from within another element.
fn parse_classification_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Classification> {
    let mut classification = Classification::default();

    // Extract attributes
    classification.common.extract_attributes(&mut attrs)?;
    classification.bibl.extract_attributes(&mut attrs)?;
    classification
        .data_pointing
        .extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // classification can contain: head*, termList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("classification")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    classification
                        .children
                        .push(ClassificationChild::Head(Box::new(head)));
                }
                "termList" => {
                    let term_list = parse_term_list_from_event(reader, child_attrs, child_empty)?;
                    classification
                        .children
                        .push(ClassificationChild::TermList(Box::new(term_list)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(classification)
}

impl MeiDeserialize for Term {
    fn element_name() -> &'static str {
        "term"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_term_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TermList {
    fn element_name() -> &'static str {
        "termList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_term_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Classification {
    fn element_name() -> &'static str {
        "classification"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_classification_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<expression>` element from within another element.
pub(crate) fn parse_expression_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Expression> {
    let mut expression = Expression::default();

    // Extract attributes
    // expression has: att.common, att.authorized, att.bibl, att.dataPointing
    expression.common.extract_attributes(&mut attrs)?;
    expression.authorized.extract_attributes(&mut attrs)?;
    expression.bibl.extract_attributes(&mut attrs)?;
    expression.data_pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // expression can contain: head*, identifier*, title+, respStmt?, dedication?,
    // key*, tempo*, meter*, mensuration*, incip*, otherChar*, creation?,
    // history?, langUsage?, perfMedium?, perfDuration?, extent*, scoreFormat?,
    // contents?, context?, biblList*, notesStmt?, classification?, componentList?,
    // relationList*, extMeta*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("expression")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Head(Box::new(head)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Identifier(Box::new(identifier)));
                }
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Title(Box::new(title)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::RespStmt(Box::new(resp_stmt)));
                }
                "dedication" => {
                    let dedication = parse_dedication_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Dedication(Box::new(dedication)));
                }
                "key" => {
                    let key = parse_key_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Key(Box::new(key)));
                }
                "tempo" => {
                    let tempo = parse_tempo_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Tempo(Box::new(tempo)));
                }
                "meter" => {
                    let meter = parse_meter_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Meter(Box::new(meter)));
                }
                "mensuration" => {
                    let mensuration =
                        parse_mensuration_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Mensuration(Box::new(mensuration)));
                }
                "incip" => {
                    let incip = parse_incip_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Incip(Box::new(incip)));
                }
                "otherChar" => {
                    let other_char = parse_other_char_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::OtherChar(Box::new(other_char)));
                }
                "creation" => {
                    let creation = parse_creation_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Creation(Box::new(creation)));
                }
                "history" => {
                    let history = parse_history_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::History(Box::new(history)));
                }
                "langUsage" => {
                    let lang_usage = parse_lang_usage_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::LangUsage(Box::new(lang_usage)));
                }
                "perfMedium" => {
                    let perf_medium =
                        parse_perf_medium_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::PerfMedium(Box::new(perf_medium)));
                }
                "perfDuration" => {
                    let perf_duration =
                        parse_perf_duration_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::PerfDuration(Box::new(perf_duration)));
                }
                "extent" => {
                    let extent = parse_extent_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Extent(Box::new(extent)));
                }
                "scoreFormat" => {
                    let score_format =
                        parse_score_format_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::ScoreFormat(Box::new(score_format)));
                }
                "contents" => {
                    let contents = parse_contents_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Contents(Box::new(contents)));
                }
                "context" => {
                    let context = parse_context_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Context(Box::new(context)));
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::BiblList(Box::new(bibl_list)));
                }
                "notesStmt" => {
                    let notes_stmt = parse_notes_stmt_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::NotesStmt(Box::new(notes_stmt)));
                }
                "classification" => {
                    let classification =
                        parse_classification_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Classification(Box::new(classification)));
                }
                "componentList" => {
                    let component_list =
                        parse_component_list_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::ComponentList(Box::new(component_list)));
                }
                "relationList" => {
                    let relation_list =
                        parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::RelationList(Box::new(relation_list)));
                }
                "extMeta" => {
                    let ext_meta = parse_ext_meta_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::ExtMeta(Box::new(ext_meta)));
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

    Ok(expression)
}

impl MeiDeserialize for Expression {
    fn element_name() -> &'static str {
        "expression"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_expression_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<expressionList>` element from within another element.
pub(crate) fn parse_expression_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ExpressionList> {
    let mut expression_list = ExpressionList::default();

    // Extract attributes
    expression_list.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // expressionList can contain: head*, expression*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("expressionList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    expression_list
                        .children
                        .push(ExpressionListChild::Head(Box::new(head)));
                }
                "expression" => {
                    let expression = parse_expression_from_event(reader, child_attrs, child_empty)?;
                    expression_list
                        .children
                        .push(ExpressionListChild::Expression(Box::new(expression)));
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

    Ok(expression_list)
}

impl MeiDeserialize for ExpressionList {
    fn element_name() -> &'static str {
        "expressionList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_expression_list_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<componentList>` element from within another element.
fn parse_component_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ComponentList> {
    let mut component_list = ComponentList::default();

    // Extract attributes
    component_list.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // componentList can contain: head*, work*, expression*, manifestation*, item*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("componentList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    component_list
                        .children
                        .push(ComponentListChild::Head(Box::new(head)));
                }
                "work" => {
                    let work = parse_work_from_event(reader, child_attrs, child_empty)?;
                    component_list
                        .children
                        .push(ComponentListChild::Work(Box::new(work)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(component_list)
}

/// Parse a `<relationList>` element from within another element.
fn parse_relation_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RelationList> {
    let mut relation_list = RelationList::default();

    // Extract attributes
    relation_list.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // relationList can contain: head*, relation*, relationList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("relationList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    relation_list
                        .children
                        .push(RelationListChild::Head(Box::new(head)));
                }
                "relationList" => {
                    let nested = parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    relation_list
                        .children
                        .push(RelationListChild::RelationList(Box::new(nested)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(relation_list)
}

/// Parse an `<extMeta>` element from within another element.
fn parse_ext_meta_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ExtMeta> {
    let mut ext_meta = ExtMeta::default();

    // Extract attributes
    ext_meta.common.extract_attributes(&mut attrs)?;
    ext_meta.bibl.extract_attributes(&mut attrs)?;
    ext_meta.whitespace.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("extMeta")? {
            if !text.trim().is_empty() {
                ext_meta
                    .children
                    .push(tusk_model::elements::ExtMetaChild::Text(text));
            }
        }
    }

    Ok(ext_meta)
}

/// Parse an `<otherChar>` element from within another element.
fn parse_other_char_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<OtherChar> {
    let mut other_char = OtherChar::default();

    // Extract attributes
    other_char.common.extract_attributes(&mut attrs)?;
    other_char.bibl.extract_attributes(&mut attrs)?;
    other_char.lang.extract_attributes(&mut attrs)?;

    // otherChar has no children in the model
    if !is_empty {
        reader.skip_to_end("otherChar")?;
    }

    Ok(other_char)
}

/// Parse a `<key>` element from within another element.
fn parse_key_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Key> {
    let mut key = Key::default();

    // Extract attributes
    key.common.extract_attributes(&mut attrs)?;
    key.accidental.extract_attributes(&mut attrs)?;
    key.bibl.extract_attributes(&mut attrs)?;
    key.key_mode.extract_attributes(&mut attrs)?;
    key.pitch.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("key")? {
            if !text.trim().is_empty() {
                key.children
                    .push(tusk_model::elements::KeyChild::Text(text));
            }
        }
    }

    Ok(key)
}

/// Parse a `<tempo>` element from within another element.
fn parse_tempo_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Tempo> {
    let mut tempo = Tempo::default();

    // Extract attributes
    tempo.common.extract_attributes(&mut attrs)?;
    tempo.bibl.extract_attributes(&mut attrs)?;
    tempo.facsimile.extract_attributes(&mut attrs)?;
    tempo.lang.extract_attributes(&mut attrs)?;
    tempo.tempo_anl.extract_attributes(&mut attrs)?;
    tempo.tempo_ges.extract_attributes(&mut attrs)?;
    tempo.tempo_log.extract_attributes(&mut attrs)?;
    tempo.tempo_vis.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("tempo")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        tempo.children.push(TempoChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            tempo.children.push(TempoChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            tempo.children.push(TempoChild::Lb(Box::new(lb)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            tempo.children.push(TempoChild::Ref(Box::new(ref_elem)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            tempo
                                .children
                                .push(TempoChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            tempo
                                .children
                                .push(TempoChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            tempo.children.push(TempoChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            tempo.children.push(TempoChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                            tempo.children.push(TempoChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            tempo
                                .children
                                .push(TempoChild::Identifier(Box::new(identifier)));
                        }
                        // Skip unknown child elements
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

    Ok(tempo)
}

/// Parse a `<meter>` element from within another element.
fn parse_meter_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Meter> {
    let mut meter = Meter::default();

    // Extract attributes
    meter.common.extract_attributes(&mut attrs)?;
    meter.bibl.extract_attributes(&mut attrs)?;
    meter.lang.extract_attributes(&mut attrs)?;
    meter.meter_sig_log.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("meter")? {
            if !text.trim().is_empty() {
                meter
                    .children
                    .push(tusk_model::elements::MeterChild::Text(text));
            }
        }
    }

    Ok(meter)
}

/// Parse a `<mensuration>` element from within another element.
fn parse_mensuration_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Mensuration> {
    let mut mensuration = Mensuration::default();

    // Extract attributes
    mensuration.common.extract_attributes(&mut attrs)?;
    mensuration.bibl.extract_attributes(&mut attrs)?;
    mensuration.lang.extract_attributes(&mut attrs)?;
    mensuration.mensur_log.extract_attributes(&mut attrs)?;
    mensuration.mensur_vis.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("mensuration")? {
            if !text.trim().is_empty() {
                mensuration
                    .children
                    .push(tusk_model::elements::MensurationChild::Text(text));
            }
        }
    }

    Ok(mensuration)
}

/// Parse an `<incipCode>` element from within another element.
fn parse_incip_code_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<IncipCode> {
    let mut incip_code = IncipCode::default();

    // Extract attributes
    incip_code.common.extract_attributes(&mut attrs)?;
    incip_code.bibl.extract_attributes(&mut attrs)?;
    incip_code.internet_media.extract_attributes(&mut attrs)?;
    incip_code.pointing.extract_attributes(&mut attrs)?;
    incip_code.whitespace.extract_attributes(&mut attrs)?;

    // Element-local attribute: @form
    if let Some(form_val) = attrs.remove("form") {
        incip_code.form = Some(form_val);
    }

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("incipCode")? {
            if !text.is_empty() {
                incip_code.children.push(IncipCodeChild::Text(text));
            }
        }
    }

    Ok(incip_code)
}

/// Parse an `<incipText>` element from within another element.
fn parse_incip_text_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<IncipText> {
    let mut incip_text = IncipText::default();

    // Extract attributes
    incip_text.common.extract_attributes(&mut attrs)?;
    incip_text.bibl.extract_attributes(&mut attrs)?;
    incip_text.lang.extract_attributes(&mut attrs)?;
    incip_text.pointing.extract_attributes(&mut attrs)?;
    incip_text.internet_media.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // incipText can contain: p, head, lg
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("incipText")?
        {
            match name.as_str() {
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    incip_text.children.push(IncipTextChild::P(Box::new(p)));
                }
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    incip_text
                        .children
                        .push(IncipTextChild::Head(Box::new(head)));
                }
                "lg" => {
                    let lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                    incip_text.children.push(IncipTextChild::Lg(Box::new(lg)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(incip_text)
}

/// Parse an `<incip>` element from within another element.
fn parse_incip_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Incip> {
    let mut incip = Incip::default();

    // Extract attributes
    incip.common.extract_attributes(&mut attrs)?;
    incip.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // incip can contain many child elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("incip")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Head(Box::new(head)));
                }
                "key" => {
                    let key = parse_key_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Key(Box::new(key)));
                }
                "meter" => {
                    let meter = parse_meter_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Meter(Box::new(meter)));
                }
                "tempo" => {
                    let tempo = parse_tempo_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Tempo(Box::new(tempo)));
                }
                "mensuration" => {
                    let mensuration =
                        parse_mensuration_from_event(reader, child_attrs, child_empty)?;
                    incip
                        .children
                        .push(IncipChild::Mensuration(Box::new(mensuration)));
                }
                "clef" => {
                    let clef = parse_clef_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Clef(Box::new(clef)));
                }
                "incipCode" => {
                    let incip_code = parse_incip_code_from_event(reader, child_attrs, child_empty)?;
                    incip
                        .children
                        .push(IncipChild::IncipCode(Box::new(incip_code)));
                }
                "incipText" => {
                    let incip_text = parse_incip_text_from_event(reader, child_attrs, child_empty)?;
                    incip
                        .children
                        .push(IncipChild::IncipText(Box::new(incip_text)));
                }
                "score" => {
                    let score = Score::from_mei_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Score(Box::new(score)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(incip)
}

// ============================================================================
// WorkList element implementation
// ============================================================================

impl MeiDeserialize for WorkList {
    fn element_name() -> &'static str {
        "workList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_work_list_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<workList>` element from within another element.
pub(crate) fn parse_work_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<WorkList> {
    let mut work_list = WorkList::default();

    // Extract attributes
    work_list.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // workList can contain: head*, work+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("workList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    work_list.children.push(WorkListChild::Head(Box::new(head)));
                }
                "work" => {
                    let work = parse_work_from_event(reader, child_attrs, child_empty)?;
                    work_list.children.push(WorkListChild::Work(Box::new(work)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(work_list)
}

// ============================================================================
// ManifestationList implementation
// ============================================================================

impl MeiDeserialize for ManifestationList {
    fn element_name() -> &'static str {
        "manifestationList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_manifestation_list_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<manifestationList>` element from within another element.
pub(crate) fn parse_manifestation_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ManifestationList> {
    let mut manifestation_list = ManifestationList::default();

    // Extract attributes
    manifestation_list.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // manifestationList can contain: head*, manifestation+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("manifestationList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    manifestation_list
                        .children
                        .push(ManifestationListChild::Head(Box::new(head)));
                }
                "manifestation" => {
                    let manifestation =
                        parse_manifestation_from_event(reader, child_attrs, child_empty)?;
                    manifestation_list
                        .children
                        .push(ManifestationListChild::Manifestation(Box::new(
                            manifestation,
                        )));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(manifestation_list)
}

impl MeiDeserialize for Manifestation {
    fn element_name() -> &'static str {
        "manifestation"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_manifestation_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<manifestation>` element from within another element.
pub(crate) fn parse_manifestation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Manifestation> {
    let mut manifestation = Manifestation::default();

    // Extract attributes
    manifestation.common.extract_attributes(&mut attrs)?;
    manifestation.authorized.extract_attributes(&mut attrs)?;
    manifestation.bibl.extract_attributes(&mut attrs)?;
    manifestation
        .component_type
        .extract_attributes(&mut attrs)?;
    manifestation.data_pointing.extract_attributes(&mut attrs)?;
    manifestation.pointing.extract_attributes(&mut attrs)?;
    manifestation.record_type.extract_attributes(&mut attrs)?;
    manifestation.target_eval.extract_attributes(&mut attrs)?;

    // Extract local attribute: @singleton
    if let Some(val) = attrs.remove("singleton") {
        manifestation.singleton = from_attr_string(&val).ok();
    }

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // manifestation can contain: head*, titleStmt?, editionStmt?, pubStmt?, physDesc?,
    // seriesStmt?, contents?, langUsage?, notesStmt?, classification?, itemList?,
    // componentList?, relationList?, extMeta*
    // Also: identifier*, physLoc?, history?, biblList*, dedication*, creation?,
    // locus*, locusGrp*, availability?
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("manifestation")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::Head(Box::new(head)));
                }
                "titleStmt" => {
                    let title_stmt = super::header::parse_title_stmt_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    manifestation
                        .children
                        .push(ManifestationChild::TitleStmt(Box::new(title_stmt)));
                }
                "editionStmt" => {
                    let edition_stmt =
                        parse_edition_stmt_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::EditionStmt(Box::new(edition_stmt)));
                }
                "pubStmt" => {
                    let pub_stmt =
                        super::header::parse_pub_stmt_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::PubStmt(Box::new(pub_stmt)));
                }
                "physDesc" => {
                    let phys_desc = parse_phys_desc_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::PhysDesc(Box::new(phys_desc)));
                }
                "seriesStmt" => {
                    let series_stmt =
                        parse_series_stmt_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::SeriesStmt(Box::new(series_stmt)));
                }
                "contents" => {
                    let contents = parse_contents_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::Contents(Box::new(contents)));
                }
                "langUsage" => {
                    let lang_usage = parse_lang_usage_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::LangUsage(Box::new(lang_usage)));
                }
                "notesStmt" => {
                    let notes_stmt = parse_notes_stmt_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::NotesStmt(Box::new(notes_stmt)));
                }
                "classification" => {
                    let classification =
                        parse_classification_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::Classification(Box::new(classification)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::Identifier(Box::new(identifier)));
                }
                "history" => {
                    let history = parse_history_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::History(Box::new(history)));
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::BiblList(Box::new(bibl_list)));
                }
                "dedication" => {
                    let dedication = parse_dedication_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::Dedication(Box::new(dedication)));
                }
                "creation" => {
                    let creation = parse_creation_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::Creation(Box::new(creation)));
                }
                "availability" => {
                    let availability = super::header::parse_availability_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    manifestation
                        .children
                        .push(ManifestationChild::Availability(Box::new(availability)));
                }
                "relationList" => {
                    let relation_list =
                        parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    manifestation
                        .children
                        .push(ManifestationChild::RelationList(Box::new(relation_list)));
                }
                // Skipped for now: itemList, componentList, physLoc, locus, locusGrp, extMeta
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(manifestation)
}

// ============================================================================
// PhysDesc implementation
// ============================================================================

impl MeiDeserialize for PhysDesc {
    fn element_name() -> &'static str {
        "physDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_phys_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<physDesc>` element from within another element.
pub(crate) fn parse_phys_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PhysDesc> {
    let mut phys_desc = PhysDesc::default();

    // Extract attributes
    phys_desc.common.extract_attributes(&mut attrs)?;
    phys_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // physDesc can contain many children; for now we handle common ones
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("physDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    phys_desc.children.push(PhysDescChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    phys_desc.children.push(PhysDescChild::P(Box::new(p)));
                }
                "plateNum" => {
                    let plate_num = parse_plate_num_from_event(reader, child_attrs, child_empty)?;
                    phys_desc
                        .children
                        .push(PhysDescChild::PlateNum(Box::new(plate_num)));
                }
                "extent" => {
                    let extent = parse_extent_from_event(reader, child_attrs, child_empty)?;
                    phys_desc
                        .children
                        .push(PhysDescChild::Extent(Box::new(extent)));
                }
                // Other physDesc children are skipped for now
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(phys_desc)
}

impl MeiDeserialize for PlateNum {
    fn element_name() -> &'static str {
        "plateNum"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_plate_num_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<plateNum>` element from within another element.
pub(crate) fn parse_plate_num_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PlateNum> {
    let mut plate_num = PlateNum::default();

    // Extract attributes
    plate_num.common.extract_attributes(&mut attrs)?;
    plate_num.bibl.extract_attributes(&mut attrs)?;
    plate_num.facsimile.extract_attributes(&mut attrs)?;
    plate_num.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read mixed content if not empty
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("plateNum")? {
            match content {
                MixedContent::Text(text) => {
                    plate_num.children.push(PlateNumChild::Text(text));
                }
                MixedContent::Element(name, _child_attrs, child_empty) => {
                    // For now, skip child elements - plateNum typically contains just text
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(plate_num)
}

impl MeiDeserialize for Work {
    fn element_name() -> &'static str {
        "work"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_work_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for History {
    fn element_name() -> &'static str {
        "history"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_history_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for EventList {
    fn element_name() -> &'static str {
        "eventList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_event_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Event {
    fn element_name() -> &'static str {
        "event"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_event_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<work>` element from within another element.
pub(crate) fn parse_work_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Work> {
    let mut work = Work::default();

    // Extract attributes
    work.common.extract_attributes(&mut attrs)?;
    work.authorized.extract_attributes(&mut attrs)?;
    work.bibl.extract_attributes(&mut attrs)?;
    work.data_pointing.extract_attributes(&mut attrs)?;
    work.pointing.extract_attributes(&mut attrs)?;
    work.target_eval.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // work can contain: head*, identifier*, title+, respStmt?, dedication?,
    // key*, tempo*, meter*, mensuration*, incip*, otherChar*, creation?,
    // history?, langUsage?, perfMedium?, perfDuration?, audience?, contents?,
    // context?, biblList*, notesStmt?, classification?, expressionList?,
    // componentList?, relationList*, extMeta*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("work")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Head(Box::new(head)));
                }
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Title(Box::new(title)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Identifier(Box::new(identifier)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::RespStmt(Box::new(resp_stmt)));
                }
                "dedication" => {
                    let dedication = parse_dedication_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Dedication(Box::new(dedication)));
                }
                "key" => {
                    let key = parse_key_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Key(Box::new(key)));
                }
                "tempo" => {
                    let tempo = parse_tempo_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Tempo(Box::new(tempo)));
                }
                "meter" => {
                    let meter = parse_meter_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Meter(Box::new(meter)));
                }
                "mensuration" => {
                    let mensuration =
                        parse_mensuration_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Mensuration(Box::new(mensuration)));
                }
                "incip" => {
                    let incip = parse_incip_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Incip(Box::new(incip)));
                }
                "otherChar" => {
                    let other_char = parse_other_char_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::OtherChar(Box::new(other_char)));
                }
                "creation" => {
                    let creation = parse_creation_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Creation(Box::new(creation)));
                }
                "history" => {
                    let history = parse_history_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::History(Box::new(history)));
                }
                "langUsage" => {
                    let lang_usage = parse_lang_usage_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::LangUsage(Box::new(lang_usage)));
                }
                "perfMedium" => {
                    let perf_medium =
                        parse_perf_medium_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::PerfMedium(Box::new(perf_medium)));
                }
                "perfDuration" => {
                    let perf_duration =
                        parse_perf_duration_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::PerfDuration(Box::new(perf_duration)));
                }
                "audience" => {
                    let audience = parse_audience_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Audience(Box::new(audience)));
                }
                "contents" => {
                    let contents = parse_contents_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Contents(Box::new(contents)));
                }
                "context" => {
                    let context = parse_context_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Context(Box::new(context)));
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::BiblList(Box::new(bibl_list)));
                }
                "notesStmt" => {
                    let notes_stmt = parse_notes_stmt_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::NotesStmt(Box::new(notes_stmt)));
                }
                "classification" => {
                    let classification =
                        parse_classification_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Classification(Box::new(classification)));
                }
                "expressionList" => {
                    let expression_list =
                        parse_expression_list_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::ExpressionList(Box::new(expression_list)));
                }
                "componentList" => {
                    let component_list =
                        parse_component_list_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::ComponentList(Box::new(component_list)));
                }
                "relationList" => {
                    let relation_list =
                        parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::RelationList(Box::new(relation_list)));
                }
                "extMeta" => {
                    let ext_meta = parse_ext_meta_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::ExtMeta(Box::new(ext_meta)));
                }
                // Handle deprecated MEI 5.x elements by converting to Creator
                "composer" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "composer",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Cmp,
                    )?;
                    work.children.push(WorkChild::Creator(Box::new(creator)));
                }
                "lyricist" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "lyricist",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Lyr,
                    )?;
                    work.children.push(WorkChild::Creator(Box::new(creator)));
                }
                "arranger" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "arranger",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Arr,
                    )?;
                    work.children.push(WorkChild::Creator(Box::new(creator)));
                }
                "author" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "author",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Aut,
                    )?;
                    work.children.push(WorkChild::Creator(Box::new(creator)));
                }
                "librettist" => {
                    let creator = parse_deprecated_creator_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                        "librettist",
                        tusk_model::generated::data::DataMarcrelatorsBasic::Lbt,
                    )?;
                    work.children.push(WorkChild::Creator(Box::new(creator)));
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

    Ok(work)
}

// ============================================================================
// RevisionDesc element implementation
// ============================================================================

impl MeiDeserialize for RevisionDesc {
    fn element_name() -> &'static str {
        "revisionDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_revision_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<revisionDesc>` element from within another element.
pub(crate) fn parse_revision_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RevisionDesc> {
    let mut revision_desc = RevisionDesc::default();

    // Extract attributes
    revision_desc.common.extract_attributes(&mut attrs)?;
    revision_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // revisionDesc can contain: head*, change+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("revisionDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    revision_desc
                        .children
                        .push(RevisionDescChild::Head(Box::new(head)));
                }
                "change" => {
                    let change = parse_change_from_event(reader, child_attrs, child_empty)?;
                    revision_desc
                        .children
                        .push(RevisionDescChild::Change(Box::new(change)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(revision_desc)
}

// ============================================================================
// Change element implementation
// ============================================================================

impl MeiDeserialize for Change {
    fn element_name() -> &'static str {
        "change"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_change_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<change>` element from within another element.
pub(crate) fn parse_change_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Change> {
    let mut change = Change::default();

    // Extract attributes
    change.common.extract_attributes(&mut attrs)?;
    change.bibl.extract_attributes(&mut attrs)?;
    change.datable.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // change can contain: date*, changeDesc*, respStmt*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("change")? {
            match name.as_str() {
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    change.children.push(ChangeChild::Date(Box::new(date)));
                }
                "changeDesc" => {
                    let change_desc =
                        parse_change_desc_from_event(reader, child_attrs, child_empty)?;
                    change
                        .children
                        .push(ChangeChild::ChangeDesc(Box::new(change_desc)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    change
                        .children
                        .push(ChangeChild::RespStmt(Box::new(resp_stmt)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(change)
}

// ============================================================================
// ChangeDesc element implementation
// ============================================================================

impl MeiDeserialize for ChangeDesc {
    fn element_name() -> &'static str {
        "changeDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_change_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<changeDesc>` element from within another element.
pub(crate) fn parse_change_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ChangeDesc> {
    let mut change_desc = ChangeDesc::default();

    // Extract attributes
    change_desc.common.extract_attributes(&mut attrs)?;
    change_desc.bibl.extract_attributes(&mut attrs)?;
    change_desc.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // changeDesc can contain: p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("changeDesc")?
        {
            match name.as_str() {
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    change_desc.children.push(ChangeDescChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(change_desc)
}

// ============================================================================
// Mei (root element) implementation
// ============================================================================

impl MeiDeserialize for tusk_model::elements::Mei {
    fn element_name() -> &'static str {
        "mei"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::{Mei, MeiChild};

        let mut mei = Mei::default();

        // Extract attributes
        // Mei has: AttId, AttMeiVersion, AttResponsibility
        // AttId: xml:id
        extract_attr!(attrs, "xml:id", string mei.id.xml_id);
        // AttMeiVersion: meiversion
        mei.mei_version.extract_attributes(&mut attrs)?;
        // AttResponsibility: resp
        mei.responsibility.extract_attributes(&mut attrs)?;

        // Parse children if not empty
        // mei can contain: meiHead?, music?
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("mei")?
            {
                match name.as_str() {
                    "meiHead" => {
                        let mei_head = tusk_model::elements::MeiHead::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));
                    }
                    "music" => {
                        let music = tusk_model::elements::Music::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        mei.children.push(MeiChild::Music(Box::new(music)));
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

        Ok(mei)
    }
}

// ============================================================================
// Music element implementation
// ============================================================================

impl MeiDeserialize for tusk_model::elements::Music {
    fn element_name() -> &'static str {
        "music"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::{Body, Music, MusicChild};

        let mut music = Music::default();

        // Extract attributes
        // Music has: AttCommon, AttMeiVersion, AttMetadataPointing
        music.common.extract_attributes(&mut attrs)?;
        music.mei_version.extract_attributes(&mut attrs)?;
        music.metadata_pointing.extract_attributes(&mut attrs)?;

        // Parse children if not empty
        // MusicChild can contain: body, group, front, back, facsimile, genDesc, performance
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("music")?
            {
                match name.as_str() {
                    "body" => {
                        let body = Body::from_mei_event(reader, child_attrs, child_empty)?;
                        music.children.push(MusicChild::Body(Box::new(body)));
                    }
                    // TODO: Add group, front, back, facsimile, genDesc, performance when needed
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(music)
    }
}

// ============================================================================
// Num element implementation
// ============================================================================

/// Parse a `<num>` element from within another element.
///
/// Num (number) contains numeric information in any form. It can contain
/// mixed content (text and child elements like rend, lb, etc.).
pub(crate) fn parse_num_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Num> {
    let mut num = Num::default();

    // Extract attributes
    num.common.extract_attributes(&mut attrs)?;
    num.facsimile.extract_attributes(&mut attrs)?;
    num.lang.extract_attributes(&mut attrs)?;
    // Note: AttMeasurement and AttRanging don't have ExtractAttributes implemented yet
    // Their attributes are extracted manually below

    // Element-local attribute: @value
    if let Some(val_str) = attrs.remove("value") {
        num.value = val_str.parse().ok();
    }

    // AttMeasurement: @unit
    if let Some(unit_str) = attrs.remove("unit") {
        num.measurement.unit = from_attr_string(&unit_str).ok();
    }

    // AttRanging: @atleast, @atmost, @min, @max, @confidence
    if let Some(val) = attrs.remove("atleast") {
        num.ranging.atleast = val.parse().ok();
    }
    if let Some(val) = attrs.remove("atmost") {
        num.ranging.atmost = val.parse().ok();
    }
    if let Some(val) = attrs.remove("min") {
        num.ranging.min = val.parse().ok();
    }
    if let Some(val) = attrs.remove("max") {
        num.ranging.max = val.parse().ok();
    }
    if let Some(val) = attrs.remove("confidence") {
        num.ranging.confidence = from_attr_string(&val).ok();
    }

    // Parse mixed content (text + child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("num")? {
            match content {
                MixedContent::Text(text) => {
                    num.children.push(NumChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            num.children.push(NumChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            num.children.push(NumChild::Lb(Box::new(lb)));
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

    Ok(num)
}

impl MeiDeserialize for Num {
    fn element_name() -> &'static str {
        "num"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_num_from_event(reader, attrs, is_empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // WorkList tests
    // ========================================================================

    #[test]
    fn work_list_deserializes_basic() {
        use tusk_model::elements::{WorkList, WorkListChild};

        let xml = r#"<workList>
            <work/>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.children.len(), 1);
        assert!(matches!(work_list.children[0], WorkListChild::Work(_)));
    }

    #[test]
    fn work_list_deserializes_with_xml_id() {
        use tusk_model::elements::WorkList;

        let xml = r#"<workList xml:id="wl1">
            <work/>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.common.xml_id, Some("wl1".to_string()));
    }

    #[test]
    fn work_list_deserializes_with_head_and_work() {
        use tusk_model::elements::{WorkList, WorkListChild};

        let xml = r#"<workList>
            <head>List of Works</head>
            <work xml:id="w1"/>
            <work xml:id="w2"/>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.children.len(), 3);
        assert!(matches!(work_list.children[0], WorkListChild::Head(_)));
        assert!(matches!(work_list.children[1], WorkListChild::Work(_)));
        assert!(matches!(work_list.children[2], WorkListChild::Work(_)));

        // Verify work xml:ids
        match &work_list.children[1] {
            WorkListChild::Work(w) => {
                assert_eq!(w.common.xml_id, Some("w1".to_string()));
            }
            _ => panic!("expected Work"),
        }
        match &work_list.children[2] {
            WorkListChild::Work(w) => {
                assert_eq!(w.common.xml_id, Some("w2".to_string()));
            }
            _ => panic!("expected Work"),
        }
    }

    #[test]
    fn work_list_deserializes_work_with_title() {
        use tusk_model::elements::{WorkList, WorkListChild};

        let xml = r#"<workList>
            <work xml:id="w1">
                <title>Symphony No. 5</title>
            </work>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.children.len(), 1);
        match &work_list.children[0] {
            WorkListChild::Work(w) => {
                assert_eq!(w.common.xml_id, Some("w1".to_string()));
                assert_eq!(w.children.len(), 1);
            }
            _ => panic!("expected Work"),
        }
    }

    #[test]
    fn work_list_deserializes_empty_element() {
        use tusk_model::elements::WorkList;

        // Empty workList (not valid per schema but we're lenient)
        let xml = r#"<workList/>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert!(work_list.children.is_empty());
    }

    #[test]
    fn work_list_in_mei_head() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead>
            <fileDesc>
                <titleStmt>
                    <title>Test</title>
                </titleStmt>
            </fileDesc>
            <workList>
                <work xml:id="w1">
                    <title>Test Work</title>
                </work>
            </workList>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        // Should have fileDesc and workList
        assert_eq!(mei_head.children.len(), 2);

        // First child should be fileDesc
        assert!(matches!(mei_head.children[0], MeiHeadChild::FileDesc(_)));

        // Second child should be workList
        assert!(matches!(mei_head.children[1], MeiHeadChild::WorkList(_)));

        match &mei_head.children[1] {
            MeiHeadChild::WorkList(wl) => {
                assert_eq!(wl.children.len(), 1);
            }
            _ => panic!("expected WorkList"),
        }
    }

    // ========================================================================
    // Work tests
    // ========================================================================

    #[test]
    fn work_deserializes_empty() {
        use tusk_model::elements::Work;

        let xml = r#"<work/>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert!(work.common.xml_id.is_none());
        assert!(work.children.is_empty());
    }

    #[test]
    fn work_deserializes_with_xml_id() {
        use tusk_model::elements::Work;

        let xml = r#"<work xml:id="w1"/>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.common.xml_id, Some("w1".to_string()));
    }

    #[test]
    fn work_deserializes_with_title() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Symphony No. 5</title>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 1);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
    }

    #[test]
    fn work_deserializes_with_identifier() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <identifier>ISMN 979-0-1234-5678-9</identifier>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
        assert!(matches!(work.children[1], WorkChild::Identifier(_)));
    }

    #[test]
    fn work_deserializes_with_resp_stmt() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <respStmt xml:id="rs1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::RespStmt(_)));
    }

    #[test]
    fn work_deserializes_with_dedication() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <dedication>For my beloved</dedication>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Dedication(_)));
    }

    #[test]
    fn work_deserializes_with_creation() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <creation>
                <date isodate="1808">1808</date>
            </creation>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Creation(_)));
    }

    #[test]
    fn work_deserializes_with_history() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <history>
                <p>Composed in Vienna.</p>
            </history>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::History(_)));
    }

    #[test]
    fn work_deserializes_with_lang_usage() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <langUsage>
                <language xml:lang="de">German</language>
            </langUsage>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::LangUsage(_)));
    }

    #[test]
    fn work_deserializes_with_perf_medium() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <perfMedium>
                <head>Instrumentation</head>
            </perfMedium>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::PerfMedium(_)));
    }

    #[test]
    fn work_deserializes_with_perf_duration() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <perfDuration xml:id="pd1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::PerfDuration(_)));
    }

    #[test]
    fn work_deserializes_with_audience() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <audience xml:id="aud1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Audience(_)));
    }

    #[test]
    fn work_deserializes_with_contents() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <contents>
                <head>Contents</head>
            </contents>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Contents(_)));
    }

    #[test]
    fn work_deserializes_with_context() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <context xml:id="ctx1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Context(_)));
    }

    #[test]
    fn work_deserializes_with_bibl_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <biblList>
                <head>Bibliography</head>
            </biblList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::BiblList(_)));
    }

    #[test]
    fn work_deserializes_with_notes_stmt() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <notesStmt>
                <head>Notes</head>
            </notesStmt>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::NotesStmt(_)));
    }

    #[test]
    fn work_deserializes_with_classification() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <classification>
                <head>Classification</head>
            </classification>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Classification(_)));
    }

    #[test]
    fn classification_deserializes_with_term_list() {
        use tusk_model::elements::{Classification, ClassificationChild, TermChild, TermListChild};

        let xml = r##"<classification>
            <termList>
                <term class="#_4021100-9">Gitarrenmusik</term>
                <term class="#_4129951-6">Instrumentalmusik</term>
                <term class="#_4302724-6">Zupfmusik</term>
            </termList>
        </classification>"##;
        let classification =
            Classification::from_mei_str(xml).expect("should deserialize classification");

        assert_eq!(classification.children.len(), 1);
        if let ClassificationChild::TermList(term_list) = &classification.children[0] {
            assert_eq!(term_list.children.len(), 3);
            // Check first term
            if let TermListChild::Term(term) = &term_list.children[0] {
                assert_eq!(term.common.class.len(), 1);
                assert_eq!(term.common.class[0].to_string(), "#_4021100-9");
                // Check text content
                assert_eq!(term.children.len(), 1);
                if let TermChild::Text(text) = &term.children[0] {
                    assert_eq!(text, "Gitarrenmusik");
                } else {
                    panic!("Expected Text child");
                }
            } else {
                panic!("Expected Term child");
            }
        } else {
            panic!("Expected TermList child");
        }
    }

    #[test]
    fn classification_with_term_list_roundtrips() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Classification;

        let xml = r##"<classification>
            <termList>
                <term class="#_4021100-9">Gitarrenmusik</term>
                <term class="#_4129951-6">Instrumentalmusik</term>
            </termList>
        </classification>"##;
        let classification =
            Classification::from_mei_str(xml).expect("should deserialize classification");

        // Serialize back
        let output_str = classification.to_mei_string().expect("should serialize");

        // Verify output contains expected content
        assert!(output_str.contains("<termList>"));
        assert!(output_str.contains("<term"));
        assert!(output_str.contains("Gitarrenmusik"));
        assert!(output_str.contains("Instrumentalmusik"));
        assert!(output_str.contains(r##"class="#_4021100-9""##));
    }

    #[test]
    fn work_deserializes_with_expression_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <expressionList>
                <head>Expressions</head>
            </expressionList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::ExpressionList(_)));
    }

    #[test]
    fn work_deserializes_with_component_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <componentList>
                <head>Components</head>
            </componentList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::ComponentList(_)));
    }

    #[test]
    fn work_deserializes_with_relation_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <relationList>
                <head>Relations</head>
            </relationList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::RelationList(_)));
    }

    #[test]
    fn work_deserializes_with_ext_meta() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <extMeta xml:id="em1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::ExtMeta(_)));
    }

    #[test]
    fn work_deserializes_with_other_char() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <otherChar xml:id="oc1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::OtherChar(_)));
    }

    #[test]
    fn work_deserializes_with_key() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <key pname="c" mode="major">C major</key>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Key(_)));
    }

    #[test]
    fn work_deserializes_with_tempo() {
        use tusk_model::elements::{TempoChild, Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <tempo>Allegro</tempo>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        match &work.children[1] {
            WorkChild::Tempo(tempo) => {
                assert_eq!(tempo.children.len(), 1);
                match &tempo.children[0] {
                    TempoChild::Text(text) => assert_eq!(text, "Allegro"),
                    _ => panic!("Expected Text child"),
                }
            }
            _ => panic!("Expected Tempo child"),
        }
    }

    #[test]
    fn work_deserializes_with_tempo_text_undefined() {
        use tusk_model::elements::{TempoChild, Work, WorkChild};

        // Exact format from Ahle_Jesu_meines_Herzens_Freud.mei
        let xml = r#"<work><tempo>undefined</tempo></work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 1);
        match &work.children[0] {
            WorkChild::Tempo(tempo) => {
                assert_eq!(tempo.children.len(), 1, "Tempo should have 1 child");
                match &tempo.children[0] {
                    TempoChild::Text(text) => assert_eq!(text, "undefined"),
                    other => panic!("Expected Text child, got {:?}", other),
                }
            }
            other => panic!("Expected Tempo child, got {:?}", other),
        }
    }

    #[test]
    fn work_deserializes_with_meter() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <meter count="4" unit="4">4/4</meter>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Meter(_)));
    }

    #[test]
    fn work_deserializes_with_mensuration() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <mensuration xml:id="mens1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Mensuration(_)));
    }

    #[test]
    fn work_deserializes_with_incip() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <incip xml:id="inc1">
                <head>Incipit</head>
            </incip>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Incip(_)));
    }

    #[test]
    fn work_deserializes_full_example() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work xml:id="w1">
            <head>Work Information</head>
            <identifier>TEST-001</identifier>
            <title>Symphony No. 5 in C minor</title>
            <respStmt xml:id="rs1"/>
            <dedication>For the Prince</dedication>
            <key pname="c" mode="minor"/>
            <tempo>Allegro con brio</tempo>
            <meter count="2" unit="4"/>
            <creation>
                <date isodate="1808">1808</date>
            </creation>
            <history>
                <p>Premiered in Vienna.</p>
            </history>
            <langUsage>
                <language xml:lang="de">German</language>
            </langUsage>
            <perfMedium>
                <head>Orchestra</head>
            </perfMedium>
            <audience xml:id="aud1"/>
            <contents>
                <head>Movements</head>
            </contents>
            <context xml:id="ctx1"/>
            <biblList>
                <head>Bibliography</head>
            </biblList>
            <notesStmt>
                <head>Notes</head>
            </notesStmt>
            <classification>
                <head>Classification</head>
            </classification>
            <expressionList>
                <head>Expressions</head>
            </expressionList>
            <componentList>
                <head>Components</head>
            </componentList>
            <relationList>
                <head>Relations</head>
            </relationList>
            <extMeta xml:id="em1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.common.xml_id, Some("w1".to_string()));
        // Should have multiple children
        assert!(work.children.len() > 10);

        // Verify specific children
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Head(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Title(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Identifier(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::RespStmt(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Dedication(_)))
        );
        assert!(work.children.iter().any(|c| matches!(c, WorkChild::Key(_))));
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Tempo(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Meter(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Creation(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::History(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::LangUsage(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::PerfMedium(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Audience(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Contents(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Context(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::BiblList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::NotesStmt(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Classification(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::ExpressionList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::ComponentList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::RelationList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::ExtMeta(_)))
        );
    }

    #[test]
    fn work_deserializes_deprecated_composer() {
        use tusk_model::elements::{Work, WorkChild};

        // MEI 5.x deprecated <composer> element - should be migrated to Creator
        let xml = r#"<work>
            <title>Walzer G-Dur</title>
            <composer>
                <persName role="creator">Dionisio Aguado</persName>
            </composer>
            <key pname="g" mode="major"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 3);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
        assert!(matches!(work.children[1], WorkChild::Creator(_)));
        assert!(matches!(work.children[2], WorkChild::Key(_)));

        // Verify the Creator has the expected role from migration
        if let WorkChild::Creator(creator) = &work.children[1] {
            // The role should be set to "cmp" (composer) from the migration
            assert!(!creator.name.role.is_empty());
        } else {
            panic!("expected Creator");
        }
    }

    #[test]
    fn work_deserializes_deprecated_lyricist() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Song Title</title>
            <lyricist>Lyrics Author</lyricist>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
        assert!(matches!(work.children[1], WorkChild::Creator(_)));
    }

    #[test]
    fn work_deserializes_deprecated_arranger() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Arranged Work</title>
            <arranger>Arranger Name</arranger>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
        assert!(matches!(work.children[1], WorkChild::Creator(_)));
    }

    #[test]
    fn work_deserializes_deprecated_author() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Text Work</title>
            <author>Author Name</author>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
        assert!(matches!(work.children[1], WorkChild::Creator(_)));
    }

    // ========== RevisionDesc tests ==========

    #[test]
    fn revision_desc_deserializes_empty_element() {
        use tusk_model::elements::RevisionDesc;

        let xml = r#"<revisionDesc/>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert!(revision_desc.common.xml_id.is_none());
        assert!(revision_desc.children.is_empty());
    }

    #[test]
    fn revision_desc_deserializes_xml_id() {
        use tusk_model::elements::RevisionDesc;

        let xml = r#"<revisionDesc xml:id="rd1"/>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.common.xml_id, Some("rd1".to_string()));
    }

    #[test]
    fn revision_desc_deserializes_bibl_attributes() {
        use tusk_model::elements::RevisionDesc;

        let xml = r#"<revisionDesc analog="MARC21"/>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.bibl.analog, Some("MARC21".to_string()));
    }

    #[test]
    fn revision_desc_deserializes_with_single_change() {
        use tusk_model::elements::{RevisionDesc, RevisionDescChild};

        let xml = r#"<revisionDesc>
            <change xml:id="ch1"/>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 1);
        match &revision_desc.children[0] {
            RevisionDescChild::Change(ch) => {
                assert_eq!(ch.common.xml_id, Some("ch1".to_string()));
            }
            _ => panic!("expected Change child"),
        }
    }

    #[test]
    fn revision_desc_deserializes_with_multiple_changes() {
        use tusk_model::elements::{RevisionDesc, RevisionDescChild};

        let xml = r#"<revisionDesc>
            <change xml:id="ch1" n="1"/>
            <change xml:id="ch2" n="2"/>
            <change xml:id="ch3" n="3"/>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 3);
        for (i, child) in revision_desc.children.iter().enumerate() {
            match child {
                RevisionDescChild::Change(ch) => {
                    assert_eq!(ch.common.xml_id, Some(format!("ch{}", i + 1)));
                }
                _ => panic!("expected Change child at index {}", i),
            }
        }
    }

    #[test]
    fn revision_desc_deserializes_with_head() {
        use tusk_model::elements::{RevisionDesc, RevisionDescChild};

        let xml = r#"<revisionDesc>
            <head>Revision History</head>
            <change xml:id="ch1"/>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 2);
        assert!(matches!(
            &revision_desc.children[0],
            RevisionDescChild::Head(_)
        ));
        assert!(matches!(
            &revision_desc.children[1],
            RevisionDescChild::Change(_)
        ));
    }

    #[test]
    fn change_deserializes_empty_element() {
        use tusk_model::elements::Change;

        let xml = r#"<change/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert!(change.common.xml_id.is_none());
        assert!(change.children.is_empty());
    }

    #[test]
    fn change_deserializes_xml_id() {
        use tusk_model::elements::Change;

        let xml = r#"<change xml:id="ch1"/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.common.xml_id, Some("ch1".to_string()));
    }

    #[test]
    fn change_deserializes_isodate_attribute() {
        use tusk_model::data::DataIsodate;
        use tusk_model::elements::Change;

        let xml = r#"<change isodate="2011-10-21"/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            change.datable.isodate,
            Some(DataIsodate("2011-10-21".to_string()))
        );
    }

    #[test]
    fn change_deserializes_n_attribute() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::Change;

        let xml = r#"<change n="3"/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.common.n, Some(DataWord("3".to_string())));
    }

    #[test]
    fn change_deserializes_with_date_child() {
        use tusk_model::data::DataIsodate;
        use tusk_model::elements::{Change, ChangeChild};

        let xml = r#"<change>
            <date isodate="2011-12-01"/>
        </change>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.children.len(), 1);
        match &change.children[0] {
            ChangeChild::Date(date) => {
                assert_eq!(
                    date.datable.isodate,
                    Some(DataIsodate("2011-12-01".to_string()))
                );
            }
            _ => panic!("expected Date child"),
        }
    }

    #[test]
    fn change_deserializes_with_resp_stmt_child() {
        use tusk_model::elements::{Change, ChangeChild};

        let xml = r#"<change>
            <respStmt xml:id="rs1"/>
        </change>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.children.len(), 1);
        match &change.children[0] {
            ChangeChild::RespStmt(rs) => {
                assert_eq!(rs.common.xml_id, Some("rs1".to_string()));
            }
            _ => panic!("expected RespStmt child"),
        }
    }

    #[test]
    fn change_deserializes_with_change_desc_child() {
        use tusk_model::elements::{Change, ChangeChild};

        let xml = r#"<change>
            <changeDesc xml:id="cd1"/>
        </change>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.children.len(), 1);
        match &change.children[0] {
            ChangeChild::ChangeDesc(cd) => {
                assert_eq!(cd.common.xml_id, Some("cd1".to_string()));
            }
            _ => panic!("expected ChangeDesc child"),
        }
    }

    #[test]
    fn change_desc_deserializes_empty_element() {
        use tusk_model::elements::ChangeDesc;

        let xml = r#"<changeDesc/>"#;
        let change_desc = ChangeDesc::from_mei_str(xml).expect("should deserialize");

        assert!(change_desc.common.xml_id.is_none());
        assert!(change_desc.children.is_empty());
    }

    #[test]
    fn change_desc_deserializes_xml_id() {
        use tusk_model::elements::ChangeDesc;

        let xml = r#"<changeDesc xml:id="cd1"/>"#;
        let change_desc = ChangeDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change_desc.common.xml_id, Some("cd1".to_string()));
    }

    #[test]
    fn change_desc_deserializes_with_p_child() {
        use tusk_model::elements::{ChangeDesc, ChangeDescChild};

        let xml = r#"<changeDesc>
            <p>Cleaned up MEI file automatically using Header.xsl.</p>
        </changeDesc>"#;
        let change_desc = ChangeDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change_desc.children.len(), 1);
        assert!(matches!(&change_desc.children[0], ChangeDescChild::P(_)));
    }

    #[test]
    fn revision_desc_deserializes_full_example() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::{ChangeChild, RevisionDesc, RevisionDescChild};

        // Based on header-sample076.txt from MEI spec examples
        let xml = r#"<revisionDesc>
            <change n="4">
                <respStmt>
                    <persName>KR</persName>
                </respStmt>
                <changeDesc>
                    <p>Cleaned up MEI file automatically using Header.xsl.</p>
                </changeDesc>
                <date isodate="2011-12-01"/>
            </change>
            <change n="3">
                <respStmt>
                    <persName>KR</persName>
                </respStmt>
                <changeDesc>
                    <p>Cleaned up MEI file automatically using ppq.xsl.</p>
                </changeDesc>
                <date isodate="2011-10-21"/>
            </change>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 2);

        // First change
        match &revision_desc.children[0] {
            RevisionDescChild::Change(ch) => {
                assert_eq!(ch.common.n, Some(DataWord("4".to_string())));
                assert_eq!(ch.children.len(), 3);
                assert!(
                    ch.children
                        .iter()
                        .any(|c| matches!(c, ChangeChild::RespStmt(_)))
                );
                assert!(
                    ch.children
                        .iter()
                        .any(|c| matches!(c, ChangeChild::ChangeDesc(_)))
                );
                assert!(
                    ch.children
                        .iter()
                        .any(|c| matches!(c, ChangeChild::Date(_)))
                );
            }
            _ => panic!("expected Change child"),
        }

        // Second change
        match &revision_desc.children[1] {
            RevisionDescChild::Change(ch) => {
                assert_eq!(ch.common.n, Some(DataWord("3".to_string())));
            }
            _ => panic!("expected Change child"),
        }
    }

    #[test]
    fn change_deserializes_with_isodate_and_resp_on_element() {
        use tusk_model::data::{DataIsodate, DataWord};
        use tusk_model::elements::Change;

        // Based on header-sample077.txt - shorter form with attributes on change itself
        let xml = "<change isodate=\"2011-10-21\" n=\"3\" resp=\"#KR #MH\"/>";
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            change.datable.isodate,
            Some(DataIsodate("2011-10-21".to_string()))
        );
        assert_eq!(change.common.n, Some(DataWord("3".to_string())));
        // resp is on common attribute class
    }

    #[test]
    fn mei_head_deserializes_with_revision_desc() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <revisionDesc xml:id="rd1">
                <change n="1">
                    <changeDesc>
                        <p>Initial encoding.</p>
                    </changeDesc>
                </change>
            </revisionDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::RevisionDesc(rd) => {
                assert_eq!(rd.common.xml_id, Some("rd1".to_string()));
                assert_eq!(rd.children.len(), 1);
            }
            _ => panic!("expected RevisionDesc child"),
        }
    }

    #[test]
    fn mei_head_deserializes_with_file_desc_and_revision_desc() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead>
            <fileDesc xml:id="fd1"/>
            <revisionDesc xml:id="rd1">
                <change/>
            </revisionDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.children.len(), 2);
        assert!(matches!(&mei_head.children[0], MeiHeadChild::FileDesc(_)));
        assert!(matches!(
            &mei_head.children[1],
            MeiHeadChild::RevisionDesc(_)
        ));
    }

    #[test]
    fn incip_deserializes_incip_code() {
        use tusk_model::elements::{IncipChild, IncipCodeChild, Work, WorkChild};

        let xml = r#"<work>
            <title>Test Work</title>
            <incip>
                <incipCode form="notAvailable">Incipit is not available</incipCode>
            </incip>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        match &work.children[1] {
            WorkChild::Incip(incip) => {
                assert_eq!(incip.children.len(), 1);
                match &incip.children[0] {
                    IncipChild::IncipCode(code) => {
                        assert_eq!(code.form.as_deref(), Some("notAvailable"));
                        assert_eq!(code.children.len(), 1);
                        match &code.children[0] {
                            IncipCodeChild::Text(text) => {
                                assert_eq!(text, "Incipit is not available");
                            }
                        }
                    }
                    _ => panic!("Expected IncipCode child"),
                }
            }
            _ => panic!("Expected Incip child"),
        }
    }

    #[test]
    fn perf_medium_deserializes_perf_res_list_with_perf_res() {
        use tusk_model::elements::{
            PerfMediumChild, PerfResChild, PerfResListChild, Work, WorkChild,
        };

        let xml = r#"<work>
            <title>Test Work</title>
            <perfMedium>
                <perfResList>
                    <perfRes n="1" codedval="tb">Guitar</perfRes>
                </perfResList>
            </perfMedium>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        match &work.children[1] {
            WorkChild::PerfMedium(perf_medium) => {
                assert_eq!(perf_medium.children.len(), 1);
                match &perf_medium.children[0] {
                    PerfMediumChild::PerfResList(perf_res_list) => {
                        assert_eq!(perf_res_list.children.len(), 1);
                        match &perf_res_list.children[0] {
                            PerfResListChild::PerfRes(perf_res) => {
                                // Check that n attribute is parsed
                                assert!(perf_res.common.n.is_some());
                                assert_eq!(perf_res.common.n.as_ref().unwrap().0, "1");
                                // Check that codedval attribute is parsed (in authorized)
                                assert_eq!(perf_res.authorized.codedval, vec!["tb"]);
                                // Check that text content is parsed
                                assert_eq!(perf_res.children.len(), 1);
                                match &perf_res.children[0] {
                                    PerfResChild::Text(text) => {
                                        assert_eq!(text, "Guitar");
                                    }
                                    _ => panic!("Expected Text child in perfRes"),
                                }
                            }
                            _ => panic!("Expected PerfRes child in perfResList"),
                        }
                    }
                    _ => panic!("Expected PerfResList child in perfMedium"),
                }
            }
            _ => panic!("Expected PerfMedium child"),
        }
    }

    #[test]
    fn history_deserializes_with_event_list() {
        use tusk_model::elements::{EventChild, EventListChild, History, HistoryChild};

        let xml = r#"<history>
            <eventList>
                <event>
                    <p>First performance<date isodate="1727-04-11"/><geogName>Leipzig</geogName></p>
                </event>
                <event>
                    <p>First publication<date>1830</date></p>
                </event>
            </eventList>
        </history>"#;
        let history = History::from_mei_str(xml).expect("should deserialize");

        assert_eq!(history.children.len(), 1);
        match &history.children[0] {
            HistoryChild::EventList(event_list) => {
                assert_eq!(event_list.children.len(), 2);
                // First event
                match &event_list.children[0] {
                    EventListChild::Event(event) => {
                        assert_eq!(event.children.len(), 1);
                        match &event.children[0] {
                            EventChild::P(_) => {} // expected
                            _ => panic!("Expected P child in event"),
                        }
                    }
                    _ => panic!("Expected Event child in eventList"),
                }
                // Second event
                match &event_list.children[1] {
                    EventListChild::Event(event) => {
                        assert_eq!(event.children.len(), 1);
                    }
                    _ => panic!("Expected Event child in eventList"),
                }
            }
            _ => panic!("Expected EventList child in history"),
        }
    }
}

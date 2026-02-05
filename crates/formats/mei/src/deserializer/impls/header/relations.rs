//! Relation, RelationList, RelatedItem, Item, ItemList, ComponentList deserializers.
//!
//! These elements are used for describing relationships and bibliographic structures.

use super::super::{extract_attr, from_attr_string};
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::AttPlist;
use tusk_model::elements::{
    ComponentList, ComponentListChild, Item, ItemChild, ItemList, ItemListChild, RelatedItem,
    RelatedItemChild, Relation, RelationList, RelationListChild,
};

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for Relation {
    fn element_name() -> &'static str {
        "relation"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_relation_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for RelationList {
    fn element_name() -> &'static str {
        "relationList"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_relation_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for RelatedItem {
    fn element_name() -> &'static str {
        "relatedItem"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_related_item_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Item {
    fn element_name() -> &'static str {
        "item"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_item_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ItemList {
    fn element_name() -> &'static str {
        "itemList"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_item_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ComponentList {
    fn element_name() -> &'static str {
        "componentList"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_component_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for tusk_model::elements::PhysLoc {
    fn element_name() -> &'static str {
        "physLoc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_phys_loc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for tusk_model::elements::Repository {
    fn element_name() -> &'static str {
        "repository"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_repository_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<relation>` element.
pub(crate) fn parse_relation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Relation> {
    let mut elem = Relation::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.evidence.extract_attributes(&mut attrs)?;
    elem.plist.extract_attributes(&mut attrs)?;
    elem.pointing.extract_attributes(&mut attrs)?;
    elem.target_eval.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "rel", elem.rel);

    // Relation is an empty element (no children)
    if !is_empty {
        reader.skip_to_end("relation")?;
    }

    Ok(elem)
}

/// Parse a `<relationList>` element.
pub(crate) fn parse_relation_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RelationList> {
    let mut elem = RelationList::default();

    elem.common.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("relationList")?
        {
            match name.as_str() {
                "relation" => {
                    let child = parse_relation_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(RelationListChild::Relation(Box::new(child)));
                }
                "relationList" => {
                    let child = parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(RelationListChild::RelationList(Box::new(child)));
                }
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(RelationListChild::Head(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<relatedItem>` element.
pub(crate) fn parse_related_item_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RelatedItem> {
    let mut elem = RelatedItem::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.data_pointing.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.pointing.extract_attributes(&mut attrs)?;
    elem.target_eval.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "rel", elem.rel);

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("relatedItem")?
        {
            match name.as_str() {
                "biblStruct" => {
                    let child =
                        super::parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(RelatedItemChild::BiblStruct(Box::new(child)));
                }
                "bibl" => {
                    let child = super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(RelatedItemChild::Bibl(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse an `<item>` element.
pub(crate) fn parse_item_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Item> {
    let mut elem = Item::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.data_pointing.extract_attributes(&mut attrs)?;
    elem.pointing.extract_attributes(&mut attrs)?;
    elem.target_eval.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("item")? {
            match name.as_str() {
                "physDesc" => {
                    let child = super::super::misc::parse_phys_desc_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(ItemChild::PhysDesc(Box::new(child)));
                }
                "history" => {
                    let child = super::super::misc::parse_history_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(ItemChild::History(Box::new(child)));
                }
                "dedication" => {
                    let child = super::super::parse_dedication_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(ItemChild::Dedication(Box::new(child)));
                }
                "physLoc" => {
                    let child = parse_phys_loc_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemChild::PhysLoc(Box::new(child)));
                }
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemChild::Head(Box::new(child)));
                }
                "notesStmt" => {
                    let child = super::super::parse_notes_stmt_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(ItemChild::NotesStmt(Box::new(child)));
                }
                "componentList" => {
                    let child = parse_component_list_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(ItemChild::ComponentList(Box::new(child)));
                }
                "identifier" => {
                    let child =
                        super::parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemChild::Identifier(Box::new(child)));
                }
                "availability" => {
                    let child =
                        super::parse_availability_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemChild::Availability(Box::new(child)));
                }
                "relationList" => {
                    let child = parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemChild::RelationList(Box::new(child)));
                }
                "extMeta" => {
                    let child = super::super::misc::parse_ext_meta_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(ItemChild::ExtMeta(Box::new(child)));
                }
                "classification" => {
                    let child = super::super::misc::parse_classification_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children
                        .push(ItemChild::Classification(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse an `<itemList>` element.
pub(crate) fn parse_item_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ItemList> {
    let mut elem = ItemList::default();

    elem.common.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("itemList")?
        {
            match name.as_str() {
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemListChild::Head(Box::new(child)));
                }
                "item" => {
                    let child = parse_item_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(ItemListChild::Item(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<componentList>` element.
pub(crate) fn parse_component_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ComponentList> {
    let mut elem = ComponentList::default();

    elem.common.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("componentList")?
        {
            match name.as_str() {
                "item" => {
                    let child = parse_item_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(ComponentListChild::Item(Box::new(child)));
                }
                "work" => {
                    let child =
                        super::super::parse_work_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(ComponentListChild::Work(Box::new(child)));
                }
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(ComponentListChild::Head(Box::new(child)));
                }
                "expression" => {
                    let child = super::super::parse_expression_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children
                        .push(ComponentListChild::Expression(Box::new(child)));
                }
                "manifestation" => {
                    let child = super::super::misc::parse_manifestation_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children
                        .push(ComponentListChild::Manifestation(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<physLoc>` element.
pub(crate) fn parse_phys_loc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::PhysLoc> {
    use tusk_model::elements::{PhysLoc, PhysLocChild};

    let mut elem = PhysLoc::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;

    elem.facsimile.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("physLoc")?
        {
            match name.as_str() {
                "identifier" => {
                    let child =
                        super::parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(PhysLocChild::Identifier(Box::new(child)));
                }
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(PhysLocChild::Head(Box::new(child)));
                }
                "history" => {
                    let child = super::super::misc::parse_history_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(PhysLocChild::History(Box::new(child)));
                }
                "repository" => {
                    let child = parse_repository_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(PhysLocChild::Repository(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<repository>` element.
pub(crate) fn parse_repository_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Repository> {
    use crate::deserializer::MixedContent;
    use tusk_model::elements::{Repository, RepositoryChild};

    let mut elem = Repository::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("repository")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(RepositoryChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "corpName" => {
                        let child =
                            super::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::CorpName(Box::new(child)));
                    }
                    "name" => {
                        let child = super::parse_name_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Name(Box::new(child)));
                    }
                    "address" => {
                        let child =
                            super::parse_address_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::Address(Box::new(child)));
                    }
                    "identifier" => {
                        let child =
                            super::parse_identifier_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::Identifier(Box::new(child)));
                    }
                    "ptr" => {
                        let child = super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Ptr(Box::new(child)));
                    }
                    "ref" => {
                        let child = super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Ref(Box::new(child)));
                    }
                    "geogName" => {
                        let child =
                            super::parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::GeogName(Box::new(child)));
                    }
                    "country" => {
                        let child =
                            super::parse_country_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::Country(Box::new(child)));
                    }
                    "region" => {
                        let child =
                            super::parse_region_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Region(Box::new(child)));
                    }
                    "settlement" => {
                        let child =
                            super::parse_settlement_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::Settlement(Box::new(child)));
                    }
                    "district" => {
                        let child =
                            super::parse_district_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(RepositoryChild::District(Box::new(child)));
                    }
                    "bloc" => {
                        let child = super::parse_bloc_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Bloc(Box::new(child)));
                    }
                    "annot" => {
                        let child =
                            super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Annot(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RepositoryChild::Rend(Box::new(child)));
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

    Ok(elem)
}

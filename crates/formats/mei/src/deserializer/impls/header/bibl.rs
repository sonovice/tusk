//! Bibliographic elements (Bibl, BiblStruct, BiblScope, Imprint, Locus, LocusGrp).

use super::super::{extract_attr, from_attr_string};
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{Bibl, BiblScope, BiblStruct, Imprint, Locus, LocusGrp};

// MeiDeserialize trait implementations
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
                            let title =
                                super::parse_title_from_event(reader, child_attrs, child_empty)?;
                            bibl.children.push(BiblChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl.children
                                .push(BiblChild::Identifier(Box::new(identifier)));
                        }
                        "creator" => {
                            let creator =
                                super::parse_creator_from_event(reader, child_attrs, child_empty)?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        // Handle deprecated MEI elements by converting to Creator
                        "composer" => {
                            let creator = super::parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "composer",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Cmp,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "lyricist" => {
                            let creator = super::parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "lyricist",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Lyr,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "arranger" => {
                            let creator = super::parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "arranger",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Arr,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "author" => {
                            let creator = super::parse_deprecated_creator_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                                "author",
                                tusk_model::generated::data::DataMarcrelatorsBasic::Aut,
                            )?;
                            bibl.children.push(BiblChild::Creator(Box::new(creator)));
                        }
                        "librettist" => {
                            let creator = super::parse_deprecated_creator_from_event(
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
                            let editor =
                                super::parse_editor_from_event(reader, child_attrs, child_empty)?;
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
                            let publisher = super::parse_publisher_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::Publisher(Box::new(publisher)));
                        }
                        "pubPlace" => {
                            let pub_place = super::parse_pub_place_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::PubPlace(Box::new(pub_place)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Date(Box::new(date)));
                        }
                        "distributor" => {
                            let distributor = super::parse_distributor_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::Distributor(Box::new(distributor)));
                        }
                        "respStmt" => {
                            let resp_stmt = super::parse_resp_stmt_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::RespStmt(Box::new(resp_stmt)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::Identifier(Box::new(identifier)));
                        }
                        "title" => {
                            let title =
                                super::parse_title_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Title(Box::new(title)));
                        }
                        "availability" => {
                            let availability = super::parse_availability_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::Availability(Box::new(availability)));
                        }
                        "extent" => {
                            let extent = super::super::parse_extent_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::Extent(Box::new(extent)));
                        }
                        "address" => {
                            let address =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
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
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            imprint
                                .children
                                .push(ImprintChild::Name(Box::new(name_elem)));
                        }
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint
                                .children
                                .push(ImprintChild::GeogName(Box::new(geog_name)));
                        }
                        "annot" => {
                            let annot =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Annot(Box::new(annot)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            imprint.children.push(ImprintChild::Lb(Box::new(lb)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Ptr(Box::new(ptr)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            imprint.children.push(ImprintChild::Ref(Box::new(ref_elem)));
                        }
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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
                            let title =
                                super::parse_title_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Date(Box::new(date)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Name(Box::new(name_elem)));
                        }
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::GeogName(Box::new(geog_name)));
                        }
                        "address" => {
                            let address =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
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
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            bibl_scope.children.push(BiblScopeChild::Lb(Box::new(lb)));
                        }
                        "annot" => {
                            let annot =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Annot(Box::new(annot)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope.children.push(BiblScopeChild::Ptr(Box::new(ptr)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            bibl_scope
                                .children
                                .push(BiblScopeChild::Ref(Box::new(ref_elem)));
                        }
                        "symbol" => {
                            let symbol = super::super::control::parse_symbol_from_event(
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

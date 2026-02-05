//! Shared mixed-content elements (P, Ptr, Ref, Annot).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{Annot, AnnotChild, P, PChild, Ptr, Ref, RefChild};

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
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            p.children.push(PChild::Rend(Box::new(rend)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            p.children.push(PChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            p.children.push(PChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            p.children.push(PChild::GeogName(Box::new(geog_name)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title =
                                super::parse_title_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Title(Box::new(title)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Annot(Box::new(annot)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            p.children.push(PChild::Lb(Box::new(lb)));
                        }
                        "bibl" => {
                            let bibl =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            p.children.push(PChild::Bibl(Box::new(bibl)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            p.children.push(PChild::Identifier(Box::new(identifier)));
                        }
                        "list" => {
                            let list = super::super::parse_list_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            ref_elem
                                .children
                                .push(RefChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            ref_elem
                                .children
                                .push(RefChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            ref_elem
                                .children
                                .push(RefChild::GeogName(Box::new(geog_name)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier = super::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            ref_elem
                                .children
                                .push(RefChild::Identifier(Box::new(identifier)));
                        }
                        "title" => {
                            let title =
                                super::parse_title_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Title(Box::new(title)));
                        }
                        "annot" => {
                            let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Annot(Box::new(annot)));
                        }
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            ref_elem.children.push(RefChild::Rend(Box::new(rend)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            ref_elem.children.push(RefChild::Ptr(Box::new(ptr)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
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

/// Parse an `<annot>` element from within another element.
pub(crate) fn parse_annot_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Annot> {
    let mut annot = Annot::default();

    // Extract attributes
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

    // annot is a mixed content element
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("annot")? {
            match content {
                MixedContent::Text(text) => {
                    annot.children.push(AnnotChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "p" => {
                            let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                            annot.children.push(AnnotChild::P(Box::new(p)));
                        }
                        "ref" => {
                            let ref_elem = parse_ref_from_event(reader, child_attrs, child_empty)?;
                            annot.children.push(AnnotChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            annot.children.push(AnnotChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            annot.children.push(AnnotChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            annot.children.push(AnnotChild::Lb(Box::new(lb)));
                        }
                        // Skip unknown children
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

    Ok(annot)
}

//! Decoration and binding elements (DecoDesc, DecoNote, BindingDesc, Binding, SealDesc, Seal).
//!
//! These elements describe decorative features, bindings, and seals of manuscript materials.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{AttContemporary, AttDatable};
use tusk_model::elements::{
    Binding, BindingChild, BindingDesc, BindingDescChild, DecoDesc, DecoDescChild, DecoNote,
    DecoNoteChild, Seal, SealChild, SealDesc, SealDescChild,
};

use super::super::extract_attr;

// ============================================================================
// Attribute class implementations
// ============================================================================

impl ExtractAttributes for AttContemporary {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "contemporary", self.contemporary);
        Ok(())
    }
}

// Note: AttDatable is already implemented in mod.rs

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for DecoDesc {
    fn element_name() -> &'static str {
        "decoDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_deco_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for DecoNote {
    fn element_name() -> &'static str {
        "decoNote"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_deco_note_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for BindingDesc {
    fn element_name() -> &'static str {
        "bindingDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_binding_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Binding {
    fn element_name() -> &'static str {
        "binding"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_binding_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SealDesc {
    fn element_name() -> &'static str {
        "sealDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_seal_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Seal {
    fn element_name() -> &'static str {
        "seal"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_seal_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<decoDesc>` element.
///
/// DecoDesc contains a description of the decoration of an item.
pub(crate) fn parse_deco_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<DecoDesc> {
    let mut elem = DecoDesc::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("decoDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(DecoDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "decoNote" => {
                            let child =
                                parse_deco_note_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoDescChild::DecoNote(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoDescChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoDescChild::P(Box::new(child)));
                        }
                        "condition" => {
                            let child = super::phys_desc::parse_condition_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(DecoDescChild::Condition(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(DecoDescChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Num(Box::new(child)));
                        }
                        "fig" => {
                            let child = super::super::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Fig(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoDescChild::Annot(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoDescChild::Bibl(Box::new(child)));
                        }
                        "biblStruct" => {
                            let child = super::parse_bibl_struct_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(DecoDescChild::BiblStruct(Box::new(child)));
                        }
                        "term" => {
                            let child = super::super::parse_term_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoDescChild::Term(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<decoNote>` element.
///
/// DecoNote contains a description of one or more decorative features of an item.
pub(crate) fn parse_deco_note_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<DecoNote> {
    let mut elem = DecoNote::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("decoNote")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(DecoNoteChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "decoNote" => {
                            let child =
                                parse_deco_note_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoNoteChild::DecoNote(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoNoteChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoNoteChild::P(Box::new(child)));
                        }
                        "condition" => {
                            let child = super::phys_desc::parse_condition_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(DecoNoteChild::Condition(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(DecoNoteChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Num(Box::new(child)));
                        }
                        "fig" => {
                            let child = super::super::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Fig(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoNoteChild::Annot(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DecoNoteChild::Bibl(Box::new(child)));
                        }
                        "biblStruct" => {
                            let child = super::parse_bibl_struct_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(DecoNoteChild::BiblStruct(Box::new(child)));
                        }
                        "term" => {
                            let child = super::super::parse_term_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DecoNoteChild::Term(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<bindingDesc>` element.
///
/// BindingDesc describes the present and former bindings of an item.
pub(crate) fn parse_binding_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<BindingDesc> {
    let mut elem = BindingDesc::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("bindingDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(BindingDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "binding" => {
                            let child = parse_binding_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(BindingDescChild::Binding(Box::new(child)));
                        }
                        "decoNote" => {
                            let child =
                                parse_deco_note_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(BindingDescChild::DecoNote(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(BindingDescChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(BindingDescChild::P(Box::new(child)));
                        }
                        "condition" => {
                            let child = super::phys_desc::parse_condition_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(BindingDescChild::Condition(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(BindingDescChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(BindingDescChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Num(Box::new(child)));
                        }
                        "fig" => {
                            let child = super::super::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Fig(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(BindingDescChild::Annot(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(BindingDescChild::Bibl(Box::new(child)));
                        }
                        "biblStruct" => {
                            let child = super::parse_bibl_struct_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(BindingDescChild::BiblStruct(Box::new(child)));
                        }
                        "term" => {
                            let child = super::super::parse_term_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(BindingDescChild::Term(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<binding>` element.
///
/// Binding contains a description of one binding (type of covering, boards, etc.).
pub(crate) fn parse_binding_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Binding> {
    let mut elem = Binding::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.contemporary.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse children (no text content, specific child elements)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("binding")?
        {
            match name.as_str() {
                "dimensions" => {
                    let child = super::phys_desc::parse_dimensions_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children
                        .push(BindingChild::Dimensions(Box::new(child)));
                }
                "p" => {
                    let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BindingChild::P(Box::new(child)));
                }
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BindingChild::Head(Box::new(child)));
                }
                "condition" => {
                    let child = super::phys_desc::parse_condition_from_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(BindingChild::Condition(Box::new(child)));
                }
                "decoNote" => {
                    let child = parse_deco_note_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BindingChild::DecoNote(Box::new(child)));
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

/// Parse a `<sealDesc>` element.
///
/// SealDesc describes the seals or similar external attachments applied to an item.
pub(crate) fn parse_seal_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SealDesc> {
    let mut elem = SealDesc::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("sealDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(SealDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "seal" => {
                            let child = parse_seal_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealDescChild::Seal(Box::new(child)));
                        }
                        "decoNote" => {
                            let child =
                                parse_deco_note_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealDescChild::DecoNote(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealDescChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealDescChild::P(Box::new(child)));
                        }
                        "condition" => {
                            let child = super::phys_desc::parse_condition_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(SealDescChild::Condition(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(SealDescChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Num(Box::new(child)));
                        }
                        "fig" => {
                            let child = super::super::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Fig(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealDescChild::Annot(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealDescChild::Bibl(Box::new(child)));
                        }
                        "biblStruct" => {
                            let child = super::parse_bibl_struct_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(SealDescChild::BiblStruct(Box::new(child)));
                        }
                        "term" => {
                            let child = super::super::parse_term_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealDescChild::Term(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<seal>` element.
///
/// Seal describes a single seal or similar attachment.
pub(crate) fn parse_seal_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Seal> {
    let mut elem = Seal::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.contemporary.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("seal")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(SealChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "decoNote" => {
                            let child =
                                parse_deco_note_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealChild::DecoNote(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealChild::P(Box::new(child)));
                        }
                        "condition" => {
                            let child = super::phys_desc::parse_condition_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Condition(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Num(Box::new(child)));
                        }
                        "fig" => {
                            let child = super::super::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Fig(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealChild::Annot(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealChild::Bibl(Box::new(child)));
                        }
                        "biblStruct" => {
                            let child = super::parse_bibl_struct_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::BiblStruct(Box::new(child)));
                        }
                        "term" => {
                            let child = super::super::parse_term_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SealChild::Term(Box::new(child)));
                        }
                        "date" => {
                            let child =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SealChild::Date(Box::new(child)));
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

    Ok(elem)
}

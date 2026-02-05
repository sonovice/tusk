//! Layout, hand, and script description elements.
//!
//! Contains: LayoutDesc, Layout, ColLayout, HandList, Hand, ScriptDesc, ScriptNote.

use super::super::extract_attr;
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    ColLayout, Hand, HandChild, HandList, HandListChild, Layout, LayoutChild, LayoutDesc,
    LayoutDescChild, ScriptDesc, ScriptDescChild, ScriptNote, ScriptNoteChild,
};

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for LayoutDesc {
    fn element_name() -> &'static str {
        "layoutDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_layout_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Layout {
    fn element_name() -> &'static str {
        "layout"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_layout_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ColLayout {
    fn element_name() -> &'static str {
        "colLayout"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_col_layout_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for HandList {
    fn element_name() -> &'static str {
        "handList"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_hand_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Hand {
    fn element_name() -> &'static str {
        "hand"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_hand_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ScriptDesc {
    fn element_name() -> &'static str {
        "scriptDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_script_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ScriptNote {
    fn element_name() -> &'static str {
        "scriptNote"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_script_note_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<layoutDesc>` element.
///
/// LayoutDesc collects layout descriptions for a manuscript.
pub(crate) fn parse_layout_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LayoutDesc> {
    let mut elem = LayoutDesc::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("layoutDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(LayoutDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "layout" => {
                            let child = parse_layout_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutDescChild::Layout(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutDescChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutDescChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(LayoutDescChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutDescChild::Num(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutDescChild::Bibl(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutDescChild::Annot(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<layout>` element.
///
/// Layout describes how text is laid out on the page.
pub(crate) fn parse_layout_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Layout> {
    let mut elem = Layout::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Element-specific attributes
    extract_attr!(attrs, "cols", elem.cols);
    extract_attr!(attrs, "ruledlines", elem.ruledlines);
    extract_attr!(attrs, "writtenlines", elem.writtenlines);
    extract_attr!(attrs, "ruledstaves", elem.ruledstaves);
    extract_attr!(attrs, "writtenstaves", elem.writtenstaves);

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("layout")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(LayoutChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Num(Box::new(child)));
                        }
                        "fig" => {
                            let child = super::super::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(LayoutChild::Fig(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutChild::Bibl(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(LayoutChild::Annot(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<colLayout>` element.
///
/// ColLayout is an empty formatting element that signals the start of columnar layout.
pub(crate) fn parse_col_layout_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ColLayout> {
    let mut elem = ColLayout::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.source.extract_attributes(&mut attrs)?;

    // Element-specific attribute
    extract_attr!(attrs, "cols", elem.cols);

    // ColLayout is typically empty, but skip any content if present
    if !is_empty {
        reader.skip_to_end("colLayout")?;
    }

    Ok(elem)
}

/// Parse a `<handList>` element.
///
/// HandList is a container for one or more hand elements.
pub(crate) fn parse_hand_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<HandList> {
    let mut elem = HandList::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;

    // Parse children (not mixed content - just specific child elements)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("handList")?
        {
            match name.as_str() {
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(HandListChild::Head(Box::new(child)));
                }
                "label" => {
                    let child =
                        super::super::parse_label_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(HandListChild::Label(Box::new(child)));
                }
                "hand" => {
                    let child = parse_hand_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(HandListChild::Hand(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<hand>` element.
///
/// Hand defines a distinct scribe or handwriting style.
pub(crate) fn parse_hand_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Hand> {
    let mut elem = Hand::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.evidence.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.medium.extract_attributes(&mut attrs)?;

    // Element-specific attribute
    extract_attr!(attrs, "initial", elem.initial);

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("hand")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(HandChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::Num(Box::new(child)));
                        }
                        "persName" => {
                            let child = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HandChild::PersName(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HandChild::Bibl(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HandChild::Annot(Box::new(child)));
                        }
                        "date" => {
                            let child =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HandChild::Date(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<scriptDesc>` element.
///
/// ScriptDesc contains a description of the letters or characters used in an autographic item.
pub(crate) fn parse_script_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ScriptDesc> {
    let mut elem = ScriptDesc::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("scriptDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(ScriptDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "scriptNote" => {
                            let child =
                                parse_script_note_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(ScriptDescChild::ScriptNote(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptDescChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptDescChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(ScriptDescChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptDescChild::Num(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptDescChild::Bibl(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptDescChild::Annot(Box::new(child)));
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

    Ok(elem)
}

/// Parse a `<scriptNote>` element.
///
/// ScriptNote describes a particular script distinguished within the description of an autographic item.
pub(crate) fn parse_script_note_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ScriptNote> {
    let mut elem = ScriptNote::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("scriptNote")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(ScriptNoteChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptNoteChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptNoteChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child = super::phys_desc::parse_dimensions_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children
                                .push(ScriptNoteChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = super::phys_desc::parse_height_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = super::phys_desc::parse_width_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = super::phys_desc::parse_depth_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = super::phys_desc::parse_dim_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ScriptNoteChild::Num(Box::new(child)));
                        }
                        "bibl" => {
                            let child =
                                super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptNoteChild::Bibl(Box::new(child)));
                        }
                        "annot" => {
                            let child =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptNoteChild::Annot(Box::new(child)));
                        }
                        "date" => {
                            let child =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ScriptNoteChild::Date(Box::new(child)));
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

    Ok(elem)
}

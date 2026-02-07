//! Deserializer implementations for text and prose MEI elements.
//!
//! This module contains implementations for Annot, Rend, Lg, Fig, FigDesc, Verse, List, Li, Seg,
//! Table, Tr, Td, Th, Caption, Front, Back, TitlePage, Argument, Epigraph, Dedication, Imprimatur,
//! Colophon and related attribute classes.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAnnotAnl, AttAnnotGes, AttAnnotLog, AttAnnotVis, AttAudience, AttColor, AttExtSymAuth,
    AttHorizontalAlign, AttLyricsAnl, AttLyricsGes, AttLyricsLog, AttLyricsVis, AttPlist,
    AttSource, AttSylAnl, AttSylGes, AttSylLog, AttSylVis, AttTabular, AttTypography, AttVerseAnl,
    AttVerseGes, AttVerseLog, AttVerseVis, AttVerticalAlign,
};
use tusk_model::elements::{
    Annot, Argument, ArgumentChild, Back, BackChild, Caption, CaptionChild, Colophon,
    ColophonChild, Dedication, DedicationChild, Div, DivChild, Epigraph, EpigraphChild, Fig,
    FigChild, FigDesc, Front, FrontChild, Imprimatur, ImprimaturChild, Lb, Lg, LgChild, Li,
    LiChild, List, ListChild, Rend, Seg, SegChild, Syl, SylChild, Table, TableChild, Td, TdChild,
    Th, ThChild, TitlePage, TitlePageChild, Tr, TrChild, Verse, VerseChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for Annot {
    fn element_name() -> &'static str {
        "annot"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
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

        // Annot has many possible children - for now we just collect text content
        // and skip other children in lenient mode
        if !is_empty {
            if let Some(text) = reader.read_text_until_end("annot")? {
                if !text.trim().is_empty() {
                    annot
                        .children
                        .push(tusk_model::elements::AnnotChild::Text(text));
                }
            }
        }

        Ok(annot)
    }
}

impl MeiDeserialize for Rend {
    fn element_name() -> &'static str {
        "rend"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_rend_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<rend>` element from within another element.
///
/// Rend has mixed content - text and various child elements including nested rend, lb, etc.
pub(crate) fn parse_rend_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Rend> {
    use tusk_model::elements::RendChild;

    let mut rend = Rend::default();

    // Extract attributes
    rend.color.extract_attributes(&mut attrs)?;
    rend.common.extract_attributes(&mut attrs)?;
    rend.ext_sym_auth.extract_attributes(&mut attrs)?;
    rend.horizontal_align.extract_attributes(&mut attrs)?;
    rend.lang.extract_attributes(&mut attrs)?;
    rend.text_rendition.extract_attributes(&mut attrs)?;
    rend.typography.extract_attributes(&mut attrs)?;
    rend.vertical_align.extract_attributes(&mut attrs)?;
    rend.whitespace.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "rotation", rend.rotation);

    // Rend has mixed content - text and various child elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("rend")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        rend.children.push(RendChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            rend.children.push(RendChild::Lb(Box::new(lb)));
                        }
                        "rend" => {
                            let nested_rend =
                                parse_rend_from_event(reader, child_attrs, child_empty)?;
                            rend.children.push(RendChild::Rend(Box::new(nested_rend)));
                        }
                        "persName" => {
                            let pers = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::PersName(Box::new(pers)));
                        }
                        "corpName" => {
                            let corp = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::CorpName(Box::new(corp)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::Ptr(Box::new(ptr)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children
                                .push(RendChild::Identifier(Box::new(identifier)));
                        }
                        "symbol" => {
                            let symbol = super::control::parse_symbol_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            rend.children.push(RendChild::Symbol(Box::new(symbol)));
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

    Ok(rend)
}

/// Parse a `<lb>` (line break) element from within another element.
///
/// Lb is an empty element with only attributes, no children.
pub(crate) fn parse_lb_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Lb> {
    let mut lb = Lb::default();

    // Extract attributes
    lb.common.extract_attributes(&mut attrs)?;
    lb.facsimile.extract_attributes(&mut attrs)?;
    lb.source.extract_attributes(&mut attrs)?;

    // lb is an empty element, but we need to consume the end tag if not self-closing
    if !is_empty {
        reader.skip_to_end("lb")?;
    }

    Ok(lb)
}

impl MeiDeserialize for Lg {
    fn element_name() -> &'static str {
        "lg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut lg = Lg::default();

        // Extract attributes
        lg.common.extract_attributes(&mut attrs)?;
        lg.facsimile.extract_attributes(&mut attrs)?;
        lg.lang.extract_attributes(&mut attrs)?;
        lg.metadata_pointing.extract_attributes(&mut attrs)?;
        lg.xy.extract_attributes(&mut attrs)?;
        lg.lyrics_anl.extract_attributes(&mut attrs)?;
        lg.lyrics_ges.extract_attributes(&mut attrs)?;
        lg.lyrics_log.extract_attributes(&mut attrs)?;
        lg.lyrics_vis.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // Lg can contain: l*, head*, lg*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("lg")? {
                match name.as_str() {
                    "l" => {
                        let l = parse_l_from_event(reader, child_attrs, child_empty)?;
                        lg.children.push(LgChild::L(Box::new(l)));
                    }
                    "head" => {
                        let head =
                            super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
                        lg.children.push(LgChild::Head(Box::new(head)));
                    }
                    "lg" => {
                        let nested_lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                        lg.children.push(LgChild::Lg(Box::new(nested_lg)));
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

        Ok(lg)
    }
}

impl MeiDeserialize for Fig {
    fn element_name() -> &'static str {
        "fig"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut fig = Fig::default();

        // Extract attributes
        fig.common.extract_attributes(&mut attrs)?;
        fig.facsimile.extract_attributes(&mut attrs)?;
        fig.horizontal_align.extract_attributes(&mut attrs)?;
        fig.vertical_align.extract_attributes(&mut attrs)?;
        fig.xy.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // Fig can contain: figDesc*, caption*, score*, graphic*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("fig")?
            {
                match name.as_str() {
                    "figDesc" => {
                        let fig_desc = parse_fig_desc_from_event(reader, child_attrs, child_empty)?;
                        fig.children.push(FigChild::FigDesc(Box::new(fig_desc)));
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

        Ok(fig)
    }
}

/// Parse a `<fig>` element from within another element.
pub(crate) fn parse_fig_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Fig> {
    Fig::from_mei_event(reader, attrs, is_empty)
}

/// Parse a `<lg>` element from within another element.
pub(crate) fn parse_lg_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Lg> {
    Lg::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for FigDesc {
    fn element_name() -> &'static str {
        "figDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_fig_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<figDesc>` element from within another element.
pub(crate) fn parse_fig_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<FigDesc> {
    let mut fig_desc = FigDesc::default();

    // Extract attributes
    fig_desc.common.extract_attributes(&mut attrs)?;
    fig_desc.lang.extract_attributes(&mut attrs)?;

    // FigDesc can have text content and many child elements
    // For now just collect text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("figDesc")? {
            if !text.trim().is_empty() {
                fig_desc
                    .children
                    .push(tusk_model::elements::FigDescChild::Text(text));
            }
        }
    }

    Ok(fig_desc)
}

impl MeiDeserialize for Verse {
    fn element_name() -> &'static str {
        "verse"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut verse = Verse::default();

        // Extract attributes
        verse.common.extract_attributes(&mut attrs)?;
        verse.facsimile.extract_attributes(&mut attrs)?;
        verse.lang.extract_attributes(&mut attrs)?;
        verse.verse_log.extract_attributes(&mut attrs)?;
        verse.verse_vis.extract_attributes(&mut attrs)?;
        verse.verse_ges.extract_attributes(&mut attrs)?;
        verse.verse_anl.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // Verse can contain: subst*, lb*, choice*, volta*, label*, space*, app*, labelAbbr*, dynam*, syl*, dir*, tempo*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("verse")?
            {
                match name.as_str() {
                    "syl" => {
                        let syl = Syl::from_mei_event(reader, child_attrs, child_empty)?;
                        verse.children.push(VerseChild::Syl(Box::new(syl)));
                    }
                    "lb" => {
                        let lb = Lb::from_mei_event(reader, child_attrs, child_empty)?;
                        verse.children.push(VerseChild::Lb(Box::new(lb)));
                    }
                    // Other verse children can be added here as needed
                    _ => {
                        // Unknown child element - skip in lenient mode
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(verse)
    }
}

impl MeiDeserialize for Lb {
    fn element_name() -> &'static str {
        "lb"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut lb = Lb::default();

        // Extract attributes
        lb.common.extract_attributes(&mut attrs)?;
        lb.facsimile.extract_attributes(&mut attrs)?;
        lb.source.extract_attributes(&mut attrs)?;

        // Lb is an empty element per MEI spec
        if !is_empty {
            reader.skip_to_end("lb")?;
        }

        Ok(lb)
    }
}

impl MeiDeserialize for Syl {
    fn element_name() -> &'static str {
        "syl"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut syl = Syl::default();

        // Extract attributes
        syl.common.extract_attributes(&mut attrs)?;
        syl.facsimile.extract_attributes(&mut attrs)?;
        syl.lang.extract_attributes(&mut attrs)?;
        syl.syl_log.extract_attributes(&mut attrs)?;
        syl.syl_vis.extract_attributes(&mut attrs)?;
        syl.syl_ges.extract_attributes(&mut attrs)?;
        syl.syl_anl.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // Syl is typically mixed content with text and possibly other elements
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("syl")? {
                match content {
                    MixedContent::Text(text) => {
                        syl.children.push(SylChild::Text(text));
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                                syl.children.push(SylChild::Rend(Box::new(rend)));
                            }
                            // Syl can contain many child elements, skip others in lenient mode
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

        Ok(syl)
    }
}

impl MeiDeserialize for List {
    fn element_name() -> &'static str {
        "list"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_list_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<list>` element from within another element.
///
/// List can contain: head*, li*, label*
pub(crate) fn parse_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<List> {
    let mut list = List::default();

    // Extract attributes
    list.basic.extract_attributes(&mut attrs)?;
    list.classed.extract_attributes(&mut attrs)?;
    list.facsimile.extract_attributes(&mut attrs)?;
    list.labelled.extract_attributes(&mut attrs)?;
    list.lang.extract_attributes(&mut attrs)?;
    list.linking.extract_attributes(&mut attrs)?;
    list.n_number_like.extract_attributes(&mut attrs)?;
    list.responsibility.extract_attributes(&mut attrs)?;
    list.xy.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "form", string list.form);
    extract_attr!(attrs, "type", string list.r#type);

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("list")? {
            match name.as_str() {
                "head" => {
                    let head =
                        super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
                    list.children.push(ListChild::Head(Box::new(head)));
                }
                "li" => {
                    let li = parse_li_from_event(reader, child_attrs, child_empty)?;
                    list.children.push(ListChild::Li(Box::new(li)));
                }
                "label" => {
                    let label = super::parse_label_from_event(reader, child_attrs, child_empty)?;
                    list.children.push(ListChild::Label(Box::new(label)));
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

    Ok(list)
}

impl MeiDeserialize for Li {
    fn element_name() -> &'static str {
        "li"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_li_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<li>` (list item) element from within another element.
///
/// Li can contain mixed content with text and many child elements.
pub(crate) fn parse_li_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Li> {
    let mut li = Li::default();

    // Extract attributes
    li.common.extract_attributes(&mut attrs)?;
    li.facsimile.extract_attributes(&mut attrs)?;
    li.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("li")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve all text content
                    if !text.trim().is_empty() {
                        li.children.push(LiChild::Text(text));
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
                            li.children.push(LiChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            li.children.push(LiChild::Rend(Box::new(rend)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name = super::header::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::GeogName(Box::new(geog_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Title(Box::new(title)));
                        }
                        "annot" => {
                            let annot = super::header::parse_annot_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Annot(Box::new(annot)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            li.children.push(LiChild::Lb(Box::new(lb)));
                        }
                        "bibl" => {
                            let bibl = super::header::parse_bibl_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Bibl(Box::new(bibl)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::Identifier(Box::new(identifier)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            li.children.push(LiChild::P(Box::new(p)));
                        }
                        "list" => {
                            let nested_list =
                                parse_list_from_event(reader, child_attrs, child_empty)?;
                            li.children.push(LiChild::List(Box::new(nested_list)));
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

    Ok(li)
}

impl MeiDeserialize for Seg {
    fn element_name() -> &'static str {
        "seg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_seg_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<seg>` element from within another element.
///
/// Seg (arbitrary segment) can contain mixed content with text and many child elements.
/// It represents any segmentation of text below the "text component" level.
pub(crate) fn parse_seg_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Seg> {
    let mut seg = Seg::default();

    // Extract attributes
    seg.common.extract_attributes(&mut attrs)?;
    seg.facsimile.extract_attributes(&mut attrs)?;
    seg.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("seg")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve all text content including whitespace-only
                    if !text.is_empty() {
                        seg.children.push(SegChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            seg.children.push(SegChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            seg.children.push(SegChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children.push(SegChild::Ptr(Box::new(ptr)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            seg.children
                                .push(SegChild::Identifier(Box::new(identifier)));
                        }
                        "seg" => {
                            // Nested seg element
                            let nested_seg =
                                parse_seg_from_event(reader, child_attrs, child_empty)?;
                            seg.children.push(SegChild::Seg(Box::new(nested_seg)));
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

    Ok(seg)
}

// ============================================================================
// Div element implementation
// ============================================================================

/// Parse a <div> element from a start event.
/// The div element contains prose text such as libretto, editorial remarks, etc.
pub(crate) fn parse_div_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Div> {
    let mut div = Div::default();

    // Extract attributes
    div.basic.extract_attributes(&mut attrs)?;
    div.classed.extract_attributes(&mut attrs)?;
    div.facsimile.extract_attributes(&mut attrs)?;
    div.labelled.extract_attributes(&mut attrs)?;
    div.lang.extract_attributes(&mut attrs)?;
    div.linking.extract_attributes(&mut attrs)?;
    div.metadata_pointing.extract_attributes(&mut attrs)?;
    div.n_number_like.extract_attributes(&mut attrs)?;
    div.responsibility.extract_attributes(&mut attrs)?;

    // Element-local attribute
    div.r#type = attrs.remove("type");

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("div")? {
            match name.as_str() {
                "p" => {
                    let p = super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
                    div.children.push(DivChild::P(Box::new(p)));
                }
                "list" => {
                    let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                    div.children.push(DivChild::List(Box::new(list)));
                }
                "head" => {
                    let head =
                        super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
                    div.children.push(DivChild::Head(Box::new(head)));
                }
                "lg" => {
                    let lg = parse_lg_from_event(reader, child_attrs, child_empty)?;
                    div.children.push(DivChild::Lg(Box::new(lg)));
                }
                // Other child types can be added as needed
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(div)
}

// ============================================================================
// Table elements (table, tr, td, th, caption)
// ============================================================================

impl MeiDeserialize for Table {
    fn element_name() -> &'static str {
        "table"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_table_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<table>` element from within another element.
///
/// Table can contain caption* and tr* elements.
pub(crate) fn parse_table_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Table> {
    let mut table = Table::default();

    // Extract attributes
    table.common.extract_attributes(&mut attrs)?;
    table.facsimile.extract_attributes(&mut attrs)?;
    table.lang.extract_attributes(&mut attrs)?;
    table.xy.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("table")? {
            match name.as_str() {
                "caption" => {
                    let caption = parse_caption_from_event(reader, child_attrs, child_empty)?;
                    table.children.push(TableChild::Caption(Box::new(caption)));
                }
                "tr" => {
                    let tr = parse_tr_from_event(reader, child_attrs, child_empty)?;
                    table.children.push(TableChild::Tr(Box::new(tr)));
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

    Ok(table)
}

impl MeiDeserialize for Tr {
    fn element_name() -> &'static str {
        "tr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_tr_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<tr>` (table row) element from within another element.
///
/// Tr can contain td* and th* elements.
pub(crate) fn parse_tr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Tr> {
    let mut tr = Tr::default();

    // Extract attributes
    tr.common.extract_attributes(&mut attrs)?;
    tr.facsimile.extract_attributes(&mut attrs)?;
    tr.lang.extract_attributes(&mut attrs)?;
    tr.xy.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("tr")? {
            match name.as_str() {
                "td" => {
                    let td = parse_td_from_event(reader, child_attrs, child_empty)?;
                    tr.children.push(TrChild::Td(Box::new(td)));
                }
                "th" => {
                    let th = parse_th_from_event(reader, child_attrs, child_empty)?;
                    tr.children.push(TrChild::Th(Box::new(th)));
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

    Ok(tr)
}

impl MeiDeserialize for Td {
    fn element_name() -> &'static str {
        "td"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_td_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<td>` (table data cell) element from within another element.
///
/// Td has mixed content with text and many possible child elements.
pub(crate) fn parse_td_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Td> {
    let mut td = Td::default();

    // Extract attributes
    td.common.extract_attributes(&mut attrs)?;
    td.facsimile.extract_attributes(&mut attrs)?;
    td.lang.extract_attributes(&mut attrs)?;
    td.xy.extract_attributes(&mut attrs)?;
    td.tabular.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("td")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        td.children.push(TdChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            td.children.push(TdChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            td.children.push(TdChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::Ptr(Box::new(ptr)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::Identifier(Box::new(identifier)));
                        }
                        "seg" => {
                            let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                            td.children.push(TdChild::Seg(Box::new(seg)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            td.children.push(TdChild::P(Box::new(p)));
                        }
                        "list" => {
                            let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                            td.children.push(TdChild::List(Box::new(list)));
                        }
                        "table" => {
                            // Nested table
                            let nested_table =
                                parse_table_from_event(reader, child_attrs, child_empty)?;
                            td.children.push(TdChild::Table(Box::new(nested_table)));
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

    Ok(td)
}

impl MeiDeserialize for Th {
    fn element_name() -> &'static str {
        "th"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_th_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<th>` (table header cell) element from within another element.
///
/// Th has mixed content with text and many possible child elements.
pub(crate) fn parse_th_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Th> {
    let mut th = Th::default();

    // Extract attributes
    th.common.extract_attributes(&mut attrs)?;
    th.facsimile.extract_attributes(&mut attrs)?;
    th.lang.extract_attributes(&mut attrs)?;
    th.xy.extract_attributes(&mut attrs)?;
    th.tabular.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("th")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        th.children.push(ThChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            th.children.push(ThChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            th.children.push(ThChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::Ptr(Box::new(ptr)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::Identifier(Box::new(identifier)));
                        }
                        "seg" => {
                            let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                            th.children.push(ThChild::Seg(Box::new(seg)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            th.children.push(ThChild::P(Box::new(p)));
                        }
                        "list" => {
                            let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                            th.children.push(ThChild::List(Box::new(list)));
                        }
                        "table" => {
                            // Nested table
                            let nested_table =
                                parse_table_from_event(reader, child_attrs, child_empty)?;
                            th.children.push(ThChild::Table(Box::new(nested_table)));
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

    Ok(th)
}

impl MeiDeserialize for Caption {
    fn element_name() -> &'static str {
        "caption"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_caption_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<caption>` element from within another element.
///
/// Caption has mixed content with text and many possible child elements.
pub(crate) fn parse_caption_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Caption> {
    let mut caption = Caption::default();

    // Extract attributes
    caption.common.extract_attributes(&mut attrs)?;
    caption.facsimile.extract_attributes(&mut attrs)?;
    caption.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("caption")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        caption.children.push(CaptionChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            caption.children.push(CaptionChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            caption.children.push(CaptionChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption
                                .children
                                .push(CaptionChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption
                                .children
                                .push(CaptionChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption
                                .children
                                .push(CaptionChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption.children.push(CaptionChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption.children.push(CaptionChild::Date(Box::new(date)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption.children.push(CaptionChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption.children.push(CaptionChild::Ptr(Box::new(ptr)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            caption
                                .children
                                .push(CaptionChild::Identifier(Box::new(identifier)));
                        }
                        "seg" => {
                            let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                            caption.children.push(CaptionChild::Seg(Box::new(seg)));
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

    Ok(caption)
}

// ============================================================================
// L (line of text) element implementation
// ============================================================================

impl MeiDeserialize for tusk_model::elements::L {
    fn element_name() -> &'static str {
        "l"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_l_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<l>` (line of text) element from within another element.
///
/// L (line) is used within lg elements to mark individual lines of verse.
/// It can contain mixed content (text and various child elements).
pub(crate) fn parse_l_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::L> {
    use tusk_model::elements::{L, LChild};

    let mut l = L::default();

    // Extract attributes
    l.common.extract_attributes(&mut attrs)?;
    l.facsimile.extract_attributes(&mut attrs)?;
    l.lang.extract_attributes(&mut attrs)?;

    // Element-local attribute
    l.rhythm = attrs.remove("rhythm");

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("l")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content
                    if !text.is_empty() {
                        l.children.push(LChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "syl" => {
                            let syl = Syl::from_mei_event(reader, child_attrs, child_empty)?;
                            l.children.push(LChild::Syl(Box::new(syl)));
                        }
                        "rend" => {
                            let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                            l.children.push(LChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                            l.children.push(LChild::Lb(Box::new(lb)));
                        }
                        "seg" => {
                            let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                            l.children.push(LChild::Seg(Box::new(seg)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Ptr(Box::new(ptr)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::CorpName(Box::new(corp_name)));
                        }
                        "geogName" => {
                            let geog_name = super::header::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::GeogName(Box::new(geog_name)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Identifier(Box::new(identifier)));
                        }
                        "annot" => {
                            let annot = super::header::parse_annot_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Annot(Box::new(annot)));
                        }
                        "bibl" => {
                            let bibl = super::header::parse_bibl_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            l.children.push(LChild::Bibl(Box::new(bibl)));
                        }
                        "fig" => {
                            let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                            l.children.push(LChild::Fig(Box::new(fig)));
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

    Ok(l)
}

// ============================================================================
// Front/Back Matter element implementations
// ============================================================================

impl MeiDeserialize for Front {
    fn element_name() -> &'static str {
        "front"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut front = Front::default();

        // Extract attributes
        front.common.extract_attributes(&mut attrs)?;
        front.facsimile.extract_attributes(&mut attrs)?;
        front.lang.extract_attributes(&mut attrs)?;
        front.metadata_pointing.extract_attributes(&mut attrs)?;

        // Read children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("front")?
            {
                match name.as_str() {
                    "titlePage" => {
                        let tp = TitlePage::from_mei_event(reader, child_attrs, child_empty)?;
                        front.children.push(FrontChild::TitlePage(Box::new(tp)));
                    }
                    "div" => {
                        let div = parse_div_from_event(reader, child_attrs, child_empty)?;
                        front.children.push(FrontChild::Div(Box::new(div)));
                    }
                    "lb" => {
                        let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        front.children.push(FrontChild::Lb(Box::new(lb)));
                    }
                    "pb" => {
                        let pb = super::structure::parse_pb_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        front.children.push(FrontChild::Pb(Box::new(pb)));
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

        Ok(front)
    }
}

/// Parse a `<front>` element from within another element.
pub(crate) fn parse_front_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Front> {
    Front::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Back {
    fn element_name() -> &'static str {
        "back"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut back = Back::default();

        // Extract attributes
        back.common.extract_attributes(&mut attrs)?;
        back.facsimile.extract_attributes(&mut attrs)?;
        back.lang.extract_attributes(&mut attrs)?;
        back.metadata_pointing.extract_attributes(&mut attrs)?;

        // Read children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("back")?
            {
                match name.as_str() {
                    "titlePage" => {
                        let tp = TitlePage::from_mei_event(reader, child_attrs, child_empty)?;
                        back.children.push(BackChild::TitlePage(Box::new(tp)));
                    }
                    "div" => {
                        let div = parse_div_from_event(reader, child_attrs, child_empty)?;
                        back.children.push(BackChild::Div(Box::new(div)));
                    }
                    "lb" => {
                        let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        back.children.push(BackChild::Lb(Box::new(lb)));
                    }
                    "pb" => {
                        let pb = super::structure::parse_pb_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        back.children.push(BackChild::Pb(Box::new(pb)));
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

        Ok(back)
    }
}

/// Parse a `<back>` element from within another element.
pub(crate) fn parse_back_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Back> {
    Back::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for TitlePage {
    fn element_name() -> &'static str {
        "titlePage"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut title_page = TitlePage::default();

        // Extract attributes
        title_page.common.extract_attributes(&mut attrs)?;
        title_page.bibl.extract_attributes(&mut attrs)?;
        title_page.facsimile.extract_attributes(&mut attrs)?;
        title_page.lang.extract_attributes(&mut attrs)?;

        // Read children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("titlePage")?
            {
                match name.as_str() {
                    "head" => {
                        let head =
                            super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Head(Box::new(head)));
                    }
                    "p" => {
                        let p =
                            super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
                        title_page.children.push(TitlePageChild::P(Box::new(p)));
                    }
                    "title" => {
                        let title = super::header::parse_title_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        title_page
                            .children
                            .push(TitlePageChild::Title(Box::new(title)));
                    }
                    "date" => {
                        let date =
                            super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Date(Box::new(date)));
                    }
                    "identifier" => {
                        let ident = super::header::parse_identifier_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        title_page
                            .children
                            .push(TitlePageChild::Identifier(Box::new(ident)));
                    }
                    "argument" => {
                        let arg = Argument::from_mei_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Argument(Box::new(arg)));
                    }
                    "epigraph" => {
                        let epi = Epigraph::from_mei_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Epigraph(Box::new(epi)));
                    }
                    "dedication" => {
                        let ded = Dedication::from_mei_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Dedication(Box::new(ded)));
                    }
                    "imprimatur" => {
                        let imp = Imprimatur::from_mei_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Imprimatur(Box::new(imp)));
                    }
                    "lg" => {
                        let lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                        title_page.children.push(TitlePageChild::Lg(Box::new(lg)));
                    }
                    "list" => {
                        let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::List(Box::new(list)));
                    }
                    "table" => {
                        let table = parse_table_from_event(reader, child_attrs, child_empty)?;
                        title_page
                            .children
                            .push(TitlePageChild::Table(Box::new(table)));
                    }
                    "fig" => {
                        let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                        title_page.children.push(TitlePageChild::Fig(Box::new(fig)));
                    }
                    "lb" => {
                        let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        title_page.children.push(TitlePageChild::Lb(Box::new(lb)));
                    }
                    "pb" => {
                        let pb = super::structure::parse_pb_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        title_page.children.push(TitlePageChild::Pb(Box::new(pb)));
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

        Ok(title_page)
    }
}

/// Parse a `<titlePage>` element from within another element.
pub(crate) fn parse_title_page_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TitlePage> {
    TitlePage::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Argument {
    fn element_name() -> &'static str {
        "argument"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut argument = Argument::default();

        // Extract attributes
        argument.common.extract_attributes(&mut attrs)?;
        argument.facsimile.extract_attributes(&mut attrs)?;
        argument.lang.extract_attributes(&mut attrs)?;
        argument.metadata_pointing.extract_attributes(&mut attrs)?;

        // Read children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("argument")?
            {
                match name.as_str() {
                    "head" => {
                        let head =
                            super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
                        argument.children.push(ArgumentChild::Head(Box::new(head)));
                    }
                    "p" => {
                        let p =
                            super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
                        argument.children.push(ArgumentChild::P(Box::new(p)));
                    }
                    "lg" => {
                        let lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                        argument.children.push(ArgumentChild::Lg(Box::new(lg)));
                    }
                    "list" => {
                        let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                        argument.children.push(ArgumentChild::List(Box::new(list)));
                    }
                    "table" => {
                        let table = parse_table_from_event(reader, child_attrs, child_empty)?;
                        argument
                            .children
                            .push(ArgumentChild::Table(Box::new(table)));
                    }
                    "fig" => {
                        let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                        argument.children.push(ArgumentChild::Fig(Box::new(fig)));
                    }
                    "lb" => {
                        let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        argument.children.push(ArgumentChild::Lb(Box::new(lb)));
                    }
                    "pb" => {
                        let pb = super::structure::parse_pb_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        argument.children.push(ArgumentChild::Pb(Box::new(pb)));
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

        Ok(argument)
    }
}

/// Parse an `<argument>` element from within another element.
pub(crate) fn parse_argument_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Argument> {
    Argument::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Epigraph {
    fn element_name() -> &'static str {
        "epigraph"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut epigraph = Epigraph::default();

        // Extract attributes
        epigraph.common.extract_attributes(&mut attrs)?;
        epigraph.facsimile.extract_attributes(&mut attrs)?;
        epigraph.lang.extract_attributes(&mut attrs)?;
        epigraph.metadata_pointing.extract_attributes(&mut attrs)?;

        // Epigraph has mixed content (text and elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("epigraph")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            epigraph.children.push(EpigraphChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "p" => {
                                let p = super::header::parse_p_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph.children.push(EpigraphChild::P(Box::new(p)));
                            }
                            "lg" => {
                                let lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Lg(Box::new(lg)));
                            }
                            "rend" => {
                                let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Lb(Box::new(lb)));
                            }
                            "pb" => {
                                let pb = super::structure::parse_pb_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph.children.push(EpigraphChild::Pb(Box::new(pb)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::Title(Box::new(title)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph.children.push(EpigraphChild::Date(Box::new(date)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::Name(Box::new(name_elem)));
                            }
                            "persName" => {
                                let pers = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::PersName(Box::new(pers)));
                            }
                            "corpName" => {
                                let corp = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::CorpName(Box::new(corp)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::Ref(Box::new(ref_elem)));
                            }
                            "ptr" => {
                                let ptr = super::header::parse_ptr_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph.children.push(EpigraphChild::Ptr(Box::new(ptr)));
                            }
                            "identifier" => {
                                let ident = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::Identifier(Box::new(ident)));
                            }
                            "num" => {
                                let num =
                                    super::parse_num_from_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Num(Box::new(num)));
                            }
                            "seg" => {
                                let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Seg(Box::new(seg)));
                            }
                            "bibl" => {
                                let bibl =
                                    super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Bibl(Box::new(bibl)));
                            }
                            "list" => {
                                let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::List(Box::new(list)));
                            }
                            "table" => {
                                let table =
                                    parse_table_from_event(reader, child_attrs, child_empty)?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::Table(Box::new(table)));
                            }
                            "fig" => {
                                let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                                epigraph.children.push(EpigraphChild::Fig(Box::new(fig)));
                            }
                            "annot" => {
                                let annot = super::header::parse_annot_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                epigraph
                                    .children
                                    .push(EpigraphChild::Annot(Box::new(annot)));
                            }
                            "quote" => {
                                // Quote not yet implemented, skip
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
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

        Ok(epigraph)
    }
}

/// Parse an `<epigraph>` element from within another element.
pub(crate) fn parse_epigraph_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Epigraph> {
    Epigraph::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Dedication {
    fn element_name() -> &'static str {
        "dedication"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut dedication = Dedication::default();

        // Extract attributes
        dedication.common.extract_attributes(&mut attrs)?;
        dedication.bibl.extract_attributes(&mut attrs)?;
        dedication.facsimile.extract_attributes(&mut attrs)?;
        dedication.lang.extract_attributes(&mut attrs)?;

        // Dedication has mixed content (text and elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("dedication")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            dedication.children.push(DedicationChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "head" => {
                                let head = super::header::parse_head_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Head(Box::new(head)));
                            }
                            "p" => {
                                let p = super::header::parse_p_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication.children.push(DedicationChild::P(Box::new(p)));
                            }
                            "lg" => {
                                let lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                                dedication.children.push(DedicationChild::Lg(Box::new(lg)));
                            }
                            "rend" => {
                                let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                                dedication.children.push(DedicationChild::Lb(Box::new(lb)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Title(Box::new(title)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Date(Box::new(date)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Name(Box::new(name_elem)));
                            }
                            "persName" => {
                                let pers = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::PersName(Box::new(pers)));
                            }
                            "corpName" => {
                                let corp = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::CorpName(Box::new(corp)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Ref(Box::new(ref_elem)));
                            }
                            "ptr" => {
                                let ptr = super::header::parse_ptr_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Ptr(Box::new(ptr)));
                            }
                            "identifier" => {
                                let ident = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Identifier(Box::new(ident)));
                            }
                            "num" => {
                                let num =
                                    super::parse_num_from_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::Num(Box::new(num)));
                            }
                            "seg" => {
                                let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::Seg(Box::new(seg)));
                            }
                            "bibl" => {
                                let bibl =
                                    super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::Bibl(Box::new(bibl)));
                            }
                            "list" => {
                                let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::List(Box::new(list)));
                            }
                            "table" => {
                                let table =
                                    parse_table_from_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::Table(Box::new(table)));
                            }
                            "fig" => {
                                let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                                dedication
                                    .children
                                    .push(DedicationChild::Fig(Box::new(fig)));
                            }
                            "annot" => {
                                let annot = super::header::parse_annot_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dedication
                                    .children
                                    .push(DedicationChild::Annot(Box::new(annot)));
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

        Ok(dedication)
    }
}

/// Parse a `<dedication>` element from within another element.
pub(crate) fn parse_dedication_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Dedication> {
    Dedication::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Imprimatur {
    fn element_name() -> &'static str {
        "imprimatur"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut imprimatur = Imprimatur::default();

        // Extract attributes
        imprimatur.common.extract_attributes(&mut attrs)?;
        imprimatur.facsimile.extract_attributes(&mut attrs)?;
        imprimatur.lang.extract_attributes(&mut attrs)?;
        imprimatur
            .metadata_pointing
            .extract_attributes(&mut attrs)?;

        // Imprimatur has mixed content (text and elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("imprimatur")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            imprimatur.children.push(ImprimaturChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                                imprimatur.children.push(ImprimaturChild::Lb(Box::new(lb)));
                            }
                            "pb" => {
                                let pb = super::structure::parse_pb_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur.children.push(ImprimaturChild::Pb(Box::new(pb)));
                            }
                            "lg" => {
                                let lg = Lg::from_mei_event(reader, child_attrs, child_empty)?;
                                imprimatur.children.push(ImprimaturChild::Lg(Box::new(lg)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Title(Box::new(title)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Date(Box::new(date)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Name(Box::new(name_elem)));
                            }
                            "persName" => {
                                let pers = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::PersName(Box::new(pers)));
                            }
                            "corpName" => {
                                let corp = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::CorpName(Box::new(corp)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Ref(Box::new(ref_elem)));
                            }
                            "ptr" => {
                                let ptr = super::header::parse_ptr_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Ptr(Box::new(ptr)));
                            }
                            "identifier" => {
                                let ident = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Identifier(Box::new(ident)));
                            }
                            "num" => {
                                let num =
                                    super::parse_num_from_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Num(Box::new(num)));
                            }
                            "seg" => {
                                let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Seg(Box::new(seg)));
                            }
                            "bibl" => {
                                let bibl =
                                    super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Bibl(Box::new(bibl)));
                            }
                            "list" => {
                                let list = parse_list_from_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::List(Box::new(list)));
                            }
                            "table" => {
                                let table =
                                    parse_table_from_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Table(Box::new(table)));
                            }
                            "fig" => {
                                let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Fig(Box::new(fig)));
                            }
                            "annot" => {
                                let annot = super::header::parse_annot_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                imprimatur
                                    .children
                                    .push(ImprimaturChild::Annot(Box::new(annot)));
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

        Ok(imprimatur)
    }
}

/// Parse an `<imprimatur>` element from within another element.
pub(crate) fn parse_imprimatur_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Imprimatur> {
    Imprimatur::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Colophon {
    fn element_name() -> &'static str {
        "colophon"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut colophon = Colophon::default();

        // Extract attributes
        colophon.common.extract_attributes(&mut attrs)?;
        colophon.bibl.extract_attributes(&mut attrs)?;
        colophon.facsimile.extract_attributes(&mut attrs)?;
        colophon.lang.extract_attributes(&mut attrs)?;

        // Colophon has mixed content (text and elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("colophon")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            colophon.children.push(ColophonChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "head" => {
                                let head = super::header::parse_head_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon.children.push(ColophonChild::Head(Box::new(head)));
                            }
                            "p" => {
                                let p = super::header::parse_p_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon.children.push(ColophonChild::P(Box::new(p)));
                            }
                            "rend" => {
                                let rend = parse_rend_from_event(reader, child_attrs, child_empty)?;
                                colophon.children.push(ColophonChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = parse_lb_from_event(reader, child_attrs, child_empty)?;
                                colophon.children.push(ColophonChild::Lb(Box::new(lb)));
                            }
                            "title" => {
                                let title = super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::Title(Box::new(title)));
                            }
                            "date" => {
                                let date = super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon.children.push(ColophonChild::Date(Box::new(date)));
                            }
                            "name" => {
                                let name_elem = super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::Name(Box::new(name_elem)));
                            }
                            "persName" => {
                                let pers = super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::PersName(Box::new(pers)));
                            }
                            "corpName" => {
                                let corp = super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::CorpName(Box::new(corp)));
                            }
                            "ref" => {
                                let ref_elem = super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::Ref(Box::new(ref_elem)));
                            }
                            "ptr" => {
                                let ptr = super::header::parse_ptr_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon.children.push(ColophonChild::Ptr(Box::new(ptr)));
                            }
                            "identifier" => {
                                let ident = super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::Identifier(Box::new(ident)));
                            }
                            "num" => {
                                let num =
                                    super::parse_num_from_event(reader, child_attrs, child_empty)?;
                                colophon.children.push(ColophonChild::Num(Box::new(num)));
                            }
                            "seg" => {
                                let seg = parse_seg_from_event(reader, child_attrs, child_empty)?;
                                colophon.children.push(ColophonChild::Seg(Box::new(seg)));
                            }
                            "bibl" => {
                                let bibl =
                                    super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                                colophon.children.push(ColophonChild::Bibl(Box::new(bibl)));
                            }
                            "fig" => {
                                let fig = Fig::from_mei_event(reader, child_attrs, child_empty)?;
                                colophon.children.push(ColophonChild::Fig(Box::new(fig)));
                            }
                            "annot" => {
                                let annot = super::header::parse_annot_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                colophon
                                    .children
                                    .push(ColophonChild::Annot(Box::new(annot)));
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

        Ok(colophon)
    }
}

/// Parse a `<colophon>` element from within another element.
pub(crate) fn parse_colophon_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Colophon> {
    Colophon::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annot_deserializes_empty() {
        let xml = r#"<annot/>"#;
        let annot = Annot::from_mei_str(xml).expect("should deserialize");

        assert!(annot.common.xml_id.is_none());
        assert!(annot.children.is_empty());
    }

    #[test]
    fn annot_deserializes_with_xml_id() {
        let xml = r#"<annot xml:id="a1"/>"#;
        let annot = Annot::from_mei_str(xml).expect("should deserialize");

        assert_eq!(annot.common.xml_id, Some("a1".to_string()));
    }

    #[test]
    fn annot_deserializes_with_text_content() {
        let xml = r#"<annot>This is an annotation.</annot>"#;
        let annot = Annot::from_mei_str(xml).expect("should deserialize");

        assert_eq!(annot.children.len(), 1);
        match &annot.children[0] {
            tusk_model::elements::AnnotChild::Text(text) => {
                assert_eq!(text, "This is an annotation.");
            }
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn annot_deserializes_with_plist() {
        use tusk_model::generated::data::DataUri;

        let xml = r##"<annot plist="#n1 #n2"/>"##;
        let annot = Annot::from_mei_str(xml).expect("should deserialize");

        assert_eq!(annot.plist.plist.len(), 2);
        assert_eq!(annot.plist.plist[0], DataUri("#n1".to_string()));
        assert_eq!(annot.plist.plist[1], DataUri("#n2".to_string()));
    }

    #[test]
    fn rend_deserializes_empty() {
        let xml = r#"<rend/>"#;
        let rend = Rend::from_mei_str(xml).expect("should deserialize");

        assert!(rend.common.xml_id.is_none());
        assert!(rend.children.is_empty());
    }

    #[test]
    fn rend_deserializes_with_text_content() {
        let xml = r#"<rend>styled text</rend>"#;
        let rend = Rend::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rend.children.len(), 1);
        match &rend.children[0] {
            tusk_model::elements::RendChild::Text(text) => {
                assert_eq!(text, "styled text");
            }
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn rend_deserializes_with_halign() {
        use tusk_model::generated::data::DataHorizontalalignment;

        let xml = r#"<rend halign="center"/>"#;
        let rend = Rend::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            rend.horizontal_align.halign,
            Some(DataHorizontalalignment::Center)
        );
    }

    #[test]
    fn lg_deserializes_empty() {
        let xml = r#"<lg/>"#;
        let lg = Lg::from_mei_str(xml).expect("should deserialize");

        assert!(lg.common.xml_id.is_none());
        assert!(lg.children.is_empty());
    }

    #[test]
    fn lg_deserializes_with_xml_id() {
        let xml = r#"<lg xml:id="lg1"/>"#;
        let lg = Lg::from_mei_str(xml).expect("should deserialize");

        assert_eq!(lg.common.xml_id, Some("lg1".to_string()));
    }

    #[test]
    fn lg_deserializes_with_nested_lg() {
        let xml = r#"<lg xml:id="lg1">
            <lg xml:id="lg2"/>
        </lg>"#;
        let lg = Lg::from_mei_str(xml).expect("should deserialize");

        assert_eq!(lg.children.len(), 1);
        match &lg.children[0] {
            LgChild::Lg(nested) => {
                assert_eq!(nested.common.xml_id, Some("lg2".to_string()));
            }
            _ => panic!("expected Lg child"),
        }
    }

    #[test]
    fn lg_deserializes_with_head() {
        let xml = r#"<lg>
            <head>Stanza 1</head>
        </lg>"#;
        let lg = Lg::from_mei_str(xml).expect("should deserialize");

        assert_eq!(lg.children.len(), 1);
        assert!(matches!(lg.children[0], LgChild::Head(_)));
    }

    #[test]
    fn fig_deserializes_empty() {
        let xml = r#"<fig/>"#;
        let fig = Fig::from_mei_str(xml).expect("should deserialize");

        assert!(fig.common.xml_id.is_none());
        assert!(fig.children.is_empty());
    }

    #[test]
    fn fig_deserializes_with_fig_desc() {
        let xml = r#"<fig>
            <figDesc>Description of figure</figDesc>
        </fig>"#;
        let fig = Fig::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fig.children.len(), 1);
        match &fig.children[0] {
            FigChild::FigDesc(fd) => {
                assert_eq!(fd.children.len(), 1);
            }
            _ => panic!("expected FigDesc child"),
        }
    }

    #[test]
    fn fig_desc_deserializes_empty() {
        let xml = r#"<figDesc/>"#;
        let fig_desc = FigDesc::from_mei_str(xml).expect("should deserialize");

        assert!(fig_desc.common.xml_id.is_none());
        assert!(fig_desc.children.is_empty());
    }

    #[test]
    fn fig_desc_deserializes_with_text() {
        let xml = r#"<figDesc>A musical example showing the theme.</figDesc>"#;
        let fig_desc = FigDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fig_desc.children.len(), 1);
        match &fig_desc.children[0] {
            tusk_model::elements::FigDescChild::Text(text) => {
                assert_eq!(text, "A musical example showing the theme.");
            }
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn verse_deserializes_empty() {
        let xml = r#"<verse/>"#;
        let verse = Verse::from_mei_str(xml).expect("should deserialize");

        assert!(verse.common.xml_id.is_none());
        assert!(verse.children.is_empty());
    }

    #[test]
    fn verse_deserializes_with_xml_id() {
        let xml = r#"<verse xml:id="v1"/>"#;
        let verse = Verse::from_mei_str(xml).expect("should deserialize");

        assert_eq!(verse.common.xml_id, Some("v1".to_string()));
    }

    #[test]
    fn list_deserializes_empty() {
        let xml = r#"<list/>"#;
        let list = List::from_mei_str(xml).expect("should deserialize");

        assert!(list.basic.xml_id.is_none());
        assert!(list.children.is_empty());
    }

    #[test]
    fn list_deserializes_with_xml_id() {
        let xml = r#"<list xml:id="lst1"/>"#;
        let list = List::from_mei_str(xml).expect("should deserialize");

        assert_eq!(list.basic.xml_id, Some("lst1".to_string()));
    }

    #[test]
    fn list_deserializes_with_form_and_type() {
        let xml = r#"<list form="simple" type="bulleted"/>"#;
        let list = List::from_mei_str(xml).expect("should deserialize");

        assert_eq!(list.form, Some("simple".to_string()));
        assert_eq!(list.r#type, Some("bulleted".to_string()));
    }

    #[test]
    fn list_deserializes_with_li_children() {
        let xml = r#"<list>
            <li>First item</li>
            <li>Second item</li>
        </list>"#;
        let list = List::from_mei_str(xml).expect("should deserialize");

        assert_eq!(list.children.len(), 2);
        match &list.children[0] {
            ListChild::Li(li) => {
                assert_eq!(li.children.len(), 1);
                match &li.children[0] {
                    LiChild::Text(text) => assert_eq!(text, "First item"),
                    _ => panic!("expected Text child"),
                }
            }
            _ => panic!("expected Li child"),
        }
    }

    #[test]
    fn list_deserializes_with_head() {
        let xml = r#"<list>
            <head>My List</head>
            <li>Item</li>
        </list>"#;
        let list = List::from_mei_str(xml).expect("should deserialize");

        assert_eq!(list.children.len(), 2);
        assert!(matches!(list.children[0], ListChild::Head(_)));
        assert!(matches!(list.children[1], ListChild::Li(_)));
    }

    #[test]
    fn li_deserializes_empty() {
        let xml = r#"<li/>"#;
        let li = Li::from_mei_str(xml).expect("should deserialize");

        assert!(li.common.xml_id.is_none());
        assert!(li.children.is_empty());
    }

    #[test]
    fn li_deserializes_with_text() {
        let xml = r#"<li>List item text</li>"#;
        let li = Li::from_mei_str(xml).expect("should deserialize");

        assert_eq!(li.children.len(), 1);
        match &li.children[0] {
            LiChild::Text(text) => assert_eq!(text, "List item text"),
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn li_deserializes_with_nested_list() {
        let xml = r#"<li>
            <list>
                <li>Nested item</li>
            </list>
        </li>"#;
        let li = Li::from_mei_str(xml).expect("should deserialize");

        assert_eq!(li.children.len(), 1);
        match &li.children[0] {
            LiChild::List(nested) => {
                assert_eq!(nested.children.len(), 1);
            }
            _ => panic!("expected List child"),
        }
    }

    // ========================================================================
    // Front/Back Matter Tests
    // ========================================================================

    #[test]
    fn front_deserializes_empty() {
        let xml = r#"<front/>"#;
        let front = Front::from_mei_str(xml).expect("should deserialize");
        assert!(front.common.xml_id.is_none());
        assert!(front.children.is_empty());
    }

    #[test]
    fn front_deserializes_with_xml_id() {
        let xml = r#"<front xml:id="front1"/>"#;
        let front = Front::from_mei_str(xml).expect("should deserialize");
        assert_eq!(front.common.xml_id, Some("front1".to_string()));
    }

    #[test]
    fn front_deserializes_with_title_page() {
        let xml = r#"<front>
            <titlePage>
                <p>Title here</p>
            </titlePage>
        </front>"#;
        let front = Front::from_mei_str(xml).expect("should deserialize");
        assert_eq!(front.children.len(), 1);
        assert!(matches!(front.children[0], FrontChild::TitlePage(_)));
    }

    #[test]
    fn back_deserializes_empty() {
        let xml = r#"<back/>"#;
        let back = Back::from_mei_str(xml).expect("should deserialize");
        assert!(back.common.xml_id.is_none());
        assert!(back.children.is_empty());
    }

    #[test]
    fn back_deserializes_with_div() {
        let xml = r#"<back>
            <div><head>Appendix</head></div>
        </back>"#;
        let back = Back::from_mei_str(xml).expect("should deserialize");
        assert_eq!(back.children.len(), 1);
        assert!(matches!(back.children[0], BackChild::Div(_)));
    }

    #[test]
    fn title_page_deserializes_empty() {
        let xml = r#"<titlePage/>"#;
        let tp = TitlePage::from_mei_str(xml).expect("should deserialize");
        assert!(tp.common.xml_id.is_none());
        assert!(tp.children.is_empty());
    }

    #[test]
    fn title_page_deserializes_with_p_child() {
        let xml = r#"<titlePage>
            <p>A Sonata</p>
        </titlePage>"#;
        let tp = TitlePage::from_mei_str(xml).expect("should deserialize");
        assert_eq!(tp.children.len(), 1);
        assert!(matches!(tp.children[0], TitlePageChild::P(_)));
    }

    #[test]
    fn argument_deserializes_empty() {
        let xml = r#"<argument/>"#;
        let arg = Argument::from_mei_str(xml).expect("should deserialize");
        assert!(arg.common.xml_id.is_none());
        assert!(arg.children.is_empty());
    }

    #[test]
    fn argument_deserializes_with_p_child() {
        let xml = r#"<argument>
            <p>The story begins...</p>
        </argument>"#;
        let arg = Argument::from_mei_str(xml).expect("should deserialize");
        assert_eq!(arg.children.len(), 1);
        assert!(matches!(arg.children[0], ArgumentChild::P(_)));
    }

    #[test]
    fn epigraph_deserializes_empty() {
        let xml = r#"<epigraph/>"#;
        let ep = Epigraph::from_mei_str(xml).expect("should deserialize");
        assert!(ep.common.xml_id.is_none());
        assert!(ep.children.is_empty());
    }

    #[test]
    fn epigraph_deserializes_with_text() {
        let xml = r#"<epigraph>A wise quote</epigraph>"#;
        let ep = Epigraph::from_mei_str(xml).expect("should deserialize");
        assert_eq!(ep.children.len(), 1);
        match &ep.children[0] {
            EpigraphChild::Text(text) => assert_eq!(text, "A wise quote"),
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn epigraph_deserializes_with_lg() {
        let xml = r#"<epigraph>
            <lg>
                <l>First line of poem</l>
            </lg>
        </epigraph>"#;
        let ep = Epigraph::from_mei_str(xml).expect("should deserialize");
        assert_eq!(ep.children.len(), 1);
        assert!(matches!(ep.children[0], EpigraphChild::Lg(_)));
    }

    #[test]
    fn dedication_deserializes_empty() {
        let xml = r#"<dedication/>"#;
        let ded = Dedication::from_mei_str(xml).expect("should deserialize");
        assert!(ded.common.xml_id.is_none());
        assert!(ded.children.is_empty());
    }

    #[test]
    fn dedication_deserializes_with_text() {
        let xml = r#"<dedication>To my beloved</dedication>"#;
        let ded = Dedication::from_mei_str(xml).expect("should deserialize");
        assert_eq!(ded.children.len(), 1);
        match &ded.children[0] {
            DedicationChild::Text(text) => assert_eq!(text, "To my beloved"),
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn imprimatur_deserializes_empty() {
        let xml = r#"<imprimatur/>"#;
        let imp = Imprimatur::from_mei_str(xml).expect("should deserialize");
        assert!(imp.common.xml_id.is_none());
        assert!(imp.children.is_empty());
    }

    #[test]
    fn imprimatur_deserializes_with_text() {
        let xml = r#"<imprimatur>Approved by the censor</imprimatur>"#;
        let imp = Imprimatur::from_mei_str(xml).expect("should deserialize");
        assert_eq!(imp.children.len(), 1);
        match &imp.children[0] {
            ImprimaturChild::Text(text) => assert_eq!(text, "Approved by the censor"),
            _ => panic!("expected Text child"),
        }
    }

    #[test]
    fn colophon_deserializes_empty() {
        let xml = r#"<colophon/>"#;
        let col = Colophon::from_mei_str(xml).expect("should deserialize");
        assert!(col.common.xml_id.is_none());
        assert!(col.children.is_empty());
    }

    #[test]
    fn colophon_deserializes_with_text() {
        let xml = r#"<colophon>Printed in Leipzig, 1850</colophon>"#;
        let col = Colophon::from_mei_str(xml).expect("should deserialize");
        assert_eq!(col.children.len(), 1);
        match &col.children[0] {
            ColophonChild::Text(text) => assert_eq!(text, "Printed in Leipzig, 1850"),
            _ => panic!("expected Text child"),
        }
    }
}

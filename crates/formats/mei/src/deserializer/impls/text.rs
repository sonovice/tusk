//! Deserializer implementations for text and prose MEI elements.
//!
//! This module contains implementations for Annot, Rend, Lg, Fig, FigDesc, Verse, List, Li, Seg
//! and related attribute classes.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAnnotAnl, AttAnnotGes, AttAnnotLog, AttAnnotVis, AttAudience, AttColor, AttExtSymAuth,
    AttHorizontalAlign, AttLyricsAnl, AttLyricsGes, AttLyricsLog, AttLyricsVis, AttPlist,
    AttSource, AttSylAnl, AttSylGes, AttSylLog, AttSylVis, AttTypography, AttVerseAnl, AttVerseGes,
    AttVerseLog, AttVerseVis, AttVerticalAlign,
};
use tusk_model::elements::{
    Annot, Div, DivChild, Fig, FigChild, FigDesc, Lb, Lg, LgChild, Li, LiChild, List, ListChild,
    Rend, Seg, SegChild, Syl, SylChild, Verse, VerseChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

impl ExtractAttributes for AttPlist {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "plist", vec self.plist);
        Ok(())
    }
}

impl ExtractAttributes for AttSource {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "source", vec self.source);
        Ok(())
    }
}

impl ExtractAttributes for AttAudience {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "audience", self.audience);
        Ok(())
    }
}

impl ExtractAttributes for AttAnnotLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttAnnotVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "place", vec self.place);
        Ok(())
    }
}

impl ExtractAttributes for AttAnnotGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttAnnotAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAnnotAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttColor {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        Ok(())
    }
}

impl ExtractAttributes for AttExtSymAuth {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        Ok(())
    }
}

impl ExtractAttributes for AttHorizontalAlign {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "halign", self.halign);
        Ok(())
    }
}

impl ExtractAttributes for AttVerticalAlign {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "valign", self.valign);
        Ok(())
    }
}

impl ExtractAttributes for AttTypography {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        Ok(())
    }
}

impl ExtractAttributes for AttVerseLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttVerseLog has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttVerseVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "voltasym", self.voltasym);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttVerseGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttVerseGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttVerseAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttVerseAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttSylLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "con", self.con);
        extract_attr!(attrs, "wordpos", self.wordpos);
        Ok(())
    }
}

impl ExtractAttributes for AttSylVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttSylGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSylGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttSylAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSylAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttLyricsLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        Ok(())
    }
}

impl ExtractAttributes for AttLyricsVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        Ok(())
    }
}

impl ExtractAttributes for AttLyricsGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLyricsGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttLyricsAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLyricsAnl has no attributes
        Ok(())
    }
}

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
                    MixedContent::Element(name, _child_attrs, child_empty) => {
                        // Syl can contain many child elements, but text is the most common
                        // For now we skip other children in lenient mode
                        if !child_empty {
                            reader.skip_to_end(&name)?;
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
}

//! Deserializer implementations for text container MEI elements.
//!
//! This module contains implementations for Group, Quote, Q, Phrase, Line,
//! Refrain, and Stack elements.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttLineAnl, AttLineGes, AttLineLog, AttLineVis, AttPhraseAnl, AttPhraseGes, AttPhraseLog,
    AttPhraseVis, AttRefrainAnl, AttRefrainGes, AttRefrainLog, AttRefrainVis,
};
use tusk_model::elements::{
    Byline, BylineChild, Explicit, ExplicitChild, Group, GroupChild, Line, LineChild, Phrase, Q,
    QChild, Quote, QuoteChild, Refrain, RefrainChild, Rubric, RubricChild, Stack, StackChild,
    Stamp, StampChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Attribute class implementations for Line
// ============================================================================

impl ExtractAttributes for AttLineLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttLineVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "width", self.width);
        extract_attr!(attrs, "endsym", self.endsym);
        extract_attr!(attrs, "endsym.size", self.endsym_size);
        extract_attr!(attrs, "startsym", self.startsym);
        extract_attr!(attrs, "startsym.size", self.startsym_size);
        Ok(())
    }
}

impl ExtractAttributes for AttLineGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttLineAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLineAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Attribute class implementations for Phrase
// ============================================================================

impl ExtractAttributes for AttPhraseLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttPhraseVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        // bezier is space-separated f64 values
        if let Some(value) = attrs.remove("bezier") {
            let items: Vec<f64> = value
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if !items.is_empty() {
                self.bezier = Some(tusk_model::generated::SpaceSeparated::new(items));
            }
        }
        // bulge is space-separated DataPercent values
        if let Some(value) = attrs.remove("bulge") {
            let items: Vec<tusk_model::generated::data::DataPercent> = value
                .split_whitespace()
                .filter_map(|s| from_attr_string(s).ok())
                .collect();
            if !items.is_empty() {
                self.bulge = Some(tusk_model::generated::SpaceSeparated::new(items));
            }
        }
        extract_attr!(attrs, "curvedir", self.curvedir);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        Ok(())
    }
}

impl ExtractAttributes for AttPhraseGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttPhraseAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "join", vec self.join);
        Ok(())
    }
}

// ============================================================================
// Attribute class implementations for Refrain
// ============================================================================

impl ExtractAttributes for AttRefrainLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRefrainLog has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRefrainVis {
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

impl ExtractAttributes for AttRefrainGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRefrainGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRefrainAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRefrainAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Group element implementation
// ============================================================================

impl MeiDeserialize for Group {
    fn element_name() -> &'static str {
        "group"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Group::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.metadata_pointing.extract_attributes(&mut attrs)?;

        // Group can contain nested group or music elements
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("group")?
            {
                match name.as_str() {
                    "group" => {
                        let child = Group::from_mei_event(reader, child_attrs, child_empty)?;
                        elem.children.push(GroupChild::Group(Box::new(child)));
                    }
                    // Skip "music" - parser not yet available
                    // "music" => { ... }
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
}

/// Parse a `<group>` element from within another element.
pub(crate) fn parse_group_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Group> {
    Group::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Quote element implementation
// ============================================================================

impl MeiDeserialize for Quote {
    fn element_name() -> &'static str {
        "quote"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Quote::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;
        elem.text_rendition.extract_attributes(&mut attrs)?;
        elem.xy.extract_attributes(&mut attrs)?;

        // Quote has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("quote")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(QuoteChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_quote_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Quote
fn parse_quote_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Quote,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "p" => {
            let child = super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::P(Box::new(child)));
        }
        "lg" => {
            let child = super::text::parse_lg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Lg(Box::new(child)));
        }
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Title(Box::new(child)));
        }
        // Skip "symbol" - parser not yet available
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Q(Box::new(child)));
        }
        "quote" => {
            let child = Quote::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Quote(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Stack(Box::new(child)));
        }
        "sp" => {
            let child = super::drama::parse_sp_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::Sp(Box::new(child)));
        }
        "stageDir" => {
            let child = super::drama::parse_stage_dir_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QuoteChild::StageDir(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<quote>` element from within another element.
pub(crate) fn parse_quote_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Quote> {
    Quote::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Q element implementation
// ============================================================================

impl MeiDeserialize for Q {
    fn element_name() -> &'static str {
        "q"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Q::default();

        // Extract attributes
        elem.basic.extract_attributes(&mut attrs)?;
        elem.classed.extract_attributes(&mut attrs)?;
        elem.labelled.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;
        elem.linking.extract_attributes(&mut attrs)?;
        elem.n_number_like.extract_attributes(&mut attrs)?;
        elem.responsibility.extract_attributes(&mut attrs)?;
        elem.text_rendition.extract_attributes(&mut attrs)?;

        // Q-specific type attribute
        extract_attr!(attrs, "type", vec_string elem.r#type);

        // Q has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("q")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(QChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_q_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Q
fn parse_q_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Q,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Title(Box::new(child)));
        }
        // Skip "symbol" - parser not yet available
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(QChild::Stack(Box::new(child)));
        }
        // Skip "abbr", "expan" - parsers not yet available
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<q>` element from within another element.
pub(crate) fn parse_q_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Q> {
    Q::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Phrase element implementation
// ============================================================================

impl MeiDeserialize for Phrase {
    fn element_name() -> &'static str {
        "phrase"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Phrase::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.phrase_anl.extract_attributes(&mut attrs)?;
        elem.phrase_ges.extract_attributes(&mut attrs)?;
        elem.phrase_log.extract_attributes(&mut attrs)?;
        elem.phrase_vis.extract_attributes(&mut attrs)?;

        // Phrase can contain curve elements
        // Skip all children - curve parser not yet available
        if !is_empty {
            while let Some((name, _child_attrs, child_empty)) =
                reader.read_next_child_start("phrase")?
            {
                if !child_empty {
                    reader.skip_to_end(&name)?;
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a `<phrase>` element from within another element.
pub(crate) fn parse_phrase_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Phrase> {
    Phrase::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Line element implementation
// ============================================================================

impl MeiDeserialize for Line {
    fn element_name() -> &'static str {
        "line"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Line::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.line_anl.extract_attributes(&mut attrs)?;
        elem.line_ges.extract_attributes(&mut attrs)?;
        elem.line_log.extract_attributes(&mut attrs)?;
        elem.line_vis.extract_attributes(&mut attrs)?;

        // Line has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("line")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(LineChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_line_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Line
fn parse_line_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Line,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Title(Box::new(child)));
        }
        // Skip "symbol" - parser not yet available
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LineChild::Stack(Box::new(child)));
        }
        // Skip "abbr", "expan" - parsers not yet available
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<line>` element from within another element.
pub(crate) fn parse_line_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Line> {
    Line::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Refrain element implementation
// ============================================================================

impl MeiDeserialize for Refrain {
    fn element_name() -> &'static str {
        "refrain"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Refrain::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;
        elem.refrain_log.extract_attributes(&mut attrs)?;
        elem.refrain_vis.extract_attributes(&mut attrs)?;
        elem.refrain_ges.extract_attributes(&mut attrs)?;
        elem.refrain_anl.extract_attributes(&mut attrs)?;

        // Refrain can contain verse-like children
        // Refrain can contain verse-like children
        // Skip all children - parsers for syl, space, dir, dynam, tempo, volta, app, choice, subst not yet available
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("refrain")?
            {
                match name.as_str() {
                    "lb" => {
                        let child =
                            super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(RefrainChild::Lb(Box::new(child)));
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
}

/// Parse a `<refrain>` element from within another element.
pub(crate) fn parse_refrain_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Refrain> {
    Refrain::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Stack element implementation
// ============================================================================

impl MeiDeserialize for Stack {
    fn element_name() -> &'static str {
        "stack"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Stack::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;

        // Stack-specific attributes
        extract_attr!(attrs, "delim", string elem.delim);
        extract_attr!(attrs, "align", string elem.align);

        // Stack has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("stack")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(StackChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_stack_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Stack
fn parse_stack_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Stack,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Title(Box::new(child)));
        }
        // Skip "symbol" - parser not yet available
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(StackChild::Stack(Box::new(child)));
        }
        // Skip "abbr", "expan" - parsers not yet available
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<stack>` element from within another element.
pub(crate) fn parse_stack_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Stack> {
    Stack::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Rubric element implementation
// ============================================================================

impl MeiDeserialize for Rubric {
    fn element_name() -> &'static str {
        "rubric"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Rubric::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.bibl.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;

        // Rubric-specific func attribute
        extract_attr!(attrs, "func", string elem.func);

        // Rubric has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("rubric")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(RubricChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_rubric_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Rubric
fn parse_rubric_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Rubric,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Title(Box::new(child)));
        }
        "p" => {
            let child = super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::P(Box::new(child)));
        }
        "head" => {
            let child = super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Head(Box::new(child)));
        }
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Stack(Box::new(child)));
        }
        "stamp" => {
            let child = Stamp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(RubricChild::Stamp(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<rubric>` element from within another element.
pub(crate) fn parse_rubric_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Rubric> {
    Rubric::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Explicit element implementation
// ============================================================================

impl MeiDeserialize for Explicit {
    fn element_name() -> &'static str {
        "explicit"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Explicit::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.bibl.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;

        // Explicit has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("explicit")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(ExplicitChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_explicit_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Explicit
fn parse_explicit_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Explicit,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Title(Box::new(child)));
        }
        "p" => {
            let child = super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::P(Box::new(child)));
        }
        "head" => {
            let child = super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Head(Box::new(child)));
        }
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Stack(Box::new(child)));
        }
        "stamp" => {
            let child = Stamp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(ExplicitChild::Stamp(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse an `<explicit>` element from within another element.
pub(crate) fn parse_explicit_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Explicit> {
    Explicit::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Byline element implementation
// ============================================================================

impl MeiDeserialize for Byline {
    fn element_name() -> &'static str {
        "byline"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Byline::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.bibl.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;

        // Byline has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("byline")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(BylineChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_byline_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Byline
fn parse_byline_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Byline,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Title(Box::new(child)));
        }
        "creator" => {
            let child = super::header::parse_creator_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Creator(Box::new(child)));
        }
        "editor" => {
            let child = super::header::parse_editor_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Editor(Box::new(child)));
        }
        "funder" => {
            let child = super::header::parse_funder_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Funder(Box::new(child)));
        }
        "sponsor" => {
            let child = super::header::parse_sponsor_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Sponsor(Box::new(child)));
        }
        "contributor" => {
            let child =
                super::header::parse_contributor_from_event(reader, child_attrs, child_empty)?;
            elem.children
                .push(BylineChild::Contributor(Box::new(child)));
        }
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Stack(Box::new(child)));
        }
        "stamp" => {
            let child = Stamp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(BylineChild::Stamp(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<byline>` element from within another element.
pub(crate) fn parse_byline_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Byline> {
    Byline::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Stamp element implementation
// ============================================================================

impl MeiDeserialize for Stamp {
    fn element_name() -> &'static str {
        "stamp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Stamp::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.bibl.extract_attributes(&mut attrs)?;
        elem.datable.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;

        // Stamp has mixed content
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("stamp")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            elem.children.push(StampChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_stamp_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

/// Parse a child element inside Stamp
fn parse_stamp_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Stamp,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "rend" => {
            let child = super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = super::text::parse_seg_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Seg(Box::new(child)));
        }
        "fig" => {
            let child = super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Fig(Box::new(child)));
        }
        "annot" => {
            let child = super::header::parse_annot_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Annot(Box::new(child)));
        }
        "bibl" => {
            let child = super::header::parse_bibl_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Bibl(Box::new(child)));
        }
        "ref" => {
            let child = super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Ref(Box::new(child)));
        }
        "ptr" => {
            let child = super::header::parse_ptr_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Ptr(Box::new(child)));
        }
        "num" => {
            let child = super::misc::parse_num_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Num(Box::new(child)));
        }
        "date" => {
            let child = super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Date(Box::new(child)));
        }
        "name" => {
            let child = super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Name(Box::new(child)));
        }
        "persName" => {
            let child =
                super::header::parse_pers_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::PersName(Box::new(child)));
        }
        "corpName" => {
            let child =
                super::header::parse_corp_name_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::CorpName(Box::new(child)));
        }
        "title" => {
            let child = super::header::parse_title_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Title(Box::new(child)));
        }
        "p" => {
            let child = super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::P(Box::new(child)));
        }
        "head" => {
            let child = super::header::parse_head_from_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Head(Box::new(child)));
        }
        "q" => {
            let child = Q::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Q(Box::new(child)));
        }
        "stack" => {
            let child = Stack::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Stack(Box::new(child)));
        }
        "stamp" => {
            let child = Stamp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(StampChild::Stamp(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

/// Parse a `<stamp>` element from within another element.
pub(crate) fn parse_stamp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Stamp> {
    Stamp::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Cb (column beginning) element implementation
// ============================================================================

use tusk_model::att::{
    AttCurveAnl, AttCurveGes, AttCurveLog, AttCurveVis, AttDivLineLog, AttExtSym, AttStaffLoc,
    AttVisibility, AttVisualOffsetHo,
};
use tusk_model::elements::{Cb, Curve, DivLine};

impl ExtractAttributes for AttVisibility {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffLoc {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "loc", self.loc);
        Ok(())
    }
}

impl ExtractAttributes for AttVisualOffsetHo {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "ho", self.ho);
        Ok(())
    }
}

impl ExtractAttributes for AttExtSym {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        Ok(())
    }
}

impl MeiDeserialize for Cb {
    fn element_name() -> &'static str {
        "cb"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Cb::default();

        // Extract attributes
        elem.basic.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.labelled.extract_attributes(&mut attrs)?;
        elem.linking.extract_attributes(&mut attrs)?;
        elem.responsibility.extract_attributes(&mut attrs)?;
        elem.source.extract_attributes(&mut attrs)?;
        elem.typed.extract_attributes(&mut attrs)?;

        // Extract element-specific n attribute (column number)
        extract_attr!(attrs, "n", elem.n);

        // Cb is an empty element
        if !is_empty {
            reader.skip_to_end("cb")?;
        }

        Ok(elem)
    }
}

/// Parse a `<cb>` element from within another element.
pub(crate) fn parse_cb_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Cb> {
    Cb::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// DivLine (division line in neumes) element implementation
// ============================================================================

impl ExtractAttributes for AttDivLineLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "form", vec self.form);
        Ok(())
    }
}

impl MeiDeserialize for DivLine {
    fn element_name() -> &'static str {
        "divLine"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = DivLine::default();

        // Extract attributes
        elem.basic.extract_attributes(&mut attrs)?;
        elem.classed.extract_attributes(&mut attrs)?;
        elem.color.extract_attributes(&mut attrs)?;
        elem.div_line_log.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.labelled.extract_attributes(&mut attrs)?;
        elem.linking.extract_attributes(&mut attrs)?;
        elem.n_number_like.extract_attributes(&mut attrs)?;
        elem.responsibility.extract_attributes(&mut attrs)?;
        elem.ext_sym.extract_attributes(&mut attrs)?;
        elem.staff_loc.extract_attributes(&mut attrs)?;
        elem.visibility.extract_attributes(&mut attrs)?;
        elem.xy.extract_attributes(&mut attrs)?;
        elem.visual_offset_ho.extract_attributes(&mut attrs)?;

        // DivLine is an empty element
        if !is_empty {
            reader.skip_to_end("divLine")?;
        }

        Ok(elem)
    }
}

/// Parse a `<divLine>` element from within another element.
pub(crate) fn parse_div_line_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<DivLine> {
    DivLine::from_mei_event(reader, attrs, is_empty)
}

// ============================================================================
// Curve (generic curved line) element implementation
// ============================================================================

impl ExtractAttributes for AttCurveLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttCurveVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        // bezier is space-separated f64 values
        if let Some(value) = attrs.remove("bezier") {
            let items: Vec<f64> = value
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if !items.is_empty() {
                self.bezier = Some(tusk_model::generated::SpaceSeparated::new(items));
            }
        }
        // bulge is space-separated DataPercent values
        if let Some(value) = attrs.remove("bulge") {
            let items: Vec<tusk_model::generated::data::DataPercent> = value
                .split_whitespace()
                .filter_map(|s| from_attr_string(s).ok())
                .collect();
            if !items.is_empty() {
                self.bulge = Some(tusk_model::generated::SpaceSeparated::new(items));
            }
        }
        extract_attr!(attrs, "curvedir", self.curvedir);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        Ok(())
    }
}

impl ExtractAttributes for AttCurveGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttCurveGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttCurveAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttCurveAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Curve {
    fn element_name() -> &'static str {
        "curve"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Curve::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.curve_anl.extract_attributes(&mut attrs)?;
        elem.curve_ges.extract_attributes(&mut attrs)?;
        elem.curve_log.extract_attributes(&mut attrs)?;
        elem.curve_vis.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;

        // Curve is an empty element
        if !is_empty {
            reader.skip_to_end("curve")?;
        }

        Ok(elem)
    }
}

/// Parse a `<curve>` element from within another element.
pub(crate) fn parse_curve_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Curve> {
    Curve::from_mei_event(reader, attrs, is_empty)
}

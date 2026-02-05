//! Deserializer implementations for harmony elements: Harm, Fb, F, Symbol.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttFAnl, AttFGes, AttFLog, AttFVis, AttHarmAnl, AttHarmGes, AttHarmLog, AttHarmVis,
    AttSymbolAnl, AttSymbolGes, AttSymbolLog, AttSymbolVis,
};
use tusk_model::elements::{F, FChild, Fb, FbChild, Harm, HarmChild, Symbol};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Harm (Harmony) element deserialization
// ============================================================================


impl ExtractAttributes for AttHarmLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
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
        extract_attr!(attrs, "chordref", self.chordref);
        Ok(())
    }
}

impl ExtractAttributes for AttHarmVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "rendgrid", self.rendgrid);
        Ok(())
    }
}

impl ExtractAttributes for AttHarmGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttHarmAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "inth", vec self.inth);
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl MeiDeserialize for Harm {
    fn element_name() -> &'static str {
        "harm"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut harm = Harm::default();

        // Extract attributes into each attribute class
        harm.common.extract_attributes(&mut attrs)?;
        harm.facsimile.extract_attributes(&mut attrs)?;
        harm.harm_log.extract_attributes(&mut attrs)?;
        harm.harm_vis.extract_attributes(&mut attrs)?;
        harm.harm_ges.extract_attributes(&mut attrs)?;
        harm.harm_anl.extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("harm")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            harm.children.push(HarmChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "fb" => {
                                let fb = parse_fb_from_event(reader, child_attrs, child_empty)?;
                                harm.children.push(HarmChild::Fb(Box::new(fb)));
                            }
                            "rend" => {
                                let rend =
                                    super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                harm.children.push(HarmChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                harm.children.push(HarmChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                harm.children.push(HarmChild::Ref(Box::new(ref_elem)));
                            }
                            _ => {
                                // Skip unknown child elements
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(harm)
    }
}

// ============================================================================
// Fb (Figured Bass) element deserialization
// ============================================================================

/// Parse a `<fb>` element from an event.
pub fn parse_fb_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Fb> {
    let mut fb = Fb::default();

    // Extract attributes
    fb.common.extract_attributes(&mut attrs)?;
    fb.facsimile.extract_attributes(&mut attrs)?;

    // Parse children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("fb")? {
            match name.as_str() {
                "f" => {
                    let f = parse_f_from_event(reader, child_attrs, child_empty)?;
                    fb.children.push(FbChild::F(Box::new(f)));
                }
                _ => {
                    // Skip unknown child elements
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(fb)
}

// ============================================================================
// F (Figure) element deserialization
// ============================================================================

impl ExtractAttributes for AttFLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
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

impl ExtractAttributes for AttFVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttFGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttFAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFAnl has no attributes
        Ok(())
    }
}

/// Parse a `<f>` (figure) element from an event.
pub fn parse_f_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<F> {
    let mut f = F::default();

    // Extract attributes
    f.common.extract_attributes(&mut attrs)?;
    f.facsimile.extract_attributes(&mut attrs)?;
    f.f_log.extract_attributes(&mut attrs)?;
    f.f_vis.extract_attributes(&mut attrs)?;
    f.f_ges.extract_attributes(&mut attrs)?;
    f.f_anl.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements like symbol)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("f")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        f.children.push(FChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "symbol" => {
                        let symbol = parse_symbol_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Symbol(Box::new(symbol)));
                    }
                    "rend" => {
                        let rend = super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Rend(Box::new(rend)));
                    }
                    "lb" => {
                        let lb = super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Lb(Box::new(lb)));
                    }
                    _ => {
                        // Skip unknown child elements
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(f)
}

// ============================================================================
// Symbol element deserialization
// ============================================================================

impl ExtractAttributes for AttSymbolLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "startid", self.startid);
        Ok(())
    }
}

impl ExtractAttributes for AttSymbolVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "scale", self.scale);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttSymbolGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSymbolGes has no attributes in the ODD spec
        Ok(())
    }
}

impl ExtractAttributes for AttSymbolAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSymbolAnl has no attributes in the ODD spec
        Ok(())
    }
}

/// Parse a `<symbol>` element from an event.
pub fn parse_symbol_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Symbol> {
    let mut symbol = Symbol::default();

    // Extract attributes
    symbol.common.extract_attributes(&mut attrs)?;
    symbol.facsimile.extract_attributes(&mut attrs)?;
    symbol.symbol_log.extract_attributes(&mut attrs)?;
    symbol.symbol_vis.extract_attributes(&mut attrs)?;
    symbol.symbol_ges.extract_attributes(&mut attrs)?;
    symbol.symbol_anl.extract_attributes(&mut attrs)?;

    // Symbol has no children, but handle non-empty gracefully
    if !is_empty {
        reader.skip_to_end("symbol")?;
    }

    Ok(symbol)
}


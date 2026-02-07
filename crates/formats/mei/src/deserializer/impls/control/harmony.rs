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
                                let rend = super::super::parse_rend_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                harm.children.push(HarmChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = super::super::parse_lb_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
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
                        let rend =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        f.children.push(FChild::Rend(Box::new(rend)));
                    }
                    "lb" => {
                        let lb =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
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

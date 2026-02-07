//! Deserializer implementations for symbol-related MEI elements.
//!
//! This module contains deserializers for:
//! - SymbolTable (container for symbol definitions)
//! - SymbolDef (symbol definition)
//! - SymName (symbol name)
//! - SymProp (symbol property)
//! - PropName (property name)
//! - PropValue (property value)
//! - Mapping (character mapping)

use super::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, extract_attr,
};
use crate::deserializer::MixedContent;
use std::io::BufRead;
use tusk_model::att::AttDataSelecting;
use tusk_model::elements::{
    AnchoredText, Mapping, MappingChild, PropName, PropNameChild, PropValue, PropValueChild,
    SymName, SymNameChild, SymProp, SymPropChild, SymbolDef, SymbolDefChild, SymbolTable,
    SymbolTableChild,
};

// Import parse functions from other modules
use super::control::parse_symbol_from_event;
use super::facsimile::parse_graphic_from_event;
use super::header::parse_annot_from_event;
use super::text_containers::{parse_curve_from_event, parse_line_from_event};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// SymbolTable element
// ============================================================================

impl MeiDeserialize for SymbolTable {
    fn element_name() -> &'static str {
        "symbolTable"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut symbol_table = SymbolTable::default();

        // Extract attributes
        symbol_table.common.extract_attributes(&mut attrs)?;

        // Read children if not empty
        // SymbolTable can contain: symbolDef*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("symbolTable")?
            {
                match name.as_str() {
                    "symbolDef" => {
                        let symbol_def =
                            parse_symbol_def_from_event(reader, child_attrs, child_empty)?;
                        symbol_table
                            .children
                            .push(SymbolTableChild::SymbolDef(Box::new(symbol_def)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(symbol_table)
    }
}

// ============================================================================
// SymbolDef element
// ============================================================================

impl MeiDeserialize for SymbolDef {
    fn element_name() -> &'static str {
        "symbolDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_symbol_def_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<symbolDef>` element from within another element.
pub(crate) fn parse_symbol_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SymbolDef> {
    let mut symbol_def = SymbolDef::default();

    // Extract attributes
    symbol_def.common.extract_attributes(&mut attrs)?;
    symbol_def.coordinated.extract_attributes(&mut attrs)?;
    symbol_def.data_selecting.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // SymbolDef can contain: annot*, symProp*, symbol*, symName*, graphic*, curve*, line*, anchoredText*, mapping*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("symbolDef")?
        {
            match name.as_str() {
                "annot" => {
                    let annot = parse_annot_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::Annot(Box::new(annot)));
                }
                "symProp" => {
                    let sym_prop = parse_sym_prop_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::SymProp(Box::new(sym_prop)));
                }
                "symbol" => {
                    let symbol = parse_symbol_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::Symbol(Box::new(symbol)));
                }
                "symName" => {
                    let sym_name = parse_sym_name_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::SymName(Box::new(sym_name)));
                }
                "graphic" => {
                    let graphic = parse_graphic_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::Graphic(Box::new(graphic)));
                }
                "curve" => {
                    let curve = parse_curve_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::Curve(Box::new(curve)));
                }
                "line" => {
                    let line = parse_line_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::Line(Box::new(line)));
                }
                "anchoredText" => {
                    let anchored_text =
                        AnchoredText::from_mei_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::AnchoredText(Box::new(anchored_text)));
                }
                "mapping" => {
                    let mapping = parse_mapping_from_event(reader, child_attrs, child_empty)?;
                    symbol_def
                        .children
                        .push(SymbolDefChild::Mapping(Box::new(mapping)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(symbol_def)
}

// ============================================================================
// SymName element
// ============================================================================

impl MeiDeserialize for SymName {
    fn element_name() -> &'static str {
        "symName"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sym_name_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<symName>` element from within another element.
pub(crate) fn parse_sym_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SymName> {
    let mut sym_name = SymName::default();

    // Extract attributes
    sym_name.common.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // SymName can contain: text content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("symName")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        sym_name.children.push(SymNameChild::Text(text));
                    }
                }
                MixedContent::Element(name, _attrs, child_empty) => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(sym_name)
}

// ============================================================================
// SymProp element
// ============================================================================

impl MeiDeserialize for SymProp {
    fn element_name() -> &'static str {
        "symProp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sym_prop_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<symProp>` element from within another element.
pub(crate) fn parse_sym_prop_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SymProp> {
    let mut sym_prop = SymProp::default();

    // Extract attributes
    sym_prop.common.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // SymProp can contain: propName*, propValue*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("symProp")?
        {
            match name.as_str() {
                "propName" => {
                    let prop_name = parse_prop_name_from_event(reader, child_attrs, child_empty)?;
                    sym_prop
                        .children
                        .push(SymPropChild::PropName(Box::new(prop_name)));
                }
                "propValue" => {
                    let prop_value = parse_prop_value_from_event(reader, child_attrs, child_empty)?;
                    sym_prop
                        .children
                        .push(SymPropChild::PropValue(Box::new(prop_value)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(sym_prop)
}

// ============================================================================
// PropName element
// ============================================================================

impl MeiDeserialize for PropName {
    fn element_name() -> &'static str {
        "propName"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_prop_name_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<propName>` element from within another element.
pub(crate) fn parse_prop_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PropName> {
    let mut prop_name = PropName::default();

    // Extract attributes
    prop_name.basic.extract_attributes(&mut attrs)?;
    prop_name.labelled.extract_attributes(&mut attrs)?;
    prop_name.linking.extract_attributes(&mut attrs)?;
    prop_name.n_number_like.extract_attributes(&mut attrs)?;
    prop_name.responsibility.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "type", string prop_name.r#type);

    // Read children if not empty
    // PropName can contain: text content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("propName")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        prop_name.children.push(PropNameChild::Text(text));
                    }
                }
                MixedContent::Element(name, _attrs, child_empty) => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(prop_name)
}

// ============================================================================
// PropValue element
// ============================================================================

impl MeiDeserialize for PropValue {
    fn element_name() -> &'static str {
        "propValue"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_prop_value_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<propValue>` element from within another element.
pub(crate) fn parse_prop_value_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PropValue> {
    let mut prop_value = PropValue::default();

    // Extract attributes
    prop_value.common.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // PropValue can contain: text content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("propValue")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        prop_value.children.push(PropValueChild::Text(text));
                    }
                }
                MixedContent::Element(name, _attrs, child_empty) => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(prop_value)
}

// ============================================================================
// Mapping element
// ============================================================================

impl MeiDeserialize for Mapping {
    fn element_name() -> &'static str {
        "mapping"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_mapping_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<mapping>` element from within another element.
pub(crate) fn parse_mapping_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Mapping> {
    let mut mapping = Mapping::default();

    // Extract attributes
    mapping.common.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // Mapping can contain: text, symbol*
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("mapping")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        mapping.children.push(MappingChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "symbol" => {
                        let symbol = parse_symbol_from_event(reader, child_attrs, child_empty)?;
                        mapping
                            .children
                            .push(MappingChild::Symbol(Box::new(symbol)));
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

    Ok(mapping)
}

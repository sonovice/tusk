//! MEI ODD file parser.
//!
//! Parses MEI ODD specification files and extracts data types, attribute classes,
//! model classes, and element definitions.

use anyhow::{Context, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::fs;
use std::path::Path;

use crate::ast::*;

/// Parse all MEI ODD files in a directory.
pub fn parse_odd_files(input_dir: &Path) -> Result<OddDefinitions> {
    let mut defs = OddDefinitions::new();

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "xml") {
            let filename = path.file_name().unwrap().to_string_lossy();
            if filename.starts_with("MEI.") {
                println!("  Parsing: {}", filename);
                parse_odd_file(&path, &mut defs)?;
            }
        }
    }

    Ok(defs)
}

/// Parse a single MEI ODD file.
pub fn parse_odd_file(path: &Path, defs: &mut OddDefinitions) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"macroSpec" => {
                    parse_macro_spec(&mut reader, &e, defs)?;
                }
                b"classSpec" => {
                    parse_class_spec(&mut reader, &e, defs)?;
                }
                b"elementSpec" => {
                    if let Some(elem) = parse_element_spec(&mut reader, &e)? {
                        defs.elements.insert(elem.ident.clone(), elem);
                    }
                }
                _ => {}
            },
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(())
}

/// Parse a macroSpec element (data type or pattern entity).
fn parse_macro_spec(
    reader: &mut Reader<&[u8]>,
    start: &BytesStart,
    defs: &mut OddDefinitions,
) -> Result<()> {
    let mut ident = String::new();
    let mut module = String::new();
    let mut type_attr = String::new();

    for attr in start.attributes() {
        let attr = attr?;
        match attr.key.as_ref() {
            b"ident" => ident = String::from_utf8_lossy(&attr.value).to_string(),
            b"module" => module = String::from_utf8_lossy(&attr.value).to_string(),
            b"type" => type_attr = String::from_utf8_lossy(&attr.value).to_string(),
            _ => {}
        }
    }

    match type_attr.as_str() {
        "dt" => {
            // Data type
            if let Some(dt) = parse_data_type(reader, ident, module)? {
                defs.data_types.insert(dt.ident.clone(), dt);
            }
        }
        "pe" => {
            // Pattern entity
            if let Some(pe) = parse_pattern_entity(reader, ident, module)? {
                defs.pattern_entities.insert(pe.ident.clone(), pe);
            }
        }
        _ => {
            skip_to_end(reader, b"macroSpec")?;
        }
    }

    Ok(())
}

/// Parse a data type definition (macroSpec type="dt").
fn parse_data_type(
    reader: &mut Reader<&[u8]>,
    ident: String,
    module: String,
) -> Result<Option<DataType>> {
    let mut desc = String::new();
    let mut kind = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"desc" => desc = read_text_content(reader, b"desc")?,
                b"content" => {
                    kind = Some(parse_datatype_content(reader)?);
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == b"macroSpec" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    // If there's no content, create an empty value list
    let kind = kind.unwrap_or(DataTypeKind::ValList(Vec::new()));
    Ok(Some(DataType {
        ident,
        module,
        desc,
        kind,
    }))
}

/// Parse the content of a data type definition.
fn parse_datatype_content(reader: &mut Reader<&[u8]>) -> Result<DataTypeKind> {
    let mut buf = Vec::new();
    let mut result = DataTypeKind::ValList(Vec::new()); // Default

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"valList" => {
                        let values = parse_val_list(reader)?;
                        result = DataTypeKind::ValList(values);
                    }
                    b"rng:data" => {
                        let (type_name, pattern, min, max) = parse_rng_data(&e, reader, false)?;
                        result = DataTypeKind::Primitive {
                            type_name,
                            pattern,
                            min_inclusive: min,
                            max_inclusive: max,
                        };
                    }
                    b"rng:ref" => {
                        if let Some(name) = get_attr(&e, b"name") {
                            result = DataTypeKind::Reference(name);
                        }
                        skip_to_end(reader, b"rng:ref")?;
                    }
                    b"alternate" => {
                        let refs = parse_alternate(reader)?;
                        result = DataTypeKind::Alternate(refs);
                    }
                    b"rng:choice" => {
                        if let Some(refs) = parse_rng_choice_refs(reader)? {
                            result = DataTypeKind::Choice(refs);
                        } else {
                            // Choice of only primitives - treat as String
                            result = DataTypeKind::Primitive {
                                type_name: "string".to_string(),
                                pattern: None,
                                min_inclusive: None,
                                max_inclusive: None,
                            };
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"rng:data" => {
                        // Self-closing rng:data (no children)
                        let (type_name, pattern, min, max) = parse_rng_data(&e, reader, true)?;
                        result = DataTypeKind::Primitive {
                            type_name,
                            pattern,
                            min_inclusive: min,
                            max_inclusive: max,
                        };
                    }
                    b"rng:ref" => {
                        if let Some(name) = get_attr(&e, b"name") {
                            result = DataTypeKind::Reference(name);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"content" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(result)
}

/// Parse a valList element.
fn parse_val_list(reader: &mut Reader<&[u8]>) -> Result<Vec<DataTypeValue>> {
    let mut values = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"valItem" => {
                let ident = get_attr(&e, b"ident").unwrap_or_default();
                let desc = parse_val_item_desc(reader)?;
                values.push(DataTypeValue { ident, desc });
            }
            Ok(Event::Empty(e)) if e.name().as_ref() == b"valItem" => {
                let ident = get_attr(&e, b"ident").unwrap_or_default();
                values.push(DataTypeValue {
                    ident,
                    desc: String::new(),
                });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"valList" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(values)
}

/// Parse the description inside a valItem.
fn parse_val_item_desc(reader: &mut Reader<&[u8]>) -> Result<String> {
    let mut desc = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"desc" => {
                desc = read_text_content(reader, b"desc")?;
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"valItem" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(desc)
}

/// Parse rng:data element.
/// `is_empty` indicates whether this was a self-closing tag (no children to parse).
fn parse_rng_data(
    start: &BytesStart,
    reader: &mut Reader<&[u8]>,
    is_empty: bool,
) -> Result<(String, Option<String>, Option<String>, Option<String>)> {
    let type_name = get_attr(start, b"type").unwrap_or_else(|| "string".to_string());
    let mut pattern = None;
    let mut min_inclusive = None;
    let mut max_inclusive = None;
    let mut buf = Vec::new();

    // If self-closing, there are no children to parse
    if is_empty {
        return Ok((type_name, pattern, min_inclusive, max_inclusive));
    }

    // Parse children (rng:param elements)
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"rng:param" {
                    let name = get_attr(&e, b"name");
                    let value = match reader.read_event_into(&mut buf)? {
                        Event::Text(t) => t.unescape()?.to_string(),
                        _ => String::new(),
                    };
                    match name.as_deref() {
                        Some("pattern") => pattern = Some(value),
                        Some("minInclusive") => min_inclusive = Some(value),
                        Some("maxInclusive") => max_inclusive = Some(value),
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"rng:data" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok((type_name, pattern, min_inclusive, max_inclusive))
}

/// Parse an alternate element (union of types).
fn parse_alternate(reader: &mut Reader<&[u8]>) -> Result<Vec<DataTypeRef>> {
    let mut refs = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"macroRef" => {
                    if let Some(key) = get_attr(&e, b"key") {
                        refs.push(DataTypeRef::MacroRef(key));
                    }
                }
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        refs.push(DataTypeRef::RngRef(name));
                    }
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == b"alternate" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(refs)
}

/// Parse rng:choice containing references or primitives.
/// Returns None if the choice only contains rng:data (primitives).
fn parse_rng_choice_refs(reader: &mut Reader<&[u8]>) -> Result<Option<Vec<DataTypeRef>>> {
    let mut refs = Vec::new();
    let mut has_data = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"rng:ref" => {
                        if let Some(name) = get_attr(&e, b"name") {
                            refs.push(DataTypeRef::RngRef(name));
                        }
                    }
                    b"rng:data" => {
                        has_data = true;
                        // Skip the content of rng:data
                        if matches!(reader.read_event_into(&mut buf)?, Event::Start(_)) {
                            skip_to_end(reader, b"rng:data")?;
                        }
                    }
                    b"macroRef" => {
                        if let Some(key) = get_attr(&e, b"key") {
                            refs.push(DataTypeRef::MacroRef(key));
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"rng:choice" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    // If only primitives, return None to indicate this is a primitive type
    if refs.is_empty() && has_data {
        return Ok(None);
    }

    Ok(Some(refs))
}

/// Parse a pattern entity (macroSpec type="pe").
fn parse_pattern_entity(
    reader: &mut Reader<&[u8]>,
    ident: String,
    module: String,
) -> Result<Option<PatternEntity>> {
    let mut desc = String::new();
    let mut content = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"desc" => desc = read_text_content(reader, b"desc")?,
                b"content" => {
                    content = parse_content_model(reader)?;
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == b"macroSpec" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(Some(PatternEntity {
        ident,
        module,
        desc,
        content,
    }))
}

/// Parse a classSpec element (attribute class or model class).
fn parse_class_spec(
    reader: &mut Reader<&[u8]>,
    start: &BytesStart,
    defs: &mut OddDefinitions,
) -> Result<()> {
    let mut ident = String::new();
    let mut module = String::new();
    let mut type_attr = String::new();

    for attr in start.attributes() {
        let attr = attr?;
        match attr.key.as_ref() {
            b"ident" => ident = String::from_utf8_lossy(&attr.value).to_string(),
            b"module" => module = String::from_utf8_lossy(&attr.value).to_string(),
            b"type" => type_attr = String::from_utf8_lossy(&attr.value).to_string(),
            _ => {}
        }
    }

    match type_attr.as_str() {
        "atts" => {
            if let Some(ac) = parse_att_class(reader, ident, module)? {
                defs.att_classes.insert(ac.ident.clone(), ac);
            }
        }
        "model" => {
            if let Some(mc) = parse_model_class(reader, ident, module)? {
                defs.model_classes.insert(mc.ident.clone(), mc);
            }
        }
        _ => {
            skip_to_end(reader, b"classSpec")?;
        }
    }

    Ok(())
}

/// Parse an attribute class (classSpec type="atts").
fn parse_att_class(
    reader: &mut Reader<&[u8]>,
    ident: String,
    module: String,
) -> Result<Option<AttClass>> {
    let mut desc = String::new();
    let mut member_of = Vec::new();
    let mut attributes = Vec::new();
    let mut constraints = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"desc" => desc = read_text_content(reader, b"desc")?,
                b"classes" => {
                    // Parse <classes> wrapper containing <memberOf> elements
                    let members = parse_classes_wrapper(reader)?;
                    member_of.extend(members);
                }
                b"memberOf" => {
                    // Direct memberOf (without classes wrapper)
                    if let Some(key) = get_attr(&e, b"key") {
                        member_of.push(key);
                    }
                }
                b"attDef" => {
                    if let Some(attr) = parse_att_def(reader, &e)? {
                        attributes.push(attr);
                    }
                }
                b"constraintSpec" => {
                    let cs = parse_constraint_spec(reader, &e)?;
                    constraints.extend(cs);
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"memberOf" {
                    if let Some(key) = get_attr(&e, b"key") {
                        member_of.push(key);
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"classSpec" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(Some(AttClass {
        ident,
        module,
        desc,
        member_of,
        attributes,
        constraints,
    }))
}

/// Parse a model class (classSpec type="model").
fn parse_model_class(
    reader: &mut Reader<&[u8]>,
    ident: String,
    module: String,
) -> Result<Option<ModelClass>> {
    let mut desc = String::new();
    let mut member_of = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"desc" => desc = read_text_content(reader, b"desc")?,
                b"classes" => {
                    // Parse <classes> wrapper containing <memberOf> elements
                    let members = parse_classes_wrapper(reader)?;
                    member_of.extend(members);
                }
                b"memberOf" => {
                    // Direct memberOf (without classes wrapper)
                    if let Some(key) = get_attr(&e, b"key") {
                        member_of.push(key);
                    }
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"memberOf" {
                    if let Some(key) = get_attr(&e, b"key") {
                        member_of.push(key);
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"classSpec" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(Some(ModelClass {
        ident,
        module,
        desc,
        member_of,
    }))
}

/// Parse a <classes> wrapper element containing <memberOf> elements.
fn parse_classes_wrapper(reader: &mut Reader<&[u8]>) -> Result<Vec<String>> {
    let mut members = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"memberOf" {
                    if let Some(key) = get_attr(&e, b"key") {
                        members.push(key);
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"memberOf" {
                    if let Some(key) = get_attr(&e, b"key") {
                        members.push(key);
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"classes" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(members)
}

/// Parse an attribute definition (attDef).
fn parse_att_def(reader: &mut Reader<&[u8]>, start: &BytesStart) -> Result<Option<Attribute>> {
    let ident = get_attr(start, b"ident").unwrap_or_default();
    let usage = get_attr(start, b"usage").unwrap_or_else(|| "opt".to_string());

    let mut desc = String::new();
    let mut datatype = None;
    let mut default_val = None;
    let mut max_occurs = None;
    let mut constraints = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"desc" => desc = read_text_content(reader, b"desc")?,
                b"datatype" => {
                    max_occurs = get_attr(&e, b"maxOccurs");
                    datatype = parse_attribute_datatype(reader)?;
                }
                b"valList" => {
                    // Inline value list
                    let values = parse_val_list(reader)?;
                    datatype = Some(AttributeDataType::InlineValList(values));
                }
                b"defaultVal" => {
                    default_val = Some(read_text_content(reader, b"defaultVal")?);
                }
                b"constraintSpec" => {
                    let cs = parse_constraint_spec(reader, &e)?;
                    constraints.extend(cs);
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == b"attDef" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(Some(Attribute {
        ident,
        desc,
        usage,
        datatype,
        default_val,
        max_occurs,
        constraints,
    }))
}

/// Parse an attList element (used for element-local attributes).
fn parse_att_list(reader: &mut Reader<&[u8]>) -> Result<Vec<Attribute>> {
    let mut attributes = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"attDef" {
                    if let Some(attr) = parse_att_def(reader, &e)? {
                        attributes.push(attr);
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"attList" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(attributes)
}

/// Parse the datatype of an attribute.
fn parse_attribute_datatype(reader: &mut Reader<&[u8]>) -> Result<Option<AttributeDataType>> {
    let mut datatype = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        datatype = Some(AttributeDataType::Ref(name));
                    }
                    skip_to_end(reader, b"rng:ref")?;
                }
                b"rng:data" => {
                    let type_name = get_attr(&e, b"type").unwrap_or_else(|| "string".to_string());
                    let pattern = parse_rng_data_pattern(reader)?;
                    datatype = Some(AttributeDataType::Primitive { type_name, pattern });
                }
                b"valList" => {
                    let values = parse_val_list(reader)?;
                    datatype = Some(AttributeDataType::InlineValList(values));
                }
                b"rng:list" => {
                    let (inner, min_occurs) = parse_rng_list_datatype(reader)?;
                    datatype = Some(AttributeDataType::List {
                        inner: Box::new(inner),
                        min_occurs,
                    });
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        datatype = Some(AttributeDataType::Ref(name));
                    }
                }
                b"rng:data" => {
                    let type_name = get_attr(&e, b"type").unwrap_or_else(|| "string".to_string());
                    datatype = Some(AttributeDataType::Primitive {
                        type_name,
                        pattern: None,
                    });
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == b"datatype" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(datatype)
}

/// Parse the pattern from inside an rng:data element.
fn parse_rng_data_pattern(reader: &mut Reader<&[u8]>) -> Result<Option<String>> {
    let mut pattern = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"rng:param" => {
                if get_attr(&e, b"name").as_deref() == Some("pattern") {
                    pattern = Some(read_text_content(reader, b"rng:param")?);
                } else {
                    skip_to_end(reader, b"rng:param")?;
                }
            }
            Ok(Event::Empty(_)) => {}
            Ok(Event::End(e)) if e.name().as_ref() == b"rng:data" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(pattern)
}

/// Parse an rng:list element in attribute datatype context.
/// Returns (inner_datatype, min_occurs).
fn parse_rng_list_datatype(reader: &mut Reader<&[u8]>) -> Result<(AttributeDataType, u32)> {
    let mut inner = AttributeDataType::Primitive {
        type_name: "string".to_string(),
        pattern: None,
    };
    let mut min_occurs = 0u32; // Default: zeroOrMore
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"rng:oneOrMore" => {
                    min_occurs = 1;
                    inner = parse_rng_list_inner(reader, b"rng:oneOrMore")?;
                }
                b"rng:zeroOrMore" => {
                    min_occurs = 0;
                    inner = parse_rng_list_inner(reader, b"rng:zeroOrMore")?;
                }
                b"rng:data" => {
                    let type_name = get_attr(&e, b"type").unwrap_or_else(|| "string".to_string());
                    let pattern = parse_rng_data_pattern(reader)?;
                    inner = AttributeDataType::Primitive { type_name, pattern };
                    min_occurs = 1; // Single item
                }
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        inner = AttributeDataType::Ref(name);
                    }
                    skip_to_end(reader, b"rng:ref")?;
                    min_occurs = 1;
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"rng:data" => {
                    let type_name = get_attr(&e, b"type").unwrap_or_else(|| "string".to_string());
                    inner = AttributeDataType::Primitive {
                        type_name,
                        pattern: None,
                    };
                    min_occurs = 1;
                }
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        inner = AttributeDataType::Ref(name);
                    }
                    min_occurs = 1;
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == b"rng:list" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok((inner, min_occurs))
}

/// Parse the inner content of rng:oneOrMore or rng:zeroOrMore inside rng:list.
fn parse_rng_list_inner(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Result<AttributeDataType> {
    let mut inner = AttributeDataType::Primitive {
        type_name: "string".to_string(),
        pattern: None,
    };
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"rng:data" => {
                    let type_name = get_attr(&e, b"type").unwrap_or_else(|| "string".to_string());
                    let pattern = parse_rng_data_pattern(reader)?;
                    inner = AttributeDataType::Primitive { type_name, pattern };
                }
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        inner = AttributeDataType::Ref(name);
                    }
                    skip_to_end(reader, b"rng:ref")?;
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"rng:data" => {
                    let type_name = get_attr(&e, b"type").unwrap_or_else(|| "string".to_string());
                    inner = AttributeDataType::Primitive {
                        type_name,
                        pattern: None,
                    };
                }
                b"rng:ref" => {
                    if let Some(name) = get_attr(&e, b"name") {
                        inner = AttributeDataType::Ref(name);
                    }
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == end_tag => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(inner)
}

/// Parse an element definition (elementSpec).
fn parse_element_spec(reader: &mut Reader<&[u8]>, start: &BytesStart) -> Result<Option<Element>> {
    let ident = get_attr(start, b"ident").unwrap_or_default();
    let module = get_attr(start, b"module").unwrap_or_default();

    let mut gloss = String::new();
    let mut desc = String::new();
    let mut member_of = Vec::new();
    let mut content = Vec::new();
    let mut constraints = Vec::new();
    let mut local_attributes = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"gloss" => gloss = read_text_content(reader, b"gloss")?,
                b"desc" => desc = read_text_content(reader, b"desc")?,
                b"memberOf" => {
                    if let Some(key) = get_attr(&e, b"key") {
                        member_of.push(key);
                    }
                }
                b"content" => {
                    content = parse_content_model(reader)?;
                }
                b"constraintSpec" => {
                    let cs = parse_constraint_spec(reader, &e)?;
                    constraints.extend(cs);
                }
                b"attList" => {
                    // Parse element-local attributes
                    let attrs = parse_att_list(reader)?;
                    local_attributes.extend(attrs);
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"memberOf" {
                    if let Some(key) = get_attr(&e, b"key") {
                        member_of.push(key);
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"elementSpec" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(Some(Element {
        ident,
        module,
        gloss,
        desc,
        member_of,
        content,
        constraints,
        local_attributes,
    }))
}

/// Parse a content model (RelaxNG patterns).
fn parse_content_model(reader: &mut Reader<&[u8]>) -> Result<ContentModel> {
    let mut items = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if let Some(item) = parse_content_item(reader, &e)? {
                    items.push(item);
                }
            }
            Ok(Event::Empty(e)) => {
                if let Some(item) = parse_content_item_empty(&e)? {
                    items.push(item);
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"content" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(items)
}

/// Parse a content model item from a start tag.
fn parse_content_item(
    reader: &mut Reader<&[u8]>,
    start: &BytesStart,
) -> Result<Option<ContentItem>> {
    let item = match start.name().as_ref() {
        b"rng:zeroOrMore" => {
            let inner = parse_content_model_inner(reader, b"rng:zeroOrMore")?;
            Some(ContentItem::ZeroOrMore(Box::new(inner)))
        }
        b"rng:oneOrMore" => {
            let inner = parse_content_model_inner(reader, b"rng:oneOrMore")?;
            Some(ContentItem::OneOrMore(Box::new(inner)))
        }
        b"rng:optional" => {
            let inner = parse_content_model_inner(reader, b"rng:optional")?;
            Some(ContentItem::Optional(Box::new(inner)))
        }
        b"rng:choice" => {
            let choices = parse_content_model_choices(reader, b"rng:choice")?;
            Some(ContentItem::Choice(choices))
        }
        b"rng:group" => {
            let items = parse_content_model_inner(reader, b"rng:group")?;
            Some(ContentItem::Group(Box::new(items)))
        }
        b"rng:interleave" => {
            let items = parse_content_model_choices(reader, b"rng:interleave")?;
            Some(ContentItem::Interleave(items))
        }
        b"rng:list" => {
            let inner = parse_content_model_inner(reader, b"rng:list")?;
            Some(ContentItem::List(Box::new(inner)))
        }
        b"rng:ref" => {
            let name = get_attr(start, b"name").unwrap_or_default();
            skip_to_end(reader, b"rng:ref")?;
            Some(ContentItem::Ref(name))
        }
        b"rng:text" => {
            skip_to_end(reader, b"rng:text")?;
            Some(ContentItem::Text)
        }
        b"empty" => {
            skip_to_end(reader, b"empty")?;
            Some(ContentItem::Empty)
        }
        b"macroRef" => {
            let key = get_attr(start, b"key").unwrap_or_default();
            skip_to_end(reader, b"macroRef")?;
            Some(ContentItem::MacroRef(key))
        }
        b"rng:element" => {
            // Complex any element pattern - skip for now
            skip_to_end(reader, b"rng:element")?;
            Some(ContentItem::AnyElement)
        }
        _ => None,
    };

    Ok(item)
}

/// Parse a content model item from an empty tag.
fn parse_content_item_empty(start: &BytesStart) -> Result<Option<ContentItem>> {
    let item = match start.name().as_ref() {
        b"rng:ref" => {
            let name = get_attr(start, b"name").unwrap_or_default();
            Some(ContentItem::Ref(name))
        }
        b"rng:text" => Some(ContentItem::Text),
        b"empty" => Some(ContentItem::Empty),
        b"macroRef" => {
            let key = get_attr(start, b"key").unwrap_or_default();
            Some(ContentItem::MacroRef(key))
        }
        _ => None,
    };

    Ok(item)
}

/// Parse inner content model items until end tag.
fn parse_content_model_inner(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Result<ContentModel> {
    let mut items = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if let Some(item) = parse_content_item(reader, &e)? {
                    items.push(item);
                }
            }
            Ok(Event::Empty(e)) => {
                if let Some(item) = parse_content_item_empty(&e)? {
                    items.push(item);
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == end_tag => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(items)
}

/// Parse choices in a rng:choice element.
fn parse_content_model_choices(
    reader: &mut Reader<&[u8]>,
    end_tag: &[u8],
) -> Result<Vec<ContentModel>> {
    let mut choices = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if let Some(item) = parse_content_item(reader, &e)? {
                    // Each item in a choice is its own alternative
                    choices.push(vec![item]);
                }
            }
            Ok(Event::Empty(e)) => {
                if let Some(item) = parse_content_item_empty(&e)? {
                    choices.push(vec![item]);
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == end_tag => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(choices)
}

// ============================================================================
// Constraint parsing
// ============================================================================

/// Parse a constraintSpec element (Schematron rules).
fn parse_constraint_spec(
    reader: &mut Reader<&[u8]>,
    start: &BytesStart,
) -> Result<Vec<Constraint>> {
    let ident = get_attr(start, b"ident").unwrap_or_default();
    let mut constraints = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"constraint" => {
                let inner_constraints = parse_schematron_constraint(reader, &ident, b"constraint")?;
                constraints.extend(inner_constraints);
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"constraintSpec" => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(constraints)
}

/// Parse Schematron rules inside a container element.
/// The `end_tag` parameter specifies which closing tag terminates parsing.
fn parse_schematron_constraint(
    reader: &mut Reader<&[u8]>,
    ident: &str,
    end_tag: &[u8],
) -> Result<Vec<Constraint>> {
    let mut constraints = Vec::new();
    let mut buf = Vec::new();
    let mut current_context = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"sch:rule" => {
                    current_context = get_attr(&e, b"context").unwrap_or_default();
                }
                b"sch:assert" => {
                    let test = get_attr(&e, b"test").unwrap_or_default();
                    let role = get_attr(&e, b"role").unwrap_or_else(|| "error".to_string());
                    let message = read_text_content(reader, b"sch:assert")?;
                    constraints.push(Constraint {
                        ident: ident.to_string(),
                        context: current_context.clone(),
                        test,
                        message: message.trim().to_string(),
                        role,
                    });
                }
                b"sch:report" => {
                    // sch:report is the inverse of sch:assert - fires when test is true
                    let test = get_attr(&e, b"test").unwrap_or_default();
                    let role = get_attr(&e, b"role").unwrap_or_else(|| "error".to_string());
                    let message = read_text_content(reader, b"sch:report")?;
                    constraints.push(Constraint {
                        ident: ident.to_string(),
                        context: current_context.clone(),
                        test: format!("not({})", test), // Invert for assert semantics
                        message: message.trim().to_string(),
                        role,
                    });
                }
                b"sch:pattern" => {
                    // Nested pattern - recurse with sch:pattern as end tag
                    let inner = parse_schematron_constraint(reader, ident, b"sch:pattern")?;
                    constraints.extend(inner);
                }
                _ => {}
            },
            Ok(Event::End(e)) if e.name().as_ref() == end_tag => break,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(constraints)
}

// ============================================================================
// Helper functions
// ============================================================================

/// Get an attribute value from a start tag.
fn get_attr(start: &BytesStart, name: &[u8]) -> Option<String> {
    start
        .attributes()
        .filter_map(|a| a.ok())
        .find(|a| a.key.as_ref() == name)
        .map(|a| String::from_utf8_lossy(&a.value).to_string())
}

/// Read text content until an end tag.
fn read_text_content(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Result<String> {
    let mut text = String::new();
    let mut buf = Vec::new();
    let mut depth = 1;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                text.push_str(&e.unescape()?);
            }
            Ok(Event::Start(_)) => {
                depth += 1;
            }
            Ok(Event::End(e)) => {
                depth -= 1;
                if depth == 0 && e.name().as_ref() == end_tag {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(text.trim().to_string())
}

/// Skip to an end tag (handles nested tags of the same name).
fn skip_to_end(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Result<()> {
    let mut buf = Vec::new();
    let mut depth = 1;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == end_tag => depth += 1,
            Ok(Event::End(e)) if e.name().as_ref() == end_tag => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attr() {
        let xml = r#"<test foo="bar" baz="qux"/>"#;
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        if let Ok(Event::Empty(e)) = reader.read_event_into(&mut buf) {
            assert_eq!(get_attr(&e, b"foo"), Some("bar".to_string()));
            assert_eq!(get_attr(&e, b"baz"), Some("qux".to_string()));
            assert_eq!(get_attr(&e, b"missing"), None);
        }
    }
}

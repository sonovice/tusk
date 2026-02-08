//! Parser for MusicXML XSD schema.
//!
//! Extracts simpleType, complexType, group, attributeGroup, and top-level
//! element definitions from specs/musicxml/schema/musicxml.xsd.

use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;
use std::path::Path;

use crate::ast::*;

fn local_name(name: &[u8]) -> Vec<u8> {
    if let Some(i) = name.iter().position(|&b| b == b':') {
        name[i + 1..].to_vec()
    } else {
        name.to_vec()
    }
}

fn get_attr<'a>(e: &quick_xml::events::BytesStart<'a>, key: &str) -> Option<String> {
    let key = key.as_bytes();
    for a in e.attributes() {
        let a = a.ok()?;
        if a.key.as_ref() == key {
            return Some(String::from_utf8_lossy(a.value.as_ref()).into_owned());
        }
    }
    None
}

fn parse_u32(s: &str) -> u32 {
    s.parse().unwrap_or(1)
}

/// Parse the MusicXML XSD file and return a Schema AST.
pub fn parse_xsd(path: &Path) -> Result<Schema> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("read XSD: {}", path.display()))?;
    parse_xsd_str(&content)
}

/// Parse XSD from string (for tests or in-memory).
pub fn parse_xsd_str(content: &str) -> Result<Schema> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);

    let mut schema = Schema::default();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                match local.as_slice() {
                    b"simpleType" => {
                        if let Some(n) = get_attr(&e, "name") {
                            if let Some(st) = read_simple_type(&mut reader, &mut buf)? {
                                schema.simple_types.insert(n, st);
                            }
                        }
                    }
                    b"complexType" => {
                        if let Some(n) = get_attr(&e, "name") {
                            if let Some(ct) = read_complex_type(&mut reader, &mut buf)? {
                                schema.complex_types.insert(n, ct);
                            }
                        }
                    }
                    b"group" => {
                        if let Some(n) = get_attr(&e, "name") {
                            if let Some(g) = read_group(&mut reader, &mut buf)? {
                                schema.groups.insert(n, g);
                            }
                        }
                    }
                    b"attributeGroup" => {
                        if let Some(n) = get_attr(&e, "name") {
                            if let Some(ag) = read_attribute_group(&mut reader, &mut buf)? {
                                schema.attribute_groups.insert(n, ag);
                            }
                        }
                    }
                    b"element" => {
                        let elem_name = get_attr(&e, "name");
                        let type_ref = get_attr(&e, "type");
                        if let Some(n) = elem_name {
                            if let Some(ed) = read_element_decl(&mut reader, &mut buf, type_ref)? {
                                schema.elements.insert(n, ed);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(schema)
}

fn read_simple_type<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<Option<SimpleType>> {
    let mut depth = 1u32;
    let mut restriction_base: Option<String> = None;
    let mut values = Vec::new();
    let mut pattern: Option<String> = None;

    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                if local == b"restriction" {
                    restriction_base = get_attr(&e, "base").or(restriction_base);
                }
                if local == b"enumeration" {
                    if let Some(v) = get_attr(&e, "value") {
                        values.push(v);
                    }
                }
                if local == b"pattern" {
                    pattern = get_attr(&e, "value").or(pattern);
                }
                if local == b"simpleType" {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                if local_name(e.name().as_ref()).as_slice() == b"simpleType" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    let base = restriction_base.unwrap_or_else(|| "xs:string".to_string());
    if !values.is_empty() {
        return Ok(Some(SimpleType::Enum { base, values }));
    }
    Ok(Some(SimpleType::Alias {
        base,
        pattern,
    }))
}

fn read_complex_type<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<Option<ComplexType>> {
    let mut depth = 1u32;
    let mut doc = None;
    let mut content = ComplexContent::Empty;
    let mut attribute_groups = Vec::new();
    let mut attributes = Vec::new();

    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                match local.as_slice() {
                    b"annotation" => {
                        let mut inner = Vec::new();
                        if let Some(d) = read_documentation(reader, &mut inner)? {
                            doc = Some(d);
                        }
                    }
                    b"simpleContent" => {
                        depth += 1;
                        let mut inner = Vec::new();
                        let (ext_base, ags, attrs) = read_extension(reader, &mut inner)?;
                        content = ComplexContent::SimpleExtension { base: ext_base };
                        attribute_groups = ags;
                        attributes = attrs;
                    }
                    b"complexContent" => {
                        let mut inner = Vec::new();
                        let (ext_base, ags, attrs) = read_extension(reader, &mut inner)?;
                        content = ComplexContent::SimpleExtension { base: ext_base };
                        attribute_groups = ags;
                        attributes = attrs;
                    }
                    b"sequence" => {
                        let mut inner = Vec::new();
                        if let Some(p) = read_sequence_or_choice(reader, &mut inner, b"sequence")? {
                            content = ComplexContent::Model(p);
                        }
                        depth += 1;
                    }
                    b"choice" => {
                        let mut inner = Vec::new();
                        if let Some(p) = read_sequence_or_choice(reader, &mut inner, b"choice")? {
                            content = ComplexContent::Model(p);
                        }
                        depth += 1;
                    }
                    b"all" => {
                        let mut inner = Vec::new();
                        if let Some(p) = read_sequence_or_choice(reader, &mut inner, b"all")? {
                            content = ComplexContent::Model(p);
                        }
                        depth += 1;
                    }
                    b"attributeGroup" => {
                        if let Some(r) = get_attr(&e, "ref") {
                            attribute_groups.push(r);
                        }
                    }
                    b"attribute" => {
                        let name = get_attr(&e, "name");
                        let type_name = get_attr(&e, "type");
                        let use_val = get_attr(&e, "use");
                        let required = use_val.as_deref() == Some("required");
                        let default_value = get_attr(&e, "default");
                        if let (Some(n), Some(t)) = (name, type_name) {
                            attributes.push(Attribute {
                                name: n,
                                type_name: t,
                                required,
                                default_value,
                            });
                        }
                    }
                    _ => {}
                }
                if local.as_slice() == b"complexType" {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                if local_name(e.name().as_ref()).as_slice() == b"complexType" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(Some(ComplexType {
        doc,
        content,
        attribute_groups,
        attributes,
    }))
}

fn read_documentation<R: BufRead>(reader: &mut Reader<R>, buf: &mut Vec<u8>) -> Result<Option<String>> {
    let mut depth = 1u32; // we're inside annotation
    let mut in_doc = false;
    let mut text = String::new();
    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                if local_name(e.name().as_ref()) == b"documentation" {
                    in_doc = true;
                }
                if local_name(e.name().as_ref()) == b"annotation" {
                    depth += 1;
                }
            }
            Ok(Event::Text(e)) => {
                if in_doc {
                    let t = e.unescape().unwrap_or_default().trim().to_string();
                    if !t.is_empty() {
                        text.push_str(&t);
                        text.push(' ');
                    }
                }
            }
            Ok(Event::End(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == b"documentation" {
                    in_doc = false;
                }
                if local.as_slice() == b"annotation" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }
    Ok(Some(text.trim().to_string()).filter(|s| !s.is_empty()))
}

fn read_extension<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<(String, Vec<String>, Vec<Attribute>)> {
    let mut base = String::new();
    let mut ags = Vec::new();
    let mut attrs = Vec::new();
    let mut depth = 2u32; // extension + simpleContent/complexContent
    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == b"extension" {
                    base = get_attr(&e, "base").unwrap_or_default();
                }
                if local.as_slice() == b"attributeGroup" {
                    if let Some(r) = get_attr(&e, "ref") {
                        ags.push(r);
                    }
                }
                if local.as_slice() == b"attribute" {
                    let name = get_attr(&e, "name");
                    let type_name = get_attr(&e, "type");
                    let required = get_attr(&e, "use").as_deref() == Some("required");
                    if let (Some(n), Some(t)) = (name, type_name) {
                        attrs.push(Attribute { name: n, type_name: t, required, default_value: None });
                    }
                }
                if local.as_slice() == b"extension" || local.as_slice() == b"simpleContent" || local.as_slice() == b"complexContent" {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == b"extension" || local.as_slice() == b"simpleContent" || local.as_slice() == b"complexContent" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }
    if base.is_empty() {
        base = "xs:string".to_string();
    }
    Ok((base, ags, attrs))
}

fn read_sequence_or_choice<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    kind: &[u8],
) -> Result<Option<Particle>> {
    let mut children = Vec::new();
    let mut depth = 1u32;
    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                match local.as_slice() {
                    b"element" => {
                        let name = get_attr(&e, "name").unwrap_or_default();
                        let type_name = get_attr(&e, "type");
                        let min_occurs = get_attr(&e, "minOccurs").map(|s| parse_u32(&s)).unwrap_or(1);
                        let max_occurs = get_attr(&e, "maxOccurs").and_then(|s| if s == "unbounded" { None } else { s.parse().ok() });
                        if !name.is_empty() {
                            children.push(Particle::Element(ElementParticle {
                                name,
                                type_name,
                                min_occurs,
                                max_occurs,
                            }));
                        }
                    }
                    b"group" => {
                        if let Some(r) = get_attr(&e, "ref") {
                            children.push(Particle::GroupRef(r));
                        }
                    }
                    b"sequence" => {
                        depth += 1;
                        let mut inner = Vec::new();
                        if let Some(p) = read_sequence_or_choice(reader, &mut inner, b"sequence")? {
                            children.push(p);
                        }
                    }
                    b"choice" => {
                        depth += 1;
                        let mut inner = Vec::new();
                        if let Some(p) = read_sequence_or_choice(reader, &mut inner, b"choice")? {
                            children.push(p);
                        }
                    }
                    _ => {}
                }
                if local.as_slice() == b"sequence" || local.as_slice() == b"choice" || local.as_slice() == b"all" {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == kind || local.as_slice() == b"sequence" || local.as_slice() == b"choice" || local.as_slice() == b"all" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }
    let particle = if kind == b"choice" {
        Particle::Choice(children)
    } else {
        Particle::Sequence(children)
    };
    Ok(Some(particle))
}

fn read_group<R: BufRead>(reader: &mut Reader<R>, buf: &mut Vec<u8>) -> Result<Option<Group>> {
    let mut depth = 1u32;
    let mut content = None;
    let mut doc = None;
    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == b"annotation" {
                    let mut inner = Vec::new();
                    doc = read_documentation(reader, &mut inner)?;
                }
                if local.as_slice() == b"sequence" {
                    let mut inner = Vec::new();
                    content = read_sequence_or_choice(reader, &mut inner, b"sequence")?;
                }
                if local.as_slice() == b"choice" {
                    let mut inner = Vec::new();
                    content = read_sequence_or_choice(reader, &mut inner, b"choice")?;
                }
                if local.as_slice() == b"group" {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                if local_name(e.name().as_ref()).as_slice() == b"group" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }
    Ok(Some(Group {
        doc,
        content: content.unwrap_or(Particle::Sequence(vec![])),
    }))
}

fn read_attribute_group<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<Option<AttributeGroup>> {
    let mut doc = None;
    let mut attributes = Vec::new();
    let mut attribute_group_refs = Vec::new();
    let mut depth = 1u32;
    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == b"annotation" {
                    let mut inner = Vec::new();
                    doc = read_documentation(reader, &mut inner)?;
                }
                if local.as_slice() == b"attribute" {
                    let name = get_attr(&e, "name");
                    let type_name = get_attr(&e, "type");
                    let required = get_attr(&e, "use").as_deref() == Some("required");
                    if let (Some(n), Some(t)) = (name, type_name) {
                        attributes.push(Attribute { name: n, type_name: t, required, default_value: None });
                    }
                }
                if local.as_slice() == b"attributeGroup" {
                    if let Some(r) = get_attr(&e, "ref") {
                        attribute_group_refs.push(r);
                    }
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                if local_name(e.name().as_ref()).as_slice() == b"attributeGroup" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }
    Ok(Some(AttributeGroup {
        doc,
        attributes,
        attribute_group_refs,
    }))
}

fn read_element_decl<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    type_ref: Option<String>,
) -> Result<Option<ElementDecl>> {
    let mut depth = 1u32;
    let mut inline_complex_type = None;
    let mut doc = None;
    while depth > 0 {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref());
                if local.as_slice() == b"annotation" {
                    let mut inner = Vec::new();
                    doc = read_documentation(reader, &mut inner)?;
                }
                if local.as_slice() == b"complexType" && get_attr(&e, "name").is_none() {
                    let mut inner = Vec::new();
                    inline_complex_type = read_complex_type(reader, &mut inner)?;
                }
                if local.as_slice() == b"element" {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                if local_name(e.name().as_ref()).as_slice() == b"element" {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }
    Ok(Some(ElementDecl {
        doc,
        type_name: type_ref,
        inline_complex_type,
    }))
}

//! MEI RNG (Relax NG) parser.
//!
//! Parses mei-all.rng and builds the same OddDefinitions structure as the ODD
//! parser, so the existing generator can emit Rust code from either source.
//!
//! RNG is the target source for MEI 6.0-dev; when --rng is passed, ODD is not used.

use anyhow::{Context, Result, bail};
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::ast::*;

/// Parse MEI RNG file and produce OddDefinitions for the generator.
pub fn parse_rng_file(path: &Path) -> Result<OddDefinitions> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read RNG: {}", path.display()))?;
    let bytes = content.as_bytes();
    let defines = collect_rng_defines(bytes)?;
    let defs = rng_to_odd(&defines)?;
    Ok(defs)
}

/// Raw content of a single RNG define (mei_* only).
#[derive(Debug, Default)]
struct RngDefine {
    name: String,
    #[allow(dead_code)] // reserved for future combine-attribute handling
    combine: Option<String>,
    /// True if the element content model includes <text/> (mixed content).
    has_text: bool,
    refs: Vec<String>,
    values: Vec<String>,
    data_type: Option<String>,
    element_name: Option<String>,
    doc: String,
    att_name: Option<String>,
}

fn get_attr(e: &BytesStart, key: &[u8]) -> Option<String> {
    e.attributes()
        .find(|a| a.as_ref().map(|a| a.key.as_ref() == key).unwrap_or(false))
        .and_then(|a| a.ok())
        .map(|a| String::from_utf8_lossy(&a.value).to_string())
}

/// Collect all <define name="mei_..."> and their direct ref/value/data/element content.
fn collect_rng_defines(content: &[u8]) -> Result<HashMap<String, RngDefine>> {
    let mut defines = HashMap::new();
    let mut reader = Reader::from_reader(content);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut depth: i32 = 0;
    let mut current: Option<RngDefine> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let local = Vec::from(e.name().as_ref());
                let local = local.as_slice();
                if local == b"define" && depth == 0 {
                    let name = get_attr(&e, b"name").unwrap_or_default();
                    let combine = get_attr(&e, b"combine");
                    if name.starts_with("mei_") && !name.starts_with("mei_svg") {
                        current = Some(RngDefine {
                            name: name.clone(),
                            combine,
                            ..Default::default()
                        });
                        depth = 1;
                    }
                } else if depth >= 1 {
                    if let Some(cur) = current.as_mut() {
                        match local {
                            b"ref" => {
                                if let Some(n) = get_attr(&e, b"name") {
                                    cur.refs.push(n);
                                }
                            }
                            b"value" => {
                                if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                                    cur.values
                                        .push(t.unescape().unwrap_or_default().to_string());
                                }
                            }
                            b"data" => {
                                cur.data_type = get_attr(&e, b"type");
                            }
                            b"element" => {
                                cur.element_name = get_attr(&e, b"name");
                            }
                            b"attribute" => {
                                cur.att_name = get_attr(&e, b"name");
                            }
                            b"a:documentation" => {
                                if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                                    let s = t.unescape().unwrap_or_default().to_string();
                                    if !s.trim().is_empty() {
                                        cur.doc = s.trim().to_string();
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    depth += 1;
                }
            }
            Ok(Event::Empty(e)) => {
                if depth >= 1 {
                    if let Some(cur) = current.as_mut() {
                        match e.name().as_ref() {
                            b"ref" => {
                                if let Some(n) = get_attr(&e, b"name") {
                                    cur.refs.push(n);
                                }
                            }
                            b"data" => {
                                cur.data_type = get_attr(&e, b"type");
                            }
                            b"text" => {
                                cur.has_text = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"define" && depth == 1 {
                    if let Some(d) = current.take() {
                        defines.insert(d.name.clone(), d);
                    }
                    depth = 0;
                } else if depth > 0 {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(defines)
}

/// Convert RNG defines to OddDefinitions. ODD ident style: data.X, att.X, model.X, element name.
fn rng_to_odd(defines: &HashMap<String, RngDefine>) -> Result<OddDefinitions> {
    let mut defs = OddDefinitions::new();
    let module = "MEI".to_string();

    // 1) Data types: mei_data.X -> data.X
    for (name, d) in defines.iter().filter(|(n, _)| n.starts_with("mei_data.")) {
        let ident = name.strip_prefix("mei_").unwrap_or(name).to_string();
        let kind = if !d.values.is_empty() {
            DataTypeKind::ValList(
                d.values
                    .iter()
                    .map(|v| DataTypeValue {
                        ident: v.clone(),
                        desc: String::new(),
                    })
                    .collect(),
            )
        } else if let Some(ref dt) = d.data_type {
            DataTypeKind::Primitive {
                type_name: dt.clone(),
                pattern: None,
                min_inclusive: None,
                max_inclusive: None,
            }
        } else if d.refs.len() == 1 {
            DataTypeKind::Reference(d.refs[0].clone())
        } else if !d.refs.is_empty() {
            DataTypeKind::Choice(
                d.refs
                    .iter()
                    .map(|r| DataTypeRef::RngRef(r.clone()))
                    .collect(),
            )
        } else {
            continue;
        };
        defs.data_types.insert(
            ident.clone(),
            DataType {
                ident,
                module: module.clone(),
                desc: d.doc.clone(),
                kind,
            },
        );
    }

    // 2) Attribute classes: mei_att.X.attributes -> att.X; refs to .attributes are member_of
    for (name, d) in defines
        .iter()
        .filter(|(n, _)| n.ends_with(".attributes") && n.starts_with("mei_att."))
    {
        let att_ident = name
            .strip_prefix("mei_att.")
            .and_then(|s| s.strip_suffix(".attributes"))
            .map(|s| format!("att.{}", s))
            .unwrap_or_else(|| name.clone());
        let member_of: Vec<String> = d
            .refs
            .iter()
            .filter(|r| r.ends_with(".attributes"))
            .map(|r| {
                r.strip_prefix("mei_att.")
                    .and_then(|s| s.strip_suffix(".attributes"))
                    .map(|s| format!("att.{}", s))
                    .unwrap_or_else(|| r.clone())
            })
            .collect();
        let mut attributes = Vec::new();
        for r in &d.refs {
            if r.contains(".attribute.") {
                if let Some(attr_def) = defines.get(r) {
                    if let Some(ref an) = attr_def.att_name {
                        // RNG can use <ref name="mei_data.X"/> or <data type="mei_data.X"/> for type
                        let datatype = attr_def
                            .refs
                            .first()
                            .map(|t| AttributeDataType::Ref(t.clone()))
                            .or_else(|| {
                                attr_def
                                    .data_type
                                    .as_ref()
                                    .map(|t| AttributeDataType::Ref(t.clone()))
                            });
                        attributes.push(Attribute {
                            ident: an.clone(),
                            desc: attr_def.doc.clone(),
                            usage: "opt".to_string(),
                            datatype,
                            default_val: None,
                            max_occurs: None,
                            constraints: vec![],
                        });
                    }
                }
            }
        }
        defs.att_classes.insert(
            att_ident.clone(),
            AttClass {
                ident: att_ident,
                module: module.clone(),
                desc: d.doc.clone(),
                member_of,
                attributes,
                constraints: vec![],
            },
        );
    }

    // 3) Model classes: mei_model.X combine=choice -> model.X
    for (name, _d) in defines.iter().filter(|(n, _)| n.starts_with("mei_model.")) {
        let model_ident = name
            .strip_prefix("mei_")
            .map(|s| s.to_string())
            .unwrap_or_else(|| name.clone());
        defs.model_classes.insert(
            model_ident.clone(),
            ModelClass {
                ident: model_ident,
                module: module.clone(),
                desc: String::new(),
                member_of: vec![],
            },
        );
    }

    // 4) Elements: mei_X with element name="x" -> Element ident x; refs to .attributes are member_of
    for (name, d) in defines.iter() {
        if let Some(ref elem_name) = d.element_name {
            if name.starts_with("mei_")
                && !name.starts_with("mei_att.")
                && !name.starts_with("mei_data.")
                && !name.starts_with("mei_model.")
            {
                let att_member_of: Vec<String> = d
                    .refs
                    .iter()
                    .filter(|r| r.ends_with(".attributes"))
                    .map(|r| {
                        r.strip_prefix("mei_att.")
                            .and_then(|s| s.strip_suffix(".attributes"))
                            .map(|s| format!("att.{}", s))
                            .unwrap_or_else(|| r.clone())
                    })
                    .collect();
                let content_refs: Vec<String> = d
                    .refs
                    .iter()
                    .filter(|r| !r.ends_with(".attributes"))
                    .cloned()
                    .collect();
                let mut content: Vec<ContentItem> = content_refs
                    .into_iter()
                    .map(|r| {
                        if r.starts_with("mei_") {
                            ContentItem::Ref(r.strip_prefix("mei_").unwrap_or(&r).to_string())
                        } else {
                            ContentItem::Ref(r)
                        }
                    })
                    .collect::<Vec<_>>();
                if d.has_text {
                    content.push(ContentItem::Text);
                }
                let content_model = if content.is_empty() {
                    vec![ContentItem::Empty]
                } else {
                    vec![ContentItem::ZeroOrMore(Box::new(vec![
                        ContentItem::Choice(vec![content]),
                    ]))]
                };
                defs.elements.insert(
                    elem_name.clone(),
                    Element {
                        ident: elem_name.clone(),
                        module: module.clone(),
                        gloss: elem_name.clone(),
                        desc: d.doc.clone(),
                        member_of: att_member_of,
                        content: content_model,
                        constraints: vec![],
                        local_attributes: vec![],
                    },
                );
            }
        }
    }

    Ok(defs)
}

//! Hand-written MeiDeserialize for the root <mei> element with extension support.
//!
//! Collects attributes and elements in the extension namespace (ext:) into
//! ExtensionBag so round-trip preserves custom data.

use super::super::{AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader};
use std::io::BufRead;
use tusk_model::elements::{Mei, MeiChild, MeiHead, Music};
use tusk_model::extensions::{ExtensionBag, TUSK_EXT_NS};

impl MeiDeserialize for Mei {
    fn element_name() -> &'static str {
        "mei"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut result = Mei::default();

        let mut custom_attributes: Vec<(String, String, String)> = Vec::new();
        let ext_prefix = "ext:";
        let mut rest_attrs = AttributeMap::new();
        for (k, v) in attrs.drain() {
            if k.starts_with(ext_prefix) {
                let local = k[ext_prefix.len()..].to_string();
                custom_attributes.push((TUSK_EXT_NS.to_string(), local, v));
            } else {
                rest_attrs.insert(k, v);
            }
        }
        attrs = rest_attrs;

        result.id.extract_attributes(&mut attrs)?;
        result.mei_version.extract_attributes(&mut attrs)?;
        result.responsibility.extract_attributes(&mut attrs)?;

        if !custom_attributes.is_empty() {
            result
                .extensions
                .get_or_insert_with(ExtensionBag::default)
                .custom_attributes = custom_attributes;
        }

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("mei")?
            {
                if name.starts_with("ext:") {
                    reader.skip_unknown_child(&name, "mei", child_empty)?;
                    if result.extensions.is_none() {
                        result.extensions = Some(ExtensionBag::default());
                    }
                    let bag = result.extensions.as_mut().unwrap();
                    let local = name["ext:".len()..].to_string();
                    let el_attrs: Vec<(String, String, String)> = child_attrs
                        .into_iter()
                        .map(|(k, v)| (TUSK_EXT_NS.to_string(), k, v))
                        .collect();
                    bag.custom_elements.push(tusk_model::ExtensionElement {
                        namespace: TUSK_EXT_NS.to_string(),
                        local_name: local,
                        attributes: el_attrs,
                        content: None,
                    });
                } else {
                    match name.as_str() {
                        "meiHead" => {
                            let elem = MeiHead::from_mei_event(reader, child_attrs, child_empty)?;
                            result.children.push(MeiChild::MeiHead(Box::new(elem)));
                        }
                        "music" => {
                            let elem = Music::from_mei_event(reader, child_attrs, child_empty)?;
                            result.children.push(MeiChild::Music(Box::new(elem)));
                        }
                        _ => {
                            reader.skip_unknown_child(&name, "mei", child_empty)?;
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}

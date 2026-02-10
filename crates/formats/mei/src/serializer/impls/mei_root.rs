//! Hand-written MeiSerialize for the root <mei> element with extension support.
//!
//! Writes extension namespace (xmlns:ext) and extension bag attributes/children
//! so round-trip preserves custom data.

use super::super::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::Mei;
use tusk_model::extensions::TUSK_EXT_NS;

impl MeiSerialize for Mei {
    fn element_name(&self) -> &'static str {
        "mei"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.id.collect_attributes());
        attrs.extend(self.mei_version.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
            || self
                .extensions
                .as_ref()
                .is_some_and(|e| !e.custom_elements.is_empty())
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            MeiSerialize::serialize_mei(child, writer)?;
        }
        if let Some(ref ext) = self.extensions {
            for el in &ext.custom_elements {
                let qname = format!("ext:{}", el.local_name);
                let mut start = writer.start_element(&qname)?;
                for (_, local, value) in &el.attributes {
                    start.push_attribute((format!("ext:{}", local).as_str(), value.as_str()));
                }
                match &el.content {
                    Some(tusk_model::ExtensionContent::Raw(s)) => {
                        writer.write_start(start)?;
                        writer.write_text(s)?;
                        writer.write_end(&qname)?;
                    }
                    Some(tusk_model::ExtensionContent::Children(children)) => {
                        if children.is_empty() {
                            writer.write_empty(start)?;
                        } else {
                            writer.write_start(start)?;
                            for child in children {
                                serialize_extension_element(child, writer)?;
                            }
                            writer.write_end(&qname)?;
                        }
                    }
                    None => {
                        writer.write_empty(start)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        writer.write_declaration()?;
        let name = self.element_name();
        let attrs = self.collect_all_attributes();
        let mut start = writer.start_element(name)?;
        for (attr_name, value) in &attrs {
            start.push_attribute((*attr_name, value.as_str()));
        }
        writer.add_root_namespaces(&mut start);
        start.push_attribute(("xmlns:ext", TUSK_EXT_NS));
        if let Some(ext) = &self.extensions {
            for (_, local, value) in &ext.custom_attributes {
                let key = format!("ext:{}", local);
                start.push_attribute((key.as_str(), value.as_str()));
            }
        }
        if self.has_children() {
            writer.write_start(start)?;
            self.serialize_children(writer)?;
            writer.write_end(name)?;
        } else {
            writer.write_empty(start)?;
        }
        Ok(())
    }
}

fn serialize_extension_element<W: Write>(
    el: &tusk_model::ExtensionElement,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    let qname = format!("ext:{}", el.local_name);
    let mut start = writer.start_element(&qname)?;
    for (_, local, value) in &el.attributes {
        start.push_attribute((format!("ext:{}", local).as_str(), value.as_str()));
    }
    match &el.content {
        Some(tusk_model::ExtensionContent::Raw(s)) => {
            writer.write_start(start)?;
            writer.write_text(s)?;
            writer.write_end(&qname)?;
        }
        Some(tusk_model::ExtensionContent::Children(children)) => {
            if children.is_empty() {
                writer.write_empty(start)?;
            } else {
                writer.write_start(start)?;
                for child in children {
                    serialize_extension_element(child, writer)?;
                }
                writer.write_end(&qname)?;
            }
        }
        None => {
            writer.write_empty(start)?;
        }
    }
    Ok(())
}

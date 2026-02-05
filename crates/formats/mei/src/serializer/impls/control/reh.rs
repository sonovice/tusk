//! Serializer implementations for rehearsal marks and anchored text: Reh, AnchoredText.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAnchoredTextAnl, AttAnchoredTextGes, AttAnchoredTextLog, AttAnchoredTextVis, AttRehAnl,
    AttRehGes, AttRehLog, AttRehVis,
};
use tusk_model::elements::{AnchoredText, AnchoredTextChild, Reh, RehChild};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

impl CollectAttributes for AttRehLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}

impl CollectAttributes for AttRehVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttRehGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRehGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttRehAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRehAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Reh {
    fn element_name(&self) -> &'static str {
        "reh"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.reh_log.collect_attributes());
        attrs.extend(self.reh_vis.collect_attributes());
        attrs.extend(self.reh_ges.collect_attributes());
        attrs.extend(self.reh_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                RehChild::Text(text) => {
                    writer.write_text(text)?;
                }
                RehChild::Rend(rend) => {
                    rend.serialize_mei(writer)?;
                }
                RehChild::Stack(_) => {
                    // Stack serialization not yet implemented - skipping
                    // This is rare in practice for reh elements
                }
                RehChild::Lb(lb) => {
                    lb.serialize_mei(writer)?;
                }
            }
        }
        Ok(())
    }
}
// ============================================================================
// AnchoredText attribute class implementations
// ============================================================================

impl CollectAttributes for AttAnchoredTextLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttAnchoredTextVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttAnchoredTextGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttAnchoredTextGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttAnchoredTextAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttAnchoredTextAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for AnchoredText {
    fn element_name(&self) -> &'static str {
        "anchoredText"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.anchored_text_log.collect_attributes());
        attrs.extend(self.anchored_text_vis.collect_attributes());
        attrs.extend(self.anchored_text_ges.collect_attributes());
        attrs.extend(self.anchored_text_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for AnchoredTextChild {
    fn element_name(&self) -> &'static str {
        match self {
            AnchoredTextChild::Text(_) => "#text",
            AnchoredTextChild::Rend(_) => "rend",
            AnchoredTextChild::Lb(_) => "lb",
            AnchoredTextChild::Ref(_) => "ref",
            AnchoredTextChild::PersName(_) => "persName",
            AnchoredTextChild::CorpName(_) => "corpName",
            AnchoredTextChild::Name(_) => "name",
            AnchoredTextChild::Seg(_) => "seg",
            AnchoredTextChild::Title(_) => "title",
            AnchoredTextChild::Identifier(_) => "identifier",
            AnchoredTextChild::Date(_) => "date",
            AnchoredTextChild::Ptr(_) => "ptr",
            // Many other child types exist but are not commonly used
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            AnchoredTextChild::Rend(r) => r.collect_all_attributes(),
            AnchoredTextChild::Lb(lb) => lb.collect_all_attributes(),
            AnchoredTextChild::Ref(r) => r.collect_all_attributes(),
            AnchoredTextChild::PersName(pn) => pn.collect_all_attributes(),
            AnchoredTextChild::CorpName(cn) => cn.collect_all_attributes(),
            AnchoredTextChild::Name(n) => n.collect_all_attributes(),
            AnchoredTextChild::Seg(s) => s.collect_all_attributes(),
            AnchoredTextChild::Title(t) => t.collect_all_attributes(),
            AnchoredTextChild::Identifier(i) => i.collect_all_attributes(),
            AnchoredTextChild::Date(d) => d.collect_all_attributes(),
            AnchoredTextChild::Ptr(p) => p.collect_all_attributes(),
            // Text and other elements - no attributes
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            AnchoredTextChild::Text(_) => false,
            AnchoredTextChild::Rend(r) => !r.children.is_empty(),
            AnchoredTextChild::Lb(_) => false,
            AnchoredTextChild::Ref(r) => !r.children.is_empty(),
            AnchoredTextChild::PersName(pn) => !pn.children.is_empty(),
            AnchoredTextChild::CorpName(cn) => !cn.children.is_empty(),
            AnchoredTextChild::Name(n) => !n.children.is_empty(),
            AnchoredTextChild::Seg(s) => !s.children.is_empty(),
            AnchoredTextChild::Title(t) => !t.children.is_empty(),
            AnchoredTextChild::Identifier(i) => !i.children.is_empty(),
            AnchoredTextChild::Date(d) => !d.children.is_empty(),
            AnchoredTextChild::Ptr(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AnchoredTextChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AnchoredTextChild::Rend(r) => r.serialize_children(writer),
            AnchoredTextChild::Lb(_) => Ok(()),
            AnchoredTextChild::Ref(r) => r.serialize_children(writer),
            AnchoredTextChild::PersName(pn) => pn.serialize_children(writer),
            AnchoredTextChild::CorpName(cn) => cn.serialize_children(writer),
            AnchoredTextChild::Name(n) => n.serialize_children(writer),
            AnchoredTextChild::Seg(s) => s.serialize_children(writer),
            AnchoredTextChild::Title(t) => t.serialize_children(writer),
            AnchoredTextChild::Identifier(i) => i.serialize_children(writer),
            AnchoredTextChild::Date(d) => d.serialize_children(writer),
            AnchoredTextChild::Ptr(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "AnchoredTextChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

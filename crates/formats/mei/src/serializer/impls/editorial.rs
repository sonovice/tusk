//! Serializer implementations for editorial MEI elements.
//!
//! This module contains implementations for App, Lem, Rdg, Choice, Corr, Sic, Add, Del,
//! Abbr, Expan, Orig, Reg, Subst, Supplied, Unclear, Damage, Gap, Restore, HandShift
//! and related attribute classes.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAgentIdent, AttCrit, AttExtent, AttHandIdent, AttMedium, AttRdgAnl, AttRdgGes, AttRdgLog,
    AttRdgVis, AttReasonIdent, AttTrans,
};
use tusk_model::elements::{
    Abbr, Add, AddChild, App, AppChild, Choice, ChoiceChild, Corr, CorrChild, Damage, Del,
    DelChild, Expan, Gap, HandShift, Lem, Orig, Rdg, Reg, Restore, Sic, SicChild, Space, Subst,
    Supplied, Unclear,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

impl CollectAttributes for AttCrit {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "hand", self.hand);
        push_attr!(attrs, "seq", self.seq);
        push_attr!(attrs, "source", vec self.source);
        push_attr!(attrs, "cause", clone self.cause);
        attrs
    }
}

impl CollectAttributes for AttRdgLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRdgLog has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttRdgVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRdgVis has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttRdgGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRdgGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttRdgAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttRdgAnl has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttExtent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unit", self.unit);
        push_attr!(attrs, "atleast", self.atleast);
        push_attr!(attrs, "atmost", self.atmost);
        push_attr!(attrs, "min", self.min);
        push_attr!(attrs, "max", self.max);
        push_attr!(attrs, "confidence", self.confidence);
        push_attr!(attrs, "extent", clone self.extent);
        attrs
    }
}

impl CollectAttributes for AttTrans {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instant", self.instant);
        push_attr!(attrs, "state", vec self.state);
        push_attr!(attrs, "hand", self.hand);
        push_attr!(attrs, "decls", vec self.decls);
        push_attr!(attrs, "seq", self.seq);
        attrs
    }
}

impl CollectAttributes for AttHandIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "hand", self.hand);
        attrs
    }
}

impl CollectAttributes for AttReasonIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "reason", clone self.reason);
        attrs
    }
}

impl CollectAttributes for AttAgentIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "agent", clone self.agent);
        attrs
    }
}

impl CollectAttributes for AttMedium {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "medium", clone self.medium);
        attrs
    }
}

// ============================================================================
// App element implementation
// ============================================================================

impl MeiSerialize for App {
    fn element_name(&self) -> &'static str {
        "app"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                AppChild::Lem(elem) => elem.serialize_mei(writer)?,
                AppChild::Rdg(elem) => elem.serialize_mei(writer)?,
            }
        }
        Ok(())
    }
}

// ============================================================================
// Lem element implementation
// ============================================================================

impl MeiSerialize for Lem {
    fn element_name(&self) -> &'static str {
        "lem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.crit.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.rdg_log.collect_attributes());
        attrs.extend(self.rdg_vis.collect_attributes());
        attrs.extend(self.rdg_ges.collect_attributes());
        attrs.extend(self.rdg_anl.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Lem can contain many children - serialize them using the dynamic dispatch
        for child in &self.children {
            serialize_lem_child(child, writer)?;
        }
        Ok(())
    }
}

// ============================================================================
// Rdg element implementation
// ============================================================================

impl MeiSerialize for Rdg {
    fn element_name(&self) -> &'static str {
        "rdg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.crit.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.rdg_log.collect_attributes());
        attrs.extend(self.rdg_vis.collect_attributes());
        attrs.extend(self.rdg_ges.collect_attributes());
        attrs.extend(self.rdg_anl.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Rdg can contain many children - serialize them using dynamic dispatch
        for child in &self.children {
            serialize_rdg_child(child, writer)?;
        }
        Ok(())
    }
}

// ============================================================================
// Choice element implementation
// ============================================================================

impl MeiSerialize for Choice {
    fn element_name(&self) -> &'static str {
        "choice"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                ChoiceChild::Sic(elem) => elem.serialize_mei(writer)?,
                ChoiceChild::Corr(elem) => elem.serialize_mei(writer)?,
                ChoiceChild::Choice(elem) => elem.serialize_mei(writer)?,
                // Other choice children not yet fully implemented - skip for now
                // These would need their own serializers when implemented
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Corr element implementation
// ============================================================================

impl MeiSerialize for Corr {
    fn element_name(&self) -> &'static str {
        "corr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                CorrChild::Text(text) => writer.write_text(text)?,
                // Other children would need their serializers
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Sic element implementation
// ============================================================================

impl MeiSerialize for Sic {
    fn element_name(&self) -> &'static str {
        "sic"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                SicChild::Text(text) => writer.write_text(text)?,
                // Other children would need their serializers
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Add element implementation
// ============================================================================

impl MeiSerialize for Add {
    fn element_name(&self) -> &'static str {
        "add"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        // Handle place attribute
        push_attr!(attrs, "place", vec self.place);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                AddChild::Text(text) => writer.write_text(text)?,
                AddChild::Space(elem) => elem.serialize_mei(writer)?,
                // Other children would need their serializers
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Del element implementation
// ============================================================================

impl MeiSerialize for Del {
    fn element_name(&self) -> &'static str {
        "del"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.text_rendition.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                DelChild::Text(text) => writer.write_text(text)?,
                // Other children would need their serializers
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Abbr element implementation
// ============================================================================

impl MeiSerialize for Abbr {
    fn element_name(&self) -> &'static str {
        "abbr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        push_attr!(attrs, "expan", clone self.expan);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::AbbrChild;
            match child {
                AbbrChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Expan element implementation
// ============================================================================

impl MeiSerialize for Expan {
    fn element_name(&self) -> &'static str {
        "expan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        push_attr!(attrs, "abbr", clone self.abbr);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::ExpanChild;
            match child {
                ExpanChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Orig element implementation
// ============================================================================

impl MeiSerialize for Orig {
    fn element_name(&self) -> &'static str {
        "orig"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::OrigChild;
            match child {
                OrigChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Reg element implementation
// ============================================================================

impl MeiSerialize for Reg {
    fn element_name(&self) -> &'static str {
        "reg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::RegChild;
            match child {
                RegChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Subst element implementation
// ============================================================================

impl MeiSerialize for Subst {
    fn element_name(&self) -> &'static str {
        "subst"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::SubstChild;
            match child {
                SubstChild::Add(elem) => elem.serialize_mei(writer)?,
                SubstChild::Del(elem) => elem.serialize_mei(writer)?,
                SubstChild::Gap(elem) => elem.serialize_mei(writer)?,
                SubstChild::Reg(elem) => elem.serialize_mei(writer)?,
                SubstChild::Sic(elem) => elem.serialize_mei(writer)?,
                SubstChild::Corr(elem) => elem.serialize_mei(writer)?,
                SubstChild::Damage(elem) => elem.serialize_mei(writer)?,
                SubstChild::HandShift(elem) => elem.serialize_mei(writer)?,
                SubstChild::Restore(elem) => elem.serialize_mei(writer)?,
                SubstChild::Unclear(elem) => elem.serialize_mei(writer)?,
                SubstChild::Orig(elem) => elem.serialize_mei(writer)?,
                SubstChild::Supplied(elem) => elem.serialize_mei(writer)?,
            }
        }
        Ok(())
    }
}

// ============================================================================
// Supplied element implementation
// ============================================================================

impl MeiSerialize for Supplied {
    fn element_name(&self) -> &'static str {
        "supplied"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.agent_ident.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.reason_ident.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::SuppliedChild;
            match child {
                SuppliedChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Unclear element implementation
// ============================================================================

impl MeiSerialize for Unclear {
    fn element_name(&self) -> &'static str {
        "unclear"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.agent_ident.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hand_ident.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.reason_ident.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::UnclearChild;
            match child {
                UnclearChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Damage element implementation
// ============================================================================

impl MeiSerialize for Damage {
    fn element_name(&self) -> &'static str {
        "damage"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.agent_ident.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hand_ident.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        push_attr!(attrs, "degree", clone self.degree);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::DamageChild;
            match child {
                DamageChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Gap element implementation (empty element)
// ============================================================================

impl MeiSerialize for Gap {
    fn element_name(&self) -> &'static str {
        "gap"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.hand_ident.collect_attributes());
        attrs.extend(self.reason_ident.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Gap is an empty element
        Ok(())
    }
}

// ============================================================================
// Restore element implementation
// ============================================================================

impl MeiSerialize for Restore {
    fn element_name(&self) -> &'static str {
        "restore"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        push_attr!(attrs, "desc", clone self.desc);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::RestoreChild;
            match child {
                RestoreChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// HandShift element implementation (empty element)
// ============================================================================

impl MeiSerialize for HandShift {
    fn element_name(&self) -> &'static str {
        "handShift"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.medium.collect_attributes());
        push_attr!(attrs, "character", clone self.character);
        push_attr!(attrs, "new", self.new);
        push_attr!(attrs, "old", self.old);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // HandShift is an empty element
        Ok(())
    }
}

// ============================================================================
// Helper functions for child serialization
// ============================================================================

use tusk_model::elements::{LemChild, RdgChild};

/// Serialize a LemChild variant to the writer.
fn serialize_lem_child<W: Write>(
    child: &LemChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        LemChild::Text(text) => writer.write_text(text),
        // For elements that have serializers implemented, use them
        // Currently skip most children until their serializers are implemented
        _ => Ok(()),
    }
}

/// Serialize a RdgChild variant to the writer.
fn serialize_rdg_child<W: Write>(
    child: &RdgChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        RdgChild::Text(text) => writer.write_text(text),
        // For elements that have serializers implemented, use them
        // Currently skip most children until their serializers are implemented
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // App tests
    // ========================================================================

    #[test]
    fn app_serializes_empty() {
        let app = App::default();
        let xml = app.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<app/>");
    }

    #[test]
    fn app_serializes_with_xml_id() {
        let mut app = App::default();
        app.common.xml_id = Some("app1".to_string());
        let xml = app.to_mei_string().expect("should serialize");
        assert_eq!(xml, r#"<app xml:id="app1"/>"#);
    }

    #[test]
    fn app_serializes_with_lem_and_rdg() {
        let mut app = App::default();
        app.children.push(AppChild::Lem(Box::new(Lem::default())));
        app.children.push(AppChild::Rdg(Box::new(Rdg::default())));
        let xml = app.to_mei_string().expect("should serialize");
        assert!(xml.contains("<lem/>"));
        assert!(xml.contains("<rdg/>"));
    }

    // ========================================================================
    // Lem tests
    // ========================================================================

    #[test]
    fn lem_serializes_empty() {
        let lem = Lem::default();
        let xml = lem.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<lem/>");
    }

    #[test]
    fn lem_serializes_with_source() {
        let mut lem = Lem::default();
        lem.crit.source = vec![tusk_model::data::DataUri("#src1".to_string())];
        let xml = lem.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"source="#src1""##));
    }

    // ========================================================================
    // Rdg tests
    // ========================================================================

    #[test]
    fn rdg_serializes_empty() {
        let rdg = Rdg::default();
        let xml = rdg.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<rdg/>");
    }

    #[test]
    fn rdg_serializes_with_source() {
        let mut rdg = Rdg::default();
        rdg.crit.source = vec![
            tusk_model::data::DataUri("#src1".to_string()),
            tusk_model::data::DataUri("#src2".to_string()),
        ];
        let xml = rdg.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"source="#src1 #src2""##));
    }

    // ========================================================================
    // Choice tests
    // ========================================================================

    #[test]
    fn choice_serializes_empty() {
        let choice = Choice::default();
        let xml = choice.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<choice/>");
    }

    #[test]
    fn choice_serializes_with_sic_corr() {
        let mut choice = Choice::default();
        choice
            .children
            .push(ChoiceChild::Sic(Box::new(Sic::default())));
        choice
            .children
            .push(ChoiceChild::Corr(Box::new(Corr::default())));
        let xml = choice.to_mei_string().expect("should serialize");
        assert!(xml.contains("<sic/>"));
        assert!(xml.contains("<corr/>"));
    }

    // ========================================================================
    // Corr tests
    // ========================================================================

    #[test]
    fn corr_serializes_empty() {
        let corr = Corr::default();
        let xml = corr.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<corr/>");
    }

    #[test]
    fn corr_serializes_with_cert() {
        let mut corr = Corr::default();
        corr.edit.cert = Some(tusk_model::data::DataCertainty::High);
        let xml = corr.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"cert="high""#));
    }

    // ========================================================================
    // Sic tests
    // ========================================================================

    #[test]
    fn sic_serializes_empty() {
        let sic = Sic::default();
        let xml = sic.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<sic/>");
    }

    // ========================================================================
    // Add tests
    // ========================================================================

    #[test]
    fn add_serializes_empty() {
        let add = Add::default();
        let xml = add.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<add/>");
    }

    #[test]
    fn add_serializes_with_hand() {
        let mut add = Add::default();
        add.trans.hand = Some(tusk_model::data::DataUri("#h1".to_string()));
        let xml = add.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"hand="#h1""##));
    }

    // ========================================================================
    // Del tests
    // ========================================================================

    #[test]
    fn del_serializes_empty() {
        let del = Del::default();
        let xml = del.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<del/>");
    }

    #[test]
    fn del_serializes_with_hand() {
        let mut del = Del::default();
        del.trans.hand = Some(tusk_model::data::DataUri("#h1".to_string()));
        let xml = del.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"hand="#h1""##));
    }

    // ========================================================================
    // Abbr tests
    // ========================================================================

    #[test]
    fn abbr_serializes_empty() {
        let abbr = Abbr::default();
        let xml = abbr.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<abbr/>");
    }

    #[test]
    fn abbr_serializes_with_expan_attr() {
        let mut abbr = Abbr::default();
        abbr.expan = Some("Doctor".to_string());
        let xml = abbr.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"expan="Doctor""#));
    }

    // ========================================================================
    // Expan tests
    // ========================================================================

    #[test]
    fn expan_serializes_empty() {
        let expan = Expan::default();
        let xml = expan.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<expan/>");
    }

    #[test]
    fn expan_serializes_with_abbr_attr() {
        let mut expan = Expan::default();
        expan.abbr = Some("Dr.".to_string());
        let xml = expan.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"abbr="Dr.""#));
    }

    // ========================================================================
    // Orig tests
    // ========================================================================

    #[test]
    fn orig_serializes_empty() {
        let orig = Orig::default();
        let xml = orig.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<orig/>");
    }

    // ========================================================================
    // Reg tests
    // ========================================================================

    #[test]
    fn reg_serializes_empty() {
        let reg = Reg::default();
        let xml = reg.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<reg/>");
    }

    // ========================================================================
    // Subst tests
    // ========================================================================

    #[test]
    fn subst_serializes_empty() {
        let subst = Subst::default();
        let xml = subst.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<subst/>");
    }

    #[test]
    fn subst_serializes_with_add_del() {
        use tusk_model::elements::SubstChild;
        let mut subst = Subst::default();
        subst
            .children
            .push(SubstChild::Del(Box::new(Del::default())));
        subst
            .children
            .push(SubstChild::Add(Box::new(Add::default())));
        let xml = subst.to_mei_string().expect("should serialize");
        assert!(xml.contains("<del/>"));
        assert!(xml.contains("<add/>"));
    }

    // ========================================================================
    // Supplied tests
    // ========================================================================

    #[test]
    fn supplied_serializes_empty() {
        let supplied = Supplied::default();
        let xml = supplied.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<supplied/>");
    }

    #[test]
    fn supplied_serializes_with_reason() {
        let mut supplied = Supplied::default();
        supplied.reason_ident.reason = Some("lost".to_string());
        let xml = supplied.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"reason="lost""#));
    }

    // ========================================================================
    // Unclear tests
    // ========================================================================

    #[test]
    fn unclear_serializes_empty() {
        let unclear = Unclear::default();
        let xml = unclear.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<unclear/>");
    }

    #[test]
    fn unclear_serializes_with_reason() {
        let mut unclear = Unclear::default();
        unclear.reason_ident.reason = Some("faded".to_string());
        let xml = unclear.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"reason="faded""#));
    }

    // ========================================================================
    // Damage tests
    // ========================================================================

    #[test]
    fn damage_serializes_empty() {
        let damage = Damage::default();
        let xml = damage.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<damage/>");
    }

    #[test]
    fn damage_serializes_with_degree() {
        let mut damage = Damage::default();
        damage.degree = Some("medium".to_string());
        let xml = damage.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"degree="medium""#));
    }

    // ========================================================================
    // Gap tests
    // ========================================================================

    #[test]
    fn gap_serializes_empty() {
        let gap = Gap::default();
        let xml = gap.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<gap/>");
    }

    #[test]
    fn gap_serializes_with_reason() {
        let mut gap = Gap::default();
        gap.reason_ident.reason = Some("illegible".to_string());
        let xml = gap.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"reason="illegible""#));
    }

    // ========================================================================
    // Restore tests
    // ========================================================================

    #[test]
    fn restore_serializes_empty() {
        let restore = Restore::default();
        let xml = restore.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<restore/>");
    }

    #[test]
    fn restore_serializes_with_desc() {
        let mut restore = Restore::default();
        restore.desc = Some("deleted and restored".to_string());
        let xml = restore.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"desc="deleted and restored""#));
    }

    // ========================================================================
    // HandShift tests
    // ========================================================================

    #[test]
    fn hand_shift_serializes_empty() {
        let hand_shift = HandShift::default();
        let xml = hand_shift.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<handShift/>");
    }

    #[test]
    fn hand_shift_serializes_with_new() {
        let mut hand_shift = HandShift::default();
        hand_shift.new = Some(tusk_model::data::DataUri("#h2".to_string()));
        let xml = hand_shift.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"new="#h2""##));
    }
}

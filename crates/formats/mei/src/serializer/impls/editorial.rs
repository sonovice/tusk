//! Serializer implementations for editorial MEI elements.
//!
//! This module contains implementations for App, Lem, Rdg, Choice, Corr, Sic, Add, Del
//! and related attribute classes.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{AttCrit, AttExtent, AttRdgAnl, AttRdgGes, AttRdgLog, AttRdgVis, AttTrans};
use tusk_model::elements::{
    Add, AddChild, App, AppChild, Choice, ChoiceChild, Corr, CorrChild, Del, DelChild, Lem, Rdg,
    Sic, SicChild,
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
}

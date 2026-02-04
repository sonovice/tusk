//! Deserializer implementations for editorial MEI elements.
//!
//! This module contains implementations for App, Lem, Rdg, Choice, Corr, Sic, Add, Del
//! and related attribute classes.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttCrit, AttExtent, AttRdgAnl, AttRdgGes, AttRdgLog, AttRdgVis, AttTextRendition, AttTrans,
};
use tusk_model::elements::{Add, App, AppChild, Choice, ChoiceChild, Corr, Del, Lem, Rdg, Sic};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

impl ExtractAttributes for AttCrit {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "hand", self.hand);
        extract_attr!(attrs, "seq", self.seq);
        extract_attr!(attrs, "source", vec self.source);
        extract_attr!(attrs, "cause", string self.cause);
        Ok(())
    }
}

impl ExtractAttributes for AttRdgLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRdgLog has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRdgVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRdgVis has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRdgGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRdgGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRdgAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRdgAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttExtent {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "unit", self.unit);
        extract_attr!(attrs, "atleast", self.atleast);
        extract_attr!(attrs, "atmost", self.atmost);
        extract_attr!(attrs, "min", self.min);
        extract_attr!(attrs, "max", self.max);
        extract_attr!(attrs, "confidence", self.confidence);
        extract_attr!(attrs, "extent", string self.extent);
        Ok(())
    }
}

impl ExtractAttributes for AttTrans {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instant", self.instant);
        extract_attr!(attrs, "state", vec self.state);
        extract_attr!(attrs, "hand", self.hand);
        extract_attr!(attrs, "decls", vec self.decls);
        extract_attr!(attrs, "seq", self.seq);
        Ok(())
    }
}

impl ExtractAttributes for AttTextRendition {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altrend", vec_string self.altrend);
        extract_attr!(attrs, "rend", vec self.rend);
        Ok(())
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for App {
    fn element_name() -> &'static str {
        "app"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut app = App::default();

        // Extract attributes
        app.common.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // App can contain: lem*, rdg*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("app")?
            {
                match name.as_str() {
                    "lem" => {
                        let lem = parse_lem_from_event(reader, child_attrs, child_empty)?;
                        app.children.push(AppChild::Lem(Box::new(lem)));
                    }
                    "rdg" => {
                        let rdg = parse_rdg_from_event(reader, child_attrs, child_empty)?;
                        app.children.push(AppChild::Rdg(Box::new(rdg)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(app)
    }
}

impl MeiDeserialize for Lem {
    fn element_name() -> &'static str {
        "lem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_lem_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Rdg {
    fn element_name() -> &'static str {
        "rdg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_rdg_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Choice {
    fn element_name() -> &'static str {
        "choice"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut choice = Choice::default();

        // Extract attributes
        choice.common.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // Choice can contain: unclear*, abbr*, expan*, choice*, sic*, orig*, subst*, reg*, corr*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("choice")?
            {
                match name.as_str() {
                    "sic" => {
                        let sic = parse_sic_from_event(reader, child_attrs, child_empty)?;
                        choice.children.push(ChoiceChild::Sic(Box::new(sic)));
                    }
                    "corr" => {
                        let corr = parse_corr_from_event(reader, child_attrs, child_empty)?;
                        choice.children.push(ChoiceChild::Corr(Box::new(corr)));
                    }
                    "choice" => {
                        let nested_choice =
                            Choice::from_mei_event(reader, child_attrs, child_empty)?;
                        choice
                            .children
                            .push(ChoiceChild::Choice(Box::new(nested_choice)));
                    }
                    // For other children (unclear, abbr, expan, orig, subst, reg), skip for now
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(choice)
    }
}

impl MeiDeserialize for Corr {
    fn element_name() -> &'static str {
        "corr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_corr_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Sic {
    fn element_name() -> &'static str {
        "sic"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sic_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Add {
    fn element_name() -> &'static str {
        "add"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_add_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Del {
    fn element_name() -> &'static str {
        "del"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_del_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Helper parse functions
// ============================================================================

/// Parse a `<lem>` element from within another element.
fn parse_lem_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Lem> {
    let mut lem = Lem::default();

    // Extract attributes
    lem.common.extract_attributes(&mut attrs)?;
    lem.crit.extract_attributes(&mut attrs)?;
    lem.pointing.extract_attributes(&mut attrs)?;
    lem.rdg_log.extract_attributes(&mut attrs)?;
    lem.rdg_vis.extract_attributes(&mut attrs)?;
    lem.rdg_ges.extract_attributes(&mut attrs)?;
    lem.rdg_anl.extract_attributes(&mut attrs)?;
    lem.target_eval.extract_attributes(&mut attrs)?;

    // Lem can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("lem")?;
    }

    Ok(lem)
}

/// Parse a `<rdg>` element from within another element.
fn parse_rdg_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Rdg> {
    let mut rdg = Rdg::default();

    // Extract attributes
    rdg.common.extract_attributes(&mut attrs)?;
    rdg.crit.extract_attributes(&mut attrs)?;
    rdg.pointing.extract_attributes(&mut attrs)?;
    rdg.rdg_log.extract_attributes(&mut attrs)?;
    rdg.rdg_vis.extract_attributes(&mut attrs)?;
    rdg.rdg_ges.extract_attributes(&mut attrs)?;
    rdg.rdg_anl.extract_attributes(&mut attrs)?;
    rdg.target_eval.extract_attributes(&mut attrs)?;

    // Rdg can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("rdg")?;
    }

    Ok(rdg)
}

/// Parse a `<corr>` element from within another element.
fn parse_corr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Corr> {
    let mut corr = Corr::default();

    // Extract attributes
    corr.common.extract_attributes(&mut attrs)?;
    corr.edit.extract_attributes(&mut attrs)?;
    corr.extent.extract_attributes(&mut attrs)?;
    corr.lang.extract_attributes(&mut attrs)?;
    corr.trans.extract_attributes(&mut attrs)?;

    // Corr can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("corr")?;
    }

    Ok(corr)
}

/// Parse a `<sic>` element from within another element.
fn parse_sic_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Sic> {
    let mut sic = Sic::default();

    // Extract attributes
    sic.common.extract_attributes(&mut attrs)?;
    sic.edit.extract_attributes(&mut attrs)?;
    sic.extent.extract_attributes(&mut attrs)?;
    sic.facsimile.extract_attributes(&mut attrs)?;
    sic.lang.extract_attributes(&mut attrs)?;

    // Sic can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("sic")?;
    }

    Ok(sic)
}

/// Parse an `<add>` element from within another element.
fn parse_add_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Add> {
    let mut add = Add::default();

    // Extract attributes
    add.common.extract_attributes(&mut attrs)?;
    add.facsimile.extract_attributes(&mut attrs)?;
    add.edit.extract_attributes(&mut attrs)?;
    add.extent.extract_attributes(&mut attrs)?;
    add.lang.extract_attributes(&mut attrs)?;
    add.trans.extract_attributes(&mut attrs)?;

    // Add can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("add")?;
    }

    Ok(add)
}

/// Parse a `<del>` element from within another element.
fn parse_del_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Del> {
    let mut del = Del::default();

    // Extract attributes
    del.common.extract_attributes(&mut attrs)?;
    del.edit.extract_attributes(&mut attrs)?;
    del.extent.extract_attributes(&mut attrs)?;
    del.facsimile.extract_attributes(&mut attrs)?;
    del.lang.extract_attributes(&mut attrs)?;
    del.text_rendition.extract_attributes(&mut attrs)?;
    del.trans.extract_attributes(&mut attrs)?;

    // Del can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("del")?;
    }

    Ok(del)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // App tests
    // ========================================================================

    #[test]
    fn app_deserializes_empty() {
        let xml = r#"<app/>"#;
        let app = App::from_mei_str(xml).expect("should deserialize");

        assert!(app.common.xml_id.is_none());
        assert!(app.children.is_empty());
    }

    #[test]
    fn app_deserializes_with_xml_id() {
        let xml = r#"<app xml:id="app1"/>"#;
        let app = App::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app.common.xml_id, Some("app1".to_string()));
    }

    #[test]
    fn app_deserializes_with_lem_and_rdg() {
        let xml = r#"<app>
            <lem/>
            <rdg/>
        </app>"#;
        let app = App::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app.children.len(), 2);
        assert!(matches!(app.children[0], AppChild::Lem(_)));
        assert!(matches!(app.children[1], AppChild::Rdg(_)));
    }

    // ========================================================================
    // Lem tests
    // ========================================================================

    #[test]
    fn lem_deserializes_empty() {
        let xml = r#"<lem/>"#;
        let lem = Lem::from_mei_str(xml).expect("should deserialize");

        assert!(lem.common.xml_id.is_none());
    }

    #[test]
    fn lem_deserializes_with_source() {
        let xml = r##"<lem source="#src1"/>"##;
        let lem = Lem::from_mei_str(xml).expect("should deserialize");

        assert_eq!(lem.crit.source.len(), 1);
    }

    // ========================================================================
    // Rdg tests
    // ========================================================================

    #[test]
    fn rdg_deserializes_empty() {
        let xml = r#"<rdg/>"#;
        let rdg = Rdg::from_mei_str(xml).expect("should deserialize");

        assert!(rdg.common.xml_id.is_none());
    }

    #[test]
    fn rdg_deserializes_with_source() {
        let xml = r##"<rdg source="#src1 #src2"/>"##;
        let rdg = Rdg::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rdg.crit.source.len(), 2);
    }

    // ========================================================================
    // Choice tests
    // ========================================================================

    #[test]
    fn choice_deserializes_empty() {
        let xml = r#"<choice/>"#;
        let choice = Choice::from_mei_str(xml).expect("should deserialize");

        assert!(choice.common.xml_id.is_none());
        assert!(choice.children.is_empty());
    }

    #[test]
    fn choice_deserializes_with_sic_corr() {
        let xml = r#"<choice>
            <sic/>
            <corr/>
        </choice>"#;
        let choice = Choice::from_mei_str(xml).expect("should deserialize");

        assert_eq!(choice.children.len(), 2);
        assert!(matches!(choice.children[0], ChoiceChild::Sic(_)));
        assert!(matches!(choice.children[1], ChoiceChild::Corr(_)));
    }

    // ========================================================================
    // Corr tests
    // ========================================================================

    #[test]
    fn corr_deserializes_empty() {
        let xml = r#"<corr/>"#;
        let corr = Corr::from_mei_str(xml).expect("should deserialize");

        assert!(corr.common.xml_id.is_none());
    }

    #[test]
    fn corr_deserializes_with_cert() {
        let xml = r#"<corr cert="high"/>"#;
        let corr = Corr::from_mei_str(xml).expect("should deserialize");

        assert!(corr.edit.cert.is_some());
    }

    // ========================================================================
    // Sic tests
    // ========================================================================

    #[test]
    fn sic_deserializes_empty() {
        let xml = r#"<sic/>"#;
        let sic = Sic::from_mei_str(xml).expect("should deserialize");

        assert!(sic.common.xml_id.is_none());
    }

    // ========================================================================
    // Add tests
    // ========================================================================

    #[test]
    fn add_deserializes_empty() {
        let xml = r#"<add/>"#;
        let add = Add::from_mei_str(xml).expect("should deserialize");

        assert!(add.common.xml_id.is_none());
    }

    #[test]
    fn add_deserializes_with_hand() {
        let xml = r##"<add hand="#h1"/>"##;
        let add = Add::from_mei_str(xml).expect("should deserialize");

        assert!(add.trans.hand.is_some());
    }

    // ========================================================================
    // Del tests
    // ========================================================================

    #[test]
    fn del_deserializes_empty() {
        let xml = r#"<del/>"#;
        let del = Del::from_mei_str(xml).expect("should deserialize");

        assert!(del.common.xml_id.is_none());
    }

    #[test]
    fn del_deserializes_with_hand() {
        let xml = r##"<del hand="#h1"/>"##;
        let del = Del::from_mei_str(xml).expect("should deserialize");

        assert!(del.trans.hand.is_some());
    }
}

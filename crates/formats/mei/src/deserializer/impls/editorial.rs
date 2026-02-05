//! Deserializer implementations for editorial MEI elements.
//!
//! This module contains implementations for App, Lem, Rdg, Choice, Corr, Sic, Add, Del,
//! Abbr, Expan, Orig, Reg, Subst, Supplied, Unclear, Damage, Gap, Restore, HandShift
//! and related attribute classes.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAgentIdent, AttCrit, AttExtent, AttHandIdent, AttMedium, AttRdgAnl, AttRdgGes, AttRdgLog,
    AttRdgVis, AttReasonIdent, AttTextRendition, AttTrans,
};
use tusk_model::elements::{
    Abbr, Add, AddChild, App, AppChild, Choice, ChoiceChild, Corr, Damage, Del, Expan, Gap,
    HandShift, Lem, Orig, Rdg, Reg, Restore, Sic, Space, Subst, Supplied, Unclear,
};

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

impl ExtractAttributes for AttHandIdent {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "hand", self.hand);
        Ok(())
    }
}

impl ExtractAttributes for AttReasonIdent {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "reason", string self.reason);
        Ok(())
    }
}

impl ExtractAttributes for AttAgentIdent {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "agent", string self.agent);
        Ok(())
    }
}

impl ExtractAttributes for AttMedium {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "medium", string self.medium);
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
    // place attribute
    extract_attr!(attrs, "place", vec add.place);

    // Read children if not an empty element
    // Add can contain mixed content: text and element children
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("add")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        add.children.push(AddChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        add.children.push(AddChild::Space(Box::new(space)));
                    }
                    _ => {
                        // Skip unknown children
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
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

// ============================================================================
// Abbr element implementation
// ============================================================================

impl MeiDeserialize for Abbr {
    fn element_name() -> &'static str {
        "abbr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut abbr = Abbr::default();

        // Extract attributes
        abbr.common.extract_attributes(&mut attrs)?;
        abbr.edit.extract_attributes(&mut attrs)?;
        abbr.facsimile.extract_attributes(&mut attrs)?;
        abbr.lang.extract_attributes(&mut attrs)?;
        abbr.trans.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "expan", string abbr.expan);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("abbr")?;
        }

        Ok(abbr)
    }
}

// ============================================================================
// Expan element implementation
// ============================================================================

impl MeiDeserialize for Expan {
    fn element_name() -> &'static str {
        "expan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut expan = Expan::default();

        // Extract attributes
        expan.common.extract_attributes(&mut attrs)?;
        expan.edit.extract_attributes(&mut attrs)?;
        expan.extent.extract_attributes(&mut attrs)?;
        expan.facsimile.extract_attributes(&mut attrs)?;
        expan.lang.extract_attributes(&mut attrs)?;
        expan.trans.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "abbr", string expan.abbr);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("expan")?;
        }

        Ok(expan)
    }
}

// ============================================================================
// Orig element implementation
// ============================================================================

impl MeiDeserialize for Orig {
    fn element_name() -> &'static str {
        "orig"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut orig = Orig::default();

        // Extract attributes
        orig.common.extract_attributes(&mut attrs)?;
        orig.edit.extract_attributes(&mut attrs)?;
        orig.extent.extract_attributes(&mut attrs)?;
        orig.facsimile.extract_attributes(&mut attrs)?;
        orig.lang.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("orig")?;
        }

        Ok(orig)
    }
}

// ============================================================================
// Reg element implementation
// ============================================================================

impl MeiDeserialize for Reg {
    fn element_name() -> &'static str {
        "reg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut reg = Reg::default();

        // Extract attributes
        reg.common.extract_attributes(&mut attrs)?;
        reg.authorized.extract_attributes(&mut attrs)?;
        reg.edit.extract_attributes(&mut attrs)?;
        reg.extent.extract_attributes(&mut attrs)?;
        reg.lang.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("reg")?;
        }

        Ok(reg)
    }
}

// ============================================================================
// Subst element implementation
// ============================================================================

impl MeiDeserialize for Subst {
    fn element_name() -> &'static str {
        "subst"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut subst = Subst::default();

        // Extract attributes
        subst.common.extract_attributes(&mut attrs)?;
        subst.edit.extract_attributes(&mut attrs)?;
        subst.trans.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("subst")?;
        }

        Ok(subst)
    }
}

// ============================================================================
// Supplied element implementation
// ============================================================================

impl MeiDeserialize for Supplied {
    fn element_name() -> &'static str {
        "supplied"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut supplied = Supplied::default();

        // Extract attributes
        supplied.common.extract_attributes(&mut attrs)?;
        supplied.agent_ident.extract_attributes(&mut attrs)?;
        supplied.edit.extract_attributes(&mut attrs)?;
        supplied.extent.extract_attributes(&mut attrs)?;
        supplied.facsimile.extract_attributes(&mut attrs)?;
        supplied.lang.extract_attributes(&mut attrs)?;
        supplied.reason_ident.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("supplied")?;
        }

        Ok(supplied)
    }
}

// ============================================================================
// Unclear element implementation
// ============================================================================

impl MeiDeserialize for Unclear {
    fn element_name() -> &'static str {
        "unclear"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut unclear = Unclear::default();

        // Extract attributes
        unclear.common.extract_attributes(&mut attrs)?;
        unclear.agent_ident.extract_attributes(&mut attrs)?;
        unclear.edit.extract_attributes(&mut attrs)?;
        unclear.extent.extract_attributes(&mut attrs)?;
        unclear.facsimile.extract_attributes(&mut attrs)?;
        unclear.hand_ident.extract_attributes(&mut attrs)?;
        unclear.lang.extract_attributes(&mut attrs)?;
        unclear.reason_ident.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("unclear")?;
        }

        Ok(unclear)
    }
}

// ============================================================================
// Damage element implementation
// ============================================================================

impl MeiDeserialize for Damage {
    fn element_name() -> &'static str {
        "damage"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut damage = Damage::default();

        // Extract attributes
        damage.common.extract_attributes(&mut attrs)?;
        damage.agent_ident.extract_attributes(&mut attrs)?;
        damage.extent.extract_attributes(&mut attrs)?;
        damage.facsimile.extract_attributes(&mut attrs)?;
        damage.hand_ident.extract_attributes(&mut attrs)?;
        damage.lang.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "degree", string damage.degree);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("damage")?;
        }

        Ok(damage)
    }
}

// ============================================================================
// Gap element implementation (empty element)
// ============================================================================

impl MeiDeserialize for Gap {
    fn element_name() -> &'static str {
        "gap"
    }

    fn from_mei_event<R: BufRead>(
        _reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        _is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut gap = Gap::default();

        // Extract attributes
        gap.common.extract_attributes(&mut attrs)?;
        gap.edit.extract_attributes(&mut attrs)?;
        gap.extent.extract_attributes(&mut attrs)?;
        gap.hand_ident.extract_attributes(&mut attrs)?;
        gap.reason_ident.extract_attributes(&mut attrs)?;

        // Gap is an empty element, no children to parse

        Ok(gap)
    }
}

// ============================================================================
// Restore element implementation
// ============================================================================

impl MeiDeserialize for Restore {
    fn element_name() -> &'static str {
        "restore"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut restore = Restore::default();

        // Extract attributes
        restore.common.extract_attributes(&mut attrs)?;
        restore.edit.extract_attributes(&mut attrs)?;
        restore.extent.extract_attributes(&mut attrs)?;
        restore.facsimile.extract_attributes(&mut attrs)?;
        restore.lang.extract_attributes(&mut attrs)?;
        restore.trans.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "desc", string restore.desc);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("restore")?;
        }

        Ok(restore)
    }
}

// ============================================================================
// HandShift element implementation (empty element)
// ============================================================================

impl MeiDeserialize for HandShift {
    fn element_name() -> &'static str {
        "handShift"
    }

    fn from_mei_event<R: BufRead>(
        _reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        _is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut hand_shift = HandShift::default();

        // Extract attributes
        hand_shift.common.extract_attributes(&mut attrs)?;
        hand_shift.edit.extract_attributes(&mut attrs)?;
        hand_shift.facsimile.extract_attributes(&mut attrs)?;
        hand_shift.medium.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "character", string hand_shift.character);
        extract_attr!(attrs, "new", hand_shift.new);
        extract_attr!(attrs, "old", hand_shift.old);

        // HandShift is an empty element, no children to parse

        Ok(hand_shift)
    }
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

    // ========================================================================
    // Abbr tests
    // ========================================================================

    #[test]
    fn abbr_deserializes_empty() {
        let xml = r#"<abbr/>"#;
        let abbr = Abbr::from_mei_str(xml).expect("should deserialize");

        assert!(abbr.common.xml_id.is_none());
    }

    #[test]
    fn abbr_deserializes_with_expan_attr() {
        let xml = r#"<abbr expan="Doctor"/>"#;
        let abbr = Abbr::from_mei_str(xml).expect("should deserialize");

        assert_eq!(abbr.expan, Some("Doctor".to_string()));
    }

    // ========================================================================
    // Expan tests
    // ========================================================================

    #[test]
    fn expan_deserializes_empty() {
        let xml = r#"<expan/>"#;
        let expan = Expan::from_mei_str(xml).expect("should deserialize");

        assert!(expan.common.xml_id.is_none());
    }

    #[test]
    fn expan_deserializes_with_abbr_attr() {
        let xml = r#"<expan abbr="Dr."/>"#;
        let expan = Expan::from_mei_str(xml).expect("should deserialize");

        assert_eq!(expan.abbr, Some("Dr.".to_string()));
    }

    // ========================================================================
    // Orig tests
    // ========================================================================

    #[test]
    fn orig_deserializes_empty() {
        let xml = r#"<orig/>"#;
        let orig = Orig::from_mei_str(xml).expect("should deserialize");

        assert!(orig.common.xml_id.is_none());
    }

    // ========================================================================
    // Reg tests
    // ========================================================================

    #[test]
    fn reg_deserializes_empty() {
        let xml = r#"<reg/>"#;
        let reg = Reg::from_mei_str(xml).expect("should deserialize");

        assert!(reg.common.xml_id.is_none());
    }

    // ========================================================================
    // Subst tests
    // ========================================================================

    #[test]
    fn subst_deserializes_empty() {
        let xml = r#"<subst/>"#;
        let subst = Subst::from_mei_str(xml).expect("should deserialize");

        assert!(subst.common.xml_id.is_none());
    }

    // ========================================================================
    // Supplied tests
    // ========================================================================

    #[test]
    fn supplied_deserializes_empty() {
        let xml = r#"<supplied/>"#;
        let supplied = Supplied::from_mei_str(xml).expect("should deserialize");

        assert!(supplied.common.xml_id.is_none());
    }

    #[test]
    fn supplied_deserializes_with_reason() {
        let xml = r#"<supplied reason="lost"/>"#;
        let supplied = Supplied::from_mei_str(xml).expect("should deserialize");

        assert_eq!(supplied.reason_ident.reason, Some("lost".to_string()));
    }

    // ========================================================================
    // Unclear tests
    // ========================================================================

    #[test]
    fn unclear_deserializes_empty() {
        let xml = r#"<unclear/>"#;
        let unclear = Unclear::from_mei_str(xml).expect("should deserialize");

        assert!(unclear.common.xml_id.is_none());
    }

    #[test]
    fn unclear_deserializes_with_reason() {
        let xml = r#"<unclear reason="faded"/>"#;
        let unclear = Unclear::from_mei_str(xml).expect("should deserialize");

        assert_eq!(unclear.reason_ident.reason, Some("faded".to_string()));
    }

    // ========================================================================
    // Damage tests
    // ========================================================================

    #[test]
    fn damage_deserializes_empty() {
        let xml = r#"<damage/>"#;
        let damage = Damage::from_mei_str(xml).expect("should deserialize");

        assert!(damage.common.xml_id.is_none());
    }

    #[test]
    fn damage_deserializes_with_degree() {
        let xml = r#"<damage degree="medium"/>"#;
        let damage = Damage::from_mei_str(xml).expect("should deserialize");

        assert_eq!(damage.degree, Some("medium".to_string()));
    }

    // ========================================================================
    // Gap tests
    // ========================================================================

    #[test]
    fn gap_deserializes_empty() {
        let xml = r#"<gap/>"#;
        let gap = Gap::from_mei_str(xml).expect("should deserialize");

        assert!(gap.common.xml_id.is_none());
    }

    #[test]
    fn gap_deserializes_with_reason() {
        let xml = r#"<gap reason="illegible"/>"#;
        let gap = Gap::from_mei_str(xml).expect("should deserialize");

        assert_eq!(gap.reason_ident.reason, Some("illegible".to_string()));
    }

    // ========================================================================
    // Restore tests
    // ========================================================================

    #[test]
    fn restore_deserializes_empty() {
        let xml = r#"<restore/>"#;
        let restore = Restore::from_mei_str(xml).expect("should deserialize");

        assert!(restore.common.xml_id.is_none());
    }

    #[test]
    fn restore_deserializes_with_desc() {
        let xml = r#"<restore desc="deleted and restored"/>"#;
        let restore = Restore::from_mei_str(xml).expect("should deserialize");

        assert_eq!(restore.desc, Some("deleted and restored".to_string()));
    }

    // ========================================================================
    // HandShift tests
    // ========================================================================

    #[test]
    fn hand_shift_deserializes_empty() {
        let xml = r#"<handShift/>"#;
        let hand_shift = HandShift::from_mei_str(xml).expect("should deserialize");

        assert!(hand_shift.common.xml_id.is_none());
    }

    #[test]
    fn hand_shift_deserializes_with_new() {
        let xml = r##"<handShift new="#h2"/>"##;
        let hand_shift = HandShift::from_mei_str(xml).expect("should deserialize");

        assert!(hand_shift.new.is_some());
    }
}

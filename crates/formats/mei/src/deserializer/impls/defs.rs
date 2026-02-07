//! Deserializer implementations for definition MEI elements.
//!
//! This module contains implementations for ScoreDef, StaffDef, LayerDef, StaffGrp,
//! PgHead, PgFoot, and Seg.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttInstrDefGes, AttLayerDefAnl, AttLayerDefGes, AttLayerDefLog, AttLayerDefVis, AttScoreDefAnl,
    AttScoreDefGes, AttScoreDefLog, AttScoreDefVis, AttStaffDefAnl, AttStaffDefGes, AttStaffDefLog,
    AttStaffDefVis, AttStaffGrpAnl, AttStaffGrpGes, AttStaffGrpLog, AttStaffGrpVis,
};
use tusk_model::elements::{
    Clef, InstrDef, LabelAbbrChild, LabelChild, LayerDef, LayerDefChild, PgFoot, PgFootChild,
    PgHead, PgHeadChild, ScoreDef, ScoreDefChild, Seg, StaffDef, StaffDefChild, StaffGrp,
    StaffGrpChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// ScoreDef attribute class implementations
// ============================================================================

// ============================================================================
// StaffDef attribute class implementations
// ============================================================================

// ============================================================================
// StaffGrp attribute class implementations
// ============================================================================

// ============================================================================
// LayerDef attribute class implementations
// ============================================================================

// ============================================================================
// InstrDef attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for ScoreDef {
    fn element_name() -> &'static str {
        "scoreDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut score_def = ScoreDef::default();

        // Extract attributes into each attribute class
        score_def.common.extract_attributes(&mut attrs)?;
        score_def.score_def_log.extract_attributes(&mut attrs)?;
        score_def.score_def_ges.extract_attributes(&mut attrs)?;
        score_def.score_def_vis.extract_attributes(&mut attrs)?;
        score_def.score_def_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // scoreDef can contain: staffGrp, keySig, meterSig, meterSigGrp, grpSym, ambitus,
        // pgFoot, pgHead, symbolTable, chordTable, instrGrp
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("scoreDef")?
            {
                match name.as_str() {
                    "staffGrp" => {
                        let staff_grp =
                            parse_staff_grp_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));
                    }
                    "keySig" => {
                        let key_sig = parse_key_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("keySig")?;
                        }
                        score_def
                            .children
                            .push(ScoreDefChild::KeySig(Box::new(key_sig)));
                    }
                    "meterSig" => {
                        let meter_sig = parse_meter_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("meterSig")?;
                        }
                        score_def
                            .children
                            .push(ScoreDefChild::MeterSig(Box::new(meter_sig)));
                    }
                    "meterSigGrp" => {
                        // MeterSigGrp - skip for now (complex element)
                        if !child_empty {
                            reader.skip_to_end("meterSigGrp")?;
                        }
                    }
                    "pgHead" => {
                        let pg_head = parse_pg_head_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::PgHead(Box::new(pg_head)));
                    }
                    "pgFoot" => {
                        let pg_foot = parse_pg_foot_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::PgFoot(Box::new(pg_foot)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        reader.skip_unknown_child(&name, "scoreDef", child_empty)?;
                    }
                }
            }
        }

        Ok(score_def)
    }
}

/// Helper to parse StaffGrp from event (recursive parsing)
fn parse_staff_grp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StaffGrp> {
    let mut staff_grp = StaffGrp::default();

    // Extract common attributes
    staff_grp.common.extract_attributes(&mut attrs)?;
    staff_grp.facsimile.extract_attributes(&mut attrs)?;
    staff_grp.metadata_pointing.extract_attributes(&mut attrs)?;

    // Extract domain-specific attributes
    staff_grp.staff_grp_log.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_ges.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_vis.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_anl.extract_attributes(&mut attrs)?;

    // Parse children if not empty
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("staffGrp")?
        {
            match name.as_str() {
                "staffDef" => {
                    let staff_def = parse_staff_def_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
                }
                "staffGrp" => {
                    // Nested staffGrp - recursive call
                    let nested_staff_grp =
                        parse_staff_grp_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::StaffGrp(Box::new(nested_staff_grp)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::Label(Box::new(label)));
                }
                "labelAbbr" => {
                    let label_abbr = parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::LabelAbbr(Box::new(label_abbr)));
                }
                "grpSym" => {
                    // GrpSym element - skip for now
                    if !child_empty {
                        reader.skip_to_end("grpSym")?;
                    }
                }
                "instrDef" => {
                    let instr_def = parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::InstrDef(Box::new(instr_def)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "staffGrp", child_empty)?;
                }
            }
        }
    }

    Ok(staff_grp)
}

impl MeiDeserialize for StaffGrp {
    fn element_name() -> &'static str {
        "staffGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_staff_grp_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for StaffDef {
    fn element_name() -> &'static str {
        "staffDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut staff_def = StaffDef::default();

        // Extract attributes into each attribute class
        staff_def.basic.extract_attributes(&mut attrs)?;
        staff_def.labelled.extract_attributes(&mut attrs)?;
        staff_def.linking.extract_attributes(&mut attrs)?;
        staff_def.metadata_pointing.extract_attributes(&mut attrs)?;
        staff_def.n_integer.extract_attributes(&mut attrs)?;
        staff_def.responsibility.extract_attributes(&mut attrs)?;
        staff_def.typed.extract_attributes(&mut attrs)?;
        staff_def.staff_def_log.extract_attributes(&mut attrs)?;
        staff_def.staff_def_ges.extract_attributes(&mut attrs)?;
        staff_def.staff_def_vis.extract_attributes(&mut attrs)?;
        staff_def.staff_def_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not an empty element
        // staffDef can contain: label, labelAbbr, clef, clefGrp, keySig, meterSig, meterSigGrp,
        // layerDef, instrDef, tuning, mensur, proport, ambitus
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("staffDef")?
            {
                match name.as_str() {
                    "clef" => {
                        let clef = parse_clef_from_event(reader, child_attrs, child_empty)?;
                        staff_def.children.push(StaffDefChild::Clef(Box::new(clef)));
                    }
                    "keySig" => {
                        let key_sig = parse_key_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("keySig")?;
                        }
                        staff_def
                            .children
                            .push(StaffDefChild::KeySig(Box::new(key_sig)));
                    }
                    "meterSig" => {
                        let meter_sig = parse_meter_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("meterSig")?;
                        }
                        staff_def
                            .children
                            .push(StaffDefChild::MeterSig(Box::new(meter_sig)));
                    }
                    "label" => {
                        let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::Label(Box::new(label)));
                    }
                    "labelAbbr" => {
                        let label_abbr =
                            parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::LabelAbbr(Box::new(label_abbr)));
                    }
                    "layerDef" => {
                        let layer_def =
                            parse_layer_def_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::LayerDef(Box::new(layer_def)));
                    }
                    "instrDef" => {
                        let instr_def =
                            parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::InstrDef(Box::new(instr_def)));
                    }
                    "clefGrp" | "meterSigGrp" | "tuning" | "mensur" | "proport" | "ambitus" => {
                        // These elements are supported but not fully parsed yet - skip for now
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "staffDef", child_empty)?;
                    }
                }
            }
        }

        Ok(staff_def)
    }
}

/// Helper to parse StaffDef from event (for use in staffGrp parsing)
fn parse_staff_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StaffDef> {
    StaffDef::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for LayerDef {
    fn element_name() -> &'static str {
        "layerDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_layer_def_from_event(reader, attrs, is_empty)
    }
}

/// Helper to parse Clef from event
pub(crate) fn parse_clef_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Clef> {
    let mut clef = Clef::default();

    // Extract common attributes
    clef.common.extract_attributes(&mut attrs)?;
    clef.facsimile.extract_attributes(&mut attrs)?;

    // Extract event attributes (when, layer, staff, tstamp, etc.)
    clef.event.extract_attributes(&mut attrs)?;

    // Clef-specific logical attributes
    extract_attr!(attrs, "shape", clef.clef_log.shape);
    extract_attr!(attrs, "line", clef.clef_log.line);
    extract_attr!(attrs, "oct", clef.clef_log.oct);
    extract_attr!(attrs, "dis", clef.clef_log.dis);
    extract_attr!(attrs, "dis.place", clef.clef_log.dis_place);
    extract_attr!(attrs, "cautionary", clef.clef_log.cautionary);

    // Extract clef visual attributes (color, visible, glyph.*, font*, etc.)
    clef.clef_vis.extract_attributes(&mut attrs)?;

    // Skip children if any (clef typically has no children)
    if !is_empty {
        reader.skip_to_end("clef")?;
    }

    Ok(clef)
}

/// Helper to parse Label from event
pub(crate) fn parse_label_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Label> {
    let mut label = tusk_model::elements::Label::default();

    // Extract all attribute classes
    label.common.extract_attributes(&mut attrs)?;
    label.facsimile.extract_attributes(&mut attrs)?;
    label.lang.extract_attributes(&mut attrs)?;
    label.source.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("label")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content
                    if !text.trim().is_empty() {
                        label.children.push(LabelChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "rend" => {
                        let rend =
                            super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        label.children.push(LabelChild::Rend(Box::new(rend)));
                    }
                    "ref" => {
                        let ref_elem =
                            super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
                        label.children.push(LabelChild::Ref(Box::new(ref_elem)));
                    }
                    "lb" => {
                        let lb =
                            super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        label.children.push(LabelChild::Lb(Box::new(lb)));
                    }
                    "persName" => {
                        let pers_name = super::header::parse_pers_name_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label
                            .children
                            .push(LabelChild::PersName(Box::new(pers_name)));
                    }
                    "corpName" => {
                        let corp_name = super::header::parse_corp_name_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label
                            .children
                            .push(LabelChild::CorpName(Box::new(corp_name)));
                    }
                    "name" => {
                        let name_elem =
                            super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
                        label.children.push(LabelChild::Name(Box::new(name_elem)));
                    }
                    "date" => {
                        let date =
                            super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
                        label.children.push(LabelChild::Date(Box::new(date)));
                    }
                    "title" => {
                        let title = super::header::parse_title_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label.children.push(LabelChild::Title(Box::new(title)));
                    }
                    "identifier" => {
                        let identifier = super::header::parse_identifier_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label
                            .children
                            .push(LabelChild::Identifier(Box::new(identifier)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "label", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(label)
}

impl MeiDeserialize for tusk_model::elements::Label {
    fn element_name() -> &'static str {
        "label"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_label_from_event(reader, attrs, is_empty)
    }
}

/// Helper to parse LabelAbbr from event
fn parse_label_abbr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::LabelAbbr> {
    let mut label_abbr = tusk_model::elements::LabelAbbr::default();

    // Extract all attribute classes
    label_abbr.common.extract_attributes(&mut attrs)?;
    label_abbr.facsimile.extract_attributes(&mut attrs)?;
    label_abbr.lang.extract_attributes(&mut attrs)?;
    label_abbr.source.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("labelAbbr")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content
                    if !text.trim().is_empty() {
                        label_abbr.children.push(LabelAbbrChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "rend" => {
                        let rend =
                            super::text::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::Rend(Box::new(rend)));
                    }
                    "ref" => {
                        let ref_elem =
                            super::header::parse_ref_from_event(reader, child_attrs, child_empty)?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::Ref(Box::new(ref_elem)));
                    }
                    "lb" => {
                        let lb =
                            super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        label_abbr.children.push(LabelAbbrChild::Lb(Box::new(lb)));
                    }
                    "persName" => {
                        let pers_name = super::header::parse_pers_name_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::PersName(Box::new(pers_name)));
                    }
                    "corpName" => {
                        let corp_name = super::header::parse_corp_name_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::CorpName(Box::new(corp_name)));
                    }
                    "name" => {
                        let name_elem =
                            super::header::parse_name_from_event(reader, child_attrs, child_empty)?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::Name(Box::new(name_elem)));
                    }
                    "date" => {
                        let date =
                            super::header::parse_date_from_event(reader, child_attrs, child_empty)?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::Date(Box::new(date)));
                    }
                    "title" => {
                        let title = super::header::parse_title_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::Title(Box::new(title)));
                    }
                    "identifier" => {
                        let identifier = super::header::parse_identifier_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        label_abbr
                            .children
                            .push(LabelAbbrChild::Identifier(Box::new(identifier)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "labelAbbr", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(label_abbr)
}

/// Helper to parse LayerDef from event
fn parse_layer_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LayerDef> {
    let mut layer_def = LayerDef::default();

    // Extract all attribute classes
    layer_def.basic.extract_attributes(&mut attrs)?;
    layer_def.labelled.extract_attributes(&mut attrs)?;
    layer_def.linking.extract_attributes(&mut attrs)?;
    layer_def.metadata_pointing.extract_attributes(&mut attrs)?;
    layer_def.n_integer.extract_attributes(&mut attrs)?;
    layer_def.responsibility.extract_attributes(&mut attrs)?;
    layer_def.typed.extract_attributes(&mut attrs)?;
    layer_def.layer_def_log.extract_attributes(&mut attrs)?;
    layer_def.layer_def_ges.extract_attributes(&mut attrs)?;
    layer_def.layer_def_vis.extract_attributes(&mut attrs)?;
    layer_def.layer_def_anl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // layerDef can contain: label, labelAbbr, instrDef, meterSig, meterSigGrp, ambitus
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("layerDef")?
        {
            match name.as_str() {
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::Label(Box::new(label)));
                }
                "labelAbbr" => {
                    let label_abbr = parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::LabelAbbr(Box::new(label_abbr)));
                }
                "instrDef" => {
                    let instr_def = parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::InstrDef(Box::new(instr_def)));
                }
                "meterSig" => {
                    let meter_sig = parse_meter_sig_from_raw(child_attrs);
                    if !child_empty {
                        reader.skip_to_end("meterSig")?;
                    }
                    layer_def
                        .children
                        .push(LayerDefChild::MeterSig(Box::new(meter_sig)));
                }
                "meterSigGrp" | "ambitus" => {
                    // These elements are supported but not fully parsed yet - skip for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                _ => {
                    reader.skip_unknown_child(&name, "layerDef", child_empty)?;
                }
            }
        }
    }

    Ok(layer_def)
}

/// Helper to parse InstrDef from event
fn parse_instr_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<InstrDef> {
    let mut instr_def = InstrDef::default();

    // Extract attributes
    instr_def.basic.extract_attributes(&mut attrs)?;
    instr_def.labelled.extract_attributes(&mut attrs)?;
    instr_def.n_integer.extract_attributes(&mut attrs)?;
    instr_def.instr_def_ges.extract_attributes(&mut attrs)?;

    // Skip children if any
    if !is_empty {
        reader.skip_to_end("instrDef")?;
    }

    Ok(instr_def)
}

/// Helper to parse KeySig from raw attributes
fn parse_key_sig_from_raw(mut attrs: AttributeMap) -> tusk_model::elements::KeySig {
    use tusk_model::elements::KeySig;

    let mut key_sig = KeySig::default();

    // Extract common attributes
    if let Some(id) = attrs.remove("xml:id") {
        key_sig.common.xml_id = Some(id);
    }

    // KeySig-specific attributes could be added here as needed

    key_sig
}

/// Helper to parse MeterSig from raw attributes
fn parse_meter_sig_from_raw(mut attrs: AttributeMap) -> tusk_model::elements::MeterSig {
    use tusk_model::elements::MeterSig;

    let mut meter_sig = MeterSig::default();

    // Extract common attributes
    if let Some(id) = attrs.remove("xml:id") {
        meter_sig.common.xml_id = Some(id);
    }

    // MeterSig-specific attributes could be added here as needed

    meter_sig
}

// ============================================================================
// PgHead and PgFoot implementations
// ============================================================================

/// Parse a `<pgHead>` element from within another element.
///
/// PgHead (page header) can contain mixed content with text and many child elements.
pub(crate) fn parse_pg_head_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PgHead> {
    let mut pg_head = PgHead::default();

    // Extract all attribute classes
    pg_head.common.extract_attributes(&mut attrs)?;
    pg_head.facsimile.extract_attributes(&mut attrs)?;
    pg_head.formework.extract_attributes(&mut attrs)?;
    pg_head.horizontal_align.extract_attributes(&mut attrs)?;
    pg_head.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("pgHead")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        pg_head.children.push(PgHeadChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            pg_head.children.push(PgHeadChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::Identifier(Box::new(identifier)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Ptr(Box::new(ptr)));
                        }
                        "lg" => {
                            let lg = tusk_model::elements::Lg::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Lg(Box::new(lg)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::P(Box::new(p)));
                        }
                        "list" => {
                            let list = super::text::parse_list_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::List(Box::new(list)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Seg(Box::new(seg)));
                        }
                        "table" => {
                            let table = super::text::parse_table_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Table(Box::new(table)));
                        }
                        "anchoredText" => {
                            let anchored_text = tusk_model::elements::AnchoredText::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::AnchoredText(Box::new(anchored_text)));
                        }
                        // Skip unknown child elements
                        _ => {
                            reader.skip_unknown_child(&name, "pgHead", child_empty)?;
                        }
                    }
                }
            }
        }
    }

    Ok(pg_head)
}

/// Parse a `<pgFoot>` element from within another element.
///
/// PgFoot (page footer) can contain mixed content with text and many child elements.
pub(crate) fn parse_pg_foot_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PgFoot> {
    let mut pg_foot = PgFoot::default();

    // Extract all attribute classes
    pg_foot.common.extract_attributes(&mut attrs)?;
    pg_foot.facsimile.extract_attributes(&mut attrs)?;
    pg_foot.formework.extract_attributes(&mut attrs)?;
    pg_foot.horizontal_align.extract_attributes(&mut attrs)?;
    pg_foot.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("pgFoot")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        pg_foot.children.push(PgFootChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            pg_foot.children.push(PgFootChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::Identifier(Box::new(identifier)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Ptr(Box::new(ptr)));
                        }
                        "lg" => {
                            let lg = tusk_model::elements::Lg::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Lg(Box::new(lg)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::P(Box::new(p)));
                        }
                        "list" => {
                            let list = super::text::parse_list_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::List(Box::new(list)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Seg(Box::new(seg)));
                        }
                        "anchoredText" => {
                            let anchored_text = tusk_model::elements::AnchoredText::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::AnchoredText(Box::new(anchored_text)));
                        }
                        "table" => {
                            let table = super::text::parse_table_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Table(Box::new(table)));
                        }
                        // Skip unknown child elements
                        _ => {
                            reader.skip_unknown_child(&name, "pgFoot", child_empty)?;
                        }
                    }
                }
            }
        }
    }

    Ok(pg_foot)
}

// ============================================================================
// InstrDef implementation
// ============================================================================

impl MeiDeserialize for InstrDef {
    fn element_name() -> &'static str {
        "instrDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_instr_def_from_event(reader, attrs, is_empty)
    }
}

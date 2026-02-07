//! Deserializer implementations for core CMN (Common Music Notation) elements.
//!
//! This module contains implementations for structural and utility elements:
//! - BarLine: Explicit barline element
//! - Stem: Explicit stem element
//! - ClefGrp: Grouped clefs
//! - Custos: End-of-line pitch indicator
//! - Pad: Horizontal spacing element
//! - KeyAccid: Accidental in a key signature
//! - MeterSigGrp: Non-standard meter signatures
//! - GrpSym: Brace/bracket for staff grouping
//! - Part/Parts: Part elements for performer views
//! - Ossia: Alternative notation passages
//! - HarpPedal: Harp pedal diagram

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBarLineAnl, AttBarLineGes, AttBarLineLog, AttBarLineVis, AttClefGrpAnl, AttClefGrpGes,
    AttClefGrpLog, AttClefGrpVis, AttCustosAnl, AttCustosGes, AttCustosLog, AttCustosVis,
    AttGrpSymAnl, AttGrpSymGes, AttGrpSymLog, AttGrpSymVis, AttHarpPedalAnl, AttHarpPedalGes,
    AttHarpPedalLog, AttHarpPedalVis, AttKeyAccidAnl, AttKeyAccidGes, AttKeyAccidLog,
    AttKeyAccidVis, AttMeterSigGrpAnl, AttMeterSigGrpGes, AttMeterSigGrpLog, AttMeterSigGrpVis,
    AttOssiaAnl, AttOssiaGes, AttOssiaLog, AttOssiaVis, AttPadAnl, AttPadGes, AttPadLog, AttPadVis,
    AttPartAnl, AttPartGes, AttPartLog, AttPartVis, AttPartsAnl, AttPartsGes, AttPartsLog,
    AttPartsVis, AttStemAnl, AttStemGes, AttStemLog, AttStemVis,
};
use tusk_model::elements::{
    Accid, BarLine, ClefGrp, ClefGrpChild, Custos, CustosChild, GrpSym, GrpSymChild, HarpPedal,
    KeyAccid, LabelAbbr, LabelAbbrChild, Layer, MeterSig, MeterSigGrp, MeterSigGrpChild, Ossia,
    OssiaChild, Pad, Part, PartChild, Parts, PartsChild, Staff, Stem,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// BarLine attribute class implementations
// ============================================================================

// ============================================================================
// Stem attribute class implementations
// ============================================================================

// ============================================================================
// ClefGrp attribute class implementations
// ============================================================================

// ============================================================================
// Custos attribute class implementations
// ============================================================================

// ============================================================================
// Pad attribute class implementations
// ============================================================================

// ============================================================================
// KeyAccid attribute class implementations
// ============================================================================

// ============================================================================
// MeterSigGrp attribute class implementations
// ============================================================================

// ============================================================================
// GrpSym attribute class implementations
// ============================================================================

// ============================================================================
// Part/Parts attribute class implementations
// ============================================================================

// ============================================================================
// Ossia attribute class implementations
// ============================================================================

// ============================================================================
// HarpPedal attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for BarLine {
    fn element_name() -> &'static str {
        "barLine"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut bar_line = BarLine::default();

        // Extract attributes
        bar_line.common.extract_attributes(&mut attrs)?;
        bar_line.facsimile.extract_attributes(&mut attrs)?;
        bar_line.pointing.extract_attributes(&mut attrs)?;
        bar_line.target_eval.extract_attributes(&mut attrs)?;
        bar_line.bar_line_log.extract_attributes(&mut attrs)?;
        bar_line.bar_line_vis.extract_attributes(&mut attrs)?;
        bar_line.bar_line_ges.extract_attributes(&mut attrs)?;
        bar_line.bar_line_anl.extract_attributes(&mut attrs)?;

        // BarLine has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("barLine")?;
        }

        Ok(bar_line)
    }
}

impl MeiDeserialize for Stem {
    fn element_name() -> &'static str {
        "stem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut stem = Stem::default();

        // Extract attributes
        stem.common.extract_attributes(&mut attrs)?;
        stem.facsimile.extract_attributes(&mut attrs)?;
        stem.stem_log.extract_attributes(&mut attrs)?;
        stem.stem_vis.extract_attributes(&mut attrs)?;
        stem.stem_ges.extract_attributes(&mut attrs)?;
        stem.stem_anl.extract_attributes(&mut attrs)?;

        // Stem has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("stem")?;
        }

        Ok(stem)
    }
}

impl MeiDeserialize for ClefGrp {
    fn element_name() -> &'static str {
        "clefGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut clef_grp = ClefGrp::default();

        // Extract attributes
        clef_grp.common.extract_attributes(&mut attrs)?;
        clef_grp.event.extract_attributes(&mut attrs)?;
        clef_grp.facsimile.extract_attributes(&mut attrs)?;
        clef_grp.clef_grp_log.extract_attributes(&mut attrs)?;
        clef_grp.clef_grp_vis.extract_attributes(&mut attrs)?;
        clef_grp.clef_grp_ges.extract_attributes(&mut attrs)?;
        clef_grp.clef_grp_anl.extract_attributes(&mut attrs)?;

        // Parse children (only clef elements)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("clefGrp")?
            {
                match name.as_str() {
                    "clef" => {
                        let clef = super::parse_clef_from_event(reader, child_attrs, child_empty)?;
                        clef_grp.children.push(ClefGrpChild::Clef(Box::new(clef)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(clef_grp)
    }
}

impl MeiDeserialize for Custos {
    fn element_name() -> &'static str {
        "custos"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut custos = Custos::default();

        // Extract attributes
        custos.common.extract_attributes(&mut attrs)?;
        custos.facsimile.extract_attributes(&mut attrs)?;
        custos.source.extract_attributes(&mut attrs)?;
        custos.custos_log.extract_attributes(&mut attrs)?;
        custos.custos_vis.extract_attributes(&mut attrs)?;
        custos.custos_ges.extract_attributes(&mut attrs)?;
        custos.custos_anl.extract_attributes(&mut attrs)?;

        // Parse children (only accid elements)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("custos")?
            {
                match name.as_str() {
                    "accid" => {
                        let accid = Accid::from_mei_event(reader, child_attrs, child_empty)?;
                        custos.children.push(CustosChild::Accid(Box::new(accid)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(custos)
    }
}

impl MeiDeserialize for Pad {
    fn element_name() -> &'static str {
        "pad"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut pad = Pad::default();

        // Extract attributes
        pad.common.extract_attributes(&mut attrs)?;
        pad.pad_log.extract_attributes(&mut attrs)?;
        pad.pad_vis.extract_attributes(&mut attrs)?;
        pad.pad_ges.extract_attributes(&mut attrs)?;
        pad.pad_anl.extract_attributes(&mut attrs)?;

        // Pad has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("pad")?;
        }

        Ok(pad)
    }
}

impl MeiDeserialize for KeyAccid {
    fn element_name() -> &'static str {
        "keyAccid"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut key_accid = KeyAccid::default();

        // Extract attributes
        key_accid.common.extract_attributes(&mut attrs)?;
        key_accid.facsimile.extract_attributes(&mut attrs)?;
        key_accid.key_accid_log.extract_attributes(&mut attrs)?;
        key_accid.key_accid_vis.extract_attributes(&mut attrs)?;
        key_accid.key_accid_ges.extract_attributes(&mut attrs)?;
        key_accid.key_accid_anl.extract_attributes(&mut attrs)?;

        // Direct attribute
        extract_attr!(attrs, "form", string key_accid.form);

        // KeyAccid has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("keyAccid")?;
        }

        Ok(key_accid)
    }
}

impl MeiDeserialize for MeterSigGrp {
    fn element_name() -> &'static str {
        "meterSigGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut meter_sig_grp = MeterSigGrp::default();

        // Extract attributes
        meter_sig_grp.common.extract_attributes(&mut attrs)?;
        meter_sig_grp.facsimile.extract_attributes(&mut attrs)?;
        meter_sig_grp
            .meter_sig_grp_log
            .extract_attributes(&mut attrs)?;
        meter_sig_grp
            .meter_sig_grp_vis
            .extract_attributes(&mut attrs)?;
        meter_sig_grp
            .meter_sig_grp_ges
            .extract_attributes(&mut attrs)?;
        meter_sig_grp
            .meter_sig_grp_anl
            .extract_attributes(&mut attrs)?;

        // Parse children (meterSig, meterSigGrp)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("meterSigGrp")?
            {
                match name.as_str() {
                    "meterSig" => {
                        let meter_sig =
                            parse_meter_sig_from_event(reader, child_attrs, child_empty)?;
                        meter_sig_grp
                            .children
                            .push(MeterSigGrpChild::MeterSig(Box::new(meter_sig)));
                    }
                    "meterSigGrp" => {
                        // Recursive for nested meterSigGrp
                        let nested = MeterSigGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        meter_sig_grp
                            .children
                            .push(MeterSigGrpChild::MeterSigGrp(Box::new(nested)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(meter_sig_grp)
    }
}

/// Helper to parse MeterSig from event
fn parse_meter_sig_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<MeterSig> {
    let mut meter_sig = MeterSig::default();

    // Extract attributes
    meter_sig.common.extract_attributes(&mut attrs)?;
    meter_sig.facsimile.extract_attributes(&mut attrs)?;

    // MeterSig has no children, skip if not empty
    if !is_empty {
        reader.skip_to_end("meterSig")?;
    }

    Ok(meter_sig)
}

/// Helper to parse LabelAbbr from event
fn parse_label_abbr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LabelAbbr> {
    use crate::deserializer::MixedContent;

    let mut label_abbr = LabelAbbr::default();

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
                    if !text.trim().is_empty() {
                        label_abbr.children.push(LabelAbbrChild::Text(text));
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
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            label_abbr.children.push(LabelAbbrChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(label_abbr)
}

impl MeiDeserialize for GrpSym {
    fn element_name() -> &'static str {
        "grpSym"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut grp_sym = GrpSym::default();

        // Extract attributes
        grp_sym.common.extract_attributes(&mut attrs)?;
        grp_sym.facsimile.extract_attributes(&mut attrs)?;
        grp_sym.grp_sym_log.extract_attributes(&mut attrs)?;
        grp_sym.grp_sym_vis.extract_attributes(&mut attrs)?;
        grp_sym.grp_sym_ges.extract_attributes(&mut attrs)?;
        grp_sym.grp_sym_anl.extract_attributes(&mut attrs)?;

        // Parse children (label, labelAbbr)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("grpSym")?
            {
                match name.as_str() {
                    "label" => {
                        let label =
                            super::parse_label_from_event(reader, child_attrs, child_empty)?;
                        grp_sym.children.push(GrpSymChild::Label(Box::new(label)));
                    }
                    "labelAbbr" => {
                        let label_abbr =
                            parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                        grp_sym
                            .children
                            .push(GrpSymChild::LabelAbbr(Box::new(label_abbr)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(grp_sym)
    }
}

impl MeiDeserialize for Part {
    fn element_name() -> &'static str {
        "part"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut part = Part::default();

        // Extract attributes
        part.common.extract_attributes(&mut attrs)?;
        part.metadata_pointing.extract_attributes(&mut attrs)?;
        part.part_log.extract_attributes(&mut attrs)?;
        part.part_vis.extract_attributes(&mut attrs)?;
        part.part_ges.extract_attributes(&mut attrs)?;
        part.part_anl.extract_attributes(&mut attrs)?;

        // Part has many possible children - handle the common ones
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("part")?
            {
                match name.as_str() {
                    "section" => {
                        let section = tusk_model::elements::Section::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::Section(Box::new(section)));
                    }
                    "scoreDef" => {
                        let score_def = tusk_model::elements::ScoreDef::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::ScoreDef(Box::new(score_def)));
                    }
                    "staffDef" => {
                        let staff_def = tusk_model::elements::StaffDef::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::StaffDef(Box::new(staff_def)));
                    }
                    "ending" => {
                        let ending = tusk_model::elements::Ending::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::Ending(Box::new(ending)));
                    }
                    "pb" => {
                        let pb = tusk_model::elements::Pb::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::Pb(Box::new(pb)));
                    }
                    "sb" => {
                        let sb = tusk_model::elements::Sb::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::Sb(Box::new(sb)));
                    }
                    "annot" => {
                        let annot = tusk_model::elements::Annot::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children.push(PartChild::Annot(Box::new(annot)));
                    }
                    "anchoredText" => {
                        let anchored_text = tusk_model::elements::AnchoredText::from_mei_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        part.children
                            .push(PartChild::AnchoredText(Box::new(anchored_text)));
                    }
                    _ => {
                        // Skip unknown children
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(part)
    }
}

impl MeiDeserialize for Parts {
    fn element_name() -> &'static str {
        "parts"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut parts = Parts::default();

        // Extract attributes
        parts.common.extract_attributes(&mut attrs)?;
        parts.metadata_pointing.extract_attributes(&mut attrs)?;
        parts.parts_log.extract_attributes(&mut attrs)?;
        parts.parts_vis.extract_attributes(&mut attrs)?;
        parts.parts_ges.extract_attributes(&mut attrs)?;
        parts.parts_anl.extract_attributes(&mut attrs)?;

        // Parse children (only part elements)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("parts")?
            {
                match name.as_str() {
                    "part" => {
                        let part = Part::from_mei_event(reader, child_attrs, child_empty)?;
                        parts.children.push(PartsChild::Part(Box::new(part)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(parts)
    }
}

impl MeiDeserialize for Ossia {
    fn element_name() -> &'static str {
        "ossia"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut ossia = Ossia::default();

        // Extract attributes
        ossia.common.extract_attributes(&mut attrs)?;
        ossia.facsimile.extract_attributes(&mut attrs)?;
        ossia.ossia_log.extract_attributes(&mut attrs)?;
        ossia.ossia_vis.extract_attributes(&mut attrs)?;
        ossia.ossia_ges.extract_attributes(&mut attrs)?;
        ossia.ossia_anl.extract_attributes(&mut attrs)?;

        // Parse children (layer, staff, oLayer, oStaff)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("ossia")?
            {
                match name.as_str() {
                    "layer" => {
                        let layer = Layer::from_mei_event(reader, child_attrs, child_empty)?;
                        ossia.children.push(OssiaChild::Layer(Box::new(layer)));
                    }
                    "staff" => {
                        let staff = Staff::from_mei_event(reader, child_attrs, child_empty)?;
                        ossia.children.push(OssiaChild::Staff(Box::new(staff)));
                    }
                    // oLayer and oStaff are currently not implemented
                    // They would need their own deserializers when added
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(ossia)
    }
}

impl MeiDeserialize for HarpPedal {
    fn element_name() -> &'static str {
        "harpPedal"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut harp_pedal = HarpPedal::default();

        // Extract attributes
        harp_pedal.common.extract_attributes(&mut attrs)?;
        harp_pedal.facsimile.extract_attributes(&mut attrs)?;
        harp_pedal.harp_pedal_log.extract_attributes(&mut attrs)?;
        harp_pedal.harp_pedal_vis.extract_attributes(&mut attrs)?;
        harp_pedal.harp_pedal_ges.extract_attributes(&mut attrs)?;
        harp_pedal.harp_pedal_anl.extract_attributes(&mut attrs)?;

        // HarpPedal has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("harpPedal")?;
        }

        Ok(harp_pedal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_line_deserializes_basic() {
        let xml = r#"<barLine form="dbl"/>"#;
        let bar_line = BarLine::from_mei_str(xml).expect("should deserialize");
        assert!(bar_line.bar_line_log.form.is_some());
    }

    #[test]
    fn bar_line_deserializes_with_id() {
        let xml = r#"<barLine xml:id="bl1" form="end"/>"#;
        let bar_line = BarLine::from_mei_str(xml).expect("should deserialize");
        assert_eq!(bar_line.common.xml_id, Some("bl1".to_string()));
    }

    #[test]
    fn stem_deserializes_basic() {
        let xml = r#"<stem dir="up"/>"#;
        let stem = Stem::from_mei_str(xml).expect("should deserialize");
        assert!(stem.stem_vis.dir.is_some());
    }

    #[test]
    fn clef_grp_deserializes_with_clefs() {
        let xml = r#"<clefGrp>
            <clef shape="G" line="2"/>
            <clef shape="F" line="4"/>
        </clefGrp>"#;
        let clef_grp = ClefGrp::from_mei_str(xml).expect("should deserialize");
        assert_eq!(clef_grp.children.len(), 2);
    }

    #[test]
    fn custos_deserializes_basic() {
        let xml = r#"<custos pname="c" oct="4"/>"#;
        let custos = Custos::from_mei_str(xml).expect("should deserialize");
        assert!(custos.custos_log.pname.is_some());
        assert!(custos.custos_log.oct.is_some());
    }

    #[test]
    fn pad_deserializes_basic() {
        let xml = r#"<pad width="2vu"/>"#;
        let pad = Pad::from_mei_str(xml).expect("should deserialize");
        assert!(pad.pad_log.width.is_some());
    }

    #[test]
    fn key_accid_deserializes_basic() {
        let xml = r#"<keyAccid accid="s" pname="f"/>"#;
        let key_accid = KeyAccid::from_mei_str(xml).expect("should deserialize");
        assert!(key_accid.key_accid_log.accid.is_some());
        assert!(key_accid.key_accid_log.pname.is_some());
    }

    #[test]
    fn meter_sig_grp_deserializes_with_children() {
        let xml = r#"<meterSigGrp func="alternating">
            <meterSig count="3" unit="4"/>
            <meterSig count="2" unit="4"/>
        </meterSigGrp>"#;
        let meter_sig_grp = MeterSigGrp::from_mei_str(xml).expect("should deserialize");
        assert!(meter_sig_grp.meter_sig_grp_log.func.is_some());
        assert_eq!(meter_sig_grp.children.len(), 2);
    }

    #[test]
    fn grp_sym_deserializes_basic() {
        let xml = r#"<grpSym symbol="brace"/>"#;
        let grp_sym = GrpSym::from_mei_str(xml).expect("should deserialize");
        assert!(grp_sym.grp_sym_log.symbol.is_some());
    }

    #[test]
    fn parts_deserializes_with_part() {
        let xml = r#"<parts>
            <part xml:id="p1"/>
        </parts>"#;
        let parts = Parts::from_mei_str(xml).expect("should deserialize");
        assert_eq!(parts.children.len(), 1);
    }

    #[test]
    fn ossia_deserializes_basic() {
        let xml = r#"<ossia xml:id="oss1"/>"#;
        let ossia = Ossia::from_mei_str(xml).expect("should deserialize");
        assert_eq!(ossia.common.xml_id, Some("oss1".to_string()));
    }

    #[test]
    fn harp_pedal_deserializes_basic() {
        let xml = r#"<harpPedal c="f" d="n" e="s" f="f" g="n" a="n" b="f"/>"#;
        let harp_pedal = HarpPedal::from_mei_str(xml).expect("should deserialize");
        assert!(harp_pedal.harp_pedal_log.c.is_some());
        assert!(harp_pedal.harp_pedal_log.d.is_some());
    }
}

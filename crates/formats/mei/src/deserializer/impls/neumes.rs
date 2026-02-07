//! Deserializer implementations for neume notation MEI elements.
//!
//! This module contains implementations for Syllable, Neume, Nc, NcGrp,
//! and neume component modifiers (Oriscus, Quilisma, Liquescent, Strophicus, Plica),
//! neume modifiers (Episema, HispanTick), and related elements (Ornam, AmbNote).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAmbNoteAnl, AttAmbNoteGes, AttAmbNoteLog, AttAmbNoteVis, AttEpisemaAnl, AttEpisemaGes,
    AttEpisemaLog, AttEpisemaVis, AttHispanTickAnl, AttHispanTickGes, AttHispanTickLog,
    AttHispanTickVis, AttLiquescentAnl, AttLiquescentGes, AttLiquescentLog, AttLiquescentVis,
    AttNcAnl, AttNcGes, AttNcGrpAnl, AttNcGrpGes, AttNcGrpLog, AttNcGrpVis, AttNcLog, AttNcVis,
    AttNeumeAnl, AttNeumeGes, AttNeumeLog, AttNeumeVis, AttOriscusAnl, AttOriscusGes,
    AttOriscusLog, AttOriscusVis, AttPlicaAnl, AttPlicaGes, AttPlicaLog, AttPlicaVis,
    AttQuilismaAnl, AttQuilismaGes, AttQuilismaLog, AttQuilismaVis, AttStrophicusAnl,
    AttStrophicusGes, AttStrophicusLog, AttStrophicusVis, AttSyllableAnl, AttSyllableGes,
    AttSyllableLog, AttSyllableVis,
};
use tusk_model::elements::{
    AmbNote, Episema, HispanTick, Liquescent, Nc, NcChild, NcGrp, NcGrpChild, Neume, NeumeChild,
    Oriscus, Plica, Quilisma, Strophicus, Syllable, SyllableChild,
};

use super::extract_attr;

// ============================================================================
// Syllable attribute class implementations
// ============================================================================

// ============================================================================
// Neume attribute class implementations
// ============================================================================

// ============================================================================
// Nc attribute class implementations
// ============================================================================

// ============================================================================
// NcGrp attribute class implementations
// ============================================================================

// ============================================================================
// Oriscus attribute class implementations
// ============================================================================

// ============================================================================
// Quilisma attribute class implementations
// ============================================================================

// ============================================================================
// Liquescent attribute class implementations
// ============================================================================

// ============================================================================
// Strophicus attribute class implementations
// ============================================================================

// ============================================================================
// Plica attribute class implementations
// ============================================================================

// ============================================================================
// Episema attribute class implementations
// ============================================================================

// ============================================================================
// HispanTick attribute class implementations
// ============================================================================

// ============================================================================
// AmbNote attribute class implementations
// ============================================================================

// ============================================================================
// Simple neume element implementations (no children)
// ============================================================================

impl MeiDeserialize for Oriscus {
    fn element_name() -> &'static str {
        "oriscus"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Oriscus::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.oriscus_log.extract_attributes(&mut attrs)?;
        elem.oriscus_vis.extract_attributes(&mut attrs)?;
        elem.oriscus_ges.extract_attributes(&mut attrs)?;
        elem.oriscus_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("oriscus")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for Quilisma {
    fn element_name() -> &'static str {
        "quilisma"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Quilisma::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.quilisma_log.extract_attributes(&mut attrs)?;
        elem.quilisma_vis.extract_attributes(&mut attrs)?;
        elem.quilisma_ges.extract_attributes(&mut attrs)?;
        elem.quilisma_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("quilisma")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for Liquescent {
    fn element_name() -> &'static str {
        "liquescent"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Liquescent::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.liquescent_log.extract_attributes(&mut attrs)?;
        elem.liquescent_vis.extract_attributes(&mut attrs)?;
        elem.liquescent_ges.extract_attributes(&mut attrs)?;
        elem.liquescent_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("liquescent")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for Strophicus {
    fn element_name() -> &'static str {
        "strophicus"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Strophicus::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.strophicus_log.extract_attributes(&mut attrs)?;
        elem.strophicus_vis.extract_attributes(&mut attrs)?;
        elem.strophicus_ges.extract_attributes(&mut attrs)?;
        elem.strophicus_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("strophicus")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for Plica {
    fn element_name() -> &'static str {
        "plica"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Plica::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.plica_log.extract_attributes(&mut attrs)?;
        elem.plica_vis.extract_attributes(&mut attrs)?;
        elem.plica_ges.extract_attributes(&mut attrs)?;
        elem.plica_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("plica")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for Episema {
    fn element_name() -> &'static str {
        "episema"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Episema::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.episema_log.extract_attributes(&mut attrs)?;
        elem.episema_vis.extract_attributes(&mut attrs)?;
        elem.episema_ges.extract_attributes(&mut attrs)?;
        elem.episema_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("episema")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for HispanTick {
    fn element_name() -> &'static str {
        "hispanTick"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = HispanTick::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.hispan_tick_log.extract_attributes(&mut attrs)?;
        elem.hispan_tick_vis.extract_attributes(&mut attrs)?;
        elem.hispan_tick_ges.extract_attributes(&mut attrs)?;
        elem.hispan_tick_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("hispanTick")?;
        }

        Ok(elem)
    }
}

impl MeiDeserialize for AmbNote {
    fn element_name() -> &'static str {
        "ambNote"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = AmbNote::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.amb_note_log.extract_attributes(&mut attrs)?;
        elem.amb_note_vis.extract_attributes(&mut attrs)?;
        elem.amb_note_ges.extract_attributes(&mut attrs)?;
        elem.amb_note_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("ambNote")?;
        }

        Ok(elem)
    }
}

// ============================================================================
// Complex neume element implementations (with children)
// ============================================================================

impl MeiDeserialize for Nc {
    fn element_name() -> &'static str {
        "nc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Nc::default();

        elem.basic.extract_attributes(&mut attrs)?;
        elem.classed.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.labelled.extract_attributes(&mut attrs)?;
        elem.linking.extract_attributes(&mut attrs)?;
        elem.n_number_like.extract_attributes(&mut attrs)?;
        elem.responsibility.extract_attributes(&mut attrs)?;
        elem.nc_log.extract_attributes(&mut attrs)?;
        elem.nc_vis.extract_attributes(&mut attrs)?;
        elem.nc_ges.extract_attributes(&mut attrs)?;
        elem.nc_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("nc")? {
                parse_nc_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_nc_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Nc,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "quilisma" => {
            let child = Quilisma::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcChild::Quilisma(Box::new(child)));
        }
        "liquescent" => {
            let child = Liquescent::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcChild::Liquescent(Box::new(child)));
        }
        "strophicus" => {
            let child = Strophicus::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcChild::Strophicus(Box::new(child)));
        }
        "oriscus" => {
            let child = Oriscus::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcChild::Oriscus(Box::new(child)));
        }
        "episema" => {
            let child = Episema::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcChild::Episema(Box::new(child)));
        }
        "hispanTick" => {
            let child = HispanTick::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcChild::HispanTick(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

impl MeiDeserialize for NcGrp {
    fn element_name() -> &'static str {
        "ncGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = NcGrp::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.nc_grp_log.extract_attributes(&mut attrs)?;
        elem.nc_grp_vis.extract_attributes(&mut attrs)?;
        elem.nc_grp_ges.extract_attributes(&mut attrs)?;
        elem.nc_grp_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("ncGrp")?
            {
                parse_nc_grp_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_nc_grp_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut NcGrp,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "nc" => {
            let child = Nc::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcGrpChild::Nc(Box::new(child)));
        }
        "ncGrp" => {
            let child = NcGrp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcGrpChild::NcGrp(Box::new(child)));
        }
        "episema" => {
            let child = Episema::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcGrpChild::Episema(Box::new(child)));
        }
        "hispanTick" => {
            let child = HispanTick::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NcGrpChild::HispanTick(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

impl MeiDeserialize for Neume {
    fn element_name() -> &'static str {
        "neume"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Neume::default();

        elem.basic.extract_attributes(&mut attrs)?;
        elem.classed.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.labelled.extract_attributes(&mut attrs)?;
        elem.linking.extract_attributes(&mut attrs)?;
        elem.n_number_like.extract_attributes(&mut attrs)?;
        elem.responsibility.extract_attributes(&mut attrs)?;
        elem.neume_log.extract_attributes(&mut attrs)?;
        elem.neume_vis.extract_attributes(&mut attrs)?;
        elem.neume_ges.extract_attributes(&mut attrs)?;
        elem.neume_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("neume")?
            {
                parse_neume_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_neume_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Neume,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "nc" => {
            let child = Nc::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NeumeChild::Nc(Box::new(child)));
        }
        "ncGrp" => {
            let child = NcGrp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NeumeChild::NcGrp(Box::new(child)));
        }
        "episema" => {
            let child = Episema::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NeumeChild::Episema(Box::new(child)));
        }
        "hispanTick" => {
            let child = HispanTick::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(NeumeChild::HispanTick(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

impl MeiDeserialize for Syllable {
    fn element_name() -> &'static str {
        "syllable"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Syllable::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.syllable_log.extract_attributes(&mut attrs)?;
        elem.syllable_vis.extract_attributes(&mut attrs)?;
        elem.syllable_ges.extract_attributes(&mut attrs)?;
        elem.syllable_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("syllable")?
            {
                parse_syllable_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_syllable_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Syllable,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "neume" => {
            let child = Neume::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(SyllableChild::Neume(Box::new(child)));
        }
        "episema" => {
            let child = Episema::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(SyllableChild::Episema(Box::new(child)));
        }
        "hispanTick" => {
            let child = HispanTick::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children
                .push(SyllableChild::HispanTick(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{
        AmbNote, Episema, HispanTick, Liquescent, Nc, NcGrp, Neume, Oriscus, Plica, Quilisma,
        Strophicus, Syllable,
    };

    #[test]
    fn oriscus_deserializes_from_empty_element() {
        let xml = r#"<oriscus/>"#;
        let elem = Oriscus::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn oriscus_deserializes_with_xml_id() {
        let xml = r#"<oriscus xml:id="or1"/>"#;
        let elem = Oriscus::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("or1".to_string()));
    }

    #[test]
    fn quilisma_deserializes_from_empty_element() {
        let xml = r#"<quilisma/>"#;
        let elem = Quilisma::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn liquescent_deserializes_from_empty_element() {
        let xml = r#"<liquescent/>"#;
        let elem = Liquescent::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn strophicus_deserializes_from_empty_element() {
        let xml = r#"<strophicus/>"#;
        let elem = Strophicus::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn plica_deserializes_from_empty_element() {
        let xml = r#"<plica/>"#;
        let elem = Plica::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn episema_deserializes_from_empty_element() {
        let xml = r#"<episema/>"#;
        let elem = Episema::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn episema_deserializes_with_staff() {
        let xml = r#"<episema xml:id="ep1" staff="1"/>"#;
        let elem = Episema::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("ep1".to_string()));
        assert_eq!(elem.episema_log.staff, vec![1]);
    }

    #[test]
    fn hispan_tick_deserializes_from_empty_element() {
        let xml = r#"<hispanTick/>"#;
        let elem = HispanTick::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn amb_note_deserializes_from_empty_element() {
        let xml = r#"<ambNote/>"#;
        let elem = AmbNote::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn nc_deserializes_from_empty_element() {
        let xml = r#"<nc/>"#;
        let elem = Nc::from_mei_str(xml).expect("should deserialize");
        assert!(elem.basic.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn nc_deserializes_with_pitch() {
        let xml = r#"<nc xml:id="nc1" pname="c" oct="4"/>"#;
        let elem = Nc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.basic.xml_id, Some("nc1".to_string()));
        assert_eq!(elem.nc_log.pname, Some("c".to_string()));
        assert_eq!(elem.nc_log.oct, Some("4".to_string()));
    }

    #[test]
    fn nc_deserializes_with_quilisma_child() {
        let xml = r#"<nc xml:id="nc1"><quilisma xml:id="q1"/></nc>"#;
        let elem = Nc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.basic.xml_id, Some("nc1".to_string()));
        assert_eq!(elem.children.len(), 1);
    }

    #[test]
    fn nc_grp_deserializes_from_empty_element() {
        let xml = r#"<ncGrp/>"#;
        let elem = NcGrp::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn nc_grp_deserializes_with_nc_child() {
        let xml = r#"<ncGrp xml:id="ncg1"><nc xml:id="nc1"/></ncGrp>"#;
        let elem = NcGrp::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("ncg1".to_string()));
        assert_eq!(elem.children.len(), 1);
    }

    #[test]
    fn neume_deserializes_from_empty_element() {
        let xml = r#"<neume/>"#;
        let elem = Neume::from_mei_str(xml).expect("should deserialize");
        assert!(elem.basic.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn neume_deserializes_with_nc_children() {
        let xml = r#"<neume xml:id="n1">
            <nc xml:id="nc1" pname="c" oct="4"/>
            <nc xml:id="nc2" pname="d" oct="4"/>
        </neume>"#;
        let elem = Neume::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.basic.xml_id, Some("n1".to_string()));
        assert_eq!(elem.children.len(), 2);
    }

    #[test]
    fn syllable_deserializes_from_empty_element() {
        let xml = r#"<syllable/>"#;
        let elem = Syllable::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn syllable_deserializes_with_neume_child() {
        let xml = r#"<syllable xml:id="syl1">
            <neume xml:id="n1">
                <nc xml:id="nc1"/>
            </neume>
        </syllable>"#;
        let elem = Syllable::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("syl1".to_string()));
        assert_eq!(elem.children.len(), 1);
    }
}

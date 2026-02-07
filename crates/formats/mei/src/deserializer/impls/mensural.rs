//! Deserializer implementations for mensural notation MEI elements.
//!
//! This module contains implementations for Mensur, Mensuration, Proport, and Ligature
//! elements used in early music notation.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttLigatureAnl, AttLigatureGes, AttLigatureLog, AttLigatureVis, AttMensurAnl, AttMensurGes,
    AttProportAnl, AttProportGes, AttProportLog, AttProportVis,
};
use tusk_model::elements::{
    Ligature, LigatureChild, Mensur, Mensuration, MensurationChild, Proport,
};

use super::extract_attr;

// ============================================================================
// Mensur attribute class implementations (Log/Vis already in mod.rs)
// ============================================================================

// ============================================================================
// Ligature attribute class implementations
// ============================================================================

// ============================================================================
// Proport attribute class implementations
// ============================================================================

// ============================================================================
// Mensur element implementation
// ============================================================================

impl MeiDeserialize for Mensur {
    fn element_name() -> &'static str {
        "mensur"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Mensur::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.mensur_log.extract_attributes(&mut attrs)?;
        elem.mensur_vis.extract_attributes(&mut attrs)?;
        elem.mensur_ges.extract_attributes(&mut attrs)?;
        elem.mensur_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("mensur")?;
        }

        Ok(elem)
    }
}

// ============================================================================
// Mensuration element implementation
// ============================================================================

impl MeiDeserialize for Mensuration {
    fn element_name() -> &'static str {
        "mensuration"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Mensuration::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.bibl.extract_attributes(&mut attrs)?;
        elem.lang.extract_attributes(&mut attrs)?;
        elem.mensur_log.extract_attributes(&mut attrs)?;
        elem.mensur_vis.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Mensuration can contain text content - read it all as text
            if let Some(text) = reader
                .read_text_until_end("mensuration")?
                .filter(|t| !t.is_empty())
            {
                elem.children.push(MensurationChild::Text(text));
            }
        }

        Ok(elem)
    }
}

// ============================================================================
// Proport element implementation
// ============================================================================

impl MeiDeserialize for Proport {
    fn element_name() -> &'static str {
        "proport"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Proport::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.proport_log.extract_attributes(&mut attrs)?;
        elem.proport_vis.extract_attributes(&mut attrs)?;
        elem.proport_ges.extract_attributes(&mut attrs)?;
        elem.proport_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("proport")?;
        }

        Ok(elem)
    }
}

// ============================================================================
// Ligature element implementation
// ============================================================================

impl MeiDeserialize for Ligature {
    fn element_name() -> &'static str {
        "ligature"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Ligature::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.ligature_log.extract_attributes(&mut attrs)?;
        elem.ligature_vis.extract_attributes(&mut attrs)?;
        elem.ligature_ges.extract_attributes(&mut attrs)?;
        elem.ligature_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("ligature")?
            {
                parse_ligature_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_ligature_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Ligature,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    use tusk_model::elements::{
        Add, App, BarLine, Choice, ClefGrp, Corr, Custos, Damage, Del, DivLine, Dot, Gap,
        HandShift, MeterSigGrp, Neume, Note, Orig, Pad, Reg, Rest, Restore, Sic, Space, Subst,
        Supplied, TabDurSym, TabGrp, Unclear,
    };

    match name {
        "note" => {
            let child = Note::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Note(Box::new(child)));
        }
        "rest" => {
            let child = Rest::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Rest(Box::new(child)));
        }
        "dot" => {
            let child = Dot::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Dot(Box::new(child)));
        }
        "space" => {
            let child = Space::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Space(Box::new(child)));
        }
        "chord" => {
            let child =
                tusk_model::elements::Chord::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Chord(Box::new(child)));
        }
        "clefGrp" => {
            let child = ClefGrp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::ClefGrp(Box::new(child)));
        }
        "meterSigGrp" => {
            let child = MeterSigGrp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children
                .push(LigatureChild::MeterSigGrp(Box::new(child)));
        }
        "mensur" => {
            let child = Mensur::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Mensur(Box::new(child)));
        }
        "proport" => {
            let child = Proport::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Proport(Box::new(child)));
        }
        "ligature" => {
            let child = Ligature::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Ligature(Box::new(child)));
        }
        "neume" => {
            let child = Neume::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Neume(Box::new(child)));
        }
        "barLine" => {
            let child = BarLine::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::BarLine(Box::new(child)));
        }
        "custos" => {
            let child = Custos::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Custos(Box::new(child)));
        }
        "pad" => {
            let child = Pad::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Pad(Box::new(child)));
        }
        "divLine" => {
            let child = DivLine::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::DivLine(Box::new(child)));
        }
        "tabGrp" => {
            let child = TabGrp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::TabGrp(Box::new(child)));
        }
        "tabDurSym" => {
            let child = TabDurSym::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children
                .push(LigatureChild::TabDurSym(Box::new(child)));
        }
        // Editorial elements
        "add" => {
            let child = Add::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Add(Box::new(child)));
        }
        "app" => {
            let child = App::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::App(Box::new(child)));
        }
        "choice" => {
            let child = Choice::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Choice(Box::new(child)));
        }
        "corr" => {
            let child = Corr::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Corr(Box::new(child)));
        }
        "damage" => {
            let child = Damage::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Damage(Box::new(child)));
        }
        "del" => {
            let child = Del::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Del(Box::new(child)));
        }
        "gap" => {
            let child = Gap::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Gap(Box::new(child)));
        }
        "handShift" => {
            let child = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children
                .push(LigatureChild::HandShift(Box::new(child)));
        }
        "orig" => {
            let child = Orig::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Orig(Box::new(child)));
        }
        "reg" => {
            let child = Reg::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Reg(Box::new(child)));
        }
        "restore" => {
            let child = Restore::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Restore(Box::new(child)));
        }
        "sic" => {
            let child = Sic::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Sic(Box::new(child)));
        }
        "subst" => {
            let child = Subst::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Subst(Box::new(child)));
        }
        "supplied" => {
            let child = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Supplied(Box::new(child)));
        }
        "unclear" => {
            let child = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(LigatureChild::Unclear(Box::new(child)));
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
    use tusk_model::elements::{Ligature, Mensur, Mensuration, Proport};

    #[test]
    fn mensur_deserializes_from_empty_element() {
        let xml = r#"<mensur/>"#;
        let elem = Mensur::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn mensur_deserializes_with_xml_id() {
        let xml = r#"<mensur xml:id="m1"/>"#;
        let elem = Mensur::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mensur_deserializes_with_attributes() {
        let xml = r#"<mensur xml:id="m1" num="3" numbase="2" tempus="3" prolatio="2"/>"#;
        let elem = Mensur::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("m1".to_string()));
        assert_eq!(elem.mensur_log.num, Some(3));
        assert_eq!(elem.mensur_log.numbase, Some(2));
    }

    #[test]
    fn mensuration_deserializes_from_empty_element() {
        let xml = r#"<mensuration/>"#;
        let elem = Mensuration::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn mensuration_deserializes_with_xml_id() {
        let xml = r#"<mensuration xml:id="mns1"/>"#;
        let elem = Mensuration::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("mns1".to_string()));
    }

    #[test]
    fn proport_deserializes_from_empty_element() {
        let xml = r#"<proport/>"#;
        let elem = Proport::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn proport_deserializes_with_xml_id() {
        let xml = r#"<proport xml:id="p1"/>"#;
        let elem = Proport::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("p1".to_string()));
    }

    #[test]
    fn proport_deserializes_with_attributes() {
        let xml = r#"<proport xml:id="p1" num="3" numbase="2"/>"#;
        let elem = Proport::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("p1".to_string()));
        assert_eq!(elem.proport_log.num, Some(3));
        assert_eq!(elem.proport_log.numbase, Some(2));
    }

    #[test]
    fn ligature_deserializes_from_empty_element() {
        let xml = r#"<ligature/>"#;
        let elem = Ligature::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn ligature_deserializes_with_xml_id() {
        let xml = r#"<ligature xml:id="lig1"/>"#;
        let elem = Ligature::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("lig1".to_string()));
    }

    #[test]
    fn ligature_deserializes_with_note_children() {
        let xml = r#"<ligature xml:id="lig1">
            <note xml:id="n1" pname="c" oct="4"/>
            <note xml:id="n2" pname="d" oct="4"/>
        </ligature>"#;
        let elem = Ligature::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("lig1".to_string()));
        assert_eq!(elem.children.len(), 2);
    }
}

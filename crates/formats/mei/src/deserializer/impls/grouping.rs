//! Deserializer implementations for grouping MEI elements.
//!
//! This module contains implementations for Beam, Tuplet, and GraceGrp elements.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBeamAnl, AttBeamGes, AttBeamLog, AttBeamVis, AttGraceGrpAnl, AttGraceGrpGes, AttGraceGrpLog,
    AttGraceGrpVis, AttTupletAnl, AttTupletGes, AttTupletLog, AttTupletVis,
};
use tusk_model::elements::{
    App, Beam, BeamChild, Chord, GraceGrp, GraceGrpChild, Note, Rest, Space, Tuplet, TupletChild,
};

use super::{extract_attr, from_attr_string, parse_clef_from_event};

// ============================================================================
// Beam attribute class implementations
// ============================================================================

impl ExtractAttributes for AttBeamLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "beam.with", self.beam_with);
        Ok(())
    }
}

impl ExtractAttributes for AttBeamVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "slash", self.slash);
        extract_attr!(attrs, "slope", self.slope);
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttBeamGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeamGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttBeamAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeamAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Tuplet attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTupletLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam.with", self.beam_with);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "num.place", self.num_place);
        extract_attr!(attrs, "num.visible", self.num_visible);
        extract_attr!(attrs, "bracket.place", self.bracket_place);
        extract_attr!(attrs, "bracket.visible", self.bracket_visible);
        extract_attr!(attrs, "num.format", self.num_format);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTupletAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// GraceGrp attribute class implementations
// ============================================================================

impl ExtractAttributes for AttGraceGrpLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "grace", self.grace);
        extract_attr!(attrs, "grace.time", self.grace_time);
        extract_attr!(attrs, "attach", self.attach);
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttGraceGrpGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttGraceGrpAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// MeiDeserialize implementations
// ============================================================================

impl MeiDeserialize for Beam {
    fn element_name() -> &'static str {
        "beam"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut beam = Beam::default();

        // Extract attributes into each attribute class
        beam.common.extract_attributes(&mut attrs)?;
        beam.facsimile.extract_attributes(&mut attrs)?;
        beam.beam_log.extract_attributes(&mut attrs)?;
        beam.beam_vis.extract_attributes(&mut attrs)?;
        beam.beam_ges.extract_attributes(&mut attrs)?;
        beam.beam_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("beam")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        // Nested beams are allowed
                        let nested_beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Beam(Box::new(nested_beam)));
                    }
                    "tuplet" => {
                        let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Tuplet(Box::new(tuplet)));
                    }
                    "graceGrp" => {
                        let grace_grp = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::GraceGrp(Box::new(grace_grp)));
                    }
                    "clef" => {
                        let clef = parse_clef_from_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Clef(Box::new(clef)));
                    }
                    "app" => {
                        let app = App::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::App(Box::new(app)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(beam)
    }
}

impl MeiDeserialize for Tuplet {
    fn element_name() -> &'static str {
        "tuplet"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tuplet = Tuplet::default();

        // Extract attributes into each attribute class
        tuplet.common.extract_attributes(&mut attrs)?;
        tuplet.facsimile.extract_attributes(&mut attrs)?;
        tuplet.tuplet_log.extract_attributes(&mut attrs)?;
        tuplet.tuplet_vis.extract_attributes(&mut attrs)?;
        tuplet.tuplet_ges.extract_attributes(&mut attrs)?;
        tuplet.tuplet_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("tuplet")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Beam(Box::new(beam)));
                    }
                    "tuplet" => {
                        // Nested tuplets are allowed
                        let nested_tuplet =
                            Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet
                            .children
                            .push(TupletChild::Tuplet(Box::new(nested_tuplet)));
                    }
                    "graceGrp" => {
                        let grace_grp = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet
                            .children
                            .push(TupletChild::GraceGrp(Box::new(grace_grp)));
                    }
                    "bTrem" => {
                        use tusk_model::elements::BTrem;
                        let b_trem = BTrem::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::BTrem(Box::new(b_trem)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(tuplet)
    }
}

impl MeiDeserialize for GraceGrp {
    fn element_name() -> &'static str {
        "graceGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut grace_grp = GraceGrp::default();

        // Extract attributes into each attribute class
        grace_grp.common.extract_attributes(&mut attrs)?;
        grace_grp.facsimile.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_log.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_vis.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_ges.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("graceGrp")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp.children.push(GraceGrpChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp.children.push(GraceGrpChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp.children.push(GraceGrpChild::Beam(Box::new(beam)));
                    }
                    "tuplet" => {
                        let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::Tuplet(Box::new(tuplet)));
                    }
                    "graceGrp" => {
                        // Nested graceGrp is allowed
                        let nested = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::GraceGrp(Box::new(nested)));
                    }
                    // Other child types (clef, barLine, etc.) can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(grace_grp)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{Beam, BeamChild, Layer, LayerChild, Tuplet, TupletChild};

    // ============================================================================
    // Beam tests
    // ============================================================================

    #[test]
    fn beam_deserializes_from_empty_element() {
        let xml = r#"<beam/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert!(beam.common.xml_id.is_none());
        assert!(beam.children.is_empty());
    }

    #[test]
    fn beam_deserializes_xml_id() {
        let xml = r#"<beam xml:id="b1"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
    }

    #[test]
    fn beam_deserializes_with_note_children() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <note xml:id="n2" pname="d" oct="4" dur="8"/>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 2);

        // Check first child is a note
        match &beam.children[0] {
            BeamChild::Note(note) => {
                assert_eq!(note.common.xml_id, Some("n1".to_string()));
            }
            _ => panic!("Expected note child"),
        }

        // Check second child is a note
        match &beam.children[1] {
            BeamChild::Note(note) => {
                assert_eq!(note.common.xml_id, Some("n2".to_string()));
            }
            _ => panic!("Expected note child"),
        }
    }

    #[test]
    fn beam_deserializes_with_mixed_children() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <rest xml:id="r1" dur="8"/>
            <chord xml:id="ch1" dur="8">
                <note pname="e" oct="4"/>
                <note pname="g" oct="4"/>
            </chord>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.children.len(), 3);

        match &beam.children[0] {
            BeamChild::Note(_) => {}
            _ => panic!("Expected note"),
        }
        match &beam.children[1] {
            BeamChild::Rest(_) => {}
            _ => panic!("Expected rest"),
        }
        match &beam.children[2] {
            BeamChild::Chord(_) => {}
            _ => panic!("Expected chord"),
        }
    }

    #[test]
    fn beam_deserializes_nested_beams() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" dur="16"/>
            <beam xml:id="b2">
                <note xml:id="n2" dur="32"/>
                <note xml:id="n3" dur="32"/>
            </beam>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 2);

        match &beam.children[1] {
            BeamChild::Beam(nested) => {
                assert_eq!(nested.common.xml_id, Some("b2".to_string()));
                assert_eq!(nested.children.len(), 2);
            }
            _ => panic!("Expected nested beam"),
        }
    }

    #[test]
    fn beam_deserializes_staff_attribute() {
        let xml = r#"<beam staff="1"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_log.staff, vec![1]);
    }

    #[test]
    fn beam_deserializes_layer_attribute() {
        let xml = r#"<beam layer="1"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_log.layer, vec![1]);
    }

    #[test]
    fn beam_deserializes_beam_with_attribute() {
        use tusk_model::data::DataNeighboringlayer;

        let xml = r#"<beam beam.with="above"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_log.beam_with, Some(DataNeighboringlayer::Above));
    }

    #[test]
    fn beam_deserializes_form_attribute() {
        use tusk_model::att::AttBeamVisForm;

        let xml = r#"<beam form="acc"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Acc));

        let xml = r#"<beam form="rit"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Rit));

        let xml = r#"<beam form="mixed"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Mixed));

        let xml = r#"<beam form="norm"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Norm));
    }

    #[test]
    fn beam_deserializes_place_attribute() {
        use tusk_model::data::DataBeamplace;

        let xml = r#"<beam place="above"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Above));

        let xml = r#"<beam place="below"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Below));

        let xml = r#"<beam place="mixed"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Mixed));
    }

    #[test]
    fn beam_deserializes_slash_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<beam slash="true"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.slash, Some(DataBoolean::True));
    }

    #[test]
    fn beam_deserializes_slope_attribute() {
        let xml = r#"<beam slope="15.5"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.slope, Some(15.5));
    }

    #[test]
    fn beam_deserializes_color_attribute() {
        let xml = r#"<beam color="red"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert!(beam.beam_vis.color.is_some());
    }

    #[test]
    fn beam_deserializes_cue_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<beam cue="true"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.cue, Some(DataBoolean::True));
    }

    #[test]
    fn beam_deserializes_visible_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<beam visible="false"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.visible, Some(DataBoolean::False));
    }

    #[test]
    fn beam_handles_unknown_attributes_leniently() {
        let xml = r#"<beam xml:id="b1" unknown="value"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
    }

    #[test]
    fn beam_handles_unknown_children_leniently() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" dur="8"/>
            <unknownElement>ignored</unknownElement>
            <note xml:id="n2" dur="8"/>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 2); // unknown element was skipped
    }

    #[test]
    fn beam_deserializes_all_common_attributes() {
        use tusk_model::att::AttBeamVisForm;
        use tusk_model::data::{DataBeamplace, DataBoolean, DataNeighboringlayer};

        let xml = r##"<beam xml:id="b1" staff="1 2" layer="1" beam.with="above" form="acc" place="above" slash="true" slope="10.0" color="blue" cue="true" visible="true"/>"##;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.beam_log.staff, vec![1, 2]);
        assert_eq!(beam.beam_log.layer, vec![1]);
        assert_eq!(beam.beam_log.beam_with, Some(DataNeighboringlayer::Above));
        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Acc));
        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Above));
        assert_eq!(beam.beam_vis.slash, Some(DataBoolean::True));
        assert_eq!(beam.beam_vis.slope, Some(10.0));
        assert!(beam.beam_vis.color.is_some());
        assert_eq!(beam.beam_vis.cue, Some(DataBoolean::True));
        assert_eq!(beam.beam_vis.visible, Some(DataBoolean::True));
    }

    #[test]
    fn beam_inside_layer_deserializes() {
        let xml = r#"<layer xml:id="l1">
            <beam xml:id="b1">
                <note xml:id="n1" dur="8"/>
                <note xml:id="n2" dur="8"/>
            </beam>
        </layer>"#;
        let layer = Layer::from_mei_str(xml).expect("should deserialize");

        assert_eq!(layer.children.len(), 1);

        match &layer.children[0] {
            LayerChild::Beam(beam) => {
                assert_eq!(beam.common.xml_id, Some("b1".to_string()));
                assert_eq!(beam.children.len(), 2);
            }
            _ => panic!("Expected beam child"),
        }
    }

    // ===== Tuplet element tests =====

    #[test]
    fn tuplet_deserializes_from_empty_element() {
        let xml = r#"<tuplet/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert!(tuplet.common.xml_id.is_none());
        assert!(tuplet.children.is_empty());
    }

    #[test]
    fn tuplet_deserializes_xml_id() {
        let xml = r#"<tuplet xml:id="t1"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tuplet_deserializes_num_and_numbase() {
        let xml = r#"<tuplet num="3" numbase="2"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_log.num, Some(3));
        assert_eq!(tuplet.tuplet_log.numbase, Some(2));
    }

    #[test]
    fn tuplet_deserializes_with_note_children() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <note xml:id="n2" pname="d" oct="4" dur="8"/>
            <note xml:id="n3" pname="e" oct="4" dur="8"/>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.tuplet_log.num, Some(3));
        assert_eq!(tuplet.tuplet_log.numbase, Some(2));
        assert_eq!(tuplet.children.len(), 3);

        // Check all children are notes
        for (i, child) in tuplet.children.iter().enumerate() {
            match child {
                TupletChild::Note(note) => {
                    assert_eq!(note.common.xml_id, Some(format!("n{}", i + 1)));
                }
                _ => panic!("Expected note child at position {}", i),
            }
        }
    }

    #[test]
    fn tuplet_deserializes_with_mixed_children() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <rest xml:id="r1" dur="8"/>
            <chord xml:id="ch1" dur="8">
                <note pname="e" oct="4"/>
                <note pname="g" oct="4"/>
            </chord>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.children.len(), 3);

        match &tuplet.children[0] {
            TupletChild::Note(_) => {}
            _ => panic!("Expected note"),
        }
        match &tuplet.children[1] {
            TupletChild::Rest(_) => {}
            _ => panic!("Expected rest"),
        }
        match &tuplet.children[2] {
            TupletChild::Chord(_) => {}
            _ => panic!("Expected chord"),
        }
    }

    #[test]
    fn tuplet_deserializes_nested_tuplets() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" dur="8"/>
            <tuplet xml:id="t2" num="5" numbase="4">
                <note xml:id="n2" dur="16"/>
                <note xml:id="n3" dur="16"/>
                <note xml:id="n4" dur="16"/>
                <note xml:id="n5" dur="16"/>
                <note xml:id="n6" dur="16"/>
            </tuplet>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.children.len(), 2);

        match &tuplet.children[1] {
            TupletChild::Tuplet(nested) => {
                assert_eq!(nested.common.xml_id, Some("t2".to_string()));
                assert_eq!(nested.tuplet_log.num, Some(5));
                assert_eq!(nested.tuplet_log.numbase, Some(4));
                assert_eq!(nested.children.len(), 5);
            }
            _ => panic!("Expected nested tuplet"),
        }
    }

    #[test]
    fn tuplet_deserializes_staff_attribute() {
        let xml = r#"<tuplet staff="1"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_log.staff, vec![1]);
    }

    #[test]
    fn tuplet_deserializes_layer_attribute() {
        let xml = r#"<tuplet layer="1"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_log.layer, vec![1]);
    }

    #[test]
    fn tuplet_deserializes_dur_attribute() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        let xml = r#"<tuplet dur="8"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_log.dur,
            vec![DataDuration::DataDurationCmn(DataDurationCmn::N8)]
        );
    }

    #[test]
    fn tuplet_deserializes_bracket_visible_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<tuplet bracket.visible="true"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.bracket_visible, Some(DataBoolean::True));

        let xml = r#"<tuplet bracket.visible="false"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.bracket_visible, Some(DataBoolean::False));
    }

    #[test]
    fn tuplet_deserializes_bracket_place_attribute() {
        use tusk_model::data::DataStaffrelBasic;

        let xml = r#"<tuplet bracket.place="above"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.bracket_place,
            Some(DataStaffrelBasic::Above)
        );

        let xml = r#"<tuplet bracket.place="below"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.bracket_place,
            Some(DataStaffrelBasic::Below)
        );
    }

    #[test]
    fn tuplet_deserializes_num_place_attribute() {
        use tusk_model::data::DataStaffrelBasic;

        let xml = r#"<tuplet num.place="above"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.num_place, Some(DataStaffrelBasic::Above));
    }

    #[test]
    fn tuplet_deserializes_num_visible_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<tuplet num.visible="true"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.num_visible, Some(DataBoolean::True));
    }

    #[test]
    fn tuplet_deserializes_num_format_attribute() {
        use tusk_model::att::AttTupletVisNumFormat;

        let xml = r#"<tuplet num.format="count"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.num_format,
            Some(AttTupletVisNumFormat::Count)
        );

        let xml = r#"<tuplet num.format="ratio"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.num_format,
            Some(AttTupletVisNumFormat::Ratio)
        );
    }

    #[test]
    fn tuplet_deserializes_color_attribute() {
        let xml = r#"<tuplet color="red"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert!(tuplet.tuplet_vis.color.is_some());
    }

    #[test]
    fn tuplet_handles_unknown_attributes_leniently() {
        let xml = r#"<tuplet xml:id="t1" unknown="value"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tuplet_handles_unknown_children_leniently() {
        let xml = r#"<tuplet xml:id="t1">
            <unknown>content</unknown>
            <note xml:id="n1" dur="8"/>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        // Unknown element should be skipped, only note remains
        assert_eq!(tuplet.children.len(), 1);
    }

    #[test]
    fn tuplet_deserializes_all_common_attributes() {
        let xml = r#"<tuplet
            xml:id="t1"
            label="triplet"
            n="1"
            num="3"
            numbase="2"
            staff="1"
            layer="1"
            bracket.visible="true"
            bracket.place="above"
            num.visible="true"
            num.place="above"
            num.format="ratio"
        />"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.common.label, Some("triplet".to_string()));
    }

    #[test]
    fn tuplet_inside_layer_deserializes() {
        let xml = r#"<layer xml:id="l1">
            <tuplet xml:id="t1" num="3" numbase="2">
                <note xml:id="n1" dur="8"/>
                <note xml:id="n2" dur="8"/>
                <note xml:id="n3" dur="8"/>
            </tuplet>
        </layer>"#;
        let layer = Layer::from_mei_str(xml).expect("should deserialize");

        assert_eq!(layer.children.len(), 1);

        match &layer.children[0] {
            LayerChild::Tuplet(tuplet) => {
                assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
                assert_eq!(tuplet.children.len(), 3);
            }
            _ => panic!("Expected tuplet child"),
        }
    }

    #[test]
    fn tuplet_with_beam_child_deserializes() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <beam xml:id="b1">
                <note xml:id="n1" dur="16"/>
                <note xml:id="n2" dur="16"/>
                <note xml:id="n3" dur="16"/>
            </beam>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.children.len(), 1);

        match &tuplet.children[0] {
            TupletChild::Beam(beam) => {
                assert_eq!(beam.common.xml_id, Some("b1".to_string()));
                assert_eq!(beam.children.len(), 3);
            }
            _ => panic!("Expected beam child"),
        }
    }

    #[test]
    fn tuplet_with_space_child_deserializes() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" dur="8"/>
            <space xml:id="s1" dur="8"/>
            <note xml:id="n2" dur="8"/>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.children.len(), 3);

        match &tuplet.children[1] {
            TupletChild::Space(space) => {
                assert_eq!(space.common.xml_id, Some("s1".to_string()));
            }
            _ => panic!("Expected space child"),
        }
    }

    // ===== Beam with clef child tests =====

    #[test]
    fn beam_deserializes_with_clef_child() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" dur="8"/>
            <clef xml:id="c1" shape="G" line="2"/>
            <note xml:id="n2" dur="8"/>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 3);

        match &beam.children[0] {
            BeamChild::Note(note) => {
                assert_eq!(note.common.xml_id, Some("n1".to_string()));
            }
            _ => panic!("Expected note child"),
        }

        match &beam.children[1] {
            BeamChild::Clef(clef) => {
                assert_eq!(clef.common.xml_id, Some("c1".to_string()));
            }
            _ => panic!("Expected clef child"),
        }

        match &beam.children[2] {
            BeamChild::Note(note) => {
                assert_eq!(note.common.xml_id, Some("n2".to_string()));
            }
            _ => panic!("Expected note child"),
        }
    }

    #[test]
    fn beam_deserializes_clef_with_attributes() {
        use tusk_model::data::{DataClefline, DataClefshape};

        let xml = r#"<beam xml:id="b1">
            <clef xml:id="c1" shape="F" line="4"/>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.children.len(), 1);

        match &beam.children[0] {
            BeamChild::Clef(clef) => {
                assert_eq!(clef.common.xml_id, Some("c1".to_string()));
                assert_eq!(clef.clef_log.shape, Some(DataClefshape::F));
                assert_eq!(clef.clef_log.line, Some(DataClefline(4)));
            }
            _ => panic!("Expected clef child"),
        }
    }
}

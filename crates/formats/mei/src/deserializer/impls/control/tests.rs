//! Tests for control element deserialization.

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::Fermata;

    // ============================================================================
    // Slur deserialization tests
    // ============================================================================

    #[test]
    fn slur_deserializes_from_empty_element() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.common.xml_id.is_none());
        assert!(slur.slur_log.startid.is_none());
        assert!(slur.slur_log.endid.is_none());
        assert!(slur.children.is_empty());
    }

    #[test]
    fn slur_deserializes_xml_id() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur xml:id="s1"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn slur_deserializes_startid_endid() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur startid="#n1" endid="#n2"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.startid.is_some());
        assert!(slur.slur_log.endid.is_some());
    }

    #[test]
    fn slur_deserializes_staff_layer() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur staff="1" layer="1"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.slur_log.staff, vec![1]);
        assert_eq!(slur.slur_log.layer, vec![1]);
    }

    #[test]
    fn slur_deserializes_tstamp_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur tstamp="1" tstamp2="0m+2"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.tstamp.is_some());
        assert!(slur.slur_log.tstamp2.is_some());
    }

    #[test]
    fn slur_deserializes_visual_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur curvedir="above" lform="solid"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_vis.curvedir.is_some());
        assert!(slur.slur_vis.lform.is_some());
    }

    #[test]
    fn slur_deserializes_gestural_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur dur.ges="4" dur.ppq="480"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_ges.dur_ges.is_some());
        assert_eq!(slur.slur_ges.dur_ppq, Some(480));
    }

    #[test]
    fn slur_deserializes_analytical_attributes() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur join="#s2"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(!slur.slur_anl.join.is_empty());
    }

    #[test]
    fn slur_deserializes_full_attributes() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur xml:id="s1" startid="#n1" endid="#n2" staff="1" layer="1" curvedir="below"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
        assert!(slur.slur_log.startid.is_some());
        assert!(slur.slur_log.endid.is_some());
        assert_eq!(slur.slur_log.staff, vec![1]);
        assert!(slur.slur_vis.curvedir.is_some());
    }

    #[test]
    fn slur_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur xml:id="s1" unknown="value"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn slur_deserializes_evaluate_attribute() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur evaluate="all"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.evaluate.is_some());
    }

    #[test]
    fn slur_deserializes_coordinate_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur x="100" y="200" x2="300" y2="250"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.slur_vis.x, Some(100.0));
        assert_eq!(slur.slur_vis.y, Some(200.0));
        assert_eq!(slur.slur_vis.x2, Some(300.0));
        assert_eq!(slur.slur_vis.y2, Some(250.0));
    }

    #[test]
    fn slur_deserializes_offset_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_vis.startho.is_some());
        assert!(slur.slur_vis.endho.is_some());
        assert!(slur.slur_vis.startvo.is_some());
        assert!(slur.slur_vis.endvo.is_some());
    }

    // ============================================================================
    // Tie deserialization tests
    // ============================================================================

    #[test]
    fn tie_deserializes_from_empty_element() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.common.xml_id.is_none());
        assert!(tie.tie_log.startid.is_none());
        assert!(tie.tie_log.endid.is_none());
        assert!(tie.children.is_empty());
    }

    #[test]
    fn tie_deserializes_xml_id() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie xml:id="t1"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tie_deserializes_startid_and_endid() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie startid="#n1" endid="#n2"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.startid.is_some());
        assert!(tie.tie_log.endid.is_some());
    }

    #[test]
    fn tie_deserializes_tstamp_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie tstamp="1" tstamp2="0m+2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.tstamp.is_some());
        assert!(tie.tie_log.tstamp2.is_some());
    }

    #[test]
    fn tie_deserializes_staff_and_layer() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie staff="1" layer="1"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.staff, vec![1]);
        assert_eq!(tie.tie_log.layer, vec![1]);
    }

    #[test]
    fn tie_deserializes_multiple_staff_values() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie staff="1 2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.staff, vec![1, 2]);
    }

    #[test]
    fn tie_deserializes_visual_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie curvedir="above" color="red"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.curvedir.is_some());
        assert!(tie.tie_vis.color.is_some());
    }

    #[test]
    fn tie_deserializes_bezier_attribute() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie bezier="19 45 -32 118"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.bezier.is_some());
    }

    #[test]
    fn tie_deserializes_gestural_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie tstamp2.ges="0m+2.5"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_ges.tstamp2_ges.is_some());
    }

    #[test]
    fn tie_deserializes_full_attributes() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie xml:id="t1" startid="#n1" endid="#n2" staff="1" layer="1" curvedir="below"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
        assert!(tie.tie_log.startid.is_some());
        assert!(tie.tie_log.endid.is_some());
        assert_eq!(tie.tie_log.staff, vec![1]);
        assert!(tie.tie_vis.curvedir.is_some());
    }

    #[test]
    fn tie_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie xml:id="t1" unknown="value"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tie_deserializes_evaluate_attribute() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie evaluate="all"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.evaluate.is_some());
    }

    #[test]
    fn tie_deserializes_coordinate_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie x="100" y="200" x2="300" y2="250"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_vis.x, Some(100.0));
        assert_eq!(tie.tie_vis.y, Some(200.0));
        assert_eq!(tie.tie_vis.x2, Some(300.0));
        assert_eq!(tie.tie_vis.y2, Some(250.0));
    }

    #[test]
    fn tie_deserializes_offset_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.startho.is_some());
        assert!(tie.tie_vis.endho.is_some());
        assert!(tie.tie_vis.startvo.is_some());
        assert!(tie.tie_vis.endvo.is_some());
    }

    #[test]
    fn tie_deserializes_plist_attribute() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie plist="#n1 #n2"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.plist.len(), 2);
    }

    #[test]
    fn tie_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie lform="dashed" lwidth="medium"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.lform.is_some());
        assert!(tie.tie_vis.lwidth.is_some());
    }

    // ============================================================================
    // Dynam deserialization tests
    // ============================================================================

    #[test]
    fn dynam_deserializes_from_empty_element() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam/>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.common.xml_id.is_none());
        assert!(dynam.dynam_log.startid.is_none());
        assert!(dynam.children.is_empty());
    }

    #[test]
    fn dynam_deserializes_with_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r#"<dynam>f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "f"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_deserializes_longer_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r#"<dynam>cresc. poco a poco</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "cresc. poco a poco"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_deserializes_xml_id() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:id="d1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dynam_deserializes_startid() {
        use tusk_model::elements::Dynam;

        let xml = r##"<dynam startid="#n1">f</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_log.startid.is_some());
    }

    #[test]
    fn dynam_deserializes_staff_and_layer() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam staff="1" layer="1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.staff, vec![1]);
        assert_eq!(dynam.dynam_log.layer, vec![1]);
    }

    #[test]
    fn dynam_deserializes_tstamp_attributes() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam tstamp="2" tstamp2="1m+1">cresc.</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_log.tstamp.is_some());
        assert!(dynam.dynam_log.tstamp2.is_some());
    }

    #[test]
    fn dynam_deserializes_place_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="above" staff="1" tstamp="1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_vis.place.is_some());
    }

    #[test]
    fn dynam_deserializes_extender_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam extender="true" tstamp="1" tstamp2="2m+1">dim.</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_vis.extender.is_some());
    }

    #[test]
    fn dynam_deserializes_val_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam val="84" staff="1" tstamp="1">f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_ges.val.is_some());
    }

    #[test]
    fn dynam_deserializes_plist_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r##"<dynam plist="#n1 #n2 #n3 #n4" startid="#n1">cresc.</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.plist.len(), 4);
    }

    #[test]
    fn dynam_deserializes_full_attributes() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r##"<dynam xml:id="d1" staff="2" place="above" startid="#n1" endid="#n4" plist="#n1 #n2 #n3 #n4">cresc. poco a poco</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
        assert_eq!(dynam.dynam_log.staff, vec![2]);
        assert!(dynam.dynam_vis.place.is_some());
        assert!(dynam.dynam_log.startid.is_some());
        assert!(dynam.dynam_log.endid.is_some());
        assert_eq!(dynam.dynam_log.plist.len(), 4);

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "cresc. poco a poco"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:id="d1" unknown="value">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dynam_deserializes_multiple_staff_values() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="between" staff="1 2" tstamp="1">f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.staff, vec![1, 2]);
    }

    #[test]
    fn dynam_deserializes_vgrp_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="below" staff="1" tstamp="2" vgrp="40">sf</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_vis.vgrp, Some(40));
    }

    #[test]
    fn dynam_deserializes_coordinate_attributes() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam x="100" y="200">mf</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_vis.x, Some(100.0));
        assert_eq!(dynam.dynam_vis.y, Some(200.0));
    }

    #[test]
    fn dynam_deserializes_duration_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam staff="2" tstamp="3" dur="1">cresc. poco a poco</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(!dynam.dynam_log.dur.is_empty());
    }

    #[test]
    fn dynam_deserializes_lang_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:lang="it">forte</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.lang.xml_lang, Some("it".to_string()));
    }

    // ============================================================================
    // Hairpin deserialization tests
    // ============================================================================

    #[test]
    fn hairpin_deserializes_from_empty_element() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.common.xml_id.is_none());
        assert!(hairpin.hairpin_log.startid.is_none());
        assert!(hairpin.hairpin_log.endid.is_none());
        assert!(hairpin.hairpin_log.form.is_none());
    }

    #[test]
    fn hairpin_deserializes_xml_id() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin xml:id="h1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn hairpin_deserializes_form_cres() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin form="cres"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Cres));
    }

    #[test]
    fn hairpin_deserializes_form_dim() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin form="dim"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    }

    #[test]
    fn hairpin_deserializes_niente_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin niente="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.niente, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_startid_endid() {
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin startid="#n1" endid="#n2"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.startid.is_some());
        assert!(hairpin.hairpin_log.endid.is_some());
    }

    #[test]
    fn hairpin_deserializes_staff_and_layer() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin staff="1" layer="1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.staff, vec![1]);
        assert_eq!(hairpin.hairpin_log.layer, vec![1]);
    }

    #[test]
    fn hairpin_deserializes_multiple_staff_values() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin staff="1 2"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.staff, vec![1, 2]);
    }

    #[test]
    fn hairpin_deserializes_tstamp_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin tstamp="1" tstamp2="0m+4"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.tstamp.is_some());
        assert!(hairpin.hairpin_log.tstamp2.is_some());
    }

    #[test]
    fn hairpin_deserializes_visual_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin place="above" color="red"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.place.is_some());
        assert!(hairpin.hairpin_vis.color.is_some());
    }

    #[test]
    fn hairpin_deserializes_opening_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin opening="1.5"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.opening.is_some());
    }

    #[test]
    fn hairpin_deserializes_closed_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin closed="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.closed, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_opening_vertical_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin opening.vertical="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            hairpin.hairpin_vis.opening_vertical,
            Some(DataBoolean::True)
        );
    }

    #[test]
    fn hairpin_deserializes_angle_optimize_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin angle.optimize="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.angle_optimize, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_line_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin lform="solid" lwidth="medium"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.lform.is_some());
        assert!(hairpin.hairpin_vis.lwidth.is_some());
    }

    #[test]
    fn hairpin_deserializes_coordinate_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin x="100" y="200" x2="300" y2="250"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.x, Some(100.0));
        assert_eq!(hairpin.hairpin_vis.y, Some(200.0));
        assert_eq!(hairpin.hairpin_vis.x2, Some(300.0));
        assert_eq!(hairpin.hairpin_vis.y2, Some(250.0));
    }

    #[test]
    fn hairpin_deserializes_offset_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.startho.is_some());
        assert!(hairpin.hairpin_vis.endho.is_some());
        assert!(hairpin.hairpin_vis.startvo.is_some());
        assert!(hairpin.hairpin_vis.endvo.is_some());
    }

    #[test]
    fn hairpin_deserializes_gestural_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin dur.ges="4" dur.ppq="480"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_ges.dur_ges.is_some());
        assert_eq!(hairpin.hairpin_ges.dur_ppq, Some(480));
    }

    #[test]
    fn hairpin_deserializes_midi_val_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin val="64" val2="100"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_ges.val.is_some());
        assert!(hairpin.hairpin_ges.val2.is_some());
    }

    #[test]
    fn hairpin_deserializes_full_attributes() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin xml:id="h1" form="cres" startid="#n1" endid="#n2" staff="1" layer="1" place="below" opening="2"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Cres));
        assert!(hairpin.hairpin_log.startid.is_some());
        assert!(hairpin.hairpin_log.endid.is_some());
        assert_eq!(hairpin.hairpin_log.staff, vec![1]);
        assert!(hairpin.hairpin_vis.place.is_some());
        assert!(hairpin.hairpin_vis.opening.is_some());
    }

    #[test]
    fn hairpin_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin xml:id="h1" unknown="value"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn hairpin_deserializes_evaluate_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin evaluate="all"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.evaluate.is_some());
    }

    #[test]
    fn hairpin_deserializes_vgrp_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin vgrp="1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.vgrp, Some(1));
    }

    #[test]
    fn hairpin_deserializes_dur_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin dur="4"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(!hairpin.hairpin_log.dur.is_empty());
    }

    #[test]
    fn hairpin_deserializes_plist_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin plist="#n1 #n2 #n3"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.plist.len(), 3);
    }

    // ============================================================================
    // Dir (directive) deserialization tests
    // ============================================================================

    #[test]
    fn dir_deserializes_from_empty_element() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir/>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.common.xml_id.is_none());
        assert!(dir.dir_log.startid.is_none());
        assert!(dir.children.is_empty());
    }

    #[test]
    fn dir_deserializes_with_text_content() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir>affettuoso</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "affettuoso"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dir_deserializes_xml_id() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:id="dir1">arco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
    }

    #[test]
    fn dir_deserializes_startid() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir startid="#n1">pizz.</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.startid.is_some());
    }

    #[test]
    fn dir_deserializes_endid() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir startid="#n1" endid="#n4">legato</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.startid.is_some());
        assert!(dir.dir_log.endid.is_some());
    }

    #[test]
    fn dir_deserializes_staff_and_layer() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir staff="1" layer="1">dolce</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.staff, vec![1]);
        assert_eq!(dir.dir_log.layer, vec![1]);
    }

    #[test]
    fn dir_deserializes_tstamp_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir tstamp="1" tstamp2="0m+4">rit.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.tstamp.is_some());
        assert!(dir.dir_log.tstamp2.is_some());
    }

    #[test]
    fn dir_deserializes_place_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir place="above" staff="1" tstamp="1">sul G</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.place.is_some());
    }

    #[test]
    fn dir_deserializes_extender_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir extender="true" tstamp="1" tstamp2="1m+1">accel.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.extender.is_some());
    }

    #[test]
    fn dir_deserializes_lang_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:lang="it">con fuoco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.lang.xml_lang, Some("it".to_string()));
    }

    #[test]
    fn dir_deserializes_dur_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir tstamp="1" dur="2">poco a poco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(!dir.dir_log.dur.is_empty());
    }

    #[test]
    fn dir_deserializes_plist_attribute() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir plist="#n1 #n2 #n3">espressivo</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.plist.len(), 3);
    }

    #[test]
    fn dir_deserializes_visual_color_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir color="red">important</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.color.is_some());
    }

    #[test]
    fn dir_deserializes_coordinate_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir x="100" y="200">text</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.ho.is_some() || dir.dir_vis.x.is_some());
    }

    #[test]
    fn dir_deserializes_vgrp_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir vgrp="1" tstamp="1">align group</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_vis.vgrp, Some(1));
    }

    #[test]
    fn dir_deserializes_gestural_duration_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir dur.ges="4" dur.ppq="480">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_ges.dur_ges.is_some());
        assert_eq!(dir.dir_ges.dur_ppq, Some(480));
    }

    #[test]
    fn dir_deserializes_multiple_staff_values() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir staff="1 2" place="between">between staves</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.staff, vec![1, 2]);
    }

    #[test]
    fn dir_deserializes_full_attributes() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r##"<dir xml:id="dir1" staff="1" place="above" startid="#n1" endid="#n4" plist="#n1 #n2 #n3 #n4" extender="true">molto espressivo</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
        assert_eq!(dir.dir_log.staff, vec![1]);
        assert!(dir.dir_vis.place.is_some());
        assert!(dir.dir_log.startid.is_some());
        assert!(dir.dir_log.endid.is_some());
        assert_eq!(dir.dir_log.plist.len(), 4);
        assert!(dir.dir_vis.extender.is_some());

        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "molto espressivo"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dir_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:id="dir1" unknown="value">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
    }

    #[test]
    fn dir_deserializes_evaluate_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir evaluate="all">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.evaluate.is_some());
    }

    #[test]
    fn dir_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir lform="dashed" lwidth="medium" extender="true">dim.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.lform.is_some());
        assert!(dir.dir_vis.lwidth.is_some());
    }

    #[test]
    fn dir_deserializes_rend_children() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir xml:id="d1"><rend fontweight="bold">forte</rend></dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("d1".to_string()));
        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Rend(rend) => {
                assert_eq!(rend.children.len(), 1);
            }
            _ => panic!("Expected Rend child"),
        }
    }

    #[test]
    fn dir_deserializes_mixed_content_with_rend() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir xml:id="d2">play <rend fontstyle="italic">quietly</rend> here</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("d2".to_string()));
        assert_eq!(dir.children.len(), 3);
        match &dir.children[0] {
            DirChild::Text(text) => assert!(text.starts_with("play")),
            _ => panic!("Expected Text child first"),
        }
        match &dir.children[1] {
            DirChild::Rend(_) => {}
            _ => panic!("Expected Rend child second"),
        }
        match &dir.children[2] {
            DirChild::Text(text) => assert!(text.ends_with("here")),
            _ => panic!("Expected Text child third"),
        }
    }

    #[test]
    fn dir_deserializes_lb_children() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir>line one<lb/>line two</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.children.len(), 3);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "line one"),
            _ => panic!("Expected Text child first"),
        }
        match &dir.children[1] {
            DirChild::Lb(_) => {}
            _ => panic!("Expected Lb child second"),
        }
        match &dir.children[2] {
            DirChild::Text(text) => assert_eq!(text, "line two"),
            _ => panic!("Expected Text child third"),
        }
    }

    // ============================================================================
    // Tempo deserialization tests
    // ============================================================================

    #[test]
    fn tempo_deserializes_from_empty_element() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo/>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.common.xml_id.is_none());
        assert!(tempo.tempo_log.startid.is_none());
        assert!(tempo.children.is_empty());
    }

    #[test]
    fn tempo_deserializes_with_text_content() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text, "Allegro"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn tempo_deserializes_xml_id() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:id="tempo1">Andante</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
    }

    #[test]
    fn tempo_deserializes_startid() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo startid="#n1">Moderato</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.startid.is_some());
    }

    #[test]
    fn tempo_deserializes_staff_and_tstamp() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo staff="1" tstamp="1">Presto</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.staff, vec![1]);
        assert!(tempo.tempo_log.tstamp.is_some());
    }

    #[test]
    fn tempo_deserializes_mm_attributes() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo mm="120" mm.unit="4" mm.dots="0">♩ = 120</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.mm.is_some());
        assert!(tempo.tempo_log.mm_unit.is_some());
        assert!(tempo.tempo_log.mm_dots.is_some());
    }

    #[test]
    fn tempo_deserializes_func_instantaneous() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="instantaneous">Largo</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
    }

    #[test]
    fn tempo_deserializes_func_continuous() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="continuous" tstamp="1" tstamp2="0m+4">accel.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Continuous));
        assert!(tempo.tempo_log.tstamp2.is_some());
    }

    #[test]
    fn tempo_deserializes_func_metricmod() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="metricmod">♩ = ♪</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Metricmod));
    }

    #[test]
    fn tempo_deserializes_place_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo place="above" staff="1" tstamp="1">Vivace</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.place.is_some());
    }

    #[test]
    fn tempo_deserializes_extender_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo extender="true" tstamp="1" tstamp2="1m+1">rit.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.extender.is_some());
    }

    #[test]
    fn tempo_deserializes_lang_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:lang="it">Allegro con brio</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.lang.xml_lang, Some("it".to_string()));
    }

    #[test]
    fn tempo_deserializes_midi_bpm() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo midi.bpm="120">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_ges.midi_bpm.is_some());
    }

    #[test]
    fn tempo_deserializes_midi_mspb() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo midi.mspb="500000">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_ges.midi_mspb.is_some());
    }

    #[test]
    fn tempo_deserializes_visual_color_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo color="red">Largo</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.color.is_some());
    }

    #[test]
    fn tempo_deserializes_coordinate_attributes() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo x="100" y="200">Adagio</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.x.is_some());
        assert!(tempo.tempo_vis.y.is_some());
    }

    #[test]
    fn tempo_deserializes_layer_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo layer="1" staff="1" tstamp="1">Andante</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.layer, vec![1]);
    }

    #[test]
    fn tempo_deserializes_endid() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo startid="#n1" endid="#n4" func="continuous">rallentando</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.startid.is_some());
        assert!(tempo.tempo_log.endid.is_some());
    }

    #[test]
    fn tempo_deserializes_plist_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo plist="#n1 #n2 #n3">Presto</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.plist.len(), 3);
    }

    #[test]
    fn tempo_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:id="tempo1" unknown="value">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
    }

    #[test]
    fn tempo_deserializes_all_common_attributes() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo xml:id="tempo1" staff="1" tstamp="1" mm="120" mm.unit="4" func="instantaneous" place="above" extender="false" xml:lang="de">Schnell</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
        assert_eq!(tempo.tempo_log.staff, vec![1]);
        assert!(tempo.tempo_log.tstamp.is_some());
        assert!(tempo.tempo_log.mm.is_some());
        assert!(tempo.tempo_log.mm_unit.is_some());
        assert!(tempo.tempo_log.func.is_some());
        assert!(tempo.tempo_vis.place.is_some());
        assert!(tempo.tempo_vis.extender.is_some());
        assert_eq!(tempo.lang.xml_lang, Some("de".to_string()));

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text, "Schnell"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn tempo_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo lform="dashed" lwidth="medium" extender="true">accel.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.lform.is_some());
        assert!(tempo.tempo_vis.lwidth.is_some());
    }

    #[test]
    fn tempo_deserializes_rend_child() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo><rend fontsize="6.9pt" fontweight="bold">A</rend></tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Rend(rend) => {
                assert!(rend.typography.fontsize.is_some());
                assert!(rend.typography.fontweight.is_some());
                assert_eq!(rend.children.len(), 1);
            }
            _ => panic!("Expected Rend child, got {:?}", tempo.children[0]),
        }
    }

    #[test]
    fn tempo_deserializes_mixed_text_and_rend_children() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Text before <rend fontweight="bold">bold</rend> text after</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 3);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text.trim(), "Text before"),
            _ => panic!("Expected Text child"),
        }
        match &tempo.children[1] {
            TempoChild::Rend(_) => {}
            _ => panic!("Expected Rend child"),
        }
        match &tempo.children[2] {
            TempoChild::Text(text) => assert_eq!(text.trim(), "text after"),
            _ => panic!("Expected Text child"),
        }
    }

    #[test]
    fn tempo_deserializes_lb_child() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Tempo<lb/>marking</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.children.len() >= 2);
        let has_lb = tempo
            .children
            .iter()
            .any(|c| matches!(c, TempoChild::Lb(_)));
        assert!(has_lb, "Expected Lb child element");
    }

    // ============================================================================
    // Fermata tests
    // ============================================================================

    #[test]
    fn fermata_deserializes_from_empty_element() {
        let xml = r#"<fermata/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.common.xml_id.is_none());
        assert!(fermata.fermata_log.startid.is_none());
        assert!(fermata.fermata_vis.form.is_none());
    }

    #[test]
    fn fermata_deserializes_xml_id() {
        let xml = r#"<fermata xml:id="f1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    #[test]
    fn fermata_deserializes_startid() {
        let xml = r##"<fermata startid="#note1"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_log.startid.is_some());
    }

    #[test]
    fn fermata_deserializes_staff_and_tstamp() {
        let xml = r#"<fermata staff="1" tstamp="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.staff, vec![1]);
        assert!(fermata.fermata_log.tstamp.is_some());
    }

    #[test]
    fn fermata_deserializes_form_norm() {
        use tusk_model::att::AttFermataVisForm;

        let xml = r#"<fermata form="norm"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Norm));
    }

    #[test]
    fn fermata_deserializes_form_inv() {
        use tusk_model::att::AttFermataVisForm;

        let xml = r#"<fermata form="inv"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Inv));
    }

    #[test]
    fn fermata_deserializes_shape_curved() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="curved"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Curved));
    }

    #[test]
    fn fermata_deserializes_shape_square() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="square"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Square));
    }

    #[test]
    fn fermata_deserializes_shape_angular() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="angular"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Angular));
    }

    #[test]
    fn fermata_deserializes_place_attribute() {
        let xml = r#"<fermata place="above"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.place.is_some());
    }

    #[test]
    fn fermata_deserializes_color_attribute() {
        let xml = r#"<fermata color="red"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.color.is_some());
    }

    #[test]
    fn fermata_deserializes_coordinate_attributes() {
        let xml = r#"<fermata x="100" y="200"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.x, Some(100.0));
        assert_eq!(fermata.fermata_vis.y, Some(200.0));
    }

    #[test]
    fn fermata_deserializes_layer_attribute() {
        let xml = r#"<fermata layer="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.layer, vec![1]);
    }

    #[test]
    fn fermata_deserializes_endid() {
        let xml = r##"<fermata startid="#note1" endid="#note2"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_log.startid.is_some());
        assert!(fermata.fermata_log.endid.is_some());
    }

    #[test]
    fn fermata_deserializes_plist_attribute() {
        let xml = r##"<fermata plist="#note1 #note2"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.plist.len(), 2);
    }

    #[test]
    fn fermata_deserializes_gestural_duration() {
        let xml = r#"<fermata dur.ppq="480" dur.real="2.5"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_ges.dur_ppq, Some(480));
        assert_eq!(fermata.fermata_ges.dur_real, Some(2.5));
    }

    #[test]
    fn fermata_deserializes_glyph_attributes() {
        use tusk_model::att::AttFermataVisGlyphAuth;

        let xml = r#"<fermata glyph.auth="smufl" glyph.name="fermataAbove"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            fermata.fermata_vis.glyph_auth,
            Some(AttFermataVisGlyphAuth::Smufl)
        );
        assert_eq!(
            fermata.fermata_vis.glyph_name,
            Some("fermataAbove".to_string())
        );
    }

    #[test]
    fn fermata_deserializes_visual_offset_attributes() {
        let xml = r#"<fermata ho="2" vo="-1" to="0.5"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.ho.is_some());
        assert!(fermata.fermata_vis.vo.is_some());
        assert!(fermata.fermata_vis.to.is_some());
    }

    #[test]
    fn fermata_deserializes_vgrp_attribute() {
        let xml = r#"<fermata vgrp="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.vgrp, Some(1));
    }

    #[test]
    fn fermata_handles_unknown_attributes_leniently() {
        let xml = r#"<fermata xml:id="f1" unknown="value"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    #[test]
    fn fermata_deserializes_all_common_attributes() {
        use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};

        let xml = r##"<fermata xml:id="f1" startid="#note1" staff="1" tstamp="2.5" form="norm" shape="curved" place="above" color="blue"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
        assert!(fermata.fermata_log.startid.is_some());
        assert_eq!(fermata.fermata_log.staff, vec![1]);
        assert!(fermata.fermata_log.tstamp.is_some());
        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Norm));
        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Curved));
        assert!(fermata.fermata_vis.place.is_some());
        assert!(fermata.fermata_vis.color.is_some());
    }

    #[test]
    fn fermata_deserializes_enclose_attribute() {
        let xml = r#"<fermata enclose="paren"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.enclose.is_some());
    }

    #[test]
    fn fermata_deserializes_altsym_attribute() {
        let xml = r##"<fermata altsym="#mySymbol"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.altsym.is_some());
    }

    #[test]
    fn fermata_deserializes_with_non_empty_element() {
        // Even though fermata has empty content, we handle non-empty elements gracefully
        let xml = r#"<fermata xml:id="f1">   </fermata>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    // ============================================================================
    // Reh (rehearsal mark) tests
    // ============================================================================

    #[test]
    fn reh_deserializes_basic_text() {
        use tusk_model::elements::{Reh, RehChild};

        let xml = r#"<reh xml:id="r1">A</reh>"#;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.common.xml_id, Some("r1".to_string()));
        assert_eq!(reh.children.len(), 1);
        match &reh.children[0] {
            RehChild::Text(text) => assert_eq!(text, "A"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn reh_deserializes_with_attributes() {
        use tusk_model::elements::Reh;

        let xml = r##"<reh xml:id="r1" staff="1" tstamp="1" place="above">1</reh>"##;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.common.xml_id, Some("r1".to_string()));
        assert_eq!(reh.reh_log.staff, vec![1]);
        assert!(reh.reh_log.tstamp.is_some());
        assert!(reh.reh_vis.place.is_some());
    }

    #[test]
    fn reh_deserializes_with_rend_child() {
        use tusk_model::elements::{Reh, RehChild};

        let xml = r#"<reh xml:id="r1"><rend fontweight="bold">A</rend></reh>"#;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.children.len(), 1);
        match &reh.children[0] {
            RehChild::Rend(rend) => {
                assert!(rend.typography.fontweight.is_some());
            }
            _ => panic!("Expected rend child"),
        }
    }

    #[test]
    fn reh_deserializes_empty_element() {
        use tusk_model::elements::Reh;

        let xml = r#"<reh xml:id="r1"/>"#;
        let reh = Reh::from_mei_str(xml).expect("should deserialize");

        assert_eq!(reh.common.xml_id, Some("r1".to_string()));
        assert!(reh.children.is_empty());
    }

    // ============================================================================
    // BeamSpan roundtrip tests
    // ============================================================================

    #[test]
    fn beamspan_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::BeamSpan;

        let xml = r#"<beamSpan/>"#;
        let beamspan = BeamSpan::from_mei_str(xml).expect("should deserialize");
        let serialized = beamspan.to_mei_string().expect("should serialize");
        let reparsed = BeamSpan::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(beamspan.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn beamspan_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::BeamSpan;

        let xml = r##"<beamSpan xml:id="bs1" staff="1" tstamp="1" tstamp2="0m+4" startid="#n1" endid="#n4"/>"##;
        let beamspan = BeamSpan::from_mei_str(xml).expect("should deserialize");
        let serialized = beamspan.to_mei_string().expect("should serialize");
        let reparsed = BeamSpan::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(beamspan.common.xml_id, reparsed.common.xml_id);
        assert_eq!(beamspan.beam_span_log.staff, reparsed.beam_span_log.staff);
        assert!(reparsed.beam_span_log.startid.is_some());
        assert!(reparsed.beam_span_log.endid.is_some());
    }

    // ============================================================================
    // Octave roundtrip tests
    // ============================================================================

    #[test]
    fn octave_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Octave;

        let xml = r#"<octave/>"#;
        let octave = Octave::from_mei_str(xml).expect("should deserialize");
        let serialized = octave.to_mei_string().expect("should serialize");
        let reparsed = Octave::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(octave.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn octave_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Octave;

        let xml = r##"<octave xml:id="o1" staff="1" tstamp="1" tstamp2="0m+4" dis="8" dis.place="above" startid="#n1" endid="#n4"/>"##;
        let octave = Octave::from_mei_str(xml).expect("should deserialize");
        let serialized = octave.to_mei_string().expect("should serialize");
        let reparsed = Octave::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(octave.common.xml_id, reparsed.common.xml_id);
        assert_eq!(octave.octave_log.staff, reparsed.octave_log.staff);
        assert!(reparsed.octave_log.dis.is_some());
        assert!(reparsed.octave_log.dis_place.is_some());
    }

    // ============================================================================
    // Gliss roundtrip tests
    // ============================================================================

    #[test]
    fn gliss_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Gliss;

        let xml = r#"<gliss/>"#;
        let gliss = Gliss::from_mei_str(xml).expect("should deserialize");
        let serialized = gliss.to_mei_string().expect("should serialize");
        let reparsed = Gliss::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(gliss.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn gliss_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Gliss;

        let xml = r##"<gliss xml:id="g1" staff="1" tstamp="1" tstamp2="0m+2" startid="#n1" endid="#n2" lform="wavy"/>"##;
        let gliss = Gliss::from_mei_str(xml).expect("should deserialize");
        let serialized = gliss.to_mei_string().expect("should serialize");
        let reparsed = Gliss::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(gliss.common.xml_id, reparsed.common.xml_id);
        assert_eq!(gliss.gliss_log.staff, reparsed.gliss_log.staff);
        assert!(reparsed.gliss_log.startid.is_some());
        assert!(reparsed.gliss_vis.lform.is_some());
    }

    // ============================================================================
    // Lv roundtrip tests
    // ============================================================================

    #[test]
    fn lv_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Lv;

        let xml = r#"<lv/>"#;
        let lv = Lv::from_mei_str(xml).expect("should deserialize");
        let serialized = lv.to_mei_string().expect("should serialize");
        let reparsed = Lv::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(lv.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn lv_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::Lv;

        let xml = r##"<lv xml:id="lv1" staff="1" tstamp="1" tstamp2="0m+4" startid="#n1" endid="#n2" curvedir="above"/>"##;
        let lv = Lv::from_mei_str(xml).expect("should deserialize");
        let serialized = lv.to_mei_string().expect("should serialize");
        let reparsed = Lv::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(lv.common.xml_id, reparsed.common.xml_id);
        assert_eq!(lv.lv_log.staff, reparsed.lv_log.staff);
        assert!(reparsed.lv_log.startid.is_some());
        assert!(reparsed.lv_vis.curvedir.is_some());
    }

    // ============================================================================
    // BracketSpan roundtrip tests
    // ============================================================================

    #[test]
    fn bracketspan_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::BracketSpan;

        let xml = r#"<bracketSpan/>"#;
        let bracketspan = BracketSpan::from_mei_str(xml).expect("should deserialize");
        let serialized = bracketspan.to_mei_string().expect("should serialize");
        let reparsed = BracketSpan::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(bracketspan.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn bracketspan_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::BracketSpan;

        let xml = r##"<bracketSpan xml:id="bsp1" staff="1" tstamp="1" tstamp2="0m+4" startid="#n1" endid="#n4" func="ligature"/>"##;
        let bracketspan = BracketSpan::from_mei_str(xml).expect("should deserialize");
        let serialized = bracketspan.to_mei_string().expect("should serialize");
        let reparsed = BracketSpan::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(bracketspan.common.xml_id, reparsed.common.xml_id);
        assert_eq!(
            bracketspan.bracket_span_log.staff,
            reparsed.bracket_span_log.staff
        );
        assert!(reparsed.bracket_span_log.startid.is_some());
        assert!(reparsed.bracket_span_log.func.is_some());
    }

    // ============================================================================
    // BTrem (bowed tremolo) roundtrip tests
    // ============================================================================

    #[test]
    fn btrem_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::BTrem;

        let xml = r#"<bTrem/>"#;
        let btrem = BTrem::from_mei_str(xml).expect("should deserialize");
        let serialized = btrem.to_mei_string().expect("should serialize");
        let reparsed = BTrem::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(btrem.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn btrem_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::BTrem;

        let xml = r#"<bTrem xml:id="bt1" dur="4" dots="0" unitdur="32"/>"#;
        let btrem = BTrem::from_mei_str(xml).expect("should deserialize");
        let serialized = btrem.to_mei_string().expect("should serialize");
        let reparsed = BTrem::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(btrem.common.xml_id, reparsed.common.xml_id);
        assert!(reparsed.b_trem_log.dur.is_some());
        assert!(reparsed.b_trem_ges.unitdur.is_some());
    }

    #[test]
    fn btrem_roundtrip_with_note_child() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::{BTrem, BTremChild};

        let xml = r#"<bTrem xml:id="bt1" dur="4"><note xml:id="n1" pname="c" oct="4"/></bTrem>"#;
        let btrem = BTrem::from_mei_str(xml).expect("should deserialize");
        let serialized = btrem.to_mei_string().expect("should serialize");
        let reparsed = BTrem::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(btrem.common.xml_id, reparsed.common.xml_id);
        assert_eq!(btrem.children.len(), reparsed.children.len());
        match (&btrem.children[0], &reparsed.children[0]) {
            (BTremChild::Note(orig), BTremChild::Note(rep)) => {
                assert_eq!(orig.common.xml_id, rep.common.xml_id);
            }
            _ => panic!("Expected Note children"),
        }
    }

    // ============================================================================
    // FTrem (fingered tremolo) roundtrip tests
    // ============================================================================

    #[test]
    fn ftrem_roundtrip_empty() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::FTrem;

        let xml = r#"<fTrem/>"#;
        let ftrem = FTrem::from_mei_str(xml).expect("should deserialize");
        let serialized = ftrem.to_mei_string().expect("should serialize");
        let reparsed = FTrem::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(ftrem.common.xml_id, reparsed.common.xml_id);
    }

    #[test]
    fn ftrem_roundtrip_with_attributes() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::FTrem;

        let xml = r#"<fTrem xml:id="ft1" dur="2" beams="3" unitdur="16"/>"#;
        let ftrem = FTrem::from_mei_str(xml).expect("should deserialize");
        let serialized = ftrem.to_mei_string().expect("should serialize");
        let reparsed = FTrem::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(ftrem.common.xml_id, reparsed.common.xml_id);
        assert!(reparsed.f_trem_log.dur.is_some());
        assert!(reparsed.f_trem_ges.unitdur.is_some());
    }

    #[test]
    fn ftrem_roundtrip_with_note_children() {
        use crate::serializer::MeiSerialize;
        use tusk_model::elements::{FTrem, FTremChild};

        let xml = r#"<fTrem xml:id="ft1" dur="2" beams="2"><note xml:id="n1" pname="c" oct="4"/><note xml:id="n2" pname="e" oct="4"/></fTrem>"#;
        let ftrem = FTrem::from_mei_str(xml).expect("should deserialize");
        let serialized = ftrem.to_mei_string().expect("should serialize");
        let reparsed = FTrem::from_mei_str(&serialized).expect("should reparse");

        assert_eq!(ftrem.common.xml_id, reparsed.common.xml_id);
        assert_eq!(ftrem.children.len(), reparsed.children.len());
        assert_eq!(ftrem.children.len(), 2);
        match (&ftrem.children[0], &reparsed.children[0]) {
            (FTremChild::Note(orig), FTremChild::Note(rep)) => {
                assert_eq!(orig.common.xml_id, rep.common.xml_id);
            }
            _ => panic!("Expected Note children"),
        }
    }
}

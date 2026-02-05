//! Round-trip serialization tests for tablature MEI elements.
//!
//! Tests for TabGrp, TabDurSym, Fing, FingGrp, String, Course, and Tuning elements
//! used in tablature notation for stringed instruments.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// TabGrp Tests
// ============================================================================

#[test]
fn tab_grp_roundtrip_empty() {
    use tusk_model::elements::TabGrp;

    let original = TabGrp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabGrp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn tab_grp_roundtrip_with_xml_id() {
    use tusk_model::elements::TabGrp;

    let mut original = TabGrp::default();
    original.common.xml_id = Some("tg1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tg1".to_string()));
}

#[test]
fn tab_grp_roundtrip_with_note_children() {
    use tusk_model::data::{DataOctave, DataPitchname};
    use tusk_model::elements::{Note, TabGrp, TabGrpChild};

    let mut original = TabGrp::default();
    original.common.xml_id = Some("tg1".to_string());

    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    original.children.push(TabGrpChild::Note(Box::new(note1)));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    original.children.push(TabGrpChild::Note(Box::new(note2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tg1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        TabGrpChild::Note(n) => {
            assert_eq!(n.common.xml_id, Some("n1".to_string()));
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("c".to_string())));
        }
        _ => panic!("Expected Note child"),
    }

    match &parsed.children[1] {
        TabGrpChild::Note(n) => {
            assert_eq!(n.common.xml_id, Some("n2".to_string()));
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("e".to_string())));
        }
        _ => panic!("Expected Note child"),
    }
}

#[test]
fn tab_grp_roundtrip_with_tab_dur_sym_child() {
    use tusk_model::elements::{TabDurSym, TabGrp, TabGrpChild};

    let mut original = TabGrp::default();
    original.common.xml_id = Some("tg1".to_string());

    let mut tds = TabDurSym::default();
    tds.common.xml_id = Some("tds1".to_string());
    original
        .children
        .push(TabGrpChild::TabDurSym(Box::new(tds)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tg1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        TabGrpChild::TabDurSym(t) => {
            assert_eq!(t.common.xml_id, Some("tds1".to_string()));
        }
        _ => panic!("Expected TabDurSym child"),
    }
}

#[test]
fn tab_grp_roundtrip_with_rest_child() {
    use tusk_model::elements::{Rest, TabGrp, TabGrpChild};

    let mut original = TabGrp::default();
    original.common.xml_id = Some("tg1".to_string());

    let mut rest = Rest::default();
    rest.common.xml_id = Some("r1".to_string());
    original.children.push(TabGrpChild::Rest(Box::new(rest)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    assert!(matches!(parsed.children[0], TabGrpChild::Rest(_)));
}

// ============================================================================
// TabDurSym Tests
// ============================================================================

#[test]
fn tab_dur_sym_roundtrip_empty() {
    use tusk_model::elements::TabDurSym;

    let original = TabDurSym::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabDurSym::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn tab_dur_sym_roundtrip_with_xml_id() {
    use tusk_model::elements::TabDurSym;

    let mut original = TabDurSym::default();
    original.common.xml_id = Some("tds1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = TabDurSym::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tds1".to_string()));
}

// ============================================================================
// Fing Tests
// ============================================================================

#[test]
fn fing_roundtrip_empty() {
    use tusk_model::elements::Fing;

    let original = Fing::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fing::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn fing_roundtrip_with_xml_id() {
    use tusk_model::elements::Fing;

    let mut original = Fing::default();
    original.common.xml_id = Some("f1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fing::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("f1".to_string()));
}

#[test]
fn fing_roundtrip_with_text_child() {
    use tusk_model::elements::{Fing, FingChild};

    let mut original = Fing::default();
    original.common.xml_id = Some("f1".to_string());
    original.children.push(FingChild::Text("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fing::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("f1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        FingChild::Text(t) => {
            assert_eq!(t, "1");
        }
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// FingGrp Tests
// ============================================================================

#[test]
fn fing_grp_roundtrip_empty() {
    use tusk_model::elements::FingGrp;

    let original = FingGrp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = FingGrp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn fing_grp_roundtrip_with_xml_id() {
    use tusk_model::elements::FingGrp;

    let mut original = FingGrp::default();
    original.common.xml_id = Some("fg1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FingGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("fg1".to_string()));
}

#[test]
fn fing_grp_roundtrip_with_fing_children() {
    use tusk_model::elements::{Fing, FingChild, FingGrp, FingGrpChild};

    let mut original = FingGrp::default();
    original.common.xml_id = Some("fg1".to_string());

    let mut fing1 = Fing::default();
    fing1.common.xml_id = Some("f1".to_string());
    fing1.children.push(FingChild::Text("1".to_string()));
    original.children.push(FingGrpChild::Fing(Box::new(fing1)));

    let mut fing2 = Fing::default();
    fing2.common.xml_id = Some("f2".to_string());
    fing2.children.push(FingChild::Text("2".to_string()));
    original.children.push(FingGrpChild::Fing(Box::new(fing2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FingGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("fg1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        FingGrpChild::Fing(f) => {
            assert_eq!(f.common.xml_id, Some("f1".to_string()));
        }
        _ => panic!("Expected Fing child"),
    }

    match &parsed.children[1] {
        FingGrpChild::Fing(f) => {
            assert_eq!(f.common.xml_id, Some("f2".to_string()));
        }
        _ => panic!("Expected Fing child"),
    }
}

#[test]
fn fing_grp_roundtrip_with_nested_fing_grp() {
    use tusk_model::elements::{Fing, FingGrp, FingGrpChild};

    let mut original = FingGrp::default();
    original.common.xml_id = Some("fg1".to_string());

    let mut inner = FingGrp::default();
    inner.common.xml_id = Some("fg2".to_string());

    let mut fing = Fing::default();
    fing.common.xml_id = Some("f1".to_string());
    inner.children.push(FingGrpChild::Fing(Box::new(fing)));

    original
        .children
        .push(FingGrpChild::FingGrp(Box::new(inner)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FingGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("fg1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        FingGrpChild::FingGrp(fg) => {
            assert_eq!(fg.common.xml_id, Some("fg2".to_string()));
            assert_eq!(fg.children.len(), 1);
        }
        _ => panic!("Expected FingGrp child"),
    }
}

// ============================================================================
// String Tests
// ============================================================================

#[test]
fn string_roundtrip_empty() {
    use tusk_model::elements::String as MeiString;

    let original = MeiString::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MeiString::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn string_roundtrip_with_xml_id() {
    use tusk_model::elements::String as MeiString;

    let mut original = MeiString::default();
    original.common.xml_id = Some("s1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MeiString::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("s1".to_string()));
}

#[test]
fn string_roundtrip_with_nested_string() {
    use tusk_model::elements::{String as MeiString, StringChild};

    let mut original = MeiString::default();
    original.common.xml_id = Some("s1".to_string());

    let mut inner = MeiString::default();
    inner.common.xml_id = Some("s2".to_string());
    original.children.push(StringChild::String(Box::new(inner)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MeiString::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("s1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        StringChild::String(s) => {
            assert_eq!(s.common.xml_id, Some("s2".to_string()));
        }
    }
}

// ============================================================================
// Course Tests
// ============================================================================

#[test]
fn course_roundtrip_empty() {
    use tusk_model::elements::Course;

    let original = Course::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Course::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn course_roundtrip_with_xml_id() {
    use tusk_model::elements::Course;

    let mut original = Course::default();
    original.common.xml_id = Some("c1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Course::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("c1".to_string()));
}

#[test]
fn course_roundtrip_with_string_children() {
    use tusk_model::elements::{Course, CourseChild, String as MeiString};

    let mut original = Course::default();
    original.common.xml_id = Some("c1".to_string());

    let mut string1 = MeiString::default();
    string1.common.xml_id = Some("s1".to_string());
    original
        .children
        .push(CourseChild::String(Box::new(string1)));

    let mut string2 = MeiString::default();
    string2.common.xml_id = Some("s2".to_string());
    original
        .children
        .push(CourseChild::String(Box::new(string2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Course::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("c1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        CourseChild::String(s) => {
            assert_eq!(s.common.xml_id, Some("s1".to_string()));
        }
    }

    match &parsed.children[1] {
        CourseChild::String(s) => {
            assert_eq!(s.common.xml_id, Some("s2".to_string()));
        }
    }
}

// ============================================================================
// Tuning Tests
// ============================================================================

#[test]
fn tuning_roundtrip_empty() {
    use tusk_model::elements::Tuning;

    let original = Tuning::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuning::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn tuning_roundtrip_with_xml_id() {
    use tusk_model::elements::Tuning;

    let mut original = Tuning::default();
    original.common.xml_id = Some("t1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuning::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
}

#[test]
fn tuning_roundtrip_with_course_children() {
    use tusk_model::elements::{Course, Tuning, TuningChild};

    let mut original = Tuning::default();
    original.common.xml_id = Some("t1".to_string());

    let mut course1 = Course::default();
    course1.common.xml_id = Some("c1".to_string());
    original
        .children
        .push(TuningChild::Course(Box::new(course1)));

    let mut course2 = Course::default();
    course2.common.xml_id = Some("c2".to_string());
    original
        .children
        .push(TuningChild::Course(Box::new(course2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuning::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        TuningChild::Course(c) => {
            assert_eq!(c.common.xml_id, Some("c1".to_string()));
        }
    }

    match &parsed.children[1] {
        TuningChild::Course(c) => {
            assert_eq!(c.common.xml_id, Some("c2".to_string()));
        }
    }
}

#[test]
fn tuning_roundtrip_with_nested_structure() {
    use tusk_model::elements::{Course, CourseChild, String as MeiString, Tuning, TuningChild};

    let mut original = Tuning::default();
    original.common.xml_id = Some("t1".to_string());

    let mut course = Course::default();
    course.common.xml_id = Some("c1".to_string());

    let mut string1 = MeiString::default();
    string1.common.xml_id = Some("s1".to_string());
    course.children.push(CourseChild::String(Box::new(string1)));

    let mut string2 = MeiString::default();
    string2.common.xml_id = Some("s2".to_string());
    course.children.push(CourseChild::String(Box::new(string2)));

    original
        .children
        .push(TuningChild::Course(Box::new(course)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuning::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        TuningChild::Course(c) => {
            assert_eq!(c.common.xml_id, Some("c1".to_string()));
            assert_eq!(c.children.len(), 2);
        }
    }
}

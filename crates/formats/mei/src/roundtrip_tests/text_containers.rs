//! Round-trip serialization tests for text container MEI elements.
//!
//! Tests for Group, Quote, Q, Phrase, Line, Refrain, Stack elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Group Tests
// ============================================================================

#[test]
fn group_roundtrip_empty() {
    use tusk_model::elements::Group;

    let original = Group::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Group::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn group_roundtrip_with_xml_id() {
    use tusk_model::elements::Group;

    let mut original = Group::default();
    original.common.xml_id = Some("group-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Group::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("group-1".to_string()));
}

// ============================================================================
// Quote Tests
// ============================================================================

#[test]
fn quote_roundtrip_empty() {
    use tusk_model::elements::Quote;

    let original = Quote::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Quote::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn quote_roundtrip_with_xml_id() {
    use tusk_model::elements::Quote;

    let mut original = Quote::default();
    original.common.xml_id = Some("quote-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Quote::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("quote-1".to_string()));
}

#[test]
fn quote_roundtrip_with_text() {
    use tusk_model::elements::{Quote, QuoteChild};

    let mut original = Quote::default();
    original.common.xml_id = Some("quote-1".to_string());
    original
        .children
        .push(QuoteChild::Text("To be or not to be".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Quote::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        QuoteChild::Text(t) => assert_eq!(t, "To be or not to be"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn quote_roundtrip_with_nested_q() {
    use tusk_model::elements::{Q, QChild, Quote, QuoteChild};

    let mut original = Quote::default();
    original.common.xml_id = Some("quote-1".to_string());

    let mut q = Q::default();
    q.basic.xml_id = Some("q-1".to_string());
    q.children.push(QChild::Text("inner quote".to_string()));
    original.children.push(QuoteChild::Q(Box::new(q)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Quote::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        QuoteChild::Q(inner) => {
            assert_eq!(inner.basic.xml_id, Some("q-1".to_string()));
        }
        _ => panic!("Expected Q child"),
    }
}

// ============================================================================
// Q Tests
// ============================================================================

#[test]
fn q_roundtrip_empty() {
    use tusk_model::elements::Q;

    let original = Q::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Q::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn q_roundtrip_with_xml_id() {
    use tusk_model::elements::Q;

    let mut original = Q::default();
    original.basic.xml_id = Some("q-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Q::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("q-1".to_string()));
}

#[test]
fn q_roundtrip_with_text() {
    use tusk_model::elements::{Q, QChild};

    let mut original = Q::default();
    original.basic.xml_id = Some("q-1".to_string());
    original
        .children
        .push(QChild::Text("quoted text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Q::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        QChild::Text(t) => assert_eq!(t, "quoted text"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn q_roundtrip_with_type() {
    use tusk_model::elements::Q;

    let mut original = Q::default();
    original.basic.xml_id = Some("q-1".to_string());
    original.r#type = vec!["direct".to_string()];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Q::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.r#type, vec!["direct".to_string()]);
}

#[test]
fn q_roundtrip_nested() {
    use tusk_model::elements::{Q, QChild};

    let mut original = Q::default();
    original.basic.xml_id = Some("q-outer".to_string());

    let mut inner = Q::default();
    inner.basic.xml_id = Some("q-inner".to_string());
    inner.children.push(QChild::Text("inner".to_string()));
    original.children.push(QChild::Q(Box::new(inner)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Q::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        QChild::Q(inner) => {
            assert_eq!(inner.basic.xml_id, Some("q-inner".to_string()));
        }
        _ => panic!("Expected nested Q"),
    }
}

// ============================================================================
// Phrase Tests
// ============================================================================

#[test]
fn phrase_roundtrip_empty() {
    use tusk_model::elements::Phrase;

    let original = Phrase::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Phrase::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn phrase_roundtrip_with_xml_id() {
    use tusk_model::elements::Phrase;

    let mut original = Phrase::default();
    original.common.xml_id = Some("phrase-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Phrase::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("phrase-1".to_string()));
}

#[test]
fn phrase_roundtrip_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Phrase;

    let mut original = Phrase::default();
    original.common.xml_id = Some("phrase-1".to_string());
    original.phrase_log.startid = Some(DataUri("#note-1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Phrase::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.phrase_log.startid,
        Some(DataUri("#note-1".to_string()))
    );
}

// ============================================================================
// Line Tests
// ============================================================================

#[test]
fn line_roundtrip_empty() {
    use tusk_model::elements::Line;

    let original = Line::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Line::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn line_roundtrip_with_xml_id() {
    use tusk_model::elements::Line;

    let mut original = Line::default();
    original.common.xml_id = Some("line-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Line::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("line-1".to_string()));
}

#[test]
fn line_roundtrip_with_text() {
    use tusk_model::elements::{Line, LineChild};

    let mut original = Line::default();
    original.common.xml_id = Some("line-1".to_string());
    original
        .children
        .push(LineChild::Text("A visual line".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Line::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LineChild::Text(t) => assert_eq!(t, "A visual line"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn line_roundtrip_with_attributes() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Line;

    let mut original = Line::default();
    original.common.xml_id = Some("line-1".to_string());
    original.line_log.startid = Some(DataUri("#note-1".to_string()));
    original.line_log.endid = Some(DataUri("#note-2".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Line::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.line_log.startid,
        Some(DataUri("#note-1".to_string()))
    );
    assert_eq!(parsed.line_log.endid, Some(DataUri("#note-2".to_string())));
}

// ============================================================================
// Refrain Tests
// ============================================================================

#[test]
fn refrain_roundtrip_empty() {
    use tusk_model::elements::Refrain;

    let original = Refrain::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Refrain::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn refrain_roundtrip_with_xml_id() {
    use tusk_model::elements::Refrain;

    let mut original = Refrain::default();
    original.common.xml_id = Some("refrain-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Refrain::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("refrain-1".to_string()));
}

#[test]
fn refrain_roundtrip_with_lb() {
    use tusk_model::elements::{Lb, Refrain, RefrainChild};

    let mut original = Refrain::default();
    original.common.xml_id = Some("refrain-1".to_string());

    let mut lb = Lb::default();
    lb.common.xml_id = Some("lb-1".to_string());
    original.children.push(RefrainChild::Lb(Box::new(lb)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Refrain::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RefrainChild::Lb(lb) => {
            assert_eq!(lb.common.xml_id, Some("lb-1".to_string()));
        }
        _ => panic!("Expected Lb child"),
    }
}

#[test]
fn refrain_roundtrip_with_lang() {
    use tusk_model::elements::Refrain;

    let mut original = Refrain::default();
    original.common.xml_id = Some("refrain-1".to_string());
    original.lang.xml_lang = Some("en".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Refrain::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.lang.xml_lang, Some("en".to_string()));
}

// ============================================================================
// Stack Tests
// ============================================================================

#[test]
fn stack_roundtrip_empty() {
    use tusk_model::elements::Stack;

    let original = Stack::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stack::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn stack_roundtrip_with_xml_id() {
    use tusk_model::elements::Stack;

    let mut original = Stack::default();
    original.common.xml_id = Some("stack-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stack::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("stack-1".to_string()));
}

#[test]
fn stack_roundtrip_with_text() {
    use tusk_model::elements::{Stack, StackChild};

    let mut original = Stack::default();
    original.common.xml_id = Some("stack-1".to_string());
    original
        .children
        .push(StackChild::Text("stacked text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stack::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StackChild::Text(t) => assert_eq!(t, "stacked text"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn stack_roundtrip_with_delim_and_align() {
    use tusk_model::elements::Stack;

    let mut original = Stack::default();
    original.common.xml_id = Some("stack-1".to_string());
    original.delim = Some("/".to_string());
    original.align = Some("center".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stack::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.delim, Some("/".to_string()));
    assert_eq!(parsed.align, Some("center".to_string()));
}

#[test]
fn stack_roundtrip_nested() {
    use tusk_model::elements::{Stack, StackChild};

    let mut original = Stack::default();
    original.common.xml_id = Some("stack-outer".to_string());

    let mut inner = Stack::default();
    inner.common.xml_id = Some("stack-inner".to_string());
    inner.children.push(StackChild::Text("inner".to_string()));
    original.children.push(StackChild::Stack(Box::new(inner)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stack::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StackChild::Stack(inner) => {
            assert_eq!(inner.common.xml_id, Some("stack-inner".to_string()));
        }
        _ => panic!("Expected nested Stack"),
    }
}

// ============================================================================
// Rubric Tests
// ============================================================================

#[test]
fn rubric_roundtrip_empty() {
    use tusk_model::elements::Rubric;

    let original = Rubric::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rubric::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn rubric_roundtrip_with_xml_id() {
    use tusk_model::elements::Rubric;

    let mut original = Rubric::default();
    original.common.xml_id = Some("rubric-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rubric::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("rubric-1".to_string()));
}

#[test]
fn rubric_roundtrip_with_text() {
    use tusk_model::elements::{Rubric, RubricChild};

    let mut original = Rubric::default();
    original.common.xml_id = Some("rubric-1".to_string());
    original
        .children
        .push(RubricChild::Text("Incipit liber primus".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rubric::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RubricChild::Text(t) => assert_eq!(t, "Incipit liber primus"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn rubric_roundtrip_with_func() {
    use tusk_model::elements::Rubric;

    let mut original = Rubric::default();
    original.common.xml_id = Some("rubric-1".to_string());
    original.func = Some("opening".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rubric::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.func, Some("opening".to_string()));
}

// ============================================================================
// Explicit Tests
// ============================================================================

#[test]
fn explicit_roundtrip_empty() {
    use tusk_model::elements::Explicit;

    let original = Explicit::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Explicit::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn explicit_roundtrip_with_xml_id() {
    use tusk_model::elements::Explicit;

    let mut original = Explicit::default();
    original.common.xml_id = Some("explicit-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Explicit::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("explicit-1".to_string()));
}

#[test]
fn explicit_roundtrip_with_text() {
    use tusk_model::elements::{Explicit, ExplicitChild};

    let mut original = Explicit::default();
    original.common.xml_id = Some("explicit-1".to_string());
    original
        .children
        .push(ExplicitChild::Text("Explicit liber primus".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Explicit::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ExplicitChild::Text(t) => assert_eq!(t, "Explicit liber primus"),
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// Byline Tests
// ============================================================================

#[test]
fn byline_roundtrip_empty() {
    use tusk_model::elements::Byline;

    let original = Byline::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Byline::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn byline_roundtrip_with_xml_id() {
    use tusk_model::elements::Byline;

    let mut original = Byline::default();
    original.common.xml_id = Some("byline-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Byline::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("byline-1".to_string()));
}

#[test]
fn byline_roundtrip_with_text() {
    use tusk_model::elements::{Byline, BylineChild};

    let mut original = Byline::default();
    original.common.xml_id = Some("byline-1".to_string());
    original
        .children
        .push(BylineChild::Text("by Johann Sebastian Bach".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Byline::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BylineChild::Text(t) => assert_eq!(t, "by Johann Sebastian Bach"),
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// Stamp Tests
// ============================================================================

#[test]
fn stamp_roundtrip_empty() {
    use tusk_model::elements::Stamp;

    let original = Stamp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stamp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn stamp_roundtrip_with_xml_id() {
    use tusk_model::elements::Stamp;

    let mut original = Stamp::default();
    original.common.xml_id = Some("stamp-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stamp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("stamp-1".to_string()));
}

#[test]
fn stamp_roundtrip_with_text() {
    use tusk_model::elements::{Stamp, StampChild};

    let mut original = Stamp::default();
    original.common.xml_id = Some("stamp-1".to_string());
    original
        .children
        .push(StampChild::Text("British Library".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stamp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StampChild::Text(t) => assert_eq!(t, "British Library"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn stamp_roundtrip_with_datable() {
    use tusk_model::elements::Stamp;

    let mut original = Stamp::default();
    original.common.xml_id = Some("stamp-1".to_string());
    original.datable.isodate = Some("1850-01-15".parse().unwrap());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stamp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.datable.isodate.is_some());
}

#[test]
fn stamp_roundtrip_nested() {
    use tusk_model::elements::{Stamp, StampChild};

    let mut original = Stamp::default();
    original.common.xml_id = Some("stamp-outer".to_string());

    let mut inner = Stamp::default();
    inner.common.xml_id = Some("stamp-inner".to_string());
    inner.children.push(StampChild::Text("inner".to_string()));
    original.children.push(StampChild::Stamp(Box::new(inner)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Stamp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StampChild::Stamp(inner) => {
            assert_eq!(inner.common.xml_id, Some("stamp-inner".to_string()));
        }
        _ => panic!("Expected nested Stamp"),
    }
}

// ============================================================================
// Cb (column beginning) Tests
// ============================================================================

#[test]
fn cb_roundtrip_empty() {
    use tusk_model::elements::Cb;

    let original = Cb::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cb::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n.is_none());
}

#[test]
fn cb_roundtrip_with_xml_id() {
    use tusk_model::elements::Cb;

    let mut original = Cb::default();
    original.basic.xml_id = Some("cb-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cb::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("cb-1".to_string()));
}

#[test]
fn cb_roundtrip_with_column_number() {
    use tusk_model::elements::Cb;

    let mut original = Cb::default();
    original.basic.xml_id = Some("cb-1".to_string());
    original.n = Some(2);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cb::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.n, Some(2));
}

#[test]
fn cb_roundtrip_with_label() {
    use tusk_model::elements::Cb;

    let mut original = Cb::default();
    original.basic.xml_id = Some("cb-1".to_string());
    original.labelled.label = Some("column A".to_string());
    original.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cb::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.labelled.label, Some("column A".to_string()));
    assert_eq!(parsed.n, Some(1));
}

// ============================================================================
// DivLine (division line in neumes) Tests
// ============================================================================

#[test]
fn div_line_roundtrip_empty() {
    use tusk_model::elements::DivLine;

    let original = DivLine::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = DivLine::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.div_line_log.form.is_empty());
}

#[test]
fn div_line_roundtrip_with_xml_id() {
    use tusk_model::elements::DivLine;

    let mut original = DivLine::default();
    original.basic.xml_id = Some("divLine-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = DivLine::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("divLine-1".to_string()));
}

#[test]
fn div_line_roundtrip_with_form() {
    use tusk_model::att::AttDivLineLogForm;
    use tusk_model::elements::DivLine;

    let mut original = DivLine::default();
    original.basic.xml_id = Some("divLine-1".to_string());
    original.div_line_log.form = vec![AttDivLineLogForm::Maior];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = DivLine::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.div_line_log.form.len(), 1);
    assert_eq!(parsed.div_line_log.form[0], AttDivLineLogForm::Maior);
}

#[test]
fn div_line_roundtrip_with_location() {
    use tusk_model::elements::DivLine;
    use tusk_model::generated::data::DataStaffloc;

    let mut original = DivLine::default();
    original.basic.xml_id = Some("divLine-1".to_string());
    original.staff_loc.loc = Some(DataStaffloc(2));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = DivLine::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.staff_loc.loc, Some(DataStaffloc(2)));
}

#[test]
fn div_line_roundtrip_with_visibility() {
    use tusk_model::elements::DivLine;
    use tusk_model::generated::data::DataBoolean;

    let mut original = DivLine::default();
    original.basic.xml_id = Some("divLine-1".to_string());
    original.visibility.visible = Some(DataBoolean::False);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = DivLine::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.visibility.visible, Some(DataBoolean::False));
}

// ============================================================================
// Curve (generic curved line) Tests
// ============================================================================

#[test]
fn curve_roundtrip_empty() {
    use tusk_model::elements::Curve;

    let original = Curve::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Curve::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn curve_roundtrip_with_xml_id() {
    use tusk_model::elements::Curve;

    let mut original = Curve::default();
    original.common.xml_id = Some("curve-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Curve::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("curve-1".to_string()));
}

#[test]
fn curve_roundtrip_with_endpoints() {
    use tusk_model::elements::Curve;
    use tusk_model::generated::data::DataUri;

    let mut original = Curve::default();
    original.common.xml_id = Some("curve-1".to_string());
    original.curve_log.startid = Some(DataUri("#note1".to_string()));
    original.curve_log.endid = Some(DataUri("#note2".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Curve::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.curve_log.startid,
        Some(DataUri("#note1".to_string()))
    );
    assert_eq!(parsed.curve_log.endid, Some(DataUri("#note2".to_string())));
}

#[test]
fn curve_roundtrip_with_curvedir() {
    use tusk_model::att::AttCurveVisCurvedir;
    use tusk_model::elements::Curve;

    let mut original = Curve::default();
    original.common.xml_id = Some("curve-1".to_string());
    original.curve_vis.curvedir = Some(AttCurveVisCurvedir::Above);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Curve::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.curve_vis.curvedir, Some(AttCurveVisCurvedir::Above));
}

#[test]
fn curve_roundtrip_with_coordinates() {
    use tusk_model::elements::Curve;

    let mut original = Curve::default();
    original.common.xml_id = Some("curve-1".to_string());
    original.curve_vis.x = Some(100.0);
    original.curve_vis.y = Some(50.0);
    original.curve_vis.x2 = Some(200.0);
    original.curve_vis.y2 = Some(75.0);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Curve::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.curve_vis.x, Some(100.0));
    assert_eq!(parsed.curve_vis.y, Some(50.0));
    assert_eq!(parsed.curve_vis.x2, Some(200.0));
    assert_eq!(parsed.curve_vis.y2, Some(75.0));
}

#[test]
fn curve_roundtrip_with_func() {
    use tusk_model::att::AttCurveLogFunc;
    use tusk_model::elements::Curve;

    let mut original = Curve::default();
    original.common.xml_id = Some("curve-1".to_string());
    original.curve_log.func = Some(AttCurveLogFunc::Unknown);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Curve::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.curve_log.func, Some(AttCurveLogFunc::Unknown));
}

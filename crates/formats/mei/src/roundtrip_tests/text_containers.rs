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

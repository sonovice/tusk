//! Round-trip serialization tests for header elements.
//!
//! Tests for MeiHead, FileDesc, TitleStmt, PubStmt, RevisionDesc, Title,
//! Date, P, Head elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Header Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_mei_head() {
    use tusk_model::elements::MeiHead;

    let original = MeiHead::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MeiHead::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_mei_head_with_xml_id() {
    use tusk_model::elements::MeiHead;

    let mut original = MeiHead::default();
    original.basic.xml_id = Some("header-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"header-1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = MeiHead::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.basic.xml_id, Some("header-1".to_string()));
}

#[test]
fn roundtrip_revision_desc_with_change() {
    use tusk_model::data::DataIsodate;
    use tusk_model::elements::{
        Change, ChangeChild, ChangeDesc, ChangeDescChild, P, PChild, RevisionDesc,
        RevisionDescChild,
    };

    let mut revision_desc = RevisionDesc::default();
    revision_desc.common.xml_id = Some("revdesc1".to_string());

    // Create a change element
    let mut change = Change::default();
    change.common.xml_id = Some("change1".to_string());
    change.datable.isodate = Some(DataIsodate("2025-01-15".to_string()));

    // Add changeDesc with a paragraph
    let mut change_desc = ChangeDesc::default();
    let mut p = P::default();
    p.children
        .push(PChild::Text("Initial encoding".to_string()));
    change_desc.children.push(ChangeDescChild::P(Box::new(p)));
    change
        .children
        .push(ChangeChild::ChangeDesc(Box::new(change_desc)));

    revision_desc
        .children
        .push(RevisionDescChild::Change(Box::new(change)));

    // Serialize and deserialize
    let xml = revision_desc.to_mei_string().expect("serialize");
    assert!(
        xml.contains("revisionDesc"),
        "should have revisionDesc: {}",
        xml
    );
    assert!(xml.contains("change"), "should have change: {}", xml);
    assert!(
        xml.contains("isodate=\"2025-01-15\""),
        "should have isodate: {}",
        xml
    );
    assert!(
        xml.contains("changeDesc"),
        "should have changeDesc: {}",
        xml
    );
    assert!(
        xml.contains("Initial encoding"),
        "should have text: {}",
        xml
    );

    let parsed = RevisionDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("revdesc1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let RevisionDescChild::Change(change) = &parsed.children[0] {
        assert_eq!(change.common.xml_id, Some("change1".to_string()));
        assert_eq!(
            change.datable.isodate,
            Some(DataIsodate("2025-01-15".to_string()))
        );
    } else {
        panic!("Expected Change child");
    }
}

#[test]
fn roundtrip_change_with_resp_stmt() {
    use tusk_model::data::DataIsodate;
    use tusk_model::elements::{
        Change, ChangeChild, ChangeDesc, ChangeDescChild, P, PChild, PersName, PersNameChild, Resp,
        RespChild, RespStmt, RespStmtChild, RevisionDesc, RevisionDescChild,
    };

    let mut revision_desc = RevisionDesc::default();
    revision_desc.common.xml_id = Some("revdesc1".to_string());

    // Create a change element with respStmt child
    let mut change = Change::default();
    change.common.xml_id = Some("change1".to_string());
    change.datable.isodate = Some(DataIsodate("2025-01-20".to_string()));

    // Add respStmt with resp and persName
    let mut resp_stmt = RespStmt::default();
    let mut resp = Resp::default();
    resp.children
        .push(RespChild::Text("Encoding by".to_string()));
    resp_stmt.children.push(RespStmtChild::Resp(Box::new(resp)));

    let mut pers_name = PersName::default();
    pers_name
        .children
        .push(PersNameChild::Text("John Doe".to_string()));
    resp_stmt
        .children
        .push(RespStmtChild::PersName(Box::new(pers_name)));

    change
        .children
        .push(ChangeChild::RespStmt(Box::new(resp_stmt)));

    // Add changeDesc with a paragraph
    let mut change_desc = ChangeDesc::default();
    let mut p = P::default();
    p.children
        .push(PChild::Text("Added new section".to_string()));
    change_desc.children.push(ChangeDescChild::P(Box::new(p)));
    change
        .children
        .push(ChangeChild::ChangeDesc(Box::new(change_desc)));

    revision_desc
        .children
        .push(RevisionDescChild::Change(Box::new(change)));

    // Serialize and deserialize
    let xml = revision_desc.to_mei_string().expect("serialize");
    assert!(xml.contains("respStmt"), "should have respStmt: {}", xml);
    assert!(xml.contains("resp"), "should have resp: {}", xml);
    assert!(
        xml.contains("Encoding by"),
        "should have resp text: {}",
        xml
    );
    assert!(xml.contains("persName"), "should have persName: {}", xml);
    assert!(
        xml.contains("John Doe"),
        "should have persName text: {}",
        xml
    );
    assert!(
        xml.contains("changeDesc"),
        "should have changeDesc: {}",
        xml
    );
    assert!(
        xml.contains("Added new section"),
        "should have changeDesc text: {}",
        xml
    );

    let parsed = RevisionDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 1);

    if let RevisionDescChild::Change(change) = &parsed.children[0] {
        // Should have 2 children: respStmt and changeDesc
        assert_eq!(change.children.len(), 2);

        // Check respStmt
        let has_resp_stmt = change
            .children
            .iter()
            .any(|c| matches!(c, ChangeChild::RespStmt(_)));
        assert!(has_resp_stmt, "should have RespStmt child");

        // Check changeDesc
        let has_change_desc = change
            .children
            .iter()
            .any(|c| matches!(c, ChangeChild::ChangeDesc(_)));
        assert!(has_change_desc, "should have ChangeDesc child");
    } else {
        panic!("Expected Change child");
    }
}

#[test]
fn serialize_title_with_text_content() {
    use tusk_model::elements::{Title, TitleChild};

    let mut title = Title::default();
    title.basic.xml_id = Some("title1".to_string());
    title
        .children
        .push(TitleChild::Text("Symphony No. 5".to_string()));

    let xml = title.to_mei_string().expect("serialize");
    assert!(xml.contains("<title"), "should have title: {}", xml);
    assert!(
        xml.contains("xml:id=\"title1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Symphony No. 5"), "should have text: {}", xml);
    assert!(xml.contains("</title>"), "should have closing tag: {}", xml);
}

#[test]
fn serialize_file_desc_structure() {
    use tusk_model::elements::{
        FileDesc, FileDescChild, PubStmt, Title, TitleChild, TitleStmt, TitleStmtChild,
    };

    let mut file_desc = FileDesc::default();
    file_desc.common.xml_id = Some("fd1".to_string());

    // Add titleStmt with title
    let mut title_stmt = TitleStmt::default();
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Test Work".to_string()));
    title_stmt
        .children
        .push(TitleStmtChild::Title(Box::new(title)));
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));

    // Add pubStmt
    let pub_stmt = PubStmt::default();
    file_desc
        .children
        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

    // Serialize
    let xml = file_desc.to_mei_string().expect("serialize");
    assert!(xml.contains("<fileDesc"), "should have fileDesc: {}", xml);
    assert!(
        xml.contains("xml:id=\"fd1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<titleStmt"), "should have titleStmt: {}", xml);
    assert!(xml.contains("<title"), "should have title: {}", xml);
    assert!(xml.contains("Test Work"), "should have title text: {}", xml);
    assert!(xml.contains("<pubStmt"), "should have pubStmt: {}", xml);
    assert!(
        xml.contains("</fileDesc>"),
        "should have closing tag: {}",
        xml
    );
}

#[test]
fn serialize_date_with_isodate() {
    use tusk_model::data::DataIsodate;
    use tusk_model::elements::{Date, DateChild};

    let mut date = Date::default();
    date.common.xml_id = Some("date1".to_string());
    date.datable.isodate = Some(DataIsodate("2025-06-15".to_string()));
    date.children
        .push(DateChild::Text("June 15, 2025".to_string()));

    let xml = date.to_mei_string().expect("serialize");
    assert!(xml.contains("<date"), "should have date: {}", xml);
    assert!(
        xml.contains("isodate=\"2025-06-15\""),
        "should have isodate: {}",
        xml
    );
    assert!(xml.contains("June 15, 2025"), "should have text: {}", xml);
    assert!(xml.contains("</date>"), "should have closing tag: {}", xml);
}

#[test]
fn serialize_p_with_text_content() {
    use tusk_model::elements::{P, PChild};

    let mut p = P::default();
    p.common.xml_id = Some("p1".to_string());
    p.children
        .push(PChild::Text("This is a paragraph.".to_string()));

    let xml = p.to_mei_string().expect("serialize");
    assert!(xml.contains("<p"), "should have p: {}", xml);
    assert!(xml.contains("xml:id=\"p1\""), "should have xml:id: {}", xml);
    assert!(
        xml.contains("This is a paragraph."),
        "should have text: {}",
        xml
    );
    assert!(xml.contains("</p>"), "should have closing tag: {}", xml);
}

#[test]
fn serialize_head_with_text() {
    use tusk_model::elements::{Head, HeadChild};

    let mut head = Head::default();
    head.common.xml_id = Some("head1".to_string());
    head.children
        .push(HeadChild::Text("Section Title".to_string()));

    let xml = head.to_mei_string().expect("serialize");
    assert!(xml.contains("<head"), "should have head: {}", xml);
    assert!(
        xml.contains("xml:id=\"head1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Section Title"), "should have text: {}", xml);
    assert!(xml.contains("</head>"), "should have closing tag: {}", xml);
}

#[test]
fn roundtrip_work_with_tempo_text() {
    use tusk_model::elements::{Tempo, TempoChild, Work, WorkChild};

    let mut work = Work::default();
    work.common.xml_id = Some("work1".to_string());

    // Add tempo with text content
    let mut tempo = Tempo::default();
    tempo
        .children
        .push(TempoChild::Text("undefined".to_string()));
    work.children.push(WorkChild::Tempo(Box::new(tempo)));

    // Serialize
    let xml = work.to_mei_string().expect("serialize");
    assert!(xml.contains("<work"), "should have work: {}", xml);
    assert!(xml.contains("<tempo>"), "should have tempo: {}", xml);
    assert!(
        xml.contains("undefined"),
        "should have tempo text 'undefined': {}",
        xml
    );
    assert!(xml.contains("</tempo>"), "should have closing tag: {}", xml);

    // Deserialize and verify
    let parsed = Work::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("work1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        WorkChild::Tempo(tempo) => {
            assert_eq!(tempo.children.len(), 1);
            match &tempo.children[0] {
                TempoChild::Text(text) => assert_eq!(text, "undefined"),
                other => panic!("Expected Text child, got {:?}", other),
            }
        }
        other => panic!("Expected Tempo child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_expression() {
    use tusk_model::elements::Expression;

    let original = Expression::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<expression"),
        "should have expression: {}",
        xml
    );

    let parsed = Expression::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_expression_with_attributes() {
    use tusk_model::elements::Expression;

    let mut expression = Expression::default();
    expression.common.xml_id = Some("expr1".to_string());
    expression.bibl.analog = Some("analog-value".to_string());

    let xml = expression.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"expr1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("analog=\"analog-value\""),
        "should have analog: {}",
        xml
    );

    let parsed = Expression::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("expr1".to_string()));
    assert_eq!(parsed.bibl.analog, Some("analog-value".to_string()));
}

#[test]
fn roundtrip_expression_with_title_child() {
    use tusk_model::elements::{Expression, ExpressionChild, Title, TitleChild};

    let mut expression = Expression::default();
    expression.common.xml_id = Some("expr1".to_string());

    // Add title child
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Test Expression".to_string()));
    expression
        .children
        .push(ExpressionChild::Title(Box::new(title)));

    let xml = expression.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<expression"),
        "should have expression: {}",
        xml
    );
    assert!(xml.contains("<title>"), "should have title: {}", xml);
    assert!(
        xml.contains("Test Expression"),
        "should have title text: {}",
        xml
    );
    assert!(
        xml.contains("</expression>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Expression::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("expr1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        ExpressionChild::Title(title) => {
            assert_eq!(title.children.len(), 1);
            match &title.children[0] {
                TitleChild::Text(text) => assert_eq!(text, "Test Expression"),
                other => panic!("Expected Text child, got {:?}", other),
            }
        }
        other => panic!("Expected Title child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_expression_list() {
    use tusk_model::elements::ExpressionList;

    let original = ExpressionList::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<expressionList"),
        "should have expressionList: {}",
        xml
    );

    let parsed = ExpressionList::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_expression_list_with_expressions() {
    use tusk_model::elements::{
        Expression, ExpressionChild, ExpressionList, ExpressionListChild, Title, TitleChild,
    };

    let mut expression_list = ExpressionList::default();
    expression_list.common.xml_id = Some("explist1".to_string());

    // Add first expression
    let mut expr1 = Expression::default();
    expr1.common.xml_id = Some("expr1".to_string());
    let mut title1 = Title::default();
    title1
        .children
        .push(TitleChild::Text("First Expression".to_string()));
    expr1
        .children
        .push(ExpressionChild::Title(Box::new(title1)));
    expression_list
        .children
        .push(ExpressionListChild::Expression(Box::new(expr1)));

    // Add second expression
    let mut expr2 = Expression::default();
    expr2.common.xml_id = Some("expr2".to_string());
    let mut title2 = Title::default();
    title2
        .children
        .push(TitleChild::Text("Second Expression".to_string()));
    expr2
        .children
        .push(ExpressionChild::Title(Box::new(title2)));
    expression_list
        .children
        .push(ExpressionListChild::Expression(Box::new(expr2)));

    let xml = expression_list.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<expressionList"),
        "should have expressionList: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"explist1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("First Expression"),
        "should have first expression text: {}",
        xml
    );
    assert!(
        xml.contains("Second Expression"),
        "should have second expression text: {}",
        xml
    );
    assert!(
        xml.contains("</expressionList>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = ExpressionList::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("explist1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    // Check first expression
    match &parsed.children[0] {
        ExpressionListChild::Expression(expr) => {
            assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
        }
        other => panic!("Expected Expression child, got {:?}", other),
    }

    // Check second expression
    match &parsed.children[1] {
        ExpressionListChild::Expression(expr) => {
            assert_eq!(expr.common.xml_id, Some("expr2".to_string()));
        }
        other => panic!("Expected Expression child, got {:?}", other),
    }
}

#[test]
fn roundtrip_work_with_expression_list() {
    use tusk_model::elements::{
        Expression, ExpressionChild, ExpressionList, ExpressionListChild, Title, TitleChild, Work,
        WorkChild,
    };

    let mut work = Work::default();
    work.common.xml_id = Some("work1".to_string());

    // Add expressionList with one expression
    let mut expression_list = ExpressionList::default();
    let mut expr = Expression::default();
    expr.common.xml_id = Some("expr1".to_string());
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Work Expression".to_string()));
    expr.children.push(ExpressionChild::Title(Box::new(title)));
    expression_list
        .children
        .push(ExpressionListChild::Expression(Box::new(expr)));
    work.children
        .push(WorkChild::ExpressionList(Box::new(expression_list)));

    let xml = work.to_mei_string().expect("serialize");
    assert!(xml.contains("<work"), "should have work: {}", xml);
    assert!(
        xml.contains("<expressionList>"),
        "should have expressionList: {}",
        xml
    );
    assert!(
        xml.contains("<expression"),
        "should have expression: {}",
        xml
    );
    assert!(
        xml.contains("Work Expression"),
        "should have title text: {}",
        xml
    );

    let parsed = Work::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("work1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        WorkChild::ExpressionList(expr_list) => {
            assert_eq!(expr_list.children.len(), 1);
            match &expr_list.children[0] {
                ExpressionListChild::Expression(expr) => {
                    assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
                }
                other => panic!("Expected Expression child, got {:?}", other),
            }
        }
        other => panic!("Expected ExpressionList child, got {:?}", other),
    }
}

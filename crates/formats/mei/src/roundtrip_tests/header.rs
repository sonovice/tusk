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

// ============================================================================
// Agent/Responsibility Element Tests
// ============================================================================

#[test]
fn roundtrip_creator_with_text() {
    use tusk_model::elements::{Creator, CreatorChild};

    let mut creator = Creator::default();
    creator.common.xml_id = Some("creator1".to_string());
    creator
        .children
        .push(CreatorChild::Text("Johann Sebastian Bach".to_string()));

    let xml = creator.to_mei_string().expect("serialize");
    assert!(xml.contains("<creator"), "should have creator: {}", xml);
    assert!(
        xml.contains("xml:id=\"creator1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("Johann Sebastian Bach"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</creator>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Creator::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("creator1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        CreatorChild::Text(text) => assert_eq!(text, "Johann Sebastian Bach"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_creator_with_pers_name() {
    use tusk_model::elements::{Creator, CreatorChild, PersName, PersNameChild};

    let mut creator = Creator::default();
    creator.common.xml_id = Some("creator1".to_string());

    let mut pers_name = PersName::default();
    pers_name.common.xml_id = Some("pn1".to_string());
    pers_name
        .children
        .push(PersNameChild::Text("Ludwig van Beethoven".to_string()));
    creator
        .children
        .push(CreatorChild::PersName(Box::new(pers_name)));

    let xml = creator.to_mei_string().expect("serialize");
    assert!(xml.contains("<creator"), "should have creator: {}", xml);
    assert!(xml.contains("<persName"), "should have persName: {}", xml);
    assert!(
        xml.contains("Ludwig van Beethoven"),
        "should have text: {}",
        xml
    );

    let parsed = Creator::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("creator1".to_string()));
    // Note: With the current simplified deserializer, the nested persName
    // is read as text. This test documents the current behavior.
    assert!(!parsed.children.is_empty());
}

#[test]
fn roundtrip_editor_with_text() {
    use tusk_model::elements::{Editor, EditorChild};

    let mut editor = Editor::default();
    editor.common.xml_id = Some("editor1".to_string());
    editor
        .children
        .push(EditorChild::Text("Klaus Döge".to_string()));

    let xml = editor.to_mei_string().expect("serialize");
    assert!(xml.contains("<editor"), "should have editor: {}", xml);
    assert!(
        xml.contains("xml:id=\"editor1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Klaus Döge"), "should have text: {}", xml);
    assert!(
        xml.contains("</editor>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Editor::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("editor1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        EditorChild::Text(text) => assert_eq!(text, "Klaus Döge"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_editor_with_pers_name() {
    use tusk_model::elements::{Editor, EditorChild, PersName, PersNameChild};

    let mut editor = Editor::default();
    editor.common.xml_id = Some("editor1".to_string());

    let mut pers_name = PersName::default();
    pers_name.common.xml_id = Some("pn1".to_string());
    pers_name
        .children
        .push(PersNameChild::Text("John Smith".to_string()));
    editor
        .children
        .push(EditorChild::PersName(Box::new(pers_name)));

    let xml = editor.to_mei_string().expect("serialize");
    assert!(xml.contains("<editor"), "should have editor: {}", xml);
    assert!(xml.contains("<persName"), "should have persName: {}", xml);
    assert!(
        xml.contains("xml:id=\"pn1\""),
        "should have persName xml:id: {}",
        xml
    );
    assert!(xml.contains("John Smith"), "should have text: {}", xml);

    let parsed = Editor::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("editor1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        EditorChild::PersName(pers) => {
            assert_eq!(pers.common.xml_id, Some("pn1".to_string()));
        }
        other => panic!("Expected PersName child, got {:?}", other),
    }
}

#[test]
fn roundtrip_funder_with_text() {
    use tusk_model::elements::{Funder, FunderChild};

    let mut funder = Funder::default();
    funder.common.xml_id = Some("funder1".to_string());
    funder
        .children
        .push(FunderChild::Text("National Science Foundation".to_string()));

    let xml = funder.to_mei_string().expect("serialize");
    assert!(xml.contains("<funder"), "should have funder: {}", xml);
    assert!(
        xml.contains("xml:id=\"funder1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("National Science Foundation"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</funder>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Funder::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("funder1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FunderChild::Text(text) => assert_eq!(text, "National Science Foundation"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_funder_with_corp_name() {
    use tusk_model::elements::{CorpName, CorpNameChild, Funder, FunderChild};

    let mut funder = Funder::default();
    funder.common.xml_id = Some("funder1".to_string());

    let mut corp_name = CorpName::default();
    corp_name.common.xml_id = Some("cn1".to_string());
    corp_name.children.push(CorpNameChild::Text(
        "Deutsche Forschungsgemeinschaft".to_string(),
    ));
    funder
        .children
        .push(FunderChild::CorpName(Box::new(corp_name)));

    let xml = funder.to_mei_string().expect("serialize");
    assert!(xml.contains("<funder"), "should have funder: {}", xml);
    assert!(xml.contains("<corpName"), "should have corpName: {}", xml);
    assert!(
        xml.contains("Deutsche Forschungsgemeinschaft"),
        "should have text: {}",
        xml
    );

    let parsed = Funder::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("funder1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FunderChild::CorpName(corp) => {
            assert_eq!(corp.common.xml_id, Some("cn1".to_string()));
        }
        other => panic!("Expected CorpName child, got {:?}", other),
    }
}

#[test]
fn roundtrip_sponsor_with_text() {
    use tusk_model::elements::{Sponsor, SponsorChild};

    let mut sponsor = Sponsor::default();
    sponsor.common.xml_id = Some("sponsor1".to_string());
    sponsor
        .children
        .push(SponsorChild::Text("University of Vienna".to_string()));

    let xml = sponsor.to_mei_string().expect("serialize");
    assert!(xml.contains("<sponsor"), "should have sponsor: {}", xml);
    assert!(
        xml.contains("xml:id=\"sponsor1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("University of Vienna"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</sponsor>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Sponsor::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sponsor1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SponsorChild::Text(text) => assert_eq!(text, "University of Vienna"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_contributor_with_text() {
    use tusk_model::elements::{Contributor, ContributorChild};

    let mut contributor = Contributor::default();
    contributor.common.xml_id = Some("contrib1".to_string());
    contributor
        .children
        .push(ContributorChild::Text("Jane Doe".to_string()));

    let xml = contributor.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<contributor"),
        "should have contributor: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"contrib1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Jane Doe"), "should have text: {}", xml);
    assert!(
        xml.contains("</contributor>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Contributor::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("contrib1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ContributorChild::Text(text) => assert_eq!(text, "Jane Doe"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_resp_with_text() {
    use tusk_model::elements::{Resp, RespChild};

    let mut resp = Resp::default();
    resp.common.xml_id = Some("resp1".to_string());
    resp.children
        .push(RespChild::Text("Transcription".to_string()));

    let xml = resp.to_mei_string().expect("serialize");
    assert!(xml.contains("<resp"), "should have resp: {}", xml);
    assert!(
        xml.contains("xml:id=\"resp1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Transcription"), "should have text: {}", xml);
    assert!(xml.contains("</resp>"), "should have closing tag: {}", xml);

    let parsed = Resp::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("resp1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RespChild::Text(text) => assert_eq!(text, "Transcription"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_resp_stmt_with_resp_and_name() {
    use tusk_model::elements::{Name, NameChild, Resp, RespChild, RespStmt, RespStmtChild};

    let mut resp_stmt = RespStmt::default();
    resp_stmt.common.xml_id = Some("rs1".to_string());

    let mut resp = Resp::default();
    resp.children
        .push(RespChild::Text("Encoding by".to_string()));
    resp_stmt.children.push(RespStmtChild::Resp(Box::new(resp)));

    let mut name = Name::default();
    name.children.push(NameChild::Text("John Doe".to_string()));
    resp_stmt.children.push(RespStmtChild::Name(Box::new(name)));

    let xml = resp_stmt.to_mei_string().expect("serialize");
    assert!(xml.contains("<respStmt"), "should have respStmt: {}", xml);
    assert!(
        xml.contains("xml:id=\"rs1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<resp>"), "should have resp: {}", xml);
    assert!(
        xml.contains("Encoding by"),
        "should have resp text: {}",
        xml
    );
    assert!(xml.contains("<name>"), "should have name: {}", xml);
    assert!(xml.contains("John Doe"), "should have name text: {}", xml);
    assert!(
        xml.contains("</respStmt>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = RespStmt::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("rs1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    let has_resp = parsed
        .children
        .iter()
        .any(|c| matches!(c, RespStmtChild::Resp(_)));
    assert!(has_resp, "should have Resp child");

    let has_name = parsed
        .children
        .iter()
        .any(|c| matches!(c, RespStmtChild::Name(_)));
    assert!(has_name, "should have Name child");
}

// ============================================================================
// Bibliographic Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_bibl_scope() {
    use tusk_model::elements::BiblScope;

    let original = BiblScope::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("<biblScope"), "should have biblScope: {}", xml);

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_bibl_scope_with_text() {
    use tusk_model::elements::{BiblScope, BiblScopeChild};

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs1".to_string());
    bibl_scope
        .children
        .push(BiblScopeChild::Text("pp. 100-150".to_string()));

    let xml = bibl_scope.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"bs1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("pp. 100-150"), "should have text: {}", xml);
    assert!(
        xml.contains("</biblScope>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bs1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BiblScopeChild::Text(text) => assert_eq!(text, "pp. 100-150"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_bibl_scope_with_from_to_attributes() {
    use tusk_model::elements::{BiblScope, BiblScopeChild};

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs2".to_string());
    bibl_scope.from = Some("1".to_string());
    bibl_scope.to = Some("10".to_string());
    bibl_scope
        .children
        .push(BiblScopeChild::Text("pages 1-10".to_string()));

    let xml = bibl_scope.to_mei_string().expect("serialize");
    assert!(xml.contains("from=\"1\""), "should have from attr: {}", xml);
    assert!(xml.contains("to=\"10\""), "should have to attr: {}", xml);
    assert!(xml.contains("pages 1-10"), "should have text: {}", xml);

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.from, Some("1".to_string()));
    assert_eq!(parsed.to, Some("10".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_bibl_scope_with_title_child() {
    use tusk_model::elements::{BiblScope, BiblScopeChild, Title, TitleChild};

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs3".to_string());

    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Chapter 1".to_string()));
    bibl_scope
        .children
        .push(BiblScopeChild::Title(Box::new(title)));

    let xml = bibl_scope.to_mei_string().expect("serialize");
    assert!(xml.contains("<biblScope"), "should have biblScope: {}", xml);
    assert!(xml.contains("<title>"), "should have title: {}", xml);
    assert!(xml.contains("Chapter 1"), "should have title text: {}", xml);

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bs3".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BiblScopeChild::Title(title) => {
            assert_eq!(title.children.len(), 1);
            match &title.children[0] {
                TitleChild::Text(text) => assert_eq!(text, "Chapter 1"),
                other => panic!("Expected Text child, got {:?}", other),
            }
        }
        other => panic!("Expected Title child, got {:?}", other),
    }
}

#[test]
fn roundtrip_series_stmt_with_bibl_scope() {
    use tusk_model::elements::{BiblScope, BiblScopeChild, SeriesStmt, SeriesStmtChild};

    let mut series_stmt = SeriesStmt::default();
    series_stmt.common.xml_id = Some("ss1".to_string());

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs1".to_string());
    bibl_scope
        .children
        .push(BiblScopeChild::Text("Volume 3".to_string()));
    series_stmt
        .children
        .push(SeriesStmtChild::BiblScope(Box::new(bibl_scope)));

    let xml = series_stmt.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<seriesStmt"),
        "should have seriesStmt: {}",
        xml
    );
    assert!(xml.contains("<biblScope"), "should have biblScope: {}", xml);
    assert!(xml.contains("Volume 3"), "should have text: {}", xml);
    assert!(
        xml.contains("</seriesStmt>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = SeriesStmt::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("ss1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SeriesStmtChild::BiblScope(bs) => {
            assert_eq!(bs.common.xml_id, Some("bs1".to_string()));
            assert_eq!(bs.children.len(), 1);
        }
        other => panic!("Expected BiblScope child, got {:?}", other),
    }
}

#[test]
fn roundtrip_bibl_with_bibl_scope() {
    use tusk_model::elements::{Bibl, BiblChild, BiblScope, BiblScopeChild};

    let mut bibl = Bibl::default();
    bibl.common.xml_id = Some("bibl1".to_string());

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs1".to_string());
    bibl_scope
        .children
        .push(BiblScopeChild::Text("pp. 25-30".to_string()));
    bibl.children
        .push(BiblChild::BiblScope(Box::new(bibl_scope)));

    let xml = bibl.to_mei_string().expect("serialize");
    assert!(xml.contains("<bibl"), "should have bibl: {}", xml);
    assert!(xml.contains("<biblScope"), "should have biblScope: {}", xml);
    assert!(xml.contains("pp. 25-30"), "should have text: {}", xml);

    let parsed = Bibl::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bibl1".to_string()));

    let has_bibl_scope = parsed
        .children
        .iter()
        .any(|c| matches!(c, BiblChild::BiblScope(_)));
    assert!(has_bibl_scope, "should have BiblScope child");
}

// ============================================================================
// Encoding Description Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_class_decls() {
    use tusk_model::elements::ClassDecls;

    let original = ClassDecls::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<classDecls"),
        "should have classDecls: {}",
        xml
    );

    let parsed = ClassDecls::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_class_decls_with_taxonomy() {
    use tusk_model::elements::{ClassDecls, ClassDeclsChild, Taxonomy};

    let mut class_decls = ClassDecls::default();
    class_decls.common.xml_id = Some("cd1".to_string());

    let mut taxonomy = Taxonomy::default();
    taxonomy.common.xml_id = Some("tax1".to_string());
    class_decls
        .children
        .push(ClassDeclsChild::Taxonomy(Box::new(taxonomy)));

    let xml = class_decls.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"cd1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<taxonomy"), "should have taxonomy: {}", xml);
    assert!(
        xml.contains("xml:id=\"tax1\""),
        "should have taxonomy xml:id: {}",
        xml
    );
    assert!(
        xml.contains("</classDecls>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = ClassDecls::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("cd1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ClassDeclsChild::Taxonomy(tax) => {
            assert_eq!(tax.common.xml_id, Some("tax1".to_string()));
        }
        other => panic!("Expected Taxonomy child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_taxonomy() {
    use tusk_model::elements::Taxonomy;

    let original = Taxonomy::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("<taxonomy"), "should have taxonomy: {}", xml);

    let parsed = Taxonomy::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_taxonomy_with_category() {
    use tusk_model::elements::{Category, Taxonomy, TaxonomyChild};

    let mut taxonomy = Taxonomy::default();
    taxonomy.common.xml_id = Some("tax1".to_string());

    let mut category = Category::default();
    category.common.xml_id = Some("cat1".to_string());
    taxonomy
        .children
        .push(TaxonomyChild::Category(Box::new(category)));

    let xml = taxonomy.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"tax1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<category"), "should have category: {}", xml);
    assert!(
        xml.contains("xml:id=\"cat1\""),
        "should have category xml:id: {}",
        xml
    );
    assert!(
        xml.contains("</taxonomy>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Taxonomy::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("tax1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TaxonomyChild::Category(cat) => {
            assert_eq!(cat.common.xml_id, Some("cat1".to_string()));
        }
        other => panic!("Expected Category child, got {:?}", other),
    }
}

#[test]
fn roundtrip_nested_taxonomy() {
    use tusk_model::elements::{Taxonomy, TaxonomyChild};

    let mut taxonomy = Taxonomy::default();
    taxonomy.common.xml_id = Some("tax1".to_string());

    let mut nested_taxonomy = Taxonomy::default();
    nested_taxonomy.common.xml_id = Some("tax2".to_string());
    taxonomy
        .children
        .push(TaxonomyChild::Taxonomy(Box::new(nested_taxonomy)));

    let xml = taxonomy.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"tax1\""),
        "should have outer xml:id: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"tax2\""),
        "should have nested xml:id: {}",
        xml
    );

    let parsed = Taxonomy::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("tax1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TaxonomyChild::Taxonomy(nested) => {
            assert_eq!(nested.common.xml_id, Some("tax2".to_string()));
        }
        other => panic!("Expected Taxonomy child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_category() {
    use tusk_model::elements::Category;

    let original = Category::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("<category"), "should have category: {}", xml);

    let parsed = Category::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_nested_category() {
    use tusk_model::elements::{Category, CategoryChild};

    let mut category = Category::default();
    category.common.xml_id = Some("cat1".to_string());

    let mut nested_category = Category::default();
    nested_category.common.xml_id = Some("cat2".to_string());
    category
        .children
        .push(CategoryChild::Category(Box::new(nested_category)));

    let xml = category.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"cat1\""),
        "should have outer xml:id: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"cat2\""),
        "should have nested xml:id: {}",
        xml
    );

    let parsed = Category::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("cat1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        CategoryChild::Category(nested) => {
            assert_eq!(nested.common.xml_id, Some("cat2".to_string()));
        }
        other => panic!("Expected Category child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_interpretation() {
    use tusk_model::elements::Interpretation;

    let original = Interpretation::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<interpretation"),
        "should have interpretation: {}",
        xml
    );

    let parsed = Interpretation::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_interpretation_with_p() {
    use tusk_model::elements::{Interpretation, InterpretationChild, P, PChild};

    let mut interpretation = Interpretation::default();
    interpretation.common.xml_id = Some("interp1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text(
        "Analysis methodology described here.".to_string(),
    ));
    interpretation
        .children
        .push(InterpretationChild::P(Box::new(p)));

    let xml = interpretation.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"interp1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("Analysis methodology"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</interpretation>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Interpretation::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("interp1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        InterpretationChild::P(_) => {}
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_normalization() {
    use tusk_model::elements::Normalization;

    let original = Normalization::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<normalization"),
        "should have normalization: {}",
        xml
    );

    let parsed = Normalization::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_normalization_with_p() {
    use tusk_model::elements::{Normalization, NormalizationChild, P, PChild};

    let mut normalization = Normalization::default();
    normalization.common.xml_id = Some("norm1".to_string());

    let mut p = P::default();
    p.children
        .push(PChild::Text("Spelling has been modernized.".to_string()));
    normalization
        .children
        .push(NormalizationChild::P(Box::new(p)));

    let xml = normalization.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"norm1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("Spelling has been modernized"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</normalization>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Normalization::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("norm1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        NormalizationChild::P(_) => {}
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_correction() {
    use tusk_model::elements::Correction;

    let original = Correction::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<correction"),
        "should have correction: {}",
        xml
    );

    let parsed = Correction::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_correction_with_p() {
    use tusk_model::elements::{Correction, CorrectionChild, P, PChild};

    let mut correction = Correction::default();
    correction.common.xml_id = Some("corr1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text(
        "Obvious errors have been silently corrected.".to_string(),
    ));
    correction.children.push(CorrectionChild::P(Box::new(p)));

    let xml = correction.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"corr1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("Obvious errors have been silently corrected"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</correction>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Correction::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("corr1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        CorrectionChild::P(_) => {}
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_segmentation() {
    use tusk_model::elements::Segmentation;

    let original = Segmentation::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<segmentation"),
        "should have segmentation: {}",
        xml
    );

    let parsed = Segmentation::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_segmentation_with_p() {
    use tusk_model::elements::{P, PChild, Segmentation, SegmentationChild};

    let mut segmentation = Segmentation::default();
    segmentation.common.xml_id = Some("seg1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text(
        "The text has been segmented by movement.".to_string(),
    ));
    segmentation
        .children
        .push(SegmentationChild::P(Box::new(p)));

    let xml = segmentation.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"seg1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("The text has been segmented by movement"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</segmentation>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Segmentation::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("seg1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SegmentationChild::P(_) => {}
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_sampling_decl() {
    use tusk_model::elements::SamplingDecl;

    let original = SamplingDecl::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<samplingDecl"),
        "should have samplingDecl: {}",
        xml
    );

    let parsed = SamplingDecl::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_sampling_decl_with_p() {
    use tusk_model::elements::{P, PChild, SamplingDecl, SamplingDeclChild};

    let mut sampling_decl = SamplingDecl::default();
    sampling_decl.common.xml_id = Some("samp1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text(
        "Only the first movement has been encoded.".to_string(),
    ));
    sampling_decl
        .children
        .push(SamplingDeclChild::P(Box::new(p)));

    let xml = sampling_decl.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"samp1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("Only the first movement has been encoded"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</samplingDecl>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = SamplingDecl::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("samp1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SamplingDeclChild::P(_) => {}
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_empty_std_vals() {
    use tusk_model::elements::StdVals;

    let original = StdVals::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("<stdVals"), "should have stdVals: {}", xml);

    let parsed = StdVals::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_std_vals_with_p() {
    use tusk_model::elements::{P, PChild, StdVals, StdValsChild};

    let mut std_vals = StdVals::default();
    std_vals.common.xml_id = Some("sv1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text(
        "Dates are given in ISO 8601 format.".to_string(),
    ));
    std_vals.children.push(StdValsChild::P(Box::new(p)));

    let xml = std_vals.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"sv1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("Dates are given in ISO 8601 format"),
        "should have text: {}",
        xml
    );
    assert!(
        xml.contains("</stdVals>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = StdVals::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sv1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StdValsChild::P(_) => {}
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_editorial_decl_with_encoding_elements() {
    use tusk_model::elements::{
        Correction, CorrectionChild, EditorialDecl, EditorialDeclChild, Interpretation,
        InterpretationChild, Normalization, NormalizationChild, P, PChild, Segmentation,
        SegmentationChild, StdVals, StdValsChild,
    };

    let mut editorial_decl = EditorialDecl::default();
    editorial_decl.common.xml_id = Some("ed1".to_string());

    // Add interpretation
    let mut interpretation = Interpretation::default();
    let mut p1 = P::default();
    p1.children.push(PChild::Text("Analysis info".to_string()));
    interpretation
        .children
        .push(InterpretationChild::P(Box::new(p1)));
    editorial_decl
        .children
        .push(EditorialDeclChild::Interpretation(Box::new(interpretation)));

    // Add normalization
    let mut normalization = Normalization::default();
    let mut p2 = P::default();
    p2.children
        .push(PChild::Text("Normalization info".to_string()));
    normalization
        .children
        .push(NormalizationChild::P(Box::new(p2)));
    editorial_decl
        .children
        .push(EditorialDeclChild::Normalization(Box::new(normalization)));

    // Add correction
    let mut correction = Correction::default();
    let mut p3 = P::default();
    p3.children
        .push(PChild::Text("Correction info".to_string()));
    correction.children.push(CorrectionChild::P(Box::new(p3)));
    editorial_decl
        .children
        .push(EditorialDeclChild::Correction(Box::new(correction)));

    // Add segmentation
    let mut segmentation = Segmentation::default();
    let mut p4 = P::default();
    p4.children
        .push(PChild::Text("Segmentation info".to_string()));
    segmentation
        .children
        .push(SegmentationChild::P(Box::new(p4)));
    editorial_decl
        .children
        .push(EditorialDeclChild::Segmentation(Box::new(segmentation)));

    // Add stdVals
    let mut std_vals = StdVals::default();
    let mut p5 = P::default();
    p5.children.push(PChild::Text("StdVals info".to_string()));
    std_vals.children.push(StdValsChild::P(Box::new(p5)));
    editorial_decl
        .children
        .push(EditorialDeclChild::StdVals(Box::new(std_vals)));

    let xml = editorial_decl.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<editorialDecl"),
        "should have editorialDecl: {}",
        xml
    );
    assert!(
        xml.contains("<interpretation>"),
        "should have interpretation: {}",
        xml
    );
    assert!(
        xml.contains("<normalization>"),
        "should have normalization: {}",
        xml
    );
    assert!(
        xml.contains("<correction>"),
        "should have correction: {}",
        xml
    );
    assert!(
        xml.contains("<segmentation>"),
        "should have segmentation: {}",
        xml
    );
    assert!(xml.contains("<stdVals>"), "should have stdVals: {}", xml);

    let parsed = EditorialDecl::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("ed1".to_string()));
    assert_eq!(parsed.children.len(), 5);

    // Verify each child type is present
    let has_interpretation = parsed
        .children
        .iter()
        .any(|c| matches!(c, EditorialDeclChild::Interpretation(_)));
    assert!(has_interpretation, "should have Interpretation child");

    let has_normalization = parsed
        .children
        .iter()
        .any(|c| matches!(c, EditorialDeclChild::Normalization(_)));
    assert!(has_normalization, "should have Normalization child");

    let has_correction = parsed
        .children
        .iter()
        .any(|c| matches!(c, EditorialDeclChild::Correction(_)));
    assert!(has_correction, "should have Correction child");

    let has_segmentation = parsed
        .children
        .iter()
        .any(|c| matches!(c, EditorialDeclChild::Segmentation(_)));
    assert!(has_segmentation, "should have Segmentation child");

    let has_std_vals = parsed
        .children
        .iter()
        .any(|c| matches!(c, EditorialDeclChild::StdVals(_)));
    assert!(has_std_vals, "should have StdVals child");
}

#[test]
fn roundtrip_encoding_desc_with_class_decls() {
    use tusk_model::elements::{ClassDecls, EncodingDesc, EncodingDescChild};

    let mut encoding_desc = EncodingDesc::default();
    encoding_desc.common.xml_id = Some("enc1".to_string());

    let mut class_decls = ClassDecls::default();
    class_decls.common.xml_id = Some("cd1".to_string());
    encoding_desc
        .children
        .push(EncodingDescChild::ClassDecls(Box::new(class_decls)));

    let xml = encoding_desc.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<encodingDesc"),
        "should have encodingDesc: {}",
        xml
    );
    assert!(
        xml.contains("<classDecls"),
        "should have classDecls: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"cd1\""),
        "should have classDecls xml:id: {}",
        xml
    );
    assert!(
        xml.contains("</encodingDesc>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = EncodingDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("enc1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        EncodingDescChild::ClassDecls(cd) => {
            assert_eq!(cd.common.xml_id, Some("cd1".to_string()));
        }
        other => panic!("Expected ClassDecls child, got {:?}", other),
    }
}

#[test]
fn roundtrip_encoding_desc_with_sampling_decl() {
    use tusk_model::elements::{EncodingDesc, EncodingDescChild, SamplingDecl};

    let mut encoding_desc = EncodingDesc::default();
    encoding_desc.common.xml_id = Some("enc1".to_string());

    let mut sampling_decl = SamplingDecl::default();
    sampling_decl.common.xml_id = Some("samp1".to_string());
    encoding_desc
        .children
        .push(EncodingDescChild::SamplingDecl(Box::new(sampling_decl)));

    let xml = encoding_desc.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<encodingDesc"),
        "should have encodingDesc: {}",
        xml
    );
    assert!(
        xml.contains("<samplingDecl"),
        "should have samplingDecl: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"samp1\""),
        "should have samplingDecl xml:id: {}",
        xml
    );

    let parsed = EncodingDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("enc1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        EncodingDescChild::SamplingDecl(sd) => {
            assert_eq!(sd.common.xml_id, Some("samp1".to_string()));
        }
        other => panic!("Expected SamplingDecl child, got {:?}", other),
    }
}

// ============================================================================
// Work Metadata Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_key_element_empty() {
    use tusk_model::elements::Key;

    let original = Key::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("<key"), "should have key: {}", xml);

    let parsed = Key::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_key_element_with_attributes() {
    use tusk_model::data::{DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::{Key, KeyChild};

    let mut key = Key::default();
    key.common.xml_id = Some("key1".to_string());
    key.pitch.pname = Some(DataPitchname("c".to_string()));
    key.key_mode.mode = Some(DataMode::DataModeCmn(DataModeCmn::Major));
    key.children.push(KeyChild::Text("C major".to_string()));

    let xml = key.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"key1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("pname=\"c\""), "should have pname: {}", xml);
    assert!(xml.contains("mode=\"major\""), "should have mode: {}", xml);
    assert!(xml.contains("C major"), "should have text: {}", xml);

    let parsed = Key::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("key1".to_string()));
    assert_eq!(parsed.pitch.pname, Some(DataPitchname("c".to_string())));
    assert_eq!(
        parsed.key_mode.mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_meter_element() {
    use tusk_model::elements::{Meter, MeterChild};

    let mut meter = Meter::default();
    meter.common.xml_id = Some("meter1".to_string());
    meter.meter_sig_log.count = Some("4".to_string());
    meter.meter_sig_log.unit = Some(4.0);
    meter.children.push(MeterChild::Text("4/4".to_string()));

    let xml = meter.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"meter1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("count=\"4\""), "should have count: {}", xml);
    assert!(xml.contains("unit=\"4\""), "should have unit: {}", xml);
    assert!(xml.contains("4/4"), "should have text: {}", xml);

    let parsed = Meter::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("meter1".to_string()));
    assert_eq!(parsed.meter_sig_log.count, Some("4".to_string()));
    assert_eq!(parsed.meter_sig_log.unit, Some(4.0));
}

#[test]
fn roundtrip_creation_element() {
    use tusk_model::elements::{Creation, CreationChild};

    let mut creation = Creation::default();
    creation.common.xml_id = Some("creation1".to_string());
    creation
        .children
        .push(CreationChild::Text("Composed in Vienna, 1800".to_string()));

    let xml = creation.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"creation1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("Composed in Vienna, 1800"),
        "should have text: {}",
        xml
    );

    let parsed = Creation::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("creation1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        CreationChild::Text(text) => assert_eq!(text, "Composed in Vienna, 1800"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_incip_element() {
    use tusk_model::elements::{Incip, IncipChild, Key};

    let mut incip = Incip::default();
    incip.common.xml_id = Some("incip1".to_string());

    // Add a key child
    let mut key = Key::default();
    key.common.xml_id = Some("incip-key".to_string());
    incip.children.push(IncipChild::Key(Box::new(key)));

    let xml = incip.to_mei_string().expect("serialize");
    assert!(xml.contains("<incip"), "should have incip: {}", xml);
    assert!(
        xml.contains("xml:id=\"incip1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<key"), "should have key child: {}", xml);

    let parsed = Incip::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("incip1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_incip_code_element() {
    use tusk_model::elements::{IncipCode, IncipCodeChild};

    let mut incip_code = IncipCode::default();
    incip_code.common.xml_id = Some("incipc1".to_string());
    incip_code.form = Some("plaineAndEasie".to_string());
    incip_code
        .children
        .push(IncipCodeChild::Text("4G-4G-4G/8E".to_string()));

    let xml = incip_code.to_mei_string().expect("serialize");
    assert!(xml.contains("<incipCode"), "should have incipCode: {}", xml);
    assert!(
        xml.contains("form=\"plaineAndEasie\""),
        "should have form: {}",
        xml
    );
    assert!(xml.contains("4G-4G-4G/8E"), "should have code: {}", xml);

    let parsed = IncipCode::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("incipc1".to_string()));
    assert_eq!(parsed.form, Some("plaineAndEasie".to_string()));
}

#[test]
fn roundtrip_incip_text_element() {
    use tusk_model::elements::{IncipText, IncipTextChild, Lg};

    let mut incip_text = IncipText::default();
    incip_text.common.xml_id = Some("incipt1".to_string());

    let lg = Lg::default();
    incip_text.children.push(IncipTextChild::Lg(Box::new(lg)));

    let xml = incip_text.to_mei_string().expect("serialize");
    assert!(xml.contains("<incipText"), "should have incipText: {}", xml);
    assert!(
        xml.contains("xml:id=\"incipt1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<lg"), "should have lg child: {}", xml);

    let parsed = IncipText::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("incipt1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_perf_medium_element() {
    use tusk_model::elements::{PerfMedium, PerfMediumChild, PerfResList};

    let mut perf_medium = PerfMedium::default();
    perf_medium.common.xml_id = Some("pm1".to_string());

    let perf_res_list = PerfResList::default();
    perf_medium
        .children
        .push(PerfMediumChild::PerfResList(Box::new(perf_res_list)));

    let xml = perf_medium.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<perfMedium"),
        "should have perfMedium: {}",
        xml
    );
    assert!(
        xml.contains("<perfResList"),
        "should have perfResList child: {}",
        xml
    );

    let parsed = PerfMedium::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("pm1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_perf_res_list_element() {
    use tusk_model::elements::{PerfRes, PerfResChild, PerfResList, PerfResListChild};

    let mut perf_res_list = PerfResList::default();
    perf_res_list.common.xml_id = Some("prl1".to_string());

    let mut perf_res = PerfRes::default();
    perf_res.common.xml_id = Some("pr1".to_string());
    perf_res
        .children
        .push(PerfResChild::Text("Violin".to_string()));
    perf_res_list
        .children
        .push(PerfResListChild::PerfRes(Box::new(perf_res)));

    let xml = perf_res_list.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<perfResList"),
        "should have perfResList: {}",
        xml
    );
    assert!(
        xml.contains("<perfRes"),
        "should have perfRes child: {}",
        xml
    );
    assert!(xml.contains("Violin"), "should have text: {}", xml);

    let parsed = PerfResList::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("prl1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_perf_res_element() {
    use tusk_model::elements::{PerfRes, PerfResChild};

    let mut perf_res = PerfRes::default();
    perf_res.common.xml_id = Some("pr1".to_string());
    perf_res.perf_res.count = Some(2);
    perf_res
        .children
        .push(PerfResChild::Text("Viola".to_string()));

    let xml = perf_res.to_mei_string().expect("serialize");
    assert!(xml.contains("<perfRes"), "should have perfRes: {}", xml);
    assert!(xml.contains("count=\"2\""), "should have count: {}", xml);
    assert!(xml.contains("Viola"), "should have text: {}", xml);

    let parsed = PerfRes::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("pr1".to_string()));
    assert_eq!(parsed.perf_res.count, Some(2));
}

#[test]
fn roundtrip_lang_usage_element() {
    use tusk_model::elements::{LangUsage, LangUsageChild, Language};

    let mut lang_usage = LangUsage::default();
    lang_usage.common.xml_id = Some("lu1".to_string());

    let language = Language::default();
    lang_usage
        .children
        .push(LangUsageChild::Language(Box::new(language)));

    let xml = lang_usage.to_mei_string().expect("serialize");
    assert!(xml.contains("<langUsage"), "should have langUsage: {}", xml);
    assert!(
        xml.contains("<language"),
        "should have language child: {}",
        xml
    );

    let parsed = LangUsage::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("lu1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_language_element() {
    use tusk_model::elements::{Language, LanguageChild};

    let mut language = Language::default();
    language.common.xml_id = Some("lang1".to_string());
    language.lang.xml_lang = Some("de".to_string());
    language
        .children
        .push(LanguageChild::Text("German".to_string()));

    let xml = language.to_mei_string().expect("serialize");
    assert!(xml.contains("<language"), "should have language: {}", xml);
    assert!(
        xml.contains("xml:lang=\"de\""),
        "should have xml:lang: {}",
        xml
    );
    assert!(xml.contains("German"), "should have text: {}", xml);

    let parsed = Language::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("lang1".to_string()));
    assert_eq!(parsed.lang.xml_lang, Some("de".to_string()));
}

#[test]
fn roundtrip_alt_id_element() {
    use tusk_model::elements::{AltId, AltIdChild};

    let mut alt_id = AltId::default();
    alt_id.common.xml_id = Some("altid1".to_string());
    alt_id
        .children
        .push(AltIdChild::Text("ISRC-12345".to_string()));

    let xml = alt_id.to_mei_string().expect("serialize");
    assert!(xml.contains("<altId"), "should have altId: {}", xml);
    assert!(
        xml.contains("xml:id=\"altid1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("ISRC-12345"), "should have text: {}", xml);

    let parsed = AltId::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("altid1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_ext_meta_element() {
    use tusk_model::elements::{ExtMeta, ExtMetaChild};

    let mut ext_meta = ExtMeta::default();
    ext_meta.common.xml_id = Some("extm1".to_string());
    ext_meta
        .children
        .push(ExtMetaChild::Text("External metadata here".to_string()));

    let xml = ext_meta.to_mei_string().expect("serialize");
    assert!(xml.contains("<extMeta"), "should have extMeta: {}", xml);
    assert!(
        xml.contains("xml:id=\"extm1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("External metadata here"),
        "should have text: {}",
        xml
    );

    let parsed = ExtMeta::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("extm1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

// ============================================================================
// Publication Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_pub_place_with_text() {
    use tusk_model::elements::{PubPlace, PubPlaceChild};

    let mut pub_place = PubPlace::default();
    pub_place.common.xml_id = Some("pp1".to_string());
    pub_place
        .children
        .push(PubPlaceChild::Text("Vienna".to_string()));

    let xml = pub_place.to_mei_string().expect("serialize");
    assert!(xml.contains("<pubPlace"), "should have pubPlace: {}", xml);
    assert!(
        xml.contains("xml:id=\"pp1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Vienna"), "should have text: {}", xml);

    let parsed = PubPlace::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("pp1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        PubPlaceChild::Text(text) => assert_eq!(text, "Vienna"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_pub_place_with_address() {
    use tusk_model::elements::{
        Address, AddressChild, PubPlace, PubPlaceChild, Settlement, SettlementChild,
    };

    let mut pub_place = PubPlace::default();
    pub_place.common.xml_id = Some("pp1".to_string());

    // Add address with settlement
    let mut address = Address::default();
    let mut settlement = Settlement::default();
    settlement
        .children
        .push(SettlementChild::Text("Vienna".to_string()));
    address
        .children
        .push(AddressChild::Settlement(Box::new(settlement)));
    pub_place
        .children
        .push(PubPlaceChild::Address(Box::new(address)));

    let xml = pub_place.to_mei_string().expect("serialize");
    assert!(xml.contains("<pubPlace"), "should have pubPlace: {}", xml);
    assert!(xml.contains("<address"), "should have address: {}", xml);
    assert!(
        xml.contains("<settlement"),
        "should have settlement: {}",
        xml
    );

    let parsed = PubPlace::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("pp1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_distributor_with_text() {
    use tusk_model::elements::{Distributor, DistributorChild};

    let mut distributor = Distributor::default();
    distributor.common.xml_id = Some("dist1".to_string());
    distributor
        .children
        .push(DistributorChild::Text("Music Archive".to_string()));

    let xml = distributor.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<distributor"),
        "should have distributor: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"dist1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Music Archive"), "should have text: {}", xml);

    let parsed = Distributor::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("dist1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DistributorChild::Text(text) => assert_eq!(text, "Music Archive"),
        other => panic!("Expected Text child, got {:?}", other),
    }
}

#[test]
fn roundtrip_distributor_with_corp_name() {
    use tusk_model::elements::{CorpName, CorpNameChild, Distributor, DistributorChild};

    let mut distributor = Distributor::default();
    let mut corp_name = CorpName::default();
    corp_name
        .children
        .push(CorpNameChild::Text("IMSLP".to_string()));
    distributor
        .children
        .push(DistributorChild::CorpName(Box::new(corp_name)));

    let xml = distributor.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<distributor"),
        "should have distributor: {}",
        xml
    );
    assert!(xml.contains("<corpName"), "should have corpName: {}", xml);
    assert!(xml.contains("IMSLP"), "should have text: {}", xml);

    let parsed = Distributor::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_availability_basic() {
    use tusk_model::elements::{Availability, AvailabilityChild};

    let mut availability = Availability::default();
    availability.common.xml_id = Some("avail1".to_string());
    availability
        .children
        .push(AvailabilityChild::Text("Public domain".to_string()));

    let xml = availability.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<availability"),
        "should have availability: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"avail1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Public domain"), "should have text: {}", xml);

    let parsed = Availability::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("avail1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_availability_with_use_restrict() {
    use tusk_model::elements::{Availability, AvailabilityChild, UseRestrict, UseRestrictChild};

    let mut availability = Availability::default();
    let mut use_restrict = UseRestrict::default();
    use_restrict.children.push(UseRestrictChild::Text(
        "Creative Commons Attribution".to_string(),
    ));
    availability
        .children
        .push(AvailabilityChild::UseRestrict(Box::new(use_restrict)));

    let xml = availability.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<availability"),
        "should have availability: {}",
        xml
    );
    assert!(
        xml.contains("<useRestrict"),
        "should have useRestrict: {}",
        xml
    );
    assert!(
        xml.contains("Creative Commons Attribution"),
        "should have text: {}",
        xml
    );

    let parsed = Availability::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AvailabilityChild::UseRestrict(ur) => {
            assert_eq!(ur.children.len(), 1);
        }
        other => panic!("Expected UseRestrict child, got {:?}", other),
    }
}

#[test]
fn roundtrip_access_restrict_with_text() {
    use tusk_model::elements::{AccessRestrict, AccessRestrictChild};

    let mut access_restrict = AccessRestrict::default();
    access_restrict.common.xml_id = Some("ar1".to_string());
    access_restrict
        .children
        .push(AccessRestrictChild::Text("Restricted access".to_string()));

    let xml = access_restrict.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<accessRestrict"),
        "should have accessRestrict: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"ar1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("Restricted access"),
        "should have text: {}",
        xml
    );

    let parsed = AccessRestrict::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("ar1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_access_restrict_with_p() {
    use tusk_model::elements::{AccessRestrict, AccessRestrictChild, P, PChild};

    let mut access_restrict = AccessRestrict::default();
    let mut p = P::default();
    p.children.push(PChild::Text(
        "Access is restricted to authorized users.".to_string(),
    ));
    access_restrict
        .children
        .push(AccessRestrictChild::P(Box::new(p)));

    let xml = access_restrict.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<accessRestrict"),
        "should have accessRestrict: {}",
        xml
    );
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(
        xml.contains("authorized users"),
        "should have text: {}",
        xml
    );

    let parsed = AccessRestrict::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AccessRestrictChild::P(p) => {
            assert_eq!(p.children.len(), 1);
        }
        other => panic!("Expected P child, got {:?}", other),
    }
}

#[test]
fn roundtrip_use_restrict_basic() {
    use tusk_model::elements::{UseRestrict, UseRestrictChild};

    let mut use_restrict = UseRestrict::default();
    use_restrict.common.xml_id = Some("ur1".to_string());
    use_restrict.children.push(UseRestrictChild::Text(
        "For educational use only".to_string(),
    ));

    let xml = use_restrict.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<useRestrict"),
        "should have useRestrict: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"ur1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("educational use"), "should have text: {}", xml);

    let parsed = UseRestrict::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("ur1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_sys_req_with_text() {
    use tusk_model::elements::{SysReq, SysReqChild};

    let mut sys_req = SysReq::default();
    sys_req.common.xml_id = Some("sr1".to_string());
    sys_req.children.push(SysReqChild::Text(
        "Requires MEI-compatible viewer".to_string(),
    ));

    let xml = sys_req.to_mei_string().expect("serialize");
    assert!(xml.contains("<sysReq"), "should have sysReq: {}", xml);
    assert!(
        xml.contains("xml:id=\"sr1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("MEI-compatible viewer"),
        "should have text: {}",
        xml
    );

    let parsed = SysReq::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sr1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_sys_req_with_p() {
    use tusk_model::elements::{P, PChild, SysReq, SysReqChild};

    let mut sys_req = SysReq::default();
    let mut p = P::default();
    p.children
        .push(PChild::Text("Minimum: Windows 10, 8GB RAM".to_string()));
    sys_req.children.push(SysReqChild::P(Box::new(p)));

    let xml = sys_req.to_mei_string().expect("serialize");
    assert!(xml.contains("<sysReq"), "should have sysReq: {}", xml);
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(xml.contains("Windows 10"), "should have text: {}", xml);

    let parsed = SysReq::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_price_with_text() {
    use tusk_model::elements::{Price, PriceChild};

    let mut price = Price::default();
    price.common.xml_id = Some("price1".to_string());
    price.children.push(PriceChild::Text("Free".to_string()));

    let xml = price.to_mei_string().expect("serialize");
    assert!(xml.contains("<price"), "should have price: {}", xml);
    assert!(
        xml.contains("xml:id=\"price1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Free"), "should have text: {}", xml);

    let parsed = Price::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("price1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_price_with_amount_and_currency() {
    use tusk_model::elements::{Price, PriceChild};

    let mut price = Price::default();
    price.common.xml_id = Some("price1".to_string());
    price.amount = Some(19.99);
    price.currency = Some("USD".to_string());
    price.children.push(PriceChild::Text("$19.99".to_string()));

    let xml = price.to_mei_string().expect("serialize");
    assert!(xml.contains("<price"), "should have price: {}", xml);
    assert!(
        xml.contains("amount=\"19.99\""),
        "should have amount: {}",
        xml
    );
    assert!(
        xml.contains("currency=\"USD\""),
        "should have currency: {}",
        xml
    );
    assert!(xml.contains("$19.99"), "should have text: {}", xml);

    let parsed = Price::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("price1".to_string()));
    assert_eq!(parsed.amount, Some(19.99));
    assert_eq!(parsed.currency, Some("USD".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_unpub_with_text() {
    use tusk_model::elements::{Unpub, UnpubChild};

    let mut unpub = Unpub::default();
    unpub.common.xml_id = Some("unpub1".to_string());
    unpub
        .children
        .push(UnpubChild::Text("Unpublished manuscript".to_string()));

    let xml = unpub.to_mei_string().expect("serialize");
    assert!(xml.contains("<unpub"), "should have unpub: {}", xml);
    assert!(
        xml.contains("xml:id=\"unpub1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(
        xml.contains("Unpublished manuscript"),
        "should have text: {}",
        xml
    );

    let parsed = Unpub::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("unpub1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        UnpubChild::Text(text) => assert_eq!(text, "Unpublished manuscript"),
    }
}

#[test]
fn roundtrip_availability_with_all_publication_elements() {
    use tusk_model::elements::{
        AccessRestrict, AccessRestrictChild, Availability, AvailabilityChild, Distributor,
        DistributorChild, Price, PriceChild, SysReq, SysReqChild, UseRestrict, UseRestrictChild,
    };

    let mut availability = Availability::default();
    availability.common.xml_id = Some("avail1".to_string());

    // Add useRestrict
    let mut use_restrict = UseRestrict::default();
    use_restrict
        .children
        .push(UseRestrictChild::Text("CC-BY-4.0".to_string()));
    availability
        .children
        .push(AvailabilityChild::UseRestrict(Box::new(use_restrict)));

    // Add accessRestrict
    let mut access_restrict = AccessRestrict::default();
    access_restrict
        .children
        .push(AccessRestrictChild::Text("Open access".to_string()));
    availability
        .children
        .push(AvailabilityChild::AccessRestrict(Box::new(access_restrict)));

    // Add sysReq
    let mut sys_req = SysReq::default();
    sys_req
        .children
        .push(SysReqChild::Text("Web browser".to_string()));
    availability
        .children
        .push(AvailabilityChild::SysReq(Box::new(sys_req)));

    // Add price
    let mut price = Price::default();
    price.amount = Some(0.0);
    price.currency = Some("USD".to_string());
    price.children.push(PriceChild::Text("Free".to_string()));
    availability
        .children
        .push(AvailabilityChild::Price(Box::new(price)));

    // Add distributor
    let mut distributor = Distributor::default();
    distributor
        .children
        .push(DistributorChild::Text("IMSLP".to_string()));
    availability
        .children
        .push(AvailabilityChild::Distributor(Box::new(distributor)));

    let xml = availability.to_mei_string().expect("serialize");
    assert!(
        xml.contains("<availability"),
        "should have availability: {}",
        xml
    );
    assert!(
        xml.contains("<useRestrict"),
        "should have useRestrict: {}",
        xml
    );
    assert!(
        xml.contains("<accessRestrict"),
        "should have accessRestrict: {}",
        xml
    );
    assert!(xml.contains("<sysReq"), "should have sysReq: {}", xml);
    assert!(xml.contains("<price"), "should have price: {}", xml);
    assert!(
        xml.contains("<distributor"),
        "should have distributor: {}",
        xml
    );

    let parsed = Availability::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("avail1".to_string()));
    assert_eq!(parsed.children.len(), 5);

    // Verify each child type
    let mut found_use_restrict = false;
    let mut found_access_restrict = false;
    let mut found_sys_req = false;
    let mut found_price = false;
    let mut found_distributor = false;

    for child in &parsed.children {
        match child {
            AvailabilityChild::UseRestrict(_) => found_use_restrict = true,
            AvailabilityChild::AccessRestrict(_) => found_access_restrict = true,
            AvailabilityChild::SysReq(_) => found_sys_req = true,
            AvailabilityChild::Price(p) => {
                found_price = true;
                assert_eq!(p.amount, Some(0.0));
                assert_eq!(p.currency, Some("USD".to_string()));
            }
            AvailabilityChild::Distributor(_) => found_distributor = true,
            _ => {}
        }
    }

    assert!(found_use_restrict, "should have useRestrict child");
    assert!(found_access_restrict, "should have accessRestrict child");
    assert!(found_sys_req, "should have sysReq child");
    assert!(found_price, "should have price child");
    assert!(found_distributor, "should have distributor child");
}

// ============================================================================
// CastList, CastGrp, CastItem, RoleDesc Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_cast_list_empty() {
    use tusk_model::elements::CastList;

    let original = CastList::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("castList"), "should contain castList: {}", xml);

    let parsed = CastList::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_cast_list_with_xml_id() {
    use tusk_model::elements::CastList;

    let mut original = CastList::default();
    original.common.xml_id = Some("cast-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"cast-1\""),
        "should contain id: {}",
        xml
    );

    let parsed = CastList::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("cast-1".to_string()));
}

#[test]
fn roundtrip_cast_list_with_head_and_cast_items() {
    use tusk_model::elements::{CastItem, CastItemChild, CastList, CastListChild, Head, HeadChild};

    let mut cast_list = CastList::default();
    cast_list.common.xml_id = Some("cast-list-1".to_string());

    // Add head
    let mut head = Head::default();
    head.children
        .push(HeadChild::Text("Dramatis Personae".to_string()));
    cast_list.children.push(CastListChild::Head(Box::new(head)));

    // Add first cast item
    let mut cast_item1 = CastItem::default();
    cast_item1.common.xml_id = Some("cast-item-1".to_string());
    cast_item1
        .children
        .push(CastItemChild::Text("Don Giovanni".to_string()));
    cast_list
        .children
        .push(CastListChild::CastItem(Box::new(cast_item1)));

    // Add second cast item
    let mut cast_item2 = CastItem::default();
    cast_item2.common.xml_id = Some("cast-item-2".to_string());
    cast_item2
        .children
        .push(CastItemChild::Text("Donna Anna".to_string()));
    cast_list
        .children
        .push(CastListChild::CastItem(Box::new(cast_item2)));

    let xml = cast_list.to_mei_string().expect("serialize");
    assert!(
        xml.contains("Dramatis Personae"),
        "should contain head text: {}",
        xml
    );
    assert!(
        xml.contains("Don Giovanni"),
        "should contain first cast item: {}",
        xml
    );
    assert!(
        xml.contains("Donna Anna"),
        "should contain second cast item: {}",
        xml
    );

    let parsed = CastList::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("cast-list-1".to_string()));
    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(parsed.children[0], CastListChild::Head(_)));
    assert!(matches!(parsed.children[1], CastListChild::CastItem(_)));
    assert!(matches!(parsed.children[2], CastListChild::CastItem(_)));
}

#[test]
fn roundtrip_cast_grp_empty() {
    use tusk_model::elements::CastGrp;

    let original = CastGrp::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("castGrp"), "should contain castGrp: {}", xml);

    let parsed = CastGrp::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_cast_grp_with_cast_items() {
    use tusk_model::elements::{CastGrp, CastGrpChild, CastItem, CastItemChild};

    let mut cast_grp = CastGrp::default();
    cast_grp.common.xml_id = Some("grp-1".to_string());

    // Add cast item
    let mut cast_item = CastItem::default();
    cast_item.common.xml_id = Some("item-1".to_string());
    cast_item
        .children
        .push(CastItemChild::Text("Leporello".to_string()));
    cast_grp
        .children
        .push(CastGrpChild::CastItem(Box::new(cast_item)));

    let xml = cast_grp.to_mei_string().expect("serialize");
    assert!(xml.contains("castGrp"), "should contain castGrp: {}", xml);
    assert!(xml.contains("castItem"), "should contain castItem: {}", xml);
    assert!(xml.contains("Leporello"), "should contain text: {}", xml);

    let parsed = CastGrp::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("grp-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    assert!(matches!(parsed.children[0], CastGrpChild::CastItem(_)));
}

#[test]
fn roundtrip_cast_grp_with_role_desc() {
    use tusk_model::elements::{CastGrp, CastGrpChild, RoleDesc, RoleDescChild};

    let mut cast_grp = CastGrp::default();
    cast_grp.common.xml_id = Some("grp-2".to_string());

    // Add role description
    let mut role_desc = RoleDesc::default();
    role_desc
        .children
        .push(RoleDescChild::Text("a young nobleman".to_string()));
    cast_grp
        .children
        .push(CastGrpChild::RoleDesc(Box::new(role_desc)));

    let xml = cast_grp.to_mei_string().expect("serialize");
    assert!(xml.contains("roleDesc"), "should contain roleDesc: {}", xml);
    assert!(
        xml.contains("a young nobleman"),
        "should contain text: {}",
        xml
    );

    let parsed = CastGrp::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 1);
    assert!(matches!(parsed.children[0], CastGrpChild::RoleDesc(_)));

    // Verify text content in role desc
    if let CastGrpChild::RoleDesc(rd) = &parsed.children[0] {
        assert_eq!(rd.children.len(), 1);
        if let RoleDescChild::Text(text) = &rd.children[0] {
            assert_eq!(text, "a young nobleman");
        } else {
            panic!("expected text child");
        }
    }
}

#[test]
fn roundtrip_cast_grp_nested() {
    use tusk_model::elements::{CastGrp, CastGrpChild, CastItem, CastItemChild};

    let mut outer_grp = CastGrp::default();
    outer_grp.common.xml_id = Some("outer".to_string());

    // Create inner group
    let mut inner_grp = CastGrp::default();
    inner_grp.common.xml_id = Some("inner".to_string());

    // Add cast item to inner group
    let mut cast_item = CastItem::default();
    cast_item
        .children
        .push(CastItemChild::Text("Zerlina".to_string()));
    inner_grp
        .children
        .push(CastGrpChild::CastItem(Box::new(cast_item)));

    // Add inner group to outer
    outer_grp
        .children
        .push(CastGrpChild::CastGrp(Box::new(inner_grp)));

    let xml = outer_grp.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"outer\""),
        "should contain outer id: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"inner\""),
        "should contain inner id: {}",
        xml
    );

    let parsed = CastGrp::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("outer".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let CastGrpChild::CastGrp(inner) = &parsed.children[0] {
        assert_eq!(inner.common.xml_id, Some("inner".to_string()));
        assert_eq!(inner.children.len(), 1);
    } else {
        panic!("expected nested CastGrp");
    }
}

#[test]
fn roundtrip_cast_item_empty() {
    use tusk_model::elements::CastItem;

    let original = CastItem::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("castItem"), "should contain castItem: {}", xml);

    let parsed = CastItem::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_cast_item_with_text() {
    use tusk_model::elements::{CastItem, CastItemChild};

    let mut cast_item = CastItem::default();
    cast_item.common.xml_id = Some("ci-1".to_string());
    cast_item
        .children
        .push(CastItemChild::Text("Il Commendatore".to_string()));

    let xml = cast_item.to_mei_string().expect("serialize");
    assert!(
        xml.contains("Il Commendatore"),
        "should contain text: {}",
        xml
    );

    let parsed = CastItem::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("ci-1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let CastItemChild::Text(text) = &parsed.children[0] {
        assert_eq!(text, "Il Commendatore");
    } else {
        panic!("expected text child");
    }
}

#[test]
fn roundtrip_cast_item_with_role_desc() {
    use tusk_model::elements::{CastItem, CastItemChild, RoleDesc, RoleDescChild};

    let mut cast_item = CastItem::default();
    cast_item.common.xml_id = Some("ci-2".to_string());

    // Add text
    cast_item
        .children
        .push(CastItemChild::Text("Masetto".to_string()));

    // Add role description
    let mut role_desc = RoleDesc::default();
    role_desc
        .children
        .push(RoleDescChild::Text("a peasant".to_string()));
    cast_item
        .children
        .push(CastItemChild::RoleDesc(Box::new(role_desc)));

    let xml = cast_item.to_mei_string().expect("serialize");
    assert!(xml.contains("Masetto"), "should contain text: {}", xml);
    assert!(xml.contains("roleDesc"), "should contain roleDesc: {}", xml);
    assert!(
        xml.contains("a peasant"),
        "should contain role text: {}",
        xml
    );

    let parsed = CastItem::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 2);

    let mut found_text = false;
    let mut found_role_desc = false;
    for child in &parsed.children {
        match child {
            CastItemChild::Text(t) => {
                found_text = true;
                assert_eq!(t, "Masetto");
            }
            CastItemChild::RoleDesc(rd) => {
                found_role_desc = true;
                assert_eq!(rd.children.len(), 1);
            }
            _ => {}
        }
    }
    assert!(found_text, "should have text child");
    assert!(found_role_desc, "should have roleDesc child");
}

#[test]
fn roundtrip_role_desc_empty() {
    use tusk_model::elements::RoleDesc;

    let original = RoleDesc::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("roleDesc"), "should contain roleDesc: {}", xml);

    let parsed = RoleDesc::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_role_desc_with_text() {
    use tusk_model::elements::{RoleDesc, RoleDescChild};

    let mut role_desc = RoleDesc::default();
    role_desc.common.xml_id = Some("rd-1".to_string());
    role_desc
        .children
        .push(RoleDescChild::Text("a servant of Don Giovanni".to_string()));

    let xml = role_desc.to_mei_string().expect("serialize");
    assert!(
        xml.contains("a servant of Don Giovanni"),
        "should contain text: {}",
        xml
    );

    let parsed = RoleDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("rd-1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let RoleDescChild::Text(text) = &parsed.children[0] {
        assert_eq!(text, "a servant of Don Giovanni");
    } else {
        panic!("expected text child");
    }
}

#[test]
fn roundtrip_cast_list_with_cast_grp() {
    use tusk_model::elements::{
        CastGrp, CastGrpChild, CastItem, CastItemChild, CastList, CastListChild, RoleDesc,
        RoleDescChild,
    };

    let mut cast_list = CastList::default();
    cast_list.common.xml_id = Some("cast-full".to_string());

    // Create a cast group for "Principal Characters"
    let mut cast_grp = CastGrp::default();
    cast_grp.common.xml_id = Some("principals".to_string());

    // Add role desc for the group
    let mut group_role = RoleDesc::default();
    group_role
        .children
        .push(RoleDescChild::Text("Principal Characters".to_string()));
    cast_grp
        .children
        .push(CastGrpChild::RoleDesc(Box::new(group_role)));

    // Add cast item
    let mut ci = CastItem::default();
    ci.children
        .push(CastItemChild::Text("Don Giovanni".to_string()));
    cast_grp.children.push(CastGrpChild::CastItem(Box::new(ci)));

    cast_list
        .children
        .push(CastListChild::CastGrp(Box::new(cast_grp)));

    let xml = cast_list.to_mei_string().expect("serialize");
    assert!(
        xml.contains("Principal Characters"),
        "should contain group desc: {}",
        xml
    );

    let parsed = CastList::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("cast-full".to_string()));
    assert_eq!(parsed.children.len(), 1);
    assert!(matches!(parsed.children[0], CastListChild::CastGrp(_)));

    if let CastListChild::CastGrp(grp) = &parsed.children[0] {
        assert_eq!(grp.common.xml_id, Some("principals".to_string()));
        assert_eq!(grp.children.len(), 2);
    }
}

// ============================================================================
// Address and Geographic Elements Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_address_with_all_children() {
    use tusk_model::elements::{
        AddrLine, AddrLineChild, Address, AddressChild, Bloc, BlocChild, Country, CountryChild,
        District, DistrictChild, GeogFeat, GeogFeatChild, PostBox, PostBoxChild, PostCode,
        PostCodeChild, PubPlace, PubPlaceChild, Region, RegionChild, Settlement, SettlementChild,
        Street, StreetChild,
    };

    let mut pub_place = PubPlace::default();
    pub_place.common.xml_id = Some("pp-addr-test".to_string());

    // Build an address with all possible children
    let mut address = Address::default();
    address.common.xml_id = Some("addr1".to_string());

    // Add addrLine
    let mut addr_line = AddrLine::default();
    addr_line
        .children
        .push(AddrLineChild::Text("123 Music Street".to_string()));
    address
        .children
        .push(AddressChild::AddrLine(Box::new(addr_line)));

    // Add street
    let mut street = Street::default();
    street
        .children
        .push(StreetChild::Text("Composer Avenue".to_string()));
    address
        .children
        .push(AddressChild::Street(Box::new(street)));

    // Add postCode
    let mut post_code = PostCode::default();
    post_code
        .children
        .push(PostCodeChild::Text("12345".to_string()));
    address
        .children
        .push(AddressChild::PostCode(Box::new(post_code)));

    // Add postBox
    let mut post_box = PostBox::default();
    post_box
        .children
        .push(PostBoxChild::Text("PO Box 100".to_string()));
    address
        .children
        .push(AddressChild::PostBox(Box::new(post_box)));

    // Add settlement (city)
    let mut settlement = Settlement::default();
    settlement.common.xml_id = Some("settle1".to_string());
    settlement
        .children
        .push(SettlementChild::Text("Vienna".to_string()));
    address
        .children
        .push(AddressChild::Settlement(Box::new(settlement)));

    // Add district
    let mut district = District::default();
    district
        .children
        .push(DistrictChild::Text("Innere Stadt".to_string()));
    address
        .children
        .push(AddressChild::District(Box::new(district)));

    // Add region
    let mut region = Region::default();
    region
        .children
        .push(RegionChild::Text("Lower Austria".to_string()));
    address
        .children
        .push(AddressChild::Region(Box::new(region)));

    // Add country
    let mut country = Country::default();
    country.common.xml_id = Some("country1".to_string());
    country
        .children
        .push(CountryChild::Text("Austria".to_string()));
    address
        .children
        .push(AddressChild::Country(Box::new(country)));

    // Add bloc
    let mut bloc = Bloc::default();
    bloc.children.push(BlocChild::Text("EU".to_string()));
    address.children.push(AddressChild::Bloc(Box::new(bloc)));

    // Add geogFeat
    let mut geog_feat = GeogFeat::default();
    geog_feat
        .children
        .push(GeogFeatChild::Text("Danube River".to_string()));
    address
        .children
        .push(AddressChild::GeogFeat(Box::new(geog_feat)));

    pub_place
        .children
        .push(PubPlaceChild::Address(Box::new(address)));

    // Serialize and deserialize
    let xml = pub_place.to_mei_string().expect("serialize");
    assert!(xml.contains("<address"), "should have address: {}", xml);
    assert!(xml.contains("<addrLine"), "should have addrLine: {}", xml);
    assert!(xml.contains("<street"), "should have street: {}", xml);
    assert!(xml.contains("<postCode"), "should have postCode: {}", xml);
    assert!(xml.contains("<postBox"), "should have postBox: {}", xml);
    assert!(
        xml.contains("<settlement"),
        "should have settlement: {}",
        xml
    );
    assert!(xml.contains("<district"), "should have district: {}", xml);
    assert!(xml.contains("<region"), "should have region: {}", xml);
    assert!(xml.contains("<country"), "should have country: {}", xml);
    assert!(xml.contains("<bloc"), "should have bloc: {}", xml);
    assert!(xml.contains("<geogFeat"), "should have geogFeat: {}", xml);

    let parsed = PubPlace::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("pp-addr-test".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let PubPlaceChild::Address(addr) = &parsed.children[0] {
        assert_eq!(addr.common.xml_id, Some("addr1".to_string()));
        assert_eq!(addr.children.len(), 10);

        // Check addrLine
        assert!(
            addr.children
                .iter()
                .any(|c| matches!(c, AddressChild::AddrLine(al) if al.children.iter().any(|c| matches!(c, AddrLineChild::Text(t) if t == "123 Music Street")))),
            "should have addrLine with correct text"
        );

        // Check street
        assert!(
            addr.children
                .iter()
                .any(|c| matches!(c, AddressChild::Street(s) if s.children.iter().any(|c| matches!(c, StreetChild::Text(t) if t == "Composer Avenue")))),
            "should have street with correct text"
        );

        // Check settlement
        assert!(
            addr.children.iter().any(
                |c| matches!(c, AddressChild::Settlement(s) if s.common.xml_id == Some("settle1".to_string()))
            ),
            "should have settlement with xml_id"
        );

        // Check country
        assert!(
            addr.children.iter().any(
                |c| matches!(c, AddressChild::Country(c) if c.common.xml_id == Some("country1".to_string()))
            ),
            "should have country with xml_id"
        );
    } else {
        panic!("Expected Address child");
    }
}

#[test]
fn roundtrip_geog_name_with_nested_elements() {
    use tusk_model::elements::{
        Address, AddressChild, BiblScope, BiblScopeChild, GeogName, GeogNameChild, Settlement,
        SettlementChild,
    };

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs-geog".to_string());

    // Create geogName with nested geogName and address
    let mut geog_name = GeogName::default();
    geog_name.common.xml_id = Some("geog1".to_string());
    geog_name
        .children
        .push(GeogNameChild::Text("Central ".to_string()));

    // Nested geogName
    let mut nested_geog = GeogName::default();
    nested_geog
        .children
        .push(GeogNameChild::Text("Europe".to_string()));
    geog_name
        .children
        .push(GeogNameChild::GeogName(Box::new(nested_geog)));

    // Add address inside geogName
    let mut addr = Address::default();
    let mut settlement = Settlement::default();
    settlement
        .children
        .push(SettlementChild::Text("Prague".to_string()));
    addr.children
        .push(AddressChild::Settlement(Box::new(settlement)));
    geog_name
        .children
        .push(GeogNameChild::Address(Box::new(addr)));

    bibl_scope
        .children
        .push(BiblScopeChild::GeogName(Box::new(geog_name)));

    let xml = bibl_scope.to_mei_string().expect("serialize");
    assert!(xml.contains("<geogName"), "should have geogName: {}", xml);
    assert!(
        xml.contains("Central "),
        "should have 'Central ' text: {}",
        xml
    );
    assert!(xml.contains("Europe"), "should have 'Europe' text: {}", xml);
    assert!(xml.contains("Prague"), "should have 'Prague' text: {}", xml);

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bs-geog".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let BiblScopeChild::GeogName(geog) = &parsed.children[0] {
        assert_eq!(geog.common.xml_id, Some("geog1".to_string()));
        assert!(geog.children.len() >= 2);

        // Check for nested geogName
        assert!(
            geog.children.iter().any(|c| matches!(c, GeogNameChild::GeogName(ng) if ng.children.iter().any(|c| matches!(c, GeogNameChild::Text(t) if t == "Europe")))),
            "should have nested geogName with 'Europe'"
        );

        // Check for address
        assert!(
            geog.children
                .iter()
                .any(|c| matches!(c, GeogNameChild::Address(_))),
            "should have Address child"
        );
    } else {
        panic!("Expected GeogName child");
    }
}

// ============================================================================
// Bibliographic Elements Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_bibl_with_imprint() {
    use tusk_model::elements::{
        Bibl, BiblChild, Date, DateChild, Imprint, ImprintChild, PubPlace, PubPlaceChild,
        Publisher, PublisherChild, Title, TitleChild,
    };

    let mut bibl = Bibl::default();
    bibl.common.xml_id = Some("bibl-imprint".to_string());

    // Add title
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Complete Piano Sonatas".to_string()));
    bibl.children.push(BiblChild::Title(Box::new(title)));

    // Add imprint with full details
    let mut imprint = Imprint::default();
    imprint.common.xml_id = Some("imp1".to_string());

    // Publisher
    let mut publisher = Publisher::default();
    publisher
        .children
        .push(PublisherChild::Text("Breitkopf und Härtel".to_string()));
    imprint
        .children
        .push(ImprintChild::Publisher(Box::new(publisher)));

    // PubPlace
    let mut pub_place = PubPlace::default();
    pub_place
        .children
        .push(PubPlaceChild::Text("Leipzig".to_string()));
    imprint
        .children
        .push(ImprintChild::PubPlace(Box::new(pub_place)));

    // Date
    let mut date = Date::default();
    date.children.push(DateChild::Text("1862".to_string()));
    imprint.children.push(ImprintChild::Date(Box::new(date)));

    bibl.children.push(BiblChild::Imprint(Box::new(imprint)));

    let xml = bibl.to_mei_string().expect("serialize");
    assert!(xml.contains("<bibl"), "should have bibl: {}", xml);
    assert!(xml.contains("<imprint"), "should have imprint: {}", xml);
    assert!(xml.contains("<publisher"), "should have publisher: {}", xml);
    assert!(xml.contains("<pubPlace"), "should have pubPlace: {}", xml);
    assert!(xml.contains("<date"), "should have date: {}", xml);
    assert!(
        xml.contains("Breitkopf und Härtel"),
        "should have publisher name: {}",
        xml
    );
    assert!(xml.contains("Leipzig"), "should have pub place: {}", xml);
    assert!(xml.contains("1862"), "should have date: {}", xml);

    let parsed = Bibl::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bibl-imprint".to_string()));
    assert_eq!(parsed.children.len(), 2);

    // Check for imprint
    assert!(
        parsed
            .children
            .iter()
            .any(|c| matches!(c, BiblChild::Imprint(imp) if imp.common.xml_id == Some("imp1".to_string()))),
        "should have Imprint child with xml_id"
    );
}

#[test]
fn roundtrip_bibl_scope_with_locus() {
    use tusk_model::elements::{BiblScope, BiblScopeChild, Locus, LocusChild};

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs-locus".to_string());
    bibl_scope.from = Some("1".to_string());
    bibl_scope.to = Some("50".to_string());

    // Add locus
    let mut locus = Locus::default();
    locus.common.xml_id = Some("loc1".to_string());
    locus
        .children
        .push(LocusChild::Text("ff. 1-50".to_string()));
    bibl_scope
        .children
        .push(BiblScopeChild::Locus(Box::new(locus)));

    let xml = bibl_scope.to_mei_string().expect("serialize");
    assert!(xml.contains("<biblScope"), "should have biblScope: {}", xml);
    assert!(xml.contains("<locus"), "should have locus: {}", xml);
    assert!(
        xml.contains("from=\"1\""),
        "should have from attribute: {}",
        xml
    );
    assert!(
        xml.contains("to=\"50\""),
        "should have to attribute: {}",
        xml
    );
    assert!(xml.contains("ff. 1-50"), "should have locus text: {}", xml);

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bs-locus".to_string()));
    assert_eq!(parsed.from, Some("1".to_string()));
    assert_eq!(parsed.to, Some("50".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let BiblScopeChild::Locus(loc) = &parsed.children[0] {
        assert_eq!(loc.common.xml_id, Some("loc1".to_string()));
        assert!(
            loc.children
                .iter()
                .any(|c| matches!(c, LocusChild::Text(t) if t == "ff. 1-50")),
            "should have locus text"
        );
    } else {
        panic!("Expected Locus child");
    }
}

#[test]
fn roundtrip_bibl_scope_with_locus_grp() {
    use tusk_model::elements::{
        BiblScope, BiblScopeChild, Locus, LocusChild, LocusGrp, LocusGrpChild,
    };

    let mut bibl_scope = BiblScope::default();
    bibl_scope.common.xml_id = Some("bs-locusgrp".to_string());

    // Add locusGrp with multiple locus elements
    let mut locus_grp = LocusGrp::default();
    locus_grp.common.xml_id = Some("locgrp1".to_string());

    let mut locus1 = Locus::default();
    locus1
        .children
        .push(LocusChild::Text("ff. 1-10".to_string()));
    locus_grp
        .children
        .push(LocusGrpChild::Locus(Box::new(locus1)));

    let mut locus2 = Locus::default();
    locus2
        .children
        .push(LocusChild::Text("ff. 20-30".to_string()));
    locus_grp
        .children
        .push(LocusGrpChild::Locus(Box::new(locus2)));

    bibl_scope
        .children
        .push(BiblScopeChild::LocusGrp(Box::new(locus_grp)));

    let xml = bibl_scope.to_mei_string().expect("serialize");
    assert!(xml.contains("<biblScope"), "should have biblScope: {}", xml);
    assert!(xml.contains("<locusGrp"), "should have locusGrp: {}", xml);
    assert!(
        xml.contains("ff. 1-10"),
        "should have first locus text: {}",
        xml
    );
    assert!(
        xml.contains("ff. 20-30"),
        "should have second locus text: {}",
        xml
    );

    let parsed = BiblScope::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bs-locusgrp".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let BiblScopeChild::LocusGrp(lg) = &parsed.children[0] {
        assert_eq!(lg.common.xml_id, Some("locgrp1".to_string()));
        assert_eq!(lg.children.len(), 2);
        assert!(
            lg.children
                .iter()
                .all(|c| matches!(c, LocusGrpChild::Locus(_))),
            "all children should be Locus"
        );
    } else {
        panic!("Expected LocusGrp child");
    }
}

#[test]
fn roundtrip_bibl_with_bibl_struct() {
    use tusk_model::elements::{Bibl, BiblChild, BiblStruct};

    let mut bibl = Bibl::default();
    bibl.common.xml_id = Some("bibl-struct".to_string());

    // Add biblStruct
    let mut bibl_struct = BiblStruct::default();
    bibl_struct.common.xml_id = Some("bstruct1".to_string());

    bibl.children
        .push(BiblChild::BiblStruct(Box::new(bibl_struct)));

    let xml = bibl.to_mei_string().expect("serialize");
    assert!(xml.contains("<bibl"), "should have bibl: {}", xml);
    assert!(
        xml.contains("<biblStruct"),
        "should have biblStruct: {}",
        xml
    );
    assert!(
        xml.contains("xml:id=\"bstruct1\""),
        "should have biblStruct xml:id: {}",
        xml
    );

    let parsed = Bibl::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bibl-struct".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let BiblChild::BiblStruct(bs) = &parsed.children[0] {
        assert_eq!(bs.common.xml_id, Some("bstruct1".to_string()));
    } else {
        panic!("Expected BiblStruct child");
    }
}

#[test]
fn roundtrip_imprint_with_all_children() {
    use tusk_model::elements::{
        Address, AddressChild, Availability, Bibl, BiblChild, Date, DateChild, Distributor,
        DistributorChild, Imprint, ImprintChild, PersName, PersNameChild, PubPlace, PubPlaceChild,
        Publisher, PublisherChild, RespStmt, RespStmtChild, Settlement, SettlementChild, Title,
        TitleChild,
    };

    let mut bibl = Bibl::default();
    bibl.common.xml_id = Some("bibl-imp-full".to_string());

    let mut imprint = Imprint::default();
    imprint.common.xml_id = Some("imp-full".to_string());

    // Publisher
    let mut publisher = Publisher::default();
    publisher
        .children
        .push(PublisherChild::Text("Peters Edition".to_string()));
    imprint
        .children
        .push(ImprintChild::Publisher(Box::new(publisher)));

    // Distributor
    let mut distributor = Distributor::default();
    distributor.children.push(DistributorChild::Text(
        "Music Distributors Inc.".to_string(),
    ));
    imprint
        .children
        .push(ImprintChild::Distributor(Box::new(distributor)));

    // PubPlace with address
    let mut pub_place = PubPlace::default();
    let mut addr = Address::default();
    let mut settlement = Settlement::default();
    settlement
        .children
        .push(SettlementChild::Text("Frankfurt".to_string()));
    addr.children
        .push(AddressChild::Settlement(Box::new(settlement)));
    pub_place
        .children
        .push(PubPlaceChild::Address(Box::new(addr)));
    imprint
        .children
        .push(ImprintChild::PubPlace(Box::new(pub_place)));

    // Date
    let mut date = Date::default();
    date.children.push(DateChild::Text("1905".to_string()));
    imprint.children.push(ImprintChild::Date(Box::new(date)));

    // RespStmt
    let mut resp_stmt = RespStmt::default();
    let mut pers_name = PersName::default();
    pers_name
        .children
        .push(PersNameChild::Text("Max Reger".to_string()));
    resp_stmt
        .children
        .push(RespStmtChild::PersName(Box::new(pers_name)));
    imprint
        .children
        .push(ImprintChild::RespStmt(Box::new(resp_stmt)));

    // Availability
    let availability = Availability::default();
    imprint
        .children
        .push(ImprintChild::Availability(Box::new(availability)));

    // Title
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Organ Works".to_string()));
    imprint.children.push(ImprintChild::Title(Box::new(title)));

    // Address directly
    let mut direct_addr = Address::default();
    let mut settle2 = Settlement::default();
    settle2
        .children
        .push(SettlementChild::Text("Berlin".to_string()));
    direct_addr
        .children
        .push(AddressChild::Settlement(Box::new(settle2)));
    imprint
        .children
        .push(ImprintChild::Address(Box::new(direct_addr)));

    bibl.children.push(BiblChild::Imprint(Box::new(imprint)));

    let xml = bibl.to_mei_string().expect("serialize");
    assert!(xml.contains("<imprint"), "should have imprint: {}", xml);
    assert!(xml.contains("<publisher"), "should have publisher: {}", xml);
    assert!(
        xml.contains("<distributor"),
        "should have distributor: {}",
        xml
    );
    assert!(xml.contains("<pubPlace"), "should have pubPlace: {}", xml);
    assert!(xml.contains("<date"), "should have date: {}", xml);
    assert!(xml.contains("<respStmt"), "should have respStmt: {}", xml);
    assert!(
        xml.contains("<availability"),
        "should have availability: {}",
        xml
    );
    assert!(xml.contains("<title"), "should have title: {}", xml);
    assert!(xml.contains("<address"), "should have address: {}", xml);
    assert!(
        xml.contains("Peters Edition"),
        "should have publisher text: {}",
        xml
    );
    assert!(
        xml.contains("Max Reger"),
        "should have respStmt text: {}",
        xml
    );
    assert!(xml.contains("Frankfurt"), "should have Frankfurt: {}", xml);
    assert!(xml.contains("Berlin"), "should have Berlin: {}", xml);

    let parsed = Bibl::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("bibl-imp-full".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let BiblChild::Imprint(imp) = &parsed.children[0] {
        assert_eq!(imp.common.xml_id, Some("imp-full".to_string()));
        // Imprint should have 8 children
        assert_eq!(imp.children.len(), 8);

        // Check for various child types
        assert!(
            imp.children
                .iter()
                .any(|c| matches!(c, ImprintChild::Publisher(_))),
            "should have Publisher"
        );
        assert!(
            imp.children
                .iter()
                .any(|c| matches!(c, ImprintChild::Distributor(_))),
            "should have Distributor"
        );
        assert!(
            imp.children
                .iter()
                .any(|c| matches!(c, ImprintChild::PubPlace(_))),
            "should have PubPlace"
        );
        assert!(
            imp.children
                .iter()
                .any(|c| matches!(c, ImprintChild::Date(_))),
            "should have Date"
        );
        assert!(
            imp.children
                .iter()
                .any(|c| matches!(c, ImprintChild::RespStmt(_))),
            "should have RespStmt"
        );
        assert!(
            imp.children
                .iter()
                .any(|c| matches!(c, ImprintChild::Address(_))),
            "should have Address"
        );
    } else {
        panic!("Expected Imprint child");
    }
}

// ============================================================================
// Physical Description Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_dimensions_empty() {
    use tusk_model::elements::Dimensions;

    let original = Dimensions::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("dimensions"), "xml should contain dimensions");

    let parsed = Dimensions::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_dimensions_with_attrs() {
    use tusk_model::att::AttMeasurementUnit;
    use tusk_model::elements::Dimensions;

    let mut original = Dimensions::default();
    original.common.xml_id = Some("dims1".to_string());
    original.measurement.unit = Some(AttMeasurementUnit::Mm);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("xml:id=\"dims1\""), "should have id: {}", xml);
    assert!(xml.contains("unit=\"mm\""), "should have unit: {}", xml);

    let parsed = Dimensions::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("dims1".to_string()));
    assert_eq!(parsed.measurement.unit, Some(AttMeasurementUnit::Mm));
}

#[test]
fn roundtrip_dimensions_with_children() {
    use tusk_model::elements::{Depth, Dimensions, DimensionsChild, Height, Width};

    let mut original = Dimensions::default();
    original.common.xml_id = Some("dims2".to_string());

    let mut height = Height::default();
    height.common.xml_id = Some("h1".to_string());
    original
        .children
        .push(DimensionsChild::Height(Box::new(height)));

    let mut width = Width::default();
    width.common.xml_id = Some("w1".to_string());
    original
        .children
        .push(DimensionsChild::Width(Box::new(width)));

    let mut depth = Depth::default();
    depth.common.xml_id = Some("d1".to_string());
    original
        .children
        .push(DimensionsChild::Depth(Box::new(depth)));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("height"), "should have height: {}", xml);
    assert!(xml.contains("width"), "should have width: {}", xml);
    assert!(xml.contains("depth"), "should have depth: {}", xml);

    let parsed = Dimensions::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 3);
}

#[test]
fn roundtrip_height_with_quantity() {
    use tusk_model::att::AttQuantityUnit;
    use tusk_model::elements::Height;

    let mut original = Height::default();
    original.common.xml_id = Some("height1".to_string());
    original.quantity.quantity = Some(250.0);
    original.quantity.unit = Some(AttQuantityUnit::Mm);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("quantity=\"250\""),
        "should have quantity: {}",
        xml
    );
    assert!(xml.contains("unit=\"mm\""), "should have unit: {}", xml);

    let parsed = Height::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("height1".to_string()));
    assert_eq!(parsed.quantity.quantity, Some(250.0));
    assert_eq!(parsed.quantity.unit, Some(AttQuantityUnit::Mm));
}

#[test]
fn roundtrip_width_with_quantity() {
    use tusk_model::elements::Width;

    let mut original = Width::default();
    original.common.xml_id = Some("width1".to_string());
    original.quantity.quantity = Some(180.0);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("quantity=\"180\""),
        "should have quantity: {}",
        xml
    );

    let parsed = Width::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.quantity.quantity, Some(180.0));
}

#[test]
fn roundtrip_depth_with_quantity() {
    use tusk_model::elements::Depth;

    let mut original = Depth::default();
    original.common.xml_id = Some("depth1".to_string());
    original.quantity.quantity = Some(50.0);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("quantity=\"50\""),
        "should have quantity: {}",
        xml
    );

    let parsed = Depth::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.quantity.quantity, Some(50.0));
}

#[test]
fn roundtrip_dim_with_text() {
    use tusk_model::elements::{Dim, DimChild};

    let mut original = Dim::default();
    original.common.xml_id = Some("dim1".to_string());
    original.form = Some("folio".to_string());
    original
        .children
        .push(DimChild::Text("12 folios".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("form=\"folio\""), "should have form: {}", xml);
    assert!(xml.contains("12 folios"), "should have text: {}", xml);

    let parsed = Dim::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.form, Some("folio".to_string()));
    assert!(
        parsed
            .children
            .iter()
            .any(|c| matches!(c, DimChild::Text(t) if t.contains("12 folios")))
    );
}

#[test]
fn roundtrip_support_empty() {
    use tusk_model::elements::Support;

    let original = Support::default();
    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("support"), "xml should contain support");

    let parsed = Support::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_support_with_children() {
    use tusk_model::elements::{Condition, Dimensions, P, PChild, Support, SupportChild};

    let mut original = Support::default();
    original.common.xml_id = Some("sup1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text("Parchment".to_string()));
    original.children.push(SupportChild::P(Box::new(p)));

    let dims = Dimensions::default();
    original
        .children
        .push(SupportChild::Dimensions(Box::new(dims)));

    let cond = Condition::default();
    original
        .children
        .push(SupportChild::Condition(Box::new(cond)));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("<p>"), "should have p: {}", xml);
    assert!(xml.contains("Parchment"), "should have text: {}", xml);
    assert!(
        xml.contains("dimensions"),
        "should have dimensions: {}",
        xml
    );
    assert!(xml.contains("condition"), "should have condition: {}", xml);

    let parsed = Support::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.children.len(), 3);
}

#[test]
fn roundtrip_support_desc_with_material() {
    use tusk_model::elements::{Support, SupportDesc, SupportDescChild};

    let mut original = SupportDesc::default();
    original.common.xml_id = Some("supdesc1".to_string());
    original.material = Some("parchment".to_string());

    let support = Support::default();
    original
        .children
        .push(SupportDescChild::Support(Box::new(support)));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("supportDesc"),
        "xml should contain supportDesc: {}",
        xml
    );
    assert!(
        xml.contains("material=\"parchment\""),
        "should have material: {}",
        xml
    );
    assert!(
        xml.contains("<support"),
        "should have support child: {}",
        xml
    );

    let parsed = SupportDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.material, Some("parchment".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_collation_with_text() {
    use tusk_model::elements::{Collation, CollationChild, P, PChild};

    let mut original = Collation::default();
    original.common.xml_id = Some("coll1".to_string());

    let mut p = P::default();
    p.children
        .push(PChild::Text("8 quires of 8 leaves".to_string()));
    original.children.push(CollationChild::P(Box::new(p)));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("collation"),
        "xml should contain collation: {}",
        xml
    );
    assert!(xml.contains("8 quires"), "should have text: {}", xml);

    let parsed = Collation::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("coll1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_foliation_with_text() {
    use tusk_model::elements::{Foliation, FoliationChild, P, PChild};

    let mut original = Foliation::default();
    original.common.xml_id = Some("fol1".to_string());

    let mut p = P::default();
    p.children
        .push(PChild::Text("Modern foliation 1-64".to_string()));
    original.children.push(FoliationChild::P(Box::new(p)));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("foliation"),
        "xml should contain foliation: {}",
        xml
    );
    assert!(
        xml.contains("Modern foliation"),
        "should have text: {}",
        xml
    );

    let parsed = Foliation::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("fol1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_condition_with_text() {
    use tusk_model::elements::{Condition, ConditionChild, P, PChild};

    let mut original = Condition::default();
    original.common.xml_id = Some("cond1".to_string());

    let mut p = P::default();
    p.children.push(PChild::Text(
        "Good condition with minor water damage".to_string(),
    ));
    original.children.push(ConditionChild::P(Box::new(p)));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("condition"),
        "xml should contain condition: {}",
        xml
    );
    assert!(xml.contains("water damage"), "should have text: {}", xml);

    let parsed = Condition::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("cond1".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

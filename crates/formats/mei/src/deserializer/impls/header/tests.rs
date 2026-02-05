use crate::deserializer::MeiDeserialize;
use tusk_model::elements::{
    AppInfo, AppInfoChild, Application, ApplicationChild, CreatorChild, EditorChild, EditorialDecl,
    EditorialDeclChild, EncodingDesc, EncodingDescChild, FileDesc, FileDescChild, MeiHead,
    MeiHeadChild, PChild, ProjectDesc, ProjectDescChild, PubStmt, PubStmtChild, Source,
    SourceChild, SourceDesc, SourceDescChild, TitleChild, TitleStmt, TitleStmtChild,
};

// ============================================================================
// MeiHead element tests
// ============================================================================

#[test]
fn mei_head_deserializes_from_empty_element() {
    let xml = r#"<meiHead/>"#;
    let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
    assert!(mei_head.basic.xml_id.is_none());
    assert!(mei_head.children.is_empty());
}

#[test]
fn mei_head_deserializes_xml_id() {
    let xml = r#"<meiHead xml:id="header1"/>"#;
    let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
    assert_eq!(mei_head.basic.xml_id, Some("header1".to_string()));
}

#[test]
fn mei_head_deserializes_file_desc_child() {
    let xml = r#"<meiHead xml:id="h1">
        <fileDesc xml:id="fd1"/>
    </meiHead>"#;
    let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
    assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
    assert_eq!(mei_head.children.len(), 1);
    match &mei_head.children[0] {
        MeiHeadChild::FileDesc(fd) => {
            assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
        }
        _ => panic!("expected FileDesc child"),
    }
}

// ========== FileDesc tests ==========

#[test]
fn file_desc_deserializes_empty_element() {
    let xml = r#"<fileDesc/>"#;
    let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
    assert!(file_desc.common.xml_id.is_none());
    assert!(file_desc.children.is_empty());
}

#[test]
fn file_desc_deserializes_xml_id() {
    let xml = r#"<fileDesc xml:id="fd1"/>"#;
    let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
    assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
}

#[test]
fn file_desc_deserializes_title_stmt_child() {
    let xml = r#"<fileDesc xml:id="fd1">
        <titleStmt>
            <title>My Composition</title>
        </titleStmt>
    </fileDesc>"#;
    let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
    assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
    assert_eq!(file_desc.children.len(), 1);
    match &file_desc.children[0] {
        FileDescChild::TitleStmt(ts) => {
            assert_eq!(ts.children.len(), 1);
            assert!(matches!(&ts.children[0], TitleStmtChild::Title(_)));
        }
        _ => panic!("expected TitleStmt child"),
    }
}

// ========== TitleStmt tests ==========

#[test]
fn title_stmt_deserializes_empty_element() {
    let xml = r#"<titleStmt/>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert!(title_stmt.common.xml_id.is_none());
    assert!(title_stmt.children.is_empty());
}

#[test]
fn title_stmt_deserializes_with_title_child() {
    let xml = r#"<titleStmt>
        <title>Test Title</title>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(title_stmt.children.len(), 1);
    match &title_stmt.children[0] {
        TitleStmtChild::Title(t) => {
            assert_eq!(t.children.len(), 1);
        }
        _ => panic!("expected Title child"),
    }
}

#[test]
fn title_stmt_deserializes_title_text_content() {
    let xml = r#"<titleStmt>
        <title>My Composition</title>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(title_stmt.children.len(), 1);
    match &title_stmt.children[0] {
        TitleStmtChild::Title(t) => {
            assert_eq!(t.children.len(), 1);
            match &t.children[0] {
                TitleChild::Text(text) => assert_eq!(text.trim(), "My Composition"),
                _ => panic!("expected text child in title"),
            }
        }
        _ => panic!("expected Title child"),
    }
}

// ========== PubStmt tests ==========

#[test]
fn pub_stmt_deserializes_empty_element() {
    let xml = r#"<pubStmt/>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
    assert!(pub_stmt.common.xml_id.is_none());
    assert!(pub_stmt.children.is_empty());
}

#[test]
fn pub_stmt_deserializes_publisher_child() {
    let xml = r#"<pubStmt>
        <publisher>Music Press</publisher>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(pub_stmt.children.len(), 1);
    match &pub_stmt.children[0] {
        PubStmtChild::Publisher(p) => {
            assert!(!p.children.is_empty());
        }
        _ => panic!("expected Publisher child"),
    }
}

#[test]
fn pub_stmt_deserializes_multiple_children() {
    let xml = r#"<pubStmt xml:id="ps1">
        <publisher xml:id="pub1">Music Press</publisher>
        <pubPlace>Vienna</pubPlace>
        <date>1800</date>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(pub_stmt.common.xml_id, Some("ps1".to_string()));
    assert_eq!(pub_stmt.children.len(), 3);
}

#[test]
fn pub_stmt_deserializes_address_child() {
    let xml = r#"<pubStmt>
        <address>
            <addrLine>123 Music Street</addrLine>
        </address>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(pub_stmt.children.len(), 1);
    match &pub_stmt.children[0] {
        PubStmtChild::Address(addr) => {
            assert!(!addr.children.is_empty());
        }
        _ => panic!("expected Address child"),
    }
}

#[test]
fn publisher_deserializes_with_corp_name_child() {
    use tusk_model::elements::{CorpNameChild, Publisher, PublisherChild};

    let xml = r#"<publisher>
        <corpName role="publisher">Musikwissenschaftliches Seminar, Detmold</corpName>
    </publisher>"#;
    let publisher = Publisher::from_mei_str(xml).expect("should deserialize");
    assert_eq!(publisher.children.len(), 1);
    match &publisher.children[0] {
        PublisherChild::CorpName(cn) => {
            assert_eq!(cn.name.role.len(), 1);
            assert_eq!(cn.children.len(), 1);
            match &cn.children[0] {
                CorpNameChild::Text(text) => {
                    assert_eq!(text.trim(), "Musikwissenschaftliches Seminar, Detmold");
                }
                _ => panic!("expected Text child in corpName"),
            }
        }
        _ => panic!("expected CorpName child"),
    }
}

#[test]
fn publisher_deserializes_with_mixed_content() {
    use tusk_model::elements::{Publisher, PublisherChild};

    let xml = r#"<publisher>Some text before <corpName>My Corp</corpName> and after</publisher>"#;
    let publisher = Publisher::from_mei_str(xml).expect("should deserialize");
    assert_eq!(publisher.children.len(), 3);
    match &publisher.children[0] {
        PublisherChild::Text(text) => {
            // XML whitespace before the corpName tag
            assert!(text.contains("Some text before"));
        }
        _ => panic!("expected Text child first"),
    }
    match &publisher.children[1] {
        PublisherChild::CorpName(_) => {}
        _ => panic!("expected CorpName child second"),
    }
    match &publisher.children[2] {
        PublisherChild::Text(text) => {
            // Text after the corpName
            assert!(text.contains("and after"));
        }
        _ => panic!("expected Text child third"),
    }
}

// ========== SourceDesc Tests ==========

#[test]
fn source_desc_deserializes_empty_element() {
    let xml = r#"<sourceDesc/>"#;
    let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
    assert!(source_desc.common.xml_id.is_none());
    assert!(source_desc.children.is_empty());
}

#[test]
fn source_desc_deserializes_source_child() {
    let xml = r#"<sourceDesc>
        <source xml:id="src1"/>
    </sourceDesc>"#;
    let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
    assert_eq!(source_desc.children.len(), 1);
    match &source_desc.children[0] {
        SourceDescChild::Source(src) => {
            assert_eq!(src.common.xml_id, Some("src1".to_string()));
        }
        _ => panic!("expected Source child"),
    }
}

// ========== EncodingDesc tests ==========

#[test]
fn encoding_desc_deserializes_empty_element() {
    let xml = r#"<encodingDesc/>"#;
    let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");
    assert!(encoding_desc.common.xml_id.is_none());
    assert!(encoding_desc.children.is_empty());
}

#[test]
fn encoding_desc_deserializes_app_info_child() {
    let xml = r#"<encodingDesc>
        <appInfo xml:id="ai1"/>
    </encodingDesc>"#;
    let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");
    assert_eq!(encoding_desc.children.len(), 1);
    match &encoding_desc.children[0] {
        EncodingDescChild::AppInfo(ai) => {
            assert_eq!(ai.common.xml_id, Some("ai1".to_string()));
        }
        _ => panic!("expected AppInfo child"),
    }
}

// ========== AppInfo tests ==========

#[test]
fn app_info_deserializes_empty_element() {
    let xml = r#"<appInfo/>"#;
    let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");
    assert!(app_info.common.xml_id.is_none());
    assert!(app_info.children.is_empty());
}

#[test]
fn app_info_deserializes_application_child() {
    let xml = r#"<appInfo>
        <application xml:id="app1">
            <name>Tusk</name>
        </application>
    </appInfo>"#;
    let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");
    assert_eq!(app_info.children.len(), 1);
    match &app_info.children[0] {
        AppInfoChild::Application(app) => {
            assert_eq!(app.common.xml_id, Some("app1".to_string()));
        }
        _ => panic!("expected Application child"),
    }
}

// ========== EditorialDecl tests ==========

#[test]
fn editorial_decl_deserializes_empty_element() {
    let xml = r#"<editorialDecl/>"#;
    let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");
    assert!(editorial_decl.common.xml_id.is_none());
    assert!(editorial_decl.children.is_empty());
}

// ========== ProjectDesc tests ==========

#[test]
fn project_desc_deserializes_empty_element() {
    let xml = r#"<projectDesc/>"#;
    let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");
    assert!(project_desc.common.xml_id.is_none());
    assert!(project_desc.children.is_empty());
}

#[test]
fn project_desc_deserializes_with_p_child() {
    let xml = r#"<projectDesc xml:id="pd1">
        <p>This project aims to create a digital edition.</p>
    </projectDesc>"#;
    let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");
    assert_eq!(project_desc.children.len(), 1);
    match &project_desc.children[0] {
        ProjectDescChild::P(p) => {
            assert_eq!(p.children.len(), 1);
            match &p.children[0] {
                PChild::Text(text) => {
                    assert!(text.contains("digital edition"));
                }
                _ => panic!("expected Text child"),
            }
        }
        _ => panic!("expected P child"),
    }
}

// ========== Integration tests ==========

#[test]
fn mei_head_file_desc_title_stmt_integration() {
    let xml = r#"<meiHead xml:id="h1">
        <fileDesc xml:id="fd1">
            <titleStmt xml:id="ts1">
                <title>Symphony No. 5</title>
                <creator>Ludwig van Beethoven</creator>
            </titleStmt>
        </fileDesc>
    </meiHead>"#;
    let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
    assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
    assert_eq!(mei_head.children.len(), 1);
    match &mei_head.children[0] {
        MeiHeadChild::FileDesc(fd) => {
            assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
            assert_eq!(fd.children.len(), 1);
            match &fd.children[0] {
                FileDescChild::TitleStmt(ts) => {
                    assert_eq!(ts.common.xml_id, Some("ts1".to_string()));
                    assert_eq!(ts.children.len(), 2);
                }
                _ => panic!("expected TitleStmt child"),
            }
        }
        _ => panic!("expected FileDesc child"),
    }
}

// ========== Deprecated element migration tests ==========

#[test]
fn title_stmt_deserializes_deprecated_composer_as_creator() {
    let xml = r#"<titleStmt>
        <title>Walzer G-Dur</title>
        <composer>Dionisio Aguado</composer>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(title_stmt.children.len(), 2);

    // First child should be title
    assert!(matches!(&title_stmt.children[0], TitleStmtChild::Title(_)));

    // Second child should be Creator (migrated from composer)
    match &title_stmt.children[1] {
        TitleStmtChild::Creator(creator) => {
            // Verify the role was set to composer (Cmp)
            assert_eq!(creator.name.role.len(), 1);
            match &creator.name.role[0] {
                tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                    assert_eq!(
                        *role,
                        tusk_model::generated::data::DataMarcrelatorsBasic::Cmp
                    );
                }
                _ => panic!("expected DataMarcrelatorsBasic role"),
            }
            // Verify text content was captured
            assert!(!creator.children.is_empty());
        }
        _ => panic!("expected Creator child (migrated from composer)"),
    }
}

#[test]
fn title_stmt_deserializes_deprecated_lyricist_as_creator() {
    let xml = r#"<titleStmt>
        <title>A Song</title>
        <lyricist>A Poet</lyricist>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(title_stmt.children.len(), 2);

    match &title_stmt.children[1] {
        TitleStmtChild::Creator(creator) => {
            assert_eq!(creator.name.role.len(), 1);
            match &creator.name.role[0] {
                tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                    assert_eq!(
                        *role,
                        tusk_model::generated::data::DataMarcrelatorsBasic::Lyr
                    );
                }
                _ => panic!("expected DataMarcrelatorsBasic role"),
            }
        }
        _ => panic!("expected Creator child (migrated from lyricist)"),
    }
}

#[test]
fn title_stmt_deserializes_deprecated_arranger_as_creator() {
    let xml = r#"<titleStmt>
        <title>Arranged Work</title>
        <arranger>An Arranger</arranger>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(title_stmt.children.len(), 2);

    match &title_stmt.children[1] {
        TitleStmtChild::Creator(creator) => {
            assert_eq!(creator.name.role.len(), 1);
            match &creator.name.role[0] {
                tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                    assert_eq!(
                        *role,
                        tusk_model::generated::data::DataMarcrelatorsBasic::Arr
                    );
                }
                _ => panic!("expected DataMarcrelatorsBasic role"),
            }
        }
        _ => panic!("expected Creator child (migrated from arranger)"),
    }
}

#[test]
fn title_stmt_deserializes_deprecated_author_as_creator() {
    let xml = r#"<titleStmt>
        <title>A Text Work</title>
        <author>An Author</author>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");
    assert_eq!(title_stmt.children.len(), 2);

    match &title_stmt.children[1] {
        TitleStmtChild::Creator(creator) => {
            assert_eq!(creator.name.role.len(), 1);
            match &creator.name.role[0] {
                tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                    assert_eq!(
                        *role,
                        tusk_model::generated::data::DataMarcrelatorsBasic::Aut
                    );
                }
                _ => panic!("expected DataMarcrelatorsBasic role"),
            }
        }
        _ => panic!("expected Creator child (migrated from author)"),
    }
}

// ========== Title mixed content tests ==========

#[test]
fn title_deserializes_with_title_part_child() {
    use tusk_model::elements::{Title, TitleChild, TitlePartChild};

    // Note: @type attribute on titlePart is a local attribute not yet generated in the model
    // (tracked as CODEGEN_BUG in tasks_mei_roundtrip.md)
    let xml = r#"<title>Walzer G-Dur<titlePart>an electronic transcription</titlePart></title>"#;
    let title = Title::from_mei_str(xml).expect("should deserialize");

    // Should have 2 children: text "Walzer G-Dur" and titlePart element
    assert_eq!(title.children.len(), 2);

    // First child should be text
    match &title.children[0] {
        TitleChild::Text(text) => {
            assert_eq!(text, "Walzer G-Dur");
        }
        _ => panic!("expected Text child first"),
    }

    // Second child should be titlePart
    match &title.children[1] {
        TitleChild::TitlePart(tp) => {
            assert_eq!(tp.children.len(), 1);
            match &tp.children[0] {
                TitlePartChild::Text(text) => {
                    assert_eq!(text, "an electronic transcription");
                }
                _ => panic!("expected Text child in titlePart"),
            }
        }
        _ => panic!("expected TitlePart child second"),
    }
}

#[test]
fn title_deserializes_text_only() {
    use tusk_model::elements::{Title, TitleChild};

    let xml = r#"<title>Simple Title</title>"#;
    let title = Title::from_mei_str(xml).expect("should deserialize");

    assert_eq!(title.children.len(), 1);
    match &title.children[0] {
        TitleChild::Text(text) => {
            assert_eq!(text, "Simple Title");
        }
        _ => panic!("expected Text child"),
    }
}

#[test]
fn title_deserializes_empty_element() {
    use tusk_model::elements::Title;

    let xml = r#"<title/>"#;
    let title = Title::from_mei_str(xml).expect("should deserialize");
    assert!(title.children.is_empty());
}

// ========== Funder tests (via TitleStmt wrapper) ==========

#[test]
fn funder_deserializes_with_corp_name_child() {
    use tusk_model::elements::{CorpNameChild, FunderChild};

    let xml = r#"<titleStmt>
      <title>Test</title>
      <funder>
        <corpName role="funder" codedval="2007744-0" auth.uri="http://d-nb.info/gnd/" auth="GND">German Research Foundation</corpName>
      </funder>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

    // Find the funder child
    let funder = title_stmt
        .children
        .iter()
        .find_map(|c| {
            if let TitleStmtChild::Funder(f) = c {
                Some(f)
            } else {
                None
            }
        })
        .expect("should have funder child");

    // Should have one child: corpName
    assert_eq!(funder.children.len(), 1);
    match &funder.children[0] {
        FunderChild::CorpName(cn) => {
            assert_eq!(cn.name.role.len(), 1);
            // Check the text content
            assert_eq!(cn.children.len(), 1);
            match &cn.children[0] {
                CorpNameChild::Text(text) => {
                    assert_eq!(text, "German Research Foundation");
                }
                _ => panic!("expected Text child in corpName"),
            }
        }
        _ => panic!("expected CorpName child, got {:?}", funder.children[0]),
    }
}

#[test]
fn funder_deserializes_corp_name_with_nested_address() {
    use tusk_model::elements::{CorpNameChild, FunderChild};

    let xml = r#"<titleStmt>
      <title>Test</title>
      <funder>
        <corpName role="funder">German Research Foundation
          <address>
            <addrLine>Kennedyallee 40</addrLine>
          </address>
        </corpName>
      </funder>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

    // Find the funder child
    let funder = title_stmt
        .children
        .iter()
        .find_map(|c| {
            if let TitleStmtChild::Funder(f) = c {
                Some(f)
            } else {
                None
            }
        })
        .expect("should have funder child");

    assert_eq!(funder.children.len(), 1);
    match &funder.children[0] {
        FunderChild::CorpName(cn) => {
            // Should have text and address children
            assert!(
                cn.children.len() >= 2,
                "expected at least 2 children (text + address)"
            );

            // Check for text content
            let has_text = cn
                .children
                .iter()
                .any(|c| matches!(c, CorpNameChild::Text(_)));
            assert!(has_text, "should have text content");

            // Check for address child
            let has_address = cn.children.iter().any(|c| {
                if let CorpNameChild::Address(addr) = c {
                    !addr.children.is_empty()
                } else {
                    false
                }
            });
            assert!(has_address, "should have address child with addrLine");
        }
        _ => panic!("expected CorpName child"),
    }
}

#[test]
fn funder_deserializes_text_only() {
    use tusk_model::elements::FunderChild;

    let xml = r#"<titleStmt>
      <title>Test</title>
      <funder>Anonymous Donor</funder>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

    // Find the funder child
    let funder = title_stmt
        .children
        .iter()
        .find_map(|c| {
            if let TitleStmtChild::Funder(f) = c {
                Some(f)
            } else {
                None
            }
        })
        .expect("should have funder child");

    assert_eq!(funder.children.len(), 1);
    match &funder.children[0] {
        FunderChild::Text(text) => {
            assert_eq!(text, "Anonymous Donor");
        }
        _ => panic!("expected Text child"),
    }
}

#[test]
fn funder_deserializes_empty_element() {
    let xml = r#"<titleStmt>
      <title>Test</title>
      <funder/>
    </titleStmt>"#;
    let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

    // Find the funder child
    let funder = title_stmt
        .children
        .iter()
        .find_map(|c| {
            if let TitleStmtChild::Funder(f) = c {
                Some(f)
            } else {
                None
            }
        })
        .expect("should have funder child");

    assert!(funder.children.is_empty());
}

// ========== Identifier tests (via PubStmt wrapper) ==========

#[test]
fn identifier_deserializes_with_ref_child() {
    use tusk_model::elements::IdentifierChild;

    let xml = r#"<pubStmt>
      <identifier>
        <ref target="http://music-encoding.org/Support/MEI_Sample_Collection"/>
      </identifier>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

    // Find the identifier child
    let identifier = pub_stmt
        .children
        .iter()
        .find_map(|c| {
            if let PubStmtChild::Identifier(id) = c {
                Some(id)
            } else {
                None
            }
        })
        .expect("should have identifier child");

    // Should have a ref child
    assert_eq!(identifier.children.len(), 1);
    match &identifier.children[0] {
        IdentifierChild::Ref(r) => {
            assert_eq!(r.pointing.target.len(), 1);
            assert_eq!(
                r.pointing.target[0].0,
                "http://music-encoding.org/Support/MEI_Sample_Collection"
            );
        }
        other => panic!("expected Ref child, got {:?}", other),
    }
}

#[test]
fn identifier_deserializes_text_only() {
    use tusk_model::elements::IdentifierChild;

    let xml = r#"<pubStmt>
      <identifier type="URI">http://example.com/test</identifier>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

    let identifier = pub_stmt
        .children
        .iter()
        .find_map(|c| {
            if let PubStmtChild::Identifier(id) = c {
                Some(id)
            } else {
                None
            }
        })
        .expect("should have identifier child");

    assert_eq!(identifier.children.len(), 1);
    match &identifier.children[0] {
        IdentifierChild::Text(text) => {
            assert_eq!(text.trim(), "http://example.com/test");
        }
        other => panic!("expected Text child, got {:?}", other),
    }
}

#[test]
fn identifier_deserializes_mixed_content() {
    use tusk_model::elements::IdentifierChild;

    let xml = r#"<pubStmt>
      <identifier>ISMN <ref target="http://ismn.org/">979-0-1234-5678-9</ref></identifier>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

    let identifier = pub_stmt
        .children
        .iter()
        .find_map(|c| {
            if let PubStmtChild::Identifier(id) = c {
                Some(id)
            } else {
                None
            }
        })
        .expect("should have identifier child");

    // Should have text followed by ref child
    assert_eq!(identifier.children.len(), 2);
    match &identifier.children[0] {
        IdentifierChild::Text(text) => {
            assert_eq!(text.trim(), "ISMN");
        }
        other => panic!("expected Text child first, got {:?}", other),
    }
    match &identifier.children[1] {
        IdentifierChild::Ref(r) => {
            assert_eq!(r.pointing.target.len(), 1);
            assert_eq!(r.pointing.target[0].0, "http://ismn.org/");
        }
        other => panic!("expected Ref child second, got {:?}", other),
    }
}

#[test]
fn identifier_deserializes_empty_element() {
    let xml = r#"<pubStmt>
      <identifier/>
    </pubStmt>"#;
    let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

    let identifier = pub_stmt
        .children
        .iter()
        .find_map(|c| {
            if let PubStmtChild::Identifier(id) = c {
                Some(id)
            } else {
                None
            }
        })
        .expect("should have identifier child");

    assert!(identifier.children.is_empty());
}

// ============================================================================
// Bibl element tests
// ============================================================================

#[test]
fn bibl_deserializes_text_content() {
    use tusk_model::elements::BiblChild;

    // bibl with text content and attributes (similar to Aguado_Walzer_G-major.mei line 122)
    let xml = r#"<source>
      <bibl xml:id="OCLC_DDC" target="http://example.com">OCLC_DDC</bibl>
    </source>"#;

    let source = Source::from_mei_str(xml).expect("should deserialize");

    let bibl = source
        .children
        .iter()
        .find_map(|c| {
            if let SourceChild::Bibl(b) = c {
                Some(b)
            } else {
                None
            }
        })
        .expect("should have bibl child");

    // Check attributes
    assert_eq!(bibl.common.xml_id, Some("OCLC_DDC".to_string()));
    assert!(!bibl.pointing.target.is_empty());

    // Check text content is preserved
    assert_eq!(bibl.children.len(), 1);
    match &bibl.children[0] {
        BiblChild::Text(text) => assert_eq!(text, "OCLC_DDC"),
        other => panic!("expected Text child, got {:?}", other),
    }
}

#[test]
fn bibl_deserializes_mixed_content() {
    use tusk_model::elements::BiblChild;

    // bibl with both child elements and text content
    let xml = r#"<source>
      <bibl>
        <title>Some Title</title>
        with some text
      </bibl>
    </source>"#;

    let source = Source::from_mei_str(xml).expect("should deserialize");

    let bibl = source
        .children
        .iter()
        .find_map(|c| {
            if let SourceChild::Bibl(b) = c {
                Some(b)
            } else {
                None
            }
        })
        .expect("should have bibl child");

    // Should have both title element and text content
    assert_eq!(bibl.children.len(), 2);

    // First child should be Title
    match &bibl.children[0] {
        BiblChild::Title(_t) => {
            // Title text check can be added if needed
        }
        other => panic!("expected Title child first, got {:?}", other),
    }

    // Second child should be Text
    match &bibl.children[1] {
        BiblChild::Text(text) => {
            assert!(text.contains("with some text"));
        }
        other => panic!("expected Text child second, got {:?}", other),
    }
}

#[test]
fn bibl_deserializes_editor_child() {
    use tusk_model::elements::BiblChild;

    // bibl with editor child element
    let xml = r#"<source>
      <bibl>
        <title>Test Work</title>
        <editor>John Smith</editor>
      </bibl>
    </source>"#;

    let source = Source::from_mei_str(xml).expect("should deserialize");

    let bibl = source
        .children
        .iter()
        .find_map(|c| {
            if let SourceChild::Bibl(b) = c {
                Some(b)
            } else {
                None
            }
        })
        .expect("should have bibl child");

    // Should have title and editor
    assert_eq!(bibl.children.len(), 2);

    // First child should be Title
    assert!(matches!(&bibl.children[0], BiblChild::Title(_)));

    // Second child should be Editor
    match &bibl.children[1] {
        BiblChild::Editor(editor) => {
            // Check text content
            assert!(editor.children.iter().any(|c| {
                if let tusk_model::elements::EditorChild::Text(t) = c {
                    t.contains("John Smith")
                } else {
                    false
                }
            }));
        }
        other => panic!("expected Editor child, got {:?}", other),
    }
}

#[test]
fn bibl_deserializes_deprecated_librettist_as_creator() {
    use tusk_model::elements::BiblChild;

    // bibl with deprecated librettist element (MEI 5.x)
    let xml = r#"<source>
      <bibl>
        <title>Test Work</title>
        <librettist>
          <persName role="librettist">John Doe</persName>
        </librettist>
      </bibl>
    </source>"#;

    let source = Source::from_mei_str(xml).expect("should deserialize");

    let bibl = source
        .children
        .iter()
        .find_map(|c| {
            if let SourceChild::Bibl(b) = c {
                Some(b)
            } else {
                None
            }
        })
        .expect("should have bibl child");

    // Should have title and creator (migrated from librettist)
    assert_eq!(bibl.children.len(), 2);

    // First child should be Title
    assert!(matches!(&bibl.children[0], BiblChild::Title(_)));

    // Second child should be Creator (migrated from librettist)
    match &bibl.children[1] {
        BiblChild::Creator(creator) => {
            // Verify the role was set to librettist (Lbt)
            assert_eq!(creator.name.role.len(), 1);
            match &creator.name.role[0] {
                tusk_model::generated::data::DataRelators::DataMarcrelatorsBasic(role) => {
                    assert_eq!(
                        *role,
                        tusk_model::generated::data::DataMarcrelatorsBasic::Lbt
                    );
                }
                _ => panic!("expected DataMarcrelatorsBasic role"),
            }
            // Verify persName child was parsed
            assert!(
                creator
                    .children
                    .iter()
                    .any(|c| { matches!(c, tusk_model::elements::CreatorChild::PersName(_)) })
            );
        }
        other => panic!(
            "expected Creator child (migrated from librettist), got {:?}",
            other
        ),
    }
}

#[test]
fn editor_deserializes_pers_name_child() {
    use tusk_model::elements::BiblChild;

    // editor with persName child element
    let xml = r#"<source>
      <bibl>
        <editor>
          <persName>John Smith</persName>
        </editor>
      </bibl>
    </source>"#;

    let source = Source::from_mei_str(xml).expect("should deserialize");

    let bibl = source
        .children
        .iter()
        .find_map(|c| {
            if let SourceChild::Bibl(b) = c {
                Some(b)
            } else {
                None
            }
        })
        .expect("should have bibl child");

    // Should have one editor
    assert_eq!(bibl.children.len(), 1);

    // First child should be Editor
    match &bibl.children[0] {
        BiblChild::Editor(editor) => {
            // Should have one persName child
            assert_eq!(editor.children.len(), 1);
            match &editor.children[0] {
                EditorChild::PersName(pers_name) => {
                    // Check text content of persName
                    assert!(pers_name.children.iter().any(|c| {
                        if let tusk_model::elements::PersNameChild::Text(t) = c {
                            t.contains("John Smith")
                        } else {
                            false
                        }
                    }));
                }
                other => panic!("expected PersName child, got {:?}", other),
            }
        }
        other => panic!("expected Editor child, got {:?}", other),
    }
}

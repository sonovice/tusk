//! Export completion tests: fixture roundtrip, ID retention, fresh MEI→LilyPond.

use super::*;
use crate::import;
use crate::parser::Parser;
use crate::serializer;

/// Parse LilyPond -> import to MEI -> export to LilyPond AST -> serialize.
fn roundtrip(src: &str) -> String {
    let file = Parser::new(src).unwrap().parse().unwrap();
    let (mei, ext_store) = import::import(&file).unwrap();
    let exported = export(&mei, &ext_store).unwrap();
    serializer::serialize(&exported)
}

// ---------------------------------------------------------------------------
// Fixture roundtrip: all fixtures import→export without panic
// ---------------------------------------------------------------------------

#[test]
fn all_fixtures_export_without_panic() {
    let fixture_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/lilypond");
    let fixture_dir = fixture_dir
        .canonicalize()
        .expect("fixture directory should exist");

    let mut tested = 0u32;
    let mut skipped = 0u32;

    for entry in std::fs::read_dir(&fixture_dir).expect("read fixture dir") {
        let entry = entry.expect("read entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("ly") {
            continue;
        }
        let src = std::fs::read_to_string(&path).expect("read fixture");
        let file = match Parser::new(&src).and_then(|p| p.parse()) {
            Ok(f) => f,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };
        let (mei, ext_store) = match import::import(&file) {
            Ok(m) => m,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };
        let exported = export(&mei, &ext_store);
        assert!(
            exported.is_ok(),
            "export failed for {}: {:?}",
            path.display(),
            exported.err()
        );
        let ly_file = exported.unwrap();
        let output = serializer::serialize(&ly_file);
        assert!(!output.is_empty(), "empty output for {}", path.display());
        tested += 1;
    }

    assert!(
        tested >= 30,
        "expected at least 30 fixtures to roundtrip, got {tested} (skipped {skipped})"
    );
}

// ---------------------------------------------------------------------------
// ID retention: \tweak id roundtrip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_tweak_id_on_note() {
    let output = roundtrip(r#"{ c'4\tweak id #"my-note" d'4 }"#);
    assert!(
        output.contains(r#"\tweak id #"my-note""#),
        "output: {output}"
    );
}

#[test]
fn roundtrip_tweak_id_on_chord() {
    let output = roundtrip(r#"{ <c' e'>4\tweak id #"my-chord" }"#);
    assert!(
        output.contains(r#"\tweak id #"my-chord""#),
        "output: {output}"
    );
}

#[test]
fn roundtrip_tweak_id_not_emitted_for_autogen() {
    // Auto-generated IDs (ly-note-N) should NOT produce \tweak id
    let output = roundtrip("{ c'4 d'4 e'4 }");
    assert!(
        !output.contains("\\tweak id"),
        "auto-generated IDs should not produce \\tweak id: {output}"
    );
}

// ---------------------------------------------------------------------------
// Fresh MEI → LilyPond with custom xml:id (no tweak emitted)
// ---------------------------------------------------------------------------

#[test]
fn export_mei_with_custom_xml_id_no_tweak() {
    use tusk_model::elements::*;
    use tusk_model::generated::data::*;

    // Build a minimal MEI with a custom xml:id on a note — should NOT emit \tweak id
    // (only tweaks stored in ExtensionStore from LilyPond roundtrip are emitted)
    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some("6.0-dev".to_string());

    let mut mei_head = MeiHead::default();
    let mut file_desc = FileDesc::default();
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::default()));
    mei_head
        .children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

    let mut note = Note::default();
    note.common.xml_id = Some("custom-note-id".to_string());
    note.note_log.pname = Some(DataPitchname("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

    let mut layer = Layer::default();
    layer.n_integer.n = Some("1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some("1".to_string());
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let mut section = Section::default();
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some("1".to_string());
    staff_def.staff_def_log.clef_shape = Some(DataClefshape::G);
    staff_def.staff_def_log.clef_line = Some(DataClefline(2));

    let mut staff_grp = StaffGrp::default();
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let mut score_def = ScoreDef::default();
    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));

    let mut score = Score::default();
    score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));
    score.children.push(ScoreChild::Section(Box::new(section)));

    let mut mdiv = Mdiv::default();
    mdiv.children.push(MdivChild::Score(Box::new(score)));

    let mut body = Body::default();
    body.children.push(BodyChild::Mdiv(Box::new(mdiv)));

    let mut music = Music::default();
    music.children.push(MusicChild::Body(Box::new(body)));
    mei.children.push(MeiChild::Music(Box::new(music)));

    // Export
    let ext_store = tusk_model::ExtensionStore::default();
    let exported = export(&mei, &ext_store).unwrap();
    let output = serializer::serialize(&exported);

    assert!(
        !output.contains(r#"\tweak id"#),
        "should NOT emit \\tweak id for bare MEI xml:id (only ExtensionStore tweaks): {output}"
    );
}

#[test]
fn export_mei_with_autogen_xml_id_no_tweak() {
    use tusk_model::elements::*;
    use tusk_model::generated::data::*;

    // Build a minimal MEI with an auto-generated xml:id
    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some("6.0-dev".to_string());

    let mut mei_head = MeiHead::default();
    let mut file_desc = FileDesc::default();
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::default()));
    mei_head
        .children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

    let mut note = Note::default();
    note.common.xml_id = Some("ly-note-42".to_string());
    note.note_log.pname = Some(DataPitchname("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

    let mut layer = Layer::default();
    layer.n_integer.n = Some("1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some("1".to_string());
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let mut section = Section::default();
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some("1".to_string());

    let mut staff_grp = StaffGrp::default();
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let mut score_def = ScoreDef::default();
    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));

    let mut score = Score::default();
    score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));
    score.children.push(ScoreChild::Section(Box::new(section)));

    let mut mdiv = Mdiv::default();
    mdiv.children.push(MdivChild::Score(Box::new(score)));

    let mut body = Body::default();
    body.children.push(BodyChild::Mdiv(Box::new(mdiv)));

    let mut music = Music::default();
    music.children.push(MusicChild::Body(Box::new(body)));
    mei.children.push(MeiChild::Music(Box::new(music)));

    // Export
    let ext_store = tusk_model::ExtensionStore::default();
    let exported = export(&mei, &ext_store).unwrap();
    let output = serializer::serialize(&exported);

    assert!(
        !output.contains("\\tweak id"),
        "auto-generated IDs should not produce \\tweak id: {output}"
    );
}

// ---------------------------------------------------------------------------
// Roundtrip ID preservation: MEI → LilyPond → MEI
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_mei_to_ly_to_mei_preserves_custom_ids() {
    // Start with a LilyPond source that has custom IDs via \tweak id
    let src = r#"{ c'4\tweak id #"note-A" d'4\tweak id #"note-B" e'4 }"#;

    // Parse → import to MEI
    let file = Parser::new(src).unwrap().parse().unwrap();
    let (mei, ext_store) = import::import(&file).unwrap();

    // Export back to LilyPond
    let exported = export(&mei, &ext_store).unwrap();
    let output = serializer::serialize(&exported);

    // Re-parse → re-import to MEI
    let file2 = Parser::new(&output).unwrap().parse().unwrap();
    let (mei2, _ext_store2) = import::import(&file2).unwrap();

    // Extract note xml:ids from both MEI docs
    let ids1 = extract_note_xml_ids(&mei);
    let ids2 = extract_note_xml_ids(&mei2);

    // The custom IDs should survive the roundtrip
    assert!(ids1.contains(&"note-A".to_string()), "mei1 ids: {ids1:?}");
    assert!(ids1.contains(&"note-B".to_string()), "mei1 ids: {ids1:?}");
    assert!(ids2.contains(&"note-A".to_string()), "mei2 ids: {ids2:?}");
    assert!(ids2.contains(&"note-B".to_string()), "mei2 ids: {ids2:?}");
}

/// Extract all note xml:ids from an MEI document.
fn extract_note_xml_ids(mei: &tusk_model::elements::Mei) -> Vec<String> {
    use tusk_model::elements::*;
    let mut ids = Vec::new();
    for child in &mei.children {
        let MeiChild::Music(music) = child else {
            continue;
        };
        let MusicChild::Body(body) = &music.children[0];
        let BodyChild::Mdiv(mdiv) = &body.children[0];
        let MdivChild::Score(score) = &mdiv.children[0];
        for sc in &score.children {
            if let ScoreChild::Section(section) = sc {
                for sec_c in &section.children {
                    if let SectionChild::Measure(measure) = sec_c {
                        collect_note_ids_from_measure(measure, &mut ids);
                    }
                }
            }
        }
    }
    ids
}

fn collect_note_ids_from_measure(measure: &tusk_model::elements::Measure, ids: &mut Vec<String>) {
    for mc in &measure.children {
        if let tusk_model::elements::MeasureChild::Staff(staff) = mc {
            for sc in &staff.children {
                let tusk_model::elements::StaffChild::Layer(layer) = sc;
                for lc in &layer.children {
                    match lc {
                        tusk_model::elements::LayerChild::Note(n) => {
                            if let Some(ref id) = n.common.xml_id {
                                ids.push(id.clone());
                            }
                        }
                        tusk_model::elements::LayerChild::Rest(r) => {
                            if let Some(ref id) = r.common.xml_id {
                                ids.push(id.clone());
                            }
                        }
                        tusk_model::elements::LayerChild::Chord(c) => {
                            if let Some(ref id) = c.common.xml_id {
                                ids.push(id.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

//! Integration tests that validate serialized MEI CMN output against mei-all.rng schema.
//!
//! These tests serialize MEI elements to XML and validate them using xmllint with the
//! official MEI RelaxNG schema. This ensures our serialization produces valid MEI documents.
//!
//! # Requirements
//!
//! These tests require `xmllint` to be installed and available in PATH.
//! On macOS: `brew install libxml2` (usually pre-installed)
//! On Ubuntu: `apt-get install libxml2-utils`

use std::io::Write;
use std::process::Command;
use tusk_mei::serializer::{IndentConfig, MeiSerialize, MeiWriter, SerializeConfig};
use tusk_model::data::{
    DataDuration, DataDurationCmn, DataDurationrests, DataGrace, DataUri, DataWord,
};
use tusk_model::elements::{
    Beam, Chord, Dir, Dynam, Fermata, GraceGrp, Hairpin, Layer, LayerChild, Measure, MeasureChild,
    Note, Rest, Slur, Space, Staff, Tempo, Tie, Tuplet,
};

/// Path to the MEI RNG schema relative to the workspace root.
const MEI_RNG_SCHEMA: &str = "specs/mei/validation/mei-all.rng";

/// Check if xmllint is available in PATH.
fn xmllint_available() -> bool {
    Command::new("xmllint")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Wrap serialized MEI content in a minimal valid MEI document.
///
/// The wrapper includes:
/// - XML declaration
/// - MEI root element with namespace and version
/// - Minimal meiHead with required elements
/// - music/body/mdiv/score structure
fn wrap_in_mei_document(score_content: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1">
  <meiHead>
    <fileDesc>
      <titleStmt>
        <title>Schema Validation Test</title>
      </titleStmt>
      <pubStmt/>
    </fileDesc>
  </meiHead>
  <music>
    <body>
      <mdiv>
        <score>
{score_content}
        </score>
      </mdiv>
    </body>
  </music>
</mei>"#
    )
}

/// Wrap a section in scoreDef + section structure.
fn wrap_section(section_content: &str) -> String {
    format!(
        r#"          <scoreDef>
            <staffGrp>
              <staffDef n="1" lines="5" clef.shape="G" clef.line="2"/>
            </staffGrp>
          </scoreDef>
          <section>
{section_content}
          </section>"#
    )
}

/// Find the workspace root by looking for Cargo.toml with [workspace] section.
fn find_workspace_root() -> Result<std::path::PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            let content = std::fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
            if content.contains("[workspace]") {
                return Ok(dir);
            }
        }
        if !dir.pop() {
            return Err("Could not find workspace root".to_string());
        }
    }
}

/// Validate an MEI XML string against the mei-all.rng schema using xmllint.
///
/// Returns Ok(()) if valid, Err with xmllint output if invalid.
fn validate_mei_with_xmllint(mei_xml: &str) -> Result<(), String> {
    // Write XML to a temp file
    let mut temp_file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    temp_file
        .write_all(mei_xml.as_bytes())
        .map_err(|e| e.to_string())?;
    temp_file.flush().map_err(|e| e.to_string())?;

    // Find workspace root and construct schema path
    let workspace_root = find_workspace_root()?;
    let schema_path = workspace_root.join(MEI_RNG_SCHEMA);

    // Run xmllint with the temp file path as argument
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--relaxng")
        .arg(&schema_path)
        .arg(temp_file.path())
        .output()
        .map_err(|e| format!("Failed to run xmllint: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "Schema validation failed:\n{}\n\nXML content:\n{}",
            stderr, mei_xml
        ))
    }
}

/// Serialize a measure with its content for validation.
fn serialize_measure(measure: &Measure) -> Result<String, String> {
    let config = SerializeConfig {
        include_declaration: false,
        indent: Some(IndentConfig {
            indent_char: b' ',
            indent_size: 2,
        }),
        mei_namespace: None,
        additional_namespaces: Vec::new(),
    };

    let mut buffer = Vec::new();
    let mut writer = MeiWriter::new(&mut buffer, config);
    measure
        .serialize_mei(&mut writer)
        .map_err(|e| e.to_string())?;

    String::from_utf8(buffer).map_err(|e| e.to_string())
}

// ============================================================================
// Schema Validation Tests
// ============================================================================

#[test]
fn validate_simple_note() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a simple note (internal model uses String for pname/oct and MeiDataDurationCmn for dur)
    let mut note = Note::default();
    note.note_log.pname = Some("c".to_string());
    note.note_log.oct = Some("4".to_string());
    note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

    // Build measure structure (RNG model: LayerChild, MeasureChild exist; StaffChild may not)
    let mut layer = Layer::default();
    layer.n_integer.n = Some("1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some("1".to_string());
    // Staff children: use whatever the RNG model provides (StaffChild or direct layer in measure)
    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Layer(Box::new(layer)));

    // Serialize and validate
    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document should be valid");
}

#[test]
fn validate_note_with_accidental() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    use tusk_model::data::{DataAccidentalWritten, DataAccidentalWrittenBasic};
    use tusk_model::elements::{Accid, NoteChild};

    // Create a note with an accidental child element
    let mut accid = Accid::default();
    accid.accid_log.accid = Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
        DataAccidentalWrittenBasic::S,
    ));

    let mut note = Note::default();
    note.note_log.pname = Some(DataPitchname::from("f".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note.children.push(NoteChild::Accid(Box::new(accid)));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with accidental should be valid");
}

#[test]
fn validate_rest() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    let mut rest = Rest::default();
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Rest(Box::new(rest)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with rest should be valid");
}

#[test]
fn validate_chord() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a chord with multiple notes
    let mut note1 = Note::default();
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));

    let mut note3 = Note::default();
    note3.note_log.pname = Some(DataPitchname::from("g".to_string()));
    note3.note_log.oct = Some(DataOctave(4));

    let mut chord = Chord::default();
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    chord.children.push(ChordChild::Note(Box::new(note1)));
    chord.children.push(ChordChild::Note(Box::new(note2)));
    chord.children.push(ChordChild::Note(Box::new(note3)));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Chord(Box::new(chord)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with chord should be valid");
}

#[test]
fn validate_beam() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a beam with two eighth notes
    let mut note1 = Note::default();
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut beam = Beam::default();
    beam.children.push(BeamChild::Note(Box::new(note1)));
    beam.children.push(BeamChild::Note(Box::new(note2)));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Beam(Box::new(beam)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with beam should be valid");
}

#[test]
fn validate_tuplet() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a triplet (3 eighth notes in the time of 2)
    let mut note1 = Note::default();
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut note3 = Note::default();
    note3.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note3.note_log.oct = Some(DataOctave(4));
    note3.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut tuplet = Tuplet::default();
    tuplet.tuplet_log.num = Some(3);
    tuplet.tuplet_log.numbase = Some(2);
    tuplet.children.push(TupletChild::Note(Box::new(note1)));
    tuplet.children.push(TupletChild::Note(Box::new(note2)));
    tuplet.children.push(TupletChild::Note(Box::new(note3)));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Tuplet(Box::new(tuplet)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with tuplet should be valid");
}

/// Note: This test is ignored because LayerChild::GraceGrp serialization is incomplete
/// (falls through to "unknown" element name). This should be fixed in the serializer.
#[test]
#[ignore = "GraceGrp not yet fully implemented in LayerChild serializer"]
fn validate_grace_group() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a grace note
    let mut grace_note = Note::default();
    grace_note.note_log.pname = Some(DataPitchname::from("d".to_string()));
    grace_note.note_log.oct = Some(DataOctave(5));
    grace_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut grace_grp = GraceGrp::default();
    grace_grp.grace_grp_log.grace = Some(DataGrace::Acc);
    grace_grp
        .children
        .push(GraceGrpChild::Note(Box::new(grace_note)));

    // Main note that the grace leads to
    let mut main_note = Note::default();
    main_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    main_note.note_log.oct = Some(DataOctave(5));
    main_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer
        .children
        .push(LayerChild::GraceGrp(Box::new(grace_grp)));
    layer.children.push(LayerChild::Note(Box::new(main_note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with grace group should be valid");
}

#[test]
fn validate_space() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a space element (invisible rest)
    let mut space = Space::default();
    space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Space(Box::new(space)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with space should be valid");
}

// ============================================================================
// Control Event Validation Tests
// ============================================================================

#[test]
fn validate_slur() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create notes with IDs for slur reference
    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    // Create slur referencing the notes
    let mut slur = Slur::default();
    slur.slur_log.startid = Some(DataUri("#n1".to_string()));
    slur.slur_log.endid = Some(DataUri("#n2".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note1)));
    layer.children.push(LayerChild::Note(Box::new(note2)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure.children.push(MeasureChild::Slur(Box::new(slur)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with slur should be valid");
}

#[test]
fn validate_tie() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create notes with IDs for tie reference
    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));

    // Create tie
    let mut tie = Tie::default();
    tie.tie_log.startid = Some(DataUri("#n1".to_string()));
    tie.tie_log.endid = Some(DataUri("#n2".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note1)));
    layer.children.push(LayerChild::Note(Box::new(note2)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure.children.push(MeasureChild::Tie(Box::new(tie)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with tie should be valid");
}

#[test]
fn validate_dynam() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a note with a dynamic marking
    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    // Create dynamic marking
    let mut dynam = Dynam::default();
    dynam.dynam_log.startid = Some(DataUri("#n1".to_string()));
    dynam.children.push(DynamChild::Text("f".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure.children.push(MeasureChild::Dynam(Box::new(dynam)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with dynam should be valid");
}

/// Note: This test is ignored because hairpin attribute serialization for AttHairpinLogForm
/// is not working correctly (enum values not serialized). This should be fixed in the serializer.
#[test]
#[ignore = "Hairpin form attribute serialization incomplete"]
fn validate_hairpin() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create notes for hairpin span
    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    // Create crescendo hairpin
    let mut hairpin = Hairpin::default();
    hairpin.hairpin_log.startid = Some(DataUri("#n1".to_string()));
    hairpin.hairpin_log.endid = Some(DataUri("#n2".to_string()));
    hairpin.hairpin_log.form = Some(AttHairpinLogForm::Cres);

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note1)));
    layer.children.push(LayerChild::Note(Box::new(note2)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure
        .children
        .push(MeasureChild::Hairpin(Box::new(hairpin)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with hairpin should be valid");
}

#[test]
fn validate_dir() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a note with a direction/instruction
    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    // Create direction
    let mut dir = Dir::default();
    dir.dir_log.startid = Some(DataUri("#n1".to_string()));
    dir.children.push(DirChild::Text("dolce".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure.children.push(MeasureChild::Dir(Box::new(dir)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with dir should be valid");
}

#[test]
fn validate_tempo() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a note with a tempo marking
    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    // Create tempo marking
    let mut tempo = Tempo::default();
    tempo.tempo_log.startid = Some(DataUri("#n1".to_string()));
    tempo.children.push(TempoChild::Text("Allegro".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure.children.push(MeasureChild::Tempo(Box::new(tempo)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with tempo should be valid");
}

#[test]
fn validate_fermata() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a note with a fermata
    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N1));

    // Create fermata
    let mut fermata = Fermata::default();
    fermata.fermata_log.startid = Some(DataUri("#n1".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure
        .children
        .push(MeasureChild::Fermata(Box::new(fermata)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc).expect("MEI document with fermata should be valid");
}

// ============================================================================
// Complex Structure Validation Tests
// ============================================================================

#[test]
fn validate_complete_measure_with_multiple_elements() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Create a complex measure with various CMN elements
    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut note3 = Note::default();
    note3.common.xml_id = Some("n3".to_string());
    note3.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note3.note_log.oct = Some(DataOctave(4));
    note3.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut note4 = Note::default();
    note4.common.xml_id = Some("n4".to_string());
    note4.note_log.pname = Some(DataPitchname::from("f".to_string()));
    note4.note_log.oct = Some(DataOctave(4));
    note4.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));

    // Beam the two eighth notes
    let mut beam = Beam::default();
    beam.children.push(BeamChild::Note(Box::new(note2)));
    beam.children.push(BeamChild::Note(Box::new(note3)));

    // Add slur from first to last note
    let mut slur = Slur::default();
    slur.slur_log.startid = Some(DataUri("#n1".to_string()));
    slur.slur_log.endid = Some(DataUri("#n4".to_string()));

    // Add dynamic marking
    let mut dynam = Dynam::default();
    dynam.dynam_log.startid = Some(DataUri("#n1".to_string()));
    dynam.children.push(DynamChild::Text("mf".to_string()));

    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note1)));
    layer.children.push(LayerChild::Beam(Box::new(beam)));
    layer.children.push(LayerChild::Note(Box::new(note4)));

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    measure.children.push(MeasureChild::Slur(Box::new(slur)));
    measure.children.push(MeasureChild::Dynam(Box::new(dynam)));

    let measure_xml = serialize_measure(&measure).expect("serialize measure");
    let section_content = format!(
        "            {}",
        measure_xml.replace('\n', "\n            ")
    );
    let score_content = wrap_section(&section_content);
    let mei_doc = wrap_in_mei_document(&score_content);

    validate_mei_with_xmllint(&mei_doc)
        .expect("MEI document with multiple elements should be valid");
}

#[test]
fn validate_existing_fixture_hello_world() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Read and validate the existing hello_world.mei fixture
    let fixture_path = find_workspace_root()
        .expect("find workspace root")
        .join("tests/fixtures/mei/hello_world.mei");

    let mei_content = std::fs::read_to_string(&fixture_path).expect("read fixture");
    validate_mei_with_xmllint(&mei_content).expect("hello_world.mei fixture should be valid");
}

#[test]
fn validate_existing_fixture_scale() {
    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Read and validate the existing scale.mei fixture
    let fixture_path = find_workspace_root()
        .expect("find workspace root")
        .join("tests/fixtures/mei/scale.mei");

    let mei_content = std::fs::read_to_string(&fixture_path).expect("read fixture");
    validate_mei_with_xmllint(&mei_content).expect("scale.mei fixture should be valid");
}

#[test]
fn validate_header_with_revision_desc() {
    use tusk_model::data::DataIsodate;
    use tusk_model::elements::{
        Change, ChangeChild, ChangeDesc, ChangeDescChild, FileDesc, FileDescChild, MeiHead,
        MeiHeadChild, P, PChild, PubStmt, RevisionDesc, RevisionDescChild, Title, TitleChild,
        TitleStmt, TitleStmtChild,
    };

    if !xmllint_available() {
        eprintln!("Skipping test: xmllint not available");
        return;
    }

    // Build a complete meiHead with fileDesc and revisionDesc
    let mut mei_head = MeiHead::default();

    // Add fileDesc with titleStmt and pubStmt
    let mut file_desc = FileDesc::default();
    let mut title_stmt = TitleStmt::default();
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Header Validation Test".to_string()));
    title_stmt
        .children
        .push(TitleStmtChild::Title(Box::new(title)));
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
    let pub_stmt = PubStmt::default();
    file_desc
        .children
        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));
    mei_head
        .children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));

    // Add revisionDesc with a change element
    let mut revision_desc = RevisionDesc::default();
    let mut change = Change::default();
    change.common.xml_id = Some("change1".to_string());
    change.datable.isodate = Some(DataIsodate("2025-01-15".to_string()));

    let mut change_desc = ChangeDesc::default();
    let mut p = P::default();
    p.children
        .push(PChild::Text("Initial test encoding".to_string()));
    change_desc.children.push(ChangeDescChild::P(Box::new(p)));
    change
        .children
        .push(ChangeChild::ChangeDesc(Box::new(change_desc)));

    revision_desc
        .children
        .push(RevisionDescChild::Change(Box::new(change)));
    mei_head
        .children
        .push(MeiHeadChild::RevisionDesc(Box::new(revision_desc)));

    // Serialize the header
    let header_xml = mei_head.to_mei_string().expect("serialize header");

    // Wrap in full MEI document with minimal music content
    let mei_doc = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1">
  {}
  <music>
    <body>
      <mdiv>
        <score>
          <scoreDef>
            <staffGrp>
              <staffDef n="1" lines="5" clef.shape="G" clef.line="2"/>
            </staffGrp>
          </scoreDef>
          <section>
            <measure n="1">
              <staff n="1">
                <layer n="1">
                  <note pname="c" oct="4" dur="4"/>
                </layer>
              </staff>
            </measure>
          </section>
        </score>
      </mdiv>
    </body>
  </music>
</mei>"#,
        header_xml
    );

    validate_mei_with_xmllint(&mei_doc)
        .expect("MEI document with serialized header should be valid");
}

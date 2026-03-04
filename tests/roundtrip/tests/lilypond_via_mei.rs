//! Cross-format stabilization: LilyPond -> MEI -> LilyPond.
//!
//! Two-pass pipeline: LilyPond -> MEI1 -> LilyPond1 -> MEI2 -> LilyPond2.
//! Asserts LilyPond1 == LilyPond2 (string equality).

use tusk_roundtrip_tests::{
    load_fixture, lilypond_via_mei, assert_stable,
};

// ============================================================================
// Golden fixture tests (must pass)
// ============================================================================

fn assert_lilypond_via_mei(subdir: &str, name: &str) {
    let src = load_fixture(subdir, name);
    let ly1 = lilypond_via_mei(&src).unwrap_or_else(|e| panic!("{name}: {e}"));
    let ly2 = lilypond_via_mei(&ly1).unwrap_or_else(|e| panic!("{name} pass2: {e}"));
    assert_stable(&ly1, &ly2, name);
}

#[test]
fn golden_simple() {
    assert_lilypond_via_mei("lilypond", "fragment_score_minimal.ly");
}

#[test]
fn golden_fragment_score_minimal() {
    assert_lilypond_via_mei("lilypond", "fragment_score_minimal.ly");
}

#[test]
#[ignore]
fn debug_prelude6_slur() {
    let src = load_fixture("lilypond/MutopiaProject/ftp/CouperinF/ArtDeToucherLeClavecin/ArtDeToucherLeClavecin-lys", "prelude6.ly");

    // Parse LilyPond
    let file = tusk_lilypond::parser::Parser::new(&src)
        .and_then(|p| p.parse())
        .expect("parse");

    // Import to MEI
    let (mei, _ext) = tusk_lilypond::import::import(&file)
        .expect("lilypond_to_mei");

    // Export MEI to string
    let mei_str = tusk_mei::export(&mei)
        .expect("mei_export");

    // Find slurs and check if notes exist
    eprintln!("\n=== SLURS IN PASS 1 MEI (STAFF 2 ONLY) ===");
    for line in mei_str.lines() {
        if line.contains("<slur") && line.contains("staff=\"2\"") {
            eprintln!("{}", line);
            // Simple extraction: look for pattern startid="#ly-note-XXX"
            let parts: Vec<&str> = line.split("startid=\"#").collect();
            if parts.len() > 1 {
                let start_rest = parts[1];
                let start_id = start_rest.split("\"").next().unwrap_or("");
                eprintln!("  startid={}", start_id);
                let found_start = mei_str.contains(&format!("xml:id=\"{}\"", start_id));
                eprintln!("    exists in MEI: {}", found_start);
            }
            let parts: Vec<&str> = line.split("endid=\"#").collect();
            if parts.len() > 1 {
                let end_rest = parts[1];
                let end_id = end_rest.split("\"").next().unwrap_or("");
                eprintln!("  endid={}", end_id);
                let found_end = mei_str.contains(&format!("xml:id=\"{}\"", end_id));
                eprintln!("    exists in MEI: {}", found_end);
            }
        }
    }

    // Find which staff/measure contains ly-note-392 and ly-note-394
    eprintln!("\n=== LOCATING END NOTES (ly-note-392, ly-note-394) ===");
    for line in mei_str.lines() {
        if line.contains("ly-note-392") || line.contains("ly-note-394") {
            eprintln!("Found in line: {}", line.trim());
        }
    }

    // Find measures 1-5 of staff 2
    eprintln!("\n=== MEASURES 1-5 OF STAFF 2 (PASS 1 MEI) ===");
    let mut in_staff2 = false;
    let mut measure_count = 0;
    for line in mei_str.lines() {
        if line.contains("<staff n=\"2\"") {
            in_staff2 = true;
            eprintln!(">>> Enter staff 2");
        }
        if in_staff2 && line.contains("</staff>") {
            in_staff2 = false;
            eprintln!(">>> Exit staff 2");
            break;
        }
        if in_staff2 && line.contains("<measure") {
            measure_count += 1;
            if measure_count > 5 {
                eprintln!(">>> Skipping measures beyond 5");
                break;
            }
        }
        if in_staff2 && measure_count <= 5 && (line.contains("<measure") || line.contains("</measure>") || line.contains("xml:id=")) {
            eprintln!("{}", line.trim());
        }
    }

    // Now check pass 2: re-parse the pass 1 output and look for slurs
    eprintln!("\n=== PASS 2: PARSING PASS 1 LILYPOND OUTPUT ===");
    let ly1 = lilypond_via_mei(&src).expect("pass 1");

    let file2 = tusk_lilypond::parser::Parser::new(&ly1)
        .and_then(|p| p.parse())
        .expect("parse pass 2");

    let (mei2, _ext2) = tusk_lilypond::import::import(&file2)
        .expect("import pass 2");

    let mei2_str = tusk_mei::export(&mei2)
        .expect("export pass 2");

    eprintln!("\n=== SLURS IN PASS 2 MEI ===");
    for line in mei2_str.lines() {
        if line.contains("<slur") {
            eprintln!("{}", line);
        }
    }

    eprintln!("\n=== LOCATING NOTES IN PASS 2 MEI ===");
    for note_id in &["ly-note-392", "ly-note-394"] {
        eprintln!("Searching for {} in pass 2 MEI:", note_id);
        let found = mei2_str.contains(&format!("xml:id=\"{}\"", note_id));
        eprintln!("  Found: {}", found);
        if found {
            for line in mei2_str.lines() {
                if line.contains(&format!("xml:id=\"{}\"", note_id)) {
                    eprintln!("    {}", line.trim());
                }
            }
        }
    }

    eprintln!("\n=== MEASURES 1-5 OF STAFF 2 (PASS 2 MEI) ===");
    in_staff2 = false;
    measure_count = 0;
    for line in mei2_str.lines() {
        if line.contains("<staff n=\"2\"") {
            in_staff2 = true;
            eprintln!(">>> Enter staff 2");
        }
        if in_staff2 && line.contains("</staff>") {
            in_staff2 = false;
            eprintln!(">>> Exit staff 2");
            break;
        }
        if in_staff2 && line.contains("<measure") {
            measure_count += 1;
            if measure_count > 5 {
                eprintln!(">>> Skipping measures beyond 5");
                break;
            }
        }
        if in_staff2 && measure_count <= 5 && (line.contains("<measure") || line.contains("</measure>") || line.contains("xml:id=")) {
            eprintln!("{}", line.trim());
        }
    }
}

// ============================================================================
// Per-file generated tests (build.rs)
// ============================================================================

include!(concat!(env!("OUT_DIR"), "/cross_lilypond_via_mei.rs"));

include!(concat!(env!("OUT_DIR"), "/mutopia_lilypond_via_mei.rs"));

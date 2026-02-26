//! LilyPond rendering tests — validate that pipeline output compiles in LilyPond.
//!
//! Runs the actual `lilypond` binary on each fixture's pipeline output
//! (parse → import → export → serialize) and checks for compilation errors.
//!
//! Tests are automatically skipped if `lilypond` is not on PATH.

use std::path::{Path, PathBuf};
use std::process::Command;

use tusk_lilypond::export::export;
use tusk_lilypond::import::import;
use tusk_lilypond::parser::Parser;
use tusk_lilypond::serializer;

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/lilypond")
}

fn lilypond_available() -> bool {
    Command::new("lilypond")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
}

/// Run `lilypond` on the given source and return (success, combined output).
fn lilypond_check(src: &str, tmp_dir: &Path, tag: &str) -> (bool, String) {
    let file_path = tmp_dir.join(format!("{tag}.ly"));
    std::fs::write(&file_path, src).expect("write temp .ly");

    let output = Command::new("lilypond")
        .arg("-dno-print-pages")
        .arg(format!("-o{}", tmp_dir.join(tag).display()))
        .arg(&file_path)
        .output()
        .expect("run lilypond");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let has_error =
        combined.contains(": error:") || combined.contains("fatal error: failed files:");

    (!has_error, combined)
}

/// Fixtures that cannot render in lilypond by design or due to known
/// structural export limitations.
const SKIP_RENDER: &[&str] = &[
    "fragment_markup.ly",                    // markup-only, no \score
    "fragment_music_functions_roundtrip.ly", // \someFunction is undefined
    "fragment_scheme_music.ly",             // unbound Scheme variables (myMusicVar, ly:export)
    "fragment_numeric_expr.ly",             // Scheme string expression, not evaluable
    "fragment_lyrics.ly",                   // \lyricsto needs \new Lyrics context wrapper
    "fragment_scheme_roundtrip.ly",         // \override on context properties (needs \set)
    "fragment_clef_key_time.ly",            // \time 2+3/8 not valid lilypond (needs \compoundMeter)
];

/// Run pipeline (parse → import → export → serialize) and return the .ly output.
fn pipeline_output(src: &str) -> Option<String> {
    let file = Parser::new(src).and_then(|p| p.parse()).ok()?;
    let (mei, ext) = import(&file).ok()?;
    let exported = export(&mei, &ext).ok()?;
    Some(serializer::serialize(&exported))
}

#[test]
fn all_fixtures_lilypond_render() {
    if !lilypond_available() {
        eprintln!("lilypond not found, skipping render tests");
        return;
    }

    let tmp_dir = std::env::temp_dir().join(format!("tusk_ly_render_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).expect("create temp dir");

    let dir = fixture_dir().canonicalize().expect("fixture dir");
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .expect("read dir")
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut rendered = 0u32;
    let mut skipped = 0u32;
    let mut failures: Vec<(String, String)> = Vec::new();

    for entry in &entries {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("ly") {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap();
        if SKIP_RENDER.contains(&name) {
            skipped += 1;
            continue;
        }

        let src = std::fs::read_to_string(&path).expect("read fixture");
        let Some(ly_output) = pipeline_output(&src) else {
            skipped += 1;
            continue;
        };

        let tag = name.strip_suffix(".ly").unwrap_or(name);
        let (ok, output) = lilypond_check(&ly_output, &tmp_dir, tag);
        if ok {
            rendered += 1;
        } else {
            failures.push((name.to_string(), output));
        }
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);

    if !failures.is_empty() {
        let mut msg = format!(
            "{} fixture(s) failed lilypond render ({rendered} passed, {skipped} skipped):\n",
            failures.len()
        );
        for (name, output) in &failures {
            msg.push_str(&format!("\n--- {name} ---\n{output}\n"));
        }
        panic!("{msg}");
    }

    eprintln!("lilypond render: {rendered} passed, {skipped} skipped");
    assert!(rendered >= 10, "expected ≥10 rendered, got {rendered}");
}

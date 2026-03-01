/// Build script that generates per-file tests:
///
/// 1. LilyPond regression tests from `lilypond/lilypond_regression/*.ly`
/// 2. Cross-format pipeline tests for all fixture directories
///
/// Regression tests (existing):
///   - `reg_ser_<name>` — serialization roundtrip
///   - `reg_tri_<name>` — triangle MEI roundtrip
///   - `reg_pipe_<name>` — pipeline stabilization
///   - `reg_xfmt_<name>` — cross-format LilyPond -> MEI -> MusicXML
///
/// Cross-format tests (new):
///   - `cross_musicxml_via_mei.rs` — MusicXML fixtures through MEI
///   - `cross_musicxml_via_lilypond.rs` — MusicXML fixtures through LilyPond
///   - `cross_mei_via_musicxml.rs` — MEI fixtures through MusicXML
///   - `cross_mei_via_lilypond.rs` — MEI fixtures through LilyPond
///   - `cross_lilypond_via_mei.rs` — LilyPond fixtures through MEI
use std::fs;
use std::path::Path;

fn main() {
    let fixtures = Path::new("../fixtures");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    generate_lilypond_regression(&fixtures, &out_dir);
    generate_cross_format_tests(&fixtures, &out_dir);
}

// ============================================================================
// LilyPond regression tests (existing, unchanged)
// ============================================================================

fn generate_lilypond_regression(fixtures: &Path, out_dir: &str) {
    let reg_dir = fixtures.join("lilypond/lilypond_regression");

    if !reg_dir.exists() {
        fs::write(format!("{out_dir}/regression_roundtrip.rs"), "").unwrap();
        fs::write(format!("{out_dir}/regression_cross_format.rs"), "").unwrap();
        return;
    }

    let mut ly_files: Vec<String> = fs::read_dir(&reg_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_str()?.to_string();
            if name.ends_with(".ly") {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    ly_files.sort();

    let standalone: Vec<&str> = ly_files
        .iter()
        .filter(|name| {
            let path = reg_dir.join(name);
            let src = fs::read_to_string(&path).unwrap_or_default();
            !src.contains("\\include \"") && !src.contains("\\include #")
        })
        .map(|s| s.as_str())
        .collect();

    let mut roundtrip = String::with_capacity(standalone.len() * 400);
    roundtrip.push_str("#[allow(non_snake_case)]\nmod regression_roundtrip {\nuse super::*;\n");
    let mut cross_fmt = String::with_capacity(standalone.len() * 300);
    cross_fmt.push_str("#[allow(non_snake_case)]\nmod regression_cross_format {\nuse super::*;\n");

    for name in &standalone {
        let ident = sanitize_ident(name);

        roundtrip.push_str(&format!(
            r#"
#[test]
fn reg_ser_{ident}() {{
    let src = load_regression("{name}");
    try_serialization_roundtrip(&src, "{name}");
}}

#[test]
fn reg_tri_{ident}() {{
    let src = load_regression("{name}");
    try_triangle_mei_roundtrip(&src, "{name}");
}}

#[test]
fn reg_pipe_{ident}() {{
    let src = load_regression("{name}");
    try_pipeline_stable(&src, "{name}");
}}
"#
        ));

        cross_fmt.push_str(&format!(
            r#"
#[test]
fn reg_xfmt_{ident}() {{
    let src = load_regression("{name}");
    try_cross_format(&src, "{name}");
}}
"#
        ));
    }

    roundtrip.push_str("}\n");
    cross_fmt.push_str("}\n");
    fs::write(format!("{out_dir}/regression_roundtrip.rs"), roundtrip).unwrap();
    fs::write(format!("{out_dir}/regression_cross_format.rs"), cross_fmt).unwrap();
}

// ============================================================================
// Cross-format pipeline tests
// ============================================================================

fn generate_cross_format_tests(fixtures: &Path, out_dir: &str) {
    gen_musicxml_via_mei(fixtures, out_dir);
    gen_musicxml_via_lilypond(fixtures, out_dir);
    gen_mei_via_musicxml(fixtures, out_dir);
    gen_mei_via_lilypond(fixtures, out_dir);
    gen_lilypond_via_mei(fixtures, out_dir);
}

/// MusicXML fixtures → MEI → MusicXML
fn gen_musicxml_via_mei(fixtures: &Path, out_dir: &str) {
    let mut code = String::from(
        "#[allow(non_snake_case)]\nmod generated_musicxml_via_mei {\n",
    );

    // Root musicxml/*.musicxml
    for (name, relpath) in collect_sorted(fixtures, "musicxml", "musicxml") {
        let ident = sanitize_ident(&name);
        write_xml_test(&mut code, &ident, &relpath, "try_musicxml_via_mei");
    }

    // musicxml/lilypond_regression/*.xml
    for (name, relpath) in collect_sorted(fixtures, "musicxml/lilypond_regression", "xml") {
        let ident = format!("lyreg_{}", sanitize_ident(&name));
        write_xml_test(&mut code, &ident, &relpath, "try_musicxml_via_mei");
    }

    code.push_str("}\n");
    fs::write(format!("{out_dir}/cross_musicxml_via_mei.rs"), code).unwrap();
}

/// MusicXML fixtures → LilyPond → MusicXML
fn gen_musicxml_via_lilypond(fixtures: &Path, out_dir: &str) {
    let mut code = String::from(
        "#[allow(non_snake_case)]\nmod generated_musicxml_via_lilypond {\n",
    );

    for (name, relpath) in collect_sorted(fixtures, "musicxml", "musicxml") {
        let ident = sanitize_ident(&name);
        write_xml_test(&mut code, &ident, &relpath, "try_musicxml_via_lilypond");
    }

    code.push_str("}\n");
    fs::write(format!("{out_dir}/cross_musicxml_via_lilypond.rs"), code).unwrap();
}

/// MEI fixtures → MusicXML → MEI
fn gen_mei_via_musicxml(fixtures: &Path, out_dir: &str) {
    let mut code = String::from(
        "#[allow(non_snake_case)]\nmod generated_mei_via_musicxml {\n",
    );
    emit_mei_fixtures(fixtures, &mut code, "try_mei_via_musicxml");
    code.push_str("}\n");
    fs::write(format!("{out_dir}/cross_mei_via_musicxml.rs"), code).unwrap();
}

/// MEI fixtures → LilyPond → MEI
fn gen_mei_via_lilypond(fixtures: &Path, out_dir: &str) {
    let mut code = String::from(
        "#[allow(non_snake_case)]\nmod generated_mei_via_lilypond {\n",
    );
    emit_mei_fixtures(fixtures, &mut code, "try_mei_via_lilypond");
    code.push_str("}\n");
    fs::write(format!("{out_dir}/cross_mei_via_lilypond.rs"), code).unwrap();
}

/// Shared: emit tests for all MEI fixtures (root + sample-encodings + verovio).
fn emit_mei_fixtures(fixtures: &Path, code: &mut String, helper: &str) {
    // Root mei/*.mei
    for (name, relpath) in collect_sorted(fixtures, "mei", "mei") {
        let ident = sanitize_ident(&name);
        write_text_test(code, &ident, &relpath, helper);
    }

    // mei/sample-encodings/**/Complete_examples/*.mei — use relpath-based ident to avoid dupes
    let samples_dir = fixtures.join("mei/sample-encodings");
    if samples_dir.exists() {
        for (_name, relpath) in collect_recursive_filtered(fixtures, &samples_dir, "mei", "Complete_examples") {
            let ident = ident_from_relpath(&relpath, "mei/sample-encodings/");
            write_text_test(code, &ident, &relpath, helper);
        }
    }

    // mei/verovio/*.mei
    let verovio_dir = fixtures.join("mei/verovio");
    if verovio_dir.exists() {
        for (name, relpath) in collect_sorted(fixtures, "mei/verovio", "mei") {
            let ident = format!("verovio_{}", sanitize_ident(&name));
            write_text_test(code, &ident, &relpath, helper);
        }
    }
}

/// LilyPond fixtures → MEI → LilyPond
fn gen_lilypond_via_mei(fixtures: &Path, out_dir: &str) {
    let mut code = String::from(
        "#[allow(non_snake_case)]\nmod generated_lilypond_via_mei {\n",
    );

    // Root lilypond/*.ly
    for (name, relpath) in collect_sorted(fixtures, "lilypond", "ly") {
        let ident = sanitize_ident(&name);
        write_text_test(&mut code, &ident, &relpath, "try_lilypond_via_mei");
    }

    // lilypond/lilypond_regression/*.ly (standalone only)
    let reg_dir = fixtures.join("lilypond/lilypond_regression");
    if reg_dir.exists() {
        for (name, relpath) in collect_standalone_ly(fixtures, &reg_dir) {
            let ident = format!("reg_{}", sanitize_ident(&name));
            write_text_test(&mut code, &ident, &relpath, "try_lilypond_via_mei");
        }
    }

    code.push_str("}\n");
    fs::write(format!("{out_dir}/cross_lilypond_via_mei.rs"), code).unwrap();
}

// ============================================================================
// Test code generators
// ============================================================================

/// Generate a test that loads an XML file (with UTF-16 support) and calls a try_* helper.
fn write_xml_test(code: &mut String, ident: &str, relpath: &str, helper: &str) {
    code.push_str(&format!(
        r#"
#[test]
fn xfmt_{ident}() {{
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../fixtures/{relpath}");
    let src = tusk_roundtrip_tests::read_xml_file(path).unwrap();
    tusk_roundtrip_tests::{helper}(&src, "{ident}");
}}
"#
    ));
}

/// Generate a test that loads a plain text file and calls a try_* helper.
fn write_text_test(code: &mut String, ident: &str, relpath: &str, helper: &str) {
    code.push_str(&format!(
        r#"
#[test]
fn xfmt_{ident}() {{
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../fixtures/{relpath}")).unwrap();
    tusk_roundtrip_tests::{helper}(&src, "{ident}");
}}
"#
    ));
}

// ============================================================================
// File collection helpers
// ============================================================================

/// Collect files from `fixtures/{subdir}/*.{ext}`, non-recursive, sorted.
/// Returns (filename, relative_path_from_fixtures_root) pairs.
fn collect_sorted(fixtures: &Path, subdir: &str, ext: &str) -> Vec<(String, String)> {
    let dir = fixtures.join(subdir);
    if !dir.exists() {
        return Vec::new();
    }
    let mut entries: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_str()?.to_string();
            if name.ends_with(&format!(".{ext}")) && e.path().is_file() {
                let relpath = format!("{subdir}/{name}");
                Some((name, relpath))
            } else {
                None
            }
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

/// Recursively collect files with given ext, filtered to paths containing `filter_segment`.
/// Returns (filename, relative_path_from_fixtures_root) pairs, sorted.
fn collect_recursive_filtered(
    fixtures: &Path,
    dir: &Path,
    ext: &str,
    filter_segment: &str,
) -> Vec<(String, String)> {
    let mut results = Vec::new();
    collect_recursive_inner(dir, ext, filter_segment, fixtures, &mut results);
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
}

fn collect_recursive_inner(
    dir: &Path,
    ext: &str,
    filter_segment: &str,
    fixtures_root: &Path,
    results: &mut Vec<(String, String)>,
) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_recursive_inner(&path, ext, filter_segment, fixtures_root, results);
        } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
            let path_str = path.to_str().unwrap_or("");
            if path_str.contains(filter_segment) {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                // Build relative path from fixtures root using forward slashes
                let rel = path.strip_prefix(fixtures_root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");
                results.push((name, rel));
            }
        }
    }
}

/// Collect standalone .ly files (no \include) from a directory, sorted.
fn collect_standalone_ly(fixtures: &Path, dir: &Path) -> Vec<(String, String)> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_str()?.to_string();
            if !name.ends_with(".ly") {
                return None;
            }
            let src = fs::read_to_string(e.path()).unwrap_or_default();
            if src.contains("\\include \"") || src.contains("\\include #") {
                return None;
            }
            let rel = e.path().strip_prefix(fixtures)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/");
            Some((name, rel))
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

// ============================================================================
// Identifier sanitization
// ============================================================================

/// Build a unique identifier from a relative path by incorporating directory structure.
/// Strips `prefix` from the relpath, then sanitizes the whole remaining path.
/// E.g. "mei/sample-encodings/MEI_3.0/Music/Complete_examples/Foo.mei"
///   → "MEI_3_0__Music__Complete_examples__Foo"
fn ident_from_relpath(relpath: &str, prefix: &str) -> String {
    let stripped = relpath.strip_prefix(prefix).unwrap_or(relpath);
    // Strip file extension
    let without_ext = stripped
        .strip_suffix(".mei")
        .or_else(|| stripped.strip_suffix(".musicxml"))
        .or_else(|| stripped.strip_suffix(".xml"))
        .or_else(|| stripped.strip_suffix(".ly"))
        .unwrap_or(stripped);
    // Replace path separators with double underscore
    let with_sep = without_ext.replace('/', "__");
    let mut ident = String::with_capacity(with_sep.len());
    for c in with_sep.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ident.push(c);
        } else {
            ident.push('_');
        }
    }
    if ident.starts_with(|c: char| c.is_ascii_digit()) {
        ident.insert(0, 'n');
    }
    ident
}

fn sanitize_ident(name: &str) -> String {
    // Strip known extensions
    let stem = name
        .strip_suffix(".ly")
        .or_else(|| name.strip_suffix(".musicxml"))
        .or_else(|| name.strip_suffix(".xml"))
        .or_else(|| name.strip_suffix(".mei"))
        .unwrap_or(name);
    let mut ident = String::with_capacity(stem.len());
    for c in stem.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ident.push(c);
        } else {
            ident.push('_');
        }
    }
    if ident.starts_with(|c: char| c.is_ascii_digit()) {
        ident.insert(0, 'n');
    }
    ident
}

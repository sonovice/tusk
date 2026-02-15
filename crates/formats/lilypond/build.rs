/// Build script that generates per-file regression tests from
/// `specs/lilypond/repo/input/regression/*.ly`.
///
/// Each standalone `.ly` file (no `\include`) gets four tests:
///   - `reg_ser_<name>` — serialization roundtrip (parse → serialize → re-parse)
///   - `reg_tri_<name>` — triangle MEI roundtrip
///   - `reg_pipe_<name>` — pipeline stabilization
///   - `reg_xfmt_<name>` — cross-format LilyPond → MEI → MusicXML
///
/// All regression tests are `#[ignore]`d by default to keep `cargo test`
/// fast. Run with `cargo test -- --ignored` to execute them.
use std::fs;
use std::path::Path;

fn main() {
    let reg_dir = Path::new("../../../specs/lilypond/repo/input/regression");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Tell Cargo to re-run if the regression dir contents change.
    println!("cargo:rerun-if-changed=build.rs");

    if !reg_dir.exists() {
        // No regression dir — write empty modules.
        fs::write(format!("{out_dir}/regression_roundtrip.rs"), "").unwrap();
        fs::write(format!("{out_dir}/regression_cross_format.rs"), "").unwrap();
        return;
    }

    let mut ly_files: Vec<String> = fs::read_dir(reg_dir)
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

    // Filter out files with \include (not standalone).
    let standalone: Vec<&str> = ly_files
        .iter()
        .filter(|name| {
            let path = reg_dir.join(name);
            let src = fs::read_to_string(&path).unwrap_or_default();
            !src.contains("\\include \"") && !src.contains("\\include #")
        })
        .map(|s| s.as_str())
        .collect();

    // Generate roundtrip tests.
    let mut roundtrip = String::with_capacity(standalone.len() * 400);
    roundtrip.push_str("#[allow(non_snake_case)]\nmod regression_roundtrip {\nuse super::*;\n");
    // Generate cross-format tests.
    let mut cross_fmt = String::with_capacity(standalone.len() * 300);
    cross_fmt.push_str("#[allow(non_snake_case)]\nmod regression_cross_format {\nuse super::*;\n");

    for name in &standalone {
        let ident = sanitize_ident(name);

        // Roundtrip tests (serialization, triangle, pipeline).
        roundtrip.push_str(&format!(
            r#"
#[test]
#[ignore]
fn reg_ser_{ident}() {{
    let src = load_regression("{name}");
    try_serialization_roundtrip(&src, "{name}");
}}

#[test]
#[ignore]
fn reg_tri_{ident}() {{
    let src = load_regression("{name}");
    try_triangle_mei_roundtrip(&src, "{name}");
}}

#[test]
#[ignore]
fn reg_pipe_{ident}() {{
    let src = load_regression("{name}");
    try_pipeline_stable(&src, "{name}");
}}
"#
        ));

        // Cross-format test.
        cross_fmt.push_str(&format!(
            r#"
#[test]
#[ignore]
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

/// Convert a `.ly` filename to a valid Rust identifier.
fn sanitize_ident(name: &str) -> String {
    let stem = name.strip_suffix(".ly").unwrap_or(name);
    let mut ident = String::with_capacity(stem.len());
    for c in stem.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ident.push(c);
        } else {
            ident.push('_');
        }
    }
    // Ensure it doesn't start with a digit.
    if ident.starts_with(|c: char| c.is_ascii_digit()) {
        ident.insert(0, 'n');
    }
    ident
}

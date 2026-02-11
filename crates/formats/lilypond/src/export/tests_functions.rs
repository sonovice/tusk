//! Roundtrip tests for music function calls.

use super::*;
use crate::import;
use crate::parser::Parser;
use crate::serializer;

/// Parse LilyPond -> import to MEI -> export to LilyPond AST -> serialize.
fn roundtrip(src: &str) -> String {
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import::import(&file).unwrap();
    let exported = export(&mei).unwrap();
    serializer::serialize(&exported)
}

#[test]
fn roundtrip_music_function_with_music_arg() {
    let output = roundtrip("{ \\someFunction { c4 d e f } g4 }");
    assert!(
        output.contains("\\someFunction"),
        "output should contain function name: {output}"
    );
}

#[test]
fn roundtrip_music_function_with_string_and_music() {
    let output = roundtrip("{ \\tag \"part\" { c4 d e f } g4 }");
    assert!(
        output.contains("\\tag \"part\""),
        "output should contain tag with string: {output}"
    );
}

#[test]
fn roundtrip_music_function_with_numeric_arg() {
    let output = roundtrip("{ \\magnifyMusic 0.63 { c4 d e f } g4 }");
    assert!(
        output.contains("\\magnifyMusic 0.63"),
        "output should contain magnifyMusic with number: {output}"
    );
}

#[test]
fn roundtrip_partial_function() {
    let output = roundtrip("{ \\tag #'score \\etc c4 d e f }");
    assert!(
        output.contains("\\tag #'score \\etc"),
        "output should contain partial function: {output}"
    );
}

#[test]
fn roundtrip_multiple_function_calls() {
    let output = roundtrip("{ \\someFunc { c4 } \\otherFunc { d4 } e4 }");
    assert!(
        output.contains("\\someFunc"),
        "output should contain someFunc: {output}"
    );
    assert!(
        output.contains("\\otherFunc"),
        "output should contain otherFunc: {output}"
    );
}

#[test]
fn roundtrip_function_with_scheme_symbol_arg() {
    let output = roundtrip("{ \\tag #'print { c4 d } e4 }");
    assert!(
        output.contains("\\tag #'print"),
        "output should contain scheme symbol arg: {output}"
    );
}

#[test]
fn roundtrip_function_with_scheme_bool_arg() {
    let output = roundtrip("{ \\myFunc ##t { c4 d } e4 }");
    assert!(
        output.contains("\\myFunc ##t"),
        "output should contain scheme bool arg: {output}"
    );
}

#[test]
fn roundtrip_function_with_scheme_integer_arg() {
    let output = roundtrip("{ \\myFunc #42 { c4 d } e4 }");
    assert!(
        output.contains("\\myFunc #42"),
        "output should contain scheme integer arg: {output}"
    );
}

#[test]
fn roundtrip_function_with_default_arg() {
    let output = roundtrip("{ \\myFunc \\default { c4 d } e4 }");
    assert!(
        output.contains("\\myFunc \\default"),
        "output should contain default arg: {output}"
    );
}

#[test]
fn roundtrip_function_with_scheme_string_arg() {
    let output = roundtrip("{ \\myFunc #\"hello\" { c4 d } e4 }");
    assert!(
        output.contains("\\myFunc #\"hello\""),
        "output should contain scheme string arg: {output}"
    );
}

#[test]
fn roundtrip_function_with_mixed_args() {
    let output = roundtrip("{ \\myFunc \"hello\" 3.5 #'sym { c4 d } e4 }");
    assert!(
        output.contains("\\myFunc \"hello\" 3.5 #'sym"),
        "output should contain mixed args: {output}"
    );
}

#[test]
fn roundtrip_function_with_duration_arg() {
    let output = roundtrip("{ \\myFunc 4. { c4 d } e4 }");
    assert!(
        output.contains("\\myFunc 4."),
        "output should contain duration arg: {output}"
    );
}

#[test]
fn roundtrip_partial_with_scheme_args() {
    let output = roundtrip("{ \\keepWithTag #'print \\etc c4 d e f }");
    assert!(
        output.contains("\\keepWithTag #'print \\etc"),
        "output should contain keepWithTag partial: {output}"
    );
}

#[test]
fn roundtrip_function_preserves_typed_args() {
    // Verify that the typed FunctionCall label preserves argument structure
    let src = "{ \\tag \"part\" { c4 d e f } g4 }";
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import::import(&file).unwrap();

    // Check that the MEI label contains typed FunctionCall JSON
    let label = find_func_dir_label(&mei);
    assert!(
        label.contains("\"name\":\"tag\""),
        "label should have typed name: {label}"
    );
    assert!(
        label.contains("\"String\""),
        "label should have String-typed arg: {label}"
    );
}

/// Helper to find the first function dir label in an MEI document.
fn find_func_dir_label(mei: &tusk_model::elements::Mei) -> String {
    use tusk_model::elements::*;
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let SectionChild::Measure(measure) = sec_c {
                                        for mc2 in &measure.children {
                                            if let MeasureChild::Dir(dir) = mc2
                                                && let Some(l) = &dir.common.label
                                                && l.starts_with("tusk:func,")
                                            {
                                                return l.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    std::string::String::new()
}

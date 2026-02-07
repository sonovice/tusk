//! Tusk CLI - Music notation format converter command-line tool.

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use tusk_format::FormatRegistry;

/// Tusk: The Ultimate Score Konverter
///
/// Bidirectional music notation format converter (MEI, MusicXML, …)
#[derive(Parser, Debug)]
#[command(name = "tusk")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file (MEI, MusicXML, …)
    #[arg(value_name = "INPUT")]
    input: std::path::PathBuf,

    /// Output file (MEI, MusicXML, …)
    #[arg(value_name = "OUTPUT")]
    output: std::path::PathBuf,
}

/// Build the default format registry with all compiled-in formats.
fn build_registry() -> FormatRegistry {
    let mut registry = FormatRegistry::new();

    // MEI
    registry.register_importer(Box::new(tusk_mei::MeiFormat));
    registry.register_exporter(Box::new(tusk_mei::MeiFormat));

    // MusicXML
    registry.register_importer(Box::new(tusk_musicxml::MusicXmlFormat));
    registry.register_exporter(Box::new(tusk_musicxml::MusicXmlFormat));

    registry
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let registry = build_registry();

    // Read input file.
    let input_content = fs::read_to_string(&cli.input)
        .with_context(|| format!("Failed to read input file: {:?}", cli.input))?;

    // Resolve input format (extension first, content fallback).
    let input_ext = cli
        .input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let importer = registry
        .find_importer(input_ext, Some(input_content.as_bytes()))
        .with_context(|| format!("Could not detect format of input file: {:?}", cli.input))?;

    // Resolve output format (extension only — we don't have content yet).
    let output_ext = cli
        .output
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let exporter = registry
        .find_exporter(output_ext)
        .with_context(|| format!("Could not detect format of output file: {:?}", cli.output))?;

    // Convert: input → MEI → output.
    let mei = importer
        .import_from_str(&input_content)
        .with_context(|| format!("Failed to import {} file", importer.name()))?;

    let output_content = exporter
        .export_to_string(&mei)
        .with_context(|| format!("Failed to export to {}", exporter.name()))?;

    // Write output file.
    fs::write(&cli.output, &output_content)
        .with_context(|| format!("Failed to write output file: {:?}", cli.output))?;

    println!(
        "Converted {:?} ({}) -> {:?} ({})",
        cli.input,
        importer.name(),
        cli.output,
        exporter.name(),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_finds_mei_by_extension() {
        let reg = build_registry();
        let imp = reg.find_importer("mei", None);
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "mei");
    }

    #[test]
    fn registry_finds_musicxml_by_extension() {
        let reg = build_registry();
        assert!(reg.find_importer("musicxml", None).is_some());
        assert!(reg.find_importer("xml", None).is_some());
    }

    #[test]
    fn registry_detects_mei_from_content() {
        let reg = build_registry();
        let content = b"<mei xmlns=\"http://www.music-encoding.org/ns/mei\">";
        let imp = reg.find_importer("unknown", Some(content.as_slice()));
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "mei");
    }

    #[test]
    fn registry_detects_musicxml_from_content() {
        let reg = build_registry();
        let content = b"<score-partwise version=\"4.0\">";
        let imp = reg.find_importer("unknown", Some(content.as_slice()));
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "musicxml");
    }

    #[test]
    fn registry_finds_exporters() {
        let reg = build_registry();
        assert!(reg.find_exporter("mei").is_some());
        assert!(reg.find_exporter("musicxml").is_some());
        assert!(reg.find_exporter("xml").is_some());
        assert!(reg.find_exporter("unknown").is_none());
    }

    #[test]
    fn xml_extension_defaults_to_musicxml() {
        let reg = build_registry();
        let exp = reg.find_exporter("xml");
        assert!(exp.is_some());
        assert_eq!(exp.unwrap().id(), "musicxml");
    }

    #[test]
    fn xml_extension_with_mei_content_detects_mei() {
        let reg = build_registry();
        let content = b"<?xml version=\"1.0\"?><mei xmlns=\"http://www.music-encoding.org/ns/mei\">";
        let imp = reg.find_importer("xml", Some(content.as_slice()));
        assert!(imp.is_some());
        // With .xml extension, both MEI and MusicXML match by extension.
        // MusicXML matches first, but MEI content detection should win
        // because MusicXML's detect() returns false for MEI content.
        assert_eq!(imp.unwrap().id(), "mei");
    }
}

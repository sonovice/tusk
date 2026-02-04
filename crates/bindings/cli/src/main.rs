//! Tusk CLI - MusicXML <-> MEI converter command-line tool.

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

/// Tusk: The Ultimate Score Konverter
///
/// Bidirectional MusicXML <-> MEI converter
#[derive(Parser, Debug)]
#[command(name = "tusk")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file (MEI or MusicXML)
    #[arg(value_name = "INPUT")]
    input: std::path::PathBuf,

    /// Output file (MEI or MusicXML)
    #[arg(value_name = "OUTPUT")]
    output: std::path::PathBuf,
}

/// Detected format of a music file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Mei,
    MusicXml,
}

/// Detect format from file extension.
fn detect_format(path: &Path) -> Option<Format> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "mei" => Some(Format::Mei),
        "xml" | "musicxml" | "mxl" => {
            // Could be either - check for MEI in filename or content hint
            let name = path.file_stem()?.to_str()?.to_lowercase();
            if name.contains("mei") || name.ends_with(".mei") {
                Some(Format::Mei)
            } else {
                Some(Format::MusicXml)
            }
        }
        _ => None,
    }
}

/// Detect format from file content (fallback).
fn detect_format_from_content(content: &str) -> Option<Format> {
    if content.contains("<mei") || content.contains("music-encoding.org") {
        Some(Format::Mei)
    } else if content.contains("<score-partwise")
        || content.contains("<score-timewise")
        || content.contains("musicxml.org")
    {
        Some(Format::MusicXml)
    } else {
        None
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read input file
    let input_xml = fs::read_to_string(&cli.input)
        .with_context(|| format!("Failed to read input file: {:?}", cli.input))?;

    // Detect input format (prefer extension, fallback to content)
    let input_format = detect_format(&cli.input)
        .or_else(|| detect_format_from_content(&input_xml))
        .with_context(|| format!("Could not detect format of input file: {:?}", cli.input))?;

    // Detect output format (from extension only)
    let output_format = detect_format(&cli.output)
        .with_context(|| format!("Could not detect format of output file: {:?}", cli.output))?;

    // Perform conversion
    let output_xml = match (input_format, output_format) {
        (Format::MusicXml, Format::Mei) => {
            // MusicXML -> MEI
            let score = tusk_musicxml::parser::parse_score_partwise(&input_xml)
                .or_else(|_| tusk_musicxml::parser::parse_score_timewise(&input_xml))
                .with_context(|| "Failed to parse MusicXML")?;
            let mei =
                tusk_musicxml::import(&score).with_context(|| "Failed to import MusicXML to MEI")?;
            tusk_mei::export(&mei).with_context(|| "Failed to export MEI")?
        }
        (Format::Mei, Format::MusicXml) => {
            // MEI -> MusicXML
            let mei = tusk_mei::import(&input_xml).with_context(|| "Failed to parse MEI")?;
            let score =
                tusk_musicxml::export(&mei).with_context(|| "Failed to export MEI to MusicXML")?;
            tusk_musicxml::serialize(&score).with_context(|| "Failed to serialize MusicXML")?
        }
        (Format::MusicXml, Format::MusicXml) => {
            bail!("Input and output are both MusicXML - no conversion needed")
        }
        (Format::Mei, Format::Mei) => {
            bail!("Input and output are both MEI - no conversion needed")
        }
    };

    // Write output file
    fs::write(&cli.output, &output_xml)
        .with_context(|| format!("Failed to write output file: {:?}", cli.output))?;

    println!(
        "Converted {:?} ({:?}) -> {:?} ({:?})",
        cli.input, input_format, cli.output, output_format
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_mei() {
        assert_eq!(detect_format(Path::new("test.mei")), Some(Format::Mei));
    }

    #[test]
    fn test_detect_format_musicxml() {
        assert_eq!(
            detect_format(Path::new("test.musicxml")),
            Some(Format::MusicXml)
        );
        assert_eq!(detect_format(Path::new("test.xml")), Some(Format::MusicXml));
    }

    #[test]
    fn test_detect_format_from_content() {
        assert_eq!(
            detect_format_from_content("<mei xmlns=\"http://www.music-encoding.org/ns/mei\">"),
            Some(Format::Mei)
        );
        assert_eq!(
            detect_format_from_content("<score-partwise version=\"4.0\">"),
            Some(Format::MusicXml)
        );
    }
}

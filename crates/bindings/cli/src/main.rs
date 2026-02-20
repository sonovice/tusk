//! Tusk CLI — Music notation format converter.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use tusk_format::FormatRegistry;

// ---------------------------------------------------------------------------
// CLI definition
// ---------------------------------------------------------------------------

/// Tusk: The Ultimate Score Konverter
#[derive(Parser, Debug)]
#[command(name = "tusk", version)]
#[command(about = "The Ultimate Score Konverter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert between music notation formats
    #[command(visible_alias = "c", long_about = "\
Convert between music notation formats.

Supported formats:
  MEI        .mei                   Full bidirectional support
  MusicXML   .musicxml .xml .mxl    Full bidirectional support
  LilyPond   .ly                    Experimental

Use `-` as INPUT or OUTPUT for stdin/stdout (requires --from or --to).")]
    Convert(ConvertArgs),

    /// List supported formats and their extensions
    #[command(visible_alias = "f")]
    Formats,
}

#[derive(clap::Args, Debug)]
struct ConvertArgs {
    /// Input file path, or `-` for stdin
    #[arg(value_name = "INPUT")]
    input: String,

    /// Output file path, or `-` for stdout
    #[arg(value_name = "OUTPUT")]
    output: String,

    /// Override input format detection (mei, musicxml, mxl, ly)
    #[arg(short = 'f', long = "from", value_name = "FORMAT")]
    from: Option<String>,

    /// Override output format detection (mei, musicxml, mxl, ly)
    #[arg(short = 't', long = "to", value_name = "FORMAT")]
    to: Option<String>,
}

// ---------------------------------------------------------------------------
// Format registry
// ---------------------------------------------------------------------------

/// Build the default format registry with all compiled-in formats.
fn build_registry() -> FormatRegistry {
    let mut registry = FormatRegistry::new();

    // MEI
    registry.register_importer(Box::new(tusk_mei::MeiFormat));
    registry.register_exporter(Box::new(tusk_mei::MeiFormat));

    // MusicXML
    registry.register_importer(Box::new(tusk_musicxml::MusicXmlFormat));
    registry.register_exporter(Box::new(tusk_musicxml::MusicXmlFormat));

    // LilyPond
    registry.register_importer(Box::new(tusk_lilypond::LilyPondFormat));
    registry.register_exporter(Box::new(tusk_lilypond::LilyPondFormat));

    registry
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert(args) => cmd_convert(&args),
        Commands::Formats => cmd_formats(),
    }
}

fn cmd_convert(args: &ConvertArgs) -> Result<()> {
    let registry = build_registry();

    // Read input as raw bytes (supports both text and binary .mxl).
    let input_bytes = read_input(&args.input)?;

    // Resolve format identifiers from flags, extensions, or content.
    let input_fmt = resolve_input_format(args);
    let output_fmt = resolve_output_format(args)?;

    let is_mxl_input = input_fmt.eq_ignore_ascii_case("mxl") || is_zip(&input_bytes);
    let is_mxl_output = output_fmt.eq_ignore_ascii_case("mxl");

    // Import: source format → MEI.
    let (mei, ext_store, in_label) = if is_mxl_input {
        let (mei, ext) =
            tusk_musicxml::import_mxl(&input_bytes).context("failed to import .mxl")?;
        (mei, ext, "MusicXML (.mxl)")
    } else {
        let input_str = std::str::from_utf8(&input_bytes).context("input is not valid UTF-8")?;
        let importer = find_importer(&registry, &input_fmt, Some(&input_bytes))?;
        let (mei, ext) = importer
            .import_from_str(input_str)
            .with_context(|| format!("failed to import {}", importer.name()))?;
        (mei, ext, importer.name())
    };

    // Export: MEI → target format.
    let out_label = if is_mxl_output {
        let bytes = tusk_musicxml::export_mxl_with_ext(&mei, &ext_store)
            .context("failed to export .mxl")?;
        write_output(&args.output, &bytes)?;
        "MusicXML (.mxl)"
    } else {
        let exporter = find_exporter(&registry, &output_fmt)?;
        let text = exporter
            .export_to_string(&mei, &ext_store)
            .with_context(|| format!("failed to export to {}", exporter.name()))?;
        write_output(&args.output, text.as_bytes())?;
        exporter.name()
    };

    // Status line on stderr (avoids polluting piped stdout).
    if args.output != "-" {
        eprintln!(
            "{} ({in_label}) → {} ({out_label})",
            args.input, args.output
        );
    }

    Ok(())
}

fn cmd_formats() -> Result<()> {
    let registry = build_registry();

    // Collect unique formats (deduplicated by ID) with their capabilities.
    let mut seen = HashSet::new();
    let mut formats: Vec<(&str, &[&str], bool, bool)> = Vec::new();

    for imp in registry.importers() {
        if seen.insert(imp.id()) {
            let can_export = registry.find_exporter_by_id(imp.id()).is_some();
            formats.push((imp.name(), imp.extensions(), true, can_export));
        }
    }
    for exp in registry.exporters() {
        if seen.insert(exp.id()) {
            let can_import = registry.find_importer_by_id(exp.id()).is_some();
            formats.push((exp.name(), exp.extensions(), can_import, true));
        }
    }

    println!("Supported formats:\n");
    println!(
        "  {:<12} {:<24} {}",
        "Format", "Extensions", "Capabilities"
    );
    println!(
        "  {:<12} {:<24} {}",
        "------", "----------", "------------"
    );
    for (name, exts, can_import, can_export) in &formats {
        let ext_str = exts
            .iter()
            .map(|e| format!(".{e}"))
            .collect::<Vec<_>>()
            .join(" ");
        let caps = match (can_import, can_export) {
            (true, true) => "import, export",
            (true, false) => "import only",
            (false, true) => "export only",
            (false, false) => "(none)",
        };
        println!("  {:<12} {:<24} {}", name, ext_str, caps);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// I/O helpers
// ---------------------------------------------------------------------------

fn read_input(path: &str) -> Result<Vec<u8>> {
    if path == "-" {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .context("failed to read stdin")?;
        Ok(buf)
    } else {
        fs::read(path).with_context(|| format!("failed to read: {path}"))
    }
}

fn write_output(path: &str, data: &[u8]) -> Result<()> {
    if path == "-" {
        io::stdout()
            .write_all(data)
            .context("failed to write to stdout")
    } else {
        fs::write(path, data).with_context(|| format!("failed to write: {path}"))
    }
}

// ---------------------------------------------------------------------------
// Format resolution
// ---------------------------------------------------------------------------

/// Determine the input format identifier from --from flag, file extension,
/// or empty string (triggers content-based detection).
fn resolve_input_format(args: &ConvertArgs) -> String {
    if let Some(ref fmt) = args.from {
        fmt.clone()
    } else if args.input == "-" {
        String::new()
    } else {
        Path::new(&args.input)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string()
    }
}

/// Determine the output format identifier from --to flag or file extension.
/// Fails for stdout without --to since extension detection is impossible.
fn resolve_output_format(args: &ConvertArgs) -> Result<String> {
    if let Some(ref fmt) = args.to {
        Ok(fmt.clone())
    } else if args.output == "-" {
        bail!("cannot detect output format for stdout; use --to <FORMAT>")
    } else {
        Ok(Path::new(&args.output)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string())
    }
}

/// Find an importer by extension or format ID, with optional content fallback.
fn find_importer<'a>(
    registry: &'a FormatRegistry,
    ext: &str,
    content: Option<&[u8]>,
) -> Result<&'a dyn tusk_format::Importer> {
    registry
        .find_importer(ext, content)
        .or_else(|| registry.find_importer_by_id(ext))
        .with_context(|| {
            if ext.is_empty() {
                "could not detect input format; use --from <FORMAT>".to_string()
            } else {
                format!("unknown input format: {ext}")
            }
        })
}

/// Find an exporter by extension or format ID.
fn find_exporter<'a>(
    registry: &'a FormatRegistry,
    ext: &str,
) -> Result<&'a dyn tusk_format::Exporter> {
    registry
        .find_exporter(ext)
        .or_else(|| registry.find_exporter_by_id(ext))
        .with_context(|| format!("unknown output format: {ext}"))
}

/// Check if data starts with ZIP magic bytes (PK\x03\x04).
fn is_zip(data: &[u8]) -> bool {
    data.len() >= 4 && data[..4] == [0x50, 0x4B, 0x03, 0x04]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

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
    fn registry_finds_lilypond_by_extension() {
        let reg = build_registry();
        let imp = reg.find_importer("ly", None);
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "lilypond");
        let exp = reg.find_exporter("ly");
        assert!(exp.is_some());
        assert_eq!(exp.unwrap().id(), "lilypond");
    }

    #[test]
    fn registry_detects_lilypond_from_content() {
        let reg = build_registry();
        let content = b"\\version \"2.24.0\"\n\\score { { c4 } }";
        let imp = reg.find_importer("unknown", Some(content.as_slice()));
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "lilypond");
    }

    #[test]
    fn xml_extension_with_mei_content_detects_mei() {
        let reg = build_registry();
        let content =
            b"<?xml version=\"1.0\"?><mei xmlns=\"http://www.music-encoding.org/ns/mei\">";
        let imp = reg.find_importer("xml", Some(content.as_slice()));
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "mei");
    }

    #[test]
    fn find_importer_by_format_id() {
        let reg = build_registry();
        let imp = find_importer(&reg, "lilypond", None);
        assert!(imp.is_ok());
        assert_eq!(imp.unwrap().id(), "lilypond");
    }

    #[test]
    fn find_exporter_by_format_id() {
        let reg = build_registry();
        let exp = find_exporter(&reg, "lilypond");
        assert!(exp.is_ok());
        assert_eq!(exp.unwrap().id(), "lilypond");
    }

    #[test]
    fn is_zip_detects_magic_bytes() {
        assert!(is_zip(&[0x50, 0x4B, 0x03, 0x04, 0x00]));
        assert!(!is_zip(&[0x50, 0x4B, 0x03]));
        assert!(!is_zip(b"<?xml"));
    }

    #[test]
    fn resolve_output_format_requires_flag_for_stdout() {
        let args = ConvertArgs {
            input: "test.mei".to_string(),
            output: "-".to_string(),
            from: None,
            to: None,
        };
        assert!(resolve_output_format(&args).is_err());

        let args = ConvertArgs {
            input: "test.mei".to_string(),
            output: "-".to_string(),
            from: None,
            to: Some("musicxml".to_string()),
        };
        assert_eq!(resolve_output_format(&args).unwrap(), "musicxml");
    }
}

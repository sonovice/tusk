//! Tusk CLI — Music notation format converter.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
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

    /// Recurse into subdirectories when INPUT is a folder
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,
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
    let input_path = Path::new(&args.input);

    if input_path.is_dir() {
        cmd_convert_dir(args)
    } else {
        let output = resolve_output_path(&args.input, &args.output, &args.to);
        convert_single_file(&args.input, &output, args)
    }
}

/// Batch-convert all files in a directory.
fn cmd_convert_dir(args: &ConvertArgs) -> Result<()> {
    let input_dir = Path::new(&args.input);
    let output_dir = Path::new(&args.output);

    if args.output == "-" {
        bail!("output must be a directory when input is a directory");
    }
    if !output_dir.is_dir() {
        fs::create_dir_all(output_dir)
            .with_context(|| format!("failed to create output directory: {}", args.output))?;
    }

    let registry = build_registry();
    let known_exts = collect_known_extensions(&registry);
    let output_ext = resolve_output_ext(args)?;

    let files = collect_input_files(input_dir, args.recursive, &known_exts)?;
    if files.is_empty() {
        bail!("no supported files found in {}", args.input);
    }

    let mut ok = 0usize;
    let mut fail = 0usize;
    for file in &files {
        let rel = file.strip_prefix(input_dir).unwrap_or(file);
        let mut out_path = output_dir.join(rel);
        out_path.set_extension(&output_ext);

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create directory: {}", parent.display())
            })?;
        }

        let in_str = file.to_string_lossy().to_string();
        let out_str = out_path.to_string_lossy().to_string();
        match convert_single_file(&in_str, &out_str, args) {
            Ok(()) => ok += 1,
            Err(e) => {
                eprintln!("ERROR: {} — {e:#}", file.display());
                fail += 1;
            }
        }
    }

    eprintln!("{ok} converted, {fail} failed");
    if fail > 0 {
        bail!("{fail} file(s) failed to convert");
    }
    Ok(())
}

/// Convert a single file.
fn convert_single_file(input: &str, output: &str, args: &ConvertArgs) -> Result<()> {
    let registry = build_registry();

    let input_bytes = read_input(input)?;

    let input_fmt = resolve_input_format_for(input, &args.from);
    let output_fmt = resolve_output_format_for(output, &args.to)?;

    let is_mxl_input = input_fmt.eq_ignore_ascii_case("mxl") || is_zip(&input_bytes);
    let is_mxl_output = output_fmt.eq_ignore_ascii_case("mxl");

    // Transcode UTF-16 to UTF-8 if needed (MusicXML files may use UTF-16 encoding)
    let input_bytes = ensure_utf8(input_bytes)?;

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

    let out_label = if is_mxl_output {
        let bytes = tusk_musicxml::export_mxl_with_ext(&mei, &ext_store)
            .context("failed to export .mxl")?;
        write_output(output, &bytes)?;
        "MusicXML (.mxl)"
    } else {
        let exporter = find_exporter(&registry, &output_fmt)?;
        let text = exporter
            .export_to_string(&mei, &ext_store)
            .with_context(|| format!("failed to export to {}", exporter.name()))?;
        write_output(output, text.as_bytes())?;
        exporter.name()
    };

    if output != "-" {
        eprintln!("{input} ({in_label}) → {output} ({out_label})");
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

/// Detect UTF-16 BOM and transcode to UTF-8 if needed.
/// Returns the (possibly transcoded) bytes unchanged if already UTF-8.
fn ensure_utf8(bytes: Vec<u8>) -> Result<Vec<u8>> {
    if bytes.len() >= 2 {
        // UTF-16 BE BOM: 0xFE 0xFF
        if bytes[0] == 0xFE && bytes[1] == 0xFF {
            let u16s: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            let s = String::from_utf16(&u16s).context("invalid UTF-16 BE")?;
            return Ok(s.into_bytes());
        }
        // UTF-16 LE BOM: 0xFF 0xFE
        if bytes[0] == 0xFF && bytes[1] == 0xFE {
            let u16s: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            let s = String::from_utf16(&u16s).context("invalid UTF-16 LE")?;
            return Ok(s.into_bytes());
        }
    }
    // UTF-8 BOM: strip it if present
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return Ok(bytes[3..].to_vec());
    }
    Ok(bytes)
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

/// Determine the input format from --from flag, file extension, or empty (content detection).
fn resolve_input_format_for(path: &str, from: &Option<String>) -> String {
    if let Some(fmt) = from {
        fmt.clone()
    } else if path == "-" {
        String::new()
    } else {
        Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string()
    }
}

/// Determine the output format from --to flag or file extension.
fn resolve_output_format_for(path: &str, to: &Option<String>) -> Result<String> {
    if let Some(fmt) = to {
        Ok(fmt.clone())
    } else if path == "-" {
        bail!("cannot detect output format for stdout; use --to <FORMAT>")
    } else {
        Ok(Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string())
    }
}

/// Determine output extension for batch mode (from --to flag or output path extension).
fn resolve_output_ext(args: &ConvertArgs) -> Result<String> {
    if let Some(ref fmt) = args.to {
        Ok(fmt.clone())
    } else {
        // Try to infer from output path extension (e.g. user wrote "out/" but we need --to).
        let ext = Path::new(&args.output)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        if ext.is_empty() {
            bail!("cannot detect output format for directory; use --to <FORMAT>")
        }
        Ok(ext)
    }
}

/// If output is a directory, place the file inside it using the input filename
/// (with extension replaced by --to format if given).
fn resolve_output_path(input: &str, output: &str, to: &Option<String>) -> String {
    let out_path = Path::new(output);
    if out_path.is_dir() {
        let mut filename = PathBuf::from(
            Path::new(input)
                .file_name()
                .unwrap_or_default(),
        );
        if let Some(ext) = to {
            filename.set_extension(ext);
        }
        out_path.join(&filename).to_string_lossy().to_string()
    } else {
        output.to_string()
    }
}

/// Collect all known importable extensions from the registry.
fn collect_known_extensions(registry: &FormatRegistry) -> HashSet<String> {
    let mut exts = HashSet::new();
    for imp in registry.importers() {
        for ext in imp.extensions() {
            exts.insert(ext.to_lowercase());
        }
    }
    exts
}

/// Collect input files from a directory, optionally recursing.
fn collect_input_files(
    dir: &Path,
    recursive: bool,
    known_exts: &HashSet<String>,
) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_inner(dir, recursive, known_exts, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_inner(
    dir: &Path,
    recursive: bool,
    known_exts: &HashSet<String>,
    out: &mut Vec<PathBuf>,
) -> Result<()> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("failed to read directory: {}", dir.display()))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if recursive {
                collect_files_inner(&path, true, known_exts, out)?;
            }
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if known_exts.contains(&ext.to_lowercase()) {
                out.push(path);
            }
        }
    }
    Ok(())
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
        assert!(resolve_output_format_for("-", &None).is_err());
        assert_eq!(
            resolve_output_format_for("-", &Some("musicxml".to_string())).unwrap(),
            "musicxml"
        );
    }

    #[test]
    fn resolve_output_path_file_to_file() {
        assert_eq!(
            resolve_output_path("input.mei", "output.musicxml", &None),
            "output.musicxml"
        );
    }

    #[test]
    fn collect_known_extensions_includes_mei() {
        let reg = build_registry();
        let exts = collect_known_extensions(&reg);
        assert!(exts.contains("mei"));
        assert!(exts.contains("musicxml"));
        assert!(exts.contains("ly"));
    }
}

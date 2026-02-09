//! Compressed MusicXML (.mxl) reading and writing.
//!
//! The .mxl format is a ZIP archive containing:
//! - `META-INF/container.xml` — describes the rootfile(s)
//! - One or more MusicXML files
//! - Optional media files (images, audio)
//!
//! Per the MusicXML 4.1 spec:
//! - The first file should be an uncompressed `mimetype` file containing
//!   `application/vnd.recordare.musicxml`
//! - `META-INF/container.xml` lists rootfiles; the first must be MusicXML
//! - Files are compressed with DEFLATE

use std::io::{Cursor, Read, Write};

use crate::model::elements::{ScorePartwise, ScoreTimewise};
use crate::parser::{self, ParseError};
use crate::serializer::{self, MusicXmlSerialize};

/// Errors that can occur when reading or writing .mxl files.
#[derive(Debug, thiserror::Error)]
pub enum MxlError {
    /// ZIP archive error.
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// I/O error reading/writing the archive.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The archive has no `META-INF/container.xml`.
    #[error("missing META-INF/container.xml in .mxl archive")]
    MissingContainer,

    /// The container.xml has no rootfile elements.
    #[error("no rootfile found in container.xml")]
    NoRootfile,

    /// The rootfile path doesn't exist in the archive.
    #[error("rootfile not found in archive: {0}")]
    RootfileNotFound(String),

    /// Error parsing MusicXML content.
    #[error("MusicXML parse error: {0}")]
    Parse(#[from] ParseError),

    /// Error serializing MusicXML content.
    #[error("MusicXML serialize error: {0}")]
    Serialize(#[from] crate::serializer::SerializeError),

    /// Error parsing container.xml.
    #[error("invalid container.xml: {0}")]
    InvalidContainer(String),
}

/// Result type for .mxl operations.
pub type MxlResult<T> = Result<T, MxlError>;

/// Metadata about a rootfile entry in the container.
#[derive(Debug, Clone)]
pub struct Rootfile {
    /// Path relative to the archive root.
    pub full_path: String,
    /// Optional media type (e.g. `application/vnd.recordare.musicxml+xml`).
    pub media_type: Option<String>,
}

/// Contents of a parsed .mxl archive.
#[derive(Debug)]
pub struct MxlArchive {
    /// The primary MusicXML score (from the first rootfile).
    pub score: ScorePartwise,
    /// All rootfile entries from container.xml.
    pub rootfiles: Vec<Rootfile>,
    /// Additional files in the archive (path → contents).
    /// Excludes META-INF/container.xml, mimetype, and the primary rootfile.
    pub additional_files: Vec<(String, Vec<u8>)>,
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

/// Read a .mxl archive from bytes and parse the primary MusicXML score.
pub fn read_mxl(data: &[u8]) -> MxlResult<MxlArchive> {
    let cursor = Cursor::new(data);
    let mut archive = zip::ZipArchive::new(cursor)?;

    // 1. Read META-INF/container.xml
    let container_xml = read_archive_file(&mut archive, "META-INF/container.xml")
        .ok_or(MxlError::MissingContainer)?;
    let container_str = std::str::from_utf8(&container_xml)
        .map_err(|e| MxlError::InvalidContainer(e.to_string()))?;

    // 2. Parse container.xml to find rootfiles
    let rootfiles = parse_container_xml(container_str)?;
    if rootfiles.is_empty() {
        return Err(MxlError::NoRootfile);
    }

    // 3. Read and parse the primary rootfile (first entry)
    let primary_path = &rootfiles[0].full_path;
    let score_xml = read_archive_file(&mut archive, primary_path)
        .ok_or_else(|| MxlError::RootfileNotFound(primary_path.clone()))?;
    let score_str =
        std::str::from_utf8(&score_xml).map_err(|e| MxlError::InvalidContainer(e.to_string()))?;

    let score = parser::parse_score_partwise(score_str)
        .or_else(|_| parser::parse_score_timewise(score_str))?;

    // 4. Collect additional files
    let skip_paths: Vec<&str> = vec!["META-INF/container.xml", "mimetype", primary_path];
    let mut additional_files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        // Skip directories and known files
        if file.is_dir() || skip_paths.iter().any(|p| *p == name) {
            continue;
        }

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        additional_files.push((name, contents));
    }

    Ok(MxlArchive {
        score,
        rootfiles,
        additional_files,
    })
}

/// Read a .mxl archive and return the primary score as a partwise document.
pub fn read_mxl_score(data: &[u8]) -> MxlResult<ScorePartwise> {
    Ok(read_mxl(data)?.score)
}

// ---------------------------------------------------------------------------
// Writing
// ---------------------------------------------------------------------------

/// Options for writing .mxl archives.
#[derive(Debug, Clone)]
pub struct MxlWriteOptions {
    /// Path for the MusicXML file inside the archive.
    /// Default: `"score.musicxml"`
    pub score_path: String,
    /// Additional rootfile entries (non-MusicXML alternate renditions).
    pub additional_rootfiles: Vec<Rootfile>,
    /// Additional files to include in the archive (path → contents).
    pub additional_files: Vec<(String, Vec<u8>)>,
}

impl Default for MxlWriteOptions {
    fn default() -> Self {
        Self {
            score_path: "score.musicxml".to_string(),
            additional_rootfiles: Vec::new(),
            additional_files: Vec::new(),
        }
    }
}

/// Write a MusicXML score as a compressed .mxl archive.
pub fn write_mxl(score: &ScorePartwise) -> MxlResult<Vec<u8>> {
    write_mxl_with_options(score, &MxlWriteOptions::default())
}

/// Write a MusicXML score as a compressed .mxl archive with custom options.
pub fn write_mxl_with_options(
    score: &ScorePartwise,
    options: &MxlWriteOptions,
) -> MxlResult<Vec<u8>> {
    let xml = score.to_musicxml_string()?;
    write_mxl_from_xml(&xml, options)
}

/// Write a MusicXML timewise score as a compressed .mxl archive.
pub fn write_mxl_timewise(score: &ScoreTimewise) -> MxlResult<Vec<u8>> {
    let xml = serializer::serialize_timewise(score)?;
    write_mxl_from_xml(&xml, &MxlWriteOptions::default())
}

/// Write raw MusicXML XML string as a compressed .mxl archive.
fn write_mxl_from_xml(xml: &str, options: &MxlWriteOptions) -> MxlResult<Vec<u8>> {
    let mut buf = Vec::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut zip = zip::ZipWriter::new(cursor);

        // 1. Write mimetype (must be first, uncompressed, no extra field)
        let mimetype_options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .large_file(false);
        zip.start_file("mimetype", mimetype_options)?;
        zip.write_all(b"application/vnd.recordare.musicxml")?;

        // 2. Write META-INF/container.xml
        let container = build_container_xml(&options.score_path, &options.additional_rootfiles);
        let deflate_options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        zip.start_file("META-INF/container.xml", deflate_options)?;
        zip.write_all(container.as_bytes())?;

        // 3. Write the MusicXML score
        zip.start_file(&options.score_path, deflate_options)?;
        zip.write_all(xml.as_bytes())?;

        // 4. Write additional files
        for (path, contents) in &options.additional_files {
            zip.start_file(path, deflate_options)?;
            zip.write_all(contents)?;
        }

        zip.finish()?;
    }
    Ok(buf)
}

// ---------------------------------------------------------------------------
// Container XML
// ---------------------------------------------------------------------------

/// Parse META-INF/container.xml to extract rootfile entries.
fn parse_container_xml(xml: &str) -> MxlResult<Vec<Rootfile>> {
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut rootfiles = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Empty(ref e)) if e.name().as_ref() == b"rootfile" => {
                let rf = parse_rootfile_attrs(e)?;
                rootfiles.push(rf);
            }
            Ok(quick_xml::events::Event::Start(ref e)) if e.name().as_ref() == b"rootfile" => {
                let rf = parse_rootfile_attrs(e)?;
                rootfiles.push(rf);
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => {
                return Err(MxlError::InvalidContainer(format!(
                    "XML parse error: {}",
                    e
                )));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(rootfiles)
}

/// Extract attributes from a <rootfile> element.
fn parse_rootfile_attrs(e: &quick_xml::events::BytesStart) -> MxlResult<Rootfile> {
    let mut full_path = None;
    let mut media_type = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"full-path" => {
                full_path = Some(
                    String::from_utf8(attr.value.to_vec())
                        .map_err(|e| MxlError::InvalidContainer(e.to_string()))?,
                );
            }
            b"media-type" => {
                media_type = Some(
                    String::from_utf8(attr.value.to_vec())
                        .map_err(|e| MxlError::InvalidContainer(e.to_string()))?,
                );
            }
            _ => {}
        }
    }

    let full_path = full_path.ok_or_else(|| {
        MxlError::InvalidContainer("rootfile missing full-path attribute".to_string())
    })?;

    Ok(Rootfile {
        full_path,
        media_type,
    })
}

/// Build META-INF/container.xml content.
fn build_container_xml(score_path: &str, additional_rootfiles: &[Rootfile]) -> String {
    let mut xml =
        String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<container>\n  <rootfiles>\n");

    // Primary rootfile (MusicXML)
    xml.push_str("    <rootfile full-path=\"");
    xml.push_str(&escape_xml_attr(score_path));
    xml.push_str("\" media-type=\"application/vnd.recordare.musicxml+xml\"/>\n");

    // Additional rootfiles
    for rf in additional_rootfiles {
        xml.push_str("    <rootfile full-path=\"");
        xml.push_str(&escape_xml_attr(&rf.full_path));
        xml.push('"');
        if let Some(ref mt) = rf.media_type {
            xml.push_str(" media-type=\"");
            xml.push_str(&escape_xml_attr(mt));
            xml.push('"');
        }
        xml.push_str("/>\n");
    }

    xml.push_str("  </rootfiles>\n</container>\n");
    xml
}

/// Minimal XML attribute escaping.
fn escape_xml_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read a file from a ZIP archive by path, returning None if not found.
fn read_archive_file(archive: &mut zip::ZipArchive<Cursor<&[u8]>>, path: &str) -> Option<Vec<u8>> {
    let mut file = archive.by_name(path).ok()?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).ok()?;
    Some(contents)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_simple_score() {
        let score = ScorePartwise::default();
        let mxl_bytes = write_mxl(&score).expect("write_mxl failed");

        let archive = read_mxl(&mxl_bytes).expect("read_mxl failed");
        assert_eq!(archive.rootfiles.len(), 1);
        assert_eq!(archive.rootfiles[0].full_path, "score.musicxml");
        assert_eq!(
            archive.rootfiles[0].media_type.as_deref(),
            Some("application/vnd.recordare.musicxml+xml")
        );
    }

    #[test]
    fn mimetype_is_first_and_uncompressed() {
        let score = ScorePartwise::default();
        let mxl_bytes = write_mxl(&score).expect("write_mxl failed");

        let cursor = Cursor::new(&mxl_bytes);
        let mut archive = zip::ZipArchive::new(cursor).expect("open zip failed");

        // First file should be mimetype
        let first = archive.by_index(0).expect("no first file");
        assert_eq!(first.name(), "mimetype");
        assert_eq!(first.compression(), zip::CompressionMethod::Stored);
    }

    #[test]
    fn container_xml_structure() {
        let xml = build_container_xml("score.musicxml", &[]);
        assert!(xml.contains("<container>"));
        assert!(xml.contains("<rootfiles>"));
        assert!(xml.contains("full-path=\"score.musicxml\""));
        assert!(xml.contains("media-type=\"application/vnd.recordare.musicxml+xml\""));
    }

    #[test]
    fn parse_container_xml_basic() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<container>
  <rootfiles>
    <rootfile full-path="MyScore.musicxml" media-type="application/vnd.recordare.musicxml+xml"/>
    <rootfile full-path="MyScore.pdf" media-type="application/pdf"/>
  </rootfiles>
</container>"#;

        let rootfiles = parse_container_xml(xml).expect("parse failed");
        assert_eq!(rootfiles.len(), 2);
        assert_eq!(rootfiles[0].full_path, "MyScore.musicxml");
        assert_eq!(
            rootfiles[0].media_type.as_deref(),
            Some("application/vnd.recordare.musicxml+xml")
        );
        assert_eq!(rootfiles[1].full_path, "MyScore.pdf");
        assert_eq!(rootfiles[1].media_type.as_deref(), Some("application/pdf"));
    }

    #[test]
    fn additional_files_preserved() {
        let score = ScorePartwise::default();
        let image_data = b"fake PNG data".to_vec();
        let options = MxlWriteOptions {
            score_path: "music/score.musicxml".to_string(),
            additional_rootfiles: vec![Rootfile {
                full_path: "images/cover.png".to_string(),
                media_type: Some("image/png".to_string()),
            }],
            additional_files: vec![("images/cover.png".to_string(), image_data.clone())],
        };

        let mxl_bytes = write_mxl_with_options(&score, &options).expect("write_mxl failed");
        let archive = read_mxl(&mxl_bytes).expect("read_mxl failed");

        assert_eq!(archive.rootfiles.len(), 2);
        assert_eq!(archive.rootfiles[0].full_path, "music/score.musicxml");
        assert_eq!(archive.rootfiles[1].full_path, "images/cover.png");
        assert_eq!(archive.additional_files.len(), 1);
        assert_eq!(archive.additional_files[0].0, "images/cover.png");
        assert_eq!(archive.additional_files[0].1, image_data);
    }
}

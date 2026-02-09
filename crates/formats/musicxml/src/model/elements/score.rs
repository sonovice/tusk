//! Score and part types for MusicXML documents.
//!
//! This module contains the core document structure types including
//! ScorePartwise, Work, Identification, Defaults, Credits, PartList, and Part.

use serde::{Deserialize, Serialize};

use crate::model::data::*;

// ============================================================================
// Core Document Types
// ============================================================================

/// The root element for a partwise MusicXML score.
///
/// Contains a score header followed by parts with measures inside.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename = "score-partwise")]
pub struct ScorePartwise {
    /// MusicXML version (default: "1.0")
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Work identification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<Work>,

    /// Movement number
    #[serde(rename = "movement-number", skip_serializing_if = "Option::is_none")]
    pub movement_number: Option<String>,

    /// Movement title
    #[serde(rename = "movement-title", skip_serializing_if = "Option::is_none")]
    pub movement_title: Option<String>,

    /// Identification metadata (creators, rights, encoding, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<Identification>,

    /// Score defaults (layout, appearance settings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,

    /// Credits appearing on pages
    #[serde(rename = "credit", default, skip_serializing_if = "Vec::is_empty")]
    pub credits: Vec<Credit>,

    /// Part list (required) - defines all parts in the score
    #[serde(rename = "part-list")]
    pub part_list: PartList,

    /// Parts containing measures
    #[serde(rename = "part", default)]
    pub parts: Vec<Part>,
}

// ============================================================================
// Score Header Types
// ============================================================================

/// Work identification information.
///
/// Optionally identifies the work by number and title, with optional
/// opus document reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Work {
    /// Work number (e.g., opus number)
    #[serde(rename = "work-number", skip_serializing_if = "Option::is_none")]
    pub work_number: Option<String>,

    /// Work title
    #[serde(rename = "work-title", skip_serializing_if = "Option::is_none")]
    pub work_title: Option<String>,

    /// Link to opus document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus: Option<Opus>,
}

/// Link to an opus document that composes multiple scores.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Opus {
    /// XLink href attribute
    #[serde(rename = "@xlink:href")]
    pub href: String,

    /// XLink type (default: "simple")
    #[serde(rename = "@xlink:type", skip_serializing_if = "Option::is_none")]
    pub xlink_type: Option<String>,
}

/// Identification metadata about the score.
///
/// Contains Dublin Core-based information: creator, rights, source, relation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Identification {
    /// Creators (composer, lyricist, arranger, etc.)
    #[serde(rename = "creator", default, skip_serializing_if = "Vec::is_empty")]
    pub creators: Vec<TypedText>,

    /// Rights/copyright notices
    #[serde(rename = "rights", default, skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<TypedText>,

    /// Encoding information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,

    /// Source of the music
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Related resources
    #[serde(rename = "relation", default, skip_serializing_if = "Vec::is_empty")]
    pub relations: Vec<TypedText>,

    /// Miscellaneous metadata not covered elsewhere
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miscellaneous: Option<Miscellaneous>,
}

/// Text with an optional type attribute.
///
/// Used for creator, rights, relation elements where different types
/// (music, words, arrangement) can be specified.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedText {
    /// The type of this text (e.g., "composer", "music", "words")
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,

    /// The text content
    #[serde(rename = "$value")]
    pub value: String,
}

impl TypedText {
    /// Create a new typed text with the given type and value.
    pub fn new(text_type: Option<&str>, value: &str) -> Self {
        Self {
            text_type: text_type.map(String::from),
            value: value.to_string(),
        }
    }

    /// Create a typed text without a type attribute.
    pub fn untyped(value: &str) -> Self {
        Self {
            text_type: None,
            value: value.to_string(),
        }
    }
}

/// Encoding information for the digital encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Encoding {
    /// Encoding date (YYYY-MM-DD format)
    #[serde(rename = "encoding-date", skip_serializing_if = "Option::is_none")]
    pub encoding_date: Option<String>,

    /// Encoder(s) with optional type
    #[serde(rename = "encoder", default, skip_serializing_if = "Vec::is_empty")]
    pub encoders: Vec<TypedText>,

    /// Software used for encoding
    #[serde(rename = "software", default, skip_serializing_if = "Vec::is_empty")]
    pub software: Vec<String>,

    /// Encoding description
    #[serde(
        rename = "encoding-description",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub encoding_descriptions: Vec<String>,

    /// Feature support information
    #[serde(rename = "supports", default, skip_serializing_if = "Vec::is_empty")]
    pub supports: Vec<Supports>,
}

/// Indicates whether a MusicXML feature is supported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Supports {
    /// The element being described
    #[serde(rename = "@element")]
    pub element: String,

    /// Whether the feature is supported
    #[serde(rename = "@type")]
    pub support_type: YesNo,

    /// Optional attribute being described
    #[serde(rename = "@attribute", skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,

    /// Optional value being described
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Miscellaneous metadata container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Miscellaneous {
    /// Miscellaneous fields (name-value pairs)
    #[serde(
        rename = "miscellaneous-field",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<MiscellaneousField>,
}

/// A name-value pair for miscellaneous data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiscellaneousField {
    /// Field name
    #[serde(rename = "@name")]
    pub name: String,

    /// Field value
    #[serde(rename = "$value")]
    pub value: String,
}

/// Score defaults for layout and appearance.
///
/// Placeholder - will be expanded in later phases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Defaults {
    /// Scaling information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling: Option<Scaling>,

    /// Page layout
    #[serde(rename = "page-layout", skip_serializing_if = "Option::is_none")]
    pub page_layout: Option<PageLayout>,

    /// System layout
    #[serde(rename = "system-layout", skip_serializing_if = "Option::is_none")]
    pub system_layout: Option<SystemLayout>,

    /// Staff layout
    #[serde(
        rename = "staff-layout",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub staff_layouts: Vec<StaffLayout>,

    /// Appearance settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<Appearance>,

    /// Music font
    #[serde(rename = "music-font", skip_serializing_if = "Option::is_none")]
    pub music_font: Option<EmptyFont>,

    /// Word font
    #[serde(rename = "word-font", skip_serializing_if = "Option::is_none")]
    pub word_font: Option<EmptyFont>,

    /// Lyric fonts
    #[serde(rename = "lyric-font", default, skip_serializing_if = "Vec::is_empty")]
    pub lyric_fonts: Vec<LyricFont>,

    /// Lyric language
    #[serde(
        rename = "lyric-language",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub lyric_languages: Vec<LyricLanguage>,
}

/// Scaling for converting tenths to physical units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scaling {
    /// Millimeters per unit
    pub millimeters: f64,

    /// Tenths per unit
    pub tenths: f64,
}

/// Page layout settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PageLayout {
    /// Page height
    #[serde(rename = "page-height", skip_serializing_if = "Option::is_none")]
    pub page_height: Option<f64>,

    /// Page width
    #[serde(rename = "page-width", skip_serializing_if = "Option::is_none")]
    pub page_width: Option<f64>,

    /// Page margins
    #[serde(
        rename = "page-margins",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub page_margins: Vec<PageMargins>,
}

/// Page margins for odd, even, or both page types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMargins {
    /// Margin type (odd, even, both)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<MarginType>,

    /// Left margin
    #[serde(rename = "left-margin")]
    pub left_margin: f64,

    /// Right margin
    #[serde(rename = "right-margin")]
    pub right_margin: f64,

    /// Top margin
    #[serde(rename = "top-margin")]
    pub top_margin: f64,

    /// Bottom margin
    #[serde(rename = "bottom-margin")]
    pub bottom_margin: f64,
}

/// Margin type for page margins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
    Odd,
    Even,
    Both,
}

/// System layout settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemLayout {
    /// System margins
    #[serde(rename = "system-margins", skip_serializing_if = "Option::is_none")]
    pub system_margins: Option<SystemMargins>,

    /// System distance
    #[serde(rename = "system-distance", skip_serializing_if = "Option::is_none")]
    pub system_distance: Option<f64>,

    /// Top system distance
    #[serde(
        rename = "top-system-distance",
        skip_serializing_if = "Option::is_none"
    )]
    pub top_system_distance: Option<f64>,

    /// System dividers
    #[serde(rename = "system-dividers", skip_serializing_if = "Option::is_none")]
    pub system_dividers: Option<SystemDividers>,
}

/// System margins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMargins {
    /// Left margin
    #[serde(rename = "left-margin")]
    pub left_margin: f64,

    /// Right margin
    #[serde(rename = "right-margin")]
    pub right_margin: f64,
}

/// System dividers configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemDividers {
    /// Left divider
    #[serde(rename = "left-divider", skip_serializing_if = "Option::is_none")]
    pub left_divider: Option<EmptyPrintStyleAlign>,

    /// Right divider
    #[serde(rename = "right-divider", skip_serializing_if = "Option::is_none")]
    pub right_divider: Option<EmptyPrintStyleAlign>,
}

/// Empty element with print-style-align attributes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyPrintStyleAlign {
    /// Whether to print
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,
}

/// Staff layout settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffLayout {
    /// Staff number
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Staff distance
    #[serde(rename = "staff-distance", skip_serializing_if = "Option::is_none")]
    pub staff_distance: Option<f64>,
}

/// Appearance settings for lines and other visual elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Appearance {
    /// Line widths
    #[serde(rename = "line-width", default, skip_serializing_if = "Vec::is_empty")]
    pub line_widths: Vec<LineWidth>,

    /// Note sizes
    #[serde(rename = "note-size", default, skip_serializing_if = "Vec::is_empty")]
    pub note_sizes: Vec<NoteSize>,

    /// Distances
    #[serde(rename = "distance", default, skip_serializing_if = "Vec::is_empty")]
    pub distances: Vec<Distance>,

    /// Glyph settings
    #[serde(rename = "glyph", default, skip_serializing_if = "Vec::is_empty")]
    pub glyphs: Vec<Glyph>,

    /// Other appearance settings
    #[serde(
        rename = "other-appearance",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_appearances: Vec<OtherAppearance>,
}

/// Line width setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineWidth {
    /// Line type
    #[serde(rename = "@type")]
    pub line_width_type: String,

    /// Width value
    #[serde(rename = "$value")]
    pub value: f64,
}

/// Note size setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteSize {
    /// Note size type
    #[serde(rename = "@type")]
    pub note_size_type: NoteSizeType,

    /// Size value (percentage)
    #[serde(rename = "$value")]
    pub value: f64,
}

/// Note size type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteSizeType {
    Cue,
    Grace,
    #[serde(rename = "grace-cue")]
    GraceCue,
    Large,
}

/// Distance setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Distance {
    /// Distance type
    #[serde(rename = "@type")]
    pub distance_type: String,

    /// Distance value
    #[serde(rename = "$value")]
    pub value: f64,
}

/// Glyph substitution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Glyph {
    /// Glyph type
    #[serde(rename = "@type")]
    pub glyph_type: String,

    /// Glyph name
    #[serde(rename = "$value")]
    pub value: String,
}

/// Other appearance setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherAppearance {
    /// Appearance type
    #[serde(rename = "@type")]
    pub appearance_type: String,

    /// Value
    #[serde(rename = "$value")]
    pub value: String,
}

/// Empty font element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyFont {
    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
}

/// Lyric font specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LyricFont {
    /// Lyric number
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Lyric name
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
}

/// Lyric language specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LyricLanguage {
    /// Lyric number
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Lyric name
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Language code
    #[serde(rename = "@xml:lang")]
    pub lang: String,
}

/// Credit text that appears on score pages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Credit {
    /// Page number for this credit
    #[serde(rename = "@page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Credit type
    #[serde(rename = "credit-type", default, skip_serializing_if = "Vec::is_empty")]
    pub credit_types: Vec<String>,

    /// Links
    #[serde(rename = "link", default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,

    /// Bookmarks
    #[serde(rename = "bookmark", default, skip_serializing_if = "Vec::is_empty")]
    pub bookmarks: Vec<Bookmark>,

    /// Credit image or words
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub content: Option<CreditContent>,
}

/// Credit content - either an image or formatted words.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreditContent {
    /// Credit image
    Image(CreditImage),
    /// Credit words (possibly with symbols)
    Words(CreditWords),
}

/// Credit image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditImage {
    /// Image element
    #[serde(rename = "credit-image")]
    pub credit_image: Image,
}

/// Credit words content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditWords {
    /// Words content (can have multiple word/symbol elements)
    #[serde(rename = "credit-words", default)]
    pub words: Vec<FormattedTextId>,
}

/// Image element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// Image source
    #[serde(rename = "@source")]
    pub source: String,

    /// Image type
    #[serde(rename = "@type")]
    pub image_type: String,

    /// Height
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// Width
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// Formatted text with ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormattedTextId {
    /// Text content
    #[serde(rename = "$value")]
    pub value: String,

    /// Optional ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Horizontal position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Vertical position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Text justification
    #[serde(rename = "@justify", skip_serializing_if = "Option::is_none")]
    pub justify: Option<LeftCenterRight>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,
}

/// Link element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// XLink href
    #[serde(rename = "@xlink:href")]
    pub href: String,

    /// XLink type
    #[serde(rename = "@xlink:type", skip_serializing_if = "Option::is_none")]
    pub xlink_type: Option<String>,

    /// XLink role
    #[serde(rename = "@xlink:role", skip_serializing_if = "Option::is_none")]
    pub xlink_role: Option<String>,

    /// XLink title
    #[serde(rename = "@xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<String>,

    /// XLink show
    #[serde(rename = "@xlink:show", skip_serializing_if = "Option::is_none")]
    pub xlink_show: Option<String>,

    /// XLink actuate
    #[serde(rename = "@xlink:actuate", skip_serializing_if = "Option::is_none")]
    pub xlink_actuate: Option<String>,

    /// Name
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Element reference
    #[serde(rename = "@element", skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,

    /// Position
    #[serde(rename = "@position", skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    /// Default X
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,
}

/// Bookmark element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    /// Bookmark ID
    #[serde(rename = "@id")]
    pub id: String,

    /// Name
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Element
    #[serde(rename = "@element", skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,

    /// Position
    #[serde(rename = "@position", skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
}

// ============================================================================
// Part List Types
// ============================================================================

/// List of parts in the score.
///
/// Contains part-group and score-part elements defining the score's part structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PartList {
    /// Part list items (groups and parts in order)
    #[serde(rename = "$value", default)]
    pub items: Vec<PartListItem>,
}

/// An item in the part list - either a part group or a score part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PartListItem {
    /// Part group start or stop
    PartGroup(Box<PartGroup>),
    /// Score part definition
    ScorePart(Box<ScorePart>),
}

/// Part grouping information.
///
/// Groups parts together visually with brackets, braces, or other symbols.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartGroup {
    /// Start or stop marker
    #[serde(rename = "@type")]
    pub group_type: StartStop,

    /// Group number (distinguishes overlapping/nested groups)
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Group name
    #[serde(rename = "group-name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Group name display override
    #[serde(rename = "group-name-display", skip_serializing_if = "Option::is_none")]
    pub group_name_display: Option<NameDisplay>,

    /// Group abbreviation
    #[serde(rename = "group-abbreviation", skip_serializing_if = "Option::is_none")]
    pub group_abbreviation: Option<String>,

    /// Group abbreviation display override
    #[serde(
        rename = "group-abbreviation-display",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_abbreviation_display: Option<NameDisplay>,

    /// Group symbol (brace, bracket, etc.)
    #[serde(rename = "group-symbol", skip_serializing_if = "Option::is_none")]
    pub group_symbol: Option<GroupSymbolValue>,

    /// Group barline behavior
    #[serde(rename = "group-barline", skip_serializing_if = "Option::is_none")]
    pub group_barline: Option<GroupBarlineValue>,

    /// Whether group shares time signatures
    #[serde(rename = "group-time", skip_serializing_if = "Option::is_none")]
    pub group_time: Option<Empty>,
}

/// Group symbol type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupSymbolValue {
    /// Symbol type
    #[serde(rename = "$value")]
    pub value: GroupSymbol,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Relative X position
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Group symbol enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupSymbol {
    None,
    Brace,
    Line,
    Bracket,
    Square,
}

/// Group barline value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupBarlineValue {
    /// Barline type
    #[serde(rename = "$value")]
    pub value: GroupBarline,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Group barline enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupBarline {
    Yes,
    No,
    Mensurstrich,
}

/// Empty element marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Empty;

/// Name display information with optional print-object control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NameDisplay {
    /// Whether to print the name
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Display text and accidentals
    #[serde(rename = "$value", default)]
    pub content: Vec<NameDisplayContent>,
}

/// Content in a name display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NameDisplayContent {
    /// Display text
    DisplayText(FormattedTextId),
    /// Accidental text
    AccidentalText(AccidentalText),
}

/// Accidental in name display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccidentalText {
    /// Accidental value
    #[serde(rename = "$value")]
    pub value: String,

    /// SMuFL glyph name
    #[serde(rename = "@smufl", skip_serializing_if = "Option::is_none")]
    pub smufl: Option<String>,
}

/// Score part definition.
///
/// Contains information for a single part/track in the score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScorePart {
    /// Part ID (required, must be unique)
    #[serde(rename = "@id")]
    pub id: String,

    /// Part-specific identification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<Identification>,

    /// Links to external part files
    #[serde(rename = "part-link", default, skip_serializing_if = "Vec::is_empty")]
    pub part_links: Vec<PartLink>,

    /// Part name (required)
    #[serde(rename = "part-name")]
    pub part_name: PartName,

    /// Part name display override
    #[serde(rename = "part-name-display", skip_serializing_if = "Option::is_none")]
    pub part_name_display: Option<NameDisplay>,

    /// Part abbreviation
    #[serde(rename = "part-abbreviation", skip_serializing_if = "Option::is_none")]
    pub part_abbreviation: Option<PartName>,

    /// Part abbreviation display override
    #[serde(
        rename = "part-abbreviation-display",
        skip_serializing_if = "Option::is_none"
    )]
    pub part_abbreviation_display: Option<NameDisplay>,

    /// Group associations
    #[serde(rename = "group", default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,

    /// Score instruments in this part
    #[serde(
        rename = "score-instrument",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub score_instruments: Vec<ScoreInstrument>,

    /// Players for this part
    #[serde(rename = "player", default, skip_serializing_if = "Vec::is_empty")]
    pub players: Vec<Player>,

    /// MIDI device/instrument assignments
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    pub midi_assignments: Vec<MidiAssignment>,
}

impl ScorePart {
    /// Create a new score part with the given ID and name.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            identification: None,
            part_links: Vec::new(),
            part_name: PartName {
                value: name.to_string(),
                ..Default::default()
            },
            part_name_display: None,
            part_abbreviation: None,
            part_abbreviation_display: None,
            groups: Vec::new(),
            score_instruments: Vec::new(),
            players: Vec::new(),
            midi_assignments: Vec::new(),
        }
    }
}

/// Part name or abbreviation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PartName {
    /// Name value
    #[serde(rename = "$value")]
    pub value: String,

    /// Whether to print
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Text justification
    #[serde(rename = "@justify", skip_serializing_if = "Option::is_none")]
    pub justify: Option<LeftCenterRight>,
}

/// Link to external part file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartLink {
    /// XLink href
    #[serde(rename = "@xlink:href")]
    pub href: String,

    /// XLink type
    #[serde(rename = "@xlink:type", skip_serializing_if = "Option::is_none")]
    pub xlink_type: Option<String>,

    /// XLink role
    #[serde(rename = "@xlink:role", skip_serializing_if = "Option::is_none")]
    pub xlink_role: Option<String>,

    /// XLink title
    #[serde(rename = "@xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<String>,

    /// XLink show
    #[serde(rename = "@xlink:show", skip_serializing_if = "Option::is_none")]
    pub xlink_show: Option<String>,

    /// XLink actuate
    #[serde(rename = "@xlink:actuate", skip_serializing_if = "Option::is_none")]
    pub xlink_actuate: Option<String>,

    /// Instrument links
    #[serde(
        rename = "instrument-link",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub instrument_links: Vec<InstrumentLink>,

    /// Group links
    #[serde(rename = "group-link", default, skip_serializing_if = "Vec::is_empty")]
    pub group_links: Vec<String>,
}

/// Instrument link for part links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentLink {
    /// Instrument ID reference
    #[serde(rename = "@id")]
    pub id: String,
}

/// Score instrument definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreInstrument {
    /// Instrument ID
    #[serde(rename = "@id")]
    pub id: String,

    /// Instrument name
    #[serde(rename = "instrument-name")]
    pub instrument_name: String,

    /// Instrument abbreviation
    #[serde(
        rename = "instrument-abbreviation",
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument_abbreviation: Option<String>,

    /// Instrument sound
    #[serde(rename = "instrument-sound", skip_serializing_if = "Option::is_none")]
    pub instrument_sound: Option<String>,

    /// Solo indication (true = present; uses bool for JSON roundtrip stability)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solo: Option<bool>,

    /// Ensemble size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensemble: Option<Ensemble>,

    /// Virtual instrument info
    #[serde(rename = "virtual-instrument", skip_serializing_if = "Option::is_none")]
    pub virtual_instrument: Option<VirtualInstrument>,
}

/// Ensemble size indicator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ensemble {
    /// Number of performers (empty means unspecified)
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u32>,
}

/// Virtual instrument definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualInstrument {
    /// Virtual library
    #[serde(rename = "virtual-library", skip_serializing_if = "Option::is_none")]
    pub virtual_library: Option<String>,

    /// Virtual instrument name
    #[serde(rename = "virtual-name", skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

/// Player definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    /// Player ID
    #[serde(rename = "@id")]
    pub id: String,

    /// Player name
    #[serde(rename = "player-name")]
    pub player_name: String,
}

/// MIDI assignment (device or instrument).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MidiAssignment {
    /// MIDI device assignment
    MidiDevice(MidiDevice),
    /// MIDI instrument assignment
    MidiInstrument(MidiInstrument),
}

/// MIDI device assignment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MidiDevice {
    /// Device name
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Port number
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// Instrument ID reference
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// MIDI instrument assignment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MidiInstrument {
    /// Instrument ID reference
    #[serde(rename = "@id")]
    pub id: String,

    /// MIDI channel (1-16)
    #[serde(rename = "midi-channel", skip_serializing_if = "Option::is_none")]
    pub midi_channel: Option<u8>,

    /// MIDI name
    #[serde(rename = "midi-name", skip_serializing_if = "Option::is_none")]
    pub midi_name: Option<String>,

    /// MIDI bank
    #[serde(rename = "midi-bank", skip_serializing_if = "Option::is_none")]
    pub midi_bank: Option<u16>,

    /// MIDI program (1-128)
    #[serde(rename = "midi-program", skip_serializing_if = "Option::is_none")]
    pub midi_program: Option<u8>,

    /// MIDI unpitched (1-128)
    #[serde(rename = "midi-unpitched", skip_serializing_if = "Option::is_none")]
    pub midi_unpitched: Option<u8>,

    /// Volume (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,

    /// Pan (-180 to 180)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<f64>,

    /// Elevation (-90 to 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
}

// ============================================================================
// Part Type
// ============================================================================

/// A part in a partwise score.
///
/// Contains measures with music content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Part {
    /// Part ID (must reference a score-part)
    #[serde(rename = "@id")]
    pub id: String,

    /// Measures in this part
    #[serde(rename = "measure", default)]
    pub measures: Vec<super::measure::Measure>,
}

impl Part {
    /// Create a new part with the given ID.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            measures: Vec::new(),
        }
    }
}

// ============================================================================
// Timewise Types (intermediate representation for export)
// ============================================================================

/// A timewise MusicXML score structure.
///
/// In timewise format, measures are the top-level containers and each measure
/// holds a list of parts. This is the natural mapping from MEI (which is also
/// measure-centric) and serves as an intermediate representation before
/// pivoting to the partwise format required for serialization.
///
/// The conversion from timewise to partwise mirrors the logic of the official
/// MusicXML `timepart.xsl` XSLT stylesheet.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScoreTimewise {
    /// MusicXML version
    pub version: Option<String>,

    /// Work identification
    pub work: Option<Work>,

    /// Movement number
    pub movement_number: Option<String>,

    /// Movement title
    pub movement_title: Option<String>,

    /// Identification metadata
    pub identification: Option<Identification>,

    /// Score defaults
    pub defaults: Option<Defaults>,

    /// Credits appearing on pages
    pub credits: Vec<Credit>,

    /// Part list (required) - defines all parts in the score
    pub part_list: PartList,

    /// Measures containing parts (timewise structure)
    pub measures: Vec<TimewiseMeasure>,
}

/// A measure in timewise format.
///
/// Contains the measure-level attributes (number, implicit, etc.) and a list
/// of part entries, each holding that part's content for this measure.
///
/// Note: Unlike partwise measures, timewise measures do NOT carry an `id`
/// attribute. In MusicXML, the `id` attribute on `<measure>` must be unique
/// across the entire document. When converting to partwise, each part's
/// measure instance could be assigned a unique scoped ID if needed.
#[derive(Debug, Clone, PartialEq)]
pub struct TimewiseMeasure {
    /// Measure number (required)
    pub number: String,

    /// Implicit measure (pickup, anacrusis)
    pub implicit: Option<super::super::data::YesNo>,

    /// Non-controlling measure (for multi-rest regions)
    pub non_controlling: Option<super::super::data::YesNo>,

    /// Measure width in tenths
    pub width: Option<f64>,

    /// Part entries within this measure
    pub parts: Vec<TimewisePart>,
}

impl TimewiseMeasure {
    /// Create a new timewise measure with the given number.
    pub fn new(number: &str) -> Self {
        Self {
            number: number.to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            parts: Vec::new(),
        }
    }
}

/// A part's content within a timewise measure.
///
/// This represents one `<part id="...">` element inside a timewise `<measure>`.
/// The content is identical to what would appear inside a partwise `<measure>`.
#[derive(Debug, Clone, PartialEq)]
pub struct TimewisePart {
    /// Part ID (must reference a score-part)
    pub id: String,

    /// Measure content for this part in this measure
    pub content: Vec<super::measure::MeasureContent>,
}

impl TimewisePart {
    /// Create a new timewise part entry with the given ID.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            content: Vec::new(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_partwise_default() {
        let score = ScorePartwise::default();
        assert!(score.work.is_none());
        assert!(score.movement_number.is_none());
        assert!(score.movement_title.is_none());
        assert!(score.identification.is_none());
        assert!(score.credits.is_empty());
        assert!(score.parts.is_empty());
    }

    #[test]
    fn test_score_partwise_with_work() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_number: Some("Op. 1".to_string()),
                work_title: Some("Symphony No. 1".to_string()),
                opus: None,
            }),
            movement_number: Some("1".to_string()),
            movement_title: Some("Allegro".to_string()),
            ..Default::default()
        };

        assert_eq!(score.version.as_deref(), Some("4.0"));
        assert_eq!(
            score.work.as_ref().unwrap().work_title.as_deref(),
            Some("Symphony No. 1")
        );
        assert_eq!(score.movement_number.as_deref(), Some("1"));
        assert_eq!(score.movement_title.as_deref(), Some("Allegro"));
    }

    #[test]
    fn test_typed_text() {
        let composer = TypedText::new(Some("composer"), "J.S. Bach");
        assert_eq!(composer.text_type.as_deref(), Some("composer"));
        assert_eq!(composer.value, "J.S. Bach");

        let untyped = TypedText::untyped("Anonymous");
        assert!(untyped.text_type.is_none());
        assert_eq!(untyped.value, "Anonymous");
    }

    #[test]
    fn test_identification() {
        let ident = Identification {
            creators: vec![
                TypedText::new(Some("composer"), "Mozart"),
                TypedText::new(Some("lyricist"), "Da Ponte"),
            ],
            rights: vec![TypedText::new(Some("music"), "Public Domain")],
            source: Some("First Edition, 1786".to_string()),
            ..Default::default()
        };

        assert_eq!(ident.creators.len(), 2);
        assert_eq!(ident.creators[0].value, "Mozart");
        assert_eq!(ident.rights.len(), 1);
        assert_eq!(ident.source.as_deref(), Some("First Edition, 1786"));
    }

    #[test]
    fn test_encoding() {
        let encoding = Encoding {
            encoding_date: Some("2024-01-15".to_string()),
            software: vec!["Tusk 1.0".to_string()],
            supports: vec![Supports {
                element: "accidental".to_string(),
                support_type: YesNo::Yes,
                attribute: None,
                value: None,
            }],
            ..Default::default()
        };

        assert_eq!(encoding.encoding_date.as_deref(), Some("2024-01-15"));
        assert_eq!(encoding.software.len(), 1);
        assert_eq!(encoding.supports.len(), 1);
        assert_eq!(encoding.supports[0].support_type, YesNo::Yes);
    }

    #[test]
    fn test_part_list_empty() {
        let part_list = PartList::default();
        assert!(part_list.items.is_empty());
    }

    #[test]
    fn test_part_list_with_parts() {
        let part_list = PartList {
            items: vec![
                PartListItem::ScorePart(Box::new(ScorePart::new("P1", "Piano"))),
                PartListItem::ScorePart(Box::new(ScorePart::new("P2", "Violin"))),
            ],
        };

        assert_eq!(part_list.items.len(), 2);
        if let PartListItem::ScorePart(sp) = &part_list.items[0] {
            assert_eq!(sp.id, "P1");
            assert_eq!(sp.part_name.value, "Piano");
        } else {
            panic!("Expected ScorePart");
        }
    }

    #[test]
    fn test_part_group() {
        let group = PartGroup {
            group_type: StartStop::Start,
            number: Some("1".to_string()),
            group_name: Some("Woodwinds".to_string()),
            group_name_display: None,
            group_abbreviation: Some("Ww.".to_string()),
            group_abbreviation_display: None,
            group_symbol: Some(GroupSymbolValue {
                value: GroupSymbol::Bracket,
                default_x: None,
                relative_x: None,
                color: None,
            }),
            group_barline: Some(GroupBarlineValue {
                value: GroupBarline::Yes,
                color: None,
            }),
            group_time: None,
        };

        assert_eq!(group.group_type, StartStop::Start);
        assert_eq!(group.group_name.as_deref(), Some("Woodwinds"));
        assert_eq!(
            group.group_symbol.as_ref().unwrap().value,
            GroupSymbol::Bracket
        );
    }

    #[test]
    fn test_score_part() {
        let mut part = ScorePart::new("P1", "Flute");
        part.part_abbreviation = Some(PartName {
            value: "Fl.".to_string(),
            ..Default::default()
        });
        part.score_instruments.push(ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Flute".to_string(),
            instrument_abbreviation: Some("Fl.".to_string()),
            instrument_sound: Some("wind.flutes.flute".to_string()),
            solo: Some(true),
            ensemble: None,
            virtual_instrument: None,
        });

        assert_eq!(part.id, "P1");
        assert_eq!(part.part_name.value, "Flute");
        assert_eq!(part.part_abbreviation.as_ref().unwrap().value, "Fl.");
        assert_eq!(part.score_instruments.len(), 1);
        assert_eq!(
            part.score_instruments[0].instrument_sound.as_deref(),
            Some("wind.flutes.flute")
        );
    }

    #[test]
    fn test_score_instrument_with_midi() {
        let instrument = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Acoustic Grand Piano".to_string(),
            instrument_abbreviation: None,
            instrument_sound: Some("keyboard.piano".to_string()),
            solo: None,
            ensemble: None,
            virtual_instrument: None,
        };

        assert_eq!(instrument.id, "P1-I1");
        assert_eq!(instrument.instrument_name, "Acoustic Grand Piano");
    }

    #[test]
    fn test_midi_instrument() {
        let midi = MidiInstrument {
            id: "P1-I1".to_string(),
            midi_channel: Some(1),
            midi_name: None,
            midi_bank: None,
            midi_program: Some(1),
            midi_unpitched: None,
            volume: Some(80.0),
            pan: Some(0.0),
            elevation: None,
        };

        assert_eq!(midi.id, "P1-I1");
        assert_eq!(midi.midi_channel, Some(1));
        assert_eq!(midi.midi_program, Some(1));
        assert_eq!(midi.volume, Some(80.0));
    }

    #[test]
    fn test_part() {
        use super::super::measure::Measure;
        let mut part = Part::new("P1");
        part.measures.push(Measure::new("1"));
        part.measures.push(Measure::new("2"));

        assert_eq!(part.id, "P1");
        assert_eq!(part.measures.len(), 2);
        assert_eq!(part.measures[0].number, "1");
        assert_eq!(part.measures[1].number, "2");
    }

    #[test]
    fn test_group_symbol_values() {
        assert_eq!(format!("{:?}", GroupSymbol::Brace), "Brace");
        assert_eq!(format!("{:?}", GroupSymbol::Bracket), "Bracket");
        assert_eq!(format!("{:?}", GroupSymbol::Square), "Square");
        assert_eq!(format!("{:?}", GroupSymbol::Line), "Line");
        assert_eq!(format!("{:?}", GroupSymbol::None), "None");
    }

    #[test]
    fn test_group_barline_values() {
        assert_eq!(format!("{:?}", GroupBarline::Yes), "Yes");
        assert_eq!(format!("{:?}", GroupBarline::No), "No");
        assert_eq!(format!("{:?}", GroupBarline::Mensurstrich), "Mensurstrich");
    }

    #[test]
    fn test_margin_type() {
        assert_eq!(format!("{:?}", MarginType::Odd), "Odd");
        assert_eq!(format!("{:?}", MarginType::Even), "Even");
        assert_eq!(format!("{:?}", MarginType::Both), "Both");
    }

    #[test]
    fn test_note_size_type() {
        assert_eq!(format!("{:?}", NoteSizeType::Cue), "Cue");
        assert_eq!(format!("{:?}", NoteSizeType::Grace), "Grace");
        assert_eq!(format!("{:?}", NoteSizeType::GraceCue), "GraceCue");
        assert_eq!(format!("{:?}", NoteSizeType::Large), "Large");
    }

    #[test]
    fn test_defaults() {
        let defaults = Defaults {
            scaling: Some(Scaling {
                millimeters: 7.0556,
                tenths: 40.0,
            }),
            page_layout: Some(PageLayout {
                page_height: Some(1683.0),
                page_width: Some(1190.0),
                page_margins: vec![PageMargins {
                    margin_type: Some(MarginType::Both),
                    left_margin: 70.0,
                    right_margin: 70.0,
                    top_margin: 88.0,
                    bottom_margin: 88.0,
                }],
            }),
            ..Default::default()
        };

        let scaling = defaults.scaling.as_ref().unwrap();
        assert!((scaling.millimeters - 7.0556).abs() < 0.0001);
        assert!((scaling.tenths - 40.0).abs() < 0.0001);

        let layout = defaults.page_layout.as_ref().unwrap();
        assert_eq!(layout.page_margins.len(), 1);
        assert_eq!(layout.page_margins[0].margin_type, Some(MarginType::Both));
    }

    #[test]
    fn test_credit() {
        let credit = Credit {
            page: Some(1),
            credit_types: vec!["title".to_string()],
            content: Some(CreditContent::Words(CreditWords {
                words: vec![FormattedTextId {
                    value: "Symphony No. 1".to_string(),
                    id: None,
                    default_x: Some(595.0),
                    default_y: Some(1558.0),
                    font_family: None,
                    font_size: Some(FontSize::Points(24.0)),
                    font_style: None,
                    font_weight: Some(FontWeight::Bold),
                    justify: Some(LeftCenterRight::Center),
                    halign: Some(LeftCenterRight::Center),
                    valign: None,
                }],
            })),
            ..Default::default()
        };

        assert_eq!(credit.page, Some(1));
        assert_eq!(credit.credit_types[0], "title");

        if let Some(CreditContent::Words(words)) = &credit.content {
            assert_eq!(words.words[0].value, "Symphony No. 1");
            assert_eq!(words.words[0].justify, Some(LeftCenterRight::Center));
        } else {
            panic!("Expected credit words");
        }
    }

    #[test]
    fn test_player() {
        let player = Player {
            id: "player1".to_string(),
            player_name: "First Violin".to_string(),
        };

        assert_eq!(player.id, "player1");
        assert_eq!(player.player_name, "First Violin");
    }

    #[test]
    fn test_virtual_instrument() {
        let vi = VirtualInstrument {
            virtual_library: Some("Vienna Symphonic Library".to_string()),
            virtual_name: Some("Solo Violin".to_string()),
        };

        assert_eq!(
            vi.virtual_library.as_deref(),
            Some("Vienna Symphonic Library")
        );
        assert_eq!(vi.virtual_name.as_deref(), Some("Solo Violin"));
    }

    #[test]
    fn test_ensemble() {
        let ensemble_specific = Ensemble { value: Some(8) };
        assert_eq!(ensemble_specific.value, Some(8));

        let ensemble_unspecified = Ensemble { value: None };
        assert!(ensemble_unspecified.value.is_none());
    }

    #[test]
    fn test_miscellaneous() {
        let misc = Miscellaneous {
            fields: vec![MiscellaneousField {
                name: "custom-field".to_string(),
                value: "custom-value".to_string(),
            }],
        };

        assert_eq!(misc.fields.len(), 1);
        assert_eq!(misc.fields[0].name, "custom-field");
        assert_eq!(misc.fields[0].value, "custom-value");
    }

    #[test]
    fn test_complete_score_structure() {
        use super::super::measure::Measure;
        // Test creating a complete minimal score structure
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_title: Some("Test Score".to_string()),
                ..Default::default()
            }),
            identification: Some(Identification {
                creators: vec![TypedText::new(Some("composer"), "Test Composer")],
                ..Default::default()
            }),
            part_list: PartList {
                items: vec![PartListItem::ScorePart(Box::new(ScorePart::new(
                    "P1", "Piano",
                )))],
            },
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![Measure::new("1"), Measure::new("2")],
            }],
            ..Default::default()
        };

        // Verify structure
        assert_eq!(score.version.as_deref(), Some("4.0"));
        assert_eq!(
            score.work.as_ref().unwrap().work_title.as_deref(),
            Some("Test Score")
        );
        assert_eq!(score.part_list.items.len(), 1);
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 2);
    }
}

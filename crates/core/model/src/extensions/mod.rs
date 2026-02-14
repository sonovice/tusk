//! Extension point for custom data not in the MEI spec.
//!
//! Custom attributes and elements live under a dedicated namespace so they never
//! collide with MEI or future MEI changes. Round-trip is preserved: unknown
//! content in the extension namespace is read into [ExtensionBag] and written
//! back in a deterministic order (by namespace, then by name).
//!
//! **Namespace**: Project URI not yet defined. Use placeholder
//! `http://tusk.example.org/ns/ext` until a project URI is available.
//!
//! # Typed Extensions
//!
//! Format-specific roundtrip data is stored in typed structs rather than opaque
//! label strings. Per-concept `HashMap<String, T>` fields on [`ExtensionStore`]
//! map MEI element IDs to their extension data without modifying the generated
//! MEI types.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Placeholder URI for Tusk extension namespace. Replace with project URI when defined.
pub const TUSK_EXT_NS: &str = "http://tusk.example.org/ns/ext";

// ---------------------------------------------------------------------------
// XML-level extension bag (existing)
// ---------------------------------------------------------------------------

/// Bag of custom attributes and child elements at the root (or per-element) level.
///
/// Serialization order: custom attributes first (sorted by namespace, then local name),
/// then custom elements (sorted by namespace, then local name). Same namespace
/// declarations as in the document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExtensionBag {
    /// Custom attributes: (namespace_uri, local_name, value).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_attributes: Vec<(String, String, String)>,

    /// Custom child elements (namespace, local name, attributes, content).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_elements: Vec<ExtensionElement>,
}

/// A single custom element in the extension namespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionElement {
    pub namespace: String,
    pub local_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<(String, String, String)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ExtensionContent>,
}

/// Content of an extension element: raw XML string or a list of child extension elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionContent {
    /// Raw XML string (preserves order and any non-extension markup).
    Raw(String),
    /// Child extension elements only.
    Children(Vec<ExtensionElement>),
}

// ---------------------------------------------------------------------------
// FormatOrigin
// ---------------------------------------------------------------------------

/// Indicates which format the data originated from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormatOrigin {
    pub format: SourceFormat,
    /// Format-specific version string (e.g. "2.24.0" for LilyPond).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Pitch language / input mode (e.g. "dutch", "english").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_language: Option<String>,
}

/// Source format enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceFormat {
    LilyPond,
    MusicXML,
    MEI,
}

// ---------------------------------------------------------------------------
// PitchContext
// ---------------------------------------------------------------------------

/// How pitches are interpreted for a staff/voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PitchContext {
    /// `\relative [pitch] { ... }` — pitches relative to previous note.
    Relative {
        /// Reference pitch (step, alter, octave marks). None = `\relative { }`.
        #[serde(skip_serializing_if = "Option::is_none")]
        ref_pitch: Option<ExtPitch>,
    },
    /// `\fixed pitch { ... }` — pitches relative to a fixed reference.
    Fixed { ref_pitch: ExtPitch },
    /// Absolute pitch mode (no wrapper).
    Absolute,
    /// `\transpose from to { ... }`.
    Transpose { from: ExtPitch, to: ExtPitch },
}

/// A pitch value for extension storage (step + alter + octave).
///
/// Simpler than the full LilyPond Pitch model — just the three essential
/// fields needed for pitch context references.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtPitch {
    /// Note step: 'a'–'g'.
    pub step: char,
    /// Alteration in half-steps (0.0 = natural, 1.0 = sharp, -1.0 = flat).
    pub alter: f32,
    /// Octave marks (0 = middle octave, 1 = one up, -1 = one down).
    pub octave: i8,
}

// ---------------------------------------------------------------------------
// OutputDef
// ---------------------------------------------------------------------------

/// A `\header`, `\paper`, `\layout`, or `\midi` block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputDef {
    pub kind: OutputDefKind,
    /// Key-value assignments within the block.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignments: Vec<ExtAssignment>,
    /// Context modification blocks (for layout/midi).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_blocks: Vec<ExtContextBlock>,
}

/// Which output definition block type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputDefKind {
    Header,
    Paper,
    Layout,
    Midi,
}

/// A key-value assignment in an output definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtAssignment {
    pub name: String,
    pub value: ExtValue,
}

/// A typed value in an assignment or property.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExtValue {
    String(String),
    Number(f64),
    Bool(bool),
    /// A Scheme expression stored as its source text.
    Scheme(String),
    /// A structured markup expression stored as its serialized form.
    Markup(String),
    /// A music expression stored as its serialized form.
    Music(String),
    /// An identifier reference.
    Identifier(String),
    /// A markup list.
    MarkupList(String),
    /// A duration value (base, dots).
    Duration(u32, u8),
    /// A symbol list (dot-separated segments).
    SymbolList(Vec<String>),
    /// `\default` — explicit placeholder for an optional argument.
    Default,
}

/// A `\context { ... }` block inside `\layout` or `\midi`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtContextBlock {
    pub items: Vec<ExtContextModItem>,
}

/// An item inside a context modification block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExtContextModItem {
    /// `\ContextName` reference.
    ContextRef(String),
    /// `\consists "engraver"`.
    Consists(String),
    /// `\remove "engraver"`.
    Remove(String),
    /// Key = value assignment.
    Assignment(ExtAssignment),
    /// `\override path = value`.
    Override { path: String, value: ExtValue },
    /// `\revert path`.
    Revert { path: String },
    /// `\set path = value`.
    Set { path: String, value: ExtValue },
    /// `\unset path`.
    Unset { path: String },
    /// `\denies "ContextName"`.
    Denies(String),
    /// `\accepts "ContextName"`.
    Accepts(String),
    /// `\alias "ContextName"`.
    Alias(String),
    /// `\defaultchild "ContextName"`.
    DefaultChild(String),
    /// `\description "text"`.
    Description(String),
    /// `\name "ContextName"`.
    Name(String),
}

// ---------------------------------------------------------------------------
// BookStructure
// ---------------------------------------------------------------------------

/// Book/bookpart hierarchy metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookStructure {
    /// Top-level book index (0-based, for files with multiple `\book` blocks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_index: Option<usize>,
    /// Bookpart index within a book (0-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookpart_index: Option<usize>,
    /// Score index within a bookpart or book (0-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_index: Option<usize>,
    /// Output defs attached to the book level (header, paper).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub book_output_defs: Vec<OutputDef>,
    /// Output defs attached to the bookpart level.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bookpart_output_defs: Vec<OutputDef>,
}

// ---------------------------------------------------------------------------
// StaffContext
// ---------------------------------------------------------------------------

/// Format-specific staff/group context info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffContext {
    /// Context type name (e.g. "Staff", "Voice", "PianoStaff", "ChordNames").
    pub context_type: String,
    /// Optional context instance name (e.g. `= "melody"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contents of `\with { ... }` block, serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_block: Option<String>,
    /// Whether `\new` or `\context` was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<ContextKeywordExt>,
}

/// Whether `\new` or `\context` was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextKeywordExt {
    New,
    Context,
}

// ---------------------------------------------------------------------------
// RepeatInfo
// ---------------------------------------------------------------------------

/// Repeat metadata for concepts not fully captured by MEI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepeatInfo {
    pub repeat_type: RepeatTypeExt,
    pub count: u32,
    /// Number of alternative endings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_count: Option<usize>,
    /// Alternative ending index (0-based) when attached to a specific ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_index: Option<usize>,
}

/// Repeat type enum (mirrors LilyPond model).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RepeatTypeExt {
    Volta,
    Unfold,
    Percent,
    Tremolo,
    Segno,
}

// ---------------------------------------------------------------------------
// GraceInfo
// ---------------------------------------------------------------------------

/// Grace note type distinction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GraceInfo {
    /// `\grace { ... }`.
    Grace,
    /// `\acciaccatura { ... }`.
    Acciaccatura,
    /// `\appoggiatura { ... }`.
    Appoggiatura,
    /// `\afterGrace [fraction] main { grace }`.
    AfterGrace {
        /// Optional fraction (numerator, denominator).
        #[serde(skip_serializing_if = "Option::is_none")]
        fraction: Option<(u32, u32)>,
    },
}

// ---------------------------------------------------------------------------
// PropertyOp
// ---------------------------------------------------------------------------

/// A property operation (override/revert/set/unset/once/tweak).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyOp {
    pub op_type: PropertyOpType,
    /// Dot-separated path (e.g. "Staff.TimeSignature.color").
    pub path: String,
    /// Value for override/set/tweak.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ExtValue>,
    /// Whether `\once` was applied.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub once: bool,
}

/// Property operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyOpType {
    Override,
    Revert,
    Set,
    Unset,
    Tweak,
}

// ---------------------------------------------------------------------------
// FunctionCall
// ---------------------------------------------------------------------------

/// A music function call with typed arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Function name (without leading backslash).
    pub name: String,
    /// Typed arguments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<ExtValue>,
    /// True when this is a partial application (`\etc`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_partial: bool,
}

// ---------------------------------------------------------------------------
// EventSequence
// ---------------------------------------------------------------------------

/// An ordered sequence of control events at specific positions in a music stream.
///
/// Used to preserve clef/key/time changes, bar checks, barlines, auto-beam
/// toggles, tempo marks, rehearsal marks, and markup at their exact positions
/// within a staff's note stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSequence {
    /// Events in stream order, each with a 0-based position index.
    pub events: Vec<PositionedEvent>,
}

/// A control event at a specific position in the music stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionedEvent {
    /// 0-based index in the note/rest event stream.
    pub position: u32,
    /// The event itself.
    pub event: ControlEvent,
}

/// A control event type within a music stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlEvent {
    /// Clef change: `\clef "name"`.
    Clef { name: String },
    /// Key signature: `\key step \mode`.
    Key {
        step: char,
        alter: f32,
        mode: String,
    },
    /// Time signature: `\time num/den`.
    Time {
        /// Numerator components (e.g. [3, 2] for 3+2/8).
        numerators: Vec<u32>,
        denominator: u32,
    },
    /// Bar check: `|`.
    BarCheck,
    /// Explicit barline: `\bar "type"`.
    BarLine { bar_type: String },
    /// `\autoBeamOn`.
    AutoBeamOn,
    /// `\autoBeamOff`.
    AutoBeamOff,
    /// Tempo mark (serialized LilyPond form).
    Tempo { serialized: String },
    /// Rehearsal mark (serialized form).
    Mark { serialized: String },
    /// Text mark (serialized form).
    TextMark { serialized: String },
    /// Markup in music context (serialized form).
    Markup { serialized: String },
    /// Markup list in music context (serialized form).
    MarkupList { serialized: String },
}

// ---------------------------------------------------------------------------
// VariableAssignments
// ---------------------------------------------------------------------------

/// Named variable definitions for roundtrip.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableAssignments {
    pub assignments: Vec<ExtAssignment>,
}

// ---------------------------------------------------------------------------
// ToplevelMarkup
// ---------------------------------------------------------------------------

/// A standalone markup or markuplist at the file top level, with its position
/// among other top-level items (scores, assignments, etc.) for ordering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToplevelMarkup {
    /// 0-based index among all top-level items in the original file.
    pub position: usize,
    /// The kind of markup.
    pub kind: ToplevelMarkupKind,
}

/// Whether the top-level item is `\markup` or `\markuplist`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToplevelMarkupKind {
    /// `\markup { ... }` — serialized form.
    Markup(String),
    /// `\markuplist { ... }` — serialized form.
    MarkupList(String),
}

// ---------------------------------------------------------------------------
// LyricsInfo
// ---------------------------------------------------------------------------

/// Lyrics attachment metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LyricsInfo {
    /// How the lyrics are attached.
    pub style: LyricsStyle,
    /// Voice ID for `\lyricsto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Number of lyrics stanzas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

/// Lyrics attachment style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LyricsStyle {
    /// `\addlyrics { ... }`.
    AddLyrics,
    /// `\lyricsto "voice" { ... }`.
    LyricsTo,
    /// `\lyricmode { ... }`.
    LyricMode,
}

// ---------------------------------------------------------------------------
// ChordRepetition
// ---------------------------------------------------------------------------

/// Marker for chord repetition (`q` in LilyPond).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChordRepetition;

// ---------------------------------------------------------------------------
// ContextChange
// ---------------------------------------------------------------------------

/// `\change ContextType = "name"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextChange {
    /// Context type (e.g. "Staff").
    pub context_type: String,
    /// Target context name.
    pub name: String,
}

// ---------------------------------------------------------------------------
// TweakInfo
// ---------------------------------------------------------------------------

/// A `\tweak` applied to an event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TweakInfo {
    /// Property path.
    pub path: String,
    /// Value.
    pub value: ExtValue,
}

// ---------------------------------------------------------------------------
// PitchedRest
// ---------------------------------------------------------------------------

/// A rest displayed at a specific pitch position (`c4\rest` in LilyPond).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PitchedRest {
    /// Pitch as LilyPond note name + octave marks (e.g. "c'", "fis,,").
    pub pitch: String,
}

// ---------------------------------------------------------------------------
// MultiMeasureRestInfo
// ---------------------------------------------------------------------------

/// Duration details for a multi-measure rest (`R1`, `R2.*3` etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultiMeasureRestInfo {
    pub base: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub dots: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub multipliers: Vec<(u32, u32)>,
}

impl Default for MultiMeasureRestInfo {
    fn default() -> Self {
        Self {
            base: 1,
            dots: 0,
            multipliers: Vec::new(),
        }
    }
}

fn is_zero_u8(v: &u8) -> bool {
    *v == 0
}

// ---------------------------------------------------------------------------
// DrumEvent
// ---------------------------------------------------------------------------

/// Serialized drum event for lossless roundtrip.
///
/// Stores the LilyPond serialized form of a drum note or drum chord
/// (e.g. `sn4` or `<sn bd>8`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrumEvent {
    /// Serialized LilyPond drum event string.
    pub serialized: String,
}

// ---------------------------------------------------------------------------
// LyricExtender
// ---------------------------------------------------------------------------

/// Marker for a lyric extender line (`__` in LilyPond) on a `<syl>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LyricExtender;

// ---------------------------------------------------------------------------
// PhrasingSlur
// ---------------------------------------------------------------------------

/// Marker that a `<slur>` is a phrasing slur (`\(` / `\)` in LilyPond).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhrasingSlur;

// ---------------------------------------------------------------------------
// TupletInfo
// ---------------------------------------------------------------------------

/// LilyPond-specific tuplet data for lossless roundtrip on `<tupletSpan>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TupletInfo {
    pub num: u32,
    pub denom: u32,
    /// Optional span duration (base value, e.g. 4 = quarter note).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_duration: Option<DurationInfo>,
}

impl Default for TupletInfo {
    fn default() -> Self {
        Self {
            num: 3,
            denom: 2,
            span_duration: None,
        }
    }
}

/// Compact duration representation for label storage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationInfo {
    pub base: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub dots: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub multipliers: Vec<(u32, u32)>,
}

impl Default for DurationInfo {
    fn default() -> Self {
        Self {
            base: 4,
            dots: 0,
            multipliers: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// OrnamentInfo
// ---------------------------------------------------------------------------

/// Ornament metadata for `<trill>`, `<mordent>`, `<turn>`, `<fermata>`, `<ornam>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrnamentInfo {
    /// Ornament name (e.g. "trill", "mordent", "shortfermata", "prallprall").
    pub name: String,
    /// Placement direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionExt>,
}

/// Placement direction for ornaments, articulations, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectionExt {
    Up,
    Down,
}

// ---------------------------------------------------------------------------
// TremoloInfo
// ---------------------------------------------------------------------------

/// Tremolo subdivision value for `<bTrem>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TremoloInfo {
    /// Subdivision value (e.g. 16, 32). 0 = unmeasured.
    pub value: u32,
}

// ---------------------------------------------------------------------------
// ArticulationInfo
// ---------------------------------------------------------------------------

/// Articulation / fingering / string number metadata on `<dir>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticulationInfo {
    /// What kind of marking this is.
    pub kind: ArticulationKind,
    /// Name or numeric value (e.g. "staccato", "1", "3").
    pub value: String,
    /// Placement direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionExt>,
}

/// Kind of articulation-like marking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArticulationKind {
    Articulation,
    Fingering,
    StringNumber,
}

// ---------------------------------------------------------------------------
// TempoInfo / MarkInfo / TextMarkInfo
// ---------------------------------------------------------------------------

/// Serialized tempo mark for lossless roundtrip on `<tempo>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TempoInfo {
    /// Full serialized LilyPond `\tempo` expression.
    pub serialized: String,
}

/// Serialized rehearsal mark for lossless roundtrip on `<dir>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkInfo {
    /// Full serialized LilyPond `\mark` expression.
    pub serialized: String,
}

/// Serialized text mark for lossless roundtrip on `<dir>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextMarkInfo {
    /// Full serialized LilyPond `\textMark` expression.
    pub serialized: String,
}

// ---------------------------------------------------------------------------
// EndingInfo
// ---------------------------------------------------------------------------

/// Alternative ending metadata on `<dir>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndingInfo {
    /// 0-based ending index.
    pub index: u32,
}

// ---------------------------------------------------------------------------
// ChordModeInfo / FiguredBassInfo
// ---------------------------------------------------------------------------

/// Serialized chord-mode event for lossless roundtrip on `<harm>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChordModeInfo {
    /// Full serialized LilyPond chord-mode event.
    pub serialized: String,
}

/// Serialized figured bass event for lossless roundtrip on `<fb>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiguredBassInfo {
    /// Full serialized LilyPond figure event.
    pub serialized: String,
}

/// Serialized property operation for lossless roundtrip on `<dir>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyOpInfo {
    /// Full serialized LilyPond property operation.
    pub serialized: String,
}

/// Serialized Scheme expression in music position for lossless roundtrip on `<dir>`.
///
/// Stores the full serialized LilyPond `#expr` text (e.g. `#(ly:export ...)`, `#myVar`)
/// so it can be re-parsed on export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemeMusicInfo {
    /// Full serialized Scheme expression (including leading `#`).
    pub serialized: String,
}

/// Serialized text script post-event for lossless roundtrip on `<dir>`.
///
/// Stores the serialized markup/string text and direction so complex markup
/// expressions survive the MEI roundtrip intact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextScriptInfo {
    /// Full serialized text content (markup or quoted string).
    pub serialized: String,
    /// Placement direction.
    pub direction: Option<DirectionExt>,
}

// ---------------------------------------------------------------------------
// ExtensionStore — side table for attaching extensions to MEI elements
// ---------------------------------------------------------------------------

/// Side table mapping MEI element IDs to extension data.
///
/// Since generated MEI types cannot be modified, we use a separate map
/// keyed by element ID string (from `@xml:id`). The store lives alongside
/// the MEI document and is consulted during import/export.
///
/// For elements without an `@xml:id`, callers must assign a synthetic ID
/// before storing extension data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtensionStore {
    // ----- MusicXML per-concept maps -----
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub harmonies: HashMap<String, crate::musicxml_ext::HarmonyData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub barlines: HashMap<String, crate::musicxml_ext::BarlineData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub sounds: HashMap<String, crate::musicxml_ext::SoundData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub prints: HashMap<String, crate::musicxml_ext::PrintData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub measure_styles: HashMap<String, crate::musicxml_ext::MeasureStyleData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub listenings: HashMap<String, crate::musicxml_ext::ListeningData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub note_visuals: HashMap<String, crate::musicxml_ext::NoteVisualData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub note_extras_map: HashMap<String, crate::musicxml_ext::NoteExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub stem_extras_map: HashMap<String, crate::musicxml_ext::StemExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub direction_visuals: HashMap<String, crate::musicxml_ext::DirectionVisualData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub instruments: HashMap<String, crate::musicxml_ext::InstrumentData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub part_details_map: HashMap<String, crate::musicxml_ext::PartDetailsData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub group_details_map: HashMap<String, crate::musicxml_ext::GroupDetailsData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub key_extras_map: HashMap<String, crate::musicxml_ext::KeyExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub time_extras_map: HashMap<String, crate::musicxml_ext::TimeExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub for_parts: HashMap<String, crate::musicxml_ext::ForPartData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub staff_details_map: HashMap<String, crate::musicxml_ext::StaffDetailsExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub part_symbols: HashMap<String, crate::musicxml_ext::PartSymbolExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub transposes: HashMap<String, crate::musicxml_ext::TransposeData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub wedge_spreads: HashMap<String, f64>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metronomes: HashMap<String, crate::musicxml_ext::MetronomeData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub direction_sounds: HashMap<String, crate::musicxml_ext::SoundData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub lyric_extras_map: HashMap<String, crate::musicxml_ext::LyricExtras>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub figured_basses: HashMap<String, crate::musicxml_ext::FiguredBassData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub direction_contents: HashMap<String, crate::musicxml_ext::DirectionContentData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub ornament_details: HashMap<String, crate::musicxml_ext::OrnamentDetailData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub technical_details: HashMap<String, crate::musicxml_ext::TechnicalDetailData>,

    /// StaffDef IDs that have Jianpu clef (mapped to G in MEI, needs Jianpu on export).
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub jianpu_clefs: HashSet<String>,

    // ----- MusicXML singleton -----
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_header: Option<crate::musicxml_ext::ScoreHeaderData>,

    // ----- LilyPond per-concept maps -----
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub format_origins: HashMap<String, FormatOrigin>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub pitch_contexts: HashMap<String, PitchContext>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub output_defs_map: HashMap<String, Vec<OutputDef>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub book_structures: HashMap<String, BookStructure>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub staff_contexts: HashMap<String, StaffContext>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub repeat_infos: HashMap<String, RepeatInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub grace_infos: HashMap<String, GraceInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub property_ops_map: HashMap<String, Vec<PropertyOp>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub function_calls: HashMap<String, FunctionCall>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub event_sequences: HashMap<String, EventSequence>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub variable_assignments_map: HashMap<String, VariableAssignments>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub toplevel_markups_map: HashMap<String, Vec<ToplevelMarkup>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub lyrics_infos: HashMap<String, LyricsInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub chord_repetitions: HashMap<String, ChordRepetition>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub context_changes: HashMap<String, ContextChange>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tweak_infos_map: HashMap<String, Vec<TweakInfo>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub pitched_rests: HashMap<String, PitchedRest>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub mrest_infos: HashMap<String, MultiMeasureRestInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub drum_events: HashMap<String, DrumEvent>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub lyric_extenders: HashMap<String, LyricExtender>,
}

/// Generate get/get_mut/insert/remove accessor methods for ExtensionStore HashMap fields.
macro_rules! ext_store_accessors {
    ($($getter:ident / $inserter:ident => $field:ident : $type:ty),* $(,)?) => {
        paste::paste! {
            impl ExtensionStore {
                $(
                    pub fn $getter(&self, id: &str) -> Option<&$type> {
                        self.$field.get(id)
                    }
                    pub fn [<$getter _mut>](&mut self, id: &str) -> Option<&mut $type> {
                        self.$field.get_mut(id)
                    }
                    pub fn $inserter(&mut self, id: String, data: $type) {
                        self.$field.insert(id, data);
                    }
                    pub fn [<remove_ $getter>](&mut self, id: &str) -> Option<$type> {
                        self.$field.remove(id)
                    }
                )*
            }
        }
    };
}

ext_store_accessors! {
    // MusicXML per-concept maps
    harmony / insert_harmony => harmonies: crate::musicxml_ext::HarmonyData,
    barline / insert_barline => barlines: crate::musicxml_ext::BarlineData,
    sound / insert_sound => sounds: crate::musicxml_ext::SoundData,
    print / insert_print => prints: crate::musicxml_ext::PrintData,
    measure_style / insert_measure_style => measure_styles: crate::musicxml_ext::MeasureStyleData,
    listening / insert_listening => listenings: crate::musicxml_ext::ListeningData,
    note_visual / insert_note_visual => note_visuals: crate::musicxml_ext::NoteVisualData,
    note_extras / insert_note_extras => note_extras_map: crate::musicxml_ext::NoteExtras,
    stem_extras / insert_stem_extras => stem_extras_map: crate::musicxml_ext::StemExtras,
    direction_visual / insert_direction_visual => direction_visuals: crate::musicxml_ext::DirectionVisualData,
    instrument / insert_instrument => instruments: crate::musicxml_ext::InstrumentData,
    part_details / insert_part_details => part_details_map: crate::musicxml_ext::PartDetailsData,
    group_details / insert_group_details => group_details_map: crate::musicxml_ext::GroupDetailsData,
    key_extras / insert_key_extras => key_extras_map: crate::musicxml_ext::KeyExtras,
    time_extras / insert_time_extras => time_extras_map: crate::musicxml_ext::TimeExtras,
    for_part / insert_for_part => for_parts: crate::musicxml_ext::ForPartData,
    staff_details / insert_staff_details => staff_details_map: crate::musicxml_ext::StaffDetailsExtras,
    part_symbol / insert_part_symbol => part_symbols: crate::musicxml_ext::PartSymbolExtras,
    transpose / insert_transpose => transposes: crate::musicxml_ext::TransposeData,
    wedge_spread / insert_wedge_spread => wedge_spreads: f64,
    metronome / insert_metronome => metronomes: crate::musicxml_ext::MetronomeData,
    direction_sound / insert_direction_sound => direction_sounds: crate::musicxml_ext::SoundData,
    lyric_extras / insert_lyric_extras => lyric_extras_map: crate::musicxml_ext::LyricExtras,
    figured_bass_data / insert_figured_bass => figured_basses: crate::musicxml_ext::FiguredBassData,
    direction_content / insert_direction_content => direction_contents: crate::musicxml_ext::DirectionContentData,
    ornament_detail / insert_ornament_detail => ornament_details: crate::musicxml_ext::OrnamentDetailData,
    technical_detail / insert_technical_detail => technical_details: crate::musicxml_ext::TechnicalDetailData,

    // LilyPond per-concept maps
    format_origin / insert_format_origin => format_origins: FormatOrigin,
    pitch_context / insert_pitch_context => pitch_contexts: PitchContext,
    output_defs / insert_output_defs => output_defs_map: Vec<OutputDef>,
    book_structure / insert_book_structure => book_structures: BookStructure,
    staff_context / insert_staff_context => staff_contexts: StaffContext,
    repeat_info / insert_repeat_info => repeat_infos: RepeatInfo,
    grace_info / insert_grace_info => grace_infos: GraceInfo,
    property_ops / insert_property_ops => property_ops_map: Vec<PropertyOp>,
    function_call / insert_function_call => function_calls: FunctionCall,
    event_sequence / insert_event_sequence => event_sequences: EventSequence,
    variable_assignments / insert_variable_assignments => variable_assignments_map: VariableAssignments,
    toplevel_markups / insert_toplevel_markups => toplevel_markups_map: Vec<ToplevelMarkup>,
    lyrics_info / insert_lyrics_info => lyrics_infos: LyricsInfo,
    chord_repetition / insert_chord_repetition => chord_repetitions: ChordRepetition,
    context_change / insert_context_change => context_changes: ContextChange,
    tweak_infos / insert_tweak_infos => tweak_infos_map: Vec<TweakInfo>,
    pitched_rest / insert_pitched_rest => pitched_rests: PitchedRest,
    mrest_info / insert_mrest_info => mrest_infos: MultiMeasureRestInfo,
    drum_event / insert_drum_event => drum_events: DrumEvent,
    lyric_extender / insert_lyric_extender => lyric_extenders: LyricExtender,
}

impl ExtensionStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests;

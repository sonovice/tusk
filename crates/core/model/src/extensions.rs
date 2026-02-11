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
//! label strings. The [`ExtData`] container holds `Option<T>` for each applicable
//! extension type. A side table ([`ExtensionStore`]) maps MEI element IDs to
//! their extension data without modifying the generated MEI types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
// Typed extension data
// ---------------------------------------------------------------------------

/// Container for typed extension data on a single MEI element.
///
/// Each field corresponds to a concept that has no native MEI representation
/// and needs lossless roundtrip storage. Fields are `Option` — only populated
/// when the concept applies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtData {
    /// Source format metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_origin: Option<FormatOrigin>,

    /// Pitch context (relative/fixed/absolute/transpose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_context: Option<PitchContext>,

    /// Output definition block (header/paper/layout/midi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_def: Option<OutputDef>,

    /// Book/bookpart hierarchy metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_structure: Option<BookStructure>,

    /// Staff/group context info (context type, name, with block).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff_context: Option<StaffContext>,

    /// Repeat metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_info: Option<RepeatInfo>,

    /// Grace note type distinction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_info: Option<GraceInfo>,

    /// Property operation (override/revert/set/unset/once/tweak).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_op: Option<PropertyOp>,

    /// Music function call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,

    /// Ordered control event sequence at specific positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_sequence: Option<EventSequence>,

    /// Named variable definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_assignments: Option<VariableAssignments>,

    /// Standalone markup/markuplist at file top level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toplevel_markup: Option<ToplevelMarkup>,

    /// Lyrics attachment metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics_info: Option<LyricsInfo>,

    /// Chord repetition marker (`q`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chord_repetition: Option<ChordRepetition>,

    /// Context change (`\change Staff = "name"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_change: Option<ContextChange>,

    /// Tweak data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweak_info: Option<TweakInfo>,

    /// Multiple tweaks on a single element.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tweaks: Vec<TweakInfo>,

    /// Multiple output defs (e.g. score-level header + layout + midi).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub output_defs: Vec<OutputDef>,

    /// Multiple property operations on a single element.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub property_ops: Vec<PropertyOp>,
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
    /// Serialized arguments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<ExtValue>,
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

/// A standalone markup or markuplist at the file top level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToplevelMarkup {
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
pub struct ExtensionStore {
    /// Map from element ID (`@xml:id` value) to extension data.
    #[serde(flatten)]
    pub data: HashMap<String, ExtData>,
}

impl ExtensionStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get extension data for an element by ID.
    pub fn get(&self, id: &str) -> Option<&ExtData> {
        self.data.get(id)
    }

    /// Get mutable extension data for an element by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut ExtData> {
        self.data.get_mut(id)
    }

    /// Insert or replace extension data for an element.
    pub fn insert(&mut self, id: String, ext: ExtData) {
        self.data.insert(id, ext);
    }

    /// Get extension data for an element, creating a default entry if absent.
    pub fn entry(&mut self, id: String) -> &mut ExtData {
        self.data.entry(id).or_default()
    }

    /// Remove extension data for an element.
    pub fn remove(&mut self, id: &str) -> Option<ExtData> {
        self.data.remove(id)
    }

    /// Returns true if the store has no entries.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Number of elements with extension data.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_data_default_is_empty() {
        let ext = ExtData::default();
        assert!(ext.format_origin.is_none());
        assert!(ext.pitch_context.is_none());
        assert!(ext.output_def.is_none());
        assert!(ext.event_sequence.is_none());
        assert!(ext.tweaks.is_empty());
        assert!(ext.output_defs.is_empty());
        assert!(ext.property_ops.is_empty());
    }

    #[test]
    fn format_origin_roundtrip() {
        let origin = FormatOrigin {
            format: SourceFormat::LilyPond,
            version: Some("2.24.0".into()),
            pitch_language: Some("dutch".into()),
        };
        let json = serde_json::to_string(&origin).unwrap();
        let back: FormatOrigin = serde_json::from_str(&json).unwrap();
        assert_eq!(origin, back);
    }

    #[test]
    fn pitch_context_relative_roundtrip() {
        let ctx = PitchContext::Relative {
            ref_pitch: Some(ExtPitch {
                step: 'c',
                alter: 0.0,
                octave: 1,
            }),
        };
        let json = serde_json::to_string(&ctx).unwrap();
        let back: PitchContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx, back);
    }

    #[test]
    fn pitch_context_transpose_roundtrip() {
        let ctx = PitchContext::Transpose {
            from: ExtPitch {
                step: 'c',
                alter: 0.0,
                octave: 0,
            },
            to: ExtPitch {
                step: 'd',
                alter: 0.0,
                octave: 0,
            },
        };
        let json = serde_json::to_string(&ctx).unwrap();
        let back: PitchContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx, back);
    }

    #[test]
    fn output_def_roundtrip() {
        let def = OutputDef {
            kind: OutputDefKind::Header,
            assignments: vec![ExtAssignment {
                name: "title".into(),
                value: ExtValue::String("My Score".into()),
            }],
            context_blocks: vec![],
        };
        let json = serde_json::to_string(&def).unwrap();
        let back: OutputDef = serde_json::from_str(&json).unwrap();
        assert_eq!(def, back);
    }

    #[test]
    fn output_def_layout_with_context_roundtrip() {
        let def = OutputDef {
            kind: OutputDefKind::Layout,
            assignments: vec![],
            context_blocks: vec![ExtContextBlock {
                items: vec![
                    ExtContextModItem::ContextRef("Score".into()),
                    ExtContextModItem::Consists("Span_arpeggio_engraver".into()),
                    ExtContextModItem::Override {
                        path: "SpacingSpanner.base-shortest-duration".into(),
                        value: ExtValue::Scheme("#(ly:make-moment 1/16)".into()),
                    },
                ],
            }],
        };
        let json = serde_json::to_string(&def).unwrap();
        let back: OutputDef = serde_json::from_str(&json).unwrap();
        assert_eq!(def, back);
    }

    #[test]
    fn grace_info_roundtrip() {
        for grace in [
            GraceInfo::Grace,
            GraceInfo::Acciaccatura,
            GraceInfo::Appoggiatura,
            GraceInfo::AfterGrace {
                fraction: Some((3, 4)),
            },
            GraceInfo::AfterGrace { fraction: None },
        ] {
            let json = serde_json::to_string(&grace).unwrap();
            let back: GraceInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(grace, back);
        }
    }

    #[test]
    fn property_op_roundtrip() {
        let op = PropertyOp {
            op_type: PropertyOpType::Override,
            path: "Staff.TimeSignature.color".into(),
            value: Some(ExtValue::Scheme("#red".into())),
            once: true,
        };
        let json = serde_json::to_string(&op).unwrap();
        let back: PropertyOp = serde_json::from_str(&json).unwrap();
        assert_eq!(op, back);
    }

    #[test]
    fn event_sequence_roundtrip() {
        let seq = EventSequence {
            events: vec![
                PositionedEvent {
                    position: 0,
                    event: ControlEvent::Clef {
                        name: "treble".into(),
                    },
                },
                PositionedEvent {
                    position: 0,
                    event: ControlEvent::Key {
                        step: 'c',
                        alter: 0.0,
                        mode: "major".into(),
                    },
                },
                PositionedEvent {
                    position: 0,
                    event: ControlEvent::Time {
                        numerators: vec![4],
                        denominator: 4,
                    },
                },
                PositionedEvent {
                    position: 4,
                    event: ControlEvent::BarCheck,
                },
                PositionedEvent {
                    position: 8,
                    event: ControlEvent::BarLine {
                        bar_type: "|.".into(),
                    },
                },
            ],
        };
        let json = serde_json::to_string(&seq).unwrap();
        let back: EventSequence = serde_json::from_str(&json).unwrap();
        assert_eq!(seq, back);
    }

    #[test]
    fn variable_assignments_roundtrip() {
        let vars = VariableAssignments {
            assignments: vec![
                ExtAssignment {
                    name: "melody".into(),
                    value: ExtValue::Music("{ c d e f }".into()),
                },
                ExtAssignment {
                    name: "tempo_val".into(),
                    value: ExtValue::Number(120.0),
                },
            ],
        };
        let json = serde_json::to_string(&vars).unwrap();
        let back: VariableAssignments = serde_json::from_str(&json).unwrap();
        assert_eq!(vars, back);
    }

    #[test]
    fn staff_context_roundtrip() {
        let ctx = StaffContext {
            context_type: "PianoStaff".into(),
            name: Some("piano".into()),
            with_block: Some("\\consists \"Span_arpeggio_engraver\"".into()),
            keyword: Some(ContextKeywordExt::New),
        };
        let json = serde_json::to_string(&ctx).unwrap();
        let back: StaffContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx, back);
    }

    #[test]
    fn repeat_info_roundtrip() {
        let info = RepeatInfo {
            repeat_type: RepeatTypeExt::Volta,
            count: 2,
            alternative_count: Some(2),
            ending_index: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        let back: RepeatInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, back);
    }

    #[test]
    fn lyrics_info_roundtrip() {
        let info = LyricsInfo {
            style: LyricsStyle::LyricsTo,
            voice_id: Some("melody".into()),
            count: Some(3),
        };
        let json = serde_json::to_string(&info).unwrap();
        let back: LyricsInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, back);
    }

    #[test]
    fn ext_data_with_multiple_fields_roundtrip() {
        let ext = ExtData {
            format_origin: Some(FormatOrigin {
                format: SourceFormat::LilyPond,
                version: Some("2.24.0".into()),
                pitch_language: None,
            }),
            pitch_context: Some(PitchContext::Relative {
                ref_pitch: Some(ExtPitch {
                    step: 'c',
                    alter: 0.0,
                    octave: 1,
                }),
            }),
            grace_info: Some(GraceInfo::Acciaccatura),
            tweaks: vec![TweakInfo {
                path: "Stem.direction".into(),
                value: ExtValue::Scheme("#UP".into()),
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&ext).unwrap();
        let back: ExtData = serde_json::from_str(&json).unwrap();
        assert_eq!(ext, back);
    }

    #[test]
    fn extension_store_basic_operations() {
        let mut store = ExtensionStore::new();
        assert!(store.is_empty());

        let ext = ExtData {
            format_origin: Some(FormatOrigin {
                format: SourceFormat::LilyPond,
                version: None,
                pitch_language: None,
            }),
            ..Default::default()
        };

        store.insert("note-1".into(), ext.clone());
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());

        let retrieved = store.get("note-1").unwrap();
        assert_eq!(retrieved, &ext);

        // entry() for new element
        let entry = store.entry("note-2".into());
        entry.grace_info = Some(GraceInfo::Grace);
        assert_eq!(store.len(), 2);

        // remove
        let removed = store.remove("note-1");
        assert!(removed.is_some());
        assert_eq!(store.len(), 1);
        assert!(store.get("note-1").is_none());
    }

    #[test]
    fn extension_store_roundtrip() {
        let mut store = ExtensionStore::new();

        store.insert(
            "staff-1".into(),
            ExtData {
                staff_context: Some(StaffContext {
                    context_type: "Staff".into(),
                    name: None,
                    with_block: None,
                    keyword: Some(ContextKeywordExt::New),
                }),
                ..Default::default()
            },
        );

        store.insert(
            "staff-2".into(),
            ExtData {
                event_sequence: Some(EventSequence {
                    events: vec![PositionedEvent {
                        position: 0,
                        event: ControlEvent::Clef {
                            name: "bass".into(),
                        },
                    }],
                }),
                ..Default::default()
            },
        );

        let json = serde_json::to_string(&store).unwrap();
        let back: ExtensionStore = serde_json::from_str(&json).unwrap();
        assert_eq!(store, back);
    }

    #[test]
    fn chord_repetition_roundtrip() {
        let cr = ChordRepetition;
        let json = serde_json::to_string(&cr).unwrap();
        let back: ChordRepetition = serde_json::from_str(&json).unwrap();
        assert_eq!(cr, back);
    }

    #[test]
    fn context_change_roundtrip() {
        let cc = ContextChange {
            context_type: "Staff".into(),
            name: "right".into(),
        };
        let json = serde_json::to_string(&cc).unwrap();
        let back: ContextChange = serde_json::from_str(&json).unwrap();
        assert_eq!(cc, back);
    }

    #[test]
    fn function_call_roundtrip() {
        let fc = FunctionCall {
            name: "breathe".into(),
            args: vec![],
        };
        let json = serde_json::to_string(&fc).unwrap();
        let back: FunctionCall = serde_json::from_str(&json).unwrap();
        assert_eq!(fc, back);
    }

    #[test]
    fn toplevel_markup_roundtrip() {
        let m = ToplevelMarkup::Markup("\\bold { Title }".into());
        let json = serde_json::to_string(&m).unwrap();
        let back: ToplevelMarkup = serde_json::from_str(&json).unwrap();
        assert_eq!(m, back);
    }

    #[test]
    fn tweak_info_roundtrip() {
        let tweak = TweakInfo {
            path: "Beam.positions".into(),
            value: ExtValue::Scheme("#'(2 . 3)".into()),
        };
        let json = serde_json::to_string(&tweak).unwrap();
        let back: TweakInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(tweak, back);
    }

    #[test]
    fn book_structure_roundtrip() {
        let bs = BookStructure {
            book_index: Some(0),
            bookpart_index: Some(1),
            score_index: None,
            book_output_defs: vec![OutputDef {
                kind: OutputDefKind::Paper,
                assignments: vec![ExtAssignment {
                    name: "indent".into(),
                    value: ExtValue::Number(0.0),
                }],
                context_blocks: vec![],
            }],
            bookpart_output_defs: vec![],
        };
        let json = serde_json::to_string(&bs).unwrap();
        let back: BookStructure = serde_json::from_str(&json).unwrap();
        assert_eq!(bs, back);
    }

    #[test]
    fn ext_data_skips_none_in_json() {
        let ext = ExtData::default();
        let json = serde_json::to_string(&ext).unwrap();
        // Should be a nearly empty object (only non-skip fields)
        assert!(!json.contains("format_origin"));
        assert!(!json.contains("pitch_context"));
        assert!(!json.contains("output_def"));
    }

    #[test]
    fn all_source_formats() {
        for fmt in [
            SourceFormat::LilyPond,
            SourceFormat::MusicXML,
            SourceFormat::MEI,
        ] {
            let json = serde_json::to_string(&fmt).unwrap();
            let back: SourceFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(fmt, back);
        }
    }

    #[test]
    fn all_repeat_types() {
        for rt in [
            RepeatTypeExt::Volta,
            RepeatTypeExt::Unfold,
            RepeatTypeExt::Percent,
            RepeatTypeExt::Tremolo,
            RepeatTypeExt::Segno,
        ] {
            let json = serde_json::to_string(&rt).unwrap();
            let back: RepeatTypeExt = serde_json::from_str(&json).unwrap();
            assert_eq!(rt, back);
        }
    }

    #[test]
    fn all_property_op_types() {
        for op in [
            PropertyOpType::Override,
            PropertyOpType::Revert,
            PropertyOpType::Set,
            PropertyOpType::Unset,
            PropertyOpType::Tweak,
        ] {
            let json = serde_json::to_string(&op).unwrap();
            let back: PropertyOpType = serde_json::from_str(&json).unwrap();
            assert_eq!(op, back);
        }
    }

    #[test]
    fn all_lyrics_styles() {
        for style in [
            LyricsStyle::AddLyrics,
            LyricsStyle::LyricsTo,
            LyricsStyle::LyricMode,
        ] {
            let json = serde_json::to_string(&style).unwrap();
            let back: LyricsStyle = serde_json::from_str(&json).unwrap();
            assert_eq!(style, back);
        }
    }

    #[test]
    fn all_output_def_kinds() {
        for kind in [
            OutputDefKind::Header,
            OutputDefKind::Paper,
            OutputDefKind::Layout,
            OutputDefKind::Midi,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: OutputDefKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }
}

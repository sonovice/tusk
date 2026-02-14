//! MusicXML-specific extension types for lossless roundtrip.
//!
//! These typed structs replace the opaque JSON-in-label patterns previously used
//! to store MusicXML data on MEI elements. They live in the core model so that
//! both import and export can reference them without circular dependencies.
//!
//! Each type captures exactly the data needed for lossless MusicXML ↔ MEI
//! roundtrip that cannot be represented natively in MEI.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// HarmonyData
// ---------------------------------------------------------------------------

/// Structured harmony data for lossless roundtrip of MusicXML `<harmony>`.
///
/// Covers chord symbols, Roman numeral analysis, and fretboard diagrams.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HarmonyData {
    /// One or more chord groups (root+kind+bass+degrees).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chords: Vec<HarmonyChordData>,

    /// Fretboard diagram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<FrameData>,

    /// Offset from current position (in divisions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<OffsetData>,

    /// Harmony type.
    #[serde(rename = "ty", skip_serializing_if = "Option::is_none")]
    pub harmony_type: Option<String>,

    /// Print control.
    #[serde(rename = "po", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<bool>,

    /// Frame print control.
    #[serde(rename = "pf", skip_serializing_if = "Option::is_none")]
    pub print_frame: Option<bool>,

    /// Arrangement (vertical/horizontal/diagonal).
    #[serde(rename = "arr", skip_serializing_if = "Option::is_none")]
    pub arrangement: Option<String>,

    /// Placement (above/below).
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,

    /// Visual attributes (font, position, color).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualAttrs>,

    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A single chord within a harmony element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmonyChordData {
    /// Root type: "root", "numeral", or "function".
    pub root_type: String,

    /// Root step (A-G) for root/numeral types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_step: Option<String>,

    /// Root chromatic alteration in semitones.
    #[serde(rename = "ra", skip_serializing_if = "Option::is_none")]
    pub root_alter: Option<f64>,

    /// Root display text override.
    #[serde(rename = "rt", skip_serializing_if = "Option::is_none")]
    pub root_text: Option<String>,

    /// Numeral value (1-7) for numeral type.
    #[serde(rename = "nv", skip_serializing_if = "Option::is_none")]
    pub numeral_value: Option<u32>,

    /// Numeral key (fifths, mode).
    #[serde(rename = "nk", skip_serializing_if = "Option::is_none")]
    pub numeral_key: Option<NumeralKeyData>,

    /// Function text for function type.
    #[serde(rename = "fn", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,

    /// Chord quality/kind.
    pub kind: KindData,

    /// Inversion number.
    #[serde(rename = "inv", skip_serializing_if = "Option::is_none")]
    pub inversion: Option<u32>,

    /// Bass note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bass: Option<BassData>,

    /// Degree modifications.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub degrees: Vec<DegreeData>,
}

/// Numeral key context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumeralKeyData {
    /// Circle of fifths position.
    pub fifths: i8,
    /// Mode (major/minor/etc).
    pub mode: String,
}

/// Chord quality/kind data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KindData {
    /// Kind value (major, minor, dominant, etc).
    #[serde(rename = "v")]
    pub value: String,

    /// Display text override.
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Use symbols.
    #[serde(rename = "us", skip_serializing_if = "Option::is_none")]
    pub use_symbols: Option<bool>,

    /// Stack degrees.
    #[serde(rename = "sd", skip_serializing_if = "Option::is_none")]
    pub stack_degrees: Option<bool>,

    /// Parentheses degrees.
    #[serde(rename = "pd", skip_serializing_if = "Option::is_none")]
    pub parentheses_degrees: Option<bool>,

    /// Bracket degrees.
    #[serde(rename = "bd", skip_serializing_if = "Option::is_none")]
    pub bracket_degrees: Option<bool>,

    /// Halign.
    #[serde(rename = "ha", skip_serializing_if = "Option::is_none")]
    pub halign: Option<String>,
}

/// Bass note data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BassData {
    /// Bass step (A-G).
    pub step: String,

    /// Bass chromatic alteration.
    #[serde(rename = "alt", skip_serializing_if = "Option::is_none")]
    pub alter: Option<f64>,

    /// Bass display text override.
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Bass separator.
    #[serde(rename = "sep", skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,

    /// Bass arrangement.
    #[serde(rename = "arr", skip_serializing_if = "Option::is_none")]
    pub arrangement: Option<String>,
}

/// Degree modification data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DegreeData {
    /// Scale degree value.
    pub value: u32,

    /// Chromatic alteration.
    pub alter: f64,

    /// Modification type (add/alter/subtract).
    #[serde(rename = "ty")]
    pub degree_type: String,

    /// Symbol for degree value.
    #[serde(rename = "sym", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Display text for degree value.
    #[serde(rename = "vt", skip_serializing_if = "Option::is_none")]
    pub value_text: Option<String>,

    /// Plus-minus for alter.
    #[serde(rename = "pm", skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<bool>,
}

/// Fretboard diagram data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameData {
    /// Number of strings.
    pub strings: u32,
    /// Number of frets.
    pub frets: u32,
    /// First fret indicator.
    #[serde(rename = "ff", skip_serializing_if = "Option::is_none")]
    pub first_fret: Option<FirstFretData>,
    /// Notes on the fretboard.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<FrameNoteData>,
    /// Dimensions and alignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualAttrs>,
    /// Unplayed string indicator.
    #[serde(rename = "up", skip_serializing_if = "Option::is_none")]
    pub unplayed: Option<String>,
    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// First fret indicator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstFretData {
    pub value: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// A note on a fretboard diagram.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameNoteData {
    pub string: u32,
    pub fret: u32,
    #[serde(rename = "fg", skip_serializing_if = "Option::is_none")]
    pub fingering: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barre: Option<String>,
}

/// Offset from current position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetData {
    /// Offset value in divisions.
    pub value: f64,
    /// Whether the offset affects sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<bool>,
}

// ---------------------------------------------------------------------------
// TransposeData
// ---------------------------------------------------------------------------

/// Transposition info from MusicXML `<transpose>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TransposeData {
    /// Staff number (for multi-staff parts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Diatonic steps to transpose.
    #[serde(rename = "dia", skip_serializing_if = "Option::is_none")]
    pub diatonic: Option<i32>,

    /// Chromatic semitones to transpose.
    pub chromatic: f64,

    /// Octave shift.
    #[serde(rename = "oct", skip_serializing_if = "Option::is_none")]
    pub octave_change: Option<i32>,

    /// Octave doubling.
    #[serde(rename = "dbl", skip_serializing_if = "Option::is_none")]
    pub double: Option<DoubleData>,

    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Octave doubling info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DoubleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub above: Option<bool>,
}

// ---------------------------------------------------------------------------
// SoundData
// ---------------------------------------------------------------------------

/// Playback/MIDI data from MusicXML `<sound>`.
///
/// Covers standalone and direction-level sound elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SoundData {
    /// Tempo in BPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f64>,

    /// Dynamics percentage.
    #[serde(rename = "dyn", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f64>,

    /// Da capo marker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dacapo: Option<bool>,

    /// Segno target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segno: Option<String>,

    /// Dal segno target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dalsegno: Option<String>,

    /// Coda target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coda: Option<String>,

    /// To-coda target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tocoda: Option<String>,

    /// Timing divisions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisions: Option<f64>,

    /// Forward repeat implied.
    #[serde(rename = "fwd", skip_serializing_if = "Option::is_none")]
    pub forward_repeat: Option<bool>,

    /// Fine marking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine: Option<String>,

    /// Time restriction.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,

    /// Pizzicato flag.
    #[serde(rename = "piz", skip_serializing_if = "Option::is_none")]
    pub pizzicato: Option<bool>,

    /// Pan (-180 to 180).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<f64>,

    /// Elevation (-180 to 180).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    /// Damper pedal.
    #[serde(rename = "dp", skip_serializing_if = "Option::is_none")]
    pub damper_pedal: Option<String>,

    /// Soft pedal.
    #[serde(rename = "sp", skip_serializing_if = "Option::is_none")]
    pub soft_pedal: Option<String>,

    /// Sostenuto pedal.
    #[serde(rename = "ss", skip_serializing_if = "Option::is_none")]
    pub sostenuto_pedal: Option<String>,

    /// MIDI instrument groups.
    #[serde(rename = "midi", skip_serializing_if = "Vec::is_empty")]
    pub midi_groups: Vec<SoundMidiGroupData>,

    /// Swing parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing: Option<SwingData>,

    /// Offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<OffsetData>,

    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// MIDI instrument group within a sound element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SoundMidiGroupData {
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub instrument_change: Option<InstrumentChangeData>,
    #[serde(rename = "md", skip_serializing_if = "Option::is_none")]
    pub midi_device: Option<MidiDeviceData>,
    #[serde(rename = "mi", skip_serializing_if = "Option::is_none")]
    pub midi_instrument: Option<MidiInstrumentDataInner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<PlayData>,
}

/// Instrument change data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentChangeData {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensemble: Option<Option<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_instrument: Option<VirtualInstrumentData>,
}

/// Swing parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwingData {
    /// "straight" or "ratio".
    pub content_type: String,
    /// First beat (for ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<u32>,
    /// Second beat (for ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<u32>,
    /// Swing type (e.g. "eighth").
    #[serde(rename = "ty", skip_serializing_if = "Option::is_none")]
    pub swing_type: Option<String>,
    /// Style string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

// ---------------------------------------------------------------------------
// ScoreHeaderData
// ---------------------------------------------------------------------------

/// Score-level metadata from MusicXML header elements.
///
/// Replaces extMeta elements on meiHead.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ScoreHeaderData {
    /// Identification (creators, rights, encoding, source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<IdentificationData>,

    /// Work metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<WorkData>,

    /// Movement number.
    #[serde(rename = "mn", skip_serializing_if = "Option::is_none")]
    pub movement_number: Option<String>,

    /// Movement title.
    #[serde(rename = "mt", skip_serializing_if = "Option::is_none")]
    pub movement_title: Option<String>,

    /// Defaults (layout, appearance, fonts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<serde_json::Value>,

    /// Credits (title page elements).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credits: Vec<serde_json::Value>,
}

/// Identification metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IdentificationData {
    /// Creators with type (composer, lyricist, arranger, etc).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creators: Vec<TypedTextData>,

    /// Rights declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<TypedTextData>,

    /// Encoding info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<serde_json::Value>,

    /// Source description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Relations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub relations: Vec<TypedTextData>,

    /// Miscellaneous fields.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub miscellaneous: Vec<MiscFieldData>,
}

/// A text value with an optional type attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedTextData {
    #[serde(rename = "ty", skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    pub value: String,
}

/// A miscellaneous name-value field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiscFieldData {
    pub name: String,
    pub value: String,
}

/// Work metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WorkData {
    #[serde(rename = "wn", skip_serializing_if = "Option::is_none")]
    pub work_number: Option<String>,
    #[serde(rename = "wt", skip_serializing_if = "Option::is_none")]
    pub work_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus: Option<String>,
}

// ---------------------------------------------------------------------------
// PrintData
// ---------------------------------------------------------------------------

/// Print/layout data from MusicXML `<print>`.
///
/// Covers system/page breaks and inline layout overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PrintData {
    /// New system break.
    #[serde(rename = "ns", skip_serializing_if = "Option::is_none")]
    pub new_system: Option<bool>,

    /// New page break.
    #[serde(rename = "np", skip_serializing_if = "Option::is_none")]
    pub new_page: Option<bool>,

    /// Number of blank pages to insert.
    #[serde(rename = "bp", skip_serializing_if = "Option::is_none")]
    pub blank_page: Option<u32>,

    /// Page number text.
    #[serde(rename = "pn", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,

    /// Staff spacing override.
    #[serde(rename = "ss", skip_serializing_if = "Option::is_none")]
    pub staff_spacing: Option<f64>,

    /// Inline page layout.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub page_layout: Option<serde_json::Value>,

    /// Inline system layout.
    #[serde(rename = "sl", skip_serializing_if = "Option::is_none")]
    pub system_layout: Option<serde_json::Value>,

    /// Staff layouts.
    #[serde(rename = "stl", skip_serializing_if = "Vec::is_empty")]
    pub staff_layouts: Vec<serde_json::Value>,

    /// Measure layout.
    #[serde(rename = "ml", skip_serializing_if = "Option::is_none")]
    pub measure_layout: Option<serde_json::Value>,

    /// Measure numbering.
    #[serde(rename = "mnum", skip_serializing_if = "Option::is_none")]
    pub measure_numbering: Option<serde_json::Value>,

    /// Part name display.
    #[serde(rename = "pnd", skip_serializing_if = "Option::is_none")]
    pub part_name_display: Option<serde_json::Value>,

    /// Part abbreviation display.
    #[serde(rename = "pad", skip_serializing_if = "Option::is_none")]
    pub part_abbreviation_display: Option<serde_json::Value>,

    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ---------------------------------------------------------------------------
// MeasureStyleData
// ---------------------------------------------------------------------------

/// Measure style info from MusicXML `<measure-style>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasureStyleData {
    /// Staff number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// Content type and data.
    pub content: MeasureStyleContentData,
}

/// Measure style content variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeasureStyleContentData {
    /// Multiple-measure rest.
    MultipleRest {
        /// Number of measures.
        value: u32,
        /// Use symbols instead of number.
        #[serde(skip_serializing_if = "Option::is_none")]
        use_symbols: Option<bool>,
    },
    /// Measure repeat.
    MeasureRepeat {
        /// Measures in pattern.
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<u32>,
        /// Start or stop.
        repeat_type: String,
        /// Number of slashes.
        #[serde(skip_serializing_if = "Option::is_none")]
        slashes: Option<u32>,
    },
    /// Beat repeat.
    BeatRepeat {
        repeat_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        slashes: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        use_dots: Option<bool>,
    },
    /// Slash notation.
    Slash {
        slash_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        use_stems: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        use_dots: Option<bool>,
    },
}

// ---------------------------------------------------------------------------
// BarlineData
// ---------------------------------------------------------------------------

/// Decorated barline extras from MusicXML `<barline>`.
///
/// Captures repeat, ending, fermata, segno, coda, wavy-line data
/// beyond the basic bar-style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BarlineData {
    /// Barline location (left/right/middle).
    #[serde(rename = "loc", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Bar style.
    #[serde(rename = "bs", skip_serializing_if = "Option::is_none")]
    pub bar_style: Option<String>,

    /// Repeat mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<RepeatData>,

    /// Volta ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending: Option<EndingData>,

    /// Fermatas (up to 2).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fermatas: Vec<serde_json::Value>,

    /// Segno mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segno: Option<serde_json::Value>,

    /// Coda mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coda: Option<serde_json::Value>,

    /// Wavy line.
    #[serde(rename = "wl", skip_serializing_if = "Option::is_none")]
    pub wavy_line: Option<serde_json::Value>,

    /// Segno attribute.
    #[serde(rename = "sa", skip_serializing_if = "Option::is_none")]
    pub segno_attr: Option<String>,

    /// Coda attribute.
    #[serde(rename = "ca", skip_serializing_if = "Option::is_none")]
    pub coda_attr: Option<String>,

    /// Divisions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisions: Option<f64>,
}

/// Repeat mark data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepeatData {
    /// Direction (forward/backward).
    pub direction: String,
    /// Repeat times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<u32>,
    /// After jump.
    #[serde(rename = "aj", skip_serializing_if = "Option::is_none")]
    pub after_jump: Option<bool>,
    /// Winged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winged: Option<String>,
}

/// Volta ending data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndingData {
    /// Ending number(s) (e.g. "1", "1,2").
    pub number: String,
    /// Ending type (start/stop/discontinue).
    #[serde(rename = "ty")]
    pub ending_type: String,
    /// Display text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Visual attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualAttrs>,
}

// ---------------------------------------------------------------------------
// ListeningData
// ---------------------------------------------------------------------------

/// Opaque roundtrip container for MusicXML 4.0 listening/grouping/link/bookmark.
///
/// These elements have no direct MEI equivalent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListeningData {
    /// MusicXML `<listening>`.
    Listening(serde_json::Value),
    /// MusicXML `<grouping>`.
    Grouping(serde_json::Value),
    /// MusicXML `<link>`.
    Link(serde_json::Value),
    /// MusicXML `<bookmark>`.
    Bookmark(serde_json::Value),
}

// ---------------------------------------------------------------------------
// NoteVisualData
// ---------------------------------------------------------------------------

/// Note-level visual/position attributes from MusicXML.
///
/// Replaces `musicxml:visual,{json}` label segments on notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NoteVisualData {
    /// Default X position.
    #[serde(rename = "dx", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "dy", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position.
    #[serde(rename = "rx", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position.
    #[serde(rename = "ry", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Print object.
    #[serde(rename = "po", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<bool>,

    /// Print ledger lines.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub print_leger: Option<bool>,

    /// Print spacing.
    #[serde(rename = "ps", skip_serializing_if = "Option::is_none")]
    pub print_spacing: Option<bool>,

    /// Color.
    #[serde(rename = "col", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// MIDI dynamics.
    #[serde(rename = "dyn", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f64>,

    /// End dynamics.
    #[serde(rename = "ed", skip_serializing_if = "Option::is_none")]
    pub end_dynamics: Option<f64>,

    /// Attack.
    #[serde(rename = "att", skip_serializing_if = "Option::is_none")]
    pub attack: Option<f64>,

    /// Release.
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub release: Option<f64>,

    /// Pizzicato.
    #[serde(rename = "piz", skip_serializing_if = "Option::is_none")]
    pub pizzicato: Option<bool>,
}

// ---------------------------------------------------------------------------
// DirectionVisualData
// ---------------------------------------------------------------------------

/// Direction-level visual attributes from MusicXML.
///
/// Replaces `musicxml:words-vis,{json}` label segments on dirs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DirectionVisualData {
    /// Full Words elements with font/position/color data.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub words: Vec<WordsVisualData>,

    /// Wedge/hairpin color.
    #[serde(rename = "wc", skip_serializing_if = "Option::is_none")]
    pub wedge_color: Option<String>,

    /// Wedge niente flag.
    #[serde(rename = "wn", skip_serializing_if = "Option::is_none")]
    pub wedge_niente: Option<bool>,
}

/// Visual attributes for a Words element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordsVisualData {
    /// Text content.
    #[serde(rename = "v")]
    pub value: String,

    /// Font/position/color attributes.
    #[serde(flatten)]
    pub visual: VisualAttrs,

    /// Enclosure shape.
    #[serde(rename = "enc", skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<String>,

    /// Horizontal alignment.
    #[serde(rename = "ha", skip_serializing_if = "Option::is_none")]
    pub halign: Option<String>,

    /// Vertical alignment.
    #[serde(rename = "va", skip_serializing_if = "Option::is_none")]
    pub valign: Option<String>,

    /// Justification.
    #[serde(rename = "j", skip_serializing_if = "Option::is_none")]
    pub justify: Option<String>,

    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ---------------------------------------------------------------------------
// InstrumentData
// ---------------------------------------------------------------------------

/// Score instrument + MIDI instrument details for a part.
///
/// Replaces `musicxml:instrument,{json}` label on instrDef.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentData {
    /// Score instrument definition.
    pub score_instrument: ScoreInstrumentData,

    /// MIDI assignments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub midi_assignments: Vec<MidiAssignmentData>,
}

/// Score instrument data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreInstrumentData {
    /// Unique instrument ID.
    pub id: String,
    /// Instrument name.
    pub name: String,
    /// Abbreviation.
    #[serde(rename = "abbr", skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    /// Instrument sound (e.g. "wind.flutes.flute").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// Solo flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solo: Option<bool>,
    /// Ensemble (None = not present, Some(None) = unspecified, Some(Some(n)) = n players).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensemble: Option<Option<u32>>,
    /// Virtual instrument.
    #[serde(rename = "vi", skip_serializing_if = "Option::is_none")]
    pub virtual_instrument: Option<VirtualInstrumentData>,
}

/// Virtual instrument data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VirtualInstrumentData {
    #[serde(rename = "lib", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,
    #[serde(rename = "nm", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// MIDI assignment data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MidiAssignmentData {
    /// MIDI device.
    #[serde(rename = "dev", skip_serializing_if = "Option::is_none")]
    pub device: Option<MidiDeviceData>,
    /// MIDI instrument settings.
    #[serde(rename = "inst", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<MidiInstrumentDataInner>,
}

/// MIDI device data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MidiDeviceData {
    /// Device name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// MIDI port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Instrument ID reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// MIDI instrument settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MidiInstrumentDataInner {
    /// Instrument ID reference.
    pub id: String,
    /// MIDI channel (1-16).
    #[serde(rename = "ch", skip_serializing_if = "Option::is_none")]
    pub channel: Option<u8>,
    /// MIDI name.
    #[serde(rename = "nm", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// MIDI bank.
    #[serde(rename = "bk", skip_serializing_if = "Option::is_none")]
    pub bank: Option<u16>,
    /// MIDI program (1-128).
    #[serde(rename = "pg", skip_serializing_if = "Option::is_none")]
    pub program: Option<u8>,
    /// MIDI unpitched (1-128).
    #[serde(rename = "up", skip_serializing_if = "Option::is_none")]
    pub unpitched: Option<u8>,
    /// Volume (0-100).
    #[serde(rename = "vol", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    /// Pan (-180 to 180).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<f64>,
    /// Elevation (-90 to 90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
}

// ---------------------------------------------------------------------------
// PartDetailsData
// ---------------------------------------------------------------------------

/// Part-name-display, abbreviation-display, players, part-links, groups.
///
/// Replaces `musicxml:part-details,{json}` label on staffDef.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PartDetailsData {
    /// Part name display.
    #[serde(rename = "pnd", skip_serializing_if = "Option::is_none")]
    pub part_name_display: Option<serde_json::Value>,

    /// Part abbreviation display.
    #[serde(rename = "pad", skip_serializing_if = "Option::is_none")]
    pub part_abbreviation_display: Option<serde_json::Value>,

    /// Players.
    #[serde(rename = "pl", skip_serializing_if = "Vec::is_empty")]
    pub players: Vec<serde_json::Value>,

    /// Part links.
    #[serde(rename = "plk", skip_serializing_if = "Vec::is_empty")]
    pub part_links: Vec<serde_json::Value>,

    /// Groups this part belongs to.
    #[serde(rename = "grp", skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
}

// ---------------------------------------------------------------------------
// GroupDetailsData
// ---------------------------------------------------------------------------

/// Group-name-display, abbreviation-display, group-time.
///
/// Replaces `musicxml:group-details,{json}` label on staffGrp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GroupDetailsData {
    /// Group name display.
    #[serde(rename = "gnd", skip_serializing_if = "Option::is_none")]
    pub group_name_display: Option<serde_json::Value>,

    /// Group abbreviation display.
    #[serde(rename = "gad", skip_serializing_if = "Option::is_none")]
    pub group_abbreviation_display: Option<serde_json::Value>,

    /// Group time flag.
    #[serde(rename = "gt", skip_serializing_if = "Option::is_none")]
    pub group_time: Option<bool>,
}

// ---------------------------------------------------------------------------
// NoteExtras
// ---------------------------------------------------------------------------

/// Note-level roundtrip data not representable in MEI.
///
/// Replaces multiple `musicxml:*` label segments on notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NoteExtras {
    /// Notehead data.
    #[serde(rename = "nh", skip_serializing_if = "Option::is_none")]
    pub notehead: Option<serde_json::Value>,

    /// Notehead text.
    #[serde(rename = "nht", skip_serializing_if = "Option::is_none")]
    pub notehead_text: Option<serde_json::Value>,

    /// Play data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<PlayData>,

    /// Listen data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<serde_json::Value>,

    /// Footnote.
    #[serde(rename = "fn", skip_serializing_if = "Option::is_none")]
    pub footnote: Option<serde_json::Value>,

    /// Level.
    #[serde(rename = "lv", skip_serializing_if = "Option::is_none")]
    pub level: Option<serde_json::Value>,

    /// Notations-level footnote.
    #[serde(rename = "nfn", skip_serializing_if = "Option::is_none")]
    pub notations_footnote: Option<serde_json::Value>,

    /// Notations-level level.
    #[serde(rename = "nlv", skip_serializing_if = "Option::is_none")]
    pub notations_level: Option<serde_json::Value>,

    /// Instrument references.
    #[serde(rename = "inst", skip_serializing_if = "Vec::is_empty")]
    pub instruments: Vec<String>,

    /// Full MusicXML articulations data for lossless multi-artic roundtrip.
    #[serde(rename = "art", skip_serializing_if = "Option::is_none")]
    pub articulations: Option<serde_json::Value>,
}

/// Play data for note-level or sound-level playback.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PlayData {
    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Play entries (IPA, mute, semi-pitched, other-play).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// StemExtras
// ---------------------------------------------------------------------------

/// Stem roundtrip for double/none values not in MEI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StemExtras {
    /// Double stem.
    Double,
    /// No stem (distinct from MEI @stem.visible=false).
    None,
}

// ---------------------------------------------------------------------------
// KeyExtras
// ---------------------------------------------------------------------------

/// Non-traditional key and key-octave roundtrip data.
///
/// Replaces `musicxml:key,{json}` label on staffDef.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KeyExtras {
    /// Full key data for non-traditional keys, cancel, or key-octaves.
    pub key: serde_json::Value,
}

// ---------------------------------------------------------------------------
// TimeExtras
// ---------------------------------------------------------------------------

/// Interchangeable time signature roundtrip data.
///
/// Replaces `musicxml:time,{json}` label on staffDef.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TimeExtras {
    /// Full time data for interchangeable or complex time signatures.
    pub time: serde_json::Value,
}

// ---------------------------------------------------------------------------
// ForPartData
// ---------------------------------------------------------------------------

/// For-part with part-clef/part-transpose roundtrip.
///
/// Replaces `musicxml:for-part,{json}` label on staffDef.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForPartData {
    /// Full for-part entries.
    pub entries: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// StaffDetailsExtras
// ---------------------------------------------------------------------------

/// Staff-details roundtrip data.
///
/// Replaces `musicxml:staff-details,{json}` label on staffDef.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StaffDetailsExtras {
    /// Full staff-details data.
    pub details: serde_json::Value,
}

// ---------------------------------------------------------------------------
// PartSymbolExtras
// ---------------------------------------------------------------------------

/// Part-symbol extras (top-staff, bottom-staff, default-x, color).
///
/// Replaces `musicxml:part-symbol,{json}` label on staffGrp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PartSymbolExtras {
    /// Part symbol value (none/brace/line/bracket/square).
    #[serde(rename = "v")]
    pub value: String,

    /// Top staff number.
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub top_staff: Option<u32>,

    /// Bottom staff number.
    #[serde(rename = "bs", skip_serializing_if = "Option::is_none")]
    pub bottom_staff: Option<u32>,

    /// Default X position.
    #[serde(rename = "dx", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Color.
    #[serde(rename = "col", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

// ---------------------------------------------------------------------------
// LyricExtras
// ---------------------------------------------------------------------------

/// Lyric extend type, elision details, visual/position attrs not captured by MEI verse/syl.
///
/// Replaces lyric-specific label segments on verse elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LyricExtras {
    /// Full lyric data for attributes not in MEI (justify, time-only, etc).
    pub lyric: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Shared Visual Attributes
// ---------------------------------------------------------------------------

/// Common visual attributes (font, position, color).
///
/// Reusable across harmony, direction, ending, and other elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VisualAttrs {
    /// Font family.
    #[serde(rename = "ff", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size.
    #[serde(rename = "fs", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,

    /// Font style (normal/italic).
    #[serde(rename = "fst", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,

    /// Font weight (normal/bold).
    #[serde(rename = "fw", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,

    /// Color.
    #[serde(rename = "col", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Default X position.
    #[serde(rename = "dx", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "dy", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position.
    #[serde(rename = "rx", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position.
    #[serde(rename = "ry", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,
}

// ---------------------------------------------------------------------------
// FiguredBassData
// ---------------------------------------------------------------------------

/// Figured bass data from MusicXML `<figured-bass>`.
///
/// Covers figures (prefix/number/suffix/extend), duration, offset,
/// parentheses, and visual attributes for lossless roundtrip.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FiguredBassData {
    /// One or more figures.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub figures: Vec<FigureData>,

    /// Duration in divisions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Editorial footnote.
    #[serde(rename = "fn", skip_serializing_if = "Option::is_none")]
    pub footnote: Option<serde_json::Value>,

    /// Editorial level.
    #[serde(rename = "lv", skip_serializing_if = "Option::is_none")]
    pub level: Option<serde_json::Value>,

    /// Offset from current position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<OffsetData>,

    /// Whether figures are in parentheses.
    #[serde(rename = "par", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<bool>,

    /// Placement (above/below).
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,

    /// Print control.
    #[serde(rename = "po", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<bool>,

    /// Visual attributes (font, position, color, alignment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualAttrs>,

    /// Horizontal alignment.
    #[serde(rename = "ha", skip_serializing_if = "Option::is_none")]
    pub halign: Option<String>,

    /// Vertical alignment.
    #[serde(rename = "va", skip_serializing_if = "Option::is_none")]
    pub valign: Option<String>,

    /// Element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A single figure within figured bass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FigureData {
    /// Prefix accidental (e.g. "flat", "sharp").
    #[serde(rename = "px", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<StyleTextData>,

    /// Figure number.
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub figure_number: Option<StyleTextData>,

    /// Suffix accidental.
    #[serde(rename = "sx", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<StyleTextData>,

    /// Extend line.
    #[serde(rename = "ex", skip_serializing_if = "Option::is_none")]
    pub extend: Option<FigureExtendData>,
}

/// Styled text with optional font/color attributes (for figure prefix/number/suffix).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleTextData {
    /// Text value.
    #[serde(rename = "v")]
    pub value: String,

    /// Font/position/color attributes.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualAttrs>,
}

/// Extend line for a figure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FigureExtendData {
    /// Type (start/stop/continue).
    #[serde(rename = "ty", skip_serializing_if = "Option::is_none")]
    pub extend_type: Option<String>,

    /// Visual attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualAttrs>,
}

// ---------------------------------------------------------------------------
// DirectionContentData
// ---------------------------------------------------------------------------

/// Direction type content for lossless roundtrip of MusicXML direction types.
///
/// Each variant captures the data for one `<direction-type>` element.
/// Types that are natively handled by MEI (Dynamics, Wedge, Metronome, Words)
/// are not included here — they use dedicated MEI elements (dynam, hairpin, tempo, dir).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DirectionContentData {
    /// Rehearsal mark(s) — text content.
    Rehearsal(serde_json::Value),
    /// Segno sign(s).
    Segno(serde_json::Value),
    /// Coda sign(s).
    Coda(serde_json::Value),
    /// SMuFL symbol(s) — glyph names.
    Symbol(serde_json::Value),
    /// Dashes (cresc./dim. text extension).
    Dashes(serde_json::Value),
    /// Bracket line.
    Bracket(serde_json::Value),
    /// Piano pedal mark.
    Pedal(serde_json::Value),
    /// Octave shift (8va, 8vb, 15ma, etc.).
    OctaveShift(serde_json::Value),
    /// Harp pedal diagram.
    HarpPedals(serde_json::Value),
    /// Harp damping.
    Damp(serde_json::Value),
    /// Damp all strings.
    DampAll(serde_json::Value),
    /// Eyeglasses symbol.
    Eyeglasses(serde_json::Value),
    /// String mute on/off.
    StringMute(serde_json::Value),
    /// Scordatura (string tuning).
    Scordatura(serde_json::Value),
    /// Embedded image.
    Image(serde_json::Value),
    /// Principal voice marking.
    PrincipalVoice(serde_json::Value),
    /// Percussion pictogram(s).
    Percussion(serde_json::Value),
    /// Accordion registration diagram.
    AccordionRegistration(serde_json::Value),
    /// Staff division symbol.
    StaffDivide(serde_json::Value),
    /// Other direction type.
    OtherDirection(serde_json::Value),
}

#[cfg(test)]
mod tests;

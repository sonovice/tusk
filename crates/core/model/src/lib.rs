// Allow clippy lints in generated code (regenerate with mei-codegen to fix properly)
#![allow(clippy::empty_docs, clippy::unnecessary_cast, unused_variables)]

//! MEI-based internal model for Tusk (MusicXML ↔ MEI converter).
//!
//! This crate contains the internal representation of music notation, modeled
//! directly after MEI (Music Encoding Initiative) elements and attribute classes.
//! The types map 1:1 to MEI constructs for lossless MEI round-trips.
//!
//! # Module Organization
//!
//! - `data` - Data types from `macroSpec type="dt"`
//! - `att/` - Attribute classes from `classSpec type="atts"`
//! - `model` - Model classes from `classSpec type="model"`
//! - `pattern_entities` - Pattern entities from `macroSpec type="pe"`
//! - `elements/` - Elements from `elementSpec`
//! - `validation` - Validation support
//!
//! All types are generated from the MEI RNG schema by `tools/mei-codegen`.
//!
//! DO NOT EDIT generated/ - regenerate with: cargo run -p mei-codegen -- --input specs/mei/modules --output crates/core/model/src/generated

pub mod extensions;
pub mod generated;
pub mod musicxml;
pub mod musicxml_ext;

// Re-export all generated modules at crate root for cleaner imports
pub use generated::SpaceSeparated;
pub use generated::att;
pub use generated::data;
pub use generated::elements;
pub use generated::model;
pub use generated::pattern_entities;
pub use generated::validation;

// Re-export commonly used types
pub use data::*;
pub use elements::*;
pub use extensions::{
    ArticulationInfo, ArticulationKind, BookStructure, ChordModeInfo, ChordRepetition,
    ContextChange, ContextKeywordExt, ControlEvent, DirectionExt, DrumEvent, DurationInfo,
    EndingInfo, EventSequence, ExtAssignment, ExtContextBlock, ExtContextModItem, ExtPitch,
    ExtValue, ExtensionBag, ExtensionContent, ExtensionElement, ExtensionStore, FiguredBassInfo,
    FormatOrigin, FunctionCall, GraceInfo, LyricExtender, LyricsInfo, LyricsStyle, MarkInfo,
    MultiMeasureRestInfo, OrnamentInfo, OutputDef, OutputDefKind, PhrasingSlur, PitchContext,
    PitchedRest, PositionedEvent, PropertyOp, PropertyOpInfo, PropertyOpType, RepeatInfo,
    RepeatTypeExt, SchemeMusicInfo, SourceFormat, StaffContext, TUSK_EXT_NS, TempoInfo,
    TextMarkInfo, TextScriptInfo, ToplevelMarkup, ToplevelMarkupKind, TremoloInfo, TupletInfo,
    TweakInfo, VariableAssignments,
};
pub use musicxml_ext::{
    BarlineData, BassData, DegreeData, DirectionVisualData, DoubleData, EndingData, FirstFretData,
    ForPartData, FrameData, FrameNoteData, GroupDetailsData, HarmonyChordData, HarmonyData,
    IdentificationData, InstrumentChangeData, InstrumentData, KeyExtras, KindData,
    ListeningData as ListeningDataExt, LyricExtras, MeasureStyleContentData, MeasureStyleData,
    MidiAssignmentData, MidiDeviceData, MidiInstrumentDataInner, MiscFieldData, NoteExtras,
    NoteVisualData, NumeralKeyData, OffsetData, PartDetailsData, PartSymbolExtras, PlayData,
    PrintData, RepeatData, ScoreHeaderData, ScoreInstrumentData, SoundData, SoundMidiGroupData,
    StaffDetailsExtras, StemExtras, SwingData, TimeExtras, TransposeData, TypedTextData,
    VirtualInstrumentData, VisualAttrs, WordsVisualData, WorkData,
};
pub use validation::{Validate, ValidationContext, ValidationError, ValidationResult};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        // Basic smoke test to verify the crate structure is valid
    }
}
